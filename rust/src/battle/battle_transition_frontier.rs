//! Translated from `src/battle_transition_frontier.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sLogoCenter_Gfx sLogoCenter_Tilemap sLogoCircles_Gfx sLogo_Pal sFiller sOamData_LogoCircles sSpriteSheet_LogoCircles sSpritePalette_LogoCircles sAnim_LogoCircle_Top sAnim_LogoCircle_Left sAnim_LogoCircle_Right sAnimTable_LogoCircles sSpriteTemplate_LogoCircles sFrontierCirclesMeet_Funcs sFrontierCirclesCross_Funcs sFrontierCirclesAsymmetricSpiral_Funcs sFrontierCirclesSymmetricSpiral_Funcs sFrontierCirclesMeetInSeq_Funcs sFrontierCirclesCrossInSeq_Funcs sFrontierCirclesAsymmetricSpiralInSeq_Funcs sFrontierCirclesSymmetricSpiralInSeq_Funcs

const PALTAG_LOGO_CIRCLES: u16 = 11920;

static sFrontierCirclesAsymmetricSpiralInSeq_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>> = Table((&raw const crate::data::battle_transition_frontier::sFrontierCirclesAsymmetricSpiralInSeq_Funcs).cast());
static sFrontierCirclesAsymmetricSpiral_Funcs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>,
> = Table(
    (&raw const crate::data::battle_transition_frontier::sFrontierCirclesAsymmetricSpiral_Funcs)
        .cast(),
);
static sFrontierCirclesCrossInSeq_Funcs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>,
> = Table(
    (&raw const crate::data::battle_transition_frontier::sFrontierCirclesCrossInSeq_Funcs).cast(),
);
static sFrontierCirclesCross_Funcs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>,
> = Table((&raw const crate::data::battle_transition_frontier::sFrontierCirclesCross_Funcs).cast());
static sFrontierCirclesMeetInSeq_Funcs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>,
> = Table(
    (&raw const crate::data::battle_transition_frontier::sFrontierCirclesMeetInSeq_Funcs).cast(),
);
static sFrontierCirclesMeet_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>> =
    Table((&raw const crate::data::battle_transition_frontier::sFrontierCirclesMeet_Funcs).cast());
static sFrontierCirclesSymmetricSpiralInSeq_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>> = Table((&raw const crate::data::battle_transition_frontier::sFrontierCirclesSymmetricSpiralInSeq_Funcs).cast());
static sFrontierCirclesSymmetricSpiral_Funcs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>,
> = Table(
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

unsafe extern "C" {
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn Cos2(a0: u16) -> i16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetBg0TilesDst(a0: *mut *mut u16, a1: *mut *mut u16);
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn Sin2(a0: u16) -> i16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
}

pub(crate) unsafe extern "C" fn LoadLogoGfx() {
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
pub(crate) unsafe extern "C" fn CreateSlidingLogoCircleSprite(
    x: i16,
    y: i16,
    delayX: u8,
    delayY: u8,
    speedX: i8,
    speedY: i8,
    spriteAnimNum: u8,
) -> u8 {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_LogoCircles).cast_mut(),
        x,
        y,
        0,
    );
    match spriteAnimNum {
        0 => {
            gSprites[spriteId].data[0] = 120;
            gSprites[spriteId].data[1] = 45;
        }
        1 => {
            gSprites[spriteId].data[0] = 89;
            gSprites[spriteId].data[1] = 97;
        }
        2 => {
            gSprites[spriteId].data[0] = 151;
            gSprites[spriteId].data[1] = 97;
        }
        _ => {}
    }
    gSprites[spriteId].data[2] = speedX as i16;
    gSprites[spriteId].data[3] = speedY as i16;
    gSprites[spriteId].data[6] = delayX as i16;
    gSprites[spriteId].data[7] = delayY as i16;
    gSprites[spriteId].data[4] = 0;
    gSprites[spriteId].data[5] = 0;
    StartSpriteAnim(&raw mut gSprites[spriteId], spriteAnimNum);
    gSprites[spriteId].callback = Some(SpriteCB_LogoCircleSlide);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_LogoCircleSlide(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn CreateSpiralingLogoCircleSprite(
    x: i16,
    y: i16,
    angle: i16,
    rotateSpeed: i16,
    radiusStart: i16,
    radiusEnd: i16,
    radiusDelta: i16,
    spriteAnimNum: u8,
) -> u8 {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_LogoCircles).cast_mut(),
        x,
        y,
        0,
    );
    match spriteAnimNum {
        0 => {
            gSprites[spriteId].data[0] = 120;
            gSprites[spriteId].data[1] = 45;
        }
        1 => {
            gSprites[spriteId].data[0] = 89;
            gSprites[spriteId].data[1] = 97;
        }
        2 => {
            gSprites[spriteId].data[0] = 151;
            gSprites[spriteId].data[1] = 97;
        }
        _ => {}
    }
    gSprites[spriteId].data[2] = angle;
    gSprites[spriteId].data[3] = rotateSpeed;
    gSprites[spriteId].data[4] = radiusStart;
    gSprites[spriteId].data[5] = radiusEnd;
    gSprites[spriteId].data[6] = radiusDelta;
    StartSpriteAnim(&raw mut gSprites[spriteId], spriteAnimNum);
    gSprites[spriteId].callback = Some(SpriteCB_LogoCircleSpiral);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_LogoCircleSpiral(sprite: *mut Sprite) {
    (*sprite).x2 = (Sin2((*sprite).data[2] as u16) as i32 * (*sprite).data[4] as i32 >> 12) as i16;
    (*sprite).y2 = (Cos2((*sprite).data[2] as u16) as i32 * (*sprite).data[4] as i32 >> 12) as i16;
    (*sprite).data[2] = (((*sprite).data[2] as i32 + (*sprite).data[3] as i32) % 360) as i16;
    if (*sprite).data[4] != (*sprite).data[5] {
        (*sprite).data[4] += (*sprite).data[6];
    } else {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn DestroyLogoCirclesGfx(task: *mut Task) {
    FreeSpriteTilesByTag(PALTAG_LOGO_CIRCLES);
    FreeSpritePaletteByTag(PALTAG_LOGO_CIRCLES);
    DestroySprite(&raw mut gSprites[(*task).data[4]]);
    DestroySprite(&raw mut gSprites[(*task).data[5]]);
    DestroySprite(&raw mut gSprites[(*task).data[6]]);
}
pub(crate) unsafe extern "C" fn IsLogoCirclesAnimFinished(task: *mut Task) -> u8 {
    if gSprites[(*task).data[4]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        && gSprites[(*task).data[5]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        && gSprites[(*task).data[6]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
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
pub(crate) unsafe extern "C" fn Circles_Init(task: *mut Task) -> u8 {
    if (*task).data[1] == 0 {
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN1_ON);
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG0_ON);
        (*task).data[1] += 1;
        return FALSE;
    } else {
        LoadLogoGfx();
        SetGpuReg(REG_OFFSET_BLDCNT, 16193);
        SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
        ChangeBgX(0, 0, BG_COORD_SET);
        ChangeBgY(0, 0, BG_COORD_SET);
        ChangeBgY(0, 0x500, BG_COORD_SUB);
        (*task).data[1] = 0;
        (*task).data[0] += 1;
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn FadeInCenterLogoCircle(task: *mut Task) -> u8 {
    if (*task).data[2] == 0 {
        SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG0_ON);
    }
    if (*task).data[2] == 16 {
        if (*task).data[3] == 31 {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 0x10, 0);
            (*task).data[0] += 1;
        } else {
            (*task).data[3] += 1;
        }
    } else {
        let mut blnd: u16 = 0;
        (*task).data[2] += 1;
        blnd = (*task).data[2] as u16;
        SetGpuReg(REG_OFFSET_BLDALPHA, 16 - blnd << 8 | blnd);
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaitForLogoCirclesAnim(task: *mut Task) -> u8 {
    if IsLogoCirclesAnimFinished(task) == TRUE {
        (*task).data[0] += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesMeet(taskId: u8) {
    while sFrontierCirclesMeet_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn CirclesMeet_CreateSprites(task: *mut Task) -> u8 {
    (*task).data[4] = CreateSlidingLogoCircleSprite(120, -51, 0, 0, 0, 2, 0) as i16;
    (*task).data[5] = CreateSlidingLogoCircleSprite(-7, 193, 0, 0, 2, -2, 1) as i16;
    (*task).data[6] = CreateSlidingLogoCircleSprite(247, 193, 0, 0, -2, -2, 2) as i16;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn CirclesMeet_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesMeet)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesCross(taskId: u8) {
    while sFrontierCirclesCross_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn CirclesCross_CreateSprites(task: *mut Task) -> u8 {
    (*task).data[4] = CreateSlidingLogoCircleSprite(120, 197, 0, 0, 0, -4, 0) as i16;
    (*task).data[5] = CreateSlidingLogoCircleSprite(241, 59, 0, 1, -4, 2, 1) as i16;
    (*task).data[6] = CreateSlidingLogoCircleSprite(-1, 59, 0, 1, 4, 2, 2) as i16;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn CirclesCross_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesCross)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesAsymmetricSpiral(taskId: u8) {
    while sFrontierCirclesAsymmetricSpiral_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn CirclesAsymmetricSpiral_CreateSprites(task: *mut Task) -> u8 {
    (*task).data[4] = CreateSpiralingLogoCircleSprite(120, 45, 12, 4, 128, 0, -4, 0) as i16;
    (*task).data[5] = CreateSpiralingLogoCircleSprite(89, 97, 252, 4, 128, 0, -4, 1) as i16;
    (*task).data[6] = CreateSpiralingLogoCircleSprite(151, 97, 132, 4, 128, 0, -4, 2) as i16;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn CirclesAsymmetricSpiral_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesAsymmetricSpiral)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesSymmetricSpiral(taskId: u8) {
    while sFrontierCirclesSymmetricSpiral_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn CirclesSymmetricSpiral_CreateSprites(task: *mut Task) -> u8 {
    (*task).data[4] = CreateSpiralingLogoCircleSprite(120, 80, 284, 8, 131, 35, -3, 0) as i16;
    (*task).data[5] = CreateSpiralingLogoCircleSprite(120, 80, 44, 8, 131, 35, -3, 1) as i16;
    (*task).data[6] = CreateSpiralingLogoCircleSprite(121, 80, 164, 8, 131, 35, -3, 2) as i16;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn CirclesSymmetricSpiral_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesSymmetricSpiral)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesMeetInSeq(taskId: u8) {
    while sFrontierCirclesMeetInSeq_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn CirclesMeetInSeq_CreateSprites(task: *mut Task) -> u8 {
    if (*task).data[1] == 0 {
        (*task).data[4] = CreateSlidingLogoCircleSprite(120, -51, 0, 0, 0, 4, 0) as i16;
    } else if (*task).data[1] == 16 {
        (*task).data[5] = CreateSlidingLogoCircleSprite(-7, 193, 0, 0, 4, -4, 1) as i16;
    } else if (*task).data[1] == 32 {
        (*task).data[6] = CreateSlidingLogoCircleSprite(247, 193, 0, 0, -4, -4, 2) as i16;
        (*task).data[0] += 1;
    }
    (*task).data[1] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn CirclesMeetInSeq_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesMeetInSeq)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesCrossInSeq(taskId: u8) {
    while sFrontierCirclesCrossInSeq_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn CirclesCrossInSeq_CreateSprites(task: *mut Task) -> u8 {
    if (*task).data[1] == 0 {
        (*task).data[4] = CreateSlidingLogoCircleSprite(120, 197, 0, 0, 0, -8, 0) as i16;
    } else if (*task).data[1] == 16 {
        (*task).data[5] = CreateSlidingLogoCircleSprite(241, 78, 0, 0, -8, 1, 1) as i16;
    } else if (*task).data[1] == 32 {
        (*task).data[6] = CreateSlidingLogoCircleSprite(-1, 78, 0, 0, 8, 1, 2) as i16;
        (*task).data[0] += 1;
    }
    (*task).data[1] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn CirclesCrossInSeq_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierCirclesCrossInSeq)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesAsymmetricSpiralInSeq(taskId: u8) {
    while sFrontierCirclesAsymmetricSpiralInSeq_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn CirclesAsymmetricSpiralInSeq_CreateSprites(task: *mut Task) -> u8 {
    if (*task).data[1] == 0 {
        (*task).data[4] = CreateSpiralingLogoCircleSprite(120, 45, 12, 4, 128, 0, -4, 0) as i16;
    } else if (*task).data[1] == 16 {
        (*task).data[5] = CreateSpiralingLogoCircleSprite(89, 97, 252, 4, 128, 0, -4, 1) as i16;
    } else if (*task).data[1] == 32 {
        (*task).data[6] = CreateSpiralingLogoCircleSprite(151, 97, 132, 4, 128, 0, -4, 2) as i16;
        (*task).data[0] += 1;
    }
    (*task).data[1] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn CirclesAsymmetricSpiralInSeq_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(
            Task_FrontierCirclesAsymmetricSpiralInSeq,
        )));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FrontierCirclesSymmetricSpiralInSeq(taskId: u8) {
    while sFrontierCirclesSymmetricSpiralInSeq_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn CirclesSymmetricSpiralInSeq_CreateSprites(task: *mut Task) -> u8 {
    if (*task).data[1] == 0 {
        (*task).data[4] = CreateSpiralingLogoCircleSprite(120, 80, 284, 8, 131, 35, -3, 0) as i16;
    } else if (*task).data[1] == 16 {
        (*task).data[5] = CreateSpiralingLogoCircleSprite(120, 80, 44, 8, 131, 35, -3, 1) as i16;
    } else if (*task).data[1] == 32 {
        (*task).data[6] = CreateSpiralingLogoCircleSprite(121, 80, 164, 8, 131, 35, -3, 2) as i16;
        (*task).data[0] += 1;
    }
    (*task).data[1] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn CirclesSymmetricSpiralInSeq_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        DestroyLogoCirclesGfx(task);
        DestroyTask(FindTaskIdByFunc(Some(
            Task_FrontierCirclesSymmetricSpiralInSeq,
        )));
    }
    return FALSE;
}
