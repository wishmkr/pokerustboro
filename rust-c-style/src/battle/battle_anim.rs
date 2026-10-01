//! Translated from `src/battle_anim.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gOamData_AffineOff_ObjNormal_8x8 gOamData_AffineOff_ObjNormal_16x16 gOamData_AffineOff_ObjNormal_32x32 gOamData_AffineOff_ObjNormal_64x64 gOamData_AffineOff_ObjNormal_16x8 gOamData_AffineOff_ObjNormal_32x8 gOamData_AffineOff_ObjNormal_32x16 gOamData_AffineOff_ObjNormal_64x32 gOamData_AffineOff_ObjNormal_8x16 gOamData_AffineOff_ObjNormal_8x32 gOamData_AffineOff_ObjNormal_16x32 gOamData_AffineOff_ObjNormal_32x64 gOamData_AffineNormal_ObjNormal_8x8 gOamData_AffineNormal_ObjNormal_16x16 gOamData_AffineNormal_ObjNormal_32x32 gOamData_AffineNormal_ObjNormal_64x64 gOamData_AffineNormal_ObjNormal_16x8 gOamData_AffineNormal_ObjNormal_32x8 gOamData_AffineNormal_ObjNormal_32x16 gOamData_AffineNormal_ObjNormal_64x32 gOamData_AffineNormal_ObjNormal_8x16 gOamData_AffineNormal_ObjNormal_8x32 gOamData_AffineNormal_ObjNormal_16x32 gOamData_AffineNormal_ObjNormal_32x64 gOamData_AffineDouble_ObjNormal_8x8 gOamData_AffineDouble_ObjNormal_16x16 gOamData_AffineDouble_ObjNormal_32x32 gOamData_AffineDouble_ObjNormal_64x64 gOamData_AffineDouble_ObjNormal_16x8 gOamData_AffineDouble_ObjNormal_32x8 gOamData_AffineDouble_ObjNormal_32x16 gOamData_AffineDouble_ObjNormal_64x32 gOamData_AffineDouble_ObjNormal_8x16 gOamData_AffineDouble_ObjNormal_8x32 gOamData_AffineDouble_ObjNormal_16x32 gOamData_AffineDouble_ObjNormal_32x64 gOamData_AffineOff_ObjBlend_8x8 gOamData_AffineOff_ObjBlend_16x16 gOamData_AffineOff_ObjBlend_32x32 gOamData_AffineOff_ObjBlend_64x64 gOamData_AffineOff_ObjBlend_16x8 gOamData_AffineOff_ObjBlend_32x8 gOamData_AffineOff_ObjBlend_32x16 gOamData_AffineOff_ObjBlend_64x32 gOamData_AffineOff_ObjBlend_8x16 gOamData_AffineOff_ObjBlend_8x32 gOamData_AffineOff_ObjBlend_16x32 gOamData_AffineOff_ObjBlend_32x64 gOamData_AffineNormal_ObjBlend_8x8 gOamData_AffineNormal_ObjBlend_16x16 gOamData_AffineNormal_ObjBlend_32x32 gOamData_AffineNormal_ObjBlend_64x64 gOamData_AffineNormal_ObjBlend_16x8 gOamData_AffineNormal_ObjBlend_32x8 gOamData_AffineNormal_ObjBlend_32x16 gOamData_AffineNormal_ObjBlend_64x32 gOamData_AffineNormal_ObjBlend_8x16 gOamData_AffineNormal_ObjBlend_8x32 gOamData_AffineNormal_ObjBlend_16x32 gOamData_AffineNormal_ObjBlend_32x64 gOamData_AffineDouble_ObjBlend_8x8 gOamData_AffineDouble_ObjBlend_16x16 gOamData_AffineDouble_ObjBlend_32x32 gOamData_AffineDouble_ObjBlend_64x64 gOamData_AffineDouble_ObjBlend_16x8 gOamData_AffineDouble_ObjBlend_32x8 gOamData_AffineDouble_ObjBlend_32x16 gOamData_AffineDouble_ObjBlend_64x32 gOamData_AffineDouble_ObjBlend_8x16 gOamData_AffineDouble_ObjBlend_8x32 gOamData_AffineDouble_ObjBlend_16x32 gOamData_AffineDouble_ObjBlend_32x64 gBattleAnimPicTable gBattleAnimPaletteTable gBattleAnimBackgroundTable sScriptCmdTable

const ANIM_SPRITE_INDEX_COUNT: i32 = 8;

static gBattleAnimBackgroundTable: Table<CArray<BattleAnimBackground, 27>> =
    Table((&raw const crate::data::battle_anim::gBattleAnimBackgroundTable).cast());
static gBattleAnimPaletteTable: Table<CArray<CompressedSpritePalette, 289>> =
    Table((&raw const crate::data::battle_anim::gBattleAnimPaletteTable).cast());
static gBattleAnimPicTable: Table<CArray<CompressedSpriteSheet, 289>> =
    Table((&raw const crate::data::battle_anim::gBattleAnimPicTable).cast());
static sScriptCmdTable: Table<CArray<Option<unsafe extern "C" fn()>, 48>> =
    Table((&raw const crate::data::battle_anim::sScriptCmdTable).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleAnimScriptPtr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleAnimScriptRetAddr: *mut u8 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimScriptCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimFramesToWait: i8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimScriptActive: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimVisualTaskCount: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimSoundTaskCount: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimDisableStructPtr: *mut DisableStruct = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimMoveDmg: i32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimMovePower: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimSpriteIndexArray: Aligned<CArray<u16, 8>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimFriendship: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gWeatherMoveAnim: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimArgs: Aligned<CArray<i16, 8>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSoundAnimFramesToWait: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMonAnimTaskIdArray: Aligned<CArray<u8, 2>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimMoveTurn: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimBackgroundFadeState: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimMoveIndex: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimAttacker: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimTarget: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimBattlerSpecies: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAnimCustomPanning: u8 = 0;

unsafe extern "C" {
    static gBattleAnims_Moves: CArray<*mut u8, 0>;
    static mut gBattleSpritesDataPtr: *mut BattleSpriteData;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattle_BG2_Y: u16;
    static mut gBattle_WIN0H: u16;
    static mut gBattle_WIN0V: u16;
    static mut gBattle_WIN1H: u16;
    static mut gBattle_WIN1V: u16;
    static mut gBattlerAttacker: u8;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gBattlerTarget: u8;
    static mut gContestResources: *mut ContestResources;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMPlayInfo_SE1: MusicPlayerInfo;
    static mut gMPlayInfo_SE2: MusicPlayerInfo;
    static mut gMain: Main;
    static gMovesWithQuietBGM: CArray<u16, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn BeginHardwarePaletteFade(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn ClearBattleAnimBg(a0: u32);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSpriteAndAnimate(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DrawBattlerOnBg(a0: i32, a1: u8, a2: u8, a3: u8, a4: u8, a5: *mut u8, a6: *mut u16, a7: u16);
    fn DrawMainBattleBackground();
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut BattleAnimBgData);
    fn GetBattleAnimBgData(a0: *mut BattleAnimBgData, a1: u32);
    fn GetBattleBgPaletteNum() -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn InitPrioritiesForVisibleBattlers();
    fn IsBattlerSpritePresent(a0: u8) -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsSEPlaying() -> u8;
    fn IsSpeciesNotUnown(a0: u16) -> u8;
    fn LZDecompressVram(a0: *mut u32, a1: *mut c_void);
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePaletteUsingHeap(a0: *mut CompressedSpritePalette) -> u8;
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8;
    fn LoadContestBgAfterMoveAnim();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn RequestDma3Fill(a0: i32, a1: *mut c_void, a2: u16, a3: u8) -> i16;
    fn SE12PanpotControl(a0: i8);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn UpdateOamPriorityInAllHealthboxes(a0: u8);
    fn m4aMPlayStop(a0: *mut MusicPlayerInfo);
    fn m4aMPlayVolumeControl(a0: *mut MusicPlayerInfo, a1: u16, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattleAnimationVars() {
    let mut i: i32 = 0;
    sAnimFramesToWait = 0;
    gAnimScriptActive = FALSE;
    gAnimVisualTaskCount = 0;
    gAnimSoundTaskCount = 0;
    gAnimDisableStructPtr = null_mut();
    gAnimMoveDmg = 0;
    gAnimMovePower = 0;
    gAnimFriendship = 0;
    i = 0;
    while i < ANIM_SPRITE_INDEX_COUNT {
        sAnimSpriteIndexArray[i] = 0xFFFF;
        i += 1;
    }
    i = 0;
    while i < ANIM_ARGS_COUNT {
        gBattleAnimArgs[i] = 0;
        i += 1;
    }
    sMonAnimTaskIdArray[0] = TASK_NONE;
    sMonAnimTaskIdArray[1] = TASK_NONE;
    gAnimMoveTurn = 0;
    sAnimBackgroundFadeState = 0;
    sAnimMoveIndex = 0;
    gBattleAnimAttacker = 0;
    gBattleAnimTarget = 0;
    gAnimCustomPanning = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMoveAnim(r#move: u16) {
    gBattleAnimAttacker = gBattlerAttacker;
    gBattleAnimTarget = gBattlerTarget;
    LaunchBattleAnimation(gBattleAnims_Moves.as_ptr().cast_mut(), r#move, TRUE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LaunchBattleAnimation(
    animsTable: *mut *mut u8,
    tableId: u16,
    isMoveAnim: u8,
) {
    let mut i: i32 = 0;
    if IsContest() == 0 {
        InitPrioritiesForVisibleBattlers();
        UpdateOamPriorityInAllHealthboxes(0);
        i = 0;
        while i < MAX_BATTLERS_COUNT as i32 {
            if GetBattlerSide(i as u8) != B_SIDE_PLAYER {
                gAnimBattlerSpecies[i] = GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[i]],
                    MON_DATA_SPECIES,
                ) as u16;
            } else {
                gAnimBattlerSpecies[i] = GetMonData2(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[i]],
                    MON_DATA_SPECIES,
                ) as u16;
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < CONTESTANT_COUNT {
            gAnimBattlerSpecies[i] = (*(*gContestResources).moveAnim).species;
            i += 1;
        }
    }
    if isMoveAnim == 0 {
        sAnimMoveIndex = 0;
    } else {
        sAnimMoveIndex = tableId;
    }
    i = 0;
    while i < ANIM_ARGS_COUNT {
        gBattleAnimArgs[i] = 0;
        i += 1;
    }
    sMonAnimTaskIdArray[0] = TASK_NONE;
    sMonAnimTaskIdArray[1] = TASK_NONE;
    sBattleAnimScriptPtr = *animsTable.at(tableId);
    gAnimScriptActive = TRUE;
    sAnimFramesToWait = 0;
    gAnimScriptCallback = Some(RunAnimScriptCommand);
    i = 0;
    while i < ANIM_SPRITE_INDEX_COUNT {
        sAnimSpriteIndexArray[i] = 0xFFFF;
        i += 1;
    }
    if isMoveAnim != 0 {
        i = 0;
        while gMovesWithQuietBGM[i] != 0xFFFF {
            if tableId == gMovesWithQuietBGM[i] {
                m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 128);
                break;
            }
            i += 1;
        }
    }
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    gBattle_WIN1H = 0;
    gBattle_WIN1V = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimSprite(sprite: *mut Sprite) {
    FreeSpriteOamMatrix(sprite);
    DestroySprite(sprite);
    gAnimVisualTaskCount -= 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimVisualTask(taskId: u8) {
    DestroyTask(taskId);
    gAnimVisualTaskCount -= 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimSoundTask(taskId: u8) {
    DestroyTask(taskId);
    gAnimSoundTaskCount -= 1;
}
pub(crate) unsafe extern "C" fn AddSpriteIndex(index: u16) {
    let mut i: i32 = 0;
    i = 0;
    while i < ANIM_SPRITE_INDEX_COUNT {
        if sAnimSpriteIndexArray[i] == 0xFFFF {
            sAnimSpriteIndexArray[i] = index;
            return;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ClearSpriteIndex(index: u16) {
    let mut i: i32 = 0;
    i = 0;
    while i < ANIM_SPRITE_INDEX_COUNT {
        if sAnimSpriteIndexArray[i] == index {
            sAnimSpriteIndexArray[i] = 0xFFFF;
            return;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn WaitAnimFrameCount() {
    if sAnimFramesToWait <= 0 {
        gAnimScriptCallback = Some(RunAnimScriptCommand);
        sAnimFramesToWait = 0;
    } else {
        sAnimFramesToWait -= 1;
    }
}
pub(crate) unsafe extern "C" fn RunAnimScriptCommand() {
    loop {
        sScriptCmdTable[*sBattleAnimScriptPtr].unwrap_unchecked()();
        if !(sAnimFramesToWait == 0 && gAnimScriptActive != 0) {
            break;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_loadspritegfx() {
    let mut index: u16 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    index = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    LoadCompressedSpriteSheetUsingHeap(
        (&raw const gBattleAnimPicTable[index as i32 - 10000]).cast_mut(),
    );
    LoadCompressedSpritePaletteUsingHeap(
        (&raw const gBattleAnimPaletteTable[index as i32 - 10000]).cast_mut(),
    );
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    AddSpriteIndex(index - 10000);
    sAnimFramesToWait = 1;
    gAnimScriptCallback = Some(WaitAnimFrameCount);
}
pub(crate) unsafe extern "C" fn Cmd_unloadspritegfx() {
    let mut index: u16 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    index = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    FreeSpriteTilesByTag(gBattleAnimPicTable[index as i32 - 10000].tag);
    FreeSpritePaletteByTag(gBattleAnimPicTable[index as i32 - 10000].tag);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    ClearSpriteIndex(index - 10000);
}
pub(crate) unsafe extern "C" fn Cmd_createsprite() {
    let mut i: i32 = 0;
    let mut template: *mut SpriteTemplate = null_mut();
    let mut argVar: u8 = 0;
    let mut argsCount: u8 = 0;
    let mut subpriority: i16 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    template = (*sBattleAnimScriptPtr as i32
        + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
        + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
        + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
        as *mut SpriteTemplate;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    argVar = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    argsCount = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    i = 0;
    while i < argsCount as i32 {
        gBattleAnimArgs[i] =
            *sBattleAnimScriptPtr as i16 | (*sBattleAnimScriptPtr.at(1) as i16) << 8;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
        i += 1;
    }
    if argVar as i32 & ANIMSPRITE_IS_TARGET != 0 {
        argVar ^= ANIMSPRITE_IS_TARGET as u8;
        if argVar >= 64 {
            argVar -= 64;
        } else {
            argVar *= 255;
        }
        subpriority = GetBattlerSpriteSubpriority(gBattleAnimTarget) as i16 + argVar as i8 as i16;
    } else {
        if argVar >= 64 {
            argVar -= 64;
        } else {
            argVar *= 255;
        }
        subpriority = GetBattlerSpriteSubpriority(gBattleAnimAttacker) as i16 + argVar as i8 as i16;
    }
    if subpriority < 3 {
        subpriority = 3;
    }
    CreateSpriteAndAnimate(
        template,
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16,
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16,
        subpriority as u8,
    );
    gAnimVisualTaskCount += 1;
}
pub(crate) unsafe extern "C" fn Cmd_createvisualtask() {
    let mut taskFunc: Option<unsafe extern "C" fn(u8)> = None;
    let mut taskPriority: u8 = 0;
    let mut taskId: u8 = 0;
    let mut numArgs: u8 = 0;
    let mut i: i32 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    taskFunc = core::mem::transmute::<usize, Option<unsafe extern "C" fn(u8)>>(
        (*sBattleAnimScriptPtr as i32
            + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize,
    );
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    taskPriority = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    numArgs = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    i = 0;
    while i < numArgs as i32 {
        gBattleAnimArgs[i] =
            *sBattleAnimScriptPtr as i16 | (*sBattleAnimScriptPtr.at(1) as i16) << 8;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
        i += 1;
    }
    taskId = CreateTask(taskFunc, taskPriority);
    taskFunc.unwrap_unchecked()(taskId);
    gAnimVisualTaskCount += 1;
}
pub(crate) unsafe extern "C" fn Cmd_delay() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    sAnimFramesToWait = *sBattleAnimScriptPtr as i8;
    if sAnimFramesToWait == 0 {
        sAnimFramesToWait = -1;
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    gAnimScriptCallback = Some(WaitAnimFrameCount);
}
pub(crate) unsafe extern "C" fn Cmd_waitforvisualfinish() {
    if gAnimVisualTaskCount == 0 {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        sAnimFramesToWait = 0;
    } else {
        sAnimFramesToWait = 1;
    }
}
pub(crate) unsafe extern "C" fn Cmd_nop() {}
pub(crate) unsafe extern "C" fn Cmd_nop2() {}
pub(crate) unsafe extern "C" fn Cmd_end() {
    let mut i: i32 = 0;
    let mut continuousAnim: u32 = FALSE as u32;
    if gAnimVisualTaskCount != 0
        || gAnimSoundTaskCount != 0
        || sMonAnimTaskIdArray[0] != TASK_NONE
        || sMonAnimTaskIdArray[1] != TASK_NONE
    {
        sSoundAnimFramesToWait = 0;
        sAnimFramesToWait = 1;
        return;
    }
    if IsSEPlaying() != 0 {
        if ({
            sSoundAnimFramesToWait += 1;
            sSoundAnimFramesToWait
        }) <= 90
        {
            sAnimFramesToWait = 1;
            return;
        } else {
            m4aMPlayStop(&raw mut gMPlayInfo_SE1);
            m4aMPlayStop(&raw mut gMPlayInfo_SE2);
        }
    }
    sSoundAnimFramesToWait = 0;
    i = 0;
    while i < ANIM_SPRITE_INDEX_COUNT {
        if sAnimSpriteIndexArray[i] != 0xFFFF {
            FreeSpriteTilesByTag(gBattleAnimPicTable[sAnimSpriteIndexArray[i]].tag);
            FreeSpritePaletteByTag(gBattleAnimPicTable[sAnimSpriteIndexArray[i]].tag);
            sAnimSpriteIndexArray[i] = 0xFFFF;
        }
        i += 1;
    }
    if continuousAnim == 0 {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 256);
        if IsContest() == 0 {
            InitPrioritiesForVisibleBattlers();
            UpdateOamPriorityInAllHealthboxes(1);
        }
        gAnimScriptActive = FALSE;
    }
}
pub(crate) unsafe extern "C" fn Cmd_playse() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    PlaySE(*sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Task_InitUpdateMonBg(taskId: u8) {
    let mut updateTaskId: u8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut battlerSpriteId: u8 = gBattlerSpriteIds[*data];
    gSprites[battlerSpriteId].set_invisible(TRUE as u16);
    if *data.at(2) == 0 {
        DestroyAnimVisualTask(taskId);
        return;
    }
    updateTaskId = CreateTask(Some(Task_UpdateMonBg), 10);
    gTasks[updateTaskId].data[0] = battlerSpriteId as i16;
    gTasks[updateTaskId].data[1] = gSprites[battlerSpriteId].x + gSprites[battlerSpriteId].x2;
    gTasks[updateTaskId].data[2] = gSprites[battlerSpriteId].y + gSprites[battlerSpriteId].y2;
    if *data.at(1) == 0 {
        gTasks[updateTaskId].data[3] = gBattle_BG1_X as i16;
        gTasks[updateTaskId].data[4] = gBattle_BG1_Y as i16;
    } else {
        gTasks[updateTaskId].data[3] = gBattle_BG2_X as i16;
        gTasks[updateTaskId].data[4] = gBattle_BG2_Y as i16;
    }
    gTasks[updateTaskId].data[5] = *data.at(1);
    gTasks[updateTaskId].data[6] = *data;
    sMonAnimTaskIdArray[*data.at(3)] = updateTaskId;
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe extern "C" fn Cmd_monbg() {
    let mut toBG_2: u8 = 0;
    let mut taskId: u8 = 0;
    let mut battler: u8 = 0;
    let mut animBattler: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    animBattler = *sBattleAnimScriptPtr;
    if animBattler as i32 & ANIM_TARGET as i32 != 0 {
        battler = gBattleAnimTarget;
    } else {
        battler = gBattleAnimAttacker;
    }
    if IsBattlerSpriteVisible(battler) != 0 {
        let mut position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        MoveBattlerSpriteToBG(battler, toBG_2, FALSE);
        taskId = CreateTask(Some(Task_InitUpdateMonBg), 10);
        gAnimVisualTaskCount += 1;
        gTasks[taskId].data[0] = battler as i16;
        gTasks[taskId].data[1] = toBG_2 as i16;
        gTasks[taskId].data[2] = TRUE as i16;
        gTasks[taskId].data[3] = FALSE as i16;
    }
    battler ^= BIT_FLANK;
    if IsBattlerSpriteVisible(battler) != 0 {
        let mut position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        MoveBattlerSpriteToBG(battler, toBG_2, FALSE);
        taskId = CreateTask(Some(Task_InitUpdateMonBg), 10);
        gAnimVisualTaskCount += 1;
        gTasks[taskId].data[0] = battler as i16;
        gTasks[taskId].data[1] = toBG_2 as i16;
        gTasks[taskId].data[2] = TRUE as i16;
        gTasks[taskId].data[3] = TRUE as i16;
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    sAnimFramesToWait = 1;
    gAnimScriptCallback = Some(WaitAnimFrameCount);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBattlerSpriteVisible(battler: u8) -> u8 {
    if IsContest() != 0 {
        if battler == gBattleAnimAttacker {
            return TRUE;
        } else {
            return FALSE;
        }
    }
    if IsBattlerSpritePresent(battler) == 0 {
        return FALSE;
    }
    if IsContest() != 0 {
        return TRUE;
    }
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).invisible() == 0
        || gSprites[gBattlerSpriteIds[battler]].invisible() == 0
    {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveBattlerSpriteToBG(battler: u8, toBG_2: u8, setSpriteInvisible: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    let mut battlerSpriteId: u8 = 0;
    if toBG_2 == 0 {
        let mut battlerPosition: u8 = 0;
        if IsContest() == TRUE {
            RequestDma3Fill(0, 0x6008000 as usize as *mut c_void, 0x2000, 1);
            RequestDma3Fill(0xFF, 0x600f000 as usize as *mut c_void, 0x1000, 0);
        } else {
            RequestDma3Fill(0, 0x6004000 as usize as *mut c_void, 0x2000, 1);
            RequestDma3Fill(0xFF, 0x600e000 as usize as *mut c_void, 0x1000, 0);
        }
        GetBattleAnimBg1Data(&raw mut animBg);
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    animBg.bgTiles as *mut c_void,
                    0x1000800,
                );
            }
        }
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 255);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    animBg.bgTilemap as *mut c_void,
                    0x1000400,
                );
            }
        }
        SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 2);
        SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 1);
        SetAnimBgAttribute(1, BG_ANIM_AREA_OVERFLOW_MODE, 0);
        battlerSpriteId = gBattlerSpriteIds[battler];
        gBattle_BG1_X = (gSprites[battlerSpriteId].x as u16 + gSprites[battlerSpriteId].x2 as u16)
            .wrapping_neg()
            + 0x20;
        if IsContest() != 0 && IsSpeciesNotUnown((*(*gContestResources).moveAnim).species) != 0 {
            gBattle_BG1_X -= 1;
        }
        gBattle_BG1_Y = (gSprites[battlerSpriteId].y as u16 + gSprites[battlerSpriteId].y2 as u16)
            .wrapping_neg()
            + 0x20;
        if setSpriteInvisible != 0 {
            gSprites[gBattlerSpriteIds[battler]].set_invisible(TRUE as u16);
        }
        SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
        SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
        LoadPalette(
            &raw mut gPlttBufferUnfaded[0x100 + battler as i32 * 16] as *mut c_void,
            0x000 + animBg.paletteId as u16 * 16,
            32,
        );
        CpuSet(
            &raw mut gPlttBufferUnfaded[0x100 + battler as i32 * 16] as *mut c_void,
            (BG_PLTT + animBg.paletteId as u32 * 32) as usize as *mut c_void,
            0x4000008,
        );
        if IsContest() != 0 {
            battlerPosition = 0;
        } else {
            battlerPosition = GetBattlerPosition(battler);
        }
        DrawBattlerOnBg(
            1,
            0,
            0,
            battlerPosition,
            animBg.paletteId,
            animBg.bgTiles,
            animBg.bgTilemap,
            animBg.tilesOffset,
        );
        if IsContest() != 0 {
            FlipBattlerBgTiles();
        }
    } else {
        RequestDma3Fill(0, 0x6006000 as usize as *mut c_void, 0x2000, 1);
        RequestDma3Fill(0, 0x600f000 as usize as *mut c_void, 0x1000, 1);
        GetBattleAnimBgData(&raw mut animBg, 2);
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    animBg.bgTiles.at(4096) as *mut c_void,
                    0x1000800,
                );
            }
        }
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    animBg.bgTilemap.at(1024) as *mut c_void,
                    0x1000400,
                );
            }
        }
        SetAnimBgAttribute(2, BG_ANIM_PRIORITY, 2);
        SetAnimBgAttribute(2, BG_ANIM_SCREEN_SIZE, 1);
        SetAnimBgAttribute(2, BG_ANIM_AREA_OVERFLOW_MODE, 0);
        battlerSpriteId = gBattlerSpriteIds[battler];
        gBattle_BG2_X = (gSprites[battlerSpriteId].x as u16 + gSprites[battlerSpriteId].x2 as u16)
            .wrapping_neg()
            + 0x20;
        gBattle_BG2_Y = (gSprites[battlerSpriteId].y as u16 + gSprites[battlerSpriteId].y2 as u16)
            .wrapping_neg()
            + 0x20;
        if setSpriteInvisible != 0 {
            gSprites[gBattlerSpriteIds[battler]].set_invisible(TRUE as u16);
        }
        SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X);
        SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y);
        LoadPalette(
            &raw mut gPlttBufferUnfaded[0x100 + battler as i32 * 16] as *mut c_void,
            144,
            32,
        );
        CpuSet(
            &raw mut gPlttBufferUnfaded[0x100 + battler as i32 * 16] as *mut c_void,
            0x5000120 as usize as *mut c_void,
            0x4000008,
        );
        DrawBattlerOnBg(
            2,
            0,
            0,
            GetBattlerPosition(battler),
            animBg.paletteId,
            animBg.bgTiles.at(4096),
            animBg.bgTilemap.at(1024),
            animBg.tilesOffset,
        );
    }
}
pub(crate) unsafe extern "C" fn FlipBattlerBgTiles() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut animBg: BattleAnimBgData = zeroed();
    let mut ptr: *mut u16 = null_mut();
    if IsSpeciesNotUnown((*(*gContestResources).moveAnim).species) != 0 {
        GetBattleAnimBg1Data(&raw mut animBg);
        ptr = animBg.bgTilemap;
        i = 0;
        while i < 8 {
            j = 0;
            while j < 4 {
                let mut temp: u16 = 0;
                temp = *ptr.at(j + i * 32);
                *ptr.at(j + i * 32) = *ptr.at(7 - j + i * 32);
                *ptr.at(7 - j + i * 32) = temp;
                j += 1;
            }
            i += 1;
        }
        i = 0;
        while i < 8 {
            j = 0;
            while j < 8 {
                *ptr.at(j + i * 32) ^= 0x400;
                j += 1;
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RelocateBattleBgPal(
    mut paletteNum: u16,
    mut dest: *mut u16,
    offset: u32,
    largeScreen: u8,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut size: i32 = 0;
    if largeScreen == 0 {
        size = 32;
    } else {
        size = 64;
    }
    paletteNum <<= 12;
    i = 0;
    while i < size {
        j = 0;
        while j < 32 {
            *dest.at(j + i * 32) = (*dest.at(j + i * 32) & 0xFFF | paletteNum) + offset as u16;
            j += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBattleAnimBg(toBG2: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    GetBattleAnimBg1Data(&raw mut animBg);
    if toBG2 == 0 || IsContest() != 0 {
        ClearBattleAnimBg(1);
        gBattle_BG1_X = 0;
        gBattle_BG1_Y = 0;
    } else {
        ClearBattleAnimBg(2);
        gBattle_BG2_X = 0;
        gBattle_BG2_Y = 0;
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateMonBg(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut battler: u8 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut animBg: BattleAnimBgData = zeroed();
    spriteId = gTasks[taskId].data[0] as u8;
    battler = gTasks[taskId].data[6] as u8;
    GetBattleAnimBg1Data(&raw mut animBg);
    x = gTasks[taskId].data[1] - (gSprites[spriteId].x + gSprites[spriteId].x2);
    y = gTasks[taskId].data[2] - (gSprites[spriteId].y + gSprites[spriteId].y2);
    if gTasks[taskId].data[5] == 0 {
        gBattle_BG1_X = x as u16 + gTasks[taskId].data[3] as u16;
        gBattle_BG1_Y = y as u16 + gTasks[taskId].data[4] as u16;
        CpuSet(
            &raw mut gPlttBufferFaded[0x100 + battler as i32 * 16] as *mut c_void,
            &raw mut gPlttBufferFaded[0x000 + animBg.paletteId as i32 * 16] as *mut c_void,
            0x4000008,
        );
    } else {
        gBattle_BG2_X = x as u16 + gTasks[taskId].data[3] as u16;
        gBattle_BG2_Y = y as u16 + gTasks[taskId].data[4] as u16;
        CpuSet(
            &raw mut gPlttBufferFaded[0x100 + battler as i32 * 16] as *mut c_void,
            &raw mut gPlttBufferFaded[144] as *mut c_void,
            0x4000008,
        );
    }
}
pub(crate) unsafe extern "C" fn Cmd_clearmonbg() {
    let mut animBattlerId: u8 = 0;
    let mut battler: u8 = 0;
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    animBattlerId = *sBattleAnimScriptPtr;
    if animBattlerId == ANIM_ATTACKER {
        animBattlerId = ANIM_ATK_PARTNER;
    } else if animBattlerId == ANIM_TARGET {
        animBattlerId = ANIM_DEF_PARTNER;
    }
    if animBattlerId == ANIM_ATTACKER || animBattlerId == ANIM_ATK_PARTNER {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if sMonAnimTaskIdArray[0] != TASK_NONE {
        gSprites[gBattlerSpriteIds[battler]].set_invisible(FALSE as u16);
    }
    if animBattlerId > 1 && sMonAnimTaskIdArray[1] != TASK_NONE {
        gSprites[gBattlerSpriteIds[battler as i32 ^ 2]].set_invisible(FALSE as u16);
    } else {
        animBattlerId = 0;
    }
    taskId = CreateTask(Some(Task_ClearMonBg), 5);
    gTasks[taskId].data[0] = animBattlerId as i16;
    gTasks[taskId].data[2] = battler as i16;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Task_ClearMonBg(taskId: u8) {
    gTasks[taskId].data[1] += 1;
    if gTasks[taskId].data[1] != 1 {
        let mut to_BG2: u8 = 0;
        let mut position: u8 = GetBattlerPosition(gTasks[taskId].data[2] as u8);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            to_BG2 = FALSE;
        } else {
            to_BG2 = TRUE;
        }
        if sMonAnimTaskIdArray[0] != TASK_NONE {
            ResetBattleAnimBg(to_BG2);
            DestroyTask(sMonAnimTaskIdArray[0]);
            sMonAnimTaskIdArray[0] = TASK_NONE;
        }
        if gTasks[taskId].data[0] > 1 {
            ResetBattleAnimBg(to_BG2 ^ 1);
            DestroyTask(sMonAnimTaskIdArray[1]);
            sMonAnimTaskIdArray[1] = TASK_NONE;
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Cmd_monbg_static() {
    let mut toBG_2: u8 = 0;
    let mut battler: u8 = 0;
    let mut animBattlerId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    animBattlerId = *sBattleAnimScriptPtr;
    if animBattlerId == ANIM_ATTACKER {
        animBattlerId = ANIM_ATK_PARTNER;
    } else if animBattlerId == ANIM_TARGET {
        animBattlerId = ANIM_DEF_PARTNER;
    }
    if animBattlerId == ANIM_ATTACKER || animBattlerId == ANIM_ATK_PARTNER {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if IsBattlerSpriteVisible(battler) != 0 {
        let mut position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        MoveBattlerSpriteToBG(battler, toBG_2, FALSE);
    }
    battler ^= BIT_FLANK;
    if animBattlerId > 1 && IsBattlerSpriteVisible(battler) != 0 {
        let mut position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        MoveBattlerSpriteToBG(battler, toBG_2, FALSE);
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_clearmonbg_static() {
    let mut animBattlerId: u8 = 0;
    let mut battler: u8 = 0;
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    animBattlerId = *sBattleAnimScriptPtr;
    if animBattlerId == ANIM_ATTACKER {
        animBattlerId = ANIM_ATK_PARTNER;
    } else if animBattlerId == ANIM_TARGET {
        animBattlerId = ANIM_DEF_PARTNER;
    }
    if animBattlerId == ANIM_ATTACKER || animBattlerId == ANIM_ATK_PARTNER {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if IsBattlerSpriteVisible(battler) != 0 {
        gSprites[gBattlerSpriteIds[battler]].set_invisible(FALSE as u16);
    }
    if animBattlerId > 1 && IsBattlerSpriteVisible(battler ^ 2) != 0 {
        gSprites[gBattlerSpriteIds[battler as i32 ^ 2]].set_invisible(FALSE as u16);
    } else {
        animBattlerId = 0;
    }
    taskId = CreateTask(Some(Task_ClearMonBgStatic), 5);
    gTasks[taskId].data[0] = animBattlerId as i16;
    gTasks[taskId].data[2] = battler as i16;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Task_ClearMonBgStatic(taskId: u8) {
    gTasks[taskId].data[1] += 1;
    if gTasks[taskId].data[1] != 1 {
        let mut toBG_2: u8 = 0;
        let mut battler: u8 = gTasks[taskId].data[2] as u8;
        let mut position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_OPPONENT_LEFT
            || position == B_POSITION_PLAYER_RIGHT
            || IsContest() != 0
        {
            toBG_2 = FALSE;
        } else {
            toBG_2 = TRUE;
        }
        if IsBattlerSpriteVisible(battler) != 0 {
            ResetBattleAnimBg(toBG_2);
        }
        if gTasks[taskId].data[0] > 1 && IsBattlerSpriteVisible(battler ^ 2) != 0 {
            ResetBattleAnimBg(toBG_2 ^ 1);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Cmd_setalpha() {
    let mut half1: u16 = 0;
    let mut half2: u16 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    half1 = *({
        let t2 = sBattleAnimScriptPtr;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        t2
    }) as u16;
    half2 = (*({
        let t4 = sBattleAnimScriptPtr;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        t4
    }) as u16)
        << 8;
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, half1 | half2);
}
pub(crate) unsafe extern "C" fn Cmd_setbldcnt() {
    let mut half1: u16 = 0;
    let mut half2: u16 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    half1 = *({
        let t2 = sBattleAnimScriptPtr;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        t2
    }) as u16;
    half2 = (*({
        let t4 = sBattleAnimScriptPtr;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        t4
    }) as u16)
        << 8;
    SetGpuReg(REG_OFFSET_BLDCNT, half1 | half2);
}
pub(crate) unsafe extern "C" fn Cmd_blendoff() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
}
pub(crate) unsafe extern "C" fn Cmd_call() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptRetAddr = sBattleAnimScriptPtr.at(4);
    sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
        + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
        + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
        + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
}
pub(crate) unsafe extern "C" fn Cmd_return() {
    sBattleAnimScriptPtr = sBattleAnimScriptRetAddr;
}
pub(crate) unsafe extern "C" fn Cmd_setarg() {
    let mut addr: *mut u8 = sBattleAnimScriptPtr;
    let mut value: u16 = 0;
    let mut argId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    argId = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    value = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    sBattleAnimScriptPtr = addr.at(4);
    gBattleAnimArgs[argId] = value as i16;
}
pub(crate) unsafe extern "C" fn Cmd_choosetwoturnanim() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    if gAnimMoveTurn as i32 & 1 != 0 {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    }
    sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
        + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
        + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
        + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
}
pub(crate) unsafe extern "C" fn Cmd_jumpifmoveturn() {
    let mut toCheck: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    toCheck = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    if toCheck == gAnimMoveTurn {
        sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
            + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
            as *mut c_void as *mut u8;
    } else {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn Cmd_goto() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
        + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
        + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
        + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
        as *mut c_void as *mut u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsContest() -> u8 {
    if gMain.inBattle() == 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Cmd_fadetobg() {
    let mut backgroundId: u8 = 0;
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    backgroundId = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    taskId = CreateTask(Some(Task_FadeToBg), 5);
    gTasks[taskId].data[0] = backgroundId as i16;
    sAnimBackgroundFadeState = 1;
}
pub(crate) unsafe extern "C" fn Cmd_fadetobgfromset() {
    let mut bg1: u8 = 0;
    let mut bg2: u8 = 0;
    let mut bg3: u8 = 0;
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    bg1 = *sBattleAnimScriptPtr;
    bg2 = *sBattleAnimScriptPtr.at(1);
    bg3 = *sBattleAnimScriptPtr.at(2);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(3);
    taskId = CreateTask(Some(Task_FadeToBg), 5);
    if IsContest() != 0 {
        gTasks[taskId].data[0] = bg3 as i16;
    } else if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        gTasks[taskId].data[0] = bg2 as i16;
    } else {
        gTasks[taskId].data[0] = bg1 as i16;
    }
    sAnimBackgroundFadeState = 1;
}
pub(crate) unsafe extern "C" fn Task_FadeToBg(taskId: u8) {
    if gTasks[taskId].data[10] == 0 {
        BeginHardwarePaletteFade(0xE8, 0, 0, 16, 0);
        gTasks[taskId].data[10] += 1;
        return;
    }
    if gPaletteFade.active() != 0 {
        return;
    }
    if gTasks[taskId].data[10] == 1 {
        gTasks[taskId].data[10] += 1;
        sAnimBackgroundFadeState = 2;
    } else if gTasks[taskId].data[10] == 2 {
        let mut bgId: i16 = gTasks[taskId].data[0];
        if bgId == -1 {
            LoadDefaultBg();
        } else {
            LoadMoveBg(bgId as u16);
        }
        BeginHardwarePaletteFade(0xE8, 0, 16, 0, 1);
        gTasks[taskId].data[10] += 1;
        return;
    }
    if gPaletteFade.active() != 0 {
        return;
    }
    if gTasks[taskId].data[10] == 3 {
        DestroyTask(taskId);
        sAnimBackgroundFadeState = 0;
    }
}
pub(crate) unsafe extern "C" fn LoadMoveBg(bgId: u16) {
    if IsContest() != 0 {
        let mut tilemap: *mut u32 = gBattleAnimBackgroundTable[bgId].tilemap;
        let mut dmaSrc: *mut c_void = null_mut();
        let mut dmaDest: *mut c_void = null_mut();
        LZDecompressWram(tilemap, gDecompressionBuffer.as_mut_ptr() as *mut c_void);
        RelocateBattleBgPal(
            GetBattleBgPaletteNum() as u16,
            gDecompressionBuffer.as_mut_ptr() as *mut c_void as *mut u16,
            0x100,
            FALSE,
        );
        dmaSrc = gDecompressionBuffer.as_mut_ptr() as *mut c_void;
        dmaDest = 0x600d000 as usize as *mut c_void;
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, dmaSrc as usize as u32);
                    volatile_write(dmaRegs.at(1), dmaDest as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x84000200);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
        LZDecompressVram(
            gBattleAnimBackgroundTable[bgId].image,
            0x6002000 as usize as *mut c_void,
        );
        LoadCompressedPalette(
            gBattleAnimBackgroundTable[bgId].palette,
            0x000 + GetBattleBgPaletteNum() as u16 * 16,
            32,
        );
    } else {
        LZDecompressVram(
            gBattleAnimBackgroundTable[bgId].tilemap,
            0x600d000 as usize as *mut c_void,
        );
        LZDecompressVram(
            gBattleAnimBackgroundTable[bgId].image,
            0x6008000 as usize as *mut c_void,
        );
        LoadCompressedPalette(gBattleAnimBackgroundTable[bgId].palette, 32, 32);
    }
}
pub(crate) unsafe extern "C" fn LoadDefaultBg() {
    if IsContest() != 0 {
        LoadContestBgAfterMoveAnim();
    } else {
        DrawMainBattleBackground();
    }
}
pub(crate) unsafe extern "C" fn Cmd_restorebg() {
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    taskId = CreateTask(Some(Task_FadeToBg), 5);
    gTasks[taskId].data[0] = -1;
    sAnimBackgroundFadeState = 1;
}
pub(crate) unsafe extern "C" fn Cmd_waitbgfadeout() {
    if sAnimBackgroundFadeState == 2 {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        sAnimFramesToWait = 0;
    } else {
        sAnimFramesToWait = 1;
    }
}
pub(crate) unsafe extern "C" fn Cmd_waitbgfadein() {
    if sAnimBackgroundFadeState == 0 {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        sAnimFramesToWait = 0;
    } else {
        sAnimFramesToWait = 1;
    }
}
pub(crate) unsafe extern "C" fn Cmd_changebg() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    LoadMoveBg(*sBattleAnimScriptPtr as u16);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAnimAdjustPanning(mut pan: i8) -> i8 {
    if IsContest() == 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gBattleAnimAttacker))
        .statusAnimActive()
            != 0
    {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            pan = SOUND_PAN_TARGET;
        } else {
            pan = SOUND_PAN_ATTACKER;
        }
    } else if IsContest() != 0 {
        if gBattleAnimAttacker != gBattleAnimTarget
            || gBattleAnimAttacker != 2
            || pan != SOUND_PAN_TARGET
        {
            pan *= -1;
        }
    } else if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
            if pan == SOUND_PAN_TARGET {
                pan = SOUND_PAN_ATTACKER;
            } else if pan != SOUND_PAN_ATTACKER {
                pan *= -1;
            }
        }
    } else if GetBattlerSide(gBattleAnimTarget) == B_SIDE_OPPONENT {
        if pan == SOUND_PAN_ATTACKER {
            pan = SOUND_PAN_TARGET;
        }
    } else {
        pan *= -1;
    }
    if pan > SOUND_PAN_TARGET {
        pan = SOUND_PAN_TARGET;
    } else if pan < SOUND_PAN_ATTACKER {
        pan = SOUND_PAN_ATTACKER;
    }
    return pan;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAnimAdjustPanning2(mut pan: i8) -> i8 {
    if IsContest() == 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gBattleAnimAttacker))
        .statusAnimActive()
            != 0
    {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            pan = SOUND_PAN_TARGET;
        } else {
            pan = SOUND_PAN_ATTACKER;
        }
    } else {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER || IsContest() != 0 {
            pan = -pan;
        }
    }
    return pan;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn KeepPanInRange(panArg: i16, oldPan: i32) -> i16 {
    let mut pan: i16 = panArg;
    if pan > SOUND_PAN_TARGET as i16 {
        pan = SOUND_PAN_TARGET as i16;
    } else if pan < SOUND_PAN_ATTACKER as i16 {
        pan = SOUND_PAN_ATTACKER as i16;
    }
    return pan;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculatePanIncrement(
    sourcePan: i16,
    targetPan: i16,
    incrementPan: i16,
) -> i16 {
    let mut ret: i16 = 0;
    if sourcePan < targetPan {
        ret = (if incrementPan < 0 {
            -(incrementPan as i32)
        } else {
            incrementPan as i32
        }) as i16;
    } else if sourcePan > targetPan {
        ret = -((if incrementPan < 0 {
            -(incrementPan as i32)
        } else {
            incrementPan as i32
        }) as i16);
    } else {
        ret = 0;
    }
    return ret;
}
pub(crate) unsafe extern "C" fn Cmd_playsewithpan() {
    let mut songId: u16 = 0;
    let mut pan: i8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    songId = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    pan = *sBattleAnimScriptPtr.at(2) as i8;
    PlaySE12WithPanning(songId, BattleAnimAdjustPanning(pan));
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(3);
}
pub(crate) unsafe extern "C" fn Cmd_setpan() {
    let mut pan: i8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    pan = *sBattleAnimScriptPtr as i8;
    SE12PanpotControl(BattleAnimAdjustPanning(pan));
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
pub(crate) unsafe extern "C" fn Cmd_panse() {
    let mut songNum: u16 = 0;
    let mut currentPanArg: i8 = 0;
    let mut incrementPan: i8 = 0;
    let mut incrementPanArg: i8 = 0;
    let mut currentPan: i8 = 0;
    let mut targetPan: i8 = 0;
    let mut framesToWait: u8 = 0;
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    songNum = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    currentPanArg = *sBattleAnimScriptPtr.at(2) as i8;
    incrementPan = *sBattleAnimScriptPtr.at(3) as i8;
    incrementPanArg = *sBattleAnimScriptPtr.at(4) as i8;
    framesToWait = *sBattleAnimScriptPtr.at(5);
    currentPan = BattleAnimAdjustPanning(currentPanArg);
    targetPan = BattleAnimAdjustPanning(incrementPan);
    incrementPan =
        CalculatePanIncrement(currentPan as i16, targetPan as i16, incrementPanArg as i16) as i8;
    taskId = CreateTask(Some(Task_PanFromInitialToTarget), 1);
    gTasks[taskId].data[0] = currentPan as i16;
    gTasks[taskId].data[1] = targetPan as i16;
    gTasks[taskId].data[2] = incrementPan as i16;
    gTasks[taskId].data[3] = framesToWait as i16;
    gTasks[taskId].data[4] = currentPan as i16;
    PlaySE12WithPanning(songNum, currentPan);
    gAnimSoundTaskCount += 1;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(6);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_PanFromInitialToTarget(taskId: u8) {
    let mut destroyTask: u32 = FALSE as u32;
    if ({
        let t1 = gTasks[taskId].data[8];
        gTasks[taskId].data[8] += 1;
        t1
    }) >= gTasks[taskId].data[3]
    {
        let mut pan: i16 = 0;
        let mut initialPanning: i16 = 0;
        let mut targetPanning: i16 = 0;
        let mut currentPan: i16 = 0;
        let mut incrementPan: i16 = 0;
        gTasks[taskId].data[8] = 0;
        initialPanning = gTasks[taskId].data[0];
        targetPanning = gTasks[taskId].data[1];
        currentPan = gTasks[taskId].data[4];
        incrementPan = gTasks[taskId].data[2];
        pan = currentPan + incrementPan;
        gTasks[taskId].data[4] = pan;
        if incrementPan == 0 {
            destroyTask = TRUE as u32;
        } else if initialPanning < targetPanning {
            if pan >= targetPanning {
                destroyTask = TRUE as u32;
            }
        } else {
            if pan <= targetPanning {
                destroyTask = TRUE as u32;
            }
        }
        if destroyTask != 0 {
            pan = targetPanning;
            DestroyTask(taskId);
            gAnimSoundTaskCount -= 1;
        }
        SE12PanpotControl(pan as i8);
    }
}
pub(crate) unsafe extern "C" fn Cmd_panse_adjustnone() {
    let mut songId: u16 = 0;
    let mut currentPan: i8 = 0;
    let mut targetPan: i8 = 0;
    let mut incrementPan: i8 = 0;
    let mut framesToWait: u8 = 0;
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    songId = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    currentPan = *sBattleAnimScriptPtr.at(2) as i8;
    targetPan = *sBattleAnimScriptPtr.at(3) as i8;
    incrementPan = *sBattleAnimScriptPtr.at(4) as i8;
    framesToWait = *sBattleAnimScriptPtr.at(5);
    taskId = CreateTask(Some(Task_PanFromInitialToTarget), 1);
    gTasks[taskId].data[0] = currentPan as i16;
    gTasks[taskId].data[1] = targetPan as i16;
    gTasks[taskId].data[2] = incrementPan as i16;
    gTasks[taskId].data[3] = framesToWait as i16;
    gTasks[taskId].data[4] = currentPan as i16;
    PlaySE12WithPanning(songId, currentPan);
    gAnimSoundTaskCount += 1;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(6);
}
pub(crate) unsafe extern "C" fn Cmd_panse_adjustall() {
    let mut songId: u16 = 0;
    let mut targetPanArg: i8 = 0;
    let mut incrementPanArg: i8 = 0;
    let mut currentPanArg: i8 = 0;
    let mut currentPan: i8 = 0;
    let mut targetPan: i8 = 0;
    let mut incrementPan: i8 = 0;
    let mut framesToWait: u8 = 0;
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    songId = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    currentPanArg = *sBattleAnimScriptPtr.at(2) as i8;
    targetPanArg = *sBattleAnimScriptPtr.at(3) as i8;
    incrementPanArg = *sBattleAnimScriptPtr.at(4) as i8;
    framesToWait = *sBattleAnimScriptPtr.at(5);
    currentPan = BattleAnimAdjustPanning2(currentPanArg);
    targetPan = BattleAnimAdjustPanning2(targetPanArg);
    incrementPan = BattleAnimAdjustPanning2(incrementPanArg);
    taskId = CreateTask(Some(Task_PanFromInitialToTarget), 1);
    gTasks[taskId].data[0] = currentPan as i16;
    gTasks[taskId].data[1] = targetPan as i16;
    gTasks[taskId].data[2] = incrementPan as i16;
    gTasks[taskId].data[3] = framesToWait as i16;
    gTasks[taskId].data[4] = currentPan as i16;
    PlaySE12WithPanning(songId, currentPan);
    gAnimSoundTaskCount += 1;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(6);
}
pub(crate) unsafe extern "C" fn Cmd_loopsewithpan() {
    let mut songId: u16 = 0;
    let mut panningArg: i8 = 0;
    let mut panning: i8 = 0;
    let mut framesToWait: u8 = 0;
    let mut numberOfPlays: u8 = 0;
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    songId = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    panningArg = *sBattleAnimScriptPtr.at(2) as i8;
    framesToWait = *sBattleAnimScriptPtr.at(3);
    numberOfPlays = *sBattleAnimScriptPtr.at(4);
    panning = BattleAnimAdjustPanning(panningArg);
    taskId = CreateTask(Some(Task_LoopAndPlaySE), 1);
    gTasks[taskId].data[0] = songId as i16;
    gTasks[taskId].data[1] = panning as i16;
    gTasks[taskId].data[2] = framesToWait as i16;
    gTasks[taskId].data[3] = numberOfPlays as i16;
    gTasks[taskId].data[8] = framesToWait as i16;
    gTasks[taskId].func.unwrap_unchecked()(taskId);
    gAnimSoundTaskCount += 1;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(5);
}
pub(crate) unsafe extern "C" fn Task_LoopAndPlaySE(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[8];
        gTasks[taskId].data[8] += 1;
        t1
    }) >= gTasks[taskId].data[2]
    {
        let mut songId: u16 = 0;
        let mut panning: i8 = 0;
        let mut numberOfPlays: u8 = 0;
        gTasks[taskId].data[8] = 0;
        songId = gTasks[taskId].data[0] as u16;
        panning = gTasks[taskId].data[1] as i8;
        numberOfPlays = ({
            gTasks[taskId].data[3] -= 1;
            gTasks[taskId].data[3]
        }) as u8;
        PlaySE12WithPanning(songId, panning);
        if numberOfPlays == 0 {
            DestroyTask(taskId);
            gAnimSoundTaskCount -= 1;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_waitplaysewithpan() {
    let mut songId: u16 = 0;
    let mut panningArg: i8 = 0;
    let mut panning: i8 = 0;
    let mut framesToWait: u8 = 0;
    let mut taskId: u8 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    songId = *sBattleAnimScriptPtr as u16 | (*sBattleAnimScriptPtr.at(1) as u16) << 8;
    panningArg = *sBattleAnimScriptPtr.at(2) as i8;
    framesToWait = *sBattleAnimScriptPtr.at(3);
    panning = BattleAnimAdjustPanning(panningArg);
    taskId = CreateTask(Some(Task_WaitAndPlaySE), 1);
    gTasks[taskId].data[0] = songId as i16;
    gTasks[taskId].data[1] = panning as i16;
    gTasks[taskId].data[2] = framesToWait as i16;
    gAnimSoundTaskCount += 1;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
}
pub(crate) unsafe extern "C" fn Task_WaitAndPlaySE(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[2];
        gTasks[taskId].data[2] -= 1;
        t1
    }) <= 0
    {
        PlaySE12WithPanning(gTasks[taskId].data[0] as u16, gTasks[taskId].data[1] as i8);
        DestroyTask(taskId);
        gAnimSoundTaskCount -= 1;
    }
}
pub(crate) unsafe extern "C" fn Cmd_createsoundtask() {
    let mut func: Option<unsafe extern "C" fn(u8)> = None;
    let mut numArgs: u8 = 0;
    let mut taskId: u8 = 0;
    let mut i: i32 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    func = core::mem::transmute::<usize, Option<unsafe extern "C" fn(u8)>>(
        (*sBattleAnimScriptPtr as i32
            + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize,
    );
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    numArgs = *sBattleAnimScriptPtr;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    i = 0;
    while i < numArgs as i32 {
        gBattleAnimArgs[i] =
            *sBattleAnimScriptPtr as i16 | (*sBattleAnimScriptPtr.at(1) as i16) << 8;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
        i += 1;
    }
    taskId = CreateTask(func, 1);
    func.unwrap_unchecked()(taskId);
    gAnimSoundTaskCount += 1;
}
pub(crate) unsafe extern "C" fn Cmd_waitsound() {
    if gAnimSoundTaskCount != 0 {
        sSoundAnimFramesToWait = 0;
        sAnimFramesToWait = 1;
    } else if IsSEPlaying() != 0 {
        if ({
            sSoundAnimFramesToWait += 1;
            sSoundAnimFramesToWait
        }) > 90
        {
            m4aMPlayStop(&raw mut gMPlayInfo_SE1);
            m4aMPlayStop(&raw mut gMPlayInfo_SE2);
            sSoundAnimFramesToWait = 0;
        } else {
            sAnimFramesToWait = 1;
        }
    } else {
        sSoundAnimFramesToWait = 0;
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
        sAnimFramesToWait = 0;
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpargeq() {
    let mut argId: u8 = 0;
    let mut valueToCheck: i16 = 0;
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    argId = *sBattleAnimScriptPtr;
    valueToCheck =
        *sBattleAnimScriptPtr.at(1) as i16 | (*sBattleAnimScriptPtr.at(1).at(1) as i16) << 8;
    if valueToCheck == gBattleAnimArgs[argId] {
        sBattleAnimScriptPtr = (*sBattleAnimScriptPtr.at(3) as i32
            + ((*sBattleAnimScriptPtr.at(3).at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(3).at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3).at(3) as i32) << 24))
            as usize as *mut c_void as *mut u8;
    } else {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(7);
    }
}
pub(crate) unsafe extern "C" fn Cmd_jumpifcontest() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    if IsContest() != 0 {
        sBattleAnimScriptPtr = (*sBattleAnimScriptPtr as i32
            + ((*sBattleAnimScriptPtr.at(1) as i32) << 8)
            + ((*sBattleAnimScriptPtr.at(2) as i32) << 16)
            + ((*sBattleAnimScriptPtr.at(3) as i32) << 24)) as usize
            as *mut c_void as *mut u8;
    } else {
        sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(4);
    }
}
pub(crate) unsafe extern "C" fn Cmd_splitbgprio() {
    let mut wantedBattler: u8 = 0;
    let mut battler: u8 = 0;
    let mut battlerPosition: u8 = 0;
    wantedBattler = *sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    if wantedBattler != ANIM_ATTACKER {
        battler = gBattleAnimTarget;
    } else {
        battler = gBattleAnimAttacker;
    }
    battlerPosition = GetBattlerPosition(battler);
    if IsContest() == 0
        && (battlerPosition == B_POSITION_PLAYER_LEFT
            || battlerPosition == B_POSITION_OPPONENT_RIGHT)
    {
        SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
        SetAnimBgAttribute(2, BG_ANIM_PRIORITY, 2);
    }
}
pub(crate) unsafe extern "C" fn Cmd_splitbgprio_all() {
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
    if IsContest() == 0 {
        SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
        SetAnimBgAttribute(2, BG_ANIM_PRIORITY, 2);
    }
}
pub(crate) unsafe extern "C" fn Cmd_splitbgprio_foes() {
    let mut wantedBattler: u8 = 0;
    let mut battlerPosition: u8 = 0;
    let mut battler: u8 = 0;
    wantedBattler = *sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    if GetBattlerSide(gBattleAnimAttacker) != GetBattlerSide(gBattleAnimTarget) {
        if wantedBattler != ANIM_ATTACKER {
            battler = gBattleAnimTarget;
        } else {
            battler = gBattleAnimAttacker;
        }
        battlerPosition = GetBattlerPosition(battler);
        if IsContest() == 0
            && (battlerPosition == B_POSITION_PLAYER_LEFT
                || battlerPosition == B_POSITION_OPPONENT_RIGHT)
        {
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            SetAnimBgAttribute(2, BG_ANIM_PRIORITY, 2);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_invisible() {
    let mut spriteId: u8 = 0;
    spriteId = GetAnimBattlerSpriteId(*sBattleAnimScriptPtr.at(1));
    if spriteId != SPRITE_NONE {
        gSprites[spriteId].set_invisible(TRUE as u16);
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_visible() {
    let mut spriteId: u8 = 0;
    spriteId = GetAnimBattlerSpriteId(*sBattleAnimScriptPtr.at(1));
    if spriteId != SPRITE_NONE {
        gSprites[spriteId].set_invisible(FALSE as u16);
    }
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
}
pub(crate) unsafe extern "C" fn Cmd_teamattack_moveback() {
    let mut wantedBattler: u8 = 0;
    let mut priorityRank: u8 = 0;
    let mut spriteId: u8 = 0;
    wantedBattler = *sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    if IsContest() == 0
        && IsDoubleBattle() != 0
        && GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget)
    {
        if wantedBattler == ANIM_ATTACKER {
            priorityRank = GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker);
            spriteId = GetAnimBattlerSpriteId(ANIM_ATTACKER);
        } else {
            priorityRank = GetBattlerSpriteBGPriorityRank(gBattleAnimTarget);
            spriteId = GetAnimBattlerSpriteId(ANIM_TARGET);
        }
        if spriteId != SPRITE_NONE {
            gSprites[spriteId].set_invisible(FALSE as u16);
            if priorityRank == 2 {
                gSprites[spriteId].oam.set_priority(3);
            }
            if priorityRank == 1 {
                ResetBattleAnimBg(FALSE);
            } else {
                ResetBattleAnimBg(TRUE);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_teamattack_movefwd() {
    let mut wantedBattler: u8 = 0;
    let mut priorityRank: u8 = 0;
    let mut spriteId: u8 = 0;
    wantedBattler = *sBattleAnimScriptPtr.at(1);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(2);
    if IsContest() == 0
        && IsDoubleBattle() != 0
        && GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget)
    {
        if wantedBattler == ANIM_ATTACKER {
            priorityRank = GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker);
            spriteId = GetAnimBattlerSpriteId(ANIM_ATTACKER);
        } else {
            priorityRank = GetBattlerSpriteBGPriorityRank(gBattleAnimTarget);
            spriteId = GetAnimBattlerSpriteId(ANIM_TARGET);
        }
        if spriteId != SPRITE_NONE && priorityRank == 2 {
            gSprites[spriteId].oam.set_priority(2);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_stopsound() {
    m4aMPlayStop(&raw mut gMPlayInfo_SE1);
    m4aMPlayStop(&raw mut gMPlayInfo_SE2);
    sBattleAnimScriptPtr = sBattleAnimScriptPtr.at(1);
}
