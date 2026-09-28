//! Translated from `src/battle_anim_ice.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sAnim_Unused sAnims_Unused sUnusedIceCrystalThrowSpriteTemplate sAnim_IceCrystalLargeChunk sAnim_IceCrystalLarge sAnim_IceCrystalSmall sAnim_Snowball sAnim_BlizzardIceCrystal sAnim_SmallBubblePair sAnims_IceCrystalLargeChunk sAnims_IceCrystalLarge sAnims_IceCrystalSmall sAnims_Snowball sAnims_BlizzardIceCrystal gAnims_SmallBubblePair sAffineAnim_IceCrystalSpiralInwardLarge sAffineAnims_IceCrystalSpiralInwardLarge gIceCrystalSpiralInwardLarge gIceCrystalSpiralInwardSmall sAffineAnim_IceBeamInnerCrystal sAffineAnims_IceBeamInnerCrystal gIceBeamInnerCrystalSpriteTemplate gIceBeamOuterCrystalSpriteTemplate sAffineAnim_IceCrystalHit sAffineAnims_IceCrystalHit gIceCrystalHitLargeSpriteTemplate gIceCrystalHitSmallSpriteTemplate gSwirlingSnowballSpriteTemplate gBlizzardIceCrystalSpriteTemplate gPowderSnowSnowballSpriteTemplate sAnim_IceGroundSpike sAnims_IceGroundSpike gIceGroundSpikeSpriteTemplate sAnim_Cloud sAnims_Cloud gMistCloudSpriteTemplate gSmogCloudSpriteTemplate sHazeBlendAmounts gMistBallSpriteTemplate sMistBlendAmounts gPoisonGasCloudSpriteTemplate sHailCoordData sAffineAnim_HailParticle_0 sAffineAnim_HailParticle_1 sAffineAnim_HailParticle_2 sAffineAnim_WeatherBallIceDown sAffineAnims_HailParticle sAffineAnims_WeatherBallIceDown gHailParticleSpriteTemplate gWeatherBallIceDownSpriteTemplate sAnim_IceBallChunk_0 sAnim_IceBallChunk_1 sAnims_IceBallChunk sAffineAnim_IceBallChunk_0 sAffineAnim_IceBallChunk_1 sAffineAnim_IceBallChunk_2 sAffineAnim_IceBallChunk_3 sAffineAnim_IceBallChunk_4 sAffineAnims_IceBallChunk gIceBallChunkSpriteTemplate gIceBallImpactShardSpriteTemplate

/// `struct HailStruct`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HailStruct {
    bits_0: u32,
}

impl HailStruct {
    #[inline(always)]
    pub fn x(&self) -> i32 {
        ((((self.bits_0 as u32 >> 0) & 0x3ff) << 22) as i32 >> 22) as i32
    }
    #[inline(always)]
    pub fn set_x(&mut self, v: i32) {
        self.bits_0 = (self.bits_0 & !(0x3ff << 0)) | ((v as u32 & 0x3ff) << 0);
    }
    #[inline(always)]
    pub fn y(&self) -> i32 {
        ((((self.bits_0 as u32 >> 10) & 0x3ff) << 22) as i32 >> 22) as i32
    }
    #[inline(always)]
    pub fn set_y(&mut self, v: i32) {
        self.bits_0 = (self.bits_0 & !(0x3ff << 10)) | ((v as u32 & 0x3ff) << 10);
    }
    #[inline(always)]
    pub fn bPosition(&self) -> i32 {
        ((((self.bits_0 as u32 >> 20) & 0xff) << 24) as i32 >> 24) as i32
    }
    #[inline(always)]
    pub fn set_bPosition(&mut self, v: i32) {
        self.bits_0 = (self.bits_0 & !(0xff << 20)) | ((v as u32 & 0xff) << 20);
    }
    #[inline(always)]
    pub fn r#type(&self) -> i32 {
        ((((self.bits_0 as u32 >> 28) & 0xf) << 28) as i32 >> 28) as i32
    }
    #[inline(always)]
    pub fn set_type(&mut self, v: i32) {
        self.bits_0 = (self.bits_0 & !(0xf << 28)) | ((v as u32 & 0xf) << 28);
    }
}

unsafe impl Sync for HailStruct {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<HailStruct>() == 4);
    assert!(offset_of!(HailStruct, bits_0) == 0);
};

const HAILSTRUCTTYPE_FIXED_POSITION: i8 = 2;
const HAILSTRUCTTYPE_NEGATIVE_POS_MOD: i8 = 0;
const HAILSTRUCTTYPE_POSITIVE_POS_MOD: i8 = 1;

static gHailParticleSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_ice::gHailParticleSpriteTemplate).cast());
static gIceCrystalHitLargeSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_ice::gIceCrystalHitLargeSpriteTemplate).cast());
static sAffineAnims_HailParticle: Table<CArray<*mut AffineAnimCmd, 3>> =
    Table((&raw const crate::data::battle_anim_ice::sAffineAnims_HailParticle).cast());
static sHailCoordData: Table<CArray<HailStruct, 10>> =
    Table((&raw const crate::data::battle_anim_ice::sHailCoordData).cast());
static sHazeBlendAmounts: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::battle_anim_ice::sHazeBlendAmounts).cast());
static sMistBlendAmounts: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::battle_anim_ice::sMistBlendAmounts).cast());

unsafe extern "C" {
    static mut gAnimDisableStructPtr: *mut DisableStruct;
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static gBattleAnimFogTilemap: CArray<u32, 0>;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattlerPositions: CArray<u8, 4>;
    static gFogPalette: CArray<u16, 0>;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gWeatherFogHorizontalTiles: CArray<u8, 0>;
    fn AnimFastTranslateLinear(a0: *mut Sprite) -> u8;
    fn AnimLoadCompressedBgTilemapHandleContest(
        a0: *mut BattleAnimBgData,
        a1: *mut c_void,
        a2: u32,
    );
    fn AnimTranslateLinear(a0: *mut Sprite) -> u8;
    fn ClearBattleAnimBg(a0: u32);
    fn ConvertPosDataToTranslateLinearData(a0: *mut Sprite);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn FreeOamMatrix(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut BattleAnimBgData);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn InitAnimArcTranslation(a0: *mut Sprite);
    fn InitAnimFastLinearTranslationWithSpeed(a0: *mut Sprite);
    fn InitAnimFastLinearTranslationWithSpeedAndPos(a0: *mut Sprite);
    fn InitAnimLinearTranslation(a0: *mut Sprite);
    fn InitAnimLinearTranslationWithSpeed(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut Sprite, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn Random2() -> u16;
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut Sprite);
    fn RunStoredCallbackWhenAnimEnds(a0: *mut Sprite);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateAnimHorizontalArc(a0: *mut Sprite) -> u8;
    fn TranslateAnimSpriteToTargetMonLocation(a0: *mut Sprite);
    fn TranslateSpriteInGrowingCircle(a0: *mut Sprite);
}

pub(crate) unsafe extern "C" fn AnimUnusedIceCrystalThrow(sprite: *mut Sprite) {
    let mut targetX: i16 = 0;
    let mut targetY: i16 = 0;
    let mut attackerX: i16 = 0;
    let mut attackerY: i16 = 0;
    (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 7);
    targetX = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    targetY = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    attackerX = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    attackerY = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = gBattleAnimArgs[0] + attackerX;
    (*sprite).data[2] = gBattleAnimArgs[2] + targetX;
    (*sprite).data[3] = gBattleAnimArgs[1] + attackerY;
    (*sprite).data[4] = gBattleAnimArgs[3] + targetY;
    ConvertPosDataToTranslateLinearData(sprite);
    while targetX >= -32 && targetX <= 272 && (targetY >= -32 && targetY <= 192) {
        targetX += (*sprite).data[1];
        targetY += (*sprite).data[2];
    }
    (*sprite).data[1] = -(*sprite).data[1];
    (*sprite).data[2] = -(*sprite).data[2];
    while attackerX >= -32 && attackerX <= 272 && (attackerY >= -32 && attackerY <= 192) {
        attackerX += (*sprite).data[1];
        attackerY += (*sprite).data[2];
    }
    (*sprite).x = attackerX;
    (*sprite).y = attackerY;
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = attackerX;
    (*sprite).data[2] = targetX;
    (*sprite).data[3] = attackerY;
    (*sprite).data[4] = targetY;
    ConvertPosDataToTranslateLinearData(sprite);
    (*sprite).data[3] = gBattleAnimArgs[5];
    (*sprite).data[4] = gBattleAnimArgs[6];
    (*sprite).callback = Some(AnimUnusedIceCrystalThrow_Step);
}
pub(crate) unsafe extern "C" fn AnimUnusedIceCrystalThrow_Step(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).data[5] += (*sprite).data[1];
        (*sprite).data[6] += (*sprite).data[2];
        (*sprite).x2 = (*sprite).data[5];
        (*sprite).y2 = (*sprite).data[6];
        (*sprite).x2 += Sin((*sprite).data[7], (*sprite).data[3]);
        (*sprite).y2 += Sin((*sprite).data[7], (*sprite).data[3]);
        (*sprite).data[7] = (*sprite).data[7] + (*sprite).data[4] & 0xFF;
        (*sprite).data[0] -= 1;
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimIcePunchSwirlingParticle(sprite: *mut Sprite) {
    (*sprite).data[0] = gBattleAnimArgs[0];
    (*sprite).data[1] = 60;
    (*sprite).data[2] = 9;
    (*sprite).data[3] = 30;
    (*sprite).data[4] = -512;
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(TranslateSpriteInGrowingCircle);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimIceBeamParticle(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).data[2] -= gBattleAnimArgs[2];
    } else {
        (*sprite).data[2] += gBattleAnimArgs[2];
    }
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + gBattleAnimArgs[3];
    (*sprite).data[0] = gBattleAnimArgs[4];
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(StartAnimLinearTranslation);
}
pub(crate) unsafe extern "C" fn AnimIceEffectParticle(sprite: *mut Sprite) {
    if gBattleAnimArgs[2] == 0 {
        InitSpritePosToAnimTarget(sprite, TRUE);
    } else {
        SetAverageBattlerPositions(
            gBattleAnimTarget,
            TRUE,
            &raw mut (*sprite).x,
            &raw mut (*sprite).y,
        );
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            gBattleAnimArgs[0] = -gBattleAnimArgs[0];
        }
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
    }
    StoreSpriteCallbackInData6(sprite, Some(AnimFlickerIceEffectParticle));
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
}
pub(crate) unsafe extern "C" fn AnimFlickerIceEffectParticle(sprite: *mut Sprite) {
    (*sprite).set_invisible((*sprite).invisible() ^ 1);
    (*sprite).data[0] += 1;
    if (*sprite).data[0] == 20 {
        DestroySpriteAndMatrix(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimSwirlingSnowball(sprite: *mut Sprite) {
    let mut i: i32 = 0;
    let mut tempDataHolder: CArray<i16, 8> = zeroed();
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    if gBattleAnimArgs[5] == 0 {
        (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET)
            as i16
            + gBattleAnimArgs[3];
    } else {
        SetAverageBattlerPositions(
            gBattleAnimTarget,
            TRUE,
            &raw mut (*sprite).data[2],
            &raw mut (*sprite).data[4],
        );
    }
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).data[2] -= gBattleAnimArgs[2];
    } else {
        (*sprite).data[2] += gBattleAnimArgs[2];
    }
    i = 0;
    while i < 8 {
        tempDataHolder[i] = (*sprite).data[i];
        i += 1;
    }
    InitAnimFastLinearTranslationWithSpeed(sprite);
    (*sprite).data[1] ^= 1;
    (*sprite).data[2] ^= 1;
    loop {
        (*sprite).data[0] = 1;
        AnimFastTranslateLinear(sprite);
        if (*sprite).x as i32 + (*sprite).x2 as i32 > 256
            || ((*sprite).x as i32 + (*sprite).x2 as i32) < -16
            || (*sprite).y as i32 + (*sprite).y2 as i32 > DISPLAY_HEIGHT as i32
            || ((*sprite).y as i32 + (*sprite).y2 as i32) < -16
        {
            break;
        }
    }
    (*sprite).x += (*sprite).x2;
    (*sprite).y += (*sprite).y2;
    (*sprite).y2 = 0;
    (*sprite).x2 = 0;
    i = 0;
    while i < 8 {
        (*sprite).data[i] = tempDataHolder[i];
        i += 1;
    }
    (*sprite).callback = Some(InitAnimFastLinearTranslationWithSpeedAndPos);
    StoreSpriteCallbackInData6(sprite, Some(AnimSwirlingSnowball_Step1));
}
pub(crate) unsafe extern "C" fn AnimSwirlingSnowball_Step1(sprite: *mut Sprite) {
    let mut tempVar: i16 = 0;
    (*sprite).x += (*sprite).x2;
    (*sprite).y += (*sprite).y2;
    (*sprite).y2 = 0;
    (*sprite).x2 = 0;
    (*sprite).data[0] = 128;
    tempVar = (if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        20
    } else {
        -20
    }) as i16;
    (*sprite).data[3] = Sin((*sprite).data[0], tempVar);
    (*sprite).data[4] = Cos((*sprite).data[0], 0xF);
    (*sprite).data[5] = 0;
    (*sprite).callback = Some(AnimSwirlingSnowball_Step2);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimSwirlingSnowball_Step2(sprite: *mut Sprite) {
    let mut tempVar: i16 = 0;
    tempVar = (if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        20
    } else {
        -20
    }) as i16;
    if (*sprite).data[5] <= 31 {
        (*sprite).x2 = Sin((*sprite).data[0], tempVar) - (*sprite).data[3];
        (*sprite).y2 = Cos((*sprite).data[0], 15) - (*sprite).data[4];
        (*sprite).data[0] = (*sprite).data[0] + 16 & 0xFF;
        (*sprite).data[5] += 1;
    } else {
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        (*sprite).data[4] = 0;
        (*sprite).data[3] = 0;
        (*sprite).callback = Some(AnimSwirlingSnowball_End);
    }
}
pub(crate) unsafe extern "C" fn AnimSwirlingSnowball_End(sprite: *mut Sprite) {
    (*sprite).data[0] = 1;
    AnimFastTranslateLinear(sprite);
    if (*sprite).x as i32 + (*sprite).x2 as i32 > 256
        || ((*sprite).x as i32 + (*sprite).x2 as i32) < -16
        || (*sprite).y as i32 + (*sprite).y2 as i32 > 256
        || ((*sprite).y as i32 + (*sprite).y2 as i32) < -16
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimMoveParticleBeyondTarget(sprite: *mut Sprite) {
    let mut i: i32 = 0;
    let mut tempDataHolder: CArray<i16, 8> = zeroed();
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    if gBattleAnimArgs[7] == 0 {
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
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).data[2] -= gBattleAnimArgs[2];
    } else {
        (*sprite).data[2] += gBattleAnimArgs[2];
    }
    (*sprite).data[4] += gBattleAnimArgs[3];
    InitAnimFastLinearTranslationWithSpeed(sprite);
    i = 0;
    while i < 8 {
        tempDataHolder[i] = (*sprite).data[i];
        i += 1;
    }
    (*sprite).data[1] ^= 1;
    (*sprite).data[2] ^= 1;
    loop {
        (*sprite).data[0] = 1;
        AnimFastTranslateLinear(sprite);
        if (*sprite).x as i32 + (*sprite).x2 as i32 > 256
            || ((*sprite).x as i32 + (*sprite).x2 as i32) < -16
            || (*sprite).y as i32 + (*sprite).y2 as i32 > DISPLAY_HEIGHT as i32
            || ((*sprite).y as i32 + (*sprite).y2 as i32) < -16
        {
            break;
        }
    }
    (*sprite).x += (*sprite).x2;
    (*sprite).y += (*sprite).y2;
    (*sprite).y2 = 0;
    (*sprite).x2 = 0;
    i = 0;
    while i < 8 {
        (*sprite).data[i] = tempDataHolder[i];
        i += 1;
    }
    (*sprite).data[5] = gBattleAnimArgs[5];
    (*sprite).data[6] = gBattleAnimArgs[6];
    (*sprite).callback = Some(AnimWiggleParticleTowardsTarget);
}
pub(crate) unsafe extern "C" fn AnimWiggleParticleTowardsTarget(sprite: *mut Sprite) {
    AnimFastTranslateLinear(sprite);
    if (*sprite).data[0] == 0 {
        (*sprite).data[0] = 1;
    }
    (*sprite).y2 += Sin((*sprite).data[7], (*sprite).data[5]);
    (*sprite).data[7] = (*sprite).data[7] + (*sprite).data[6] & 0xFF;
    if (*sprite).data[0] == 1 {
        if (*sprite).x as i32 + (*sprite).x2 as i32 > 256
            || ((*sprite).x as i32 + (*sprite).x2 as i32) < -16
            || (*sprite).y as i32 + (*sprite).y2 as i32 > DISPLAY_HEIGHT as i32
            || ((*sprite).y as i32 + (*sprite).y2 as i32) < -16
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaveFromCenterOfTarget(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        if gBattleAnimArgs[2] == 0 {
            InitSpritePosToAnimTarget(sprite, FALSE);
        } else {
            SetAverageBattlerPositions(
                gBattleAnimTarget,
                FALSE,
                &raw mut (*sprite).x,
                &raw mut (*sprite).y,
            );
            if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
                gBattleAnimArgs[0] = -gBattleAnimArgs[0];
            }
            (*sprite).x += gBattleAnimArgs[0];
            (*sprite).y += gBattleAnimArgs[1];
        }
        (*sprite).data[0] += 1;
    } else {
        if (*sprite).animEnded() != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn InitSwirlingFogAnim(sprite: *mut Sprite) {
    let mut tempVar: i16 = 0;
    let mut battler: u8 = 0;
    if gBattleAnimArgs[4] == 0 {
        if gBattleAnimArgs[5] == 0 {
            InitSpritePosToAnimAttacker(sprite, FALSE);
        } else {
            SetAverageBattlerPositions(
                gBattleAnimAttacker,
                FALSE,
                &raw mut (*sprite).x,
                &raw mut (*sprite).y,
            );
            if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
                (*sprite).x -= gBattleAnimArgs[0];
            } else {
                (*sprite).x += gBattleAnimArgs[0];
            }
            (*sprite).y += gBattleAnimArgs[1];
        }
        battler = gBattleAnimAttacker;
    } else {
        if gBattleAnimArgs[5] == 0 {
            InitSpritePosToAnimTarget(sprite, FALSE);
        } else {
            SetAverageBattlerPositions(
                gBattleAnimTarget,
                FALSE,
                &raw mut (*sprite).x,
                &raw mut (*sprite).y,
            );
            if GetBattlerSide(gBattleAnimTarget) != B_SIDE_PLAYER {
                (*sprite).x -= gBattleAnimArgs[0];
            } else {
                (*sprite).x += gBattleAnimArgs[0];
            }
            (*sprite).y += gBattleAnimArgs[1];
        }
        battler = gBattleAnimTarget;
    }
    (*sprite).data[7] = battler as i16;
    if gBattleAnimArgs[5] == 0 || IsDoubleBattle() == 0 {
        tempVar = 0x20;
    } else {
        tempVar = 0x40;
    }
    (*sprite).data[6] = tempVar;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*sprite).y += 8;
    }
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = (*sprite).y + gBattleAnimArgs[2];
    InitAnimLinearTranslation(sprite);
    (*sprite).data[5] = 64;
    (*sprite).callback = Some(AnimSwirlingFogAnim);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimSwirlingFogAnim(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) == 0 {
        (*sprite).x2 += Sin((*sprite).data[5], (*sprite).data[6]);
        (*sprite).y2 += Cos((*sprite).data[5], -6);
        if (*sprite).data[5] as u16 as i32 - 64 <= 0x7F {
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority((*sprite).data[7] as u8) as u16);
        } else {
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority((*sprite).data[7] as u8) as u16 + 1);
        }
        (*sprite).data[5] = (*sprite).data[5] + 3 & 0xFF;
    } else {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_HazeScrollingFog(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
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
    LoadBgTiles(
        animBg.bgId,
        gWeatherFogHorizontalTiles.as_ptr().cast_mut() as *mut c_void,
        0x800,
        animBg.tilesOffset,
    );
    AnimLoadCompressedBgTilemapHandleContest(
        &raw mut animBg,
        gBattleAnimFogTilemap.as_ptr().cast_mut() as *mut c_void,
        FALSE as u32,
    );
    LoadPalette(
        (&raw const gFogPalette).cast_mut() as *mut c_void,
        0x000 + animBg.paletteId as u16 * 16,
        32,
    );
    gTasks[taskId].func = Some(AnimTask_HazeScrollingFog_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_HazeScrollingFog_Step(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    gBattle_BG1_X += 65535;
    gBattle_BG1_Y += 0;
    'l1: {
        let sw1: i16 = gTasks[taskId].data[12];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if ({
                gTasks[taskId].data[10] += 1;
                gTasks[taskId].data[10]
            }) == 4
            {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[9] += 1;
                gTasks[taskId].data[11] = sHazeBlendAmounts[gTasks[taskId].data[9]] as i16;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - gTasks[taskId].data[11] as u16) << 8 | gTasks[taskId].data[11] as u16,
                );
                if gTasks[taskId].data[11] == 9 {
                    gTasks[taskId].data[12] += 1;
                    gTasks[taskId].data[11] = 0;
                }
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if ({
                gTasks[taskId].data[11] += 1;
                gTasks[taskId].data[11]
            }) == 0x51
            {
                gTasks[taskId].data[11] = 9;
                gTasks[taskId].data[12] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
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
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            GetBattleAnimBg1Data(&raw mut animBg);
            ClearBattleAnimBg(1);
            ClearBattleAnimBg(2);
            gTasks[taskId].data[12] += 1;
        }
        if fall || sw1 == 4 {
            fall = true;
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            DestroyAnimVisualTask(taskId);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn AnimThrowMistBall(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).callback = Some(TranslateAnimSpriteToTargetMonLocation);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MistBallFog(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
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
    LoadBgTiles(
        animBg.bgId,
        gWeatherFogHorizontalTiles.as_ptr().cast_mut() as *mut c_void,
        0x800,
        animBg.tilesOffset,
    );
    AnimLoadCompressedBgTilemapHandleContest(
        &raw mut animBg,
        gBattleAnimFogTilemap.as_ptr().cast_mut() as *mut c_void,
        FALSE as u32,
    );
    LoadPalette(
        (&raw const gFogPalette).cast_mut() as *mut c_void,
        0x000 + animBg.paletteId as u16 * 16,
        32,
    );
    gTasks[taskId].data[15] = -1;
    gTasks[taskId].func = Some(AnimTask_MistBallFog_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_MistBallFog_Step(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    gBattle_BG1_X += gTasks[taskId].data[15] as u16;
    gBattle_BG1_Y += 0;
    'l1: {
        let sw1: i16 = gTasks[taskId].data[12];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            gTasks[taskId].data[9] += 1;
            gTasks[taskId].data[11] = sMistBlendAmounts[gTasks[taskId].data[9]] as i16;
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                (17 - gTasks[taskId].data[11] as u16) << 8 | gTasks[taskId].data[11] as u16,
            );
            if gTasks[taskId].data[11] == 5 {
                gTasks[taskId].data[12] += 1;
                gTasks[taskId].data[11] = 0;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if ({
                gTasks[taskId].data[11] += 1;
                gTasks[taskId].data[11]
            }) == 0x51
            {
                gTasks[taskId].data[11] = 5;
                gTasks[taskId].data[12] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
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
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            GetBattleAnimBg1Data(&raw mut animBg);
            ClearBattleAnimBg(1);
            ClearBattleAnimBg(2);
            gTasks[taskId].data[12] += 1;
        }
        if fall || sw1 == 4 {
            fall = true;
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            DestroyAnimVisualTask(taskId);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn InitPoisonGasCloudAnim(sprite: *mut Sprite) {
    (*sprite).data[0] = gBattleAnimArgs[0];
    if GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2)
        < GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2)
    {
        (*sprite).data[7] = -32768;
    }
    if gBattlerPositions[gBattleAnimTarget] as i32 & 1 == B_SIDE_PLAYER as i32 {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        gBattleAnimArgs[3] = -gBattleAnimArgs[3];
        if (*sprite).data[7] as i32 & 0x8000 != 0
            && gBattlerPositions[gBattleAnimAttacker] as i32 & 1 == B_SIDE_PLAYER as i32
        {
            (*sprite).subpriority = gSprites[GetAnimBattlerSpriteId(1)].subpriority + 1;
        }
        (*sprite).data[6] = 1;
    }
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    if gBattleAnimArgs[7] != 0 {
        (*sprite).data[1] = (*sprite).x + gBattleAnimArgs[1];
        (*sprite).data[2] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[3];
        (*sprite).data[3] = (*sprite).y + gBattleAnimArgs[2];
        (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET)
            as i16
            + gBattleAnimArgs[4];
        (*sprite).data[7] |= (GetBattlerSpriteBGPriority(gBattleAnimTarget) as i16) << 8;
    } else {
        (*sprite).data[1] = (*sprite).x + gBattleAnimArgs[1];
        (*sprite).data[2] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16 + gBattleAnimArgs[3];
        (*sprite).data[3] = (*sprite).y + gBattleAnimArgs[2];
        (*sprite).data[4] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + gBattleAnimArgs[4];
        (*sprite).data[7] |= (GetBattlerSpriteBGPriority(gBattleAnimTarget) as i16) << 8;
    }
    if IsContest() != 0 {
        (*sprite).data[6] = 1;
        (*sprite).subpriority = 0x80;
    }
    InitAnimLinearTranslation(sprite);
    (*sprite).callback = Some(MovePoisonGasCloud);
}
pub(crate) unsafe extern "C" fn MovePoisonGasCloud(sprite: *mut Sprite) {
    let mut value: i32 = 0;
    match (*sprite).data[7] as i32 & 0xFF {
        0 => {
            AnimTranslateLinear(sprite);
            value = gSineTable[(*sprite).data[5]] as i32;
            (*sprite).x2 += (value >> 4) as i16;
            if (*sprite).data[6] != 0 {
                (*sprite).data[5] = (*sprite).data[5] - 8 & 0xFF;
            } else {
                (*sprite).data[5] = (*sprite).data[5] + 8 & 0xFF;
            }
            if (*sprite).data[0] <= 0 {
                (*sprite).data[0] = 80;
                (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
                (*sprite).data[1] = (*sprite).x;
                (*sprite).data[2] = (*sprite).x;
                (*sprite).y += (*sprite).y2;
                (*sprite).data[3] = (*sprite).y;
                (*sprite).data[4] = (*sprite).y + 29;
                (*sprite).data[7] += 1;
                if IsContest() != 0 {
                    (*sprite).data[5] = 80;
                } else if gBattlerPositions[gBattleAnimTarget] as i32 & 1 != B_SIDE_PLAYER as i32 {
                    (*sprite).data[5] = 204;
                } else {
                    (*sprite).data[5] = 80;
                }
                (*sprite).y2 = 0;
                value = gSineTable[(*sprite).data[5]] as i32;
                (*sprite).x2 = (value >> 3) as i16;
                (*sprite).data[5] = (*sprite).data[5] + 2 & 0xFF;
                InitAnimLinearTranslation(sprite);
            }
        }
        1 => {
            AnimTranslateLinear(sprite);
            value = gSineTable[(*sprite).data[5]] as i32;
            (*sprite).x2 += (value >> 3) as i16;
            (*sprite).y2 += (gSineTable[(*sprite).data[5] as i32 + 0x40] as i32 * -3 >> 8) as i16;
            if IsContest() == 0 {
                let mut var0: u16 = (*sprite).data[5] as u16 - 0x40;
                if var0 <= 0x7F {
                    (*sprite).oam.set_priority(((*sprite).data[7] >> 8) as u16);
                } else {
                    (*sprite)
                        .oam
                        .set_priority(((*sprite).data[7] >> 8) as u16 + 1);
                }
                (*sprite).data[5] = (*sprite).data[5] + 4 & 0xFF;
            } else {
                let mut var0: u16 = (*sprite).data[5] as u16 - 0x40;
                if var0 <= 0x7F {
                    (*sprite).subpriority = 128;
                } else {
                    (*sprite).subpriority = 140;
                }
                (*sprite).data[5] = (*sprite).data[5] - 4 & 0xFF;
            }
            if (*sprite).data[0] <= 0 {
                (*sprite).data[0] = 0x300;
                (*sprite).data[1] = {
                    (*sprite).x += (*sprite).x2;
                    (*sprite).x
                };
                (*sprite).data[3] = {
                    (*sprite).y += (*sprite).y2;
                    (*sprite).y
                };
                (*sprite).data[4] = (*sprite).y + 4;
                if IsContest() != 0 {
                    (*sprite).data[2] = -16;
                } else if gBattlerPositions[gBattleAnimTarget] as i32 & 1 != B_SIDE_PLAYER as i32 {
                    (*sprite).data[2] = 256;
                } else {
                    (*sprite).data[2] = -16;
                }
                (*sprite).data[7] += 1;
                (*sprite).x2 = {
                    (*sprite).y2 = 0;
                    (*sprite).y2
                };
                InitAnimLinearTranslationWithSpeed(sprite);
            }
        }
        2 => {
            if AnimTranslateLinear(sprite) != 0 {
                if (*sprite).oam.affineMode() & 1 != 0 {
                    FreeOamMatrix((*sprite).oam.matrixNum() as u8);
                    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
                }
                DestroySprite(sprite);
                gAnimVisualTaskCount -= 1;
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Hail(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).func = Some(AnimTask_Hail2);
}
pub(crate) unsafe extern "C" fn AnimTask_Hail2(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[4] += 1;
                (*task).data[4]
            }) > 2
            {
                (*task).data[4] = 0;
                (*task).data[5] = 0;
                (*task).data[2] = 0;
                (*task).data[0] += 1;
            }
        }
        1 => {
            if (*task).data[5] == 0 {
                if GenerateHailParticle((*task).data[3] as u8, (*task).data[2] as u8, taskId, 1)
                    != 0
                {
                    (*task).data[1] += 1;
                }
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) == 3
                {
                    if ({
                        (*task).data[3] += 1;
                        (*task).data[3]
                    }) == 10
                    {
                        (*task).data[0] += 1;
                    } else {
                        (*task).data[0] -= 1;
                    }
                } else {
                    (*task).data[5] = 1;
                }
            } else {
                (*task).data[5] -= 1;
            }
        }
        2 => {
            if (*task).data[1] == 0 {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn GenerateHailParticle(
    hailStructId: u8,
    affineAnimNum: u8,
    taskId: u8,
    c: u8,
) -> u8 {
    let mut id: u8 = 0;
    let mut battlerX: i16 = 0;
    let mut battlerY: i16 = 0;
    let mut spriteX: i16 = 0;
    let mut shouldSpawnImpactEffect: u8 = FALSE;
    let mut r#type: i8 = sHailCoordData[hailStructId].r#type() as i8;
    if r#type != HAILSTRUCTTYPE_FIXED_POSITION {
        id = GetBattlerAtPosition(sHailCoordData[hailStructId].bPosition() as u8);
        if IsBattlerSpriteVisible(id) != 0 {
            shouldSpawnImpactEffect = TRUE;
            battlerX = GetBattlerSpriteCoord(id, BATTLER_COORD_X_2) as i16;
            battlerY = GetBattlerSpriteCoord(id, BATTLER_COORD_Y_PIC_OFFSET) as i16;
            match r#type {
                HAILSTRUCTTYPE_NEGATIVE_POS_MOD => {
                    battlerX -= GetBattlerSpriteCoordAttr(id, BATTLER_COORD_ATTR_WIDTH) / 6;
                    battlerY -= GetBattlerSpriteCoordAttr(id, BATTLER_COORD_ATTR_HEIGHT) / 6;
                }
                HAILSTRUCTTYPE_POSITIVE_POS_MOD => {
                    battlerX += GetBattlerSpriteCoordAttr(id, BATTLER_COORD_ATTR_WIDTH) / 6;
                    battlerY += GetBattlerSpriteCoordAttr(id, BATTLER_COORD_ATTR_HEIGHT) / 6;
                }
                _ => {}
            }
        } else {
            battlerX = sHailCoordData[hailStructId].x() as i16;
            battlerY = sHailCoordData[hailStructId].y() as i16;
        }
    } else {
        battlerX = sHailCoordData[hailStructId].x() as i16;
        battlerY = sHailCoordData[hailStructId].y() as i16;
    }
    spriteX = battlerX - ((battlerY as i32 + 8) / 2) as i16;
    id = CreateSprite(
        (&raw const *gHailParticleSpriteTemplate).cast_mut(),
        spriteX,
        -8,
        18,
    );
    if id == MAX_SPRITES {
        return FALSE;
    } else {
        StartSpriteAffineAnim(&raw mut gSprites[id], affineAnimNum);
        gSprites[id].data[0] = shouldSpawnImpactEffect as i16;
        gSprites[id].data[3] = battlerX;
        gSprites[id].data[4] = battlerY;
        gSprites[id].data[5] = affineAnimNum as i16;
        gSprites[id].data[6] = taskId as i16;
        gSprites[id].data[7] = c as i16;
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AnimHailBegin(sprite: *mut Sprite) {
    let mut spriteId: u8 = 0;
    (*sprite).x += 4;
    (*sprite).y += 8;
    if (*sprite).x < (*sprite).data[3] && (*sprite).y < (*sprite).data[4] {
        return;
    }
    if (*sprite).data[0] == 1 && (*sprite).data[5] == 0 {
        spriteId = CreateSprite(
            (&raw const *gIceCrystalHitLargeSpriteTemplate).cast_mut(),
            (*sprite).data[3],
            (*sprite).data[4],
            (*sprite).subpriority,
        );
        (*sprite).data[0] = spriteId as i16;
        if spriteId != MAX_SPRITES {
            gSprites[(*sprite).data[0]].callback = Some(AnimHailContinue);
            gSprites[(*sprite).data[0]].data[6] = (*sprite).data[6];
            gSprites[(*sprite).data[0]].data[7] = (*sprite).data[7];
        }
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        DestroySprite(sprite);
    } else {
        gTasks[(*sprite).data[6]].data[(*sprite).data[7]] -= 1;
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimHailContinue(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 20
    {
        gTasks[(*sprite).data[6]].data[(*sprite).data[7]] -= 1;
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn InitIceBallAnim(sprite: *mut Sprite) {
    let mut animNum: u8 = (*gAnimDisableStructPtr).rolloutTimerStartValue()
        - (*gAnimDisableStructPtr).rolloutTimer()
        - 1;
    if animNum > 4 {
        animNum = 4;
    }
    StartSpriteAffineAnim(sprite, animNum);
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[4];
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[2];
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + gBattleAnimArgs[3];
    (*sprite).data[5] = gBattleAnimArgs[5];
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimThrowIceBall);
}
pub(crate) unsafe extern "C" fn AnimThrowIceBall(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) == 0 {
        return;
    }
    StartSpriteAnim(sprite, 1);
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn InitIceBallParticle(sprite: *mut Sprite) {
    let mut randA: i16 = 0;
    let mut randB: i16 = 0;
    (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 8);
    InitSpritePosToAnimTarget(sprite, TRUE);
    randA = (Random2() as i16 & 0xFF) + 256;
    randB = Random2() as i16 & 0x1FF;
    if randB > 0xFF {
        randB = 256 - randB;
    }
    (*sprite).data[1] = randA;
    (*sprite).data[2] = randB;
    (*sprite).callback = Some(AnimIceBallParticle);
}
pub(crate) unsafe extern "C" fn AnimIceBallParticle(sprite: *mut Sprite) {
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).data[4] += (*sprite).data[2];
    if (*sprite).data[1] as i32 & 1 != 0 {
        (*sprite).x2 = -((*sprite).data[3] >> 8);
    } else {
        (*sprite).x2 = (*sprite).data[3] >> 8;
    }
    (*sprite).y2 = (*sprite).data[4] >> 8;
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 21
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetIceBallCounter(taskId: u8) {
    let mut arg: u8 = gBattleAnimArgs[0] as u8;
    gBattleAnimArgs[arg] = (*gAnimDisableStructPtr).rolloutTimerStartValue() as i16
        - (*gAnimDisableStructPtr).rolloutTimer() as i16
        - 1;
    DestroyAnimVisualTask(taskId);
}
