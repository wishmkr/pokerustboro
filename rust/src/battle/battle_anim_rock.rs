//! Translated from `src/battle_anim_rock.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sAnim_FlyingRock_0 sAnim_FlyingRock_1 sAnim_FlyingRock_2 sAnims_FlyingRock gFallingRockSpriteTemplate gRockFragmentSpriteTemplate gSwirlingDirtSpriteTemplate sAffineAnim_Whirlpool sAffineAnims_Whirlpool gWhirlpoolSpriteTemplate gFireSpinSpriteTemplate gFlyingSandCrescentSpriteTemplate sFlyingSandSubsprites sFlyingSandSubspriteTable sAnim_Rock_Biggest sAnim_Rock_Bigger sAnim_Rock_Big sAnim_Rock_Small sAnim_Rock_Smaller sAnim_Rock_Smallest sAnims_BasicRock gAncientPowerRockSpriteTemplate gRolloutMudSpriteTemplate gRolloutRockSpriteTemplate gRockTombRockSpriteTemplate sAffineAnim_BasicRock_0 sAffineAnim_BasicRock_1 sAffineAnims_BasicRock gRockBlastRockSpriteTemplate gRockScatterSpriteTemplate gTwisterRockSpriteTemplate gWeatherBallRockDownSpriteTemplate

static gRolloutMudSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_rock::gRolloutMudSpriteTemplate).cast());
static gRolloutRockSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_rock::gRolloutRockSpriteTemplate).cast());
static sFlyingSandSubspriteTable: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::battle_anim_rock::sFlyingSandSubspriteTable).cast());

unsafe extern "C" {
    static mut gAnimDisableStructPtr: *mut DisableStruct;
    static mut gAnimMoveDmg: i32;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static gBattleAnimBgImage_Sandstorm: CArray<u32, 0>;
    static gBattleAnimBgTilemap_Sandstorm: CArray<u32, 0>;
    static gBattleAnimSpritePal_FlyingDirt: CArray<u32, 0>;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG3_Y: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemapHandleContest(
        a0: *mut BattleAnimBgData,
        a1: *mut c_void,
        a2: u32,
    );
    fn AnimateSprite(a0: *mut Sprite);
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn ClearBattleAnimBg(a0: u32);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut BattleAnimBgData);
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn InitAnimArcTranslation(a0: *mut Sprite);
    fn InitSpriteDataForLinearTranslation(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut Sprite, a1: u8);
    fn IsContest() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateAnimHorizontalArc(a0: *mut Sprite) -> u8;
    fn TranslateAnimSpriteToTargetMonLocation(a0: *mut Sprite);
    fn TranslateSpriteInEllipse(a0: *mut Sprite);
    fn TranslateSpriteLinearFixedPoint(a0: *mut Sprite);
    fn UpdateAnimBg3ScreenSize(a0: u8);
}

pub(crate) unsafe extern "C" fn AnimFallingRock(sprite: *mut Sprite) {
    if gBattleAnimArgs[3] != 0 {
        SetAverageBattlerPositions(
            gBattleAnimTarget,
            FALSE,
            &raw mut (*sprite).x,
            &raw mut (*sprite).y,
        );
    }
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += 14;
    StartSpriteAnim(sprite, gBattleAnimArgs[1] as u8);
    AnimateSprite(sprite);
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = 4;
    (*sprite).data[3] = 16;
    (*sprite).data[4] = -70;
    (*sprite).data[5] = gBattleAnimArgs[2];
    StoreSpriteCallbackInData6(sprite, Some(AnimFallingRock_Step));
    (*sprite).callback = Some(TranslateSpriteInEllipse);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimFallingRock_Step(sprite: *mut Sprite) {
    (*sprite).x += (*sprite).data[5];
    (*sprite).data[0] = 192;
    (*sprite).data[1] = (*sprite).data[5];
    (*sprite).data[2] = 4;
    (*sprite).data[3] = 32;
    (*sprite).data[4] = -24;
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(TranslateSpriteInEllipse);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimRockFragment(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, gBattleAnimArgs[5] as u8);
    AnimateSprite(sprite);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= gBattleAnimArgs[0];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
    }
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = (*sprite).x + gBattleAnimArgs[2];
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = (*sprite).y + gBattleAnimArgs[3];
    InitSpriteDataForLinearTranslation(sprite);
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).callback = Some(TranslateSpriteLinearFixedPoint);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
}
pub(crate) unsafe extern "C" fn AnimParticleInVortex(sprite: *mut Sprite) {
    if gBattleAnimArgs[6] == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, FALSE);
    } else {
        InitSpritePosToAnimTarget(sprite, FALSE);
    }
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[2] = gBattleAnimArgs[4];
    (*sprite).data[3] = gBattleAnimArgs[5];
    (*sprite).callback = Some(AnimParticleInVortex_Step);
}
pub(crate) unsafe extern "C" fn AnimParticleInVortex_Step(sprite: *mut Sprite) {
    (*sprite).data[4] += (*sprite).data[1];
    (*sprite).y2 = -((*sprite).data[4] >> 8);
    (*sprite).x2 = Sin((*sprite).data[5], (*sprite).data[3]);
    (*sprite).data[5] = (*sprite).data[5] + (*sprite).data[2] & 0xFF;
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == -1
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_LoadSandstormBackground(taskId: u8) {
    let mut var0: i32 = 0;
    let mut animBg: BattleAnimBgData = zeroed();
    var0 = 0;
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
    SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 0);
    if IsContest() == 0 {
        SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 1);
    }
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    GetBattleAnimBg1Data(&raw mut animBg);
    AnimLoadCompressedBgGfx(
        animBg.bgId as u32,
        gBattleAnimBgImage_Sandstorm.as_ptr().cast_mut(),
        animBg.tilesOffset as u32,
    );
    AnimLoadCompressedBgTilemapHandleContest(
        &raw mut animBg,
        gBattleAnimBgTilemap_Sandstorm.as_ptr().cast_mut() as *mut c_void,
        FALSE as u32,
    );
    LoadCompressedPalette(
        gBattleAnimSpritePal_FlyingDirt.as_ptr().cast_mut(),
        0x000 + animBg.paletteId as u16 * 16,
        32,
    );
    if gBattleAnimArgs[0] != 0 && GetBattlerSide(gBattleAnimAttacker) != 0 {
        var0 = 1;
    }
    gTasks[taskId].data[0] = var0 as i16;
    gTasks[taskId].func = Some(AnimTask_LoadSandstormBackground_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_LoadSandstormBackground_Step(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    if gTasks[taskId].data[0] == 0 {
        gBattle_BG1_X += 65530;
    } else {
        gBattle_BG1_X += 6;
    }
    gBattle_BG1_Y += 65535;
    match gTasks[taskId].data[12] {
        0 => {
            if ({
                gTasks[taskId].data[10] += 1;
                gTasks[taskId].data[10]
            }) == 4
            {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[11] += 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - gTasks[taskId].data[11] as u16) << 8 | gTasks[taskId].data[11] as u16,
                );
                if gTasks[taskId].data[11] == 7 {
                    gTasks[taskId].data[12] += 1;
                    gTasks[taskId].data[11] = 0;
                }
            }
        }
        1 => {
            if ({
                gTasks[taskId].data[11] += 1;
                gTasks[taskId].data[11]
            }) == 101
            {
                gTasks[taskId].data[11] = 7;
                gTasks[taskId].data[12] += 1;
            }
        }
        2 => {
            if ({
                gTasks[taskId].data[10] += 1;
                gTasks[taskId].data[10]
            }) == 4
            {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[11] -= 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - gTasks[taskId].data[11] as u16) << 8 | gTasks[taskId].data[11] as u16,
                );
                if gTasks[taskId].data[11] == 0 {
                    gTasks[taskId].data[12] += 1;
                    gTasks[taskId].data[11] = 0;
                }
            }
        }
        3 => {
            GetBattleAnimBg1Data(&raw mut animBg);
            ClearBattleAnimBg(animBg.bgId as u32);
            gTasks[taskId].data[12] += 1;
        }
        4 => {
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimFlyingSandCrescent(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        if gBattleAnimArgs[3] != 0 && GetBattlerSide(gBattleAnimAttacker) != 0 {
            (*sprite).x = 304;
            gBattleAnimArgs[1] = -gBattleAnimArgs[1];
            (*sprite).data[5] = 1;
            (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
        } else {
            (*sprite).x = -64;
        }
        (*sprite).y = gBattleAnimArgs[0];
        SetSubspriteTables(sprite, sFlyingSandSubspriteTable.as_ptr().cast_mut());
        (*sprite).data[1] = gBattleAnimArgs[1];
        (*sprite).data[2] = gBattleAnimArgs[2];
        (*sprite).data[0] += 1;
    } else {
        (*sprite).data[3] += (*sprite).data[1];
        (*sprite).data[4] += (*sprite).data[2];
        (*sprite).x2 += (*sprite).data[3] >> 8;
        (*sprite).y2 += (*sprite).data[4] >> 8;
        (*sprite).data[3] &= 0xFF;
        (*sprite).data[4] &= 0xFF;
        if (*sprite).data[5] == 0 {
            if (*sprite).x as i32 + (*sprite).x2 as i32 > 272 {
                (*sprite).callback = Some(DestroyAnimSprite);
            }
        } else if ((*sprite).x as i32 + (*sprite).x2 as i32) < -32 {
            (*sprite).callback = Some(DestroyAnimSprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRaiseSprite(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, gBattleAnimArgs[4] as u8);
    InitSpritePosToAnimAttacker(sprite, FALSE);
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).data[2] = (*sprite).x;
    (*sprite).data[4] = (*sprite).y + gBattleAnimArgs[2];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Rollout(taskId: u8) {
    let mut var0: u16 = 0;
    let mut var1: u16 = 0;
    let mut var2: u16 = 0;
    let mut var3: u16 = 0;
    let mut rolloutCounter: u8 = 0;
    let mut pan1: i16 = 0;
    let mut pan2: i16 = 0;
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[taskId];
    var0 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as u16;
    var1 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as u16 + 24;
    var2 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as u16;
    var3 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as u16 + 24;
    if gBattleAnimAttacker as i32 ^ 2 == gBattleAnimTarget as i32 {
        var3 = var1;
    }
    rolloutCounter = GetRolloutCounter();
    if rolloutCounter == 1 {
        (*task).data[8] = 32;
    } else {
        (*task).data[8] = 48 - rolloutCounter as i16 * 8;
    }
    (*task).data[0] = 0;
    (*task).data[11] = 0;
    (*task).data[9] = 0;
    (*task).data[12] = 1;
    (*task).data[10] = (*task).data[8] / 8 - 1;
    (*task).data[2] = var0 as i16 * 8;
    (*task).data[3] = var1 as i16 * 8;
    (*task).data[4] = div_i32((var2 as i32 - var0 as i32) * 8, (*task).data[8] as i32) as i16;
    (*task).data[5] = div_i32((var3 as i32 - var1 as i32) * 8, (*task).data[8] as i32) as i16;
    (*task).data[6] = 0;
    (*task).data[7] = 0;
    pan1 = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER) as i16;
    pan2 = BattleAnimAdjustPanning(SOUND_PAN_TARGET) as i16;
    (*task).data[13] = pan1;
    (*task).data[14] = div_i32(pan2 as i32 - pan1 as i32, (*task).data[8] as i32) as i16;
    (*task).data[1] = rolloutCounter as i16;
    (*task).data[15] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*task).func = Some(AnimTask_Rollout_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_Rollout_Step(taskId: u8) {
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[6] -= (*task).data[4];
            (*task).data[7] -= (*task).data[5];
            gSprites[(*task).data[15]].x2 = (*task).data[6] >> 3;
            gSprites[(*task).data[15]].y2 = (*task).data[7] >> 3;
            if ({
                (*task).data[9] += 1;
                (*task).data[9]
            }) == 10
            {
                (*task).data[11] = 20;
                (*task).data[0] += 1;
            }
            PlaySE12WithPanning(SE_M_HEADBUTT, (*task).data[13] as i8);
        }
        1 => {
            if ({
                (*task).data[11] -= 1;
                (*task).data[11]
            }) == 0
            {
                (*task).data[0] += 1;
            }
        }
        2 => {
            if ({
                (*task).data[9] -= 1;
                (*task).data[9]
            }) != 0
            {
                (*task).data[6] += (*task).data[4];
                (*task).data[7] += (*task).data[5];
            } else {
                (*task).data[6] = 0;
                (*task).data[7] = 0;
                (*task).data[0] += 1;
            }
            gSprites[(*task).data[15]].x2 = (*task).data[6] >> 3;
            gSprites[(*task).data[15]].y2 = (*task).data[7] >> 3;
        }
        3 => {
            (*task).data[2] += (*task).data[4];
            (*task).data[3] += (*task).data[5];
            if ({
                (*task).data[9] += 1;
                (*task).data[9]
            }) >= (*task).data[10]
            {
                (*task).data[9] = 0;
                CreateRolloutDirtSprite(task);
                (*task).data[13] += (*task).data[14];
                PlaySE12WithPanning(SE_M_DIG, (*task).data[13] as i8);
            }
            if ({
                (*task).data[8] -= 1;
                (*task).data[8]
            }) == 0
            {
                (*task).data[0] += 1;
            }
        }
        4 => {
            if (*task).data[11] == 0 {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateRolloutDirtSprite(task: *mut Task) {
    let mut spriteTemplate: *mut SpriteTemplate = null_mut();
    let mut tileOffset: i32 = 0;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut spriteId: u8 = 0;
    match (*task).data[1] {
        1 => {
            spriteTemplate = (&raw const *gRolloutMudSpriteTemplate).cast_mut();
            tileOffset = 0;
        }
        2 | 3 => {
            spriteTemplate = (&raw const *gRolloutRockSpriteTemplate).cast_mut();
            tileOffset = 80;
        }
        4 => {
            spriteTemplate = (&raw const *gRolloutRockSpriteTemplate).cast_mut();
            tileOffset = 64;
        }
        5 => {
            spriteTemplate = (&raw const *gRolloutRockSpriteTemplate).cast_mut();
            tileOffset = 48;
        }
        _ => {
            return;
        }
    }
    x = ((*task).data[2] >> 3) as u16;
    y = ((*task).data[3] >> 3) as u16;
    x += (*task).data[12] as u16 * 4;
    spriteId = CreateSprite(spriteTemplate, x as i16, y as i16, 35);
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[0] = 18;
        gSprites[spriteId].data[2] = (*task).data[12] * 20 + x as i16 + (*task).data[1] * 3;
        gSprites[spriteId].data[4] = y as i16;
        gSprites[spriteId].data[5] = -16 - (*task).data[1] * 2;
        gSprites[spriteId]
            .oam
            .set_tileNum(gSprites[spriteId].oam.tileNum() + tileOffset as u16);
        InitAnimArcTranslation(&raw mut gSprites[spriteId]);
        (*task).data[11] += 1;
    }
    (*task).data[12] *= -1;
}
pub(crate) unsafe extern "C" fn AnimRolloutParticle(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        let mut taskId: u8 = FindTaskIdByFunc(Some(AnimTask_Rollout_Step));
        if taskId != TASK_NONE {
            gTasks[taskId].data[11] -= 1;
        }
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn GetRolloutCounter() -> u8 {
    let mut retVal: u8 =
        (*gAnimDisableStructPtr).rolloutTimerStartValue() - (*gAnimDisableStructPtr).rolloutTimer();
    let mut var0: u8 = retVal - 1;
    if var0 > 4 {
        retVal = 1;
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn AnimRockTomb(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, gBattleAnimArgs[4] as u8);
    (*sprite).x2 = gBattleAnimArgs[0];
    (*sprite).data[2] = gBattleAnimArgs[1];
    (*sprite).data[3] -= gBattleAnimArgs[2];
    (*sprite).data[0] = 3;
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).callback = Some(AnimRockTomb_Step);
    (*sprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn AnimRockTomb_Step(sprite: *mut Sprite) {
    (*sprite).set_invisible(FALSE as u16);
    if (*sprite).data[3] != 0 {
        (*sprite).y2 = (*sprite).data[2] + (*sprite).data[3];
        (*sprite).data[3] += (*sprite).data[0];
        (*sprite).data[0] += 1;
        if (*sprite).data[3] > 0 {
            (*sprite).data[3] = 0;
        }
    } else {
        if ({
            (*sprite).data[1] -= 1;
            (*sprite).data[1]
        }) == 0
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRockBlastRock(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
        StartSpriteAffineAnim(sprite, 1);
    }
    TranslateAnimSpriteToTargetMonLocation(sprite);
}
pub(crate) unsafe extern "C" fn AnimRockScatter(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[1] = gBattleAnimArgs[0];
    (*sprite).data[2] = gBattleAnimArgs[1];
    (*sprite).data[5] = gBattleAnimArgs[2];
    StartSpriteAnim(sprite, gBattleAnimArgs[3] as u8);
    (*sprite).callback = Some(AnimRockScatter_Step);
}
pub(crate) unsafe extern "C" fn AnimRockScatter_Step(sprite: *mut Sprite) {
    (*sprite).data[0] += 8;
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).data[4] += (*sprite).data[2];
    (*sprite).x2 += (*sprite).data[3] / 40;
    (*sprite).y2 -= Sin((*sprite).data[0], (*sprite).data[5]);
    if (*sprite).data[0] > 140 {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetSeismicTossDamageLevel(taskId: u8) {
    if gAnimMoveDmg < 33 {
        gBattleAnimArgs[7] = 0;
    }
    if gAnimMoveDmg as u32 - 33 < 33 {
        gBattleAnimArgs[7] = 1;
    }
    if gAnimMoveDmg > 65 {
        gBattleAnimArgs[7] = 2;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MoveSeismicTossBg(taskId: u8) {
    if gTasks[taskId].data[0] == 0 {
        UpdateAnimBg3ScreenSize(FALSE);
        gTasks[taskId].data[1] = 200;
    }
    gBattle_BG3_Y += (gTasks[taskId].data[1] / 10) as u16;
    gTasks[taskId].data[1] -= 3;
    if gTasks[taskId].data[0] == 120 {
        UpdateAnimBg3ScreenSize(TRUE);
        DestroyAnimVisualTask(taskId);
    }
    gTasks[taskId].data[0] += 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SeismicTossBgAccelerateDownAtEnd(taskId: u8) {
    if gTasks[taskId].data[0] == 0 {
        UpdateAnimBg3ScreenSize(FALSE);
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].data[2] = gBattle_BG3_Y as i16;
    }
    gTasks[taskId].data[1] += 80;
    gTasks[taskId].data[1] &= 0xFF;
    gBattle_BG3_Y = gTasks[taskId].data[2] as u16 + Cos(4, gTasks[taskId].data[1]) as u16;
    if gBattleAnimArgs[7] == 0xFFF {
        gBattle_BG3_Y = 0;
        UpdateAnimBg3ScreenSize(TRUE);
        DestroyAnimVisualTask(taskId);
    }
}
