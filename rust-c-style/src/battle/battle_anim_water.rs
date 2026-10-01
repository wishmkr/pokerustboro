//! Translated from `src/battle_anim_water.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sUnusedWater_Gfx sUnusedWater sAnim_RainDrop sAnims_RainDrop gRainDropSpriteTemplate sAffineAnim_WaterBubbleProjectile sAffineAnims_WaterBubbleProjectile sAnim_WaterBubbleProjectile sAnims_WaterBubbleProjectile gWaterBubbleProjectileSpriteTemplate sAnim_AuroraBeamRing_0 sAnim_AuroraBeamRing_1 sAnims_AuroraBeamRing sAffineAnim_AuroraBeamRing sAffineAnims_AuroraBeamRing gAuroraBeamRingSpriteTemplate sAnim_WaterMudOrb gAnims_WaterMudOrb gHydroPumpOrbSpriteTemplate gMudShotOrbSpriteTemplate gSignalBeamRedOrbSpriteTemplate gSignalBeamGreenOrbSpriteTemplate sAnim_FlamethrowerFlame sAnims_FlamethrowerFlame gFlamethrowerFlameSpriteTemplate gPsywaveRingSpriteTemplate sAffineAnim_HydroCannonCharge sAffineAnim_HydroCannonBeam sAffineAnims_HydroCannonCharge sAffineAnims_HydroCannonBeam gHydroCannonChargeSpriteTemplate gHydroCannonBeamSpriteTemplate sAnim_WaterBubble sAnim_WaterGunDroplet gAnims_WaterBubble sAnims_WaterGunDroplet gWaterGunProjectileSpriteTemplate gWaterGunDropletSpriteTemplate gSmallBubblePairSpriteTemplate gSmallDriftingBubblesSpriteTemplate gSmallWaterOrbSpriteTemplate sAnim_WaterPulseBubble_0 sAnim_WaterPulseBubble_1 sAnim_WeatherBallWaterDown sAnims_WaterPulseBubble sAnims_WeatherBallWaterDown sAffineAnim_WaterPulseRingBubble_0 sAffineAnim_WaterPulseRingBubble_1 sAffineAnim_WeatherBallWaterDown sAffineAnims_WaterPulseRingBubble sAffineAnims_WeatherBallWaterDown gWaterPulseBubbleSpriteTemplate gWaterPulseRingBubbleSpriteTemplate gWeatherBallWaterDownSpriteTemplate

/// `__anon1`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon1 {
    pub palette: i16,
}

unsafe impl Sync for Anon1 {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Anon1>() == 2);
    assert!(offset_of!(Anon1, palette) == 0);
};

static gRainDropSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_water::gRainDropSpriteTemplate).cast());
static gSmallWaterOrbSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_water::gSmallWaterOrbSpriteTemplate).cast());
static gWaterPulseRingBubbleSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_water::gWaterPulseRingBubbleSpriteTemplate).cast());

unsafe extern "C" {
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static gBattleAnimBackgroundImageMuddyWater_Pal: CArray<u32, 0>;
    static gBattleAnimBgImage_Surf: CArray<u32, 0>;
    static gBattleAnimBgPalette_Surf: CArray<u32, 0>;
    static gBattleAnimBgTilemap_SurfContest: CArray<u32, 0>;
    static gBattleAnimBgTilemap_SurfOpponent: CArray<u32, 0>;
    static gBattleAnimBgTilemap_SurfPlayer: CArray<u32, 0>;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gScanlineEffect: ScanlineEffect;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gWaterHitSplatSpriteTemplate: SpriteTemplate;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemap(a0: u32, a1: *mut c_void);
    fn AnimLoadCompressedBgTilemapHandleContest(
        a0: *mut BattleAnimBgData,
        a1: *mut c_void,
        a2: u32,
    );
    fn AnimTask_HorizontalShake(a0: u8);
    fn AnimTranslateLinear(a0: *mut Sprite) -> u8;
    fn ClearBattleAnimBg(a0: u32);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut Sprite)>) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut BattleAnimBgData);
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAnimArcTranslation(a0: *mut Sprite);
    fn InitAnimLinearTranslation(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut Sprite, a1: u8);
    fn IsContest() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn PrepareBattlerSpriteForRotScale(a0: u8, a1: u8);
    fn PrepareEruptAnimTaskData(a0: *mut Task, a1: u8, a2: i16, a3: i16, a4: i16, a5: i16, a6: u16);
    fn Random2() -> u16;
    fn ResetSpriteRotScale(a0: u8);
    fn RunStoredCallbackWhenAnimEnds(a0: *mut Sprite);
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn ScanlineEffect_Stop();
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBattlerSpriteYOffsetFromYScale(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateAnimHorizontalArc(a0: *mut Sprite) -> u8;
    fn UpdateEruptAnimTask(a0: *mut Task) -> u8;
    fn WaitAnimForDuration(a0: *mut Sprite);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CreateRaindrops(taskId: u8) {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    if gTasks[taskId].data[0] == 0 {
        gTasks[taskId].data[1] = gBattleAnimArgs[0];
        gTasks[taskId].data[2] = gBattleAnimArgs[1];
        gTasks[taskId].data[3] = gBattleAnimArgs[2];
    }
    gTasks[taskId].data[0] += 1;
    if rem_i32(gTasks[taskId].data[0] as i32, gTasks[taskId].data[2] as i32) == 1 {
        x = (Random2() as i32 % 240) as u8;
        y = (Random2() as i32 % 80) as u8;
        CreateSprite(
            (&raw const *gRainDropSpriteTemplate).cast_mut(),
            x as i16,
            y as i16,
            4,
        );
    }
    if gTasks[taskId].data[0] == gTasks[taskId].data[3] {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimRainDrop(sprite: *mut Sprite) {
    (*sprite).callback = Some(AnimRainDrop_Step);
}
pub(crate) unsafe extern "C" fn AnimRainDrop_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) <= 13
    {
        (*sprite).x2 += 1;
        (*sprite).y2 += 4;
    }
    if (*sprite).animEnded() != 0 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimWaterBubbleProjectile(sprite: *mut Sprite) {
    let mut spriteId: u8 = 0;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16
            - gBattleAnimArgs[0];
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16
            + gBattleAnimArgs[1];
        (*sprite).set_animPaused(TRUE);
    } else {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16
            + gBattleAnimArgs[0];
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16
            + gBattleAnimArgs[1];
        (*sprite).set_animPaused(TRUE);
    }
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[6];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    InitAnimLinearTranslation(sprite);
    spriteId = CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy));
    (*sprite).data[5] = spriteId as i16;
    (*sprite).x -= Sin(gBattleAnimArgs[4] as u8 as i16, gBattleAnimArgs[2]);
    (*sprite).y -= Cos(gBattleAnimArgs[4] as u8 as i16, gBattleAnimArgs[3]);
    gSprites[spriteId].data[0] = gBattleAnimArgs[2];
    gSprites[spriteId].data[1] = gBattleAnimArgs[3];
    gSprites[spriteId].data[2] = gBattleAnimArgs[5];
    gSprites[spriteId].data[3] = gBattleAnimArgs[4] as u8 as i16 * 256;
    gSprites[spriteId].data[4] = gBattleAnimArgs[6];
    (*sprite).callback = Some(AnimWaterBubbleProjectile_Step1);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimWaterBubbleProjectile_Step1(sprite: *mut Sprite) {
    let mut otherSpriteId: u8 = (*sprite).data[5] as u8;
    let mut timer: u8 = gSprites[otherSpriteId].data[4] as u8;
    let mut trigIndex: u16 = gSprites[otherSpriteId].data[3] as u16;
    (*sprite).data[0] = 1;
    AnimTranslateLinear(sprite);
    (*sprite).x2 += Sin((trigIndex >> 8) as i16, gSprites[otherSpriteId].data[0]);
    (*sprite).y2 += Cos((trigIndex >> 8) as i16, gSprites[otherSpriteId].data[1]);
    gSprites[otherSpriteId].data[3] = trigIndex as i16 + gSprites[otherSpriteId].data[2];
    if ({
        timer -= 1;
        timer
    }) != 0
    {
        gSprites[otherSpriteId].data[4] = timer as i16;
    } else {
        (*sprite).callback = Some(AnimWaterBubbleProjectile_Step2);
        DestroySprite(&raw mut gSprites[otherSpriteId]);
    }
}
pub(crate) unsafe extern "C" fn AnimWaterBubbleProjectile_Step2(sprite: *mut Sprite) {
    (*sprite).set_animPaused(FALSE);
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(AnimWaterBubbleProjectile_Step3));
}
pub(crate) unsafe extern "C" fn AnimWaterBubbleProjectile_Step3(sprite: *mut Sprite) {
    (*sprite).data[0] = 10;
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
}
pub(crate) unsafe extern "C" fn AnimAuroraBeamRings(sprite: *mut Sprite) {
    let mut unkArg: i16 = 0;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        unkArg = -gBattleAnimArgs[2];
    } else {
        unkArg = gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + unkArg;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + gBattleAnimArgs[3];
    InitAnimLinearTranslation(sprite);
    (*sprite).callback = Some(AnimAuroraBeamRings_Step);
    (*sprite).set_affineAnimPaused(TRUE);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimAuroraBeamRings_Step(sprite: *mut Sprite) {
    if gBattleAnimArgs[7] as u16 == 0xFFFF {
        StartSpriteAnim(sprite, 1);
        (*sprite).set_affineAnimPaused(FALSE);
    }
    if AnimTranslateLinear(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_RotateAuroraRingColors(taskId: u8) {
    gTasks[taskId].data[0] = gBattleAnimArgs[0];
    gTasks[taskId].data[2] = 0x100 + IndexOfSpritePaletteTag(ANIM_TAG_RAINBOW_RINGS) as i16 * 16;
    gTasks[taskId].func = Some(AnimTask_RotateAuroraRingColors_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_RotateAuroraRingColors_Step(taskId: u8) {
    let mut i: i32 = 0;
    let mut palIndex: u16 = 0;
    if ({
        gTasks[taskId].data[10] += 1;
        gTasks[taskId].data[10]
    }) == 3
    {
        let mut rgbBuffer: u16 = 0;
        gTasks[taskId].data[10] = 0;
        palIndex = gTasks[taskId].data[2] as u16 + 1;
        rgbBuffer = gPlttBufferFaded[palIndex];
        i = 1;
        while i < 8 {
            gPlttBufferFaded[palIndex as i32 + i - 1] = gPlttBufferFaded[palIndex as i32 + i];
            i += 1;
        }
        gPlttBufferFaded[palIndex as i32 + 7] = rgbBuffer;
    }
    if ({
        gTasks[taskId].data[11] += 1;
        gTasks[taskId].data[11]
    }) == gTasks[taskId].data[0]
    {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimToTargetInSinWave(sprite: *mut Sprite) {
    let mut retArg: u16 = 0;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = 30;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    InitAnimLinearTranslation(sprite);
    (*sprite).data[5] = div_i32(0xD200, (*sprite).data[0] as i32) as i16;
    (*sprite).data[7] = gBattleAnimArgs[3];
    retArg = gBattleAnimArgs[7] as u16;
    if gBattleAnimArgs[7] > 127 {
        (*sprite).data[6] = (retArg as i16 - 127) * 256;
        (*sprite).data[7] = -(*sprite).data[7];
    } else {
        (*sprite).data[6] = retArg as i16 * 256;
    }
    (*sprite).callback = Some(AnimToTargetInSinWave_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimToTargetInSinWave_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
    (*sprite).y2 += Sin((*sprite).data[6] >> 8, (*sprite).data[7]);
    if (*sprite).data[6] as i32 + (*sprite).data[5] as i32 >> 8 > 127 {
        (*sprite).data[6] = 0;
        (*sprite).data[7] = -(*sprite).data[7];
    } else {
        (*sprite).data[6] += (*sprite).data[5];
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StartSinAnimTimer(taskId: u8) {
    gTasks[taskId].data[0] = gBattleAnimArgs[0];
    gBattleAnimArgs[7] = 0;
    gTasks[taskId].func = Some(AnimTask_RunSinAnimTimer);
}
pub(crate) unsafe extern "C" fn AnimTask_RunSinAnimTimer(taskId: u8) {
    gBattleAnimArgs[7] = gBattleAnimArgs[7] + 3 & 0xFF;
    if ({
        gTasks[taskId].data[0] -= 1;
        gTasks[taskId].data[0]
    }) == 0
    {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimHydroCannonCharge(sprite: *mut Sprite) {
    let mut priority: u8 = 0;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
    (*sprite).y2 = -10;
    priority = GetBattlerSpriteSubpriority(gBattleAnimAttacker);
    if IsContest() == 0 {
        if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
            (*sprite).x2 = 10;
            (*sprite).subpriority = priority + 2;
        } else {
            (*sprite).x2 = -10;
            (*sprite).subpriority = priority - 2;
        }
    } else {
        (*sprite).x2 = -10;
        (*sprite).subpriority = priority + 2;
    }
    (*sprite).callback = Some(AnimHydroCannonCharge_Step);
}
pub(crate) unsafe extern "C" fn AnimHydroCannonCharge_Step(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimHydroCannonBeam(sprite: *mut Sprite) {
    let mut respectMonPicOffsets: u8 = 0;
    let mut coordType: u8 = 0;
    if GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget) {
        gBattleAnimArgs[0] *= -1;
        if GetBattlerPosition(gBattleAnimAttacker) == B_POSITION_PLAYER_LEFT
            || GetBattlerPosition(gBattleAnimAttacker) == B_POSITION_OPPONENT_LEFT
        {
            gBattleAnimArgs[0] *= -1;
        }
    }
    if gBattleAnimArgs[5] as i32 & 0xFF00 == 0 {
        respectMonPicOffsets = TRUE;
    } else {
        respectMonPicOffsets = FALSE;
    }
    if gBattleAnimArgs[5] as u8 == 0 {
        coordType = BATTLER_COORD_Y_PIC_OFFSET;
    } else {
        coordType = BATTLER_COORD_Y;
    }
    InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[2];
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimTarget, coordType) as i16 + gBattleAnimArgs[3];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn AnimWaterGunDroplet(sprite: *mut Sprite) {
    InitSpritePosToAnimTarget(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] = (*sprite).x + gBattleAnimArgs[2];
    (*sprite).data[4] = (*sprite).y + gBattleAnimArgs[4];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn AnimSmallBubblePair(sprite: *mut Sprite) {
    if gBattleAnimArgs[3] != ANIM_ATTACKER as i16 {
        InitSpritePosToAnimTarget(sprite, TRUE);
    } else {
        InitSpritePosToAnimAttacker(sprite, TRUE);
    }
    (*sprite).data[7] = gBattleAnimArgs[2];
    (*sprite).callback = Some(AnimSmallBubblePair_Step);
}
pub(crate) unsafe extern "C" fn AnimSmallBubblePair_Step(sprite: *mut Sprite) {
    (*sprite).data[0] = (*sprite).data[0] + 11 & 0xFF;
    (*sprite).x2 = Sin((*sprite).data[0], 4);
    (*sprite).data[1] += 48;
    (*sprite).y2 = -((*sprite).data[1] >> 8);
    if ({
        (*sprite).data[7] -= 1;
        (*sprite).data[7]
    }) == -1
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CreateSurfWave(taskId: u8) {
    let mut cmd: *mut Anon1 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon1;
    let mut animBg: BattleAnimBgData = zeroed();
    let mut taskId2: u8 = 0;
    let mut x: *mut u16 = null_mut();
    let mut y: *mut u16 = null_mut();
    x = &raw mut gBattle_BG1_X;
    y = &raw mut gBattle_BG1_Y;
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
    SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 1);
    GetBattleAnimBg1Data(&raw mut animBg);
    if IsContest() == 0 {
        SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 1);
        if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
            AnimLoadCompressedBgTilemap(
                animBg.bgId as u32,
                gBattleAnimBgTilemap_SurfOpponent.as_ptr().cast_mut() as *mut c_void,
            );
        } else {
            AnimLoadCompressedBgTilemap(
                animBg.bgId as u32,
                gBattleAnimBgTilemap_SurfPlayer.as_ptr().cast_mut() as *mut c_void,
            );
        }
    } else {
        AnimLoadCompressedBgTilemapHandleContest(
            &raw mut animBg,
            gBattleAnimBgTilemap_SurfContest.as_ptr().cast_mut() as *mut c_void,
            TRUE as u32,
        );
    }
    AnimLoadCompressedBgGfx(
        animBg.bgId as u32,
        gBattleAnimBgImage_Surf.as_ptr().cast_mut(),
        animBg.tilesOffset as u32,
    );
    if (*cmd).palette == ANIM_SURF_PAL_SURF {
        LoadCompressedPalette(
            gBattleAnimBgPalette_Surf.as_ptr().cast_mut(),
            0x000 + animBg.paletteId as u16 * 16,
            32,
        );
    } else {
        LoadCompressedPalette(
            gBattleAnimBackgroundImageMuddyWater_Pal.as_ptr().cast_mut(),
            0x000 + animBg.paletteId as u16 * 16,
            32,
        );
    }
    taskId2 = CreateTask(
        Some(AnimTask_SurfWaveScanlineEffect),
        gTasks[taskId].priority + 1,
    );
    gTasks[taskId].data[15] = taskId2 as i16;
    gTasks[taskId2].data[0] = 0;
    gTasks[taskId2].data[1] = 0x1000;
    gTasks[taskId2].data[2] = 0x1000;
    if IsContest() != 0 {
        *x = 65456;
        *y = 65488;
        gTasks[taskId].data[0] = 2;
        gTasks[taskId].data[1] = 1;
        gTasks[taskId2].data[3] = 0;
    } else if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
        *x = 65312;
        *y = 256;
        gTasks[taskId].data[0] = 2;
        gTasks[taskId].data[1] = -1;
        gTasks[taskId2].data[3] = 1;
    } else {
        *x = 0;
        *y = 65488;
        gTasks[taskId].data[0] = -2;
        gTasks[taskId].data[1] = 1;
        gTasks[taskId2].data[3] = 0;
    }
    SetGpuReg(REG_OFFSET_BG1HOFS, *x);
    SetGpuReg(REG_OFFSET_BG1VOFS, *y);
    if gTasks[taskId2].data[3] == 0 {
        gTasks[taskId2].data[4] = 48;
        gTasks[taskId2].data[5] = 112;
    } else {
        gTasks[taskId2].data[4] = 0;
        gTasks[taskId2].data[5] = 0;
    }
    gTasks[taskId].data[6] = 1;
    gTasks[taskId].func = Some(AnimTask_CreateSurfWave_Step1);
}
pub(crate) unsafe extern "C" fn AnimTask_CreateSurfWave_Step1(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    let mut i: u8 = 0;
    let mut rgbBuffer: u16 = 0;
    let mut BGptrX: *mut u16 = &raw mut gBattle_BG1_X;
    let mut BGptrY: *mut u16 = &raw mut gBattle_BG1_Y;
    *BGptrX += gTasks[taskId].data[0] as u16;
    *BGptrY += gTasks[taskId].data[1] as u16;
    GetBattleAnimBg1Data(&raw mut animBg);
    gTasks[taskId].data[2] += gTasks[taskId].data[1];
    if ({
        gTasks[taskId].data[5] += 1;
        gTasks[taskId].data[5]
    }) == 4
    {
        rgbBuffer = gPlttBufferFaded[0x000 + animBg.paletteId as i32 * 16 + 7];
        i = 6;
        while i != 0 {
            gPlttBufferFaded[0x000 + animBg.paletteId as i32 * 16 + 1 + i as i32] =
                gPlttBufferFaded[0x000 + animBg.paletteId as i32 * 16 + 1 + i as i32 - 1];
            i -= 1;
        }
        gPlttBufferFaded[0x000 + animBg.paletteId as i32 * 16 + 1] = rgbBuffer;
        gTasks[taskId].data[5] = 0;
    }
    if ({
        gTasks[taskId].data[6] += 1;
        gTasks[taskId].data[6]
    }) > 1
    {
        gTasks[taskId].data[6] = 0;
        if ({
            gTasks[taskId].data[3] += 1;
            gTasks[taskId].data[3]
        }) <= 13
        {
            gTasks[gTasks[taskId].data[15]].data[1] =
                gTasks[taskId].data[3] | 16 - gTasks[taskId].data[3] << 8;
            gTasks[taskId].data[4] += 1;
        }
        if gTasks[taskId].data[3] > 54 {
            gTasks[taskId].data[4] -= 1;
            gTasks[gTasks[taskId].data[15]].data[1] =
                gTasks[taskId].data[4] | 16 - gTasks[taskId].data[4] << 8;
        }
    }
    if gTasks[gTasks[taskId].data[15]].data[1] as i32 & 0x1F == 0 {
        gTasks[taskId].data[0] = gTasks[gTasks[taskId].data[15]].data[1] & 0x1F;
        gTasks[taskId].func = Some(AnimTask_CreateSurfWave_Step2);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_CreateSurfWave_Step2(taskId: u8) {
    let mut BGptrX: *mut u16 = &raw mut gBattle_BG1_X;
    let mut BGptrY: *mut u16 = &raw mut gBattle_BG1_Y;
    if gTasks[taskId].data[0] == 0 {
        ClearBattleAnimBg(1);
        ClearBattleAnimBg(2);
        gTasks[taskId].data[0] += 1;
    } else {
        if IsContest() == 0 {
            SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
        }
        *BGptrX = 0;
        *BGptrY = 0;
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        gTasks[gTasks[taskId].data[15]].data[15] = -1;
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SurfWaveScanlineEffect(taskId: u8) {
    let mut i: i16 = 0;
    let mut params: ScanlineEffectParams = zeroed();
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            i = 0;
            while i < (*task).data[4] {
                gScanlineEffectRegBuffers[0][i] = {
                    gScanlineEffectRegBuffers[1][i] = (*task).data[2] as u16;
                    gScanlineEffectRegBuffers[1][i]
                };
                i += 1;
            }
            i = (*task).data[4];
            while i < (*task).data[5] {
                gScanlineEffectRegBuffers[0][i] = {
                    gScanlineEffectRegBuffers[1][i] = (*task).data[1] as u16;
                    gScanlineEffectRegBuffers[1][i]
                };
                i += 1;
            }
            i = (*task).data[5];
            while i < 160 {
                gScanlineEffectRegBuffers[0][i] = {
                    gScanlineEffectRegBuffers[1][i] = (*task).data[2] as u16;
                    gScanlineEffectRegBuffers[1][i]
                };
                i += 1;
            }
            if (*task).data[4] == 0 {
                gScanlineEffectRegBuffers[0][i] = {
                    gScanlineEffectRegBuffers[1][i] = (*task).data[1] as u16;
                    gScanlineEffectRegBuffers[1][i]
                };
            } else {
                gScanlineEffectRegBuffers[0][i] = {
                    gScanlineEffectRegBuffers[1][i] = (*task).data[2] as u16;
                    gScanlineEffectRegBuffers[1][i]
                };
            }
            params.dmaDest = 67108946 as usize as *mut u16 as *mut c_void;
            params.dmaControl = 0xa2600001;
            params.initState = 1;
            params.unused9 = 0;
            ScanlineEffect_SetParams(params);
            (*task).data[0] += 1;
        }
        1 => {
            if (*task).data[3] == 0 {
                if ({
                    (*task).data[4] -= 1;
                    (*task).data[4]
                }) <= 0
                {
                    (*task).data[4] = 0;
                    (*task).data[0] += 1;
                }
            } else if ({
                (*task).data[5] += 1;
                (*task).data[5]
            }) > 111
            {
                (*task).data[0] += 1;
            }
            i = 0;
            while i < (*task).data[4] {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] = (*task).data[2] as u16;
                i += 1;
            }
            i = (*task).data[4];
            while i < (*task).data[5] {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] = (*task).data[1] as u16;
                i += 1;
            }
            i = (*task).data[5];
            while i < 160 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] = (*task).data[2] as u16;
                i += 1;
            }
        }
        2 => {
            i = 0;
            while i < (*task).data[4] {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] = (*task).data[2] as u16;
                i += 1;
            }
            i = (*task).data[4];
            while i < (*task).data[5] {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] = (*task).data[1] as u16;
                i += 1;
            }
            i = (*task).data[5];
            while i < 160 {
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] = (*task).data[2] as u16;
                i += 1;
            }
            if (*task).data[15] == -1 {
                ScanlineEffect_Stop();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimSmallDriftingBubbles(sprite: *mut Sprite) {
    let mut randData: i16 = 0;
    let mut randData2: i16 = 0;
    (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 8);
    InitSpritePosToAnimTarget(sprite, TRUE);
    randData = Random2() as i16 & 0xFF | 256;
    randData2 = Random2() as i16 & 0x1FF;
    if randData2 > 255 {
        randData2 = 256 - randData2;
    }
    (*sprite).data[1] = randData;
    (*sprite).data[2] = randData2;
    (*sprite).callback = Some(AnimSmallDriftingBubbles_Step);
}
pub(crate) unsafe extern "C" fn AnimSmallDriftingBubbles_Step(sprite: *mut Sprite) {
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
pub unsafe extern "C" fn AnimTask_WaterSpoutLaunch(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[15] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*task).data[5] = gSprites[(*task).data[15]].y;
    (*task).data[1] = GetWaterSpoutPowerForAnim() as i16;
    PrepareBattlerSpriteForRotScale((*task).data[15] as u8, ST_OAM_OBJ_NORMAL);
    (*task).func = Some(AnimTask_WaterSpoutLaunch_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_WaterSpoutLaunch_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    'l1: {
        let sw1: i16 = (*task).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            PrepareEruptAnimTaskData(task, (*task).data[15] as u8, 0x100, 0x100, 0xE0, 0x200, 32);
            (*task).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if ({
                (*task).data[3] += 1;
                (*task).data[3]
            }) > 1
            {
                (*task).data[3] = 0;
                if ({
                    (*task).data[4] += 1;
                    (*task).data[4]
                }) as i32
                    & 1
                    != 0
                {
                    gSprites[(*task).data[15]].x2 = 3;
                    gSprites[(*task).data[15]].y += 1;
                } else {
                    gSprites[(*task).data[15]].x2 = -3;
                }
            }
            if UpdateEruptAnimTask(task) == 0 {
                SetBattlerSpriteYOffsetFromYScale((*task).data[15] as u8);
                gSprites[(*task).data[15]].x2 = 0;
                (*task).data[3] = 0;
                (*task).data[4] = 0;
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if ({
                (*task).data[3] += 1;
                (*task).data[3]
            }) > 4
            {
                PrepareEruptAnimTaskData(task, (*task).data[15] as u8, 0xE0, 0x200, 0x180, 0xE0, 8);
                (*task).data[3] = 0;
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if UpdateEruptAnimTask(task) == 0 {
                (*task).data[3] = 0;
                (*task).data[4] = 0;
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            CreateWaterSpoutLaunchDroplets(task, taskId);
            (*task).data[0] += 1;
        }
        if fall || sw1 == 5 {
            fall = true;
            if ({
                (*task).data[3] += 1;
                (*task).data[3]
            }) > 1
            {
                (*task).data[3] = 0;
                if ({
                    (*task).data[4] += 1;
                    (*task).data[4]
                }) as i32
                    & 1
                    != 0
                {
                    gSprites[(*task).data[15]].y2 += 2;
                } else {
                    gSprites[(*task).data[15]].y2 -= 2;
                }
                if (*task).data[4] == 10 {
                    PrepareEruptAnimTaskData(
                        task,
                        (*task).data[15] as u8,
                        0x180,
                        0xE0,
                        0x100,
                        0x100,
                        8,
                    );
                    (*task).data[3] = 0;
                    (*task).data[4] = 0;
                    (*task).data[0] += 1;
                }
            }
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            gSprites[(*task).data[15]].y -= 1;
            if UpdateEruptAnimTask(task) == 0 {
                ResetSpriteRotScale((*task).data[15] as u8);
                gSprites[(*task).data[15]].y = (*task).data[5];
                (*task).data[4] = 0;
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
            if (*task).data[2] == 0 {
                DestroyAnimVisualTask(taskId);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn GetWaterSpoutPowerForAnim() -> u8 {
    let mut i: u8 = 0;
    let mut hp: u16 = 0;
    let mut maxhp: u16 = 0;
    let mut partyIndex: u16 = 0;
    let mut slot: *mut Pokemon = null_mut();
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        partyIndex = gBattlerPartyIndexes[gBattleAnimAttacker];
        slot = &raw mut gPlayerParty[partyIndex];
        maxhp = GetMonData2(slot, MON_DATA_MAX_HP) as u16;
        hp = GetMonData2(slot, MON_DATA_HP) as u16;
        maxhp = (maxhp as i32 / 4) as u16;
    } else {
        partyIndex = gBattlerPartyIndexes[gBattleAnimAttacker];
        slot = &raw mut gEnemyParty[partyIndex];
        maxhp = GetMonData2(slot, MON_DATA_MAX_HP) as u16;
        hp = GetMonData2(slot, MON_DATA_HP) as u16;
        maxhp = (maxhp as i32 / 4) as u16;
    }
    i = 0;
    while i < 3 {
        if (hp as i32) < maxhp as i32 * (i as i32 + 1) {
            return i;
        }
        i += 1;
    }
    return 3;
}
pub(crate) unsafe extern "C" fn CreateWaterSpoutLaunchDroplets(task: *mut Task, taskId: u8) {
    let mut i: i16 = 0;
    let mut attackerCoordX: i16 =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    let mut attackerCoordY: i16 =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    let mut trigIndex: i16 = 172;
    let mut subpriority: u8 = GetBattlerSpriteSubpriority(gBattleAnimAttacker) - 1;
    let mut increment: i16 = 4 - (*task).data[1];
    let mut spriteId: u8 = 0;
    if increment <= 0 {
        increment = 1;
    }
    i = 0;
    while i < 20 {
        spriteId = CreateSprite(
            (&raw const *gSmallWaterOrbSpriteTemplate).cast_mut(),
            attackerCoordX,
            attackerCoordY,
            subpriority,
        );
        if spriteId != MAX_SPRITES {
            gSprites[spriteId].data[1] = i;
            gSprites[spriteId].data[2] = attackerCoordX * 16;
            gSprites[spriteId].data[3] = attackerCoordY * 16;
            gSprites[spriteId].data[4] = Cos(trigIndex, 64);
            gSprites[spriteId].data[5] = Sin(trigIndex, 64);
            gSprites[spriteId].data[6] = taskId as i16;
            gSprites[spriteId].data[7] = 2;
            if (*task).data[2] as i32 & 1 != 0 {
                AnimSmallWaterOrb(&raw mut gSprites[spriteId]);
            }
            (*task).data[2] += 1;
        }
        trigIndex = trigIndex + increment * 2;
        trigIndex &= 0xFF;
        i += increment;
    }
}
pub(crate) unsafe extern "C" fn AnimSmallWaterOrb(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).data[4] += (*sprite).data[1] % 6 * 3;
            (*sprite).data[5] += (*sprite).data[1] % 3 * 3;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).data[2] += (*sprite).data[4];
            (*sprite).data[3] += (*sprite).data[5];
            (*sprite).x = (*sprite).data[2] >> 4;
            (*sprite).y = (*sprite).data[3] >> 4;
            if (*sprite).x < -8 || (*sprite).x > 248 || (*sprite).y < -8 || (*sprite).y > 120 {
                gTasks[(*sprite).data[6]].data[(*sprite).data[7]] -= 1;
                DestroySprite(sprite);
            }
            break 'l1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_WaterSpoutRain(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[1] = GetWaterSpoutPowerForAnim() as i16;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*task).data[4] = 136;
        (*task).data[6] = 40;
    } else {
        (*task).data[4] = 16;
        (*task).data[6] = 80;
    }
    (*task).data[5] = 98;
    (*task).data[7] = (*task).data[4] + 49;
    (*task).data[12] = (*task).data[1] * 5 + 5;
    (*task).func = Some(AnimTask_WaterSpoutRain_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_WaterSpoutRain_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut taskId2: u8 = 0;
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[2] += 1;
                (*task).data[2]
            }) > 2
            {
                (*task).data[2] = 0;
                CreateWaterSpoutRainDroplet(task, taskId);
            }
            if (*task).data[10] != 0 && (*task).data[13] == 0 {
                gBattleAnimArgs[0] = ANIM_TARGET as i16;
                gBattleAnimArgs[1] = 0;
                gBattleAnimArgs[2] = 12;
                taskId2 = CreateTask(Some(AnimTask_HorizontalShake), 80);
                if taskId2 != TASK_NONE {
                    gTasks[taskId2].func.unwrap_unchecked()(taskId2);
                    gAnimVisualTaskCount += 1;
                }
                gBattleAnimArgs[0] = ANIM_DEF_PARTNER as i16;
                taskId2 = CreateTask(Some(AnimTask_HorizontalShake), 80);
                if taskId2 != TASK_NONE {
                    gTasks[taskId2].func.unwrap_unchecked()(taskId2);
                    gAnimVisualTaskCount += 1;
                }
                (*task).data[13] = 1;
            }
            if (*task).data[11] >= (*task).data[12] {
                (*task).data[0] += 1;
            }
        }
        1 => {
            if (*task).data[9] == 0 {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateWaterSpoutRainDroplet(task: *mut Task, taskId: u8) {
    let mut yPosArg: u16 =
        (gSineTable[(*task).data[8]] as i32 + 3 >> 4) as u16 + (*task).data[6] as u16;
    let mut spriteId: u8 = CreateSprite(
        (&raw const *gSmallWaterOrbSpriteTemplate).cast_mut(),
        (*task).data[7],
        0,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].callback = Some(AnimWaterSpoutRain);
        gSprites[spriteId].data[5] = yPosArg as i16;
        gSprites[spriteId].data[6] = taskId as i16;
        gSprites[spriteId].data[7] = 9;
        (*task).data[9] += 1;
    }
    (*task).data[11] += 1;
    (*task).data[8] = (*task).data[8] + 39 & 0xFF;
    (*task).data[7] = rem_i32(
        1103515245 * (*task).data[7] as i32 + 12345,
        (*task).data[5] as i32,
    ) as i16
        + (*task).data[4];
}
pub(crate) unsafe extern "C" fn AnimWaterSpoutRain(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).y += 8;
        if (*sprite).y >= (*sprite).data[5] {
            gTasks[(*sprite).data[6]].data[10] = 1;
            (*sprite).data[1] = CreateSprite(
                (&raw const gWaterHitSplatSpriteTemplate).cast_mut(),
                (*sprite).x,
                (*sprite).y,
                1,
            ) as i16;
            if (*sprite).data[1] != MAX_SPRITES as i16 {
                StartSpriteAffineAnim(&raw mut gSprites[(*sprite).data[1]], 3);
                gSprites[(*sprite).data[1]].data[6] = (*sprite).data[6];
                gSprites[(*sprite).data[1]].data[7] = (*sprite).data[7];
                gSprites[(*sprite).data[1]].callback = Some(AnimWaterSpoutRainHit);
            }
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterSpoutRainHit(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 1
    {
        (*sprite).data[1] = 0;
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) == 12
        {
            gTasks[(*sprite).data[6]].data[(*sprite).data[7]] -= 1;
            FreeOamMatrix((*sprite).oam.matrixNum() as u8);
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_WaterSport(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[3] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*task).data[4] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*task).data[7] = (if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        1
    } else {
        -1
    }) as i16;
    if IsContest() != 0 {
        (*task).data[7] *= -1;
    }
    (*task).data[5] = (*task).data[3] + (*task).data[7] * 8;
    (*task).data[6] = (*task).data[4] - (*task).data[7] * 8;
    (*task).data[9] = -32;
    (*task).data[1] = 0;
    (*task).data[0] = 0;
    (*task).func = Some(AnimTask_WaterSport_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_WaterSport_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            CreateWaterSportDroplet(task);
            if (*task).data[10] != 0 {
                (*task).data[0] += 1;
            }
        }
        1 => {
            CreateWaterSportDroplet(task);
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 16
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        2 => {
            CreateWaterSportDroplet(task);
            (*task).data[5] += (*task).data[7] * 6;
            if !((*task).data[5] >= -16 && (*task).data[5] <= 256) {
                if ({
                    (*task).data[12] += 1;
                    (*task).data[12]
                }) > 2
                {
                    (*task).data[13] = 1;
                    (*task).data[0] = 6;
                    (*task).data[1] = 0;
                } else {
                    (*task).data[1] = 0;
                    (*task).data[0] += 1;
                }
            }
        }
        3 => {
            CreateWaterSportDroplet(task);
            (*task).data[6] -= (*task).data[7] * 2;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 7
            {
                (*task).data[0] += 1;
            }
        }
        4 => {
            CreateWaterSportDroplet(task);
            (*task).data[5] -= (*task).data[7] * 6;
            if !((*task).data[5] >= -16 && (*task).data[5] <= 256) {
                (*task).data[12] += 1;
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        5 => {
            CreateWaterSportDroplet(task);
            (*task).data[6] -= (*task).data[7] * 2;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 7
            {
                (*task).data[0] = 2;
            }
        }
        6 => {
            if (*task).data[8] == 0 {
                (*task).data[0] += 1;
            }
        }
        _ => {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateWaterSportDroplet(task: *mut Task) {
    let mut spriteId: u8 = 0;
    if ({
        (*task).data[2] += 1;
        (*task).data[2]
    }) > 1
    {
        (*task).data[2] = 0;
        spriteId = CreateSprite(
            (&raw const *gSmallWaterOrbSpriteTemplate).cast_mut(),
            (*task).data[3],
            (*task).data[4],
            10,
        );
        if spriteId != MAX_SPRITES {
            gSprites[spriteId].data[0] = 16;
            gSprites[spriteId].data[2] = (*task).data[5];
            gSprites[spriteId].data[4] = (*task).data[6];
            gSprites[spriteId].data[5] = (*task).data[9];
            InitAnimArcTranslation(&raw mut gSprites[spriteId]);
            gSprites[spriteId].callback = Some(AnimWaterSportDroplet);
            (*task).data[8] += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterSportDroplet(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).data[0] = 6;
        (*sprite).data[2] = (Random2() as i16 & 0x1F) - 16 + (*sprite).x;
        (*sprite).data[4] = (Random2() as i16 & 0x1F) - 16 + (*sprite).y;
        (*sprite).data[5] = !(Random2() as i16 & 7);
        InitAnimArcTranslation(sprite);
        (*sprite).callback = Some(AnimWaterSportDroplet_Step);
    }
}
pub(crate) unsafe extern "C" fn AnimWaterSportDroplet_Step(sprite: *mut Sprite) {
    let mut i: u16 = 0;
    if TranslateAnimHorizontalArc(sprite) != 0 {
        i = 0;
        while i < NUM_TASKS as u16 {
            if gTasks[i].func == Some(AnimTask_WaterSport_Step as unsafe extern "C" fn(u8)) {
                gTasks[i].data[10] = 1;
                gTasks[i].data[8] -= 1;
                DestroySprite(sprite);
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterPulseBubble(sprite: *mut Sprite) {
    (*sprite).x = gBattleAnimArgs[0];
    (*sprite).y = gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[2] = gBattleAnimArgs[4];
    (*sprite).data[3] = gBattleAnimArgs[5];
    (*sprite).callback = Some(AnimWaterPulseBubble_Step);
}
pub(crate) unsafe extern "C" fn AnimWaterPulseBubble_Step(sprite: *mut Sprite) {
    (*sprite).data[4] -= (*sprite).data[0];
    (*sprite).y2 = (*sprite).data[4] / 10;
    (*sprite).data[5] = (*sprite).data[5] + (*sprite).data[1] & 0xFF;
    (*sprite).x2 = Sin((*sprite).data[5], (*sprite).data[2]);
    if ({
        (*sprite).data[3] -= 1;
        (*sprite).data[3]
    }) == 0
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimWaterPulseRingBubble(sprite: *mut Sprite) {
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).data[4] += (*sprite).data[2];
    (*sprite).x2 = (*sprite).data[3] >> 7;
    (*sprite).y2 = (*sprite).data[4] >> 7;
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == 0
    {
        FreeSpriteOamMatrix(sprite);
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimWaterPulseRing(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[1] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[3] = gBattleAnimArgs[2];
    (*sprite).data[4] = gBattleAnimArgs[3];
    (*sprite).callback = Some(AnimWaterPulseRing_Step);
}
pub(crate) unsafe extern "C" fn AnimWaterPulseRing_Step(sprite: *mut Sprite) {
    let mut xDiff: i32 = (*sprite).data[1] as i32 - (*sprite).x as i32;
    let mut yDiff: i32 = (*sprite).data[2] as i32 - (*sprite).y as i32;
    (*sprite).x2 = div_i32((*sprite).data[0] as i32 * xDiff, (*sprite).data[3] as i32) as i16;
    (*sprite).y2 = div_i32((*sprite).data[0] as i32 * yDiff, (*sprite).data[3] as i32) as i16;
    if ({
        (*sprite).data[5] += 1;
        (*sprite).data[5]
    }) == (*sprite).data[4]
    {
        (*sprite).data[5] = 0;
        CreateWaterPulseRingBubbles(sprite, xDiff, yDiff);
    }
    if (*sprite).data[3] == (*sprite).data[0] {
        DestroyAnimSprite(sprite);
    }
    (*sprite).data[0] += 1;
}
pub(crate) unsafe extern "C" fn CreateWaterPulseRingBubbles(
    sprite: *mut Sprite,
    xDiff: i32,
    yDiff: i32,
) {
    let mut combinedX: i16 = 0;
    let mut combinedY: i16 = 0;
    let mut i: i16 = 0;
    let mut something: i16 = 0;
    let mut unusedVar: i16 = 1;
    let mut randomSomethingY: i16 = 0;
    let mut randomSomethingX: i16 = 0;
    let mut spriteId: u8 = 0;
    something = (*sprite).data[0] / 2;
    combinedX = (*sprite).x + (*sprite).x2;
    combinedY = (*sprite).y + (*sprite).y2;
    if yDiff < 0 {
        unusedVar *= -1;
    }
    randomSomethingY = yDiff as i16 + (Random2() as i32 % 10) as i16 - 5;
    randomSomethingX = -(xDiff as i16) + (Random2() as i32 % 10) as i16 - 5;
    i = 0;
    while i <= 0 {
        spriteId = CreateSprite(
            (&raw const *gWaterPulseRingBubbleSpriteTemplate).cast_mut(),
            combinedX,
            combinedY + something,
            130,
        );
        gSprites[spriteId].data[0] = 20;
        gSprites[spriteId].data[1] = randomSomethingY;
        gSprites[spriteId].subpriority = GetBattlerSpriteSubpriority(gBattleAnimAttacker) - 1;
        if randomSomethingX < 0 {
            gSprites[spriteId].data[2] = -randomSomethingX;
        } else {
            gSprites[spriteId].data[2] = randomSomethingX;
        }
        i += 1;
    }
    i = 0;
    while i <= 0 {
        spriteId = CreateSprite(
            (&raw const *gWaterPulseRingBubbleSpriteTemplate).cast_mut(),
            combinedX,
            combinedY - something,
            130,
        );
        gSprites[spriteId].data[0] = 20;
        gSprites[spriteId].data[1] = randomSomethingY;
        gSprites[spriteId].subpriority = GetBattlerSpriteSubpriority(gBattleAnimAttacker) - 1;
        if randomSomethingX > 0 {
            gSprites[spriteId].data[2] = -randomSomethingX;
        } else {
            gSprites[spriteId].data[2] = randomSomethingX;
        }
        i += 1;
    }
}
