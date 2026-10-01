//! Translated from `src/pokemon_animation.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    unused_assignments,
    unused_variables
)]

use crate::battle_main::gBattlerPartyIndexes;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::pokemon::{GetNature, gPlayerParty};
use crate::sprite::FreeOamMatrix;
use crate::task::DestroyTask;
use crate::task::{task_get, task_set};
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CalcCenterToCornerVec` with this module's view of its types.
#[inline]
unsafe fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8) {
    unsafe {
        crate::sprite::CalcCenterToCornerVec(a0 as _, a1, a2, a3);
    }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `InitSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn InitSpriteAffineAnim(a0: *mut Sprite) {
    unsafe {
        crate::sprite::InitSpriteAffineAnim(a0 as _);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sDontFlip: usize = 1;
const tPtrHi: usize = 1;
const tPtrLo: usize = 2;
const tAnimId: usize = 3;
const tBattlerId: usize = 4;
const tSpeciesId: usize = 5;
// Data tables (translate with cdata.py): sSpeciesToBackAnimSet sYellowFlashData sVerticalShakeData sMonAnimFunctions sBackAnimationIds sBackAnimNatureModTable sMonAffineAnim_0 sMonAffineAnim_1 sMonAffineAnims sZigzagData sBounceRotateToSidesData sTriangleDownData sShakeYellowFlashData_Fast sShakeYellowFlashData_Normal sShakeYellowFlashData_Slow sShakeYellowFlashData sColors.0

/// `struct PokemonAnimData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PokemonAnimData {
    pub delay: u16,
    pub speed: i16,
    pub runs: i16,
    pub rotation: i16,
    pub data: i16,
}

unsafe impl Sync for PokemonAnimData {}

/// `struct YellowFlashData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct YellowFlashData {
    pub isYellow: u8,
    pub time: u8,
}

unsafe impl Sync for YellowFlashData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokemonAnimData>() == 12);
    assert!(offset_of!(PokemonAnimData, delay) == 0);
    assert!(offset_of!(PokemonAnimData, speed) == 2);
    assert!(offset_of!(PokemonAnimData, runs) == 4);
    assert!(offset_of!(PokemonAnimData, rotation) == 6);
    assert!(offset_of!(PokemonAnimData, data) == 8);
    assert!(size_of::<YellowFlashData>() == 4);
    assert!(offset_of!(YellowFlashData, isYellow) == 0);
    assert!(offset_of!(YellowFlashData, time) == 1);
};

const SHAKEGLOW_BLUE: i16 = 2;
const SHAKEGLOW_GREEN: i16 = 1;
const SHAKEGLOW_RED: i16 = 0;

static sBackAnimNatureModTable: Table<CArray<u8, 25>> =
    Table((&raw const crate::data::pokemon_animation::sBackAnimNatureModTable).cast());
static sBackAnimationIds: Table<CArray<u8, 75>> =
    Table((&raw const crate::data::pokemon_animation::sBackAnimationIds).cast());
static sBounceRotateToSidesData: Table<CArray<CArray<CArray<i8, 3>, 8>, 2>> =
    Table((&raw const crate::data::pokemon_animation::sBounceRotateToSidesData).cast());
static sColors_0: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::pokemon_animation::sColors_0).cast());
static sMonAffineAnims: Table<CArray<*mut AffineAnimCmd, 2>> =
    Table((&raw const crate::data::pokemon_animation::sMonAffineAnims).cast());
static sMonAnimFunctions: Table<CArray<Option<unsafe fn(*mut Sprite)>, 151>> =
    Table((&raw const crate::data::pokemon_animation::sMonAnimFunctions).cast());
static sShakeYellowFlashData: Table<CArray<*mut YellowFlashData, 3>> =
    Table((&raw const crate::data::pokemon_animation::sShakeYellowFlashData).cast());
static sSpeciesToBackAnimSet: Table<CArray<u8, 412>> =
    Table((&raw const crate::data::pokemon_animation::sSpeciesToBackAnimSet).cast());
static sTriangleDownData: Table<CArray<CArray<i8, 3>, 4>> =
    Table((&raw const crate::data::pokemon_animation::sTriangleDownData).cast());
static sVerticalShakeData: Table<CArray<CArray<u8, 2>, 4>> =
    Table((&raw const crate::data::pokemon_animation::sVerticalShakeData).cast());
static sYellowFlashData: Table<CArray<CArray<u8, 2>, 14>> =
    Table((&raw const crate::data::pokemon_animation::sYellowFlashData).cast());
static sZigzagData: Table<CArray<CArray<i8, 3>, 10>> =
    Table((&raw const crate::data::pokemon_animation::sZigzagData).cast());

pub(crate) static mut sAnims: CArray<PokemonAnimData, 4> = unsafe { zeroed() };
pub(crate) static sAnimIdx: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sIsSummaryAnim: crate::global::Global<u32> = crate::global::Global::new(0);

/// `ObjAffineSet` with this module's view of its types.
#[inline]
unsafe fn ObjAffineSet(a0: *mut ObjAffineSrcData, a1: *mut c_void, a2: i32, a3: i32) {
    unsafe {
        crate::syscall::ObjAffineSet(a0 as _, a1 as _, a2, a3);
    }
}

pub(crate) unsafe fn MonAnimDummySpriteCallback(sprite: *mut Sprite) {}
unsafe fn SetPosForRotation(
    sprite: *mut Sprite,
    index: u16,
    mut amplitudeX: i16,
    mut amplitudeY: i16,
) {
    amplitudeX *= -1;
    amplitudeY *= -1;
    let xAdder: i16 = Cos(index as i16, amplitudeX) - Sin(index as i16, amplitudeY);
    let yAdder: i16 = Cos(index as i16, amplitudeY) + Sin(index as i16, amplitudeX);
    amplitudeX *= -1;
    amplitudeY *= -1;
    (*sprite).x2 = xAdder + amplitudeX;
    (*sprite).y2 = yAdder + amplitudeY;
}
pub fn GetSpeciesBackAnimSet(species: u16) -> u8 {
    if sSpeciesToBackAnimSet[species] != BACK_ANIM_NONE {
        return sSpeciesToBackAnimSet[species] - 1;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Task_HandleMonAnimation(taskId: u8) {
    let sprite: *mut Sprite = ((task_get(taskId, 1) as i32) << 16
        | task_get(taskId, 2) as u16 as i32) as usize as *mut Sprite;
    if task_get(taskId, 0) == 0 {
        task_set(taskId, tBattlerId, (*sprite).data[0]);
        task_set(taskId, tSpeciesId, (*sprite).data[2]);
        (*sprite).data[1] = TRUE as i16;
        (*sprite).data[0] = 0;
        for i in 2..8u32 {
            (*sprite).data[i] = 0;
        }
        (*sprite).callback = sMonAnimFunctions[task_get(taskId, tAnimId)];
        sIsSummaryAnim.set(FALSE as u32);
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    }
    if (*sprite).callback == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite)) {
        (*sprite).data[0] = task_get(taskId, tBattlerId);
        (*sprite).data[2] = task_get(taskId, tSpeciesId);
        (*sprite).data[1] = 0;
        DestroyTask(taskId);
    }
}
pub unsafe fn LaunchAnimationTaskForFrontSprite(sprite: *mut Sprite, frontAnimId: u8) {
    let taskId: u8 = CreateTask(Some(Task_HandleMonAnimation), 128);
    task_set(taskId, tPtrHi, (sprite as usize as u32 >> 16) as i16);
    task_set(taskId, tPtrLo, sprite as usize as u32 as i16);
    task_set(taskId, tAnimId, frontAnimId as i16);
}
pub unsafe fn StartMonSummaryAnimation(sprite: *mut Sprite, frontAnimId: u8) {
    sIsSummaryAnim.set(TRUE as u32);
    (*sprite).callback = sMonAnimFunctions[frontAnimId];
}
pub unsafe fn LaunchAnimationTaskForBackSprite(sprite: *mut Sprite, backAnimSet: u8) {
    let taskId: u8 = CreateTask(Some(Task_HandleMonAnimation), 128);
    task_set(taskId, tPtrHi, (sprite as usize as u32 >> 16) as i16);
    task_set(taskId, tPtrLo, sprite as usize as u32 as i16);
    let battler: u8 = (*sprite).data[0] as u8;
    let nature: u8 = GetNature(&raw mut gPlayerParty[gBattlerPartyIndexes[battler]]);
    let animId: u8 = 3 * backAnimSet + sBackAnimNatureModTable[nature];
    task_set(taskId, tAnimId, sBackAnimationIds[animId] as i16);
}
pub unsafe fn SetSpriteCB_MonAnimDummy(sprite: *mut Sprite) {
    (*sprite).callback = Some(MonAnimDummySpriteCallback);
}
unsafe fn SetAffineData(sprite: *mut Sprite, xScale: i16, yScale: i16, rotation: u16) {
    let mut matrixNum: u8 = 0;
    let mut affineSrcData: ObjAffineSrcData = zeroed();
    let mut dest: OamMatrix = zeroed();
    affineSrcData.xScale = xScale;
    affineSrcData.yScale = yScale;
    affineSrcData.rotation = rotation;
    matrixNum = (*sprite).oam.matrixNum() as u8;
    ObjAffineSet(&raw mut affineSrcData, &raw mut dest as *mut c_void, 1, 2);
    (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[matrixNum]
        .a = dest.a;
    (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[matrixNum]
        .b = dest.b;
    (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[matrixNum]
        .c = dest.c;
    (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[matrixNum]
        .d = dest.d;
}
unsafe fn HandleStartAffineAnim(sprite: *mut Sprite) {
    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    (*sprite).affineAnims = sMonAffineAnims.as_ptr().cast_mut();
    if sIsSummaryAnim.get() == TRUE as u32 {
        InitSpriteAffineAnim(sprite);
    }
    if (*sprite).data[sDontFlip] == 0 {
        StartSpriteAffineAnim(sprite, 1);
    } else {
        StartSpriteAffineAnim(sprite, 0);
    }
    CalcCenterToCornerVec(
        sprite,
        (*sprite).oam.shape() as u8,
        (*sprite).oam.size() as u8,
        (*sprite).oam.affineMode() as u8,
    );
    (*sprite).set_affineAnimPaused(TRUE);
}
unsafe fn HandleSetAffineData(
    sprite: *mut Sprite,
    mut xScale: i16,
    yScale: i16,
    mut rotation: u16,
) {
    if (*sprite).data[sDontFlip] == 0 {
        xScale *= -1;
        rotation *= 65535;
    }
    SetAffineData(sprite, xScale, yScale, rotation);
}
unsafe fn TryFlipX(sprite: *mut Sprite) {
    if (*sprite).data[sDontFlip] == 0 {
        (*sprite).x2 *= -1;
    }
}
unsafe fn InitAnimData(id: u8) -> u32 {
    if id >= MAX_BATTLERS_COUNT {
        return FALSE as u32;
    } else {
        sAnims[id].rotation = 0;
        sAnims[id].delay = 0;
        sAnims[id].runs = 1;
        sAnims[id].speed = 0;
        sAnims[id].data = 0;
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn AddNewAnim() -> u8 {
    sAnimIdx.set(((sAnimIdx.get() as i32 + 1) % 4) as u8);
    InitAnimData(sAnimIdx.get());
    sAnimIdx.get()
}
unsafe fn ResetSpriteAfterAnim(sprite: *mut Sprite) {
    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    CalcCenterToCornerVec(
        sprite,
        (*sprite).oam.shape() as u8,
        (*sprite).oam.size() as u8,
        (*sprite).oam.affineMode() as u8,
    );
    if sIsSummaryAnim.get() == TRUE as u32 {
        if (*sprite).data[sDontFlip] == 0 {
            (*sprite).set_hFlip(TRUE as u16);
        } else {
            (*sprite).set_hFlip(FALSE as u16);
        }
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        (*sprite)
            .oam
            .set_matrixNum((*sprite).oam.matrixNum() | ((*sprite).hFlip() as u32) << 3);
        (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
    }
}
pub(crate) unsafe fn Anim_CircularStretchTwice(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] > 40 {
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        let var: i16 = ((*sprite).data[2] as i32 * 512 / 40 % 256) as i16;
        (*sprite).data[4] = Sin(var, 32) + 256;
        (*sprite).data[5] = Cos(var, 32) + 256;
        HandleSetAffineData(sprite, (*sprite).data[4], (*sprite).data[5], 0);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_HorizontalVibrate(sprite: *mut Sprite) {
    if (*sprite).data[2] > 40 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
    } else {
        let mut sign: i8 = 0;
        if (*sprite).data[2] as i32 & 1 == 0 {
            sign = 1;
        } else {
            sign = -1;
        }
        (*sprite).x2 = Sin(((*sprite).data[2] as i32 * 128 / 40 % 256) as i16, 6) * sign as i16;
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn HorizontalSlide(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] > (*sprite).data[0] {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
    } else {
        (*sprite).x2 = Sin(
            (div_i32((*sprite).data[2] as i32 * 384, (*sprite).data[0] as i32) % 256) as i16,
            6,
        );
    }
    (*sprite).data[2] += 1;
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_HorizontalSlide(sprite: *mut Sprite) {
    (*sprite).data[0] = 40;
    HorizontalSlide(sprite);
    (*sprite).callback = Some(HorizontalSlide);
}
pub(crate) unsafe fn VerticalSlide(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] > (*sprite).data[0] {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).y2 = 0;
    } else {
        (*sprite).y2 = -Sin(
            (div_i32((*sprite).data[2] as i32 * 384, (*sprite).data[0] as i32) % 256) as i16,
            6,
        );
    }
    (*sprite).data[2] += 1;
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_VerticalSlide(sprite: *mut Sprite) {
    (*sprite).data[0] = 40;
    VerticalSlide(sprite);
    (*sprite).callback = Some(VerticalSlide);
}
pub(crate) unsafe fn VerticalJumps(sprite: *mut Sprite) {
    let mut counter: i32 = (*sprite).data[2] as i32;
    if counter > 384 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
    } else {
        let divCounter: i16 = (counter / 128) as i16;
        match divCounter {
            0 | 1 => {
                (*sprite).y2 = -Sin((counter % 128) as i16, (*sprite).data[0] * 2);
            }
            2 | 3 => {
                counter -= 256;
                (*sprite).y2 = -Sin(counter as i16, (*sprite).data[0] * 3);
            }
            _ => {}
        }
    }
    (*sprite).data[2] += 12;
}
pub(crate) unsafe fn Anim_VerticalJumps_Big(sprite: *mut Sprite) {
    (*sprite).data[0] = 4;
    VerticalJumps(sprite);
    (*sprite).callback = Some(VerticalJumps);
}
pub(crate) unsafe fn Anim_VerticalJumpsHorizontalJumps(sprite: *mut Sprite) {
    let mut counter: i32 = (*sprite).data[2] as i32;
    if counter > 768 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
    } else {
        let divCounter: i16 = (counter / 128) as i16;
        match divCounter {
            0 | 1 => {
                (*sprite).x2 = 0;
            }
            2 => {
                counter = 0;
            }
            3 => {
                (*sprite).x2 = (-(counter % 128 * 8) / 128) as i16;
            }
            4 => {
                (*sprite).x2 = (counter % 128 / 8) as i16 - 8;
            }
            5 => {
                (*sprite).x2 = (-(counter % 128 * 8) / 128) as i16 + 8;
            }
            _ => {}
        }
        (*sprite).y2 = -Sin((counter % 128) as i16, 8);
    }
    (*sprite).data[2] += 12;
}
pub(crate) unsafe fn Anim_GrowVibrate(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] > 40 {
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        let index: i16 = ((*sprite).data[2] as i32 * 256 / 40 % 256) as i16;
        if (*sprite).data[2] % 2 == 0 {
            (*sprite).data[4] = Sin(index, 32) + 256;
            (*sprite).data[5] = Sin(index, 32) + 256;
        } else {
            (*sprite).data[4] = Sin(index, 8) + 256;
            (*sprite).data[5] = Sin(index, 8) + 256;
        }
        HandleSetAffineData(sprite, (*sprite).data[4], (*sprite).data[5], 0);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Zigzag(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] == 0 {
        (*sprite).data[3] = 0;
    }
    if sZigzagData[(*sprite).data[3]][2] as i16 == (*sprite).data[2] {
        if sZigzagData[(*sprite).data[3]][2] == 0 {
            (*sprite).callback = Some(WaitAnimEnd);
        } else {
            (*sprite).data[3] += 1;
            (*sprite).data[2] = 0;
        }
    }
    if sZigzagData[(*sprite).data[3]][2] == 0 {
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).x2 += sZigzagData[(*sprite).data[3]][0] as i16;
        (*sprite).y2 += sZigzagData[(*sprite).data[3]][1] as i16;
        (*sprite).data[2] += 1;
        TryFlipX(sprite);
    }
}
pub(crate) unsafe fn Anim_ZigzagFast(sprite: *mut Sprite) {
    Zigzag(sprite);
    (*sprite).callback = Some(Zigzag);
}
pub(crate) unsafe fn HorizontalShake(sprite: *mut Sprite) {
    let counter: i32 = (*sprite).data[2] as i32;
    if counter > 2304 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
    } else {
        (*sprite).x2 = Sin((counter % 256) as i16, (*sprite).data[7]);
    }
    (*sprite).data[2] += (*sprite).data[0];
}
pub(crate) unsafe fn Anim_HorizontalShake(sprite: *mut Sprite) {
    (*sprite).data[0] = 60;
    (*sprite).data[7] = 3;
    HorizontalShake(sprite);
    (*sprite).callback = Some(HorizontalShake);
}
pub(crate) unsafe fn VerticalShake(sprite: *mut Sprite) {
    let counter: i32 = (*sprite).data[2] as i32;
    if counter > 2304 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).y2 = 0;
    } else {
        (*sprite).y2 = Sin((counter % 256) as i16, 3);
    }
    (*sprite).data[2] += (*sprite).data[0];
}
pub(crate) unsafe fn Anim_VerticalShake(sprite: *mut Sprite) {
    (*sprite).data[0] = 60;
    VerticalShake(sprite);
    (*sprite).callback = Some(VerticalShake);
}
pub(crate) unsafe fn Anim_CircularVibrate(sprite: *mut Sprite) {
    if (*sprite).data[2] > 512 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
    } else {
        let mut sign: i8 = 0;
        if (*sprite).data[2] as i32 & 1 == 0 {
            sign = 1;
        } else {
            sign = -1;
        }
        let amplitude: i32 = Sin((*sprite).data[2] / 4, 8) as i32;
        let index: i32 = ((*sprite).data[2] % 256) as i32;
        (*sprite).y2 = Sin(index as i16, amplitude as i16) * sign as i16;
        (*sprite).x2 = Cos(index as i16, amplitude as i16) * sign as i16;
    }
    (*sprite).data[2] += 9;
}
pub(crate) unsafe fn Twist(sprite: *mut Sprite) {
    let id: i16 = (*sprite).data[0];
    if sAnims[id].delay != 0 {
        sAnims[id].delay -= 1;
    } else {
        if (*sprite).data[2] == 0 && sAnims[id].data == 0 {
            HandleStartAffineAnim(sprite);
            sAnims[id].data += 1;
        }
        if (*sprite).data[2] > sAnims[id].rotation {
            HandleSetAffineData(sprite, 256, 256, 0);
            if sAnims[id].runs > 1 {
                sAnims[id].runs -= 1;
                sAnims[id].delay = 10;
                (*sprite).data[2] = 0;
            } else {
                ResetSpriteAfterAnim(sprite);
                (*sprite).callback = Some(WaitAnimEnd);
            }
        } else {
            (*sprite).data[6] = Sin((*sprite).data[2] % 256, 4096);
            HandleSetAffineData(sprite, 256, 256, (*sprite).data[6] as u16);
        }
        (*sprite).data[2] += 16;
    }
}
pub(crate) unsafe fn Anim_Twist(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 512;
    sAnims[id].delay = 0;
    Twist(sprite);
    (*sprite).callback = Some(Twist);
}
pub(crate) unsafe fn Spin(sprite: *mut Sprite) {
    let id: u8 = (*sprite).data[0] as u8;
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] as i32 > sAnims[id].delay as i32 {
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[6] = div_i32(0x10000, sAnims[id].data as i32) as i16 * (*sprite).data[2];
        HandleSetAffineData(sprite, 256, 256, (*sprite).data[6] as u16);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_Spin_Long(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].delay = 60;
    sAnims[id].data = 20;
    Spin(sprite);
    (*sprite).callback = Some(Spin);
}
pub(crate) unsafe fn CircleCounterclockwise(sprite: *mut Sprite) {
    let id: u8 = (*sprite).data[0] as u8;
    TryFlipX(sprite);
    if (*sprite).data[2] > sAnims[id].rotation {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        let index: i16 = (((*sprite).data[2] as i32 + 192) % 256) as i16;
        (*sprite).x2 = -Cos(index, sAnims[id].data * 2);
        (*sprite).y2 = Sin(index, sAnims[id].data) + sAnims[id].data;
    }
    (*sprite).data[2] += sAnims[id].speed;
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_CircleCounterclockwise(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 512;
    sAnims[id].data = 6;
    sAnims[id].speed = 24;
    CircleCounterclockwise(sprite);
    (*sprite).callback = Some(CircleCounterclockwise);
}
pub(crate) unsafe fn Anim_GlowBlack(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
    }
    if (*sprite).data[2] > 128 {
        BlendPalette((*sprite).data[7] as u16, 16, 0, 0);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[6] = Sin((*sprite).data[2], 16);
        BlendPalette((*sprite).data[7] as u16, 16, (*sprite).data[6] as u8, 0);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_HorizontalStretch(sprite: *mut Sprite) {
    let mut index1: i16 = 0;
    let mut index2: i16 = 0;
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] > 40 {
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        index2 = ((*sprite).data[2] as i32 * 128 / 40) as i16;
        if (*sprite).data[2] >= 10 && (*sprite).data[2] <= 29 {
            (*sprite).data[7] += 51;
            index1 = 0xFF & (*sprite).data[7];
        }
        if (*sprite).data[sDontFlip] == 0 {
            (*sprite).data[4] = Sin(index2, 40) - 256 + Sin(index1, 16);
        } else {
            (*sprite).data[4] = 256 - Sin(index2, 40) - Sin(index1, 16);
        }
        (*sprite).data[5] = Sin(index2, 16) + 256;
        SetAffineData(sprite, (*sprite).data[4], (*sprite).data[5], 0);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_VerticalStretch(sprite: *mut Sprite) {
    let mut posY: i16 = 0;
    let mut index1: i16 = 0;
    let mut index2: i16 = 0;
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] > 40 {
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).y2 = posY;
    } else {
        index2 = ((*sprite).data[2] as i32 * 128 / 40) as i16;
        if (*sprite).data[2] >= 10 && (*sprite).data[2] <= 29 {
            (*sprite).data[7] += 51;
            index1 = 0xFF & (*sprite).data[7];
        }
        if (*sprite).data[sDontFlip] == 0 {
            (*sprite).data[4] = -Sin(index2, 16) - 256;
        } else {
            (*sprite).data[4] = Sin(index2, 16) + 256;
        }
        (*sprite).data[5] = 256 - Sin(index2, 40) - Sin(index1, 8);
        if (*sprite).data[5] != 256 {
            posY = ((256 - (*sprite).data[5] as i32) / 8) as i16;
        }
        (*sprite).y2 = -posY;
        SetAffineData(sprite, (*sprite).data[4], (*sprite).data[5], 0);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn VerticalShakeTwice(sprite: *mut Sprite) {
    let index: u8 = (*sprite).data[2] as u8;
    let var7: u8 = (*sprite).data[6] as u8;
    let var5: u8 = sVerticalShakeData[(*sprite).data[5]][0];
    let var6: u8 = sVerticalShakeData[(*sprite).data[5]][1];
    let mut amplitude: u8 = 0;
    if var5 != 254 {
        amplitude = div_i32((var6 as i32 - var7 as i32) * var5 as i32, var6 as i32) as u8;
    } else {
        amplitude = 0;
    }
    if var5 == 255 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).y2 = 0;
    } else {
        (*sprite).y2 = Sin(index as i16, amplitude as i16);
        if var7 == var6 {
            (*sprite).data[5] += 1;
            (*sprite).data[6] = 0;
        } else {
            (*sprite).data[2] += (*sprite).data[0];
            (*sprite).data[6] += 1;
        }
    }
}
pub(crate) unsafe fn Anim_VerticalShakeTwice(sprite: *mut Sprite) {
    (*sprite).data[0] = 48;
    VerticalShakeTwice(sprite);
    (*sprite).callback = Some(VerticalShakeTwice);
}
pub(crate) unsafe fn Anim_TipMoveForward(sprite: *mut Sprite) {
    TryFlipX(sprite);
    let counter: u8 = (*sprite).data[2] as u8;
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] > 35 {
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
    } else {
        let index: i16 = ((counter as i32 - 10) * 128 / 20) as i16;
        if counter < 10 {
            HandleSetAffineData(sprite, 256, 256, (counter as i32 / 2) as u16 * 512);
        } else if (10..=29).contains(&counter) {
            (*sprite).x2 = -Sin(index, 5);
        } else {
            HandleSetAffineData(sprite, 256, 256, ((35 - counter as i32) / 2) as u16 * 1024);
        }
    }
    (*sprite).data[2] += 1;
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_HorizontalPivot(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] > 100 {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).y2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        let index: i16 = ((*sprite).data[2] as i32 * 256 / 100) as i16;
        (*sprite).y2 = Sin(index, 10);
        HandleSetAffineData(sprite, 256, 256, Sin(index, 3276) as u16);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn VerticalSlideWobble(sprite: *mut Sprite) {
    let mut var: i32 = 0;
    let mut index: i16 = 0;
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] > 100 {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).y2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        index = ((*sprite).data[2] as i32 * 256 / 100) as i16;
        var = (*sprite).data[2] as i32 * 512 / 100;
        var &= 0xFF;
        (*sprite).y2 = Sin(index, (*sprite).data[0]);
        HandleSetAffineData(sprite, 256, 256, Sin(var as i16, 3276) as u16);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_VerticalSlideWobble(sprite: *mut Sprite) {
    (*sprite).data[0] = 10;
    VerticalSlideWobble(sprite);
    (*sprite).callback = Some(VerticalSlideWobble);
}
pub(crate) unsafe fn RisingWobble(sprite: *mut Sprite) {
    let mut var: i32 = 0;
    let mut index: i16 = 0;
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] > 100 {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).y2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        index = ((*sprite).data[2] as i32 * 256 / 100) as i16;
        var = (*sprite).data[2] as i32 * 512 / 100;
        var &= 0xFF;
        (*sprite).y2 = -Sin(index / 2, (*sprite).data[0] * 2);
        HandleSetAffineData(sprite, 256, 256, Sin(var as i16, 3276) as u16);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_RisingWobble(sprite: *mut Sprite) {
    (*sprite).data[0] = 5;
    RisingWobble(sprite);
    (*sprite).callback = Some(RisingWobble);
}
pub(crate) unsafe fn Anim_HorizontalSlideWobble(sprite: *mut Sprite) {
    let mut index: i16 = 0;
    TryFlipX(sprite);
    let mut var: i32 = 0;
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    if (*sprite).data[2] > 100 {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).x2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        index = ((*sprite).data[2] as i32 * 256 / 100) as i16;
        var = (*sprite).data[2] as i32 * 512 / 100;
        var &= 0xFF;
        (*sprite).x2 = Sin(index, 8);
        HandleSetAffineData(sprite, 256, 256, Sin(var as i16, 3276) as u16);
    }
    (*sprite).data[2] += 1;
    TryFlipX(sprite);
}
pub(crate) unsafe fn VerticalSquishBounce(sprite: *mut Sprite) {
    let mut posY: i16 = 0;
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[3] = 0;
    }
    TryFlipX(sprite);
    if (*sprite).data[2] as i32 > (*sprite).data[0] as i32 * 3 {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).y2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        let yScale: i16 = Sin((*sprite).data[4], 32) + 256;
        if (*sprite).data[2] > (*sprite).data[0]
            && ((*sprite).data[2] as i32) < (*sprite).data[0] as i32 * 2
        {
            (*sprite).data[3] += div_i32(128, (*sprite).data[0] as i32) as i16;
        }
        if yScale > 256 {
            posY = ((256 - yScale as i32) / 8) as i16;
        }
        (*sprite).y2 = -Sin((*sprite).data[3], 10) - posY;
        HandleSetAffineData(sprite, 256 - Sin((*sprite).data[4], 32), yScale, 0);
        (*sprite).data[2] += 1;
        (*sprite).data[4] =
            ((*sprite).data[4] + div_i32(128, (*sprite).data[0] as i32) as i16) & 0xFF;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_VerticalSquishBounce(sprite: *mut Sprite) {
    (*sprite).data[0] = 16;
    VerticalSquishBounce(sprite);
    (*sprite).callback = Some(VerticalSquishBounce);
}
unsafe fn ShrinkGrow(sprite: *mut Sprite) {
    let mut posY: i16 = 0;
    if (*sprite).data[2] as i32 > div_i32(128, (*sprite).data[6] as i32) * (*sprite).data[7] as i32
    {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).y2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        let yScale: i16 = Sin((*sprite).data[4], 32) + 256;
        if yScale > 256 {
            posY = ((256 - yScale as i32) / 8) as i16;
        }
        (*sprite).y2 = -posY;
        HandleSetAffineData(sprite, Sin((*sprite).data[4], 48) + 256, yScale, 0);
        (*sprite).data[2] += 1;
        (*sprite).data[4] = ((*sprite).data[4] + (*sprite).data[6]) & 0xFF;
    }
}
pub(crate) unsafe fn Anim_ShrinkGrow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[7] = 3;
        (*sprite).data[6] = 8;
    }
    ShrinkGrow(sprite);
}
pub(crate) unsafe fn BounceRotateToSides(sprite: *mut Sprite) {
    TryFlipX(sprite);
    let structId: u8 = (*sprite).data[0] as u8;
    let var: i16 = sAnims[structId].rotation;
    let r9: i8 = sBounceRotateToSidesData[sAnims[structId].data][(*sprite).data[4]][0];
    let r10: i16 =
        sBounceRotateToSidesData[sAnims[structId].data][(*sprite).data[4]][1] as i16 - r9 as i16;
    let arrId: u32 = sAnims[structId].data as u32;
    let r7: i16 = (*sprite).data[3];
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
    }
    if sBounceRotateToSidesData[arrId][(*sprite).data[4]][2] == 0 {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).y2 = -Sin(
            div_i32(
                r7 as i32 * 128,
                sBounceRotateToSidesData[arrId][(*sprite).data[4]][2] as i32,
            ) as i16,
            10,
        );
        (*sprite).x2 = div_i32(
            r10 as i32 * r7 as i32,
            sBounceRotateToSidesData[arrId][(*sprite).data[4]][2] as i32,
        ) as i16
            + r9 as i16;
        let rotation: u16 = (-(var as i32 * (*sprite).x2 as i32) / 8) as u16;
        HandleSetAffineData(sprite, 256, 256, rotation);
        if r7 == sBounceRotateToSidesData[arrId][(*sprite).data[4]][2] as i16 {
            (*sprite).data[4] += 1;
            (*sprite).data[3] = 0;
        } else {
            (*sprite).data[3] += 1;
        }
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_BounceRotateToSides(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 4096;
    sAnims[id].data = (*sprite).data[6];
    BounceRotateToSides(sprite);
    (*sprite).callback = Some(BounceRotateToSides);
}
pub(crate) unsafe fn Anim_GlowOrange(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
    }
    if (*sprite).data[2] > 128 {
        BlendPalette((*sprite).data[7] as u16, 16, 0, 735);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[6] = Sin((*sprite).data[2], 12);
        BlendPalette((*sprite).data[7] as u16, 16, (*sprite).data[6] as u8, 735);
    }
    (*sprite).data[2] += 2;
}
pub(crate) unsafe fn Anim_GlowRed(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
    }
    if (*sprite).data[2] > 128 {
        BlendPalette((*sprite).data[7] as u16, 16, 0, 31);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[6] = Sin((*sprite).data[2], 12);
        BlendPalette((*sprite).data[7] as u16, 16, (*sprite).data[6] as u8, 31);
    }
    (*sprite).data[2] += 2;
}
pub(crate) unsafe fn Anim_GlowBlue(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
    }
    if (*sprite).data[2] > 128 {
        BlendPalette((*sprite).data[7] as u16, 16, 0, 31744);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[6] = Sin((*sprite).data[2], 12);
        BlendPalette((*sprite).data[7] as u16, 16, (*sprite).data[6] as u8, 31744);
    }
    (*sprite).data[2] += 2;
}
pub(crate) unsafe fn Anim_GlowYellow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
    }
    if (*sprite).data[2] > 128 {
        BlendPalette((*sprite).data[7] as u16, 16, 0, 1023);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[6] = Sin((*sprite).data[2], 12);
        BlendPalette((*sprite).data[7] as u16, 16, (*sprite).data[6] as u8, 1023);
    }
    (*sprite).data[2] += 2;
}
pub(crate) unsafe fn Anim_GlowPurple(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
    }
    if (*sprite).data[2] > 128 {
        BlendPalette((*sprite).data[7] as u16, 16, 0, 24600);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[6] = Sin((*sprite).data[2], 12);
        BlendPalette((*sprite).data[7] as u16, 16, (*sprite).data[6] as u8, 24600);
    }
    (*sprite).data[2] += 2;
}
pub(crate) unsafe fn Anim_BackAndLunge(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).callback = Some(BackAndLunge_0);
}
pub(crate) unsafe fn BackAndLunge_0(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if ({
        (*sprite).x2 += 1;
        (*sprite).x2
    }) > 7
    {
        (*sprite).x2 = 8;
        (*sprite).data[7] = 2;
        (*sprite).callback = Some(BackAndLunge_1);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn BackAndLunge_1(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 -= (*sprite).data[7];
    (*sprite).data[7] += 1;
    if (*sprite).x2 <= 0 {
        let mut var: u8 = (*sprite).data[7] as u8;
        (*sprite).data[6] = 0;
        let mut subResult: i16 = (*sprite).x2;
        loop {
            subResult -= var as i16;
            (*sprite).data[6] += 1;
            var += 1;
            if subResult <= -8 {
                break;
            }
        }
        (*sprite).data[5] = 1;
        (*sprite).callback = Some(BackAndLunge_2);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn BackAndLunge_2(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 -= (*sprite).data[7];
    (*sprite).data[7] += 1;
    let rotation: u8 = div_i32((*sprite).data[5] as i32 * 6, (*sprite).data[6] as i32) as u8;
    if ({
        (*sprite).data[5] += 1;
        (*sprite).data[5]
    }) > (*sprite).data[6]
    {
        (*sprite).data[5] = (*sprite).data[6];
    }
    HandleSetAffineData(sprite, 256, 256, rotation as u16 * 256);
    if (*sprite).x2 < -8 {
        (*sprite).x2 = -8;
        (*sprite).data[4] = 2;
        (*sprite).data[3] = 0;
        (*sprite).data[2] = rotation as i16;
        (*sprite).callback = Some(BackAndLunge_3);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn BackAndLunge_3(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[3] > 11 {
        (*sprite).data[2] -= 2;
        if (*sprite).data[2] < 0 {
            (*sprite).data[2] = 0;
        }
        HandleSetAffineData(sprite, 256, 256, ((*sprite).data[2] as u16) << 8);
        if (*sprite).data[2] == 0 {
            (*sprite).callback = Some(BackAndLunge_4);
        }
    } else {
        (*sprite).x2 += (*sprite).data[4];
        (*sprite).data[4] *= -1;
        (*sprite).data[3] += 1;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn BackAndLunge_4(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 += 2;
    if (*sprite).x2 > 0 {
        (*sprite).x2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_BackFlip(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).data[3] = 0;
    (*sprite).callback = Some(BackFlip_0);
}
pub(crate) unsafe fn BackFlip_0(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 += 1;
    (*sprite).y2 -= 1;
    if (*sprite).x2 % 2 == 0 && (*sprite).data[3] <= 0 {
        (*sprite).data[3] = 10;
    }
    if (*sprite).x2 > 7 {
        (*sprite).x2 = 8;
        (*sprite).y2 = -8;
        (*sprite).data[4] = 0;
        (*sprite).callback = Some(BackFlip_1);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn BackFlip_1(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 = Cos((*sprite).data[4], 16) - 8;
    (*sprite).y2 = Sin((*sprite).data[4], 16) - 8;
    if (*sprite).data[4] > 63 {
        (*sprite).data[2] = 160;
        (*sprite).data[3] = 10;
        (*sprite).callback = Some(BackFlip_2);
    }
    (*sprite).data[4] += 8;
    if (*sprite).data[4] > 64 {
        (*sprite).data[4] = 64;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn BackFlip_2(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[3] > 0 {
        (*sprite).data[3] -= 1;
    } else {
        (*sprite).x2 = Cos((*sprite).data[2], 5) - 4;
        (*sprite).y2 = -Sin((*sprite).data[2], 5) + 4;
        (*sprite).data[2] -= 4;
        let rotation: u32 = (*sprite).data[2] as u32 - 32;
        HandleSetAffineData(sprite, 256, 256, rotation as u16 * 512);
        if (*sprite).data[2] <= 32 {
            (*sprite).x2 = 0;
            (*sprite).y2 = 0;
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
        }
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_Flicker(sprite: *mut Sprite) {
    if (*sprite).data[3] > 0 {
        (*sprite).data[3] -= 1;
    } else {
        (*sprite).data[4] = (if (*sprite).data[4] == 0 {
            TRUE as i32
        } else {
            0
        }) as i16;
        (*sprite).set_invisible((*sprite).data[4] as u16);
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) > 19
        {
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).callback = Some(WaitAnimEnd);
        }
        (*sprite).data[3] = 2;
    }
}
pub(crate) unsafe fn Anim_BackFlipBig(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).callback = Some(BackFlipBig_0);
}
pub(crate) unsafe fn BackFlipBig_0(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 -= 1;
    (*sprite).y2 += 1;
    if (*sprite).x2 <= -16 {
        (*sprite).x2 = -16;
        (*sprite).y2 = 16;
        (*sprite).callback = Some(BackFlipBig_1);
        (*sprite).data[2] = 160;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn BackFlipBig_1(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).data[2] -= 4;
    (*sprite).x2 = Cos((*sprite).data[2], 22);
    (*sprite).y2 = -Sin((*sprite).data[2], 22);
    let rotation: u32 = (*sprite).data[2] as u32 - 32;
    HandleSetAffineData(sprite, 256, 256, rotation as u16 * 512);
    if (*sprite).data[2] <= 32 {
        (*sprite).callback = Some(BackFlipBig_2);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn BackFlipBig_2(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 -= 1;
    (*sprite).y2 += 1;
    if (*sprite).x2 <= 0 {
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_FrontFlip(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).callback = Some(FrontFlip_0);
}
pub(crate) unsafe fn FrontFlip_0(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 += 1;
    (*sprite).y2 -= 1;
    if (*sprite).x2 > 15 {
        (*sprite).data[2] = 0;
        (*sprite).callback = Some(FrontFlip_1);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn FrontFlip_1(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).data[2] += 16;
    if (*sprite).x2 <= -16 {
        (*sprite).x2 = -16;
        (*sprite).y2 = 16;
        (*sprite).data[2] = 0;
        (*sprite).callback = Some(FrontFlip_2);
    } else {
        (*sprite).x2 -= 2;
        (*sprite).y2 += 2;
    }
    HandleSetAffineData(sprite, 256, 256, ((*sprite).data[2] as u16) << 8);
    TryFlipX(sprite);
}
pub(crate) unsafe fn FrontFlip_2(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 += 1;
    (*sprite).y2 -= 1;
    if (*sprite).x2 >= 0 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_TumblingFrontFlip(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].speed = 2;
    TumblingFrontFlip(sprite);
    (*sprite).callback = Some(TumblingFrontFlip);
}
pub(crate) unsafe fn TumblingFrontFlip(sprite: *mut Sprite) {
    if sAnims[(*sprite).data[0]].delay != 0 {
        sAnims[(*sprite).data[0]].delay -= 1;
    } else {
        TryFlipX(sprite);
        if (*sprite).data[2] == 0 {
            (*sprite).data[2] += 1;
            HandleStartAffineAnim(sprite);
            (*sprite).data[7] = sAnims[(*sprite).data[0]].speed;
            (*sprite).data[3] = -1;
            (*sprite).data[4] = -1;
            (*sprite).data[5] = 0;
            (*sprite).data[6] = 0;
        }
        (*sprite).x2 += (*sprite).data[7] * 2 * (*sprite).data[3];
        (*sprite).y2 += (*sprite).data[7] * (*sprite).data[4];
        (*sprite).data[6] += 8;
        if (*sprite).x2 <= -16 || (*sprite).x2 >= 16 {
            (*sprite).x2 = (*sprite).data[3] * 16;
            (*sprite).data[3] *= -1;
            (*sprite).data[5] += 1;
        } else if (*sprite).y2 <= -16 || (*sprite).y2 >= 16 {
            (*sprite).y2 = (*sprite).data[4] * 16;
            (*sprite).data[4] *= -1;
            (*sprite).data[5] += 1;
        }
        if (*sprite).data[5] > 5 && (*sprite).x2 <= 0 {
            (*sprite).x2 = 0;
            (*sprite).y2 = 0;
            if sAnims[(*sprite).data[0]].runs > 1 {
                sAnims[(*sprite).data[0]].runs -= 1;
                (*sprite).data[5] = 0;
                (*sprite).data[6] = 0;
                sAnims[(*sprite).data[0]].delay = 10;
            } else {
                ResetSpriteAfterAnim(sprite);
                (*sprite).callback = Some(WaitAnimEnd);
            }
        }
        HandleSetAffineData(sprite, 256, 256, ((*sprite).data[6] as u16) << 8);
        TryFlipX(sprite);
    }
}
pub(crate) unsafe fn Anim_Figure8(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).data[6] = 0;
    (*sprite).data[7] = 0;
    (*sprite).callback = Some(Figure8);
}
pub(crate) unsafe fn Figure8(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).data[6] += 4;
    (*sprite).x2 = -Sin((*sprite).data[6], 16);
    (*sprite).y2 = -Sin(((*sprite).data[6] * 2) & 0xFF, 8);
    if (*sprite).data[6] > 192 && (*sprite).data[7] == 1 {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).data[7] += 1;
    } else if (*sprite).data[6] > 64 && (*sprite).data[7] == 0 {
        HandleSetAffineData(sprite, -256, 256, 0);
        (*sprite).data[7] += 1;
    }
    if (*sprite).data[6] > 255 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_FlashYellow(sprite: *mut Sprite) {
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) == 1
    {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[6] = 0;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 0;
    }
    if sYellowFlashData[(*sprite).data[6]][1] == 255 {
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        if (*sprite).data[4] == 1 {
            if sYellowFlashData[(*sprite).data[6]][0] != 0 {
                BlendPalette((*sprite).data[7] as u16, 16, 16, 1023);
            } else {
                BlendPalette((*sprite).data[7] as u16, 16, 0, 1023);
            }
            (*sprite).data[4] = 0;
        }
        if sYellowFlashData[(*sprite).data[6]][1] as i16 == (*sprite).data[5] {
            (*sprite).data[4] = 1;
            (*sprite).data[5] = 0;
            (*sprite).data[6] += 1;
        } else {
            (*sprite).data[5] += 1;
        }
    }
}
pub(crate) unsafe fn SwingConcave(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    TryFlipX(sprite);
    if (*sprite).data[2] > sAnims[(*sprite).data[0]].data {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).x2 = 0;
        if sAnims[(*sprite).data[0]].runs > 1 {
            sAnims[(*sprite).data[0]].runs -= 1;
            (*sprite).data[2] = 0;
        } else {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
        }
    } else {
        let index: i16 = div_i32(
            (*sprite).data[2] as i32 * 256,
            sAnims[(*sprite).data[0]].data as i32,
        ) as i16;
        (*sprite).x2 = -Sin(index, 10);
        HandleSetAffineData(sprite, 256, 256, Sin(index, 3276) as u16);
    }
    (*sprite).data[2] += 1;
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_SwingConcave_FastShort(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].data = 50;
    SwingConcave(sprite);
    (*sprite).callback = Some(SwingConcave);
}
pub(crate) unsafe fn SwingConvex(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
    }
    TryFlipX(sprite);
    if (*sprite).data[2] > sAnims[(*sprite).data[0]].data {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).x2 = 0;
        if sAnims[(*sprite).data[0]].runs > 1 {
            sAnims[(*sprite).data[0]].runs -= 1;
            (*sprite).data[2] = 0;
        } else {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
        }
    } else {
        let index: i16 = div_i32(
            (*sprite).data[2] as i32 * 256,
            sAnims[(*sprite).data[0]].data as i32,
        ) as i16;
        (*sprite).x2 = -Sin(index, 10);
        HandleSetAffineData(sprite, 256, 256, (Sin(index, 3276) as u16).wrapping_neg());
    }
    (*sprite).data[2] += 1;
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_SwingConvex_FastShort(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].data = 50;
    SwingConvex(sprite);
    (*sprite).callback = Some(SwingConvex);
}
pub(crate) unsafe fn Anim_RotateUpSlamDown(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).data[6] = -((14 * (*sprite).centerToCornerVecX as i32 / 10) as i16);
    (*sprite).data[7] = 128;
    (*sprite).callback = Some(RotateUpSlamDown_0);
}
pub(crate) unsafe fn RotateUpSlamDown_0(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).data[7] -= 1;
    (*sprite).x2 = (*sprite).data[6] + Cos((*sprite).data[7], (*sprite).data[6]);
    (*sprite).y2 = -Sin((*sprite).data[7], (*sprite).data[6]);
    HandleSetAffineData(sprite, 256, 256, ((*sprite).data[7] as u16 - 128) << 8);
    if (*sprite).data[7] <= 120 {
        (*sprite).data[7] = 120;
        (*sprite).data[3] = 0;
        (*sprite).callback = Some(RotateUpSlamDown_1);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn RotateUpSlamDown_1(sprite: *mut Sprite) {
    if (*sprite).data[3] == 20 {
        (*sprite).callback = Some(RotateUpSlamDown_2);
        (*sprite).data[3] = 0;
    }
    (*sprite).data[3] += 1;
}
pub(crate) unsafe fn RotateUpSlamDown_2(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).data[7] += 2;
    (*sprite).x2 = (*sprite).data[6] + Cos((*sprite).data[7], (*sprite).data[6]);
    (*sprite).y2 = -Sin((*sprite).data[7], (*sprite).data[6]);
    HandleSetAffineData(sprite, 256, 256, ((*sprite).data[7] as u16 - 128) << 8);
    if (*sprite).data[7] >= 128 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).data[2] = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(Anim_VerticalShake);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn DeepVerticalSquishBounce(sprite: *mut Sprite) {
    if sAnims[(*sprite).data[0]].delay != 0 {
        sAnims[(*sprite).data[0]].delay -= 1;
    } else {
        if (*sprite).data[2] == 0 {
            HandleStartAffineAnim(sprite);
            (*sprite).data[4] = 0;
            (*sprite).data[5] = 0;
            (*sprite).data[2] = 1;
        }
        if (*sprite).data[5] == 0 {
            (*sprite).data[7] = Sin((*sprite).data[4], 256);
            (*sprite).y2 = Sin((*sprite).data[4], 16);
            (*sprite).data[6] = Sin((*sprite).data[4], 32);
            HandleSetAffineData(sprite, 256 - (*sprite).data[6], 256 + (*sprite).data[7], 0);
            if (*sprite).data[4] == 128 {
                (*sprite).data[4] = 0;
                (*sprite).data[5] = 1;
            }
        } else if (*sprite).data[5] == 1 {
            (*sprite).data[7] = Sin((*sprite).data[4], 32);
            (*sprite).y2 = -Sin((*sprite).data[4], 8);
            (*sprite).data[6] = Sin((*sprite).data[4], 128);
            HandleSetAffineData(sprite, 256 + (*sprite).data[6], 256 - (*sprite).data[7], 0);
            if (*sprite).data[4] == 128 {
                if sAnims[(*sprite).data[0]].runs > 1 {
                    sAnims[(*sprite).data[0]].runs -= 1;
                    sAnims[(*sprite).data[0]].delay = 10;
                    (*sprite).data[4] = 0;
                    (*sprite).data[5] = 0;
                } else {
                    HandleSetAffineData(sprite, 256, 256, 0);
                    ResetSpriteAfterAnim(sprite);
                    (*sprite).callback = Some(WaitAnimEnd);
                }
            }
        }
        (*sprite).data[4] += sAnims[(*sprite).data[0]].rotation;
    }
}
pub(crate) unsafe fn Anim_DeepVerticalSquishBounce(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 4;
    DeepVerticalSquishBounce(sprite);
    (*sprite).callback = Some(DeepVerticalSquishBounce);
}
pub(crate) unsafe fn Anim_HorizontalJumps(sprite: *mut Sprite) {
    let counter: i32 = (*sprite).data[2] as i32;
    TryFlipX(sprite);
    if counter > 512 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
    } else {
        match (*sprite).data[2] / 128 {
            0 => {
                (*sprite).x2 = (-(counter % 128 * 8) / 128) as i16;
            }
            1 => {
                (*sprite).x2 = (counter % 128 / 16) as i16 - 8;
            }
            2 => {
                (*sprite).x2 = (counter % 128 / 16) as i16;
            }
            3 => {
                (*sprite).x2 = (-(counter % 128 * 8) / 128) as i16 + 8;
            }
            _ => {}
        }
        (*sprite).y2 = -Sin((counter % 128) as i16, 8);
    }
    (*sprite).data[2] += 12;
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_HorizontalJumpsVerticalStretch(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].data = -1;
    HandleStartAffineAnim(sprite);
    (*sprite).data[3] = 0;
    HorizontalJumpsVerticalStretch_0(sprite);
    (*sprite).callback = Some(HorizontalJumpsVerticalStretch_0);
}
pub(crate) unsafe fn HorizontalJumpsVerticalStretch_0(sprite: *mut Sprite) {
    if sAnims[(*sprite).data[0]].delay != 0 {
        sAnims[(*sprite).data[0]].delay -= 1;
    } else {
        TryFlipX(sprite);
        let counter: i32 = (*sprite).data[2] as i32;
        if (*sprite).data[2] > 128 {
            (*sprite).data[2] = 0;
            (*sprite).callback = Some(HorizontalJumpsVerticalStretch_1);
        } else {
            let var: i32 = 8 * sAnims[(*sprite).data[0]].data as i32;
            (*sprite).x2 = (var * (counter % 128) / 128) as i16;
            (*sprite).y2 = -Sin((counter % 128) as i16, 8);
            (*sprite).data[2] += 12;
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe fn HorizontalJumpsVerticalStretch_1(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] > 48 {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).y2 = 0;
        (*sprite).data[2] = 0;
        (*sprite).callback = Some(HorizontalJumpsVerticalStretch_2);
    } else {
        let yScale: i16 = Sin((*sprite).data[4], 64) + 256;
        if (*sprite).data[2] >= 16 && (*sprite).data[2] <= 31 {
            (*sprite).data[3] += 8;
            (*sprite).x2 -= sAnims[(*sprite).data[0]].data;
        }
        let mut yDelta: i16 = 0;
        if yScale > 256 {
            yDelta = ((256 - yScale as i32) / 8) as i16;
        }
        (*sprite).y2 = -Sin((*sprite).data[3], 20) - yDelta;
        HandleSetAffineData(sprite, 256 - Sin((*sprite).data[4], 32), yScale, 0);
        (*sprite).data[2] += 1;
        (*sprite).data[4] += 8;
        (*sprite).data[4] &= 0xFF;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn HorizontalJumpsVerticalStretch_2(sprite: *mut Sprite) {
    TryFlipX(sprite);
    let counter: i32 = (*sprite).data[2] as i32;
    if counter > 128 {
        if sAnims[(*sprite).data[0]].runs > 1 {
            sAnims[(*sprite).data[0]].runs -= 1;
            sAnims[(*sprite).data[0]].delay = 10;
            (*sprite).data[3] = 0;
            (*sprite).data[2] = 0;
            (*sprite).data[4] = 0;
            (*sprite).callback = Some(HorizontalJumpsVerticalStretch_0);
        } else {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
        }
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
    } else {
        let var: i32 = sAnims[(*sprite).data[0]].data as i32;
        (*sprite).x2 = (var * (counter % 128 * 8) / 128) as i16 + 8 * -(var as i16);
        (*sprite).y2 = -Sin((counter % 128) as i16, 8);
    }
    (*sprite).data[2] += 12;
    TryFlipX(sprite);
}
pub(crate) unsafe fn RotateToSides(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
    }
    TryFlipX(sprite);
    if (*sprite).data[7] > 254 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        if sAnims[(*sprite).data[0]].runs > 1 {
            sAnims[(*sprite).data[0]].runs -= 1;
            (*sprite).data[2] = 0;
            (*sprite).data[7] = 0;
        } else {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
        }
        TryFlipX(sprite);
    } else {
        (*sprite).x2 = -Sin((*sprite).data[7], 16);
        let rotation: u16 = Sin((*sprite).data[7], 32) as u16;
        HandleSetAffineData(sprite, 256, 256, rotation << 8);
        (*sprite).data[7] += sAnims[(*sprite).data[0]].rotation;
        TryFlipX(sprite);
    }
}
pub(crate) unsafe fn Anim_RotateToSides_Fast(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 4;
    RotateToSides(sprite);
    (*sprite).callback = Some(RotateToSides);
}
pub(crate) unsafe fn Anim_RotateUpToSides(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
    }
    TryFlipX(sprite);
    if (*sprite).data[7] > 254 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
        TryFlipX(sprite);
    } else {
        (*sprite).x2 = -Sin((*sprite).data[7], 16);
        (*sprite).y2 = -Sin((*sprite).data[7] % 128, 16);
        let rotation: u16 = Sin((*sprite).data[7], 32) as u16;
        HandleSetAffineData(sprite, 256, 256, rotation << 8);
        (*sprite).data[7] += 8;
        TryFlipX(sprite);
    }
}
pub(crate) unsafe fn Anim_FlickerIncreasing(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0;
    }
    if (*sprite).data[2] == (*sprite).data[7] {
        (*sprite).data[7] = 0;
        (*sprite).data[2] += 1;
        (*sprite).set_invisible(FALSE as u16);
    } else {
        (*sprite).data[7] += 1;
        (*sprite).set_invisible(TRUE as u16);
    }
    if (*sprite).data[2] > 10 {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).callback = Some(WaitAnimEnd);
    }
}
pub(crate) unsafe fn Anim_TipHopForward(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).data[7] = 0;
    (*sprite).callback = Some(TipHopForward_0);
}
pub(crate) unsafe fn TipHopForward_0(sprite: *mut Sprite) {
    if (*sprite).data[7] > 31 {
        (*sprite).data[7] = 32;
        (*sprite).data[2] = 0;
        (*sprite).callback = Some(TipHopForward_1);
    } else {
        (*sprite).data[7] += 4;
    }
    HandleSetAffineData(sprite, 256, 256, ((*sprite).data[7] as u16) << 8);
}
pub(crate) unsafe fn TipHopForward_1(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] > 512 {
        (*sprite).callback = Some(TipHopForward_2);
        (*sprite).data[6] = 0;
    } else {
        (*sprite).x2 = (-((*sprite).data[2] as i32 * 16) / 512) as i16;
        (*sprite).y2 = -Sin((*sprite).data[2] % 128, 4);
        (*sprite).data[2] += 12;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn TipHopForward_2(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).data[7] -= 2;
    if (*sprite).data[7] < 0 {
        (*sprite).data[7] = 0;
        (*sprite).x2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).x2 = -Sin((*sprite).data[7] * 2, 16);
    }
    HandleSetAffineData(sprite, 256, 256, ((*sprite).data[7] as u16) << 8);
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_PivotShake(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
    }
    TryFlipX(sprite);
    if (*sprite).data[7] > 255 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).data[7] = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[7] += 16;
        (*sprite).x2 = -Sin((*sprite).data[7] % 128, 8);
        (*sprite).y2 = -Sin((*sprite).data[7] % 128, 8);
    }
    let rotation: u16 = Sin((*sprite).data[7] % 128, 16) as u16;
    HandleSetAffineData(sprite, 256, 256, rotation << 8);
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_TipAndShake(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).data[7] = 0;
    (*sprite).data[4] = 0;
    (*sprite).callback = Some(TipAndShake_0);
}
pub(crate) unsafe fn TipAndShake_0(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[7] > 24 {
        if ({
            (*sprite).data[4] += 1;
            (*sprite).data[4]
        }) > 4
        {
            (*sprite).data[4] = 0;
            (*sprite).callback = Some(TipAndShake_1);
        }
    } else {
        (*sprite).data[7] += 2;
        (*sprite).x2 = Sin((*sprite).data[7], 8);
        (*sprite).y2 = -Sin((*sprite).data[7], 8);
    }
    HandleSetAffineData(
        sprite,
        256,
        256,
        ((*sprite).data[7] as u16).wrapping_neg() << 8,
    );
    TryFlipX(sprite);
}
pub(crate) unsafe fn TipAndShake_1(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[7] > 32 {
        (*sprite).data[6] = 1;
        (*sprite).callback = Some(TipAndShake_2);
    } else {
        (*sprite).data[7] += 2;
        (*sprite).x2 = Sin((*sprite).data[7], 8);
        (*sprite).y2 = -Sin((*sprite).data[7], 8);
    }
    HandleSetAffineData(
        sprite,
        256,
        256,
        ((*sprite).data[7] as u16).wrapping_neg() << 8,
    );
    TryFlipX(sprite);
}
pub(crate) unsafe fn TipAndShake_2(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).data[7] += (*sprite).data[6] * 4;
    if (*sprite).data[5] > 9 {
        (*sprite).data[7] = 32;
        (*sprite).callback = Some(TipAndShake_3);
    }
    (*sprite).x2 = Sin((*sprite).data[7], 8);
    (*sprite).y2 = -Sin((*sprite).data[7], 8);
    if (*sprite).data[7] <= 28 || (*sprite).data[7] >= 36 {
        (*sprite).data[6] *= -1;
        (*sprite).data[5] += 1;
    }
    HandleSetAffineData(
        sprite,
        256,
        256,
        ((*sprite).data[7] as u16).wrapping_neg() << 8,
    );
    TryFlipX(sprite);
}
pub(crate) unsafe fn TipAndShake_3(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[7] <= 0 {
        (*sprite).data[7] = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[7] -= 2;
        (*sprite).x2 = Sin((*sprite).data[7], 8);
        (*sprite).y2 = -Sin((*sprite).data[7], 8);
    }
    HandleSetAffineData(
        sprite,
        256,
        256,
        ((*sprite).data[7] as u16).wrapping_neg() << 8,
    );
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_VibrateToCorners(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] > 40 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
    } else {
        let mut sign: i8 = 0;
        if (*sprite).data[2] as i32 & 1 == 0 {
            sign = 1;
        } else {
            sign = -1;
        }
        if (*sprite).data[2] % 4 / 2 == 0 {
            (*sprite).x2 =
                Sin(((*sprite).data[2] as i32 * 128 / 40 % 256) as i16, 16) * sign as i16;
            (*sprite).y2 = -(*sprite).x2;
        } else {
            (*sprite).x2 =
                -Sin(((*sprite).data[2] as i32 * 128 / 40 % 256) as i16, 16) * sign as i16;
            (*sprite).y2 = (*sprite).x2;
        }
    }
    (*sprite).data[2] += 1;
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_GrowInStages(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[5] = 0;
        (*sprite).data[6] = 0;
        (*sprite).data[7] = 0;
        (*sprite).data[2] += 1;
    }
    if (*sprite).data[6] > 0 {
        (*sprite).data[6] -= 1;
        if (*sprite).data[5] != 3 {
            let mut scale: i16 = (8 * (*sprite).data[6] as i32 / 20) as i16;
            scale = Sin((*sprite).data[7] - scale, 64);
            HandleSetAffineData(sprite, 256 - scale, 256 - scale, 0);
        }
    } else {
        let mut var: i16 = 0;
        if (*sprite).data[5] == 3 {
            if (*sprite).data[7] > 63 {
                (*sprite).data[7] = 64;
                HandleSetAffineData(sprite, 256, 256, 0);
                ResetSpriteAfterAnim(sprite);
                (*sprite).callback = Some(WaitAnimEnd);
            }
            var = Cos((*sprite).data[7], 64);
        } else {
            var = Sin((*sprite).data[7], 64);
            if (*sprite).data[7] > 63 {
                (*sprite).data[5] = 3;
                (*sprite).data[6] = 10;
                (*sprite).data[7] = 0;
            } else {
                if var > 48 && (*sprite).data[5] == 1 {
                    (*sprite).data[5] = 2;
                    (*sprite).data[6] = 20;
                } else if var > 16 && (*sprite).data[5] == 0 {
                    (*sprite).data[5] = 1;
                    (*sprite).data[6] = 20;
                }
            }
        }
        (*sprite).data[7] += 2;
        HandleSetAffineData(sprite, 256 - var, 256 - var, 0);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_VerticalSpring(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
    }
    if (*sprite).data[7] > 512 {
        (*sprite).y2 = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).y2 = Sin((*sprite).data[7] % 256, 8);
        (*sprite).data[7] += 8;
        let yScale: i16 = Sin((*sprite).data[7] % 128, 96);
        HandleSetAffineData(sprite, 256, yScale + 256, 0);
    }
}
pub(crate) unsafe fn Anim_VerticalRepeatedSpring(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
    }
    if (*sprite).data[7] > 256 {
        (*sprite).y2 = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).y2 = Sin((*sprite).data[7], 16);
        (*sprite).data[7] += 4;
        let yScale: i16 = Sin((*sprite).data[7] % 64 * 2, 128);
        HandleSetAffineData(sprite, 256, yScale + 256, 0);
    }
}
pub(crate) unsafe fn Anim_SpringRising(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).callback = Some(SpringRising_0);
    (*sprite).data[7] = 0;
}
pub(crate) unsafe fn SpringRising_0(sprite: *mut Sprite) {
    let mut yScale: i16 = 0;
    (*sprite).data[7] += 8;
    if (*sprite).data[7] > 63 {
        (*sprite).data[7] = 0;
        (*sprite).data[6] = 0;
        (*sprite).callback = Some(SpringRising_1);
        yScale = Sin(64, 128);
    } else {
        yScale = Sin((*sprite).data[7], 128);
    }
    HandleSetAffineData(sprite, 256, 256 + yScale, 0);
}
pub(crate) unsafe fn SpringRising_1(sprite: *mut Sprite) {
    let mut yScale: i16 = 0;
    (*sprite).data[7] += 4;
    if (*sprite).data[7] > 95 {
        yScale = Cos(0, 128);
        (*sprite).data[7] = 0;
        (*sprite).data[6] += 1;
    } else {
        let mut sign: i16 = 0;
        let mut index: i16 = 0;
        (*sprite).y2 = -((*sprite).data[6] * 4) - Sin((*sprite).data[7], 8);
        if (*sprite).data[7] > 63 {
            sign = -1;
            index = (*sprite).data[7] - 64;
        } else {
            sign = 1;
            index = 0;
        }
        yScale = Cos(index * 2 + (*sprite).data[7], 128) * sign;
    }
    HandleSetAffineData(sprite, 256, 256 + yScale, 0);
    if (*sprite).data[6] == 3 {
        (*sprite).data[7] = 0;
        (*sprite).callback = Some(SpringRising_2);
    }
}
pub(crate) unsafe fn SpringRising_2(sprite: *mut Sprite) {
    (*sprite).data[7] += 8;
    let yScale: i16 = Cos((*sprite).data[7], 128);
    (*sprite).y2 = -Cos((*sprite).data[7], 12);
    if (*sprite).data[7] > 63 {
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).y2 = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
    }
    HandleSetAffineData(sprite, 256, 256 + yScale, 0);
}
unsafe fn HorizontalSpring(sprite: *mut Sprite) {
    if (*sprite).data[7] > (*sprite).data[5] {
        (*sprite).x2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
        HandleSetAffineData(sprite, 256, 256, 0);
    } else {
        (*sprite).x2 = Sin((*sprite).data[7] % 256, (*sprite).data[4]);
        (*sprite).data[7] += (*sprite).data[6];
        let xScale: i16 = Sin((*sprite).data[7] % 128, 96);
        HandleSetAffineData(sprite, 256 + xScale, 256, 0);
    }
}
pub(crate) unsafe fn Anim_HorizontalSpring(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
        (*sprite).data[6] = 8;
        (*sprite).data[5] = 512;
        (*sprite).data[4] = 8;
    }
    HorizontalSpring(sprite);
}
unsafe fn HorizontalRepeatedSpring(sprite: *mut Sprite) {
    if (*sprite).data[7] > (*sprite).data[5] {
        (*sprite).x2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
        HandleSetAffineData(sprite, 256, 256, 0);
    } else {
        (*sprite).x2 = Sin((*sprite).data[7] % 256, (*sprite).data[4]);
        (*sprite).data[7] += (*sprite).data[6];
        let xScale: i16 = Sin((*sprite).data[7] % 64 * 2, 128);
        HandleSetAffineData(sprite, 256 + xScale, 256, 0);
    }
}
pub(crate) unsafe fn Anim_HorizontalRepeatedSpring_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
        (*sprite).data[6] = 4;
        (*sprite).data[5] = 256;
        (*sprite).data[4] = 16;
    }
    HorizontalRepeatedSpring(sprite);
}
pub(crate) unsafe fn Anim_HorizontalSlideShrink(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
    }
    if (*sprite).data[7] > 512 {
        (*sprite).x2 = 0;
        ResetSpriteAfterAnim(sprite);
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).x2 = Sin((*sprite).data[7] % 256, 8);
        (*sprite).data[7] += 8;
        let scale: i16 = Sin((*sprite).data[7] % 128, 96);
        HandleSetAffineData(sprite, 256 + scale, 256 + scale, 0);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_LungeGrow(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
    }
    if (*sprite).data[7] > 512 {
        (*sprite).x2 = 0;
        ResetSpriteAfterAnim(sprite);
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).x2 = -Sin((*sprite).data[7] % 256 / 2, 16);
        (*sprite).data[7] += 8;
        let scale: i16 = -Sin((*sprite).data[7] % 256 / 2, 64);
        HandleSetAffineData(sprite, 256 + scale, 256 + scale, 0);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_CircleIntoBackground(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
    }
    if (*sprite).data[7] > 512 {
        (*sprite).x2 = 0;
        ResetSpriteAfterAnim(sprite);
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).x2 = -Sin((*sprite).data[7] % 256, 8);
        (*sprite).data[7] += 8;
        let scale: i16 = Sin((*sprite).data[7] % 256 / 2, 96);
        HandleSetAffineData(sprite, 256 + scale, 256 + scale, 0);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_RapidHorizontalHops(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] > 2048 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).data[6] = 0;
    } else {
        let caseVar: i16 = (*sprite).data[2] / 512 % 4;
        match caseVar {
            0 => {
                (*sprite).x2 = (-(((*sprite).data[2] % 512) as i32 * 16) / 512) as i16;
            }
            1 => {
                (*sprite).x2 = (*sprite).data[2] % 512 / 32 - 16;
            }
            2 => {
                (*sprite).x2 = (*sprite).data[2] % 512 / 32;
            }
            3 => {
                (*sprite).x2 = (-(((*sprite).data[2] % 512) as i32 * 16) / 512) as i16 + 16;
            }
            _ => {}
        }
        (*sprite).y2 = -Sin((*sprite).data[2] % 128, 4);
        (*sprite).data[2] += 24;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_FourPetal(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] == 0 {
        (*sprite).data[6] = 0;
        (*sprite).data[7] = 64;
        (*sprite).data[2] += 1;
    }
    (*sprite).data[7] += 8;
    if (*sprite).data[6] == 4 {
        if (*sprite).data[7] > 63 {
            (*sprite).data[7] = 0;
            (*sprite).data[6] += 1;
        }
    } else {
        if (*sprite).data[7] > 127 {
            (*sprite).data[7] = 0;
            (*sprite).data[6] += 1;
        }
    }
    match (*sprite).data[6] {
        1 => {
            (*sprite).x2 = -Cos((*sprite).data[7], 8);
            (*sprite).y2 = Sin((*sprite).data[7], 8) - 8;
        }
        2 => {
            (*sprite).x2 = Sin((*sprite).data[7] + 128, 8) + 8;
            (*sprite).y2 = -Cos((*sprite).data[7], 8);
        }
        3 => {
            (*sprite).x2 = Cos((*sprite).data[7], 8);
            (*sprite).y2 = Sin((*sprite).data[7] + 128, 8) + 8;
        }
        0 | 4 => {
            (*sprite).x2 = Sin((*sprite).data[7], 8) - 8;
            (*sprite).y2 = Cos((*sprite).data[7], 8);
        }
        _ => {
            (*sprite).x2 = 0;
            (*sprite).y2 = 0;
            (*sprite).callback = Some(WaitAnimEnd);
        }
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_VerticalSquishBounce_Slow(sprite: *mut Sprite) {
    (*sprite).data[0] = 32;
    VerticalSquishBounce(sprite);
    (*sprite).callback = Some(VerticalSquishBounce);
}
pub(crate) unsafe fn Anim_HorizontalSlide_Slow(sprite: *mut Sprite) {
    (*sprite).data[0] = 80;
    HorizontalSlide(sprite);
    (*sprite).callback = Some(HorizontalSlide);
}
pub(crate) unsafe fn Anim_VerticalSlide_Slow(sprite: *mut Sprite) {
    (*sprite).data[0] = 80;
    VerticalSlide(sprite);
    (*sprite).callback = Some(VerticalSlide);
}
pub(crate) unsafe fn Anim_BounceRotateToSides_Small(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 2048;
    sAnims[id].data = (*sprite).data[6];
    BounceRotateToSides(sprite);
    (*sprite).callback = Some(BounceRotateToSides);
}
pub(crate) unsafe fn Anim_BounceRotateToSides_Slow(sprite: *mut Sprite) {
    (*sprite).data[6] = 1;
    Anim_BounceRotateToSides(sprite);
}
pub(crate) unsafe fn Anim_BounceRotateToSides_SmallSlow(sprite: *mut Sprite) {
    (*sprite).data[6] = 1;
    Anim_BounceRotateToSides_Small(sprite);
}
pub(crate) unsafe fn Anim_ZigzagSlow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[0] = 0;
    }
    if (*sprite).data[0] <= 0 {
        Zigzag(sprite);
        (*sprite).data[0] = 1;
    } else {
        (*sprite).data[0] -= 1;
    }
}
pub(crate) unsafe fn Anim_HorizontalShake_Slow(sprite: *mut Sprite) {
    (*sprite).data[0] = 30;
    (*sprite).data[7] = 3;
    HorizontalShake(sprite);
    (*sprite).callback = Some(HorizontalShake);
}
pub(crate) unsafe fn Anim_VertialShake_Slow(sprite: *mut Sprite) {
    (*sprite).data[0] = 30;
    VerticalShake(sprite);
    (*sprite).callback = Some(VerticalShake);
}
pub(crate) unsafe fn Anim_Twist_Twice(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 1024;
    sAnims[id].delay = 0;
    sAnims[id].runs = 2;
    Twist(sprite);
    (*sprite).callback = Some(Twist);
}
pub(crate) unsafe fn Anim_CircleCounterclockwise_Slow(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 512;
    sAnims[id].data = 3;
    sAnims[id].speed = 12;
    CircleCounterclockwise(sprite);
    (*sprite).callback = Some(CircleCounterclockwise);
}
pub(crate) unsafe fn Anim_VerticalShakeTwice_Slow(sprite: *mut Sprite) {
    (*sprite).data[0] = 24;
    VerticalShakeTwice(sprite);
    (*sprite).callback = Some(VerticalShakeTwice);
}
pub(crate) unsafe fn Anim_VerticalSlideWobble_Small(sprite: *mut Sprite) {
    (*sprite).data[0] = 5;
    VerticalSlideWobble(sprite);
    (*sprite).callback = Some(VerticalSlideWobble);
}
pub(crate) unsafe fn Anim_VerticalJumps_Small(sprite: *mut Sprite) {
    (*sprite).data[0] = 3;
    VerticalJumps(sprite);
    (*sprite).callback = Some(VerticalJumps);
}
pub(crate) unsafe fn Anim_Spin(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].delay = 60;
    sAnims[id].data = 30;
    Spin(sprite);
    (*sprite).callback = Some(Spin);
}
pub(crate) unsafe fn Anim_TumblingFrontFlip_Twice(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].speed = 1;
    sAnims[id].runs = 2;
    TumblingFrontFlip(sprite);
    (*sprite).callback = Some(TumblingFrontFlip);
}
pub(crate) unsafe fn Anim_DeepVerticalSquishBounce_Twice(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 4;
    sAnims[id].runs = 2;
    DeepVerticalSquishBounce(sprite);
    (*sprite).callback = Some(DeepVerticalSquishBounce);
}
pub(crate) unsafe fn Anim_HorizontalJumpsVerticalStretch_Twice(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].data = 1;
    sAnims[id].runs = 2;
    HandleStartAffineAnim(sprite);
    (*sprite).data[3] = 0;
    HorizontalJumpsVerticalStretch_0(sprite);
    (*sprite).callback = Some(HorizontalJumpsVerticalStretch_0);
}
pub(crate) unsafe fn Anim_RotateToSides(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 2;
    RotateToSides(sprite);
    (*sprite).callback = Some(RotateToSides);
}
pub(crate) unsafe fn Anim_RotateToSides_Twice(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 4;
    sAnims[id].runs = 2;
    RotateToSides(sprite);
    (*sprite).callback = Some(RotateToSides);
}
pub(crate) unsafe fn Anim_SwingConcave(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].data = 100;
    SwingConcave(sprite);
    (*sprite).callback = Some(SwingConcave);
}
pub(crate) unsafe fn Anim_SwingConcave_Fast(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].data = 50;
    sAnims[id].runs = 2;
    SwingConcave(sprite);
    (*sprite).callback = Some(SwingConcave);
}
pub(crate) unsafe fn Anim_SwingConvex(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].data = 100;
    SwingConvex(sprite);
    (*sprite).callback = Some(SwingConvex);
}
pub(crate) unsafe fn Anim_SwingConvex_Fast(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].data = 50;
    sAnims[id].runs = 2;
    SwingConvex(sprite);
    (*sprite).callback = Some(SwingConvex);
}
pub(crate) unsafe fn VerticalShakeBack(sprite: *mut Sprite) {
    let counter: i32 = (*sprite).data[2] as i32;
    if counter > 2304 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).y2 = 0;
    } else {
        (*sprite).y2 = Sin(((counter + 192) % 256) as i16, (*sprite).data[7]) + (*sprite).data[7];
    }
    (*sprite).data[2] += (*sprite).data[0];
}
pub(crate) unsafe fn Anim_VerticalShakeBack(sprite: *mut Sprite) {
    (*sprite).data[0] = 60;
    (*sprite).data[7] = 3;
    VerticalShakeBack(sprite);
    (*sprite).callback = Some(VerticalShakeBack);
}
pub(crate) unsafe fn Anim_VerticalShakeBack_Slow(sprite: *mut Sprite) {
    (*sprite).data[0] = 30;
    (*sprite).data[7] = 3;
    VerticalShakeBack(sprite);
    (*sprite).callback = Some(VerticalShakeBack);
}
pub(crate) unsafe fn Anim_VerticalShakeHorizontalSlide_Slow(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] > 2048 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).data[6] = 0;
    } else {
        let divCase: i16 = (*sprite).data[2] / 512 % 4;
        match divCase {
            0 => {
                (*sprite).x2 = (*sprite).data[2] % 512 / 32;
            }
            2 => {
                (*sprite).x2 = (-(((*sprite).data[2] % 512) as i32 * 16) / 512) as i16;
            }
            1 => {
                (*sprite).x2 = (-(((*sprite).data[2] % 512) as i32 * 16) / 512) as i16 + 16;
            }
            3 => {
                (*sprite).x2 = (*sprite).data[2] % 512 / 32 - 16;
            }
            _ => {}
        }
        (*sprite).y2 = Sin((*sprite).data[2] % 128, 4);
        (*sprite).data[2] += 24;
    }
    TryFlipX(sprite);
}
unsafe fn VerticalStretchBothEnds(sprite: *mut Sprite) {
    let mut index1: i16 = 0;
    let mut index2: i16 = 0;
    if (*sprite).data[5] > (*sprite).data[6] {
        (*sprite).y2 = 0;
        (*sprite).data[5] = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        if (*sprite).data[4] <= 1 {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
        } else {
            (*sprite).data[4] -= 1;
            (*sprite).data[7] = 0;
        }
    } else {
        let mut xScale: i16 = 0;
        index2 = div_i32((*sprite).data[5] as i32 * 128, (*sprite).data[6] as i32) as i16;
        let cmpVal1: u8 = ((*sprite).data[6] / 4) as u8;
        let cmpVal2: u8 = cmpVal1 * 3;
        if (*sprite).data[5] >= cmpVal1 as i16 && (*sprite).data[5] < cmpVal2 as i16 {
            (*sprite).data[7] += 51;
            index1 = (*sprite).data[7] & 0xFF;
        }
        if (*sprite).data[sDontFlip] == 0 {
            xScale = -256 - Sin(index2, 16);
        } else {
            xScale = 256 + Sin(index2, 16);
        }
        let amplitude: u8 = (*sprite).data[3] as u8;
        let yScale: i16 =
            256 - Sin(index2, amplitude as i16) - Sin(index1, (amplitude as i32 / 5) as i16);
        SetAffineData(sprite, xScale, yScale, 0);
        (*sprite).data[5] += 1;
    }
}
pub(crate) unsafe fn Anim_VerticalStretchBothEnds_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        HandleStartAffineAnim(sprite);
        (*sprite).data[4] = 1;
        (*sprite).data[6] = 40;
        (*sprite).data[3] = 40;
        (*sprite).data[5] = 0;
        (*sprite).data[7] = 0;
    }
    VerticalStretchBothEnds(sprite);
}
unsafe fn HorizontalStretchFar(sprite: *mut Sprite) {
    let mut index1: i16 = 0;
    let mut index2: i16 = 0;
    if (*sprite).data[5] > (*sprite).data[6] {
        (*sprite).data[5] = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        if (*sprite).data[4] <= 1 {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
        } else {
            (*sprite).data[4] -= 1;
            (*sprite).data[7] = 0;
        }
    } else {
        let mut xScale: i16 = 0;
        index2 = div_i32((*sprite).data[5] as i32 * 128, (*sprite).data[6] as i32) as i16;
        let cmpVal1: u8 = ((*sprite).data[6] / 4) as u8;
        let cmpVal2: u8 = cmpVal1 * 3;
        if (*sprite).data[5] >= cmpVal1 as i16 && (*sprite).data[5] < cmpVal2 as i16 {
            (*sprite).data[7] += 51;
            index1 = (*sprite).data[7] & 0xFF;
        }
        let amplitude: u8 = (*sprite).data[3] as u8;
        if (*sprite).data[sDontFlip] == 0 {
            xScale = -256
                + Sin(index2, amplitude as i16)
                + Sin(index1, (amplitude as i32 / 5) as i16 * 2);
        } else {
            xScale = 256
                - Sin(index2, amplitude as i16)
                - Sin(index1, (amplitude as i32 / 5) as i16 * 2);
        }
        SetAffineData(sprite, xScale, 256, 0);
        (*sprite).data[5] += 1;
    }
}
pub(crate) unsafe fn Anim_HorizontalStretchFar_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        HandleStartAffineAnim(sprite);
        (*sprite).data[4] = 1;
        (*sprite).data[6] = 40;
        (*sprite).data[3] = 40;
        (*sprite).data[5] = 0;
        (*sprite).data[7] = 0;
    }
    HorizontalStretchFar(sprite);
}
pub(crate) unsafe fn VerticalShakeLowTwice(sprite: *mut Sprite) {
    let var8: u8 = (*sprite).data[2] as u8;
    let var9: u8 = (*sprite).data[6] as u8;
    let mut var5: u8 = sVerticalShakeData[(*sprite).data[5]][0];
    if var5 != 255 {
        var5 = (*sprite).data[7] as u8;
    }
    let var6: u8 = sVerticalShakeData[(*sprite).data[5]][1];
    let mut var7: u8 = 0;
    if sVerticalShakeData[(*sprite).data[5]][0] != 254 {
        var7 = div_i32((var6 as i32 - var9 as i32) * var5 as i32, var6 as i32) as u8;
    } else {
        var7 = 0;
    }
    if var5 == 255 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).y2 = 0;
    } else {
        (*sprite).y2 = Sin(((var8 as i32 + 192) % 256) as i16, var7 as i16) + var7 as i16;
        if var9 == var6 {
            (*sprite).data[5] += 1;
            (*sprite).data[6] = 0;
        } else {
            (*sprite).data[2] += (*sprite).data[0];
            (*sprite).data[6] += 1;
        }
    }
}
pub(crate) unsafe fn Anim_VerticalShakeLowTwice(sprite: *mut Sprite) {
    (*sprite).data[0] = 40;
    (*sprite).data[7] = 6;
    VerticalShakeLowTwice(sprite);
    (*sprite).callback = Some(VerticalShakeLowTwice);
}
pub(crate) unsafe fn Anim_HorizontalShake_Fast(sprite: *mut Sprite) {
    (*sprite).data[0] = 70;
    (*sprite).data[7] = 6;
    HorizontalShake(sprite);
    (*sprite).callback = Some(HorizontalShake);
}
pub(crate) unsafe fn Anim_HorizontalSlide_Fast(sprite: *mut Sprite) {
    (*sprite).data[0] = 20;
    HorizontalSlide(sprite);
    (*sprite).callback = Some(HorizontalSlide);
}
pub(crate) unsafe fn Anim_HorizontalVibrate_Fast(sprite: *mut Sprite) {
    if (*sprite).data[2] > 40 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
    } else {
        let mut sign: i8 = 0;
        if (*sprite).data[2] as i32 & 1 == 0 {
            sign = 1;
        } else {
            sign = -1;
        }
        (*sprite).x2 = Sin(((*sprite).data[2] as i32 * 128 / 40 % 256) as i16, 9) * sign as i16;
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_HorizontalVibrate_Fastest(sprite: *mut Sprite) {
    if (*sprite).data[2] > 40 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).x2 = 0;
    } else {
        let mut sign: i8 = 0;
        if (*sprite).data[2] as i32 & 1 == 0 {
            sign = 1;
        } else {
            sign = -1;
        }
        (*sprite).x2 = Sin(((*sprite).data[2] as i32 * 128 / 40 % 256) as i16, 12) * sign as i16;
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_VerticalShakeBack_Fast(sprite: *mut Sprite) {
    (*sprite).data[0] = 70;
    (*sprite).data[7] = 6;
    VerticalShakeBack(sprite);
    (*sprite).callback = Some(VerticalShakeBack);
}
pub(crate) unsafe fn Anim_VerticalShakeLowTwice_Slow(sprite: *mut Sprite) {
    (*sprite).data[0] = 24;
    (*sprite).data[7] = 6;
    VerticalShakeLowTwice(sprite);
    (*sprite).callback = Some(VerticalShakeLowTwice);
}
pub(crate) unsafe fn Anim_VerticalShakeLowTwice_Fast(sprite: *mut Sprite) {
    (*sprite).data[0] = 56;
    (*sprite).data[7] = 9;
    VerticalShakeLowTwice(sprite);
    (*sprite).callback = Some(VerticalShakeLowTwice);
}
pub(crate) unsafe fn Anim_CircleCounterclockwise_Long(sprite: *mut Sprite) {
    let id: u8 = ({
        (*sprite).data[0] = AddNewAnim() as i16;
        (*sprite).data[0]
    }) as u8;
    sAnims[id].rotation = 1024;
    sAnims[id].data = 6;
    sAnims[id].speed = 24;
    CircleCounterclockwise(sprite);
    (*sprite).callback = Some(CircleCounterclockwise);
}
unsafe fn GrowStutter(sprite: *mut Sprite) {
    let mut index1: i16 = 0;
    let mut index2: i16 = 0;
    if (*sprite).data[5] > (*sprite).data[6] {
        (*sprite).y2 = 0;
        (*sprite).data[5] = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        if (*sprite).data[4] <= 1 {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
        } else {
            (*sprite).data[4] -= 1;
            (*sprite).data[7] = 0;
        }
    } else {
        let mut xScale: i16 = 0;
        index2 = div_i32((*sprite).data[5] as i32 * 128, (*sprite).data[6] as i32) as i16;
        let cmpVal1: u8 = ((*sprite).data[6] / 4) as u8;
        let cmpVal2: u8 = cmpVal1 * 3;
        if (*sprite).data[5] >= cmpVal1 as i16 && (*sprite).data[5] < cmpVal2 as i16 {
            (*sprite).data[7] += 51;
            index1 = (*sprite).data[7] & 0xFF;
        }
        let amplitude: u8 = (*sprite).data[3] as u8;
        if (*sprite).data[sDontFlip] == 0 {
            xScale = Sin(index2, amplitude as i16)
                + (Sin(index1, (amplitude as i32 / 5) as i16 * 2) - 256);
        } else {
            xScale = 256
                - Sin(index1, (amplitude as i32 / 5) as i16 * 2)
                - Sin(index2, amplitude as i16);
        }
        let yScale: i16 =
            256 - Sin(index1, (amplitude as i32 / 5) as i16) - Sin(index2, amplitude as i16);
        SetAffineData(sprite, xScale, yScale, 0);
        (*sprite).data[5] += 1;
    }
}
pub(crate) unsafe fn Anim_GrowStutter_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        HandleStartAffineAnim(sprite);
        (*sprite).data[4] = 1;
        (*sprite).data[6] = 40;
        (*sprite).data[3] = 40;
        (*sprite).data[5] = 0;
        (*sprite).data[7] = 0;
    }
    GrowStutter(sprite);
}
pub(crate) unsafe fn Anim_VerticalShakeHorizontalSlide(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] > 2048 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).data[6] = 0;
    } else {
        let divCase: i16 = (*sprite).data[2] / 512 % 4;
        match divCase {
            0 => {
                (*sprite).x2 = (*sprite).data[2] % 512 / 32;
            }
            2 => {
                (*sprite).x2 = (-(((*sprite).data[2] % 512) as i32 * 16) / 512) as i16;
            }
            1 => {
                (*sprite).x2 = (-(((*sprite).data[2] % 512) as i32 * 16) / 512) as i16 + 16;
            }
            3 => {
                (*sprite).x2 = (*sprite).data[2] % 512 / 32 - 16;
            }
            _ => {}
        }
        (*sprite).y2 = Sin((*sprite).data[2] % 128, 4);
        (*sprite).data[2] += 48;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_VerticalShakeHorizontalSlide_Fast(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] > 2048 {
        (*sprite).callback = Some(WaitAnimEnd);
        (*sprite).data[6] = 0;
    } else {
        let divCase: i16 = (*sprite).data[2] / 512 % 4;
        match divCase {
            0 => {
                (*sprite).x2 = (*sprite).data[2] % 512 / 32;
            }
            2 => {
                (*sprite).x2 = (-(((*sprite).data[2] % 512) as i32 * 16) / 512) as i16;
            }
            1 => {
                (*sprite).x2 = (-(((*sprite).data[2] % 512) as i32 * 16) / 512) as i16 + 16;
            }
            3 => {
                (*sprite).x2 = (*sprite).data[2] % 512 / 32 - 16;
            }
            _ => {}
        }
        (*sprite).y2 = Sin((*sprite).data[2] % 96, 4);
        (*sprite).data[2] += 64;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn TriangleDown(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] == 0 {
        (*sprite).data[3] = 0;
    }
    if div_i32(
        sTriangleDownData[(*sprite).data[3]][2] as i32,
        (*sprite).data[5] as i32,
    ) == (*sprite).data[2] as i32
    {
        (*sprite).data[3] += 1;
        (*sprite).data[2] = 0;
    }
    if div_i32(
        sTriangleDownData[(*sprite).data[3]][2] as i32,
        (*sprite).data[5] as i32,
    ) == 0
    {
        if ({
            (*sprite).data[6] -= 1;
            (*sprite).data[6]
        }) == 0
        {
            (*sprite).callback = Some(WaitAnimEnd);
        } else {
            (*sprite).data[2] = 0;
        }
    } else {
        let amplitude: i32 = (*sprite).data[5] as i32;
        (*sprite).x2 += sTriangleDownData[(*sprite).data[3]][0] as i16 * amplitude as i16;
        (*sprite).y2 += sTriangleDownData[(*sprite).data[3]][1] as i16 * (*sprite).data[5];
        (*sprite).data[2] += 1;
        TryFlipX(sprite);
    }
}
pub(crate) unsafe fn Anim_TriangleDown_Slow(sprite: *mut Sprite) {
    (*sprite).data[5] = 1;
    (*sprite).data[6] = 1;
    TriangleDown(sprite);
    (*sprite).callback = Some(TriangleDown);
}
pub(crate) unsafe fn Anim_TriangleDown(sprite: *mut Sprite) {
    (*sprite).data[5] = 2;
    (*sprite).data[6] = 1;
    TriangleDown(sprite);
    (*sprite).callback = Some(TriangleDown);
}
pub(crate) unsafe fn Anim_TriangleDown_Fast(sprite: *mut Sprite) {
    (*sprite).data[5] = 2;
    (*sprite).data[6] = 2;
    TriangleDown(sprite);
    (*sprite).callback = Some(TriangleDown);
}
unsafe fn Grow(sprite: *mut Sprite) {
    if (*sprite).data[7] > 255 {
        if (*sprite).data[5] <= 1 {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
            HandleSetAffineData(sprite, 256, 256, 0);
        } else {
            (*sprite).data[5] -= 1;
            (*sprite).data[7] = 0;
        }
    } else {
        (*sprite).data[7] += (*sprite).data[6];
        if (*sprite).data[7] > 256 {
            (*sprite).data[7] = 256;
        }
        let scale: i16 = Sin((*sprite).data[7] / 2, 64);
        HandleSetAffineData(sprite, 256 - scale, 256 - scale, 0);
    }
}
pub(crate) unsafe fn Anim_Grow(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
        (*sprite).data[6] = 4;
        (*sprite).data[5] = 1;
    }
    Grow(sprite);
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_Grow_Twice(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
        (*sprite).data[6] = 8;
        (*sprite).data[5] = 2;
    }
    Grow(sprite);
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_HorizontalSpring_Fast(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
        (*sprite).data[6] = 8;
        (*sprite).data[5] = 512;
        (*sprite).data[4] = 16;
    }
    HorizontalSpring(sprite);
}
pub(crate) unsafe fn Anim_HorizontalSpring_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
        (*sprite).data[6] = 4;
        (*sprite).data[5] = 256;
        (*sprite).data[4] = 16;
    }
    HorizontalSpring(sprite);
}
pub(crate) unsafe fn Anim_HorizontalRepeatedSpring_Fast(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
        (*sprite).data[6] = 8;
        (*sprite).data[5] = 512;
        (*sprite).data[4] = 16;
    }
    HorizontalRepeatedSpring(sprite);
}
pub(crate) unsafe fn Anim_HorizontalRepeatedSpring(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[2] += 1;
        (*sprite).data[7] = 0;
        (*sprite).data[6] = 8;
        (*sprite).data[5] = 512;
        (*sprite).data[4] = 8;
    }
    HorizontalRepeatedSpring(sprite);
}
pub(crate) unsafe fn Anim_ShrinkGrow_Fast(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[7] = 5;
        (*sprite).data[6] = 8;
    }
    ShrinkGrow(sprite);
}
pub(crate) unsafe fn Anim_ShrinkGrow_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[7] = 3;
        (*sprite).data[6] = 4;
    }
    ShrinkGrow(sprite);
}
pub(crate) unsafe fn Anim_VerticalStretchBothEnds(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        HandleStartAffineAnim(sprite);
        (*sprite).data[4] = 1;
        (*sprite).data[6] = 30;
        (*sprite).data[3] = 60;
        (*sprite).data[7] = 0;
    }
    VerticalStretchBothEnds(sprite);
}
pub(crate) unsafe fn Anim_VerticalStretchBothEnds_Twice(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        HandleStartAffineAnim(sprite);
        (*sprite).data[4] = 2;
        (*sprite).data[6] = 20;
        (*sprite).data[3] = 70;
        (*sprite).data[7] = 0;
    }
    VerticalStretchBothEnds(sprite);
}
pub(crate) unsafe fn Anim_HorizontalStretchFar_Twice(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        HandleStartAffineAnim(sprite);
        (*sprite).data[4] = 2;
        (*sprite).data[6] = 20;
        (*sprite).data[3] = 70;
        (*sprite).data[5] = 0;
        (*sprite).data[7] = 0;
    }
    HorizontalStretchFar(sprite);
}
pub(crate) unsafe fn Anim_HorizontalStretchFar(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        HandleStartAffineAnim(sprite);
        (*sprite).data[4] = 1;
        (*sprite).data[6] = 30;
        (*sprite).data[3] = 60;
        (*sprite).data[5] = 0;
        (*sprite).data[7] = 0;
    }
    HorizontalStretchFar(sprite);
}
pub(crate) unsafe fn Anim_GrowStutter_Twice(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        HandleStartAffineAnim(sprite);
        (*sprite).data[4] = 2;
        (*sprite).data[6] = 20;
        (*sprite).data[3] = 70;
        (*sprite).data[5] = 0;
        (*sprite).data[7] = 0;
    }
    GrowStutter(sprite);
}
pub(crate) unsafe fn Anim_GrowStutter(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        HandleStartAffineAnim(sprite);
        (*sprite).data[4] = 1;
        (*sprite).data[6] = 30;
        (*sprite).data[3] = 60;
        (*sprite).data[5] = 0;
        (*sprite).data[7] = 0;
    }
    GrowStutter(sprite);
}
unsafe fn ConcaveArc(sprite: *mut Sprite) {
    if (*sprite).data[7] > 255 {
        if (*sprite).data[6] <= 1 {
            (*sprite).callback = Some(WaitAnimEnd);
            (*sprite).x2 = 0;
            (*sprite).y2 = 0;
        } else {
            (*sprite).data[7] %= 256;
            (*sprite).data[6] -= 1;
        }
    } else {
        (*sprite).x2 = -Sin((*sprite).data[7], (*sprite).data[5]);
        (*sprite).y2 = Sin(
            (((*sprite).data[7] as i32 + 192) % 256) as i16,
            (*sprite).data[4],
        );
        if (*sprite).y2 > 0 {
            (*sprite).y2 *= -1;
        }
        (*sprite).y2 += (*sprite).data[4];
        (*sprite).data[7] += (*sprite).data[3];
    }
}
pub(crate) unsafe fn Anim_ConcaveArcLarge_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        (*sprite).data[6] = 1;
        (*sprite).data[7] = 0;
        (*sprite).data[5] = 12;
        (*sprite).data[4] = 12;
        (*sprite).data[3] = 4;
    }
    ConcaveArc(sprite);
}
pub(crate) unsafe fn Anim_ConcaveArcLarge(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        (*sprite).data[6] = 1;
        (*sprite).data[7] = 0;
        (*sprite).data[5] = 12;
        (*sprite).data[4] = 12;
        (*sprite).data[3] = 6;
    }
    ConcaveArc(sprite);
}
pub(crate) unsafe fn Anim_ConcaveArcLarge_Twice(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        (*sprite).data[6] = 2;
        (*sprite).data[7] = 0;
        (*sprite).data[5] = 12;
        (*sprite).data[4] = 12;
        (*sprite).data[3] = 8;
    }
    ConcaveArc(sprite);
}
unsafe fn ConvexDoubleArc(sprite: *mut Sprite) {
    if (*sprite).data[7] > 256 {
        if (*sprite).data[6] <= (*sprite).data[4] {
            (*sprite).callback = Some(WaitAnimEnd);
        } else {
            (*sprite).data[4] += 1;
            (*sprite).data[7] = 0;
        }
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
    } else {
        if (*sprite).data[7] > 159 {
            if (*sprite).data[7] > 256 {
                (*sprite).data[7] = 256;
            }
            (*sprite).y2 = -Sin((*sprite).data[7] % 256, 8);
        } else if (*sprite).data[7] > 95 {
            (*sprite).y2 = Sin(96, 6) - Sin(((*sprite).data[7] - 96) * 2, 4);
        } else {
            (*sprite).y2 = Sin((*sprite).data[7], 6);
        }
        let mut posX: i16 = -Sin((*sprite).data[7] / 2, (*sprite).data[5]);
        if (*sprite).data[4] % 2 == 0 {
            posX *= -1;
        }
        (*sprite).x2 = posX;
        (*sprite).data[7] += (*sprite).data[3];
    }
}
pub(crate) unsafe fn Anim_ConvexDoubleArc_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        (*sprite).data[6] = 2;
        (*sprite).data[7] = 0;
        (*sprite).data[5] = 16;
        (*sprite).data[4] = 1;
        (*sprite).data[3] = 4;
    }
    ConvexDoubleArc(sprite);
}
pub(crate) unsafe fn Anim_ConvexDoubleArc(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        (*sprite).data[6] = 2;
        (*sprite).data[7] = 0;
        (*sprite).data[5] = 16;
        (*sprite).data[4] = 1;
        (*sprite).data[3] = 6;
    }
    ConvexDoubleArc(sprite);
}
pub(crate) unsafe fn Anim_ConvexDoubleArc_Twice(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        (*sprite).data[6] = 3;
        (*sprite).data[7] = 0;
        (*sprite).data[5] = 16;
        (*sprite).data[4] = 1;
        (*sprite).data[3] = 8;
    }
    ConvexDoubleArc(sprite);
}
pub(crate) unsafe fn Anim_ConcaveArcSmall_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        (*sprite).data[6] = 1;
        (*sprite).data[7] = 0;
        (*sprite).data[5] = 4;
        (*sprite).data[4] = 6;
        (*sprite).data[3] = 4;
    }
    ConcaveArc(sprite);
}
pub(crate) unsafe fn Anim_ConcaveArcSmall(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        (*sprite).data[6] = 1;
        (*sprite).data[7] = 0;
        (*sprite).data[5] = 4;
        (*sprite).data[4] = 6;
        (*sprite).data[3] = 6;
    }
    ConcaveArc(sprite);
}
pub(crate) unsafe fn Anim_ConcaveArcSmall_Twice(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[2] = 1;
        (*sprite).data[6] = 2;
        (*sprite).data[7] = 0;
        (*sprite).data[5] = 4;
        (*sprite).data[4] = 6;
        (*sprite).data[3] = 8;
    }
    ConcaveArc(sprite);
}
unsafe fn SetHorizontalDip(sprite: *mut Sprite) {
    let index: u16 = Sin(
        div_i32((*sprite).data[2] as i32 * 128, (*sprite).data[7] as i32) as i16,
        (*sprite).data[5],
    ) as u16;
    (*sprite).data[6] = -((index as i16) << 8);
    SetPosForRotation(sprite, index, (*sprite).data[4], 0);
    HandleSetAffineData(sprite, 256, 256, (*sprite).data[6] as u16);
}
pub(crate) unsafe fn Anim_HorizontalDip(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[7] = 60;
        (*sprite).data[5] = 8;
        (*sprite).data[4] = -32;
        (*sprite).data[3] = 1;
        (*sprite).data[0] = 0;
    }
    if (*sprite).data[2] > (*sprite).data[7] {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).data[0] += 1;
        if (*sprite).data[3] <= (*sprite).data[0] {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
            return;
        } else {
            (*sprite).data[2] = 0;
        }
    } else {
        SetHorizontalDip(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_HorizontalDip_Fast(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[7] = 90;
        (*sprite).data[5] = 8;
        (*sprite).data[4] = -32;
        (*sprite).data[3] = 1;
        (*sprite).data[0] = 0;
    }
    if (*sprite).data[2] > (*sprite).data[7] {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).data[0] += 1;
        if (*sprite).data[3] <= (*sprite).data[0] {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
            return;
        } else {
            (*sprite).data[2] = 0;
        }
    } else {
        SetHorizontalDip(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_HorizontalDip_Twice(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).data[7] = 30;
        (*sprite).data[5] = 8;
        (*sprite).data[4] = -32;
        (*sprite).data[3] = 2;
        (*sprite).data[0] = 0;
    }
    if (*sprite).data[2] > (*sprite).data[7] {
        HandleSetAffineData(sprite, 256, 256, 0);
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).data[0] += 1;
        if (*sprite).data[3] <= (*sprite).data[0] {
            ResetSpriteAfterAnim(sprite);
            (*sprite).callback = Some(WaitAnimEnd);
            return;
        } else {
            (*sprite).data[2] = 0;
        }
    } else {
        SetHorizontalDip(sprite);
    }
    (*sprite).data[2] += 1;
}
unsafe fn ShrinkGrowVibrate(sprite: *mut Sprite) {
    if (*sprite).data[2] > (*sprite).data[7] {
        (*sprite).y2 = 0;
        HandleSetAffineData(sprite, 256, 256, 0);
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        let mut sinY: i8 = 0;
        let index: i16 = (div_i32(
            rem_i32((*sprite).data[2] as i32, (*sprite).data[6] as i32) as u16 as i32 * 256,
            (*sprite).data[6] as i32,
        ) % 256) as i16;
        if (*sprite).data[2] % 2 == 0 {
            (*sprite).data[4] = Sin(index, 32) + 256;
            (*sprite).data[5] = Sin(index, 32) + 256;
            sinY = Sin(index, 32) as i8;
        } else {
            (*sprite).data[4] = Sin(index, 8) + 256;
            (*sprite).data[5] = Sin(index, 8) + 256;
            sinY = Sin(index, 8) as i8;
        }
        let y: u16 = (sinY / 8) as u16;
        (*sprite).y2 = y as i16;
        HandleSetAffineData(sprite, (*sprite).data[4], (*sprite).data[5], 0);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_ShrinkGrowVibrate_Fast(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).y2 += 2;
        (*sprite).data[6] = 40;
        (*sprite).data[7] = 80;
    }
    ShrinkGrowVibrate(sprite);
}
pub(crate) unsafe fn Anim_ShrinkGrowVibrate(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).y2 += 2;
        (*sprite).data[6] = 40;
        (*sprite).data[7] = 40;
    }
    ShrinkGrowVibrate(sprite);
}
pub(crate) unsafe fn Anim_ShrinkGrowVibrate_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        HandleStartAffineAnim(sprite);
        (*sprite).y2 += 2;
        (*sprite).data[6] = 80;
        (*sprite).data[7] = 80;
    }
    ShrinkGrowVibrate(sprite);
}
pub(crate) unsafe fn JoltRight(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 -= (*sprite).data[2];
    if (*sprite).x2 as i32 <= -((*sprite).data[6] as i32) {
        (*sprite).x2 = -(*sprite).data[6];
        (*sprite).data[7] = 2;
        (*sprite).callback = Some(JoltRight_0);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn JoltRight_0(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 += (*sprite).data[7];
    (*sprite).data[7] += 1;
    if (*sprite).x2 >= 0 {
        (*sprite).callback = Some(JoltRight_1);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn JoltRight_1(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 += (*sprite).data[7];
    (*sprite).data[7] += 1;
    if (*sprite).x2 > (*sprite).data[6] {
        (*sprite).x2 = (*sprite).data[6];
        (*sprite).callback = Some(JoltRight_2);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn JoltRight_2(sprite: *mut Sprite) {
    TryFlipX(sprite);
    if (*sprite).data[3] >= (*sprite).data[5] {
        (*sprite).callback = Some(JoltRight_3);
    } else {
        (*sprite).x2 += (*sprite).data[4];
        (*sprite).data[4] *= -1;
        (*sprite).data[3] += 1;
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn JoltRight_3(sprite: *mut Sprite) {
    TryFlipX(sprite);
    (*sprite).x2 -= 2;
    if (*sprite).x2 <= 0 {
        (*sprite).x2 = 0;
        ResetSpriteAfterAnim(sprite);
        (*sprite).callback = Some(WaitAnimEnd);
    }
    TryFlipX(sprite);
}
pub(crate) unsafe fn Anim_JoltRight_Fast(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).data[7] = 4;
    (*sprite).data[6] = 12;
    (*sprite).data[5] = 16;
    (*sprite).data[4] = 4;
    (*sprite).data[3] = 0;
    (*sprite).data[2] = 2;
    (*sprite).callback = Some(JoltRight);
}
pub(crate) unsafe fn Anim_JoltRight(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).data[7] = 2;
    (*sprite).data[6] = 8;
    (*sprite).data[5] = 12;
    (*sprite).data[4] = 2;
    (*sprite).data[3] = 0;
    (*sprite).data[2] = 1;
    (*sprite).callback = Some(JoltRight);
}
pub(crate) unsafe fn Anim_JoltRight_Slow(sprite: *mut Sprite) {
    HandleStartAffineAnim(sprite);
    (*sprite).data[7] = 0;
    (*sprite).data[6] = 6;
    (*sprite).data[5] = 6;
    (*sprite).data[4] = 2;
    (*sprite).data[3] = 0;
    (*sprite).data[2] = 1;
    (*sprite).callback = Some(JoltRight);
}
unsafe fn SetShakeFlashYellowPos(sprite: *mut Sprite) {
    (*sprite).x2 = (*sprite).data[1];
    if (*sprite).data[0] > 1 {
        (*sprite).data[1] *= -1;
        (*sprite).data[0] = 0;
    } else {
        (*sprite).data[0] += 1;
    }
}
unsafe fn ShakeFlashYellow(sprite: *mut Sprite) {
    let array: *mut YellowFlashData = sShakeYellowFlashData[(*sprite).data[3]];
    SetShakeFlashYellowPos(sprite);
    if (*array.at((*sprite).data[6])).time == 255 {
        (*sprite).x2 = 0;
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        if (*sprite).data[4] == 1 {
            if (*array.at((*sprite).data[6])).isYellow != 0 {
                BlendPalette((*sprite).data[7] as u16, 16, 16, 1023);
            } else {
                BlendPalette((*sprite).data[7] as u16, 16, 0, 1023);
            }
            (*sprite).data[4] = 0;
        }
        if (*array.at((*sprite).data[6])).time as i16 == (*sprite).data[5] {
            (*sprite).data[4] = 1;
            (*sprite).data[5] = 0;
            (*sprite).data[6] += 1;
        } else {
            (*sprite).data[5] += 1;
        }
    }
}
pub(crate) unsafe fn Anim_ShakeFlashYellow_Fast(sprite: *mut Sprite) {
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) == 1
    {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[6] = 0;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 0;
        (*sprite).data[3] = 0;
    }
    ShakeFlashYellow(sprite);
}
pub(crate) unsafe fn Anim_ShakeFlashYellow(sprite: *mut Sprite) {
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) == 1
    {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[6] = 0;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 0;
        (*sprite).data[3] = 1;
    }
    ShakeFlashYellow(sprite);
}
pub(crate) unsafe fn Anim_ShakeFlashYellow_Slow(sprite: *mut Sprite) {
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) == 1
    {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[6] = 0;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 0;
        (*sprite).data[3] = 2;
    }
    ShakeFlashYellow(sprite);
}
unsafe fn ShakeGlow_Blend(sprite: *mut Sprite) {
    if (*sprite).data[2] > 127 {
        BlendPalette((*sprite).data[7] as u16, 16, 0, 31);
        (*sprite).callback = Some(WaitAnimEnd);
    } else {
        (*sprite).data[6] = Sin((*sprite).data[2], 12);
        BlendPalette(
            (*sprite).data[7] as u16,
            16,
            (*sprite).data[6] as u8,
            sColors_0[(*sprite).data[1]],
        );
    }
}
unsafe fn ShakeGlow_Move(sprite: *mut Sprite) {
    if (*sprite).data[3] < (*sprite).data[4] {
        TryFlipX(sprite);
        if (*sprite).data[5] > (*sprite).data[0] {
            if ({
                (*sprite).data[3] += 1;
                (*sprite).data[3]
            }) < (*sprite).data[4]
            {
                (*sprite).data[5] = 0;
            }
            (*sprite).x2 = 0;
        } else {
            let sign: i8 = 1 - ((*sprite).data[3] % 2) as i8 * 2;
            (*sprite).x2 = sign as i16
                * Sin(
                    (div_i32((*sprite).data[5] as i32 * 384, (*sprite).data[0] as i32) % 256)
                        as i16,
                    6,
                );
            (*sprite).data[5] += 1;
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe fn Anim_ShakeGlowRed_Fast(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[0] = 10;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 2;
        (*sprite).data[3] = 0;
        (*sprite).data[1] = SHAKEGLOW_RED;
    }
    if (*sprite).data[2] % 2 == 0 {
        ShakeGlow_Blend(sprite);
    }
    if (*sprite).data[2] as i32 >= (128 - (*sprite).data[0] as i32 * (*sprite).data[4] as i32) / 2 {
        ShakeGlow_Move(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_ShakeGlowRed(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[0] = 20;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 1;
        (*sprite).data[3] = 0;
        (*sprite).data[1] = SHAKEGLOW_RED;
    }
    if (*sprite).data[2] % 2 == 0 {
        ShakeGlow_Blend(sprite);
    }
    if (*sprite).data[2] as i32 >= (128 - (*sprite).data[0] as i32 * (*sprite).data[4] as i32) / 2 {
        ShakeGlow_Move(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_ShakeGlowRed_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[0] = 80;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 1;
        (*sprite).data[3] = 0;
        (*sprite).data[1] = SHAKEGLOW_RED;
    }
    if (*sprite).data[2] % 2 == 0 {
        ShakeGlow_Blend(sprite);
    }
    if (*sprite).data[2] as i32 >= (128 - (*sprite).data[0] as i32 * (*sprite).data[4] as i32) / 2 {
        ShakeGlow_Move(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_ShakeGlowGreen_Fast(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[0] = 10;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 2;
        (*sprite).data[3] = 0;
        (*sprite).data[1] = SHAKEGLOW_GREEN;
    }
    if (*sprite).data[2] % 2 == 0 {
        ShakeGlow_Blend(sprite);
    }
    if (*sprite).data[2] as i32 >= (128 - (*sprite).data[0] as i32 * (*sprite).data[4] as i32) / 2 {
        ShakeGlow_Move(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_ShakeGlowGreen(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[0] = 20;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 1;
        (*sprite).data[3] = 0;
        (*sprite).data[1] = SHAKEGLOW_GREEN;
    }
    if (*sprite).data[2] % 2 == 0 {
        ShakeGlow_Blend(sprite);
    }
    if (*sprite).data[2] as i32 >= (128 - (*sprite).data[0] as i32 * (*sprite).data[4] as i32) / 2 {
        ShakeGlow_Move(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_ShakeGlowGreen_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[0] = 80;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 1;
        (*sprite).data[3] = 0;
        (*sprite).data[1] = SHAKEGLOW_GREEN;
    }
    if (*sprite).data[2] % 2 == 0 {
        ShakeGlow_Blend(sprite);
    }
    if (*sprite).data[2] as i32 >= (128 - (*sprite).data[0] as i32 * (*sprite).data[4] as i32) / 2 {
        ShakeGlow_Move(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_ShakeGlowBlue_Fast(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[0] = 10;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 2;
        (*sprite).data[3] = 0;
        (*sprite).data[1] = SHAKEGLOW_BLUE;
    }
    if (*sprite).data[2] % 2 == 0 {
        ShakeGlow_Blend(sprite);
    }
    if (*sprite).data[2] as i32 >= (128 - (*sprite).data[0] as i32 * (*sprite).data[4] as i32) / 2 {
        ShakeGlow_Move(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_ShakeGlowBlue(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[0] = 20;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 1;
        (*sprite).data[3] = 0;
        (*sprite).data[1] = SHAKEGLOW_BLUE;
    }
    if (*sprite).data[2] % 2 == 0 {
        ShakeGlow_Blend(sprite);
    }
    if (*sprite).data[2] as i32 >= (128 - (*sprite).data[0] as i32 * (*sprite).data[4] as i32) / 2 {
        ShakeGlow_Move(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn Anim_ShakeGlowBlue_Slow(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).data[7] = 0x100 + (*sprite).oam.paletteNum() as i16 * 16;
        (*sprite).data[0] = 80;
        (*sprite).data[5] = 0;
        (*sprite).data[4] = 1;
        (*sprite).data[3] = 0;
        (*sprite).data[1] = SHAKEGLOW_BLUE;
    }
    if (*sprite).data[2] % 2 == 0 {
        ShakeGlow_Blend(sprite);
    }
    if (*sprite).data[2] as i32 >= (128 - (*sprite).data[0] as i32 * (*sprite).data[4] as i32) / 2 {
        ShakeGlow_Move(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn WaitAnimEnd(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
