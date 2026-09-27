//! Translated from `src/battle_bg.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sUnrefArray sVsLetter_V_OamData sVsLetter_S_OamData sVsLetterAffineAnimCmds0 sVsLetterAffineAnimCmds1 sVsLetterAffineAnimTable sVsLetter_V_SpriteTemplate sVsLetter_S_SpriteTemplate sVsLettersSpriteSheet gBattleBgTemplates sStandardBattleWindowTemplates sBattleArenaWindowTemplates gBattleWindowTemplates sBattleEnvironmentTable
#[allow(unused_imports)]
use crate::data::battle_bg::*;

unsafe extern "C" {
    static mut gBattleAnimBgTilemapBuffer: u8;
    static mut gBattleEnvironment: u8;
    static mut gBattleEnvironmentAnimTilemap_Building: u8;
    static mut gBattleEnvironmentAnimTilemap_Cave: u8;
    static mut gBattleEnvironmentAnimTilemap_Rayquaza: u8;
    static mut gBattleEnvironmentAnimTilemap_Underwater: u8;
    static mut gBattleEnvironmentAnimTiles_Building: u8;
    static mut gBattleEnvironmentAnimTiles_Cave: u8;
    static mut gBattleEnvironmentAnimTiles_Rayquaza: u8;
    static mut gBattleEnvironmentAnimTiles_Underwater: u8;
    static mut gBattleEnvironmentPalette_BuildingGym: u8;
    static mut gBattleEnvironmentPalette_BuildingLeader: u8;
    static mut gBattleEnvironmentPalette_Frontier: u8;
    static mut gBattleEnvironmentPalette_Groudon: u8;
    static mut gBattleEnvironmentPalette_Kyogre: u8;
    static mut gBattleEnvironmentPalette_Rayquaza: u8;
    static mut gBattleEnvironmentPalette_StadiumAqua: u8;
    static mut gBattleEnvironmentPalette_StadiumDrake: u8;
    static mut gBattleEnvironmentPalette_StadiumGlacia: u8;
    static mut gBattleEnvironmentPalette_StadiumMagma: u8;
    static mut gBattleEnvironmentPalette_StadiumPhoebe: u8;
    static mut gBattleEnvironmentPalette_StadiumSidney: u8;
    static mut gBattleEnvironmentPalette_StadiumWallace: u8;
    static mut gBattleEnvironmentTilemap_Building: u8;
    static mut gBattleEnvironmentTilemap_Cave: u8;
    static mut gBattleEnvironmentTilemap_Rayquaza: u8;
    static mut gBattleEnvironmentTilemap_Stadium: u8;
    static mut gBattleEnvironmentTilemap_Water: u8;
    static mut gBattleEnvironmentTiles_Building: u8;
    static mut gBattleEnvironmentTiles_Cave: u8;
    static mut gBattleEnvironmentTiles_Rayquaza: u8;
    static mut gBattleEnvironmentTiles_Stadium: u8;
    static mut gBattleEnvironmentTiles_Water: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleScripting: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTextboxPalette: u8;
    static mut gBattleTextboxTilemap: u8;
    static mut gBattleTextboxTiles: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattleVSFrame_Gfx: u8;
    static mut gBattleVSFrame_Pal: u8;
    static mut gBattleVSFrame_Tilemap: u8;
    static mut gBattleWindowTextPalette: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_BG2_Y: u8;
    static mut gGameVersion: u8;
    static mut gLinkPlayers: u8;
    static mut gMultiBattleIntroBg_Opponent_Tilemap: u8;
    static mut gMultiBattleIntroBg_Player_Tilemap: u8;
    static mut gPartnerTrainerId: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_Draw: u8;
    static mut gText_Loss: u8;
    static mut gText_Win: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainers: u8;
    static mut gUnusedBattleInitSprite: u8;
    static mut gVsLettersGfx: u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AnimateSprites();
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BuildOamBuffer();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn Cos2(a0: u16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn DisableInterrupts(a0: u16);
    fn EnableInterrupts(a0: u16);
    fn GetCurrentMapBattleScene() -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LZDecompressVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut u8) -> u8;
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn Menu_LoadStdPalAt(a0: u16);
    fn PlaySE(a0: u16);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetSpriteData();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn Sin2(a0: u16) -> i16;
}

pub(crate) unsafe extern "C" fn UnusedBattleInit() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        ResetSpriteData();
        spriteId = CreateSprite(
            (&raw mut gUnusedBattleInitSprite).cast::<u8>(),
            0i16,
            0i16,
            0u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        SetMainCallback2(Some(CB2_UnusedBattleInit));
    }
}
pub(crate) unsafe extern "C" fn CB2_UnusedBattleInit() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleInitBgsAndWindows() {
    unsafe {
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const gBattleBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 262144u32) != 0 {
            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(36)).write(1u8);
            SetBgTilemapBuffer(
                1u8,
                ((&raw mut gBattleAnimBgTilemapBuffer).cast::<*mut u8>()).read(),
            );
            SetBgTilemapBuffer(
                2u8,
                ((&raw mut gBattleAnimBgTilemapBuffer).cast::<*mut u8>()).read(),
            );
        } else {
            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(36)).write(0u8);
        }
        InitWindows(
            ((((&raw const gBattleWindowTemplates)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(36)).read()) as i32)
                    as isize,
            ))
            .read(),
        );
        DeactivateAllTextPrinters();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBattleBgsVideo() {
    unsafe {
        DisableInterrupts(2u16);
        EnableInterrupts(197u16);
        BattleInitBgsAndWindows();
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        SetGpuReg(0u8, 45120u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattleMenuWindowGfx() {
    unsafe {
        LoadUserWindowBorderGfx(2u8, 18u16, 16u8);
        LoadUserWindowBorderGfx(2u8, 34u16, 16u8);
        LoadCompressedPalette(
            ((&raw mut gBattleWindowTextPalette).cast::<u32>()).cast::<u32>(),
            80u16,
            32u16,
        );
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 262144u32) != 0 {
            Menu_LoadStdPalAt(112u16);
            LoadMessageBoxGfx(0u8, 48u16, 112u8);
            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(118))
                .write(0u16);
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(118))
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(118))
                                .cast::<u8>(),
                                (0u32
                                    | (crate::c::div_u32(
                                        2u32,
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
pub unsafe extern "C" fn DrawMainBattleBackground() {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 37685506u32) != 0 {
            LZDecompressVram(
                ((&raw mut gBattleEnvironmentTiles_Building).cast::<u32>()).cast::<u32>(),
                ((100696064i32) as usize as *mut u8),
            );
            LZDecompressVram(
                ((&raw mut gBattleEnvironmentTilemap_Building).cast::<u32>()).cast::<u32>(),
                ((100716544i32) as usize as *mut u8),
            );
            LoadCompressedPalette(
                ((&raw mut gBattleEnvironmentPalette_Frontier).cast::<u32>()).cast::<u32>(),
                32u16,
                96u16,
            );
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 268435456u32) != 0 {
                LZDecompressVram(
                    ((&raw mut gBattleEnvironmentTiles_Cave).cast::<u32>()).cast::<u32>(),
                    ((100696064i32) as usize as *mut u8),
                );
                LZDecompressVram(
                    ((&raw mut gBattleEnvironmentTilemap_Cave).cast::<u32>()).cast::<u32>(),
                    ((100716544i32) as usize as *mut u8),
                );
                LoadCompressedPalette(
                    ((&raw mut gBattleEnvironmentPalette_Groudon).cast::<u32>()).cast::<u32>(),
                    32u16,
                    96u16,
                );
            } else {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 536870912u32) != 0 {
                    LZDecompressVram(
                        ((&raw mut gBattleEnvironmentTiles_Water).cast::<u32>()).cast::<u32>(),
                        ((100696064i32) as usize as *mut u8),
                    );
                    LZDecompressVram(
                        ((&raw mut gBattleEnvironmentTilemap_Water).cast::<u32>()).cast::<u32>(),
                        ((100716544i32) as usize as *mut u8),
                    );
                    LoadCompressedPalette(
                        ((&raw mut gBattleEnvironmentPalette_Kyogre).cast::<u32>()).cast::<u32>(),
                        32u16,
                        96u16,
                    );
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1073741824u32) != 0 {
                        LZDecompressVram(
                            ((&raw mut gBattleEnvironmentTiles_Rayquaza).cast::<u32>())
                                .cast::<u32>(),
                            ((100696064i32) as usize as *mut u8),
                        );
                        LZDecompressVram(
                            ((&raw mut gBattleEnvironmentTilemap_Rayquaza).cast::<u32>())
                                .cast::<u32>(),
                            ((100716544i32) as usize as *mut u8),
                        );
                        LoadCompressedPalette(
                            ((&raw mut gBattleEnvironmentPalette_Rayquaza).cast::<u32>())
                                .cast::<u32>(),
                            32u16,
                            96u16,
                        );
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                            let mut trainerClass: u8 = ((((&raw mut gTrainers).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                        as i32) as isize
                                        * 40,
                                ))
                            .wrapping_add(1))
                            .read();
                            if ((trainerClass) as i32) == 32i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_BuildingLeader)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                return;
                            } else {
                                if ((trainerClass) as i32) == 38i32 {
                                    LZDecompressVram(
                                        ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                            .cast::<u32>(),
                                        ((100696064i32) as usize as *mut u8),
                                    );
                                    LZDecompressVram(
                                        ((&raw mut gBattleEnvironmentTilemap_Stadium)
                                            .cast::<u32>())
                                        .cast::<u32>(),
                                        ((100716544i32) as usize as *mut u8),
                                    );
                                    LoadCompressedPalette(
                                        ((&raw mut gBattleEnvironmentPalette_StadiumWallace)
                                            .cast::<u32>())
                                        .cast::<u32>(),
                                        32u16,
                                        96u16,
                                    );
                                    return;
                                }
                            }
                        }
                        'l1: {
                            let __sw1 = ((GetCurrentMapBattleScene()) as i32);
                            let __matched = __sw1 == 0i32
                                || __sw1 == 1i32
                                || __sw1 == 2i32
                                || __sw1 == 3i32
                                || __sw1 == 4i32
                                || __sw1 == 5i32
                                || __sw1 == 6i32
                                || __sw1 == 7i32
                                || __sw1 == 8i32;
                            if __sw1 == 0i32 || !__matched {
                                LZDecompressVram(
                                    ((((((&raw const sBattleEnvironmentTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleEnvironment).cast::<u8>()).read())
                                            as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((((((&raw const sBattleEnvironmentTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleEnvironment).cast::<u8>()).read())
                                            as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((((((&raw const sBattleEnvironmentTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleEnvironment).cast::<u8>()).read())
                                            as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .wrapping_add(16)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            }
                            if __sw1 == 1i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_BuildingGym)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            }
                            if __sw1 == 2i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumMagma)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            }
                            if __sw1 == 3i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumAqua)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            }
                            if __sw1 == 4i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumSidney)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            }
                            if __sw1 == 5i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumPhoebe)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            }
                            if __sw1 == 6i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumGlacia)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            }
                            if __sw1 == 7i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumDrake)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            }
                            if __sw1 == 8i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_Frontier).cast::<u32>())
                                        .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattleTextboxAndBackground() {
    unsafe {
        LZDecompressVram(
            ((&raw mut gBattleTextboxTiles).cast::<u32>()).cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        CopyToBgTilemapBuffer(
            0u8,
            (((&raw mut gBattleTextboxTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u16,
            0u16,
        );
        CopyBgTilemapBufferToVram(0u8);
        LoadCompressedPalette(
            ((&raw mut gBattleTextboxPalette).cast::<u32>()).cast::<u32>(),
            0u16,
            64u16,
        );
        LoadBattleMenuWindowGfx();
        DrawMainBattleBackground();
    }
}
pub(crate) unsafe extern "C" fn DrawLinkBattleParticipantPokeballs(
    taskId: u8,
    multiplayerId: u8,
    bgId: u8,
    destX: u8,
    destY: u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut multiplayerId = multiplayerId;
        let mut bgId = bgId;
        let mut destX = destX;
        let mut destY = destY;
        let mut i: i32 = 0i32;
        let mut pokeballStatuses: u16 = 0u16;
        let mut tiles = crate::ffi::Align4([0u8; 12]);
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32)
                != 0i32
            {
                'l1: {
                    let __sw1 = ((multiplayerId) as i32);
                    if __sw1 == 0i32 {
                        pokeballStatuses = ((63i32
                            & ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .read()) as i32)) as u16);
                        break 'l1;
                    }
                    if __sw1 == 1i32 {
                        pokeballStatuses = (((4032i32
                            & ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as i32))
                            >> 6) as u16);
                        break 'l1;
                    }
                    if __sw1 == 2i32 {
                        pokeballStatuses = (((4032i32
                            & ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .read()) as i32))
                            >> 6) as u16);
                        break 'l1;
                    }
                    if __sw1 == 3i32 {
                        pokeballStatuses = ((63i32
                            & ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as i32)) as u16);
                        break 'l1;
                    }
                }
            } else {
                'l2: {
                    let __sw2 = ((multiplayerId) as i32);
                    if __sw2 == 0i32 {
                        pokeballStatuses = ((63i32
                            & ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .read()) as i32)) as u16);
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        pokeballStatuses = ((63i32
                            & ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as i32)) as u16);
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        pokeballStatuses = (((4032i32
                            & ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .read()) as i32))
                            >> 6) as u16);
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        pokeballStatuses = (((4032i32
                            & ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as i32))
                            >> 6) as u16);
                        break 'l2;
                    }
                }
            }
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 3i32) {
                        break 'l3;
                    }
                    'l4: {
                        (((&raw mut tiles).cast::<u16>()).wrapping_offset((i) as isize)).write(
                            (((crate::c::shr_i32(
                                (((pokeballStatuses) as i32)
                                    & crate::c::shl_i32(3i32, (((i).wrapping_mul(2i32)) as u32))),
                                (((i).wrapping_mul(2i32)) as u32),
                            ))
                            .wrapping_add(24577i32)) as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            CopyToBgTilemapBufferRect_ChangePalette(
                bgId,
                ((&raw mut tiles).cast::<u16>()).cast::<u8>(),
                destX,
                destY,
                3u8,
                1u8,
                17u8,
            );
            CopyBgTilemapBufferToVram(bgId);
        } else {
            if ((multiplayerId) as i32)
                == (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read()) as i32)
            {
                pokeballStatuses = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as u16);
            } else {
                pokeballStatuses = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16);
            }
            {
                i = 0i32;
                'l5: loop {
                    if !(i < 6i32) {
                        break 'l5;
                    }
                    'l6: {
                        (((&raw mut tiles).cast::<u16>()).wrapping_offset((i) as isize)).write(
                            (((crate::c::shr_i32(
                                (((pokeballStatuses) as i32)
                                    & crate::c::shl_i32(3i32, (((i).wrapping_mul(2i32)) as u32))),
                                (((i).wrapping_mul(2i32)) as u32),
                            ))
                            .wrapping_add(24577i32)) as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            CopyToBgTilemapBufferRect_ChangePalette(
                bgId,
                ((&raw mut tiles).cast::<u16>()).cast::<u8>(),
                destX,
                destY,
                6u8,
                1u8,
                17u8,
            );
            CopyBgTilemapBufferToVram(bgId);
        }
    }
}
pub(crate) unsafe extern "C" fn DrawLinkBattleVsScreenOutcomeText() {
    unsafe {
        if ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 3i32 {
            BattlePutTextOnWindow((&raw mut gText_Draw).cast::<u8>(), 21u8);
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                if ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 1i32 {
                    'l1: {
                        let __sw1 = ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read())
                                as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(24)
                        .cast::<u16>())
                        .read()) as i32);
                        if __sw1 == 0i32 {
                            BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 22u8);
                            BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 23u8);
                            break 'l1;
                        }
                        if __sw1 == 1i32 {
                            BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 23u8);
                            BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 22u8);
                            break 'l1;
                        }
                        if __sw1 == 2i32 {
                            BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 22u8);
                            BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 23u8);
                            break 'l1;
                        }
                        if __sw1 == 3i32 {
                            BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 23u8);
                            BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 22u8);
                            break 'l1;
                        }
                    }
                } else {
                    'l2: {
                        let __sw2 = ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read())
                                as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(24)
                        .cast::<u16>())
                        .read()) as i32);
                        if __sw2 == 0i32 {
                            BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 23u8);
                            BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 22u8);
                            break 'l2;
                        }
                        if __sw2 == 1i32 {
                            BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 22u8);
                            BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 23u8);
                            break 'l2;
                        }
                        if __sw2 == 2i32 {
                            BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 23u8);
                            BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 22u8);
                            break 'l2;
                        }
                        if __sw2 == 3i32 {
                            BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 22u8);
                            BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 23u8);
                            break 'l2;
                        }
                    }
                }
            } else {
                if ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 1i32 {
                    if ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read())
                            as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 23u8);
                        BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 22u8);
                    } else {
                        BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 22u8);
                        BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 23u8);
                    }
                } else {
                    if ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read())
                            as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 22u8);
                        BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 23u8);
                    } else {
                        BattlePutTextOnWindow((&raw mut gText_Win).cast::<u8>(), 23u8);
                        BattlePutTextOnWindow((&raw mut gText_Loss).cast::<u8>(), 22u8);
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitLinkBattleVsScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut linkPlayer: *mut u8 = core::ptr::null_mut();
        let mut name: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut palId: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                name = ((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28))
                                .wrapping_add(8))
                                .cast::<u8>();
                                linkPlayer = ((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28);
                                'l4: {
                                    let __sw2 = ((((linkPlayer).wrapping_add(24).cast::<u16>())
                                        .read())
                                        as i32);
                                    if __sw2 == 0i32 {
                                        BattlePutTextOnWindow(name, 17u8);
                                        DrawLinkBattleParticipantPokeballs(
                                            taskId,
                                            ((((linkPlayer).wrapping_add(24).cast::<u16>()).read())
                                                as u8),
                                            1u8,
                                            2u8,
                                            4u8,
                                        );
                                        break 'l4;
                                    }
                                    if __sw2 == 1i32 {
                                        BattlePutTextOnWindow(name, 18u8);
                                        DrawLinkBattleParticipantPokeballs(
                                            taskId,
                                            ((((linkPlayer).wrapping_add(24).cast::<u16>()).read())
                                                as u8),
                                            2u8,
                                            2u8,
                                            4u8,
                                        );
                                        break 'l4;
                                    }
                                    if __sw2 == 2i32 {
                                        BattlePutTextOnWindow(name, 19u8);
                                        DrawLinkBattleParticipantPokeballs(
                                            taskId,
                                            ((((linkPlayer).wrapping_add(24).cast::<u16>()).read())
                                                as u8),
                                            1u8,
                                            2u8,
                                            8u8,
                                        );
                                        break 'l4;
                                    }
                                    if __sw2 == 3i32 {
                                        BattlePutTextOnWindow(name, 20u8);
                                        DrawLinkBattleParticipantPokeballs(
                                            taskId,
                                            ((((linkPlayer).wrapping_add(24).cast::<u16>()).read())
                                                as u8),
                                            2u8,
                                            2u8,
                                            8u8,
                                        );
                                        break 'l4;
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                } else {
                    let mut playerId: u8 =
                        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read();
                    let mut opponentId: u8 = ((((playerId) as i32) ^ 1i32) as u8);
                    let mut opponentId_copy: u8 = opponentId;
                    if ((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize * 28))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        opponentId = playerId;
                        playerId = opponentId_copy;
                    }
                    name = ((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize * 28))
                    .wrapping_add(8))
                    .cast::<u8>();
                    BattlePutTextOnWindow(name, 15u8);
                    name = ((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((opponentId) as i32) as isize * 28))
                    .wrapping_add(8))
                    .cast::<u8>();
                    BattlePutTextOnWindow(name, 16u8);
                    DrawLinkBattleParticipantPokeballs(taskId, playerId, 1u8, 2u8, 7u8);
                    DrawLinkBattleParticipantPokeballs(taskId, opponentId, 2u8, 2u8, 7u8);
                }
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                palId = ((AllocSpritePalette(10000u16)) as i32);
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    (((256i32).wrapping_add((palId).wrapping_mul(16i32))).wrapping_add(15i32))
                        as isize,
                ))
                .write({
                    let __v4 = 32767u16;
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                        (((256i32).wrapping_add((palId).wrapping_mul(16i32))).wrapping_add(15i32))
                            as isize,
                    ))
                    .write(__v4);
                    __v4
                });
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(125)).write(
                    CreateSprite(
                        (&raw const sVsLetter_V_SpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        111i16,
                        80i16,
                        0u8,
                    ),
                );
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(126)).write(
                    CreateSprite(
                        (&raw const sVsLetter_S_SpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        129i16,
                        80i16,
                        0u8,
                    ),
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(125))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(126))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                let __p5 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32)
                    != 0i32
                {
                    ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
                        (((-20i32).wrapping_sub(crate::c::div_i32(
                            ((Sin2(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as u16),
                            )) as i32),
                            32i32,
                        ))) as u16),
                    );
                    ((&raw mut gBattle_BG2_X).cast::<u16>()).write(
                        (((-140i32).wrapping_sub(crate::c::div_i32(
                            ((Sin2(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as u16),
                            )) as i32),
                            32i32,
                        ))) as u16),
                    );
                    ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(65500u16);
                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(65500u16);
                } else {
                    ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
                        (((-20i32).wrapping_sub(crate::c::div_i32(
                            ((Sin2(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as u16),
                            )) as i32),
                            32i32,
                        ))) as u16),
                    );
                    ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
                        (((crate::c::div_i32(
                            ((Cos2(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as u16),
                            )) as i32),
                            32i32,
                        ))
                        .wrapping_sub(164i32)) as u16),
                    );
                    ((&raw mut gBattle_BG2_X).cast::<u16>()).write(
                        (((-140i32).wrapping_sub(crate::c::div_i32(
                            ((Sin2(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as u16),
                            )) as i32),
                            32i32,
                        ))) as u16),
                    );
                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(
                        (((crate::c::div_i32(
                            ((Cos2(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as u16),
                            )) as i32),
                            32i32,
                        ))
                        .wrapping_sub(164i32)) as u16),
                    );
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    != 0i32
                {
                    let __p6 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p6).write((((((__p6).read()) as i32).wrapping_sub(2i32)) as i16));
                    let __p7 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p7).write((((((__p7).read()) as i32).wrapping_add(2i32)) as i16));
                } else {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32)
                        != 0i32
                    {
                        DrawLinkBattleVsScreenOutcomeText();
                    }
                    PlaySE(120u16);
                    DestroyTask(taskId);
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(125))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(126))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(126))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(4),
                        0,
                        10,
                        ((((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(126))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(4),
                            0,
                            10,
                            false,
                        ) as u16) as i32)
                            .wrapping_add(64i32)) as u16) as i32,
                    );
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(125))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(0i16);
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(126))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(1i16);
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(125))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(125))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(32)
                        .cast::<i16>())
                        .read(),
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(126))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(126))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(32)
                        .cast::<i16>())
                        .read(),
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(125))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(126))
                            .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawBattleEntryBackground() {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            LZDecompressVram(
                ((&raw mut gBattleVSFrame_Gfx).cast::<u32>()).cast::<u32>(),
                ((100679680i32) as usize as *mut u8),
            );
            LZDecompressVram(
                ((&raw mut gVsLettersGfx).cast::<u32>()).cast::<u32>(),
                ((100728832i32) as usize as *mut u8),
            );
            LoadCompressedPalette(
                ((&raw mut gBattleVSFrame_Pal).cast::<u32>()).cast::<u32>(),
                96u16,
                32u16,
            );
            SetBgAttribute(1u8, 3u8, 1u8);
            SetGpuReg(10u8, 23556u16);
            CopyToBgTilemapBuffer(
                1u8,
                (((&raw mut gBattleVSFrame_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                0u16,
                0u16,
            );
            CopyToBgTilemapBuffer(
                2u8,
                (((&raw mut gBattleVSFrame_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                0u16,
                0u16,
            );
            CopyBgTilemapBufferToVram(1u8);
            CopyBgTilemapBufferToVram(2u8);
            SetGpuReg(72u8, 54u16);
            SetGpuReg(74u8, 54u16);
            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(65372u16);
            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(65372u16);
            LoadCompressedSpriteSheetUsingHeap(
                (&raw const sVsLettersSpriteSheet).cast::<u8>().cast_mut(),
            );
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 37685506u32) != 0 {
                if (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0))
                    || (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32) == 3075i32)
                {
                    LZDecompressVram(
                        ((&raw mut gBattleEnvironmentAnimTiles_Building).cast::<u32>())
                            .cast::<u32>(),
                        ((100679680i32) as usize as *mut u8),
                    );
                    LZDecompressVram(
                        ((&raw mut gBattleEnvironmentAnimTilemap_Building).cast::<u32>())
                            .cast::<u32>(),
                        ((100720640i32) as usize as *mut u8),
                    );
                } else {
                    SetBgAttribute(1u8, 1u8, 2u8);
                    SetBgAttribute(2u8, 1u8, 2u8);
                    CopyToBgTilemapBuffer(
                        1u8,
                        (((&raw mut gMultiBattleIntroBg_Opponent_Tilemap).cast::<u32>())
                            .cast::<u32>())
                        .cast::<u8>(),
                        0u16,
                        0u16,
                    );
                    CopyToBgTilemapBuffer(
                        2u8,
                        (((&raw mut gMultiBattleIntroBg_Player_Tilemap).cast::<u32>())
                            .cast::<u32>())
                        .cast::<u8>(),
                        0u16,
                        0u16,
                    );
                    CopyBgTilemapBufferToVram(1u8);
                    CopyBgTilemapBufferToVram(2u8);
                }
            } else {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 268435456u32) != 0 {
                    LZDecompressVram(
                        ((&raw mut gBattleEnvironmentAnimTiles_Cave).cast::<u32>()).cast::<u32>(),
                        ((100679680i32) as usize as *mut u8),
                    );
                    LZDecompressVram(
                        ((&raw mut gBattleEnvironmentAnimTilemap_Cave).cast::<u32>()).cast::<u32>(),
                        ((100720640i32) as usize as *mut u8),
                    );
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 536870912u32) != 0 {
                        LZDecompressVram(
                            ((&raw mut gBattleEnvironmentAnimTiles_Underwater).cast::<u32>())
                                .cast::<u32>(),
                            ((100679680i32) as usize as *mut u8),
                        );
                        LZDecompressVram(
                            ((&raw mut gBattleEnvironmentAnimTilemap_Underwater).cast::<u32>())
                                .cast::<u32>(),
                            ((100720640i32) as usize as *mut u8),
                        );
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1073741824u32) != 0
                        {
                            LZDecompressVram(
                                ((&raw mut gBattleEnvironmentAnimTiles_Rayquaza).cast::<u32>())
                                    .cast::<u32>(),
                                ((100679680i32) as usize as *mut u8),
                            );
                            LZDecompressVram(
                                ((&raw mut gBattleEnvironmentAnimTilemap_Rayquaza).cast::<u32>())
                                    .cast::<u32>(),
                                ((100720640i32) as usize as *mut u8),
                            );
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                                let mut trainerClass: u8 = ((((&raw mut gTrainers).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 40,
                                    ))
                                .wrapping_add(1))
                                .read();
                                if ((trainerClass) as i32) == 32i32 {
                                    LZDecompressVram(
                                        ((&raw mut gBattleEnvironmentAnimTiles_Building)
                                            .cast::<u32>())
                                        .cast::<u32>(),
                                        ((100679680i32) as usize as *mut u8),
                                    );
                                    LZDecompressVram(
                                        ((&raw mut gBattleEnvironmentAnimTilemap_Building)
                                            .cast::<u32>())
                                        .cast::<u32>(),
                                        ((100720640i32) as usize as *mut u8),
                                    );
                                    return;
                                } else {
                                    if ((trainerClass) as i32) == 38i32 {
                                        LZDecompressVram(
                                            ((&raw mut gBattleEnvironmentAnimTiles_Building)
                                                .cast::<u32>())
                                            .cast::<u32>(),
                                            ((100679680i32) as usize as *mut u8),
                                        );
                                        LZDecompressVram(
                                            ((&raw mut gBattleEnvironmentAnimTilemap_Building)
                                                .cast::<u32>())
                                            .cast::<u32>(),
                                            ((100720640i32) as usize as *mut u8),
                                        );
                                        return;
                                    }
                                }
                            }
                            if ((GetCurrentMapBattleScene()) as i32) == 0i32 {
                                LZDecompressVram(
                                    ((((((&raw const sBattleEnvironmentTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleEnvironment).cast::<u8>()).read())
                                            as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .wrapping_add(8)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>(),
                                    ((100679680i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((((((&raw const sBattleEnvironmentTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleEnvironment).cast::<u8>()).read())
                                            as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .wrapping_add(12)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>(),
                                    ((100720640i32) as usize as *mut u8),
                                );
                            } else {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentAnimTiles_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100679680i32) as usize as *mut u8),
                                );
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentAnimTilemap_Building)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    ((100720640i32) as usize as *mut u8),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadChosenBattleElement(caseId: u8) -> u8 {
    unsafe {
        let mut caseId = caseId;
        let mut ret: u8 = 0u8;
        'l1: {
            let __sw1 = ((caseId) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 {
                LZDecompressVram(
                    ((&raw mut gBattleTextboxTiles).cast::<u32>()).cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                CopyToBgTilemapBuffer(
                    0u8,
                    (((&raw mut gBattleTextboxTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyBgTilemapBufferToVram(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadCompressedPalette(
                    ((&raw mut gBattleTextboxPalette).cast::<u32>()).cast::<u32>(),
                    0u16,
                    64u16,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 37685506u32) != 0 {
                    LZDecompressVram(
                        ((&raw mut gBattleEnvironmentTiles_Building).cast::<u32>()).cast::<u32>(),
                        ((100696064i32) as usize as *mut u8),
                    );
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 268435456u32) != 0 {
                        LZDecompressVram(
                            ((&raw mut gBattleEnvironmentTiles_Cave).cast::<u32>()).cast::<u32>(),
                            ((100696064i32) as usize as *mut u8),
                        );
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                            let mut trainerClass: u8 = ((((&raw mut gTrainers).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                        as i32) as isize
                                        * 40,
                                ))
                            .wrapping_add(1))
                            .read();
                            if ((trainerClass) as i32) == 32i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l1;
                            } else {
                                if ((trainerClass) as i32) == 38i32 {
                                    LZDecompressVram(
                                        ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                            .cast::<u32>(),
                                        ((100696064i32) as usize as *mut u8),
                                    );
                                    break 'l1;
                                }
                            }
                        }
                        'l2: {
                            let __sw2 = ((GetCurrentMapBattleScene()) as i32);
                            let __matched = __sw2 == 0i32
                                || __sw2 == 1i32
                                || __sw2 == 2i32
                                || __sw2 == 3i32
                                || __sw2 == 4i32
                                || __sw2 == 5i32
                                || __sw2 == 6i32
                                || __sw2 == 7i32
                                || __sw2 == 8i32;
                            if __sw2 == 0i32 || !__matched {
                                LZDecompressVram(
                                    ((((((&raw const sBattleEnvironmentTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleEnvironment).cast::<u8>()).read())
                                            as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l2;
                            }
                            if __sw2 == 1i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l2;
                            }
                            if __sw2 == 2i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l2;
                            }
                            if __sw2 == 3i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l2;
                            }
                            if __sw2 == 4i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l2;
                            }
                            if __sw2 == 5i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l2;
                            }
                            if __sw2 == 6i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l2;
                            }
                            if __sw2 == 7i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l2;
                            }
                            if __sw2 == 8i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTiles_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100696064i32) as usize as *mut u8),
                                );
                                break 'l2;
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 37685506u32) != 0 {
                    LZDecompressVram(
                        ((&raw mut gBattleEnvironmentTilemap_Building).cast::<u32>()).cast::<u32>(),
                        ((100716544i32) as usize as *mut u8),
                    );
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4096u32) != 0 {
                        if ((((&raw mut gGameVersion).cast::<u8>()).read()) as i32) == 2i32 {
                            LZDecompressVram(
                                ((&raw mut gBattleEnvironmentTilemap_Cave).cast::<u32>())
                                    .cast::<u32>(),
                                ((100716544i32) as usize as *mut u8),
                            );
                        } else {
                            LZDecompressVram(
                                ((&raw mut gBattleEnvironmentTilemap_Water).cast::<u32>())
                                    .cast::<u32>(),
                                ((100716544i32) as usize as *mut u8),
                            );
                        }
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                            let mut trainerClass: u8 = ((((&raw mut gTrainers).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                        as i32) as isize
                                        * 40,
                                ))
                            .wrapping_add(1))
                            .read();
                            if ((trainerClass) as i32) == 32i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l1;
                            } else {
                                if ((trainerClass) as i32) == 38i32 {
                                    LZDecompressVram(
                                        ((&raw mut gBattleEnvironmentTilemap_Stadium)
                                            .cast::<u32>())
                                        .cast::<u32>(),
                                        ((100716544i32) as usize as *mut u8),
                                    );
                                    break 'l1;
                                }
                            }
                        }
                        'l3: {
                            let __sw3 = ((GetCurrentMapBattleScene()) as i32);
                            let __matched = __sw3 == 0i32
                                || __sw3 == 1i32
                                || __sw3 == 2i32
                                || __sw3 == 3i32
                                || __sw3 == 4i32
                                || __sw3 == 5i32
                                || __sw3 == 6i32
                                || __sw3 == 7i32
                                || __sw3 == 8i32;
                            if __sw3 == 0i32 || !__matched {
                                LZDecompressVram(
                                    ((((((&raw const sBattleEnvironmentTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleEnvironment).cast::<u8>()).read())
                                            as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l3;
                            }
                            if __sw3 == 1i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l3;
                            }
                            if __sw3 == 2i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l3;
                            }
                            if __sw3 == 3i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l3;
                            }
                            if __sw3 == 4i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l3;
                            }
                            if __sw3 == 5i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l3;
                            }
                            if __sw3 == 6i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l3;
                            }
                            if __sw3 == 7i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Stadium).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l3;
                            }
                            if __sw3 == 8i32 {
                                LZDecompressVram(
                                    ((&raw mut gBattleEnvironmentTilemap_Building).cast::<u32>())
                                        .cast::<u32>(),
                                    ((100716544i32) as usize as *mut u8),
                                );
                                break 'l3;
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 37685506u32) != 0 {
                    LoadCompressedPalette(
                        ((&raw mut gBattleEnvironmentPalette_Frontier).cast::<u32>()).cast::<u32>(),
                        32u16,
                        96u16,
                    );
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4096u32) != 0 {
                        if ((((&raw mut gGameVersion).cast::<u8>()).read()) as i32) == 2i32 {
                            LoadCompressedPalette(
                                ((&raw mut gBattleEnvironmentPalette_Groudon).cast::<u32>())
                                    .cast::<u32>(),
                                32u16,
                                96u16,
                            );
                        } else {
                            LoadCompressedPalette(
                                ((&raw mut gBattleEnvironmentPalette_Kyogre).cast::<u32>())
                                    .cast::<u32>(),
                                32u16,
                                96u16,
                            );
                        }
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                            let mut trainerClass: u8 = ((((&raw mut gTrainers).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                        as i32) as isize
                                        * 40,
                                ))
                            .wrapping_add(1))
                            .read();
                            if ((trainerClass) as i32) == 32i32 {
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_BuildingLeader)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l1;
                            } else {
                                if ((trainerClass) as i32) == 38i32 {
                                    LoadCompressedPalette(
                                        ((&raw mut gBattleEnvironmentPalette_StadiumWallace)
                                            .cast::<u32>())
                                        .cast::<u32>(),
                                        32u16,
                                        96u16,
                                    );
                                    break 'l1;
                                }
                            }
                        }
                        'l4: {
                            let __sw4 = ((GetCurrentMapBattleScene()) as i32);
                            let __matched = __sw4 == 0i32
                                || __sw4 == 1i32
                                || __sw4 == 2i32
                                || __sw4 == 3i32
                                || __sw4 == 4i32
                                || __sw4 == 5i32
                                || __sw4 == 6i32
                                || __sw4 == 7i32
                                || __sw4 == 8i32;
                            if __sw4 == 0i32 || !__matched {
                                LoadCompressedPalette(
                                    ((((((&raw const sBattleEnvironmentTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleEnvironment).cast::<u8>()).read())
                                            as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .wrapping_add(16)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l4;
                            }
                            if __sw4 == 1i32 {
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_BuildingGym)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l4;
                            }
                            if __sw4 == 2i32 {
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumMagma)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l4;
                            }
                            if __sw4 == 3i32 {
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumAqua)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l4;
                            }
                            if __sw4 == 4i32 {
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumSidney)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l4;
                            }
                            if __sw4 == 5i32 {
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumPhoebe)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l4;
                            }
                            if __sw4 == 6i32 {
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumGlacia)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l4;
                            }
                            if __sw4 == 7i32 {
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_StadiumDrake)
                                        .cast::<u32>())
                                    .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l4;
                            }
                            if __sw4 == 8i32 {
                                LoadCompressedPalette(
                                    ((&raw mut gBattleEnvironmentPalette_Frontier).cast::<u32>())
                                        .cast::<u32>(),
                                    32u16,
                                    96u16,
                                );
                                break 'l4;
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                LoadBattleMenuWindowGfx();
                break 'l1;
            }
            if !__matched {
                ret = 1u8;
                break 'l1;
            }
        }
        return ret;
    }
}
