//! Translated from `src/battle_anim_bug.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sAffineAnim_MegahornHorn_0 sAffineAnim_MegahornHorn_1 sAffineAnim_MegahornHorn_2 sAffineAnims_MegahornHorn gMegahornHornSpriteTemplate sAffineAnim_LeechLifeNeedle_0 sAffineAnim_LeechLifeNeedle_1 sAffineAnim_LeechLifeNeedle_2 sAffineAnims_LeechLifeNeedle gLeechLifeNeedleSpriteTemplate gWebThreadSpriteTemplate gStringWrapSpriteTemplate sAffineAnim_SpiderWeb sAffineAnims_SpiderWeb gSpiderWebSpriteTemplate gLinearStingerSpriteTemplate gPinMissileSpriteTemplate gIcicleSpearSpriteTemplate sAffineAnim_TailGlowOrb sAffineAnims_TailGlowOrb gTailGlowOrbSpriteTemplate

/// `__anon1`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon1 {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon1 {}

/// `__anon2`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon2 {
    pub x: i16,
    pub y: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon2 {}

/// `__anon3`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon3 {
    pub x: i16,
    pub y: i16,
    pub unk2: i16,
    pub amplitude: i16,
    pub targetsBoth: i16,
}

unsafe impl Sync for Anon3 {}

/// `__anon4`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon4 {
    pub x: i16,
    pub y: i16,
}

unsafe impl Sync for Anon4 {}

/// `__anon5`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon5 {
    pub initialX: i16,
    pub intialY: i16,
    pub targetX: i16,
    pub targetY: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon5 {}

/// `__anon6`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon6 {
    pub initialX: i16,
    pub intialY: i16,
    pub targetX: i16,
    pub targetY: i16,
    pub duration: i16,
    pub waveAmplitude: i16,
}

unsafe impl Sync for Anon6 {}

/// `__anon7`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon7 {
    pub relativeTo: i16,
}

unsafe impl Sync for Anon7 {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Anon1>() == 10);
    assert!(offset_of!(Anon1, x1) == 0);
    assert!(offset_of!(Anon1, y1) == 2);
    assert!(offset_of!(Anon1, x2) == 4);
    assert!(offset_of!(Anon1, y2) == 6);
    assert!(offset_of!(Anon1, duration) == 8);
    assert!(size_of::<Anon2>() == 6);
    assert!(offset_of!(Anon2, x) == 0);
    assert!(offset_of!(Anon2, y) == 2);
    assert!(offset_of!(Anon2, duration) == 4);
    assert!(size_of::<Anon3>() == 10);
    assert!(offset_of!(Anon3, x) == 0);
    assert!(offset_of!(Anon3, y) == 2);
    assert!(offset_of!(Anon3, unk2) == 4);
    assert!(offset_of!(Anon3, amplitude) == 6);
    assert!(offset_of!(Anon3, targetsBoth) == 8);
    assert!(size_of::<Anon4>() == 4);
    assert!(offset_of!(Anon4, x) == 0);
    assert!(offset_of!(Anon4, y) == 2);
    assert!(size_of::<Anon5>() == 10);
    assert!(offset_of!(Anon5, initialX) == 0);
    assert!(offset_of!(Anon5, intialY) == 2);
    assert!(offset_of!(Anon5, targetX) == 4);
    assert!(offset_of!(Anon5, targetY) == 6);
    assert!(offset_of!(Anon5, duration) == 8);
    assert!(size_of::<Anon6>() == 12);
    assert!(offset_of!(Anon6, initialX) == 0);
    assert!(offset_of!(Anon6, intialY) == 2);
    assert!(offset_of!(Anon6, targetX) == 4);
    assert!(offset_of!(Anon6, targetY) == 6);
    assert!(offset_of!(Anon6, duration) == 8);
    assert!(offset_of!(Anon6, waveAmplitude) == 10);
    assert!(size_of::<Anon7>() == 2);
    assert!(offset_of!(Anon7, relativeTo) == 0);
};

unsafe extern "C" {
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    fn AnimTranslateLinear(a0: *mut Sprite) -> u8;
    fn ArcTan2Neg(a0: i16, a1: i16) -> u16;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoord2(a0: u8, a1: u8) -> u8;
    fn InitAnimArcTranslation(a0: *mut Sprite);
    fn InitAnimLinearTranslationWithSpeed(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn IsContest() -> u8;
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut Sprite);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateAnimHorizontalArc(a0: *mut Sprite) -> u8;
    fn TrySetSpriteRotScale(a0: *mut Sprite, a1: u8, a2: i16, a3: i16, a4: u16);
}

pub(crate) unsafe extern "C" fn AnimMegahornHorn(sprite: *mut Sprite) {
    let mut cmd: *mut Anon1 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon1;
    if IsContest() != 0 {
        StartSpriteAffineAnim(sprite, 2);
        (*cmd).x2 = -(*cmd).x2;
        (*cmd).x1 = -(*cmd).x1;
    } else if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        StartSpriteAffineAnim(sprite, 1);
        (*cmd).y1 = -(*cmd).y1;
        (*cmd).x2 = -(*cmd).x2;
        (*cmd).y2 = -(*cmd).y2;
        (*cmd).x1 = -(*cmd).x1;
    }
    (*sprite).x = GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + (*cmd).x1;
    (*sprite).y =
        GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + (*cmd).y1;
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + (*cmd).x2;
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + (*cmd).y2;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn AnimLeechLifeNeedle(sprite: *mut Sprite) {
    let mut cmd: *mut Anon2 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon2;
    if IsContest() != 0 {
        (*cmd).x = -(*cmd).x;
        StartSpriteAffineAnim(sprite, 2);
    } else if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*cmd).y = -(*cmd).y;
        (*cmd).x = -(*cmd).x;
    }
    (*sprite).x = GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + (*cmd).x;
    (*sprite).y =
        GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + (*cmd).y;
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn AnimTranslateWebThread(sprite: *mut Sprite) {
    let mut cmd: *mut Anon3 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon3;
    if IsContest() != 0 {
        (*cmd).unk2 = (*cmd).unk2 / 2;
    }
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = (*cmd).unk2;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    if (*cmd).targetsBoth == 0 {
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
    InitAnimLinearTranslationWithSpeed(sprite);
    (*sprite).data[5] = (*cmd).amplitude;
    (*sprite).callback = Some(AnimTranslateWebThread_Step);
}
pub(crate) unsafe extern "C" fn AnimTranslateWebThread_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 {
        DestroyAnimSprite(sprite);
        return;
    }
    (*sprite).x2 += Sin((*sprite).data[6], (*sprite).data[5]);
    (*sprite).data[6] = (*sprite).data[6] + 13 & 0xFF;
}
pub(crate) unsafe extern "C" fn AnimStringWrap(sprite: *mut Sprite) {
    let mut cmd: *mut Anon4 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon4;
    SetAverageBattlerPositions(
        gBattleAnimTarget,
        FALSE,
        &raw mut (*sprite).x,
        &raw mut (*sprite).y,
    );
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        (*sprite).x -= (*cmd).x;
    } else {
        (*sprite).x += (*cmd).x;
    }
    (*sprite).y += (*cmd).y;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*sprite).y += 8;
    }
    (*sprite).callback = Some(AnimStringWrap_Step);
}
pub(crate) unsafe extern "C" fn AnimStringWrap_Step(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimSpiderWeb(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 16);
    (*sprite).data[0] = 16;
    (*sprite).callback = Some(AnimSpiderWeb_Step);
}
pub(crate) unsafe extern "C" fn AnimSpiderWeb_Step(sprite: *mut Sprite) {
    if (*sprite).data[2] < 20 {
        (*sprite).data[2] += 1;
    } else if ({
        let t1 = (*sprite).data[1];
        (*sprite).data[1] += 1;
        t1
    }) as i32
        & 1
        != 0
    {
        (*sprite).data[0] -= 1;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (16 - (*sprite).data[0] as u16) << 8 | (*sprite).data[0] as u16,
        );
        if (*sprite).data[0] == 0 {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).callback = Some(AnimSpiderWeb_End);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSpiderWeb_End(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    DestroyAnimSprite(sprite);
}
pub(crate) unsafe extern "C" fn AnimTranslateStinger(sprite: *mut Sprite) {
    let mut cmd: *mut Anon5 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon5;
    let mut lVarX: i16 = 0;
    let mut lVarY: i16 = 0;
    let mut rot: u16 = 0;
    if IsContest() != 0 {
        (*cmd).targetX = -(*cmd).targetX;
    } else {
        if GetBattlerSide(gBattleAnimAttacker) != 0 {
            (*cmd).targetX = -(*cmd).targetX;
            (*cmd).intialY = -(*cmd).intialY;
            (*cmd).targetY = -(*cmd).targetY;
        }
    }
    if IsContest() == 0 && GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget)
    {
        if GetBattlerPosition(gBattleAnimTarget) == B_POSITION_PLAYER_LEFT
            || GetBattlerPosition(gBattleAnimTarget) == B_POSITION_OPPONENT_LEFT
        {
            (*cmd).targetX *= -1;
            (*cmd).initialX *= -1;
        }
    }
    InitSpritePosToAnimAttacker(sprite, TRUE);
    lVarX = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + (*cmd).targetX;
    lVarY = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + (*cmd).targetY;
    rot = ArcTan2Neg(lVarX - (*sprite).x, lVarY - (*sprite).y);
    rot -= 0x4000;
    TrySetSpriteRotScale(sprite, FALSE, 0x100, 0x100, rot);
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[2] = lVarX;
    (*sprite).data[4] = lVarY;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn AnimMissileArc(sprite: *mut Sprite) {
    let mut cmd: *mut Anon6 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon6;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        (*cmd).targetX = -(*cmd).targetX;
    }
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + (*cmd).targetX;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + (*cmd).targetY;
    (*sprite).data[5] = (*cmd).waveAmplitude;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimMissileArc_Step);
    (*sprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn AnimMissileArc_Step(sprite: *mut Sprite) {
    (*sprite).set_invisible(FALSE as u16);
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    } else {
        let mut tempData: CArray<i16, 8> = zeroed();
        let mut x2: i16 = 0;
        let mut y2: i16 = 0;
        let mut i: i32 = 0;
        i = 0;
        while i < 8 {
            tempData[i] = (*sprite).data[i];
            i += 1;
        }
        x2 = (*sprite).x + (*sprite).x2;
        y2 = (*sprite).y + (*sprite).y2;
        if TranslateAnimHorizontalArc(sprite) == 0 {
            let mut rotation: u16 = ArcTan2Neg(
                (*sprite).x + (*sprite).x2 - x2,
                (*sprite).y + (*sprite).y2 - y2,
            );
            rotation -= 0x4000;
            TrySetSpriteRotScale(sprite, FALSE, 0x100, 0x100, rotation);
            i = 0;
            while i < 8 {
                (*sprite).data[i] = tempData[i];
                i += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTailGlowOrb(sprite: *mut Sprite) {
    let mut cmd: *mut Anon7 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon7;
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).y =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16 + 18;
    } else {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).y =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + 18;
    }
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
}
