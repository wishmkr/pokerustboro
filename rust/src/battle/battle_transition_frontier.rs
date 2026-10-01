//! Translated from `src/battle_transition_frontier.c` by tools/rustport/c2rs.py.
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
    clippy::too_many_arguments,
    clippy::type_complexity
)]

use crate::battle_transition::GetBg0TilesDst;
use crate::bg::{ChangeBgX, ChangeBgY};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::{ClearGpuRegBits, SetGpuReg, SetGpuRegBits};
use crate::palette::{BeginNormalPaletteFade, LoadPalette, gPaletteFade};
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag};
use crate::task::DestroyTask;
use crate::task::{gTasks, task_get};
use crate::trig::{Cos2, Sin2};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sTargetX: usize = 0;
const tState: usize = 0;
const sTargetY: usize = 1;
const tTimer: usize = 1;
const sAngle: usize = 2;
const sSpeedX: usize = 2;
const tBlend: usize = 2;
const sRotateSpeed: usize = 3;
const sSpeedY: usize = 3;
const tFadeTimer: usize = 3;
const sRadius: usize = 4;
const sTimerX: usize = 4;
const tCircle1SpriteId: usize = 4;
const sTargetRadius: usize = 5;
const sTimerY: usize = 5;
const tCircle2SpriteId: usize = 5;
const sDelayX: usize = 6;
const sRadiusDelta: usize = 6;
const tCircle3SpriteId: usize = 6;
const sDelayY: usize = 7;
// Data tables (translate with cdata.py): sLogoCenter_Gfx sLogoCenter_Tilemap sLogoCircles_Gfx sLogo_Pal sFiller sOamData_LogoCircles sSpriteSheet_LogoCircles sSpritePalette_LogoCircles sAnim_LogoCircle_Top sAnim_LogoCircle_Left sAnim_LogoCircle_Right sAnimTable_LogoCircles sSpriteTemplate_LogoCircles sFrontierCirclesMeet_Funcs sFrontierCirclesCross_Funcs sFrontierCirclesAsymmetricSpiral_Funcs sFrontierCirclesSymmetricSpiral_Funcs sFrontierCirclesMeetInSeq_Funcs sFrontierCirclesCrossInSeq_Funcs sFrontierCirclesAsymmetricSpiralInSeq_Funcs sFrontierCirclesSymmetricSpiralInSeq_Funcs

const PALTAG_LOGO_CIRCLES: u16 = 11920;

static sFrontierCirclesAsymmetricSpiralInSeq_Funcs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 5>> = Table((&raw const crate::data::battle_transition_frontier::sFrontierCirclesAsymmetricSpiralInSeq_Funcs).cast());
static sFrontierCirclesAsymmetricSpiral_Funcs: Table<
    CArray<Option<unsafe fn(*mut Task) -> u8>, 5>,
> = Table(
    (&raw const crate::data::battle_transition_frontier::sFrontierCirclesAsymmetricSpiral_Funcs)
        .cast(),
);
static sFrontierCirclesCrossInSeq_Funcs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 5>> =
    Table(
        (&raw const crate::data::battle_transition_frontier::sFrontierCirclesCrossInSeq_Funcs)
            .cast(),
    );
static sFrontierCirclesCross_Funcs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 5>> =
    Table((&raw const crate::data::battle_transition_frontier::sFrontierCirclesCross_Funcs).cast());
static sFrontierCirclesMeetInSeq_Funcs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 5>> =
    Table(
        (&raw const crate::data::battle_transition_frontier::sFrontierCirclesMeetInSeq_Funcs)
            .cast(),
    );
static sFrontierCirclesMeet_Funcs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 5>> =
    Table((&raw const crate::data::battle_transition_frontier::sFrontierCirclesMeet_Funcs).cast());
static sFrontierCirclesSymmetricSpiralInSeq_Funcs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 5>> = Table((&raw const crate::data::battle_transition_frontier::sFrontierCirclesSymmetricSpiralInSeq_Funcs).cast());
static sFrontierCirclesSymmetricSpiral_Funcs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 5>> =
    Table(
        (&raw const crate::data::battle_transition_frontier::sFrontierCirclesSymmetricSpiral_Funcs)
            .cast(),
    );
static sLogoCenter_Gfx: Table<CArray<u32, 119>> =
    Table((&raw const crate::data::battle_transition_frontier::sLogoCenter_Gfx).cast());
static sLogoCenter_Tilemap: Table<CArray<u32, 107>> =
    Table((&raw const crate::data::battle_transition_frontier::sLogoCenter_Tilemap).cast());
static sLogo_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_transition_frontier::sLogo_Pal).cast());
static sSpritePalette_LogoCircles: Table<SpritePalette> =
    Table((&raw const crate::data::battle_transition_frontier::sSpritePalette_LogoCircles).cast());
static sSpriteSheet_LogoCircles: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::battle_transition_frontier::sSpriteSheet_LogoCircles).cast());
static sSpriteTemplate_LogoCircles: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_transition_frontier::sSpriteTemplate_LogoCircles).cast());

/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompVram(a0 as _, a1 as _);
    }
}

unsafe fn LoadLogoGfx() {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LZ77UnCompVram(sLogoCenter_Gfx.as_ptr().cast_mut(), tileset as *mut c_void);
    LZ77UnCompVram(
        sLogoCenter_Tilemap.as_ptr().cast_mut(),
        tilemap as *mut c_void,
    );
    LoadPalette(sLogo_Pal.as_ptr().cast_mut() as *mut c_void, 240, 32);
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_LogoCircles).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_LogoCircles).cast_mut());
}
unsafe fn CreateSlidingLogoCircleSprite(
    x: i16,
    y: i16,
    delayX: u8,
    delayY: u8,
    speedX: i8,
    speedY: i8,
    spriteAnimNum: u8,
) -> u8 {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_LogoCircles).cast_mut(),
        x,
        y,
        0,
    );
    match spriteAnimNum {
        0 => {
            gSprites[spriteId].data[sTargetX] = 120;
            gSprites[spriteId].data[sTargetY] = 45;
        }
        1 => {
            gSprites[spriteId].data[sTargetX] = 89;
            gSprites[spriteId].data[sTargetY] = 97;
        }
        2 => {
            gSprites[spriteId].data[sTargetX] = 151;
            gSprites[spriteId].data[sTargetY] = 97;
        }
        _ => {}
    }
    gSprites[spriteId].data[sSpeedX] = speedX as i16;
    gSprites[spriteId].data[sSpeedY] = speedY as i16;
    gSprites[spriteId].data[sDelayX] = delayX as i16;
    gSprites[spriteId].data[sDelayY] = delayY as i16;
    gSprites[spriteId].data[sTimerX] = 0;
    gSprites[spriteId].data[sTimerY] = 0;
    StartSpriteAnim(&raw mut gSprites[spriteId], spriteAnimNum);
    gSprites[spriteId].callback = Some(SpriteCB_LogoCircleSlide);
    spriteId
}
pub(crate) unsafe fn SpriteCB_LogoCircleSlide(sprite: *mut Sprite) {
    let data: *mut i16 = (*sprite).data.as_mut_ptr();
    if (*sprite).x == *data && (*sprite).y == *data.at(1) {
        (*sprite).callback = Some(SpriteCallbackDummy);
    } else {
        if *data.at(4) == *data.at(6) {
            (*sprite).x += *data.at(2);
            *data.at(4) = 0;
        } else {
            *data.at(4) += 1;
        }
        if *data.at(5) == *data.at(7) {
            (*sprite).y += *data.at(3);
            *data.at(5) = 0;
        } else {
            *data.at(5) += 1;
        }
    }
}
unsafe fn CreateSpiralingLogoCircleSprite(
    x: i16,
    y: i16,
    angle: i16,
    rotateSpeed: i16,
    radiusStart: i16,
    radiusEnd: i16,
    radiusDelta: i16,
    spriteAnimNum: u8,
) -> u8 {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_LogoCircles).cast_mut(),
        x,
        y,
        0,
    );
    match spriteAnimNum {
        0 => {
            gSprites[spriteId].data[sTargetX] = 120;
            gSprites[spriteId].data[sTargetY] = 45;
        }
        1 => {
            gSprites[spriteId].data[sTargetX] = 89;
            gSprites[spriteId].data[sTargetY] = 97;
        }
        2 => {
            gSprites[spriteId].data[sTargetX] = 151;
            gSprites[spriteId].data[sTargetY] = 97;
        }
        _ => {}
    }
    gSprites[spriteId].data[sAngle] = angle;
    gSprites[spriteId].data[sRotateSpeed] = rotateSpeed;
    gSprites[spriteId].data[sRadius] = radiusStart;
    gSprites[spriteId].data[sTargetRadius] = radiusEnd;
    gSprites[spriteId].data[sRadiusDelta] = radiusDelta;
    StartSpriteAnim(&raw mut gSprites[spriteId], spriteAnimNum);
    gSprites[spriteId].callback = Some(SpriteCB_LogoCircleSpiral);
    spriteId
}
pub(crate) unsafe fn SpriteCB_LogoCircleSpiral(sprite: *mut Sprite) {
    (*sprite).x2 = ((Sin2((*sprite).data[sAngle] as u16) as i32 * (*sprite).data[sRadius] as i32)
        >> 12) as i16;
    (*sprite).y2 = ((Cos2((*sprite).data[sAngle] as u16) as i32 * (*sprite).data[sRadius] as i32)
        >> 12) as i16;
    (*sprite).data[sAngle] =
        (((*sprite).data[sAngle] as i32 + (*sprite).data[sRotateSpeed] as i32) % 360) as i16;
    if (*sprite).data[sRadius] != (*sprite).data[sTargetRadius] {
        (*sprite).data[sRadius] += (*sprite).data[sRadiusDelta];
    } else {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
unsafe fn DestroyLogoCirclesGfx(task: *mut Task) {
    FreeSpriteTilesByTag(PALTAG_LOGO_CIRCLES);
    FreeSpritePaletteByTag(PALTAG_LOGO_CIRCLES);
    DestroySprite(&raw mut gSprites[(*task).data[tCircle1SpriteId]]);
    DestroySprite(&raw mut gSprites[(*task).data[tCircle2SpriteId]]);
    DestroySprite(&raw mut gSprites[(*task).data[tCircle3SpriteId]]);
}
unsafe fn IsLogoCirclesAnimFinished(task: *mut Task) -> u8 {
    if gSprites[(*task).data[tCircle1SpriteId]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && gSprites[(*task).data[tCircle2SpriteId]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && gSprites[(*task).data[tCircle3SpriteId]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Circles_Init(task: *mut Task) -> u8 {
    if (*task).data[tTimer] == 0 {
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN1_ON);
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG0_ON);
        (*task).data[tTimer] += 1;
        return FALSE;
    } else {
        LoadLogoGfx();
        SetGpuReg(REG_OFFSET_BLDCNT, 16193);
        SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
        ChangeBgX(0, 0, BG_COORD_SET);
        ChangeBgY(0, 0, BG_COORD_SET);
        ChangeBgY(0, 0x500, BG_COORD_SUB);
        (*task).data[tTimer] = 0;
        (*task).data[tState] += 1;
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn FadeInCenterLogoCircle(task: *mut Task) -> u8 {
    if (*task).data[tBlend] == 0 {
        SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG0_ON);
    }
    if (*task).data[tBlend] == 16 {
        if (*task).data[tFadeTimer] == 31 {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 0x10, 0);
            (*task).data[tState] += 1;
        } else {
            (*task).data[tFadeTimer] += 1;
        }
    } else {
        (*task).data[tBlend] += 1;
        let blnd: u16 = (*task).data[tBlend] as u16;
        SetGpuReg(REG_OFFSET_BLDALPHA, (16 - blnd) << 8 | blnd);
    }
    FALSE
}
pub(crate) unsafe fn WaitForLogoCirclesAnim(task: *mut Task) -> u8 {
    if IsLogoCirclesAnimFinished(task) == TRUE {
        (*task).data[tState] += 1;
    }
    FALSE
}
pub unsafe fn Task_FrontierCirclesMeet(taskId: u8) {
    while sFrontierCirclesMeet_Funcs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn CirclesMeet_CreateSprites(task: *mut Task) -> u8 {
    (*task).data[tCircle1SpriteId] = CreateSlidingLogoCircleSprite(120, -51, 0, 0, 0, 2, 0) as i16;
    (*task).data[tCircle2SpriteId] = CreateSlidingLogoCircleSprite(-7, 193, 0, 0, 2, -2, 1) as i16;
    (*task).data[tCircle3SpriteId] =
        CreateSlidingLogoCircleSprite(247, 193, 0, 0, -2, -2, 2) as i16;
    (*task).data[tState] += 1;
    FALSE
}
pub(crate) unsafe fn CirclesMeet_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesMeet)));
    }
    FALSE
}
pub unsafe fn Task_FrontierCirclesCross(taskId: u8) {
    while sFrontierCirclesCross_Funcs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn CirclesCross_CreateSprites(task: *mut Task) -> u8 {
    (*task).data[tCircle1SpriteId] = CreateSlidingLogoCircleSprite(120, 197, 0, 0, 0, -4, 0) as i16;
    (*task).data[tCircle2SpriteId] = CreateSlidingLogoCircleSprite(241, 59, 0, 1, -4, 2, 1) as i16;
    (*task).data[tCircle3SpriteId] = CreateSlidingLogoCircleSprite(-1, 59, 0, 1, 4, 2, 2) as i16;
    (*task).data[tState] += 1;
    FALSE
}
pub(crate) unsafe fn CirclesCross_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesCross)));
    }
    FALSE
}
pub unsafe fn Task_FrontierCirclesAsymmetricSpiral(taskId: u8) {
    while sFrontierCirclesAsymmetricSpiral_Funcs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn CirclesAsymmetricSpiral_CreateSprites(task: *mut Task) -> u8 {
    (*task).data[tCircle1SpriteId] =
        CreateSpiralingLogoCircleSprite(120, 45, 12, 4, 128, 0, -4, 0) as i16;
    (*task).data[tCircle2SpriteId] =
        CreateSpiralingLogoCircleSprite(89, 97, 252, 4, 128, 0, -4, 1) as i16;
    (*task).data[tCircle3SpriteId] =
        CreateSpiralingLogoCircleSprite(151, 97, 132, 4, 128, 0, -4, 2) as i16;
    (*task).data[tState] += 1;
    FALSE
}
pub(crate) unsafe fn CirclesAsymmetricSpiral_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesAsymmetricSpiral)));
    }
    FALSE
}
pub unsafe fn Task_FrontierCirclesSymmetricSpiral(taskId: u8) {
    while sFrontierCirclesSymmetricSpiral_Funcs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn CirclesSymmetricSpiral_CreateSprites(task: *mut Task) -> u8 {
    (*task).data[tCircle1SpriteId] =
        CreateSpiralingLogoCircleSprite(120, 80, 284, 8, 131, 35, -3, 0) as i16;
    (*task).data[tCircle2SpriteId] =
        CreateSpiralingLogoCircleSprite(120, 80, 44, 8, 131, 35, -3, 1) as i16;
    (*task).data[tCircle3SpriteId] =
        CreateSpiralingLogoCircleSprite(121, 80, 164, 8, 131, 35, -3, 2) as i16;
    (*task).data[tState] += 1;
    FALSE
}
pub(crate) unsafe fn CirclesSymmetricSpiral_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesSymmetricSpiral)));
    }
    FALSE
}
pub unsafe fn Task_FrontierCirclesMeetInSeq(taskId: u8) {
    while sFrontierCirclesMeetInSeq_Funcs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn CirclesMeetInSeq_CreateSprites(task: *mut Task) -> u8 {
    if (*task).data[tTimer] == 0 {
        (*task).data[tCircle1SpriteId] =
            CreateSlidingLogoCircleSprite(120, -51, 0, 0, 0, 4, 0) as i16;
    } else if (*task).data[tTimer] == 16 {
        (*task).data[tCircle2SpriteId] =
            CreateSlidingLogoCircleSprite(-7, 193, 0, 0, 4, -4, 1) as i16;
    } else if (*task).data[tTimer] == 32 {
        (*task).data[tCircle3SpriteId] =
            CreateSlidingLogoCircleSprite(247, 193, 0, 0, -4, -4, 2) as i16;
        (*task).data[tState] += 1;
    }
    (*task).data[tTimer] += 1;
    FALSE
}
pub(crate) unsafe fn CirclesMeetInSeq_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesMeetInSeq)));
    }
    FALSE
}
pub unsafe fn Task_FrontierCirclesCrossInSeq(taskId: u8) {
    while sFrontierCirclesCrossInSeq_Funcs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn CirclesCrossInSeq_CreateSprites(task: *mut Task) -> u8 {
    if (*task).data[tTimer] == 0 {
        (*task).data[tCircle1SpriteId] =
            CreateSlidingLogoCircleSprite(120, 197, 0, 0, 0, -8, 0) as i16;
    } else if (*task).data[tTimer] == 16 {
        (*task).data[tCircle2SpriteId] =
            CreateSlidingLogoCircleSprite(241, 78, 0, 0, -8, 1, 1) as i16;
    } else if (*task).data[tTimer] == 32 {
        (*task).data[tCircle3SpriteId] =
            CreateSlidingLogoCircleSprite(-1, 78, 0, 0, 8, 1, 2) as i16;
        (*task).data[tState] += 1;
    }
    (*task).data[tTimer] += 1;
    FALSE
}
pub(crate) unsafe fn CirclesCrossInSeq_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesCrossInSeq)));
    }
    FALSE
}
pub unsafe fn Task_FrontierCirclesAsymmetricSpiralInSeq(taskId: u8) {
    while sFrontierCirclesAsymmetricSpiralInSeq_Funcs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn CirclesAsymmetricSpiralInSeq_CreateSprites(task: *mut Task) -> u8 {
    if (*task).data[tTimer] == 0 {
        (*task).data[tCircle1SpriteId] =
            CreateSpiralingLogoCircleSprite(120, 45, 12, 4, 128, 0, -4, 0) as i16;
    } else if (*task).data[tTimer] == 16 {
        (*task).data[tCircle2SpriteId] =
            CreateSpiralingLogoCircleSprite(89, 97, 252, 4, 128, 0, -4, 1) as i16;
    } else if (*task).data[tTimer] == 32 {
        (*task).data[tCircle3SpriteId] =
            CreateSpiralingLogoCircleSprite(151, 97, 132, 4, 128, 0, -4, 2) as i16;
        (*task).data[tState] += 1;
    }
    (*task).data[tTimer] += 1;
    FALSE
}
pub(crate) unsafe fn CirclesAsymmetricSpiralInSeq_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(
            Task_FrontierCirclesAsymmetricSpiralInSeq,
        )));
    }
    FALSE
}
pub unsafe fn Task_FrontierCirclesSymmetricSpiralInSeq(taskId: u8) {
    while sFrontierCirclesSymmetricSpiralInSeq_Funcs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn CirclesSymmetricSpiralInSeq_CreateSprites(task: *mut Task) -> u8 {
    if (*task).data[tTimer] == 0 {
        (*task).data[tCircle1SpriteId] =
            CreateSpiralingLogoCircleSprite(120, 80, 284, 8, 131, 35, -3, 0) as i16;
    } else if (*task).data[tTimer] == 16 {
        (*task).data[tCircle2SpriteId] =
            CreateSpiralingLogoCircleSprite(120, 80, 44, 8, 131, 35, -3, 1) as i16;
    } else if (*task).data[tTimer] == 32 {
        (*task).data[tCircle3SpriteId] =
            CreateSpiralingLogoCircleSprite(121, 80, 164, 8, 131, 35, -3, 2) as i16;
        (*task).data[tState] += 1;
    }
    (*task).data[tTimer] += 1;
    FALSE
}
pub(crate) unsafe fn CirclesSymmetricSpiralInSeq_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(
            Task_FrontierCirclesSymmetricSpiralInSeq,
        )));
    }
    FALSE
}
