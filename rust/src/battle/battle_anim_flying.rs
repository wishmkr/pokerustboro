//! Translated from `src/battle_anim_flying.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gEllipticalGustSpriteTemplate sAffineAnim_GustToTarget sAffineAnims_GustToTarget gGustToTargetSpriteTemplate sAffineAnim_AirWaveCrescent sAffineAnims_AirWaveCrescent gAirWaveCrescentSpriteTemplate sAffineAnim_FlyBallUp sAffineAnims_FlyBallUp sAffineAnim_FlyBallAttack_0 sAffineAnim_FlyBallAttack_1 sAffineAnims_FlyBallAttack gFlyBallUpSpriteTemplate gFlyBallAttackSpriteTemplate sAnim_FallingFeather_0 sAnim_FallingFeather_1 sAnims_FallingFeather gFallingFeatherSpriteTemplate sUnusedBubbleThrowSpriteTemplate sAnim_WhirlwindLines sAnims_WhirlwindLines gWhirlwindLineSpriteTemplate sAffineAnim_BounceBallShrink sAffineAnims_BounceBallShrink gBounceBallShrinkSpriteTemplate sAffineAnim_BounceBallLand sAffineAnims_BounceBallLand gBounceBallLandSpriteTemplate sAffineAnim_DiveBall sAffineAnims_DiveBall gDiveBallSpriteTemplate sAnim_Unused sAnims_Unused gDiveWaterSplashSpriteTemplate gSprayWaterDropletSpriteTemplate sUnusedFlashingLightSpriteTemplate gSkyAttackBirdSpriteTemplate

/// `struct FeatherDanceData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct FeatherDanceData {
    bits_0: u8,
    bits_1: u8,
    pub unk2: u16,
    pub unk4: i16,
    pub unk6: u16,
    pub unk8: u16,
    pub unkA: u16,
    pub unkC: CArray<u8, 2>,
    bits_14: u16,
}

impl FeatherDanceData {
    #[inline(always)]
    pub fn unk0_0a(&self) -> u16 {
        ((self.bits_0 as u32 >> 0) & 0x1) as u16
    }
    #[inline(always)]
    pub fn set_unk0_0a(&mut self, v: u16) {
        self.bits_0 = (self.bits_0 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn unk0_0b(&self) -> u16 {
        ((self.bits_0 as u32 >> 1) & 0x1) as u16
    }
    #[inline(always)]
    pub fn set_unk0_0b(&mut self, v: u16) {
        self.bits_0 = (self.bits_0 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn unk0_0c(&self) -> u16 {
        ((self.bits_0 as u32 >> 2) & 0x1) as u16
    }
    #[inline(always)]
    pub fn set_unk0_0c(&mut self, v: u16) {
        self.bits_0 = (self.bits_0 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn unk0_0d(&self) -> u16 {
        ((self.bits_0 as u32 >> 3) & 0x1) as u16
    }
    #[inline(always)]
    pub fn set_unk0_0d(&mut self, v: u16) {
        self.bits_0 = (self.bits_0 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn unk0_1(&self) -> u16 {
        ((self.bits_0 as u32 >> 4) & 0xf) as u16
    }
    #[inline(always)]
    pub fn set_unk0_1(&mut self, v: u16) {
        self.bits_0 = (self.bits_0 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
    #[inline(always)]
    pub fn unk1(&self) -> u16 {
        ((self.bits_1 as u32 >> 0) & 0xff) as u16
    }
    #[inline(always)]
    pub fn set_unk1(&mut self, v: u16) {
        self.bits_1 = (self.bits_1 & !(0xff << 0)) | ((v as u8 & 0xff) << 0);
    }
    #[inline(always)]
    pub fn unkE_0(&self) -> u16 {
        ((self.bits_14 as u32 >> 0) & 0x1) as u16
    }
    #[inline(always)]
    pub fn set_unkE_0(&mut self, v: u16) {
        self.bits_14 = (self.bits_14 & !(0x1 << 0)) | ((v as u16 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn unkE_1(&self) -> u16 {
        ((self.bits_14 as u32 >> 1) & 0x7fff) as u16
    }
    #[inline(always)]
    pub fn set_unkE_1(&mut self, v: u16) {
        self.bits_14 = (self.bits_14 & !(0x7fff << 1)) | ((v as u16 & 0x7fff) << 1);
    }
}

unsafe impl Sync for FeatherDanceData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<FeatherDanceData>() == 16);
    assert!(offset_of!(FeatherDanceData, bits_0) == 0);
    assert!(offset_of!(FeatherDanceData, bits_1) == 1);
    assert!(offset_of!(FeatherDanceData, unk2) == 2);
    assert!(offset_of!(FeatherDanceData, unk4) == 4);
    assert!(offset_of!(FeatherDanceData, unk6) == 6);
    assert!(offset_of!(FeatherDanceData, unk8) == 8);
    assert!(offset_of!(FeatherDanceData, unkA) == 10);
    assert!(offset_of!(FeatherDanceData, unkC) == 12);
    assert!(offset_of!(FeatherDanceData, bits_14) == 14);
};

unsafe extern "C" {
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static gFlashingHitSplatSpriteTemplate: SpriteTemplate;
    static mut gOamMatrices: CArray<OamMatrix, 32>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn AnimTranslateLinear(a0: *mut Sprite) -> u8;
    fn ArcTan2Neg(a0: i16, a1: i16) -> u16;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSpriteAndAnimate(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn FreeOamMatrix(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAnimLinearTranslation(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut Sprite, a1: u8);
    fn IsContest() -> u8;
    fn Random2() -> u16;
    fn ResetSpriteRotScale_PreserveAffine(a0: *mut Sprite);
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut Sprite);
    fn SeekSpriteAnim(a0: *mut Sprite, a1: u8);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateAnimSpriteToTargetMonLocation(a0: *mut Sprite);
    fn TrySetSpriteRotScale(a0: *mut Sprite, a1: u8, a2: i16, a3: i16, a4: u16);
}

pub(crate) unsafe extern "C" fn AnimEllipticalGust(sprite: *mut Sprite) {
    InitSpritePosToAnimTarget(sprite, FALSE);
    (*sprite).y += 20;
    (*sprite).data[1] = 191;
    (*sprite).callback = Some(AnimEllipticalGust_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimEllipticalGust_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[1], 32);
    (*sprite).y2 = Cos((*sprite).data[1], 8);
    (*sprite).data[1] += 5;
    (*sprite).data[1] &= 0xFF;
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 71
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AnimateGustTornadoPalette(taskId: u8) {
    gTasks[taskId].data[0] = gBattleAnimArgs[1];
    gTasks[taskId].data[1] = gBattleAnimArgs[0];
    gTasks[taskId].data[2] = IndexOfSpritePaletteTag(ANIM_TAG_GUST) as i16;
    gTasks[taskId].func = Some(AnimTask_AnimateGustTornadoPalette_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_AnimateGustTornadoPalette_Step(taskId: u8) {
    let mut data2: u8 = 0;
    let mut temp: u16 = 0;
    let mut i: i32 = 0;
    let mut base: i32 = 0;
    if ({
        let t1 = gTasks[taskId].data[10];
        gTasks[taskId].data[10] += 1;
        t1
    }) == gTasks[taskId].data[1]
    {
        gTasks[taskId].data[10] = 0;
        data2 = gTasks[taskId].data[2] as u8;
        temp = gPlttBufferFaded[0x100 + data2 as i32 * 16 + 8];
        i = 7;
        base = data2 as i32 * 16;
        loop {
            gPlttBufferFaded[base + OBJ_PLTT_OFFSET as i32 + 1 + i] =
                gPlttBufferFaded[base + OBJ_PLTT_OFFSET as i32 + i];
            i -= 1;
            if i <= 0 {
                break;
            }
        }
        gPlttBufferFaded[base + OBJ_PLTT_OFFSET as i32 + 1] = temp;
    }
    if ({
        gTasks[taskId].data[0] -= 1;
        gTasks[taskId].data[0]
    }) == 0
    {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimGustToTarget(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[2];
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + gBattleAnimArgs[3];
    InitAnimLinearTranslation(sprite);
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(AnimGustToTarget_Step));
}
pub(crate) unsafe extern "C" fn AnimGustToTarget_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimAirWaveCrescent(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[0] = -gBattleAnimArgs[0];
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
        gBattleAnimArgs[3] = -gBattleAnimArgs[3];
    }
    if IsContest() != 0 {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        gBattleAnimArgs[3] = -gBattleAnimArgs[3];
    }
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[4];
    if gBattleAnimArgs[6] == 0 {
        (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).data[4] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    } else {
        SetAverageBattlerPositions(
            gBattleAnimTarget,
            TRUE,
            &raw mut (*sprite).data[2],
            &raw mut (*sprite).data[4],
        );
    }
    (*sprite).data[2] = (*sprite).data[2] + gBattleAnimArgs[2];
    (*sprite).data[4] = (*sprite).data[4] + gBattleAnimArgs[3];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    SeekSpriteAnim(sprite, gBattleAnimArgs[5] as u8);
}
pub(crate) unsafe extern "C" fn AnimFlyBallUp(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).callback = Some(AnimFlyBallUp_Step);
    gSprites[GetAnimBattlerSpriteId(ANIM_ATTACKER)].set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn AnimFlyBallUp_Step(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
    } else {
        (*sprite).data[2] += (*sprite).data[1];
        (*sprite).y2 -= (*sprite).data[2] >> 8;
    }
    if ((*sprite).y as i32 + (*sprite).y2 as i32) < -32 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimFlyBallAttack(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x = 272;
        (*sprite).y = -32;
        StartSpriteAffineAnim(sprite, 1);
    } else {
        (*sprite).x = -32;
        (*sprite).y = -32;
    }
    (*sprite).data[0] = gBattleAnimArgs[0];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    InitAnimLinearTranslation(sprite);
    (*sprite).callback = Some(AnimFlyBallAttack_Step);
}
pub(crate) unsafe extern "C" fn AnimFlyBallAttack_Step(sprite: *mut Sprite) {
    (*sprite).data[0] = 1;
    AnimTranslateLinear(sprite);
    if (*sprite).data[3] as u16 >> 8 > 200 {
        (*sprite).x += (*sprite).x2;
        (*sprite).x2 = 0;
        (*sprite).data[3] &= 0xFF;
    }
    if ((*sprite).x as i32 + (*sprite).x2 as i32) < -32
        || (*sprite).x as i32 + (*sprite).x2 as i32 > 272
        || (*sprite).y as i32 + (*sprite).y2 as i32 > DISPLAY_HEIGHT as i32
    {
        gSprites[GetAnimBattlerSpriteId(0)].set_invisible(0);
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimSpriteAfterTimer(sprite: *mut Sprite) {
    if ({
        let t1 = (*sprite).data[0];
        (*sprite).data[0] -= 1;
        t1
    }) <= 0
    {
        if (*sprite).oam.affineMode() & ST_OAM_AFFINE_ON_MASK != 0 {
            FreeOamMatrix((*sprite).oam.matrixNum() as u8);
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
        }
        DestroySprite(sprite);
        gAnimVisualTaskCount -= 1;
    }
}
pub(crate) unsafe extern "C" fn AnimFallingFeather(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    let mut matrixNum: u8 = 0;
    let mut sinIndex: u8 = 0;
    let mut spriteCoord: i16 = 0;
    let mut data: *mut FeatherDanceData = (*sprite).data.as_mut_ptr() as *mut FeatherDanceData;
    if gBattleAnimArgs[7] as i32 & 0x100 != 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        gBattleAnimArgs[0] = -gBattleAnimArgs[0];
    }
    (*sprite).x =
        GetBattlerSpriteCoord(battler, BATTLER_COORD_ATTR_HEIGHT) as i16 + gBattleAnimArgs[0];
    spriteCoord = GetBattlerSpriteCoord(battler, BATTLER_COORD_ATTR_WIDTH) as i16;
    (*sprite).y = spriteCoord + gBattleAnimArgs[1];
    (*data).unk8 = ((*sprite).y as u16) << 8;
    (*data).set_unkE_1(spriteCoord as u16 + gBattleAnimArgs[6] as u16);
    (*data).set_unk0_0c(1);
    (*data).unk2 = gBattleAnimArgs[2] as u16 & 0xFF;
    (*data).unkA = (gBattleAnimArgs[2] >> 8) as u16 & 0xFF;
    (*data).unk4 = gBattleAnimArgs[3];
    (*data).unk6 = gBattleAnimArgs[4] as u16;
    *((*data).unkC.as_mut_ptr() as *mut u16) = gBattleAnimArgs[5] as u16;
    if (*data).unk2 >= 64 && (*data).unk2 <= 191 {
        if IsContest() == 0 {
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority(battler) as u16 + 1);
        } else {
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority(battler) as u16);
        }
        (*data).set_unkE_0(0);
        if (*data).unk4 as i32 & 0x8000 == 0 {
            (*sprite).set_hFlip((*sprite).hFlip() ^ 1);
            (*sprite).animNum = (*sprite).hFlip() as u8;
            (*sprite).set_animBeginning(1);
            (*sprite).set_animEnded(0);
        }
    } else {
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(battler) as u16);
        (*data).set_unkE_0(1);
        if (*data).unk4 as i32 & 0x8000 != 0 {
            (*sprite).set_hFlip((*sprite).hFlip() ^ 1);
            (*sprite).animNum = (*sprite).hFlip() as u8;
            (*sprite).set_animBeginning(1);
            (*sprite).set_animEnded(0);
        }
    }
    (*data).set_unk0_1((*data).unk2 >> 6);
    (*sprite).x2 = (gSineTable[(*data).unk2] as i32 * (*data).unkC[0] as i32 >> 8) as i16;
    matrixNum = (*sprite).oam.matrixNum() as u8;
    sinIndex = (-((*sprite).x2 as i32) >> 1) as u8 + (*data).unkA as u8;
    spriteCoord = gSineTable[sinIndex];
    gOamMatrices[matrixNum].a = {
        gOamMatrices[matrixNum].d = gSineTable[sinIndex as i32 + 64];
        gOamMatrices[matrixNum].d
    };
    gOamMatrices[matrixNum].b = spriteCoord;
    gOamMatrices[matrixNum].c = -spriteCoord;
    (*sprite).callback = Some(AnimFallingFeather_Step);
}
pub(crate) unsafe extern "C" fn AnimFallingFeather_Step(sprite: *mut Sprite) {
    let mut matrixNum: u8 = 0;
    let mut sinIndex: u8 = 0;
    let mut sinVal: i16 = 0;
    let mut data: *mut FeatherDanceData = (*sprite).data.as_mut_ptr() as *mut FeatherDanceData;
    if (*data).unk0_0a() != 0 {
        if ({
            let t1 = (*data).unk1();
            (*data).set_unk1((*data).unk1() - 1);
            t1
        }) as i32
            % 256
            == 0
        {
            (*data).set_unk0_0a(0);
            (*data).set_unk1(0);
        }
    } else {
        match (*data).unk2 as i32 / 64 {
            0 => {
                if (*data).unk0_1() as u8 == 1 {
                    (*data).set_unk0_0d(1);
                    (*data).set_unk0_0a(1);
                    (*data).set_unk1(0);
                } else if (*data).unk0_1() as u8 == 3 {
                    (*data).set_unk0_0b((*data).unk0_0b() ^ 1);
                    (*data).set_unk0_0a(1);
                    (*data).set_unk1(0);
                } else if (*data).unk0_0d() != 0 {
                    (*sprite).set_hFlip((*sprite).hFlip() ^ 1);
                    (*sprite).animNum = (*sprite).hFlip() as u8;
                    (*sprite).set_animBeginning(TRUE as u16);
                    (*sprite).set_animEnded(FALSE as u16);
                    if (*data).unk0_0c() != 0 {
                        if IsContest() == 0 {
                            if (*data).unkE_0() == 0 {
                                (*sprite).oam.set_priority((*sprite).oam.priority() - 1);
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            } else {
                                (*sprite).oam.set_priority((*sprite).oam.priority() + 1);
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            }
                        } else {
                            if (*data).unkE_0() == 0 {
                                (*sprite).subpriority -= 12;
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            } else {
                                (*sprite).subpriority += 12;
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            }
                        }
                    }
                    (*data).set_unk0_0d(0);
                }
                (*data).set_unk0_1(0);
            }
            1 => {
                if (*data).unk0_1() as u8 == 0 {
                    (*data).set_unk0_0d(1);
                    (*data).set_unk0_0a(1);
                    (*data).set_unk1(0);
                } else if (*data).unk0_1() as u8 == 2 {
                    (*data).set_unk0_0a(1);
                    (*data).set_unk1(0);
                } else if (*data).unk0_0d() != 0 {
                    (*sprite).set_hFlip((*sprite).hFlip() ^ 1);
                    (*sprite).animNum = (*sprite).hFlip() as u8;
                    (*sprite).set_animBeginning(TRUE as u16);
                    (*sprite).set_animEnded(FALSE as u16);
                    if (*data).unk0_0c() != 0 {
                        if IsContest() == 0 {
                            if (*data).unkE_0() == 0 {
                                (*sprite).oam.set_priority((*sprite).oam.priority() - 1);
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            } else {
                                (*sprite).oam.set_priority((*sprite).oam.priority() + 1);
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            }
                        } else {
                            if (*data).unkE_0() == 0 {
                                (*sprite).subpriority -= 12;
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            } else {
                                (*sprite).subpriority += 12;
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            }
                        }
                    }
                    (*data).set_unk0_0d(0);
                }
                (*data).set_unk0_1(1);
            }
            2 => {
                if (*data).unk0_1() as u8 == 3 {
                    (*data).set_unk0_0d(1);
                    (*data).set_unk0_0a(1);
                    (*data).set_unk1(0);
                } else if (*data).unk0_1() as u8 == 1 {
                    (*data).set_unk0_0a(1);
                    (*data).set_unk1(0);
                } else if (*data).unk0_0d() != 0 {
                    (*sprite).set_hFlip((*sprite).hFlip() ^ 1);
                    (*sprite).animNum = (*sprite).hFlip() as u8;
                    (*sprite).set_animBeginning(TRUE as u16);
                    (*sprite).set_animEnded(FALSE as u16);
                    if (*data).unk0_0c() != 0 {
                        if IsContest() == 0 {
                            if (*data).unkE_0() == 0 {
                                (*sprite).oam.set_priority((*sprite).oam.priority() - 1);
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            } else {
                                (*sprite).oam.set_priority((*sprite).oam.priority() + 1);
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            }
                        } else {
                            if (*data).unkE_0() == 0 {
                                (*sprite).subpriority -= 12;
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            } else {
                                (*sprite).subpriority += 12;
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            }
                        }
                    }
                    (*data).set_unk0_0d(0);
                }
                (*data).set_unk0_1(2);
            }
            3 => {
                if (*data).unk0_1() as u8 == 2 {
                    (*data).set_unk0_0d(1);
                } else if (*data).unk0_1() as u8 == 0 {
                    (*data).set_unk0_0b((*data).unk0_0b() ^ 1);
                    (*data).set_unk0_0a(1);
                    (*data).set_unk1(0);
                } else if (*data).unk0_0d() != 0 {
                    (*sprite).set_hFlip((*sprite).hFlip() ^ 1);
                    (*sprite).animNum = (*sprite).hFlip() as u8;
                    (*sprite).set_animBeginning(TRUE as u16);
                    (*sprite).set_animEnded(FALSE as u16);
                    if (*data).unk0_0c() != 0 {
                        if IsContest() == 0 {
                            if (*data).unkE_0() == 0 {
                                (*sprite).oam.set_priority((*sprite).oam.priority() - 1);
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            } else {
                                (*sprite).oam.set_priority((*sprite).oam.priority() + 1);
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            }
                        } else {
                            if (*data).unkE_0() == 0 {
                                (*sprite).subpriority -= 12;
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            } else {
                                (*sprite).subpriority += 12;
                                (*data).set_unkE_0((*data).unkE_0() ^ 1);
                            }
                        }
                    }
                    (*data).set_unk0_0d(0);
                }
                (*data).set_unk0_1(3);
            }
            _ => {}
        }
        (*sprite).x2 =
            ((*data).unkC[(*data).unk0_0b()] as i32 * gSineTable[(*data).unk2] as i32 >> 8) as i16;
        matrixNum = (*sprite).oam.matrixNum() as u8;
        sinIndex = (-((*sprite).x2 as i32) >> 1) as u8 + (*data).unkA as u8;
        sinVal = gSineTable[sinIndex];
        gOamMatrices[matrixNum].a = {
            gOamMatrices[matrixNum].d = gSineTable[sinIndex as i32 + 64];
            gOamMatrices[matrixNum].d
        };
        gOamMatrices[matrixNum].b = sinVal;
        gOamMatrices[matrixNum].c = -sinVal;
        (*data).unk8 += (*data).unk6;
        (*sprite).y = ((*data).unk8 >> 8) as i16;
        if (*data).unk4 as i32 & 0x8000 != 0 {
            (*data).unk2 = (*data).unk2 - ((*data).unk4 as u16 & 0x7FFF) & 0xFF;
        } else {
            (*data).unk2 = (*data).unk2 + ((*data).unk4 as u16 & 0x7FFF) & 0xFF;
        }
        if (*sprite).y as i32 + (*sprite).y2 as i32 >= (*data).unkE_1() as i32 {
            (*sprite).data[0] = 0;
            (*sprite).callback = Some(DestroyAnimSpriteAfterTimer);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedBubbleThrow(sprite: *mut Sprite) {
    (*sprite)
        .oam
        .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16);
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).callback = Some(TranslateAnimSpriteToTargetMonLocation);
}
pub(crate) unsafe extern "C" fn AnimWhirlwindLine(sprite: *mut Sprite) {
    let mut offset: u16 = 0;
    let mut mult: u8 = 0;
    if gBattleAnimArgs[2] == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, FALSE);
    } else {
        InitSpritePosToAnimTarget(sprite, FALSE);
    }
    if gBattleAnimArgs[2] == 0 && GetBattlerSide(gBattleAnimAttacker) == 0
        || gBattleAnimArgs[2] == ANIM_TARGET as i16
            && GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER
    {
        (*sprite).x += 8;
    }
    SeekSpriteAnim(sprite, gBattleAnimArgs[4] as u8);
    (*sprite).x -= 32;
    (*sprite).data[1] = 0x0ccc;
    offset = gBattleAnimArgs[4] as u16;
    mult = 12;
    (*sprite).x2 += mult as i16 * offset as i16;
    (*sprite).data[0] = offset as i16;
    (*sprite).data[7] = gBattleAnimArgs[3];
    (*sprite).callback = Some(AnimWhirlwindLine_Step);
}
pub(crate) unsafe extern "C" fn AnimWhirlwindLine_Step(sprite: *mut Sprite) {
    (*sprite).x2 += (*sprite).data[1] >> 8;
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 6
    {
        (*sprite).data[0] = 0;
        (*sprite).x2 = 0;
        StartSpriteAnim(sprite, 0);
    }
    if ({
        (*sprite).data[7] -= 1;
        (*sprite).data[7]
    }) == -1
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DrillPeckHitSplats(task: u8) {
    if gTasks[task].data[0] % 32 == 0 {
        gAnimVisualTaskCount += 1;
        gBattleAnimArgs[0] = Sin(gTasks[task].data[0], -13);
        gBattleAnimArgs[1] = Cos(gTasks[task].data[0], -13);
        gBattleAnimArgs[2] = 1;
        gBattleAnimArgs[3] = 3;
        CreateSpriteAndAnimate(
            (&raw const gFlashingHitSplatSpriteTemplate).cast_mut(),
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16,
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16,
            3,
        );
    }
    gTasks[task].data[0] += 8;
    if gTasks[task].data[0] > 255 {
        DestroyAnimVisualTask(task);
    }
}
pub(crate) unsafe extern "C" fn AnimBounceBallShrink(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            InitSpritePosToAnimAttacker(sprite, TRUE);
            gSprites[GetAnimBattlerSpriteId(ANIM_ATTACKER)].set_invisible(TRUE as u16);
            (*sprite).data[0] += 1;
        }
        1 => {
            if (*sprite).affineAnimEnded() != 0 {
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimBounceBallLand(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
            (*sprite).y2 = -(*sprite).y - 32;
            (*sprite).data[0] += 1;
        }
        1 => {
            (*sprite).y2 += 10;
            if (*sprite).y2 >= 0 {
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            (*sprite).y2 -= 10;
            if ((*sprite).y as i32 + (*sprite).y2 as i32) < -32 {
                gSprites[GetAnimBattlerSpriteId(0)].set_invisible(0);
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimDiveBall(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).callback = Some(AnimDiveBall_Step1);
    gSprites[GetAnimBattlerSpriteId(ANIM_ATTACKER)].set_invisible(TRUE as u16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimDiveBall_Step1(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
    } else if (*sprite).y as i32 + (*sprite).y2 as i32 > -32 {
        (*sprite).data[2] += (*sprite).data[1];
        (*sprite).y2 -= (*sprite).data[2] >> 8;
    } else {
        (*sprite).set_invisible(TRUE as u16);
        if ({
            let t1 = (*sprite).data[3];
            (*sprite).data[3] += 1;
            t1
        }) > 20
        {
            (*sprite).callback = Some(AnimDiveBall_Step2);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDiveBall_Step2(sprite: *mut Sprite) {
    (*sprite).y2 += (*sprite).data[2] >> 8;
    if (*sprite).y as i32 + (*sprite).y2 as i32 > -32 {
        (*sprite).set_invisible(FALSE as u16);
    }
    if (*sprite).y2 > 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimDiveWaterSplash(sprite: *mut Sprite) {
    let mut matrixNum: u32 = 0;
    let mut t1: i32 = 0;
    let mut t2: i32 = 0;
    match (*sprite).data[0] {
        0 => {
            if gBattleAnimArgs[0] == 0 {
                (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
                (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
            } else {
                (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
                (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
            }
            (*sprite).data[1] = 0x200;
            TrySetSpriteRotScale(sprite, 0, 0x100, (*sprite).data[1], 0);
            (*sprite).data[0] += 1;
        }
        1 => {
            if (*sprite).data[2] <= 11 {
                (*sprite).data[1] -= 40;
            } else {
                (*sprite).data[1] += 40;
            }
            (*sprite).data[2] += 1;
            TrySetSpriteRotScale(sprite, 0, 0x100, (*sprite).data[1], 0);
            matrixNum = (*sprite).oam.matrixNum();
            t1 = 0x3D00;
            t2 = div_i32(t1, gOamMatrices[matrixNum].d as i32) + 1;
            if t2 > 128 {
                t2 = 128;
            }
            t2 = (64 - t2) / 2;
            (*sprite).y2 = t2 as i16;
            if (*sprite).data[2] == 24 {
                ResetSpriteRotScale_PreserveAffine(sprite);
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimSprayWaterDroplet(sprite: *mut Sprite) {
    let mut v1: i32 = 0x1ff & Random2() as i32;
    let mut v2: i32 = 0x7f & Random2() as i32;
    if v1 % 2 != 0 {
        (*sprite).data[0] = 736 + v1 as i16;
    } else {
        (*sprite).data[0] = 736 - v1 as i16;
    }
    if v2 % 2 != 0 {
        (*sprite).data[1] = 896 + v2 as i16;
    } else {
        (*sprite).data[1] = 896 - v2 as i16;
    }
    (*sprite).data[2] = gBattleAnimArgs[0];
    if (*sprite).data[2] != 0 {
        (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
    }
    if gBattleAnimArgs[1] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + 32;
    } else {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + 32;
    }
    (*sprite).callback = Some(AnimSprayWaterDroplet_Step);
}
pub(crate) unsafe extern "C" fn AnimSprayWaterDroplet_Step(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).x2 += (*sprite).data[0] >> 8;
        (*sprite).y2 -= (*sprite).data[1] >> 8;
    } else {
        (*sprite).x2 -= (*sprite).data[0] >> 8;
        (*sprite).y2 -= (*sprite).data[1] >> 8;
    }
    (*sprite).data[0] = (*sprite).data[0];
    (*sprite).data[1] -= 32;
    if (*sprite).data[0] < 0 {
        (*sprite).data[0] = 0;
    }
    if ({
        (*sprite).data[3] += 1;
        (*sprite).data[3]
    }) == 31
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedFlashingLight(sprite: *mut Sprite) {
    (*sprite).data[6] = 0;
    (*sprite).data[7] = 64;
    (*sprite).callback = Some(AnimUnusedFlashingLight_Step);
}
pub(crate) unsafe extern "C" fn AnimUnusedFlashingLight_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 8
            {
                (*sprite).data[1] = 0;
                (*sprite).set_invisible((*sprite).invisible() ^ 1);
                if ({
                    (*sprite).data[2] += 1;
                    (*sprite).data[2]
                }) > 5
                    && (*sprite).invisible() != 0
                {
                    (*sprite).data[0] += 1;
                }
            }
        }
        1 => {
            DestroyAnimSprite(sprite);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimSkyAttackBird(sprite: *mut Sprite) {
    let mut rotation: u16 = 0;
    let mut posx: i16 = (*sprite).x;
    let mut posy: i16 = (*sprite).y;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[4] = (*sprite).x << 4;
    (*sprite).data[5] = (*sprite).y << 4;
    (*sprite).data[6] = (((posx as i32 - (*sprite).x as i32) << 4) / 12) as i16;
    (*sprite).data[7] = (((posy as i32 - (*sprite).y as i32) << 4) / 12) as i16;
    rotation = ArcTan2Neg(posx - (*sprite).x, posy - (*sprite).y);
    rotation -= 16384;
    TrySetSpriteRotScale(sprite, TRUE, 0x100, 0x100, rotation);
    (*sprite).callback = Some(AnimSkyAttackBird_Step);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimSkyAttackBird_Step(sprite: *mut Sprite) {
    (*sprite).data[4] += (*sprite).data[6];
    (*sprite).data[5] += (*sprite).data[7];
    (*sprite).x = (*sprite).data[4] >> 4;
    (*sprite).y = (*sprite).data[5] >> 4;
    if (*sprite).x > 285 || (*sprite).x < -45 || (*sprite).y > 157 || (*sprite).y < -45 {
        DestroySpriteAndMatrix(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SetAttackerVisibility(taskId: u8) {
    if gBattleAnimArgs[0] == 0 {
        let mut spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
        gSprites[spriteId].set_invisible(TRUE as u16);
    } else {
        let mut spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
        gSprites[spriteId].set_invisible(FALSE as u16);
    }
    DestroyAnimVisualTask(taskId);
}
