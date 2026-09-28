//! Translated from `src/battle_anim_electric.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sAnim_Lightning sAnims_Lightning gLightningSpriteTemplate sAffineAnim_UnusedSpinningFist sAffineAnims_UnusedSpinningFist sUnusedSpinningFistSpriteTemplate sAnim_UnusedCirclingShock sAnims_UnusedCirclingShock sUnusedCirclingShockSpriteTemplate gSparkElectricitySpriteTemplate gZapCannonBallSpriteTemplate sAffineAnim_FlashingSpark sAffineAnims_FlashingSpark gZapCannonSparkSpriteTemplate sAnim_ThunderboltOrb sAnims_ThunderboltOrb sAffineAnim_ThunderboltOrb sAffineAnims_ThunderboltOrb gThunderboltOrbSpriteTemplate gSparkElectricityFlashingSpriteTemplate gElectricitySpriteTemplate gElectricBoltSegmentSpriteTemplate gThunderWaveSpriteTemplate sElectricChargingParticleCoordOffsets sAnim_ElectricChargingParticles_0 sAnim_ElectricChargingParticles_1 sAnims_ElectricChargingParticles gElectricChargingParticlesSpriteTemplate sAffineAnim_GrowingElectricOrb_0 sAffineAnim_GrowingElectricOrb_1 sAffineAnim_GrowingElectricOrb_2 sAffineAnims_GrowingElectricOrb gGrowingChargeOrbSpriteTemplate sAnim_ElectricPuff sAnims_ElectricPuff gElectricPuffSpriteTemplate gVoltTackleOrbSlideSpriteTemplate sAnim_VoltTackleBolt_0 sAnim_VoltTackleBolt_1 sAnim_VoltTackleBolt_2 sAnim_VoltTackleBolt_3 sAnims_VoltTackleBolt sAffineAnim_VoltTackleBolt sAffineAnims_VoltTackleBolt gVoltTackleBoltSpriteTemplate gGrowingShockWaveOrbSpriteTemplate gShockWaveProgressingBoltSpriteTemplate

static gElectricBoltSegmentSpriteTemplate: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_anim_electric::gElectricBoltSegmentSpriteTemplate).cast(),
);
static gElectricChargingParticlesSpriteTemplate: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_anim_electric::gElectricChargingParticlesSpriteTemplate).cast(),
);
static gLightningSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_electric::gLightningSpriteTemplate).cast());
static gShockWaveProgressingBoltSpriteTemplate: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_anim_electric::gShockWaveProgressingBoltSpriteTemplate).cast(),
);
static gThunderWaveSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_electric::gThunderWaveSpriteTemplate).cast());
static gVoltTackleBoltSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_electric::gVoltTackleBoltSpriteTemplate).cast());
static sElectricChargingParticleCoordOffsets: Table<CArray<CArray<i8, 2>, 16>> = Table(
    (&raw const crate::data::battle_anim_electric::sElectricChargingParticleCoordOffsets).cast(),
);

unsafe extern "C" {
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gOamMatrices: CArray<OamMatrix, 32>;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn AnimTranslateLinear(a0: *mut Sprite) -> u8;
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimSpriteAfterTimer(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn FreeOamMatrix(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn InitAnimLinearTranslation(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut Sprite, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut Sprite);
    fn RunStoredCallbackWhenAnimEnds(a0: *mut Sprite);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateSpriteInCircle(a0: *mut Sprite);
    fn WaitAnimForDuration(a0: *mut Sprite);
}

pub(crate) unsafe extern "C" fn AnimLightning(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= gBattleAnimArgs[0];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
    }
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).callback = Some(AnimLightning_Step);
}
pub(crate) unsafe extern "C" fn AnimLightning_Step(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedSpinningFist(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= gBattleAnimArgs[0];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
    }
    (*sprite).callback = Some(AnimUnusedSpinningFist_Step);
}
pub(crate) unsafe extern "C" fn AnimUnusedSpinningFist_Step(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        DestroySpriteAndMatrix(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedCirclingShock(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= gBattleAnimArgs[0];
        (*sprite).y -= gBattleAnimArgs[1];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
    }
    (*sprite).data[0] = 0;
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[2] = gBattleAnimArgs[3];
    (*sprite).data[3] = gBattleAnimArgs[4];
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(TranslateSpriteInCircle);
}
pub(crate) unsafe extern "C" fn AnimSparkElectricity(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    let mut matrixNum: u32 = 0;
    let mut sineVal: i16 = 0;
    match gBattleAnimArgs[4] {
        0 => {
            battler = gBattleAnimAttacker;
        }
        2 => {
            if IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) == 0 {
                battler = gBattleAnimAttacker;
            } else {
                battler = gBattleAnimAttacker ^ 2;
            }
        }
        3 => {
            if IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) != 0 {
                battler = gBattleAnimTarget ^ 2;
            } else {
                battler = gBattleAnimTarget;
            }
        }
        _ => {
            battler = gBattleAnimTarget;
        }
    }
    if gBattleAnimArgs[5] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as i16;
    } else {
        (*sprite).x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    }
    (*sprite).x2 = (gSineTable[gBattleAnimArgs[0]] as i32 * gBattleAnimArgs[1] as i32 >> 8) as i16;
    (*sprite).y2 =
        (gSineTable[gBattleAnimArgs[0] as i32 + 64] as i32 * gBattleAnimArgs[1] as i32 >> 8) as i16;
    if gBattleAnimArgs[6] as i32 & 1 != 0 {
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(battler) as u16 + 1);
    }
    matrixNum = (*sprite).oam.matrixNum();
    sineVal = gSineTable[gBattleAnimArgs[2]];
    gOamMatrices[matrixNum].a = {
        gOamMatrices[matrixNum].d = gSineTable[gBattleAnimArgs[2] as i32 + 64];
        gOamMatrices[matrixNum].d
    };
    gOamMatrices[matrixNum].b = sineVal;
    gOamMatrices[matrixNum].c = -sineVal;
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).callback = Some(DestroyAnimSpriteAfterTimer);
}
pub(crate) unsafe extern "C" fn AnimZapCannonSpark(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    InitAnimLinearTranslation(sprite);
    (*sprite).data[5] = gBattleAnimArgs[2];
    (*sprite).data[6] = gBattleAnimArgs[5];
    (*sprite).data[7] = gBattleAnimArgs[4];
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + gBattleAnimArgs[6] as u16 * 4);
    (*sprite).callback = Some(AnimZapCannonSpark_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimZapCannonSpark_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) == 0 {
        (*sprite).x2 += Sin((*sprite).data[7], (*sprite).data[5]);
        (*sprite).y2 += Cos((*sprite).data[7], (*sprite).data[5]);
        (*sprite).data[7] = (*sprite).data[7] + (*sprite).data[6] & 0xFF;
        if (*sprite).data[7] % 3 == 0 {
            (*sprite).set_invisible((*sprite).invisible() ^ 1);
        }
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimThunderboltOrb_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[5] -= 1;
        (*sprite).data[5]
    }) == -1
    {
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
        (*sprite).data[5] = (*sprite).data[4];
    }
    if ({
        let t2 = (*sprite).data[3];
        (*sprite).data[3] -= 1;
        t2
    }) <= 0
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimThunderboltOrb(sprite: *mut Sprite) {
    if IsContest() != 0 || GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
    }
    (*sprite).x =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[1];
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + gBattleAnimArgs[2];
    (*sprite).data[3] = gBattleAnimArgs[0];
    (*sprite).data[4] = gBattleAnimArgs[3];
    (*sprite).data[5] = gBattleAnimArgs[3];
    (*sprite).callback = Some(AnimThunderboltOrb_Step);
}
pub(crate) unsafe extern "C" fn AnimSparkElectricityFlashing(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    (*sprite).data[0] = gBattleAnimArgs[3];
    if gBattleAnimArgs[7] as i32 & 0x8000 != 0 {
        battler = gBattleAnimTarget;
    } else {
        battler = gBattleAnimAttacker;
    }
    if IsContest() != 0 || GetBattlerSide(battler) == B_SIDE_PLAYER {
        gBattleAnimArgs[0] = -gBattleAnimArgs[0];
    }
    (*sprite).x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[0];
    (*sprite).y =
        GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16 + gBattleAnimArgs[1];
    (*sprite).data[4] = gBattleAnimArgs[7] & 0x7FFF;
    (*sprite).data[5] = gBattleAnimArgs[2];
    (*sprite).data[6] = gBattleAnimArgs[5];
    (*sprite).data[7] = gBattleAnimArgs[4];
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + gBattleAnimArgs[6] as u16 * 4);
    (*sprite).callback = Some(AnimSparkElectricityFlashing_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimSparkElectricityFlashing_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[7], (*sprite).data[5]);
    (*sprite).y2 = Cos((*sprite).data[7], (*sprite).data[5]);
    (*sprite).data[7] = (*sprite).data[7] + (*sprite).data[6] & 0xFF;
    if rem_i32((*sprite).data[7] as i32, (*sprite).data[4] as i32) == 0 {
        (*sprite).set_invisible((*sprite).invisible() ^ TRUE as u16);
    }
    if ({
        let t1 = (*sprite).data[0];
        (*sprite).data[0] -= 1;
        t1
    }) <= 0
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimElectricity(sprite: *mut Sprite) {
    InitSpritePosToAnimTarget(sprite, FALSE);
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + gBattleAnimArgs[3] as u16 * 4);
    if gBattleAnimArgs[3] == 1 {
        (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
    } else if gBattleAnimArgs[3] == 2 {
        (*sprite).oam.set_matrixNum(ST_OAM_VFLIP);
    }
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ElectricBolt(taskId: u8) {
    gTasks[taskId].data[0] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16 + gBattleAnimArgs[0];
    gTasks[taskId].data[1] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + gBattleAnimArgs[1];
    gTasks[taskId].data[2] = gBattleAnimArgs[2];
    gTasks[taskId].func = Some(AnimTask_ElectricBolt_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_ElectricBolt_Step(taskId: u8) {
    let mut r8: u16 = 0;
    let mut r2: u16 = 0;
    let mut r12: i16 = 0;
    let mut spriteId: u8 = 0;
    let mut r7: u8 = 0;
    let mut sp: u8 = gTasks[taskId].data[2] as u8;
    let mut x: i16 = gTasks[taskId].data[0];
    let mut y: i16 = gTasks[taskId].data[1];
    if gTasks[taskId].data[2] == 0 {
        r8 = 0;
        r2 = 1;
        r12 = 16;
    } else {
        r12 = 16;
        r8 = 8;
        r2 = 4;
    }
    match gTasks[taskId].data[10] {
        0 => {
            r12 *= 1;
            spriteId = CreateSprite(
                (&raw const *gElectricBoltSegmentSpriteTemplate).cast_mut(),
                x,
                y + r12,
                2,
            );
            r7 += 1;
        }
        2 => {
            r12 *= 2;
            r8 += r2;
            spriteId = CreateSprite(
                (&raw const *gElectricBoltSegmentSpriteTemplate).cast_mut(),
                x,
                y + r12,
                2,
            );
            r7 += 1;
        }
        4 => {
            r12 *= 3;
            r8 += r2 * 2;
            spriteId = CreateSprite(
                (&raw const *gElectricBoltSegmentSpriteTemplate).cast_mut(),
                x,
                y + r12,
                2,
            );
            r7 += 1;
        }
        6 => {
            r12 *= 4;
            r8 += r2 * 3;
            spriteId = CreateSprite(
                (&raw const *gElectricBoltSegmentSpriteTemplate).cast_mut(),
                x,
                y + r12,
                2,
            );
            r7 += 1;
        }
        8 => {
            r12 *= 5;
            spriteId = CreateSprite(
                (&raw const *gElectricBoltSegmentSpriteTemplate).cast_mut(),
                x,
                y + r12,
                2,
            );
            r7 += 1;
        }
        10 => {
            DestroyAnimVisualTask(taskId);
            return;
        }
        _ => {}
    }
    if r7 != 0 {
        gSprites[spriteId]
            .oam
            .set_tileNum(gSprites[spriteId].oam.tileNum() + r8);
        gSprites[spriteId].data[0] = sp as i16;
        gSprites[spriteId].callback.unwrap_unchecked()(&raw mut gSprites[spriteId]);
    }
    gTasks[taskId].data[10] += 1;
}
pub(crate) unsafe extern "C" fn AnimElectricBoltSegment(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).oam.set_shape(2);
        (*sprite).oam.set_size(0);
    } else {
        (*sprite).oam.set_shape(0);
        (*sprite).oam.set_size(1);
    }
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) == 15
    {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimThunderWave(sprite: *mut Sprite) {
    let mut spriteId: u8 = 0;
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += gBattleAnimArgs[1];
    spriteId = CreateSprite(
        (&raw const *gThunderWaveSpriteTemplate).cast_mut(),
        (*sprite).x + 32,
        (*sprite).y,
        (*sprite).subpriority,
    );
    gSprites[spriteId]
        .oam
        .set_tileNum(gSprites[spriteId].oam.tileNum() + 8);
    gAnimVisualTaskCount += 1;
    gSprites[spriteId].callback = Some(AnimThunderWave_Step);
    (*sprite).callback = Some(AnimThunderWave_Step);
}
pub(crate) unsafe extern "C" fn AnimThunderWave_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 3
    {
        (*sprite).data[0] = 0;
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
    }
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) == 51
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ElectricChargingParticles(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if gBattleAnimArgs[0] == 0 {
        (*task).data[14] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*task).data[15] =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    } else {
        (*task).data[14] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*task).data[15] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    }
    (*task).data[6] = gBattleAnimArgs[1];
    (*task).data[7] = 0;
    (*task).data[8] = 0;
    (*task).data[9] = 0;
    (*task).data[10] = 0;
    (*task).data[11] = gBattleAnimArgs[3];
    (*task).data[12] = 0;
    (*task).data[13] = gBattleAnimArgs[2];
    (*task).func = Some(AnimTask_ElectricChargingParticles_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_ElectricChargingParticles_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if (*task).data[6] != 0 {
        if ({
            (*task).data[12] += 1;
            (*task).data[12]
        }) > (*task).data[13]
        {
            let mut spriteId: u8 = 0;
            (*task).data[12] = 0;
            spriteId = CreateSprite(
                (&raw const *gElectricChargingParticlesSpriteTemplate).cast_mut(),
                (*task).data[14],
                (*task).data[15],
                2,
            );
            if spriteId != MAX_SPRITES {
                let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
                (*sprite).x += sElectricChargingParticleCoordOffsets[(*task).data[9]][0] as i16;
                (*sprite).y += sElectricChargingParticleCoordOffsets[(*task).data[9]][1] as i16;
                (*sprite).data[0] = 40 - (*task).data[8] * 5;
                (*sprite).data[1] = (*sprite).x;
                (*sprite).data[2] = (*task).data[14];
                (*sprite).data[3] = (*sprite).y;
                (*sprite).data[4] = (*task).data[15];
                (*sprite).data[5] = taskId as i16;
                InitAnimLinearTranslation(sprite);
                StoreSpriteCallbackInData6(sprite, Some(AnimElectricChargingParticles));
                (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
                if ({
                    (*task).data[9] += 1;
                    (*task).data[9]
                }) > 15
                {
                    (*task).data[9] = 0;
                }
                if ({
                    (*task).data[10] += 1;
                    (*task).data[10]
                }) >= (*task).data[11]
                {
                    (*task).data[10] = 0;
                    if (*task).data[8] <= 5 {
                        (*task).data[8] += 1;
                    }
                }
                (*task).data[7] += 1;
                (*task).data[6] -= 1;
            }
        }
    } else if (*task).data[7] == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimElectricChargingParticles_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 {
        gTasks[(*sprite).data[5]].data[7] -= 1;
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimElectricChargingParticles(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, 1);
    (*sprite).callback = Some(AnimElectricChargingParticles_Step);
}
pub(crate) unsafe extern "C" fn AnimGrowingChargeOrb(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    } else {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    }
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
}
pub(crate) unsafe extern "C" fn AnimElectricPuff(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    } else {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    }
    (*sprite).x2 = gBattleAnimArgs[1];
    (*sprite).y2 = gBattleAnimArgs[2];
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
}
pub(crate) unsafe extern "C" fn AnimVoltTackleOrbSlide(sprite: *mut Sprite) {
    StartSpriteAffineAnim(sprite, 1);
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[6] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*sprite).data[7] = 16;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
        (*sprite).data[7] *= -1;
    }
    (*sprite).callback = Some(AnimVoltTackleOrbSlide_Step);
}
pub(crate) unsafe extern "C" fn AnimVoltTackleOrbSlide_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 40
            {
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            (*sprite).x += (*sprite).data[7];
            gSprites[(*sprite).data[6]].x2 += (*sprite).data[7];
            if (*sprite).x as u16 as i32 + 80 > 400 {
                DestroySpriteAndMatrix(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_VoltTackleAttackerReappear(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[15] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
            (*task).data[14] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
            if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                (*task).data[14] = -32;
                (*task).data[13] = 2;
            } else {
                (*task).data[14] = 32;
                (*task).data[13] = -2;
            }
            gSprites[(*task).data[15]].x2 = (*task).data[14];
            (*task).data[0] += 1;
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                gSprites[(*task).data[15]]
                    .set_invisible(gSprites[(*task).data[15]].invisible() ^ 1);
                if (*task).data[14] != 0 {
                    (*task).data[14] += (*task).data[13];
                    gSprites[(*task).data[15]].x2 = (*task).data[14];
                } else {
                    (*task).data[0] += 1;
                }
            }
        }
        2 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                gSprites[(*task).data[15]]
                    .set_invisible(gSprites[(*task).data[15]].invisible() ^ 1);
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) == 8
                {
                    (*task).data[0] += 1;
                }
            }
        }
        3 => {
            gSprites[(*task).data[15]].set_invisible(FALSE as u16);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_VoltTackleBolt(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[1] = (if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                1
            } else {
                -1
            }) as i16;
            match gBattleAnimArgs[0] {
                0 => {
                    (*task).data[3] =
                        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
                    (*task).data[5] =
                        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET)
                            as i16;
                    (*task).data[4] = (*task).data[1] * 128 + 120;
                }
                4 => {
                    (*task).data[3] = 120 - (*task).data[1] * 128;
                    (*task).data[5] =
                        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
                    (*task).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2)
                        as i16
                        - (*task).data[1] * 32;
                }
                _ => {
                    if gBattleAnimArgs[0] as i32 & 1 != 0 {
                        (*task).data[3] = 256;
                        (*task).data[4] = -16;
                    } else {
                        (*task).data[3] = -16;
                        (*task).data[4] = 256;
                    }
                    if (*task).data[1] == 1 {
                        (*task).data[5] = 80 - gBattleAnimArgs[0] * 10;
                    } else {
                        let mut temp: u16 = 0;
                        (*task).data[5] = gBattleAnimArgs[0] * 10 + 40;
                        temp = (*task).data[3] as u16;
                        (*task).data[3] = (*task).data[4];
                        (*task).data[4] = temp as i16;
                    }
                }
            }
            if (*task).data[3] < (*task).data[4] {
                (*task).data[1] = 1;
                (*task).data[6] = 0;
            } else {
                (*task).data[1] = -1;
                (*task).data[6] = 3;
            }
            (*task).data[0] += 1;
        }
        1 => {
            if ({
                (*task).data[2] += 1;
                (*task).data[2]
            }) > 0
            {
                (*task).data[2] = 0;
                if CreateVoltTackleBolt(task, taskId) != 0
                    || CreateVoltTackleBolt(task, taskId) != 0
                {
                    (*task).data[0] += 1;
                }
            }
        }
        2 => {
            if (*task).data[7] == 0 {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateVoltTackleBolt(task: *mut Task, taskId: u8) -> u8 {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *gVoltTackleBoltSpriteTemplate).cast_mut(),
        (*task).data[3],
        (*task).data[5],
        35,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[6] = taskId as i16;
        gSprites[spriteId].data[7] = 7;
        (*task).data[7] += 1;
    }
    (*task).data[6] += (*task).data[1];
    if (*task).data[6] < 0 {
        (*task).data[6] = 3;
    }
    if (*task).data[6] > 3 {
        (*task).data[6] = 0;
    }
    (*task).data[3] += (*task).data[1] * 16;
    if (*task).data[1] == 1 && (*task).data[3] >= (*task).data[4]
        || (*task).data[1] == -1 && (*task).data[3] <= (*task).data[4]
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AnimVoltTackleBolt(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 12
    {
        gTasks[(*sprite).data[6]].data[(*sprite).data[7]] -= 1;
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimGrowingShockWaveOrb(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
            (*sprite).y =
                GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
            StartSpriteAffineAnim(sprite, 2);
            (*sprite).data[0] += 1;
        }
        1 => {
            if (*sprite).affineAnimEnded() != 0 {
                DestroySpriteAndMatrix(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ShockWaveProgressingBolt(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[6] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
            (*task).data[7] =
                GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
            (*task).data[8] = 4;
            (*task).data[10] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
            (*task).data[9] = (((*task).data[10] as i32 - (*task).data[6] as i32) / 5) as i16;
            (*task).data[4] = 7;
            (*task).data[5] = -1;
            (*task).data[11] = 12;
            (*task).data[12] = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER) as i16;
            (*task).data[13] = BattleAnimAdjustPanning(SOUND_PAN_TARGET) as i16;
            (*task).data[14] = (*task).data[12];
            (*task).data[15] = (((*task).data[13] as i32 - (*task).data[12] as i32) / 3) as i16;
            (*task).data[0] += 1;
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 0
            {
                (*task).data[1] = 0;
                if CreateShockWaveBoltSprite(task, taskId) != 0 {
                    if (*task).data[2] == 5 {
                        (*task).data[0] = 3;
                    } else {
                        (*task).data[0] += 1;
                    }
                }
            }
            if (*task).data[11] != 0 {
                (*task).data[11] -= 1;
            }
        }
        2 => {
            if (*task).data[11] != 0 {
                (*task).data[11] -= 1;
            }
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 4
            {
                (*task).data[1] = 0;
                if (*task).data[2] as i32 & 1 != 0 {
                    (*task).data[7] = 4;
                    (*task).data[8] = 68;
                    (*task).data[4] = 0;
                    (*task).data[5] = 1;
                } else {
                    (*task).data[7] = 68;
                    (*task).data[8] = 4;
                    (*task).data[4] = 7;
                    (*task).data[5] = -1;
                }
                if (*task).data[11] != 0 {
                    (*task).data[0] = 4;
                } else {
                    (*task).data[0] = 1;
                }
            }
        }
        3 => {
            if (*task).data[3] == 0 {
                DestroyAnimVisualTask(taskId);
            }
        }
        4 => {
            if (*task).data[11] != 0 {
                (*task).data[11] -= 1;
            } else {
                (*task).data[0] = 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateShockWaveBoltSprite(task: *mut Task, taskId: u8) -> u8 {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *gShockWaveProgressingBoltSpriteTemplate).cast_mut(),
        (*task).data[6],
        (*task).data[7],
        35,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId]
            .oam
            .set_tileNum(gSprites[spriteId].oam.tileNum() + (*task).data[4] as u16);
        (*task).data[4] += (*task).data[5];
        if (*task).data[4] < 0 {
            (*task).data[4] = 7;
        }
        if (*task).data[4] > 7 {
            (*task).data[4] = 0;
        }
        gSprites[spriteId].data[6] = taskId as i16;
        gSprites[spriteId].data[7] = 3;
        (*task).data[3] += 1;
    }
    if (*task).data[4] == 0 && (*task).data[5] > 0 {
        (*task).data[14] += (*task).data[15];
        PlaySE12WithPanning(SE_M_THUNDERBOLT, (*task).data[14] as i8);
    }
    if (*task).data[5] < 0 && (*task).data[7] <= (*task).data[8]
        || (*task).data[5] > 0 && (*task).data[7] >= (*task).data[8]
    {
        (*task).data[2] += 1;
        (*task).data[6] += (*task).data[9];
        return TRUE;
    } else {
        (*task).data[7] += (*task).data[5] * 8;
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AnimShockWaveProgressingBolt(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 12
    {
        gTasks[(*sprite).data[6]].data[(*sprite).data[7]] -= 1;
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ShockWaveLightning(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[15] =
                GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + 32;
            (*task).data[14] = (*task).data[15];
            while (*task).data[14] > 16 {
                (*task).data[14] -= 32;
            }
            (*task).data[13] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
            (*task).data[12] = GetBattlerSpriteSubpriority(gBattleAnimTarget) as i16 - 2;
            (*task).data[0] += 1;
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                if CreateShockWaveLightningSprite(task, taskId) != 0 {
                    (*task).data[0] += 1;
                }
            }
        }
        2 => {
            if (*task).data[10] == 0 {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateShockWaveLightningSprite(task: *mut Task, taskId: u8) -> u8 {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *gLightningSpriteTemplate).cast_mut(),
        (*task).data[13],
        (*task).data[14],
        (*task).data[12] as u8,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].callback = Some(AnimShockWaveLightning);
        gSprites[spriteId].data[6] = taskId as i16;
        gSprites[spriteId].data[7] = 10;
        (*task).data[10] += 1;
    }
    if (*task).data[14] >= (*task).data[15] {
        return TRUE;
    }
    (*task).data[14] += 32;
    return FALSE;
}
pub(crate) unsafe extern "C" fn AnimShockWaveLightning(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        gTasks[(*sprite).data[6]].data[(*sprite).data[7]] -= 1;
        DestroySprite(sprite);
    }
}
