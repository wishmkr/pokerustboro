//! Translated from `src/battle_anim.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gOamData_AffineOff_ObjNormal_8x8 gOamData_AffineOff_ObjNormal_16x16 gOamData_AffineOff_ObjNormal_32x32 gOamData_AffineOff_ObjNormal_64x64 gOamData_AffineOff_ObjNormal_16x8 gOamData_AffineOff_ObjNormal_32x8 gOamData_AffineOff_ObjNormal_32x16 gOamData_AffineOff_ObjNormal_64x32 gOamData_AffineOff_ObjNormal_8x16 gOamData_AffineOff_ObjNormal_8x32 gOamData_AffineOff_ObjNormal_16x32 gOamData_AffineOff_ObjNormal_32x64 gOamData_AffineNormal_ObjNormal_8x8 gOamData_AffineNormal_ObjNormal_16x16 gOamData_AffineNormal_ObjNormal_32x32 gOamData_AffineNormal_ObjNormal_64x64 gOamData_AffineNormal_ObjNormal_16x8 gOamData_AffineNormal_ObjNormal_32x8 gOamData_AffineNormal_ObjNormal_32x16 gOamData_AffineNormal_ObjNormal_64x32 gOamData_AffineNormal_ObjNormal_8x16 gOamData_AffineNormal_ObjNormal_8x32 gOamData_AffineNormal_ObjNormal_16x32 gOamData_AffineNormal_ObjNormal_32x64 gOamData_AffineDouble_ObjNormal_8x8 gOamData_AffineDouble_ObjNormal_16x16 gOamData_AffineDouble_ObjNormal_32x32 gOamData_AffineDouble_ObjNormal_64x64 gOamData_AffineDouble_ObjNormal_16x8 gOamData_AffineDouble_ObjNormal_32x8 gOamData_AffineDouble_ObjNormal_32x16 gOamData_AffineDouble_ObjNormal_64x32 gOamData_AffineDouble_ObjNormal_8x16 gOamData_AffineDouble_ObjNormal_8x32 gOamData_AffineDouble_ObjNormal_16x32 gOamData_AffineDouble_ObjNormal_32x64 gOamData_AffineOff_ObjBlend_8x8 gOamData_AffineOff_ObjBlend_16x16 gOamData_AffineOff_ObjBlend_32x32 gOamData_AffineOff_ObjBlend_64x64 gOamData_AffineOff_ObjBlend_16x8 gOamData_AffineOff_ObjBlend_32x8 gOamData_AffineOff_ObjBlend_32x16 gOamData_AffineOff_ObjBlend_64x32 gOamData_AffineOff_ObjBlend_8x16 gOamData_AffineOff_ObjBlend_8x32 gOamData_AffineOff_ObjBlend_16x32 gOamData_AffineOff_ObjBlend_32x64 gOamData_AffineNormal_ObjBlend_8x8 gOamData_AffineNormal_ObjBlend_16x16 gOamData_AffineNormal_ObjBlend_32x32 gOamData_AffineNormal_ObjBlend_64x64 gOamData_AffineNormal_ObjBlend_16x8 gOamData_AffineNormal_ObjBlend_32x8 gOamData_AffineNormal_ObjBlend_32x16 gOamData_AffineNormal_ObjBlend_64x32 gOamData_AffineNormal_ObjBlend_8x16 gOamData_AffineNormal_ObjBlend_8x32 gOamData_AffineNormal_ObjBlend_16x32 gOamData_AffineNormal_ObjBlend_32x64 gOamData_AffineDouble_ObjBlend_8x8 gOamData_AffineDouble_ObjBlend_16x16 gOamData_AffineDouble_ObjBlend_32x32 gOamData_AffineDouble_ObjBlend_64x64 gOamData_AffineDouble_ObjBlend_16x8 gOamData_AffineDouble_ObjBlend_32x8 gOamData_AffineDouble_ObjBlend_32x16 gOamData_AffineDouble_ObjBlend_64x32 gOamData_AffineDouble_ObjBlend_8x16 gOamData_AffineDouble_ObjBlend_8x32 gOamData_AffineDouble_ObjBlend_16x32 gOamData_AffineDouble_ObjBlend_32x64 gBattleAnimPicTable gBattleAnimPaletteTable gBattleAnimBackgroundTable sScriptCmdTable
#[allow(unused_imports)]
use crate::data::battle_anim::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleAnimScriptPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleAnimScriptRetAddr: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimScriptCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimFramesToWait: i8 = 0i8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimScriptActive: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimVisualTaskCount: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimSoundTaskCount: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimDisableStructPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimMoveDmg: i32 = 0i32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimMovePower: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimSpriteIndexArray: crate::ffi::Align4<[u8; 16]> =
    crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimFriendship: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gWeatherMoveAnim: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimArgs: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSoundAnimFramesToWait: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMonAnimTaskIdArray: crate::ffi::Align4<[u8; 2]> = crate::ffi::Align4([0; 2]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimMoveTurn: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimBackgroundFadeState: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimMoveIndex: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimAttacker: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimTarget: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimBattlerSpecies: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimCustomPanning: u8 = 0u8;

unsafe extern "C" {
    static mut gBattleAnims_Moves: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_BG2_Y: u8;
    static mut gBattle_WIN0H: u8;
    static mut gBattle_WIN0V: u8;
    static mut gBattle_WIN1H: u8;
    static mut gBattle_WIN1V: u8;
    static mut gBattlerAttacker: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBattlerTarget: u8;
    static mut gContestResources: u8;
    static mut gDecompressionBuffer: u8;
    static mut gEnemyParty: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMPlayInfo_SE1: u8;
    static mut gMPlayInfo_SE2: u8;
    static mut gMain: u8;
    static mut gMovesWithQuietBGM: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn BeginHardwarePaletteFade(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn ClearBattleAnimBg(a0: u32);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSpriteAndAnimate(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DrawBattlerOnBg(a0: i32, a1: u8, a2: u8, a3: u8, a4: u8, a5: *mut u8, a6: *mut u16, a7: u16);
    fn DrawMainBattleBackground();
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattleAnimBgData(a0: *mut u8, a1: u32);
    fn GetBattleBgPaletteNum() -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn InitPrioritiesForVisibleBattlers();
    fn IsBattlerSpritePresent(a0: u8) -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsSEPlaying() -> u8;
    fn IsSpeciesNotUnown(a0: u16) -> u8;
    fn LZDecompressVram(a0: *mut u32, a1: *mut u8);
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePaletteUsingHeap(a0: *mut u8) -> u8;
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut u8) -> u8;
    fn LoadContestBgAfterMoveAnim();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn RequestDma3Fill(a0: i32, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn SE12PanpotControl(a0: i8);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn UpdateOamPriorityInAllHealthboxes(a0: u8);
    fn m4aMPlayStop(a0: *mut u8);
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattleAnimationVars() {
    unsafe {
        let mut i: i32 = 0i32;
        ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(0i8);
        ((&raw mut gAnimScriptActive).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gAnimVisualTaskCount).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gAnimDisableStructPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((&raw mut gAnimMoveDmg).cast::<u8>().cast::<i32>()).write(0i32);
        ((&raw mut gAnimMovePower).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gAnimFriendship).cast::<u8>().cast::<u8>()).write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 8i32) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut gBattleAnimArgs).cast::<u8>().cast::<i16>()).cast::<i16>())
                        .wrapping_offset((i) as isize))
                    .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).write(255u8);
        ((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(255u8);
        ((&raw mut gAnimMoveTurn).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sAnimBackgroundFadeState)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        ((&raw mut sAnimMoveIndex).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gAnimCustomPanning).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMoveAnim(r#move: u16) {
    unsafe {
        let mut r#move = r#move;
        ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>())
            .write(((&raw mut gBattlerAttacker).cast::<u8>()).read());
        ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>())
            .write(((&raw mut gBattlerTarget).cast::<u8>()).read());
        LaunchBattleAnimation(
            ((&raw mut gBattleAnims_Moves).cast::<*mut u8>()).cast::<*mut u8>(),
            r#move,
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LaunchBattleAnimation(
    animsTable: *mut *mut u8,
    tableId: u16,
    isMoveAnim: u8,
) {
    unsafe {
        let mut animsTable = animsTable;
        let mut tableId = tableId;
        let mut isMoveAnim = isMoveAnim;
        let mut i: i32 = 0i32;
        if !((IsContest()) != 0) {
            InitPrioritiesForVisibleBattlers();
            UpdateOamPriorityInAllHealthboxes(0u8);
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((GetBattlerSide(((i) as u8))) as i32) != 0i32 {
                            ((((&raw mut gAnimBattlerSpecies).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 100,
                                    ),
                                    11i32,
                                )) as u16),
                            );
                        } else {
                            ((((&raw mut gAnimBattlerSpecies).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 100,
                                    ),
                                    11i32,
                                )) as u16),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((&raw mut gAnimBattlerSpecies).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(24)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u16>())
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if !((isMoveAnim) != 0) {
            ((&raw mut sAnimMoveIndex).cast::<u8>().cast::<u16>()).write(0u16);
        } else {
            ((&raw mut sAnimMoveIndex).cast::<u8>().cast::<u16>()).write(tableId);
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 8i32) {
                    break 'l5;
                }
                'l6: {
                    ((((&raw mut gBattleAnimArgs).cast::<u8>().cast::<i16>()).cast::<i16>())
                        .wrapping_offset((i) as isize))
                    .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).write(255u8);
        ((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(255u8);
        ((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(((animsTable).wrapping_offset(((tableId) as i32) as isize)).read());
        ((&raw mut gAnimScriptActive).cast::<u8>().cast::<u8>()).write(1u8);
        ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(0i8);
        ((&raw mut gAnimScriptCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(RunAnimScriptCommand));
        {
            i = 0i32;
            'l7: loop {
                if !(i < 8i32) {
                    break 'l7;
                }
                'l8: {
                    ((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        if (isMoveAnim) != 0 {
            {
                i = 0i32;
                'l9: loop {
                    if !(((((((&raw mut gMovesWithQuietBGM).cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 65535i32)
                    {
                        break 'l9;
                    }
                    'l10: {
                        if ((tableId) as i32)
                            == ((((((&raw mut gMovesWithQuietBGM).cast::<u16>()).cast::<u16>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            m4aMPlayVolumeControl(
                                (&raw mut gMPlayInfo_BGM).cast::<u8>(),
                                65535u16,
                                128u16,
                            );
                            break 'l9;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN1H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN1V).cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        FreeSpriteOamMatrix(sprite);
        DestroySprite(sprite);
        let __p1 = (&raw mut gAnimVisualTaskCount).cast::<u8>().cast::<u8>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimVisualTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
        let __p1 = (&raw mut gAnimVisualTaskCount).cast::<u8>().cast::<u8>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimSoundTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
        let __p1 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
    }
}
pub(crate) unsafe extern "C" fn AddSpriteIndex(index: u16) {
    unsafe {
        let mut index = index;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 65535i32
                    {
                        ((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(index);
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearSpriteIndex(index: u16) {
    unsafe {
        let mut index = index;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((index) as i32)
                    {
                        ((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(65535u16);
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WaitAnimFrameCount() {
    unsafe {
        if ((((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).read()) as i32) <= 0i32 {
            ((&raw mut gAnimScriptCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(RunAnimScriptCommand));
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(0i8);
        } else {
            let __p1 = (&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn RunAnimScriptCommand() {
    unsafe {
        'l1: loop {
            'l2: {
                (((((&raw const sScriptCmdTable)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    (((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .read()) as i32) as isize,
                ))
                .read())
                .unwrap_unchecked()();
            }
            if !((((((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).read()) as i32)
                == 0i32)
                && ((((&raw mut gAnimScriptActive).cast::<u8>().cast::<u8>()).read()) != 0))
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_loadspritegfx() {
    unsafe {
        let mut index: u16 = 0u16;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        index = (((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            | (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        LoadCompressedSpriteSheetUsingHeap(
            (((&raw const gBattleAnimPicTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((index) as i32).wrapping_sub(10000i32)) as isize * 8),
        );
        LoadCompressedSpritePaletteUsingHeap(
            (((&raw const gBattleAnimPaletteTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((index) as i32).wrapping_sub(10000i32)) as isize * 8),
        );
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(2));
        AddSpriteIndex(((((index) as i32).wrapping_sub(10000i32)) as u16));
        ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(1i8);
        ((&raw mut gAnimScriptCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(WaitAnimFrameCount));
    }
}
pub(crate) unsafe extern "C" fn Cmd_unloadspritegfx() {
    unsafe {
        let mut index: u16 = 0u16;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        index = (((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            | (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        FreeSpriteTilesByTag(
            (((((&raw const gBattleAnimPicTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((index) as i32).wrapping_sub(10000i32)) as isize * 8))
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        );
        FreeSpritePaletteByTag(
            (((((&raw const gBattleAnimPicTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((index) as i32).wrapping_sub(10000i32)) as isize * 8))
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        );
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(2));
        ClearSpriteIndex(((((index) as i32).wrapping_sub(10000i32)) as u16));
    }
}
pub(crate) unsafe extern "C" fn Cmd_createsprite() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut template: *mut u8 = core::ptr::null_mut();
        let mut argVar: u8 = 0u8;
        let mut argsCount: u8 = 0u8;
        let mut subpriority: i16 = 0i16;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        template = (((((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(1))
                .read()) as i32)
                    << 8),
            ))
        .wrapping_add(
            (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(2))
            .read()) as i32)
                << 16),
        ))
        .wrapping_add(
            (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(3))
            .read()) as i32)
                << 24),
        )) as usize as *mut u8);
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(4));
        argVar = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        let __p3 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(1));
        argsCount = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        let __p4 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p4).write(((__p4).read()).wrapping_offset(1));
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((argsCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gBattleAnimArgs).cast::<u8>().cast::<i16>()).cast::<i16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((((&raw mut sBattleAnimScriptPtr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .read()) as i32)
                            | (((((((&raw mut sBattleAnimScriptPtr)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8)) as i16),
                    );
                    let __p5 = (&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>();
                    (__p5).write(((__p5).read()).wrapping_offset(2));
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((argVar) as i32) & 128i32) != 0 {
            argVar = ((((argVar) as i32) ^ 128i32) as u8);
            if ((argVar) as i32) >= 64i32 {
                argVar = ((((argVar) as i32).wrapping_sub(64i32)) as u8);
            } else {
                argVar = ((((argVar) as i32).wrapping_mul((-1i32))) as u8);
            }
            subpriority = ((((GetBattlerSpriteSubpriority(
                ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read(),
            )) as i32)
                .wrapping_add((((argVar) as i8) as i32))) as i16);
        } else {
            if ((argVar) as i32) >= 64i32 {
                argVar = ((((argVar) as i32).wrapping_sub(64i32)) as u8);
            } else {
                argVar = ((((argVar) as i32).wrapping_mul((-1i32))) as u8);
            }
            subpriority = ((((GetBattlerSpriteSubpriority(
                ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read(),
            )) as i32)
                .wrapping_add((((argVar) as i8) as i32))) as i16);
        }
        if ((subpriority) as i32) < 3i32 {
            subpriority = 3i16;
        }
        CreateSpriteAndAnimate(
            template,
            ((GetBattlerSpriteCoord(
                ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read(),
                2u8,
            )) as i16),
            ((GetBattlerSpriteCoord(
                ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read(),
                3u8,
            )) as i16),
            ((subpriority) as u8),
        );
        let __p6 = (&raw mut gAnimVisualTaskCount).cast::<u8>().cast::<u8>();
        (__p6).write(((__p6).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_createvisualtask() {
    unsafe {
        let mut taskFunc: Option<unsafe extern "C" fn(u8)> = None;
        let mut taskPriority: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let mut numArgs: u8 = 0u8;
        let mut i: i32 = 0i32;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        taskFunc = (core::mem::transmute::<usize, Option<unsafe extern "C" fn(u8)>>(
            ((((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                .wrapping_add(
                    (((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8),
                ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(2))
                .read()) as i32)
                    << 16),
            ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(3))
                .read()) as i32)
                    << 24),
            )) as usize,
        ));
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(4));
        taskPriority = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        let __p3 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(1));
        numArgs = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        let __p4 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p4).write(((__p4).read()).wrapping_offset(1));
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((numArgs) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gBattleAnimArgs).cast::<u8>().cast::<i16>()).cast::<i16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((((&raw mut sBattleAnimScriptPtr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .read()) as i32)
                            | (((((((&raw mut sBattleAnimScriptPtr)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8)) as i16),
                    );
                    let __p5 = (&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>();
                    (__p5).write(((__p5).read()).wrapping_offset(2));
                }
                i = (i).wrapping_add(1);
            }
        }
        taskId = CreateTask(taskFunc, taskPriority);
        (taskFunc).unwrap_unchecked()(taskId);
        let __p6 = (&raw mut gAnimVisualTaskCount).cast::<u8>().cast::<u8>();
        (__p6).write(((__p6).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_delay() {
    unsafe {
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(
            (((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i8),
        );
        if ((((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).read()) as i32) == 0i32 {
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write((-1i8));
        }
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
        ((&raw mut gAnimScriptCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(WaitAnimFrameCount));
    }
}
pub(crate) unsafe extern "C" fn Cmd_waitforvisualfinish() {
    unsafe {
        if ((((&raw mut gAnimVisualTaskCount).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            let __p1 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(1));
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(0i8);
        } else {
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(1i8);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_nop() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_nop2() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn Cmd_end() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut continuousAnim: u32 = 0u32;
        if (((((((&raw mut gAnimVisualTaskCount).cast::<u8>().cast::<u8>()).read()) as i32)
            != 0i32)
            || (((((&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>()).read()) as i32)
                != 0i32))
            || ((((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).read()) as i32)
                != 255i32))
            || (((((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .read()) as i32)
                != 255i32)
        {
            ((&raw mut sSoundAnimFramesToWait).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(1i8);
            return;
        }
        if (IsSEPlaying()) != 0 {
            if (({
                let __p1 = (&raw mut sSoundAnimFramesToWait).cast::<u8>().cast::<u16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                <= 90i32
            {
                ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(1i8);
                return;
            } else {
                m4aMPlayStop((&raw mut gMPlayInfo_SE1).cast::<u8>());
                m4aMPlayStop((&raw mut gMPlayInfo_SE2).cast::<u8>());
            }
        }
        ((&raw mut sSoundAnimFramesToWait).cast::<u8>().cast::<u16>()).write(0u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 65535i32
                    {
                        FreeSpriteTilesByTag(
                            (((((&raw const gBattleAnimPicTable).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 8,
                            ))
                            .wrapping_add(6)
                            .cast::<u16>())
                            .read(),
                        );
                        FreeSpritePaletteByTag(
                            (((((&raw const gBattleAnimPicTable).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 8,
                            ))
                            .wrapping_add(6)
                            .cast::<u16>())
                            .read(),
                        );
                        ((((&raw mut sAnimSpriteIndexArray).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(65535u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((continuousAnim) != 0) {
            m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
            if !((IsContest()) != 0) {
                InitPrioritiesForVisibleBattlers();
                UpdateOamPriorityInAllHealthboxes(1u8);
            }
            ((&raw mut gAnimScriptActive).cast::<u8>().cast::<u8>()).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_playse() {
    unsafe {
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        PlaySE(
            (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                | (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(1))
                .read()) as i32)
                    << 8)) as u16),
        );
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Task_InitUpdateMonBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut updateTaskId: u8 = 0u8;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut battlerSpriteId: u8 = (((&raw mut gBattlerSpriteIds).cast::<u8>())
            .wrapping_offset((((data).read()) as i32) as isize))
        .read();
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        if !((((data).wrapping_offset(2)).read()) != 0) {
            DestroyAnimVisualTask(taskId);
            return;
        }
        updateTaskId = CreateTask(Some(Task_UpdateMonBg), 10u8);
        (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((updateTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .write(((battlerSpriteId) as i16));
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((updateTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((updateTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
        );
        if !((((data).wrapping_offset(1)).read()) != 0) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((updateTaskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((updateTaskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i16));
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((updateTaskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(((((&raw mut gBattle_BG2_X).cast::<u16>()).read()) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((updateTaskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(((((&raw mut gBattle_BG2_Y).cast::<u16>()).read()) as i16));
        }
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((updateTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((data).wrapping_offset(1)).read());
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((updateTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write((data).read());
        ((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(3)).read()) as i32) as isize))
        .write(updateTaskId);
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Cmd_monbg() {
    unsafe {
        let mut toBG_2: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut animBattler: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        animBattler = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        if (((animBattler) as i32) & 1i32) != 0 {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read();
        }
        if (IsBattlerSpriteVisible(battler)) != 0 {
            let mut position: u8 = GetBattlerPosition(battler);
            if ((((position) as i32) == 1i32) || (((position) as i32) == 2i32))
                || ((IsContest()) != 0)
            {
                toBG_2 = 0u8;
            } else {
                toBG_2 = 1u8;
            }
            MoveBattlerSpriteToBG(battler, toBG_2, 0u8);
            taskId = CreateTask(Some(Task_InitUpdateMonBg), 10u8);
            let __p2 = (&raw mut gAnimVisualTaskCount).cast::<u8>().cast::<u8>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(((battler) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((toBG_2) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(1i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(0i16);
        }
        battler = ((((battler) as i32) ^ 2i32) as u8);
        if (IsBattlerSpriteVisible(battler)) != 0 {
            let mut position: u8 = GetBattlerPosition(battler);
            if ((((position) as i32) == 1i32) || (((position) as i32) == 2i32))
                || ((IsContest()) != 0)
            {
                toBG_2 = 0u8;
            } else {
                toBG_2 = 1u8;
            }
            MoveBattlerSpriteToBG(battler, toBG_2, 0u8);
            taskId = CreateTask(Some(Task_InitUpdateMonBg), 10u8);
            let __p3 = (&raw mut gAnimVisualTaskCount).cast::<u8>().cast::<u8>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(((battler) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((toBG_2) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(1i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(1i16);
        }
        let __p4 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p4).write(((__p4).read()).wrapping_offset(1));
        ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(1i8);
        ((&raw mut gAnimScriptCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(WaitAnimFrameCount));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBattlerSpriteVisible(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        if (IsContest()) != 0 {
            if ((battler) as i32)
                == ((((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()) as i32)
            {
                return 1u8;
            } else {
                return 0u8;
            }
        }
        if !((IsBattlerSpritePresent(battler)) != 0) {
            return 0u8;
        }
        if (IsContest()) != 0 {
            return 1u8;
        }
        if (!((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(0),
            0,
            1,
            false,
        ) as u16)
            != 0))
            || (!((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16)
                != 0))
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveBattlerSpriteToBG(battler: u8, toBG_2: u8, setSpriteInvisible: u8) {
    unsafe {
        let mut battler = battler;
        let mut toBG_2 = toBG_2;
        let mut setSpriteInvisible = setSpriteInvisible;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let mut battlerSpriteId: u8 = 0u8;
        if !((toBG_2) != 0) {
            let mut battlerPosition: u8 = 0u8;
            if ((IsContest()) as i32) == 1i32 {
                RequestDma3Fill(0i32, ((100696064i32) as usize as *mut u8), 8192u16, 1u8);
                RequestDma3Fill(255i32, ((100724736i32) as usize as *mut u8), 4096u16, 0u8);
            } else {
                RequestDma3Fill(0i32, ((100679680i32) as usize as *mut u8), 8192u16, 1u8);
                RequestDma3Fill(255i32, ((100720640i32) as usize as *mut u8), 4096u16, 0u8);
            }
            GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
            'l1: loop {
                'l2: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (((&raw mut animBg).cast::<u8>()).cast::<*mut u8>()).read(),
                                    ((16777216i32
                                        | (crate::c::div_i32(
                                            4096i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
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
            'l5: loop {
                'l6: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(255u16);
                        'l7: loop {
                            'l8: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    ((((&raw mut animBg).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<*mut u16>())
                                    .read())
                                    .cast::<u8>(),
                                    ((16777216i32
                                        | (crate::c::div_i32(
                                            2048i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l5;
                }
            }
            SetAnimBgAttribute(1u8, 4u8, 2u8);
            SetAnimBgAttribute(1u8, 0u8, 1u8);
            SetAnimBgAttribute(1u8, 1u8, 0u8);
            battlerSpriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read();
            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
                ((((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .read()) as i32),
                    ))
                .wrapping_neg())
                .wrapping_add(32i32)) as u16),
            );
            if ((IsContest()) != 0)
                && ((IsSpeciesNotUnown(
                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u16>())
                    .read(),
                )) != 0)
            {
                let __p1 = (&raw mut gBattle_BG1_X).cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
                ((((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .read()) as i32),
                    ))
                .wrapping_neg())
                .wrapping_add(32i32)) as u16),
            );
            if (setSpriteInvisible) != 0 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
            }
            SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
            SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
            LoadPalette(
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    ((256i32).wrapping_add(((battler) as i32).wrapping_mul(16i32))) as isize,
                ))
                .cast::<u8>(),
                (((0i32).wrapping_add(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16),
                32u16,
            );
            'l9: loop {
                'l10: {
                    'l11: loop {
                        'l12: {
                            CpuSet(
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((256i32)
                                            .wrapping_add(((battler) as i32).wrapping_mul(16i32)))
                                            as isize,
                                    ))
                                .cast::<u8>(),
                                (((83886080u32).wrapping_add(
                                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read())
                                        as u32)
                                        .wrapping_mul(32u32),
                                )) as usize as *mut u8),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l9;
                }
            }
            if (IsContest()) != 0 {
                battlerPosition = 0u8;
            } else {
                battlerPosition = GetBattlerPosition(battler);
            }
            DrawBattlerOnBg(
                1i32,
                0u8,
                0u8,
                battlerPosition,
                (((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read(),
                (((&raw mut animBg).cast::<u8>()).cast::<*mut u8>()).read(),
                (((&raw mut animBg).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<*mut u16>())
                .read(),
                (((&raw mut animBg).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read(),
            );
            if (IsContest()) != 0 {
                FlipBattlerBgTiles();
            }
        } else {
            RequestDma3Fill(0i32, ((100687872i32) as usize as *mut u8), 8192u16, 1u8);
            RequestDma3Fill(0i32, ((100724736i32) as usize as *mut u8), 4096u16, 1u8);
            GetBattleAnimBgData((&raw mut animBg).cast::<u8>(), 2u32);
            'l13: loop {
                'l14: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l15: loop {
                            'l16: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    ((((&raw mut animBg).cast::<u8>()).cast::<*mut u8>()).read())
                                        .wrapping_offset(4096),
                                    ((16777216i32
                                        | (crate::c::div_i32(
                                            4096i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l15;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l13;
                }
            }
            'l17: loop {
                'l18: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l19: loop {
                            'l20: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (((((&raw mut animBg).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(1024))
                                    .cast::<u8>(),
                                    ((16777216i32
                                        | (crate::c::div_i32(
                                            2048i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l19;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l17;
                }
            }
            SetAnimBgAttribute(2u8, 4u8, 2u8);
            SetAnimBgAttribute(2u8, 0u8, 1u8);
            SetAnimBgAttribute(2u8, 1u8, 0u8);
            battlerSpriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read();
            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(
                ((((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .read()) as i32),
                    ))
                .wrapping_neg())
                .wrapping_add(32i32)) as u16),
            );
            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(
                ((((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .read()) as i32),
                    ))
                .wrapping_neg())
                .wrapping_add(32i32)) as u16),
            );
            if (setSpriteInvisible) != 0 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
            }
            SetGpuReg(24u8, ((&raw mut gBattle_BG2_X).cast::<u16>()).read());
            SetGpuReg(26u8, ((&raw mut gBattle_BG2_Y).cast::<u16>()).read());
            LoadPalette(
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    ((256i32).wrapping_add(((battler) as i32).wrapping_mul(16i32))) as isize,
                ))
                .cast::<u8>(),
                144u16,
                32u16,
            );
            'l21: loop {
                'l22: {
                    'l23: loop {
                        'l24: {
                            CpuSet(
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((256i32)
                                            .wrapping_add(((battler) as i32).wrapping_mul(16i32)))
                                            as isize,
                                    ))
                                .cast::<u8>(),
                                ((83886368u32) as usize as *mut u8),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l23;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l21;
                }
            }
            DrawBattlerOnBg(
                2i32,
                0u8,
                0u8,
                GetBattlerPosition(battler),
                (((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read(),
                ((((&raw mut animBg).cast::<u8>()).cast::<*mut u8>()).read()).wrapping_offset(4096),
                ((((&raw mut animBg).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<*mut u16>())
                .read())
                .wrapping_offset(1024),
                (((&raw mut animBg).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FlipBattlerBgTiles() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let mut ptr: *mut u16 = core::ptr::null_mut();
        if (IsSpeciesNotUnown(
            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .read(),
        )) != 0
        {
            GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
            ptr = (((&raw mut animBg).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u16>())
            .read();
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 4i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    let mut temp: u16 = 0u16;
                                    {
                                        temp = ((ptr).wrapping_offset(
                                            ((j).wrapping_add((i).wrapping_mul(32i32))) as isize,
                                        ))
                                        .read();
                                        ((ptr).wrapping_offset(
                                            ((j).wrapping_add((i).wrapping_mul(32i32))) as isize,
                                        ))
                                        .write(
                                            ((ptr).wrapping_offset(
                                                (((7i32).wrapping_sub(j))
                                                    .wrapping_add((i).wrapping_mul(32i32)))
                                                    as isize,
                                            ))
                                            .read(),
                                        );
                                        ((ptr).wrapping_offset(
                                            (((7i32).wrapping_sub(j))
                                                .wrapping_add((i).wrapping_mul(32i32)))
                                                as isize,
                                        ))
                                        .write(temp);
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l5: loop {
                    if !(i < 8i32) {
                        break 'l5;
                    }
                    'l6: {
                        {
                            j = 0i32;
                            'l7: loop {
                                if !(j < 8i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    let __p1 = (ptr).wrapping_offset(
                                        ((j).wrapping_add((i).wrapping_mul(32i32))) as isize,
                                    );
                                    (__p1).write((((((__p1).read()) as i32) ^ 1024i32) as u16));
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RelocateBattleBgPal(
    paletteNum: u16,
    dest: *mut u16,
    offset: u32,
    largeScreen: u8,
) {
    unsafe {
        let mut paletteNum = paletteNum;
        let mut dest = dest;
        let mut offset = offset;
        let mut largeScreen = largeScreen;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut size: i32 = 0i32;
        if !((largeScreen) != 0) {
            size = 32i32;
        } else {
            size = 64i32;
        }
        paletteNum = ((((paletteNum) as i32) << 12) as u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < size) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 32i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((dest).wrapping_offset(
                                    ((j).wrapping_add((i).wrapping_mul(32i32))) as isize,
                                ))
                                .write(
                                    (((((((((dest).wrapping_offset(
                                        ((j).wrapping_add((i).wrapping_mul(32i32))) as isize,
                                    ))
                                    .read()) as i32)
                                        & 4095i32)
                                        | ((paletteNum) as i32))
                                        as u32)
                                        .wrapping_add(offset))
                                        as u16),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBattleAnimBg(toBG2: u8) {
    unsafe {
        let mut toBG2 = toBG2;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
        if (!((toBG2) != 0)) || ((IsContest()) != 0) {
            ClearBattleAnimBg(1u32);
            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        } else {
            ClearBattleAnimBg(2u32);
            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateMonBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        spriteId = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        battler = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .read()) as u8);
        GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
        x = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            .wrapping_sub(
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .read()) as i32),
                    ),
            )) as i16);
        y = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            .wrapping_sub(
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .read()) as i32),
                    ),
            )) as i16);
        if !((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read())
            != 0)
        {
            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
                ((((x) as i32).wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32),
                )) as u16),
            );
            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
                ((((y) as i32).wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32),
                )) as u16),
            );
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(((256i32).wrapping_add(((((battler) as i32))).wrapping_mul(16i32))) as isize)).cast::<u8>(), ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(((0i32).wrapping_add((((((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32))).wrapping_mul(16i32))) as isize)).cast::<u8>(), (67108864u32 | (crate::c::div_u32(32u32, (((crate::c::div_i32(32i32, 8i32)) as u32))) & 2097151u32)));
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
        } else {
            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(
                ((((x) as i32).wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32),
                )) as u16),
            );
            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(
                ((((y) as i32).wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32),
                )) as u16),
            );
            'l5: loop {
                'l6: {
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((256i32)
                                            .wrapping_add(((battler) as i32).wrapping_mul(16i32)))
                                            as isize,
                                    ))
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(144))
                                .cast::<u8>(),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l5;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_clearmonbg() {
    unsafe {
        let mut animBattlerId: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        animBattlerId = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        if ((animBattlerId) as i32) == 0i32 {
            animBattlerId = 2u8;
        } else {
            if ((animBattlerId) as i32) == 1i32 {
                animBattlerId = 3u8;
            }
        }
        if (((animBattlerId) as i32) == 0i32) || (((animBattlerId) as i32) == 2i32) {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read();
        }
        if (((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).read()) as i32) != 255i32
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
        if (((animBattlerId) as i32) > 1i32)
            && (((((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .read()) as i32)
                != 255i32)
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset((((battler) as i32) ^ 2i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        } else {
            animBattlerId = 0u8;
        }
        taskId = CreateTask(Some(Task_ClearMonBg), 5u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((animBattlerId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((battler) as i16));
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Task_ClearMonBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            != 1i32
        {
            let mut to_BG2: u8 = 0u8;
            let mut position: u8 = GetBattlerPosition(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u8),
            );
            if ((((position) as i32) == 1i32) || (((position) as i32) == 2i32))
                || ((IsContest()) != 0)
            {
                to_BG2 = 0u8;
            } else {
                to_BG2 = 1u8;
            }
            if (((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).read()) as i32)
                != 255i32
            {
                ResetBattleAnimBg(to_BG2);
                DestroyTask((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).read());
                (((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).write(255u8);
            }
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                > 1i32
            {
                ResetBattleAnimBg(((((to_BG2) as i32) ^ 1i32) as u8));
                DestroyTask(
                    ((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1))
                    .read(),
                );
                ((((&raw mut sMonAnimTaskIdArray).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                    .write(255u8);
            }
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_monbg_static() {
    unsafe {
        let mut toBG_2: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut animBattlerId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        animBattlerId = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        if ((animBattlerId) as i32) == 0i32 {
            animBattlerId = 2u8;
        } else {
            if ((animBattlerId) as i32) == 1i32 {
                animBattlerId = 3u8;
            }
        }
        if (((animBattlerId) as i32) == 0i32) || (((animBattlerId) as i32) == 2i32) {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read();
        }
        if (IsBattlerSpriteVisible(battler)) != 0 {
            let mut position: u8 = GetBattlerPosition(battler);
            if ((((position) as i32) == 1i32) || (((position) as i32) == 2i32))
                || ((IsContest()) != 0)
            {
                toBG_2 = 0u8;
            } else {
                toBG_2 = 1u8;
            }
            MoveBattlerSpriteToBG(battler, toBG_2, 0u8);
        }
        battler = ((((battler) as i32) ^ 2i32) as u8);
        if (((animBattlerId) as i32) > 1i32) && ((IsBattlerSpriteVisible(battler)) != 0) {
            let mut position: u8 = GetBattlerPosition(battler);
            if ((((position) as i32) == 1i32) || (((position) as i32) == 2i32))
                || ((IsContest()) != 0)
            {
                toBG_2 = 0u8;
            } else {
                toBG_2 = 1u8;
            }
            MoveBattlerSpriteToBG(battler, toBG_2, 0u8);
        }
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_clearmonbg_static() {
    unsafe {
        let mut animBattlerId: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        animBattlerId = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        if ((animBattlerId) as i32) == 0i32 {
            animBattlerId = 2u8;
        } else {
            if ((animBattlerId) as i32) == 1i32 {
                animBattlerId = 3u8;
            }
        }
        if (((animBattlerId) as i32) == 0i32) || (((animBattlerId) as i32) == 2i32) {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read();
        }
        if (IsBattlerSpriteVisible(battler)) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
        if (((animBattlerId) as i32) > 1i32)
            && ((IsBattlerSpriteVisible(((((battler) as i32) ^ 2i32) as u8))) != 0)
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset((((battler) as i32) ^ 2i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        } else {
            animBattlerId = 0u8;
        }
        taskId = CreateTask(Some(Task_ClearMonBgStatic), 5u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((animBattlerId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((battler) as i16));
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Task_ClearMonBgStatic(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            != 1i32
        {
            let mut toBG_2: u8 = 0u8;
            let mut battler: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u8);
            let mut position: u8 = GetBattlerPosition(battler);
            if ((((position) as i32) == 1i32) || (((position) as i32) == 2i32))
                || ((IsContest()) != 0)
            {
                toBG_2 = 0u8;
            } else {
                toBG_2 = 1u8;
            }
            if (IsBattlerSpriteVisible(battler)) != 0 {
                ResetBattleAnimBg(toBG_2);
            }
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                > 1i32)
                && ((IsBattlerSpriteVisible(((((battler) as i32) ^ 2i32) as u8))) != 0)
            {
                ResetBattleAnimBg(((((toBG_2) as i32) ^ 1i32) as u8));
            }
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_setalpha() {
    unsafe {
        let mut half1: u16 = 0u16;
        let mut half2: u16 = 0u16;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        half1 = ((({
            let __p4 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            let __t5 = (__p4).read();
            (__p4).write(((__p4).read()).wrapping_offset(1));
            __t5
        })
        .read()) as u16);
        half2 = ((((({
            let __p8 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            let __t9 = (__p8).read();
            (__p8).write(((__p8).read()).wrapping_offset(1));
            __t9
        })
        .read()) as i32)
            << 8) as u16);
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, ((((half1) as i32) | ((half2) as i32)) as u16));
    }
}
pub(crate) unsafe extern "C" fn Cmd_setbldcnt() {
    unsafe {
        let mut half1: u16 = 0u16;
        let mut half2: u16 = 0u16;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        half1 = ((({
            let __p4 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            let __t5 = (__p4).read();
            (__p4).write(((__p4).read()).wrapping_offset(1));
            __t5
        })
        .read()) as u16);
        half2 = ((((({
            let __p8 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            let __t9 = (__p8).read();
            (__p8).write(((__p8).read()).wrapping_offset(1));
            __t9
        })
        .read()) as i32)
            << 8) as u16);
        SetGpuReg(80u8, ((((half1) as i32) | ((half2) as i32)) as u16));
    }
}
pub(crate) unsafe extern "C" fn Cmd_blendoff() {
    unsafe {
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn Cmd_call() {
    unsafe {
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        ((&raw mut sBattleAnimScriptRetAddr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(
            (((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(4),
        );
        ((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(
            (((((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                .wrapping_add(
                    (((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8),
                ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(2))
                .read()) as i32)
                    << 16),
            ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(3))
                .read()) as i32)
                    << 24),
            )) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Cmd_return() {
    unsafe {
        ((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(
            ((&raw mut sBattleAnimScriptRetAddr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn Cmd_setarg() {
    unsafe {
        let mut addr: *mut u8 = ((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read();
        let mut value: u16 = 0u16;
        let mut argId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        argId = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
        value = (((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            | (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        ((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write((addr).wrapping_offset(4));
        ((((&raw mut gBattleAnimArgs).cast::<u8>().cast::<i16>()).cast::<i16>())
            .wrapping_offset(((argId) as i32) as isize))
        .write(((value) as i16));
    }
}
pub(crate) unsafe extern "C" fn Cmd_choosetwoturnanim() {
    unsafe {
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        if (((((&raw mut gAnimMoveTurn).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32) != 0 {
            let __p2 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            (__p2).write(((__p2).read()).wrapping_offset(4));
        }
        ((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(
            (((((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                .wrapping_add(
                    (((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8),
                ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(2))
                .read()) as i32)
                    << 16),
            ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(3))
                .read()) as i32)
                    << 24),
            )) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifmoveturn() {
    unsafe {
        let mut toCheck: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        toCheck = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
        if ((toCheck) as i32)
            == ((((&raw mut gAnimMoveTurn).cast::<u8>().cast::<u8>()).read()) as i32)
        {
            ((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(
                (((((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .read()) as i32)
                    .wrapping_add(
                        (((((((&raw mut sBattleAnimScriptPtr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(1))
                        .read()) as i32)
                            << 8),
                    ))
                .wrapping_add(
                    (((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16),
                ))
                .wrapping_add(
                    (((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24),
                )) as usize as *mut u8),
            );
        } else {
            let __p3 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            (__p3).write(((__p3).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_goto() {
    unsafe {
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        ((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(
            (((((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                .wrapping_add(
                    (((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8),
                ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(2))
                .read()) as i32)
                    << 16),
            ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(3))
                .read()) as i32)
                    << 24),
            )) as usize as *mut u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsContest() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_fadetobg() {
    unsafe {
        let mut backgroundId: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        backgroundId = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
        taskId = CreateTask(Some(Task_FadeToBg), 5u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((backgroundId) as i16));
        ((&raw mut sAnimBackgroundFadeState)
            .cast::<u8>()
            .cast::<u8>())
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_fadetobgfromset() {
    unsafe {
        let mut bg1: u8 = 0u8;
        let mut bg2: u8 = 0u8;
        let mut bg3: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        bg1 = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        bg2 = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(1))
        .read();
        bg3 = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(2))
        .read();
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(3));
        taskId = CreateTask(Some(Task_FadeToBg), 5u8);
        if (IsContest()) != 0 {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(((bg3) as i16));
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read()))
                as i32)
                == 0i32
            {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((bg2) as i16));
            } else {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((bg1) as i16));
            }
        }
        ((&raw mut sAnimBackgroundFadeState)
            .cast::<u8>()
            .cast::<u8>())
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Task_FadeToBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as i32)
            == 0i32
        {
            BeginHardwarePaletteFade(232u8, 0u8, 0u8, 16u8, 0u8);
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            (__p1).write(((__p1).read()).wrapping_add(1));
            return;
        }
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0
        {
            return;
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as i32)
            == 1i32
        {
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((&raw mut sAnimBackgroundFadeState)
                .cast::<u8>()
                .cast::<u8>())
            .write(2u8);
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                == 2i32
            {
                let mut bgId: i16 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read();
                if ((bgId) as i32) == (-1i32) {
                    LoadDefaultBg();
                } else {
                    LoadMoveBg(((bgId) as u16));
                }
                BeginHardwarePaletteFade(232u8, 0u8, 16u8, 0u8, 1u8);
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p3).write(((__p3).read()).wrapping_add(1));
                return;
            }
        }
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0
        {
            return;
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as i32)
            == 3i32
        {
            DestroyTask(taskId);
            ((&raw mut sAnimBackgroundFadeState)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadMoveBg(bgId: u16) {
    unsafe {
        let mut bgId = bgId;
        if (IsContest()) != 0 {
            let mut tilemap: *mut u32 = (((((&raw const gBattleAnimBackgroundTable)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((bgId) as i32) as isize * 12))
            .wrapping_add(8)
            .cast::<*mut u32>())
            .read();
            let mut dmaSrc: *mut u8 = core::ptr::null_mut();
            let mut dmaDest: *mut u8 = core::ptr::null_mut();
            LZDecompressWram(tilemap, (&raw mut gDecompressionBuffer).cast::<u8>());
            RelocateBattleBgPal(
                ((GetBattleBgPaletteNum()) as u16),
                ((&raw mut gDecompressionBuffer).cast::<u8>()).cast::<u16>(),
                256u32,
                0u8,
            );
            dmaSrc = (&raw mut gDecompressionBuffer).cast::<u8>();
            dmaDest = ((100716544i32) as usize as *mut u8);
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((dmaSrc) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    ((dmaDest) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2080374784i32)
                                        | crate::c::div_i32(
                                            2048i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
            LZDecompressVram(
                (((((&raw const gBattleAnimBackgroundTable)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((bgId) as i32) as isize * 12))
                .cast::<*mut u32>())
                .read(),
                ((100671488i32) as usize as *mut u8),
            );
            LoadCompressedPalette(
                (((((&raw const gBattleAnimBackgroundTable)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((bgId) as i32) as isize * 12))
                .wrapping_add(4)
                .cast::<*mut u32>())
                .read(),
                (((0i32).wrapping_add(((GetBattleBgPaletteNum()) as i32).wrapping_mul(16i32)))
                    as u16),
                32u16,
            );
        } else {
            LZDecompressVram(
                (((((&raw const gBattleAnimBackgroundTable)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((bgId) as i32) as isize * 12))
                .wrapping_add(8)
                .cast::<*mut u32>())
                .read(),
                ((100716544i32) as usize as *mut u8),
            );
            LZDecompressVram(
                (((((&raw const gBattleAnimBackgroundTable)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((bgId) as i32) as isize * 12))
                .cast::<*mut u32>())
                .read(),
                ((100696064i32) as usize as *mut u8),
            );
            LoadCompressedPalette(
                (((((&raw const gBattleAnimBackgroundTable)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((bgId) as i32) as isize * 12))
                .wrapping_add(4)
                .cast::<*mut u32>())
                .read(),
                32u16,
                32u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn LoadDefaultBg() {
    unsafe {
        if (IsContest()) != 0 {
            LoadContestBgAfterMoveAnim();
        } else {
            DrawMainBattleBackground();
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_restorebg() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        taskId = CreateTask(Some(Task_FadeToBg), 5u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write((-1i16));
        ((&raw mut sAnimBackgroundFadeState)
            .cast::<u8>()
            .cast::<u8>())
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_waitbgfadeout() {
    unsafe {
        if ((((&raw mut sAnimBackgroundFadeState)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 2i32
        {
            let __p1 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(1));
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(0i8);
        } else {
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(1i8);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_waitbgfadein() {
    unsafe {
        if ((((&raw mut sAnimBackgroundFadeState)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 0i32
        {
            let __p1 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            (__p1).write(((__p1).read()).wrapping_offset(1));
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(0i8);
        } else {
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(1i8);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_changebg() {
    unsafe {
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        LoadMoveBg(
            (((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as u16),
        );
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAnimAdjustPanning(pan: i8) -> i8 {
    unsafe {
        let mut pan = pan;
        if (!((IsContest()) != 0))
            && ((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 12,
                ))
                .wrapping_add(0),
                4,
                1,
                false,
            ) as u8)
                != 0)
        {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()))
                as i32)
                != 0i32
            {
                pan = 63i8;
            } else {
                pan = (-64i8);
            }
        } else {
            if (IsContest()) != 0 {
                if ((((((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()) as i32)
                    != ((((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read()) as i32))
                    || (((((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read())
                        as i32)
                        != 2i32))
                    || (((pan) as i32) != 63i32)
                {
                    pan = ((((pan) as i32).wrapping_mul((-1i32))) as i8);
                }
            } else {
                if ((GetBattlerSide(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read(),
                )) as i32)
                    == 0i32
                {
                    if ((GetBattlerSide(
                        ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read(),
                    )) as i32)
                        == 0i32
                    {
                        if ((pan) as i32) == 63i32 {
                            pan = (-64i8);
                        } else {
                            if ((pan) as i32) != (-64i32) {
                                pan = ((((pan) as i32).wrapping_mul((-1i32))) as i8);
                            }
                        }
                    }
                } else {
                    if ((GetBattlerSide(
                        ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read(),
                    )) as i32)
                        == 1i32
                    {
                        if ((pan) as i32) == (-64i32) {
                            pan = 63i8;
                        }
                    } else {
                        pan = ((((pan) as i32).wrapping_mul((-1i32))) as i8);
                    }
                }
            }
        }
        if ((pan) as i32) > 63i32 {
            pan = 63i8;
        } else {
            if ((pan) as i32) < (-64i32) {
                pan = (-64i8);
            }
        }
        return pan;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAnimAdjustPanning2(pan: i8) -> i8 {
    unsafe {
        let mut pan = pan;
        if (!((IsContest()) != 0))
            && ((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 12,
                ))
                .wrapping_add(0),
                4,
                1,
                false,
            ) as u8)
                != 0)
        {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()))
                as i32)
                != 0i32
            {
                pan = 63i8;
            } else {
                pan = (-64i8);
            }
        } else {
            if (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()))
                as i32)
                != 0i32)
                || ((IsContest()) != 0)
            {
                pan = ((((pan) as i32).wrapping_neg()) as i8);
            }
        }
        return pan;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn KeepPanInRange(panArg: i16, oldPan: i32) -> i16 {
    unsafe {
        let mut panArg = panArg;
        let mut oldPan = oldPan;
        let mut pan: i16 = panArg;
        if ((pan) as i32) > 63i32 {
            pan = 63i16;
        } else {
            if ((pan) as i32) < (-64i32) {
                pan = (-64i16);
            }
        }
        return pan;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculatePanIncrement(
    sourcePan: i16,
    targetPan: i16,
    incrementPan: i16,
) -> i16 {
    unsafe {
        let mut sourcePan = sourcePan;
        let mut targetPan = targetPan;
        let mut incrementPan = incrementPan;
        let mut ret: i16 = 0i16;
        if ((sourcePan) as i32) < ((targetPan) as i32) {
            ret = ((if ((incrementPan) as i32) < 0i32 {
                ((incrementPan) as i32).wrapping_neg()
            } else {
                ((incrementPan) as i32)
            }) as i16);
        } else {
            if ((sourcePan) as i32) > ((targetPan) as i32) {
                ret = (((if ((incrementPan) as i32) < 0i32 {
                    ((incrementPan) as i32).wrapping_neg()
                } else {
                    ((incrementPan) as i32)
                })
                .wrapping_neg()) as i16);
            } else {
                ret = 0i16;
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn Cmd_playsewithpan() {
    unsafe {
        let mut songId: u16 = 0u16;
        let mut pan: i8 = 0i8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        songId = (((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            | (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        pan = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(2))
        .read()) as i8);
        PlaySE12WithPanning(songId, BattleAnimAdjustPanning(pan));
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(3));
    }
}
pub(crate) unsafe extern "C" fn Cmd_setpan() {
    unsafe {
        let mut pan: i8 = 0i8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        pan = (((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i8);
        SE12PanpotControl(BattleAnimAdjustPanning(pan));
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_panse() {
    unsafe {
        let mut songNum: u16 = 0u16;
        let mut currentPanArg: i8 = 0i8;
        let mut incrementPan: i8 = 0i8;
        let mut incrementPanArg: i8 = 0i8;
        let mut currentPan: i8 = 0i8;
        let mut targetPan: i8 = 0i8;
        let mut framesToWait: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        songNum = (((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            | (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        currentPanArg = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(2))
        .read()) as i8);
        incrementPan = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(3))
        .read()) as i8);
        incrementPanArg = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(4))
        .read()) as i8);
        framesToWait = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(5))
        .read();
        currentPan = BattleAnimAdjustPanning(currentPanArg);
        targetPan = BattleAnimAdjustPanning(incrementPan);
        incrementPan = ((CalculatePanIncrement(
            ((currentPan) as i16),
            ((targetPan) as i16),
            ((incrementPanArg) as i16),
        )) as i8);
        taskId = CreateTask(Some(Task_PanFromInitialToTarget), 1u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((currentPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((targetPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((incrementPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((framesToWait) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((currentPan) as i16));
        PlaySE12WithPanning(songNum, currentPan);
        let __p2 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        let __p3 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(6));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_PanFromInitialToTarget(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut destroyTask: u32 = 0u32;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            >= ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
        {
            let mut pan: i16 = 0i16;
            let mut initialPanning: i16 = 0i16;
            let mut targetPanning: i16 = 0i16;
            let mut currentPan: i16 = 0i16;
            let mut incrementPan: i16 = 0i16;
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write(0i16);
            initialPanning = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read();
            targetPanning = ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read();
            currentPan = ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read();
            incrementPan = ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read();
            pan = ((((currentPan) as i32).wrapping_add(((incrementPan) as i32))) as i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(pan);
            if ((incrementPan) as i32) == 0i32 {
                destroyTask = 1u32;
            } else {
                if ((initialPanning) as i32) < ((targetPanning) as i32) {
                    if ((pan) as i32) >= ((targetPanning) as i32) {
                        destroyTask = 1u32;
                    }
                } else {
                    if ((pan) as i32) <= ((targetPanning) as i32) {
                        destroyTask = 1u32;
                    }
                }
            }
            if (destroyTask) != 0 {
                pan = targetPanning;
                DestroyTask(taskId);
                let __p3 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
                (__p3).write(((__p3).read()).wrapping_sub(1));
            }
            SE12PanpotControl(((pan) as i8));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_panse_adjustnone() {
    unsafe {
        let mut songId: u16 = 0u16;
        let mut currentPan: i8 = 0i8;
        let mut targetPan: i8 = 0i8;
        let mut incrementPan: i8 = 0i8;
        let mut framesToWait: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        songId = (((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            | (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        currentPan = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(2))
        .read()) as i8);
        targetPan = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(3))
        .read()) as i8);
        incrementPan = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(4))
        .read()) as i8);
        framesToWait = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(5))
        .read();
        taskId = CreateTask(Some(Task_PanFromInitialToTarget), 1u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((currentPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((targetPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((incrementPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((framesToWait) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((currentPan) as i16));
        PlaySE12WithPanning(songId, currentPan);
        let __p2 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        let __p3 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(6));
    }
}
pub(crate) unsafe extern "C" fn Cmd_panse_adjustall() {
    unsafe {
        let mut songId: u16 = 0u16;
        let mut targetPanArg: i8 = 0i8;
        let mut incrementPanArg: i8 = 0i8;
        let mut currentPanArg: i8 = 0i8;
        let mut currentPan: i8 = 0i8;
        let mut targetPan: i8 = 0i8;
        let mut incrementPan: i8 = 0i8;
        let mut framesToWait: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        songId = (((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            | (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        currentPanArg = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(2))
        .read()) as i8);
        targetPanArg = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(3))
        .read()) as i8);
        incrementPanArg = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(4))
        .read()) as i8);
        framesToWait = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(5))
        .read();
        currentPan = BattleAnimAdjustPanning2(currentPanArg);
        targetPan = BattleAnimAdjustPanning2(targetPanArg);
        incrementPan = BattleAnimAdjustPanning2(incrementPanArg);
        taskId = CreateTask(Some(Task_PanFromInitialToTarget), 1u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((currentPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((targetPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((incrementPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((framesToWait) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((currentPan) as i16));
        PlaySE12WithPanning(songId, currentPan);
        let __p2 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        let __p3 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(6));
    }
}
pub(crate) unsafe extern "C" fn Cmd_loopsewithpan() {
    unsafe {
        let mut songId: u16 = 0u16;
        let mut panningArg: i8 = 0i8;
        let mut panning: i8 = 0i8;
        let mut framesToWait: u8 = 0u8;
        let mut numberOfPlays: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        songId = (((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            | (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        panningArg = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(2))
        .read()) as i8);
        framesToWait = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(3))
        .read();
        numberOfPlays = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(4))
        .read();
        panning = BattleAnimAdjustPanning(panningArg);
        taskId = CreateTask(Some(Task_LoopAndPlaySE), 1u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((songId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((panning) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((framesToWait) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((numberOfPlays) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(((framesToWait) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
        let __p2 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        let __p3 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(5));
    }
}
pub(crate) unsafe extern "C" fn Task_LoopAndPlaySE(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            >= ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
        {
            let mut songId: u16 = 0u16;
            let mut panning: i8 = 0i8;
            let mut numberOfPlays: u8 = 0u8;
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write(0i16);
            songId = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u16);
            panning = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i8);
            numberOfPlays = (({
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3);
                let __t4 = ((__p3).read()).wrapping_sub(1);
                (__p3).write(__t4);
                __t4
            }) as u8);
            PlaySE12WithPanning(songId, panning);
            if ((numberOfPlays) as i32) == 0i32 {
                DestroyTask(taskId);
                let __p5 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
                (__p5).write(((__p5).read()).wrapping_sub(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_waitplaysewithpan() {
    unsafe {
        let mut songId: u16 = 0u16;
        let mut panningArg: i8 = 0i8;
        let mut panning: i8 = 0i8;
        let mut framesToWait: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        songId = (((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read()) as i32)
            | (((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as u16);
        panningArg = ((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(2))
        .read()) as i8);
        framesToWait = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(3))
        .read();
        panning = BattleAnimAdjustPanning(panningArg);
        taskId = CreateTask(Some(Task_WaitAndPlaySE), 1u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((songId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((panning) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((framesToWait) as i16));
        let __p2 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        let __p3 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(4));
    }
}
pub(crate) unsafe extern "C" fn Task_WaitAndPlaySE(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            <= 0i32
        {
            PlaySE12WithPanning(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i8),
            );
            DestroyTask(taskId);
            let __p3 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_createsoundtask() {
    unsafe {
        let mut func: Option<unsafe extern "C" fn(u8)> = None;
        let mut numArgs: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let mut i: i32 = 0i32;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        func = (core::mem::transmute::<usize, Option<unsafe extern "C" fn(u8)>>(
            ((((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                .wrapping_add(
                    (((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8),
                ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(2))
                .read()) as i32)
                    << 16),
            ))
            .wrapping_add(
                (((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(3))
                .read()) as i32)
                    << 24),
            )) as usize,
        ));
        let __p2 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(4));
        numArgs = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        let __p3 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(1));
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((numArgs) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gBattleAnimArgs).cast::<u8>().cast::<i16>()).cast::<i16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((((&raw mut sBattleAnimScriptPtr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .read()) as i32)
                            | (((((((&raw mut sBattleAnimScriptPtr)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8)) as i16),
                    );
                    let __p4 = (&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>();
                    (__p4).write(((__p4).read()).wrapping_offset(2));
                }
                i = (i).wrapping_add(1);
            }
        }
        taskId = CreateTask(func, 1u8);
        (func).unwrap_unchecked()(taskId);
        let __p5 = (&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>();
        (__p5).write(((__p5).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Cmd_waitsound() {
    unsafe {
        if ((((&raw mut gAnimSoundTaskCount).cast::<u8>().cast::<u8>()).read()) as i32) != 0i32 {
            ((&raw mut sSoundAnimFramesToWait).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(1i8);
        } else {
            if (IsSEPlaying()) != 0 {
                if (({
                    let __p1 = (&raw mut sSoundAnimFramesToWait).cast::<u8>().cast::<u16>();
                    let __t2 = ((__p1).read()).wrapping_add(1);
                    (__p1).write(__t2);
                    __t2
                }) as i32)
                    > 90i32
                {
                    m4aMPlayStop((&raw mut gMPlayInfo_SE1).cast::<u8>());
                    m4aMPlayStop((&raw mut gMPlayInfo_SE2).cast::<u8>());
                    ((&raw mut sSoundAnimFramesToWait).cast::<u8>().cast::<u16>()).write(0u16);
                } else {
                    ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(1i8);
                }
            } else {
                ((&raw mut sSoundAnimFramesToWait).cast::<u8>().cast::<u16>()).write(0u16);
                let __p3 = (&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>();
                (__p3).write(((__p3).read()).wrapping_offset(1));
                ((&raw mut sAnimFramesToWait).cast::<u8>().cast::<i8>()).write(0i8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpargeq() {
    unsafe {
        let mut argId: u8 = 0u8;
        let mut valueToCheck: i16 = 0i16;
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        argId = (((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .read();
        valueToCheck = ((((((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .wrapping_offset(1))
            .read()) as i32)
                << 8)) as i16);
        if ((valueToCheck) as i32)
            == ((((((&raw mut gBattleAnimArgs).cast::<u8>().cast::<i16>()).cast::<i16>())
                .wrapping_offset(((argId) as i32) as isize))
            .read()) as i32)
        {
            ((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(
                ((((((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(3))
                .read()) as i32)
                    .wrapping_add(
                        ((((((((&raw mut sBattleAnimScriptPtr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(3))
                        .wrapping_offset(1))
                        .read()) as i32)
                            << 8),
                    ))
                .wrapping_add(
                    ((((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(3))
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16),
                ))
                .wrapping_add(
                    ((((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(3))
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24),
                )) as usize as *mut u8),
            );
        } else {
            let __p2 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            (__p2).write(((__p2).read()).wrapping_offset(7));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifcontest() {
    unsafe {
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        if (IsContest()) != 0 {
            ((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(
                (((((((((&raw mut sBattleAnimScriptPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .read()) as i32)
                    .wrapping_add(
                        (((((((&raw mut sBattleAnimScriptPtr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(1))
                        .read()) as i32)
                            << 8),
                    ))
                .wrapping_add(
                    (((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16),
                ))
                .wrapping_add(
                    (((((((&raw mut sBattleAnimScriptPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24),
                )) as usize as *mut u8),
            );
        } else {
            let __p2 = (&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>();
            (__p2).write(((__p2).read()).wrapping_offset(4));
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_splitbgprio() {
    unsafe {
        let mut wantedBattler: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut battlerPosition: u8 = 0u8;
        wantedBattler = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(1))
        .read();
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
        if ((wantedBattler) as i32) != 0i32 {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read();
        }
        battlerPosition = GetBattlerPosition(battler);
        if (!((IsContest()) != 0))
            && ((((battlerPosition) as i32) == 0i32) || (((battlerPosition) as i32) == 3i32))
        {
            SetAnimBgAttribute(1u8, 4u8, 1u8);
            SetAnimBgAttribute(2u8, 4u8, 2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_splitbgprio_all() {
    unsafe {
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        if !((IsContest()) != 0) {
            SetAnimBgAttribute(1u8, 4u8, 1u8);
            SetAnimBgAttribute(2u8, 4u8, 2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_splitbgprio_foes() {
    unsafe {
        let mut wantedBattler: u8 = 0u8;
        let mut battlerPosition: u8 = 0u8;
        let mut battler: u8 = 0u8;
        wantedBattler = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(1))
        .read();
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()))
            as i32)
            != ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read()))
                as i32)
        {
            if ((wantedBattler) as i32) != 0i32 {
                battler = ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read();
            } else {
                battler = ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read();
            }
            battlerPosition = GetBattlerPosition(battler);
            if (!((IsContest()) != 0))
                && ((((battlerPosition) as i32) == 0i32) || (((battlerPosition) as i32) == 3i32))
            {
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                SetAnimBgAttribute(2u8, 4u8, 2u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_invisible() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        spriteId = GetAnimBattlerSpriteId(
            ((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read(),
        );
        if ((spriteId) as i32) != 255i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_visible() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        spriteId = GetAnimBattlerSpriteId(
            ((((&raw mut sBattleAnimScriptPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1))
            .read(),
        );
        if ((spriteId) as i32) != 255i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
    }
}
pub(crate) unsafe extern "C" fn Cmd_teamattack_moveback() {
    unsafe {
        let mut wantedBattler: u8 = 0u8;
        let mut priorityRank: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        wantedBattler = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(1))
        .read();
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
        if ((!((IsContest()) != 0)) && ((IsDoubleBattle()) != 0))
            && (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()))
                as i32)
                == ((GetBattlerSide(
                    ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read(),
                )) as i32))
        {
            if ((wantedBattler) as i32) == 0i32 {
                priorityRank = GetBattlerSpriteBGPriorityRank(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read(),
                );
                spriteId = GetAnimBattlerSpriteId(0u8);
            } else {
                priorityRank = GetBattlerSpriteBGPriorityRank(
                    ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read(),
                );
                spriteId = GetAnimBattlerSpriteId(1u8);
            }
            if ((spriteId) as i32) != 255i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                if ((priorityRank) as i32) == 2i32 {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        (3u16) as i32,
                    );
                }
                if ((priorityRank) as i32) == 1i32 {
                    ResetBattleAnimBg(0u8);
                } else {
                    ResetBattleAnimBg(1u8);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_teamattack_movefwd() {
    unsafe {
        let mut wantedBattler: u8 = 0u8;
        let mut priorityRank: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        wantedBattler = ((((&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(1))
        .read();
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(2));
        if ((!((IsContest()) != 0)) && ((IsDoubleBattle()) != 0))
            && (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read()))
                as i32)
                == ((GetBattlerSide(
                    ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read(),
                )) as i32))
        {
            if ((wantedBattler) as i32) == 0i32 {
                priorityRank = GetBattlerSpriteBGPriorityRank(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>().cast::<u8>()).read(),
                );
                spriteId = GetAnimBattlerSpriteId(0u8);
            } else {
                priorityRank = GetBattlerSpriteBGPriorityRank(
                    ((&raw mut gBattleAnimTarget).cast::<u8>().cast::<u8>()).read(),
                );
                spriteId = GetAnimBattlerSpriteId(1u8);
            }
            if (((spriteId) as i32) != 255i32) && (((priorityRank) as i32) == 2i32) {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    (2u16) as i32,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_stopsound() {
    unsafe {
        m4aMPlayStop((&raw mut gMPlayInfo_SE1).cast::<u8>());
        m4aMPlayStop((&raw mut gMPlayInfo_SE2).cast::<u8>());
        let __p1 = (&raw mut sBattleAnimScriptPtr)
            .cast::<u8>()
            .cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
