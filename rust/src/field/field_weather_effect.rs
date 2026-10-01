//! Translated from `src/field_weather_effect.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_labels
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_object_movement::SetSpritePosToMapCoords;
use crate::field_weather::{
    ApplyWeatherColorMapIfIdle, ApplyWeatherColorMapIfIdle_Gradual, DroughtStateInit,
    DroughtStateRun, LoadCustomWeatherSpritePalette, LoadDroughtWeatherPalettes,
    ResetDroughtWeatherPaletteLoading, SetCurrentAndNextWeather, SetNextWeather,
    SetRainStrengthFromSoundEffect, Weather_SetBlendCoeffs, Weather_SetTargetBlendCoeffs,
    Weather_UpdateBlend,
};
use crate::fieldmap::gMapHeader;
use crate::gpu_regs::SetGpuReg;
use crate::load_save::gSaveBlock1Ptr;
use crate::overworld::IncrementGameStat;
use crate::random::Random;
use crate::script::ScriptContext_Enable;
use crate::sound::{IsSEPlaying, PlaySE};
use crate::sprite::gSprites;
use crate::sprite::{FreeSpriteTilesByTag, gSpriteCoordOffsetX, gSpriteCoordOffsetY};
use crate::task::DestroyTask;
use crate::task::gTasks;
#[allow(unused_imports)]
use crate::types::*;
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
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `CreateSpriteAtEnd` with this module's view of its types.
#[inline]
unsafe fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSpriteAtEnd(a0 as _, a1, a2, a3) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
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
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const tOffsetY: usize = 0;
const tRadius: usize = 0;
const tScrollXCounter: usize = 0;
const tBlendY: usize = 1;
const tCounterY: usize = 1;
const tDeltaY: usize = 1;
const tRandom: usize = 1;
const tScrollXDir: usize = 1;
const tBlendDelay: usize = 2;
const tPosX: usize = 2;
const tRadiusCounter: usize = 2;
const tWaveDelta: usize = 2;
const tEntranceDelay: usize = 3;
const tWinRange: usize = 3;
const tSnowflakeId: usize = 4;
const tActive: usize = 5;
const tFallCounter: usize = 5;
const tFallDuration: usize = 6;
const tWaiting: usize = 6;
const tDeltaY2: usize = 7;
// Data tables (translate with cdata.py): gCloudsWeatherPalette gSandstormWeatherPalette gWeatherFogDiagonalTiles gWeatherFogHorizontalTiles gWeatherCloudTiles gWeatherSnow1Tiles gWeatherSnow2Tiles gWeatherBubbleTiles gWeatherAshTiles gWeatherRainTiles gWeatherSandstormTiles sCloudSpriteMapCoords sCloudSpriteSheet sCloudSpriteOamData sCloudSpriteAnimCmd sCloudSpriteAnimCmds sCloudSpriteTemplate sRainSpriteCoords sRainSpriteOamData sRainSpriteFallAnimCmd sRainSpriteSplashAnimCmd sRainSpriteHeavySplashAnimCmd sRainSpriteAnimCmds sRainSpriteTemplate sRainSpriteMovement sRainSpriteFallingDurations sRainSpriteSheet sSnowflakeSpriteOamData sSnowflakeSpriteImages sSnowflakeAnimCmd0 sSnowflakeAnimCmd1 sSnowflakeAnimCmds sSnowflakeSpriteTemplate sUnusedData sOamData_FogH sAnim_FogH_0 sAnim_FogH_1 sAnim_FogH_2 sAnim_FogH_3 sAnim_FogH_4 sAnim_FogH_5 sAnims_FogH sAffineAnim_FogH sAffineAnims_FogH sFogHorizontalSpriteTemplate sAshSpriteSheet sAshSpriteOamData sAshSpriteAnimCmd0 sAshSpriteAnimCmds sAshSpriteTemplate sFogDiagonalSpriteSheet sFogDiagonalSpriteOamData sFogDiagonalSpriteAnimCmd0 sFogDiagonalSpriteAnimCmds sFogDiagonalSpriteTemplate sSandstormSpriteOamData sSandstormSpriteAnimCmd0 sSandstormSpriteAnimCmd1 sSandstormSpriteAnimCmds sSandstormSpriteTemplate sSandstormSpriteSheet sSwirlEntranceDelays sBubbleStartDelays sWeatherBubbleSpriteSheet sBubbleStartCoords sBubbleSpriteAnimCmd0 sBubbleSpriteAnimCmds sBubbleSpriteTemplate sWeatherCycleRoute119 sWeatherCycleRoute123

const MIN_SANDSTORM_WAVE_INDEX: u16 = 32;
const THUNDER_STATE_CREATE_RAIN: u16 = 1;
const THUNDER_STATE_END_BOLT_LONG: u16 = 14;
const THUNDER_STATE_FADE_BOLT_LONG: u16 = 13;
const THUNDER_STATE_INIT_BOLT_LONG: u16 = 11;
const THUNDER_STATE_INIT_CYCLE_1: u16 = 6;
const THUNDER_STATE_INIT_CYCLE_2: u16 = 7;
const THUNDER_STATE_INIT_RAIN: u16 = 2;
const THUNDER_STATE_LOAD_RAIN: u16 = 0;
const THUNDER_STATE_NEW_CYCLE: u16 = 4;
const THUNDER_STATE_NEW_CYCLE_WAIT: u16 = 5;
const THUNDER_STATE_SHORT_BOLT: u16 = 8;
const THUNDER_STATE_TRY_NEW_BOLT: u16 = 9;
const THUNDER_STATE_WAIT_BOLT_LONG: u16 = 12;
const THUNDER_STATE_WAIT_BOLT_SHORT: u16 = 10;
const THUNDER_STATE_WAIT_CHANGE: u16 = 3;
const WEATHER_CYCLE_LENGTH: i32 = 4;

static gCloudsWeatherPalette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::field_weather_effect::gCloudsWeatherPalette).cast());
static gSandstormWeatherPalette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::field_weather_effect::gSandstormWeatherPalette).cast());
static gWeatherFogHorizontalTiles: Table<CArray<u8, 2048>> =
    Table((&raw const crate::data::field_weather_effect::gWeatherFogHorizontalTiles).cast());
static sAshSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::field_weather_effect::sAshSpriteSheet).cast());
static sAshSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_weather_effect::sAshSpriteTemplate).cast());
static sBubbleSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_weather_effect::sBubbleSpriteTemplate).cast());
static sBubbleStartCoords: Table<CArray<CArray<i16, 2>, 13>> =
    Table((&raw const crate::data::field_weather_effect::sBubbleStartCoords).cast());
static sBubbleStartDelays: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::field_weather_effect::sBubbleStartDelays).cast());
static sCloudSpriteMapCoords: Table<CArray<Coords16, 3>> =
    Table((&raw const crate::data::field_weather_effect::sCloudSpriteMapCoords).cast());
static sCloudSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::field_weather_effect::sCloudSpriteSheet).cast());
static sCloudSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_weather_effect::sCloudSpriteTemplate).cast());
static sFogDiagonalSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::field_weather_effect::sFogDiagonalSpriteSheet).cast());
static sFogDiagonalSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_weather_effect::sFogDiagonalSpriteTemplate).cast());
static sFogHorizontalSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_weather_effect::sFogHorizontalSpriteTemplate).cast());
static sRainSpriteCoords: Table<CArray<Coords16, 24>> =
    Table((&raw const crate::data::field_weather_effect::sRainSpriteCoords).cast());
static sRainSpriteFallingDurations: Table<CArray<CArray<u16, 2>, 2>> =
    Table((&raw const crate::data::field_weather_effect::sRainSpriteFallingDurations).cast());
static sRainSpriteMovement: Table<CArray<CArray<i16, 2>, 2>> =
    Table((&raw const crate::data::field_weather_effect::sRainSpriteMovement).cast());
static sRainSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::field_weather_effect::sRainSpriteSheet).cast());
static sRainSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_weather_effect::sRainSpriteTemplate).cast());
static sSandstormSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::field_weather_effect::sSandstormSpriteSheet).cast());
static sSandstormSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_weather_effect::sSandstormSpriteTemplate).cast());
static sSnowflakeSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_weather_effect::sSnowflakeSpriteTemplate).cast());
static sSwirlEntranceDelays: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::field_weather_effect::sSwirlEntranceDelays).cast());
static sWeatherBubbleSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::field_weather_effect::sWeatherBubbleSpriteSheet).cast());
static sWeatherCycleRoute119: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::field_weather_effect::sWeatherCycleRoute119).cast());
static sWeatherCycleRoute123: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::field_weather_effect::sWeatherCycleRoute123).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sCurrentAbnormalWeather: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sUnusedWeatherRelated: crate::global::Global<u16> = crate::global::Global::new(0);

pub unsafe fn Clouds_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep = 0;
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .cloudSpritesCreated
        == FALSE
    {
        Weather_SetBlendCoeffs(0, 16);
    }
}
pub unsafe fn Clouds_InitAll() {
    Clouds_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == FALSE
    {
        Clouds_Main();
    }
}
pub unsafe fn Clouds_Main() {
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep
    {
        0 => {
            CreateCloudSprites();
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        1 => {
            Weather_SetTargetBlendCoeffs(12, 8, 1);
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        2 if Weather_UpdateBlend() != 0 => {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .weatherGfxLoaded = TRUE;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        _ => {}
    }
}
pub unsafe fn Clouds_Finish() -> u8 {
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .finishStep
    {
        0 => {
            Weather_SetTargetBlendCoeffs(0, 16, 1);
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .finishStep += 1;
            return TRUE;
        }
        1 => {
            if Weather_UpdateBlend() != 0 {
                DestroyCloudSprites();
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
            }
            return TRUE;
        }
        _ => {}
    }
    FALSE
}
pub unsafe fn Sunny_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
}
pub unsafe fn Sunny_InitAll() {
    Sunny_InitVars();
}
pub fn Sunny_Main() {}
pub fn Sunny_Finish() -> u8 {
    FALSE
}
pub(crate) unsafe fn CreateCloudSprites() {
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .cloudSpritesCreated
        == TRUE
    {
        return;
    }
    LoadSpriteSheet((&raw const *sCloudSpriteSheet).cast_mut());
    LoadCustomWeatherSpritePalette(gCloudsWeatherPalette.as_ptr().cast_mut());
    for i in 0..NUM_CLOUD_SPRITES {
        spriteId = CreateSprite((&raw const *sCloudSpriteTemplate).cast_mut(), 0, 0, 0xFF);
        if spriteId != MAX_SPRITES {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s1
                .cloudSprites[i] = &raw mut gSprites[spriteId];
            sprite = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                .cast::<*mut Weather>()))
            .sprites
            .s1
            .cloudSprites[i];
            SetSpritePosToMapCoords(
                sCloudSpriteMapCoords[i].x + MAP_OFFSET as i16,
                sCloudSpriteMapCoords[i].y + MAP_OFFSET as i16,
                &raw mut (*sprite).x,
                &raw mut (*sprite).y,
            );
            (*sprite).set_coordOffsetEnabled(TRUE as u16);
        } else {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s1
                .cloudSprites[i] = null_mut();
        }
    }
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .cloudSpritesCreated = TRUE;
}
unsafe fn DestroyCloudSprites() {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .cloudSpritesCreated
        == 0
    {
        return;
    }
    for i in 0..NUM_CLOUD_SPRITES {
        if !(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sprites
            .s1
            .cloudSprites[i]
            .is_null()
        {
            DestroySprite(
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s1
                    .cloudSprites[i],
            );
        }
    }
    FreeSpriteTilesByTag(GFXTAG_CLOUD);
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .cloudSpritesCreated = FALSE;
}
pub(crate) unsafe fn UpdateCloudSprite(sprite: *mut Sprite) {
    (*sprite).data[0] = ((*sprite).data[0] + 1) & 1;
    if (*sprite).data[0] != 0 {
        (*sprite).x -= 1;
    }
}
pub unsafe fn Drought_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 0;
}
pub unsafe fn Drought_InitAll() {
    Drought_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == FALSE
    {
        Drought_Main();
    }
}
pub unsafe fn Drought_Main() {
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep
    {
        0 => {
            if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .palProcessingState
                != WEATHER_PAL_STATE_CHANGING_WEATHER
            {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
        }
        1 => {
            ResetDroughtWeatherPaletteLoading();
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        2 => {
            if LoadDroughtWeatherPalettes() == FALSE {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
        }
        3 => {
            DroughtStateInit();
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        4 => {
            DroughtStateRun();
            if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .droughtBrightnessStage
                == 6
            {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .weatherGfxLoaded = TRUE;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
        }
        _ => {
            DroughtStateRun();
        }
    }
}
pub fn Drought_Finish() -> u8 {
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn StartDroughtWeatherBlend() {
    CreateTask(Some(UpdateDroughtBlend), 80);
}
pub(crate) unsafe fn UpdateDroughtBlend(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    'l1: {
        let sw1: i16 = (*task).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*task).data[tBlendY] = 0;
            (*task).data[tBlendDelay] = 0;
            (*task).data[tWinRange] = (67108936_usize as *mut u16).read_volatile() as i16;
            SetGpuReg(REG_OFFSET_WININ, 16191);
            SetGpuReg(REG_OFFSET_BLDCNT, 158);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            (*task).data[0] += 1;
        }
        if fall || sw1 == 1 {
            (*task).data[tBlendY] += 3;
            if (*task).data[tBlendY] > 16 {
                (*task).data[tBlendY] = 16;
            }
            SetGpuReg(REG_OFFSET_BLDY, (*task).data[tBlendY] as u16);
            if (*task).data[tBlendY] >= 16 {
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            (*task).data[tBlendDelay] += 1;
            if (*task).data[tBlendDelay] > 9 {
                (*task).data[tBlendDelay] = 0;
                (*task).data[tBlendY] -= 1;
                if (*task).data[tBlendY] <= 0 {
                    (*task).data[tBlendY] = 0;
                    (*task).data[0] += 1;
                }
                SetGpuReg(REG_OFFSET_BLDY, (*task).data[tBlendY] as u16);
            }
            break 'l1;
        }
        if sw1 == 3 {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_WININ, (*task).data[tWinRange] as u16);
            (*task).data[0] += 1;
            break 'l1;
        }
        if sw1 == 4 {
            ScriptContext_Enable();
            DestroyTask(taskId);
            break 'l1;
        }
    }
}
pub unsafe fn Rain_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .rainSpriteVisibleCounter = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .rainSpriteVisibleDelay = 8;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).isDownpour =
        FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetRainSpriteCount = 10;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 3;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
    SetRainStrengthFromSoundEffect(SE_RAIN);
}
pub unsafe fn Rain_InitAll() {
    Rain_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == 0
    {
        Rain_Main();
    }
}
pub unsafe fn Rain_Main() {
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep
    {
        0 => {
            LoadRainSpriteSheet();
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        1 => {
            if CreateRainSprite() == 0 {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
        }
        2 if UpdateVisibleRainSprites() == 0 => {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .weatherGfxLoaded = TRUE;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        _ => {}
    }
}
pub unsafe fn Rain_Finish() -> u8 {
    'l1: {
        let sw1: u16 = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .finishStep;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .nextWeather
                == WEATHER_RAIN
                || (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .nextWeather
                    == WEATHER_RAIN_THUNDERSTORM
                || (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .nextWeather
                    == WEATHER_DOWNPOUR
            {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .finishStep = 0xFF;
                return FALSE;
            } else {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .targetRainSpriteCount = 0;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
            }
        }
        if fall || sw1 == 1 {
            if UpdateVisibleRainSprites() == 0 {
                DestroyRainSprites();
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
                return FALSE;
            }
            return TRUE;
        }
    }
    FALSE
}
unsafe fn StartRainSpriteFall(sprite: *mut Sprite) {
    if (*sprite).data[tRandom] == 0 {
        (*sprite).data[tRandom] = 361;
    }
    let rand: u32 = 0x41c64e6d * (*sprite).data[tRandom] as u32 + 12345;
    (*sprite).data[tRandom] = (((rand & 0x7FFF0000) >> 16) % 600) as i16;
    let numFallingFrames: u16 = sRainSpriteFallingDurations
        [(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .isDownpour][0];
    let tileX: i32 = ((*sprite).data[tRandom] % 30) as i32;
    (*sprite).data[tPosX] = tileX as i16 * 8;
    let tileY: i32 = ((*sprite).data[tRandom] / 30) as i32;
    (*sprite).data[3] = tileY as i16 * 8;
    (*sprite).data[tPosX] = tileX as i16;
    (*sprite).data[tPosX] <<= 7;
    (*sprite).data[3] = tileY as i16;
    (*sprite).data[3] <<= 7;
    (*sprite).data[tPosX] -= sRainSpriteMovement
        [(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .isDownpour][0]
        * numFallingFrames as i16;
    (*sprite).data[3] -= sRainSpriteMovement
        [(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .isDownpour][1]
        * numFallingFrames as i16;
    StartSpriteAnim(sprite, 0);
    (*sprite).data[4] = 0;
    (*sprite).set_coordOffsetEnabled(FALSE as u16);
    (*sprite).data[0] = numFallingFrames as i16;
}
pub(crate) unsafe fn UpdateRainSprite(sprite: *mut Sprite) {
    if (*sprite).data[4] == 0 {
        (*sprite).data[tPosX] += sRainSpriteMovement
            [(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .isDownpour][0];
        (*sprite).data[3] += sRainSpriteMovement
            [(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .isDownpour][1];
        (*sprite).x = (*sprite).data[tPosX] >> 4;
        (*sprite).y = (*sprite).data[3] >> 4;
        if (*sprite).data[tActive] != 0
            && ((*sprite).x >= -8 && (*sprite).x <= 248)
            && (*sprite).y >= -16
            && (*sprite).y <= 176
        {
            (*sprite).set_invisible(FALSE as u16);
        } else {
            (*sprite).set_invisible(TRUE as u16);
        }
        if ({
            (*sprite).data[0] -= 1;
            (*sprite).data[0]
        }) == 0
        {
            StartSpriteAnim(
                sprite,
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .isDownpour
                    + 1,
            );
            (*sprite).data[4] = 1;
            (*sprite).x -= gSpriteCoordOffsetX;
            (*sprite).y -= gSpriteCoordOffsetY;
            (*sprite).set_coordOffsetEnabled(TRUE as u16);
        }
    } else if (*sprite).animEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
        StartRainSpriteFall(sprite);
    }
}
pub(crate) unsafe fn WaitRainSprite(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        StartRainSpriteFall(sprite);
        (*sprite).callback = Some(UpdateRainSprite);
    } else {
        (*sprite).data[0] -= 1;
    }
}
unsafe fn InitRainSpriteMovement(sprite: *mut Sprite, val: u16) {
    let numFallingFrames: u16 = sRainSpriteFallingDurations
        [(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .isDownpour][0];
    let mut numAdvanceRng: u16 = div_i32(
        val as i32,
        sRainSpriteFallingDurations[(*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .isDownpour][1] as i32
            + numFallingFrames as i32,
    ) as u16;
    let mut frameVal: u16 = rem_i32(
        val as i32,
        sRainSpriteFallingDurations[(*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .isDownpour][1] as i32
            + numFallingFrames as i32,
    ) as u16;
    while ({
        numAdvanceRng -= 1;
        numAdvanceRng
    }) != 0xFFFF
    {
        StartRainSpriteFall(sprite);
    }
    if frameVal < numFallingFrames {
        while ({
            frameVal -= 1;
            frameVal
        }) != 0xFFFF
        {
            UpdateRainSprite(sprite);
        }
        (*sprite).data[tWaiting] = 0;
    } else {
        (*sprite).data[0] = frameVal as i16 - numFallingFrames as i16;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).data[tWaiting] = 1;
    }
}
unsafe fn LoadRainSpriteSheet() {
    LoadSpriteSheet((&raw const *sRainSpriteSheet).cast_mut());
}
unsafe fn CreateRainSprite() -> u8 {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .rainSpriteCount
        == MAX_RAIN_SPRITES
    {
        return FALSE;
    }
    let spriteIndex: u8 = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
        .cast::<*mut Weather>()))
    .rainSpriteCount;
    let spriteId: u8 = CreateSpriteAtEnd(
        (&raw const *sRainSpriteTemplate).cast_mut(),
        sRainSpriteCoords[spriteIndex].x,
        sRainSpriteCoords[spriteIndex].y,
        78,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[tActive] = FALSE as i16;
        gSprites[spriteId].data[tRandom] = spriteIndex as i16 * 145;
        while gSprites[spriteId].data[tRandom] >= 600 {
            gSprites[spriteId].data[tRandom] -= 600;
        }
        StartRainSpriteFall(&raw mut gSprites[spriteId]);
        InitRainSpriteMovement(&raw mut gSprites[spriteId], spriteIndex as u16 * 9);
        gSprites[spriteId].set_invisible(TRUE as u16);
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sprites
            .s1
            .rainSprites[spriteIndex] = &raw mut gSprites[spriteId];
    } else {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sprites
            .s1
            .rainSprites[spriteIndex] = null_mut();
    }
    if ({
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .rainSpriteCount += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .rainSpriteCount
    }) == MAX_RAIN_SPRITES
    {
        for i in 0..(MAX_RAIN_SPRITES as u16) {
            if !(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s1
                .rainSprites[i]
                .is_null()
            {
                if (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s1
                .rainSprites[i])
                    .data[tWaiting]
                    == 0
                {
                    (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s1
                    .rainSprites[i])
                        .callback = Some(UpdateRainSprite);
                } else {
                    (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s1
                    .rainSprites[i])
                        .callback = Some(WaitRainSprite);
                }
            }
        }
        return FALSE;
    }
    TRUE
}
unsafe fn UpdateVisibleRainSprites() -> u8 {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .curRainSpriteIndex
        == (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .targetRainSpriteCount
    {
        return FALSE;
    }
    if ({
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .rainSpriteVisibleCounter += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .rainSpriteVisibleCounter
    }) > (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .rainSpriteVisibleDelay as u16
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .rainSpriteVisibleCounter = 0;
        if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .curRainSpriteIndex
            < (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .targetRainSpriteCount
        {
            (*(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s1
                .rainSprites[{
                let t2 = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .curRainSpriteIndex;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .curRainSpriteIndex += 1;
                t2
            }])
            .data[tActive] = TRUE as i16;
        } else {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .curRainSpriteIndex -= 1;
            (*(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s1
                .rainSprites[(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                .cast::<*mut Weather>()))
            .curRainSpriteIndex])
                .data[tActive] = FALSE as i16;
            (*(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s1
                .rainSprites[(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                .cast::<*mut Weather>()))
            .curRainSpriteIndex])
                .set_invisible(TRUE as u16);
        }
    }
    TRUE
}
unsafe fn DestroyRainSprites() {
    let mut i: u16 = 0;
    while i
        < (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .rainSpriteCount as u16
    {
        if !(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sprites
            .s1
            .rainSprites[i]
            .is_null()
        {
            DestroySprite(
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s1
                    .rainSprites[i],
            );
        }
        i += 1;
    }
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .rainSpriteCount = 0;
    FreeSpriteTilesByTag(GFXTAG_RAIN);
}
pub unsafe fn Snow_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 3;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetSnowflakeSpriteCount = NUM_SNOWFLAKE_SPRITES;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .snowflakeVisibleCounter = 0;
}
pub unsafe fn Snow_InitAll() {
    let mut i: u16 = 0;
    Snow_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == FALSE
    {
        Snow_Main();
        i = 0;
        while i
            < (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .snowflakeSpriteCount as u16
        {
            UpdateSnowflakeSprite(
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s1
                    .snowflakeSprites[i],
            );
            i += 1;
        }
    }
}
pub unsafe fn Snow_Main() {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep
        == 0
        && UpdateVisibleSnowflakeSprites() == 0
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .weatherGfxLoaded = TRUE;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .initStep += 1;
    }
}
pub unsafe fn Snow_Finish() -> u8 {
    'l1: {
        let sw1: u16 = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .finishStep;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .targetSnowflakeSpriteCount = 0;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .snowflakeVisibleCounter = 0;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .finishStep += 1;
        }
        if fall || sw1 == 1 {
            if UpdateVisibleSnowflakeSprites() == 0 {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
                return FALSE;
            }
            return TRUE;
        }
    }
    FALSE
}
unsafe fn UpdateVisibleSnowflakeSprites() -> u8 {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .snowflakeSpriteCount
        == (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .targetSnowflakeSpriteCount
    {
        return FALSE;
    }
    if ({
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .snowflakeVisibleCounter += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .snowflakeVisibleCounter
    }) > 36
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .snowflakeVisibleCounter = 0;
        if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .snowflakeSpriteCount
            < (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .targetSnowflakeSpriteCount
        {
            CreateSnowflakeSprite();
        } else {
            DestroySnowflakeSprite();
        }
    }
    ((*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .snowflakeSpriteCount
        != (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .targetSnowflakeSpriteCount) as u8
}
unsafe fn CreateSnowflakeSprite() -> u8 {
    let spriteId: u8 =
        CreateSpriteAtEnd((&raw const *sSnowflakeSpriteTemplate).cast_mut(), 0, 0, 78);
    if spriteId == MAX_SPRITES {
        return FALSE;
    }
    gSprites[spriteId].data[tSnowflakeId] =
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .snowflakeSpriteCount as i16;
    InitSnowflakeSpriteMovement(&raw mut gSprites[spriteId]);
    gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sprites
        .s1
        .snowflakeSprites[{
        let t1 = (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .snowflakeSpriteCount;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .snowflakeSpriteCount += 1;
        t1
    }] = &raw mut gSprites[spriteId];
    TRUE
}
unsafe fn DestroySnowflakeSprite() -> u8 {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .snowflakeSpriteCount
        != 0
    {
        DestroySprite(
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s1
                .snowflakeSprites[{
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .snowflakeSpriteCount -= 1;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .snowflakeSpriteCount
            }],
        );
        return TRUE;
    }
    FALSE
}
unsafe fn InitSnowflakeSpriteMovement(sprite: *mut Sprite) {
    let x: u16 =
        (((*sprite).data[tSnowflakeId] as u16 * 5) & 7) * 30 + (Random() as i32 % 30) as u16;
    (*sprite).y = -3 - (gSpriteCoordOffsetY + (*sprite).centerToCornerVecY as i16);
    (*sprite).x = x as i16 - (gSpriteCoordOffsetX + (*sprite).centerToCornerVecX as i16);
    (*sprite).data[0] = (*sprite).y * 128;
    (*sprite).x2 = 0;
    let rand: u16 = Random();
    (*sprite).data[tDeltaY] = (rand as i16 & 3) * 5 + 64;
    (*sprite).data[tDeltaY2] = (*sprite).data[tDeltaY];
    StartSpriteAnim(sprite, (if rand as i32 & 1 != 0 { 0 } else { 1 }) as u8);
    (*sprite).data[3] = 0;
    (*sprite).data[tWaveDelta] = (if rand as i32 & 3 == 0 { 2 } else { 1 }) as i16;
    (*sprite).data[tFallDuration] = (rand as i16 & 0x1F) + 210;
    (*sprite).data[tFallCounter] = 0;
}
pub(crate) unsafe fn WaitSnowflakeSprite(sprite: *mut Sprite) {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .snowflakeTimer
        > 18
    {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).callback = Some(UpdateSnowflakeSprite);
        (*sprite).y = 250 - (gSpriteCoordOffsetY + (*sprite).centerToCornerVecY as i16);
        (*sprite).data[0] = (*sprite).y * 128;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .snowflakeTimer = 0;
    }
}
pub(crate) unsafe fn UpdateSnowflakeSprite(sprite: *mut Sprite) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    (*sprite).data[0] += (*sprite).data[tDeltaY];
    (*sprite).y = (*sprite).data[0] >> 7;
    (*sprite).data[3] += (*sprite).data[tWaveDelta];
    (*sprite).data[3] &= 0xFF;
    (*sprite).x2 =
        (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[(*sprite).data[3]] / 64;
    x = ((*sprite).x + (*sprite).centerToCornerVecX as i16 + gSpriteCoordOffsetX) & 0x1FF;
    if x as i32 & 0x100 != 0 {
        x |= -256;
    }
    if x < -3 {
        (*sprite).x = 242 - (gSpriteCoordOffsetX + (*sprite).centerToCornerVecX as i16);
    } else if x > 242 {
        (*sprite).x = -3 - (gSpriteCoordOffsetX + (*sprite).centerToCornerVecX as i16);
    }
    y = ((*sprite).y + (*sprite).centerToCornerVecY as i16 + gSpriteCoordOffsetY) & 0xFF;
    if y > 163 && y < 171 {
        (*sprite).y = 250 - (gSpriteCoordOffsetY + (*sprite).centerToCornerVecY as i16);
        (*sprite).data[0] = (*sprite).y * 128;
        (*sprite).data[tFallCounter] = 0;
        (*sprite).data[tFallDuration] = 220;
    } else if y > 242 && y < 250 {
        (*sprite).y = 163;
        (*sprite).data[0] = (*sprite).y * 128;
        (*sprite).data[tFallCounter] = 0;
        (*sprite).data[tFallDuration] = 220;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(WaitSnowflakeSprite);
    }
    if ({
        (*sprite).data[tFallCounter] += 1;
        (*sprite).data[tFallCounter]
    }) == (*sprite).data[tFallDuration]
    {
        InitSnowflakeSpriteMovement(sprite);
        (*sprite).y = 250;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(WaitSnowflakeSprite);
    }
}
pub unsafe fn Thunderstorm_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep =
        THUNDER_STATE_LOAD_RAIN;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .rainSpriteVisibleCounter = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .rainSpriteVisibleDelay = 4;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).isDownpour =
        FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetRainSpriteCount = 16;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 3;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .thunderEnqueued = FALSE;
    SetRainStrengthFromSoundEffect(SE_THUNDERSTORM);
}
pub unsafe fn Thunderstorm_InitAll() {
    Thunderstorm_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == FALSE
    {
        Thunderstorm_Main();
    }
}
pub unsafe fn Downpour_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep =
        THUNDER_STATE_LOAD_RAIN;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .rainSpriteVisibleCounter = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .rainSpriteVisibleDelay = 4;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).isDownpour =
        TRUE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetRainSpriteCount = 24;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 3;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    SetRainStrengthFromSoundEffect(SE_DOWNPOUR);
}
pub unsafe fn Downpour_InitAll() {
    Downpour_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == FALSE
    {
        Thunderstorm_Main();
    }
}
pub unsafe fn Thunderstorm_Main() {
    UpdateThunderSound();
    'l1: {
        let sw1: u16 = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .initStep;
        let mut fall = false;
        if sw1 == THUNDER_STATE_LOAD_RAIN {
            LoadRainSpriteSheet();
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
            break 'l1;
        }
        if sw1 == THUNDER_STATE_CREATE_RAIN {
            if CreateRainSprite() == 0 {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_INIT_RAIN {
            if UpdateVisibleRainSprites() == 0 {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .weatherGfxLoaded = TRUE;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_WAIT_CHANGE {
            if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .palProcessingState
                != WEATHER_PAL_STATE_CHANGING_WEATHER
            {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .initStep = THUNDER_STATE_INIT_CYCLE_1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_NEW_CYCLE {
            fall = true;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderAllowEnd = TRUE;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderTimer = (Random() as i32 % 360) as u16 + 360;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        if fall || sw1 == THUNDER_STATE_NEW_CYCLE_WAIT {
            if ({
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .thunderTimer -= 1;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderTimer
            }) == 0
            {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_INIT_CYCLE_1 {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderAllowEnd = TRUE;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderLongBolt = (Random() as i32 % 2) as u8;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
            break 'l1;
        }
        if sw1 == THUNDER_STATE_INIT_CYCLE_2 {
            fall = true;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderShortBolts = (Random() as u8 & 1) + 1;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        if fall || sw1 == THUNDER_STATE_SHORT_BOLT {
            ApplyWeatherColorMapIfIdle(19);
            if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderLongBolt
                == 0
                && (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderShortBolts
                    == 1
            {
                EnqueueThunder(20);
            }
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderTimer = (Random() as i32 % 3) as u16 + 6;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
            break 'l1;
        }
        if sw1 == THUNDER_STATE_TRY_NEW_BOLT {
            if ({
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .thunderTimer -= 1;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderTimer
            }) == 0
            {
                ApplyWeatherColorMapIfIdle(3);
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderAllowEnd = TRUE;
                if ({
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .thunderShortBolts -= 1;
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .thunderShortBolts
                }) != 0
                {
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .thunderTimer = (Random() as i32 % 16) as u16 + 60;
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .initStep = THUNDER_STATE_WAIT_BOLT_SHORT;
                } else if (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .thunderLongBolt
                    == 0
                {
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .initStep = THUNDER_STATE_NEW_CYCLE;
                } else {
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .initStep = THUNDER_STATE_INIT_BOLT_LONG;
                }
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_WAIT_BOLT_SHORT {
            if ({
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .thunderTimer -= 1;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderTimer
            }) == 0
            {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .initStep = THUNDER_STATE_SHORT_BOLT;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_INIT_BOLT_LONG {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderTimer = (Random() as i32 % 16) as u16 + 60;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
            break 'l1;
        }
        if sw1 == THUNDER_STATE_WAIT_BOLT_LONG {
            if ({
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .thunderTimer -= 1;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderTimer
            }) == 0
            {
                EnqueueThunder(100);
                ApplyWeatherColorMapIfIdle(19);
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderTimer = (Random() & 0xF) + 30;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_FADE_BOLT_LONG {
            if ({
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .thunderTimer -= 1;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderTimer
            }) == 0
            {
                ApplyWeatherColorMapIfIdle_Gradual(19, 3, 5);
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_END_BOLT_LONG {
            if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .palProcessingState
                == WEATHER_PAL_STATE_IDLE
            {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderAllowEnd = TRUE;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .initStep = THUNDER_STATE_NEW_CYCLE;
            }
            break 'l1;
        }
    }
}
pub unsafe fn Thunderstorm_Finish() -> u8 {
    'l1: {
        let sw1: u16 = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .finishStep;
        let matched = sw1 == 0 || sw1 == 1 || sw1 == 2;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderAllowEnd = FALSE;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .finishStep += 1;
        }
        if fall || sw1 == 1 {
            Thunderstorm_Main();
            if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderAllowEnd
                != 0
            {
                if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .nextWeather
                    == WEATHER_RAIN
                    || (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .nextWeather
                        == WEATHER_RAIN_THUNDERSTORM
                    || (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .nextWeather
                        == WEATHER_DOWNPOUR
                {
                    return FALSE;
                }
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .targetRainSpriteCount = 0;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            if UpdateVisibleRainSprites() == 0 {
                DestroyRainSprites();
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .thunderEnqueued = FALSE;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
                return FALSE;
            }
            break 'l1;
        }
        if !matched {
            return FALSE;
        }
    }
    TRUE
}
unsafe fn EnqueueThunder(waitFrames: u16) {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .thunderEnqueued
        == 0
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .thunderSETimer = rem_i32(Random() as i32, waitFrames as i32) as u16;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .thunderEnqueued = TRUE;
    }
}
unsafe fn UpdateThunderSound() {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .thunderEnqueued
        == TRUE
    {
        if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .thunderSETimer
            == 0
        {
            if IsSEPlaying() != 0 {
                return;
            }
            if Random() as i32 & 1 != 0 {
                PlaySE(SE_THUNDER);
            } else {
                PlaySE(SE_THUNDER2);
            }
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderEnqueued = FALSE;
        } else {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .thunderSETimer -= 1;
        }
    }
}
pub unsafe fn FogHorizontal_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogHSpritesCreated
        == 0
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollCounter = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollOffset = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollPosX = 0;
        Weather_SetBlendCoeffs(0, 16);
    }
}
pub unsafe fn FogHorizontal_InitAll() {
    FogHorizontal_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == FALSE
    {
        FogHorizontal_Main();
    }
}
pub unsafe fn FogHorizontal_Main() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogHScrollPosX = (gSpriteCoordOffsetX as u16
        - (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollOffset)
        & 0xFF;
    if ({
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollCounter += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollCounter
    }) > 3
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollCounter = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollOffset += 1;
    }
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep
    {
        0 => {
            CreateFogHorizontalSprites();
            if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .currWeather
                == WEATHER_FOG_HORIZONTAL
            {
                Weather_SetTargetBlendCoeffs(12, 8, 3);
            } else {
                Weather_SetTargetBlendCoeffs(4, 16, 0);
            }
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        1 if Weather_UpdateBlend() != 0 => {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .weatherGfxLoaded = TRUE;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        _ => {}
    }
}
pub unsafe fn FogHorizontal_Finish() -> u8 {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogHScrollPosX = (gSpriteCoordOffsetX as u16
        - (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollOffset)
        & 0xFF;
    if ({
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollCounter += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollCounter
    }) > 3
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollCounter = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHScrollOffset += 1;
    }
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .finishStep
    {
        0 => {
            Weather_SetTargetBlendCoeffs(0, 16, 3);
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .finishStep += 1;
        }
        1 => {
            if Weather_UpdateBlend() != 0 {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
            }
        }
        2 => {
            DestroyFogHorizontalSprites();
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .finishStep += 1;
        }
        _ => {
            return FALSE;
        }
    }
    TRUE
}
pub(crate) unsafe fn FogHorizontalSpriteCallback(sprite: *mut Sprite) {
    (*sprite).y2 = gSpriteCoordOffsetY as u8 as i16;
    (*sprite).x = (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogHScrollPosX as i16
        + 32
        + (*sprite).data[0] * 64;
    if (*sprite).x >= 272 {
        (*sprite).x = 480
            + (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .fogHScrollPosX as i16
            - (4 - (*sprite).data[0]) * 64;
        (*sprite).x &= 0x1FF;
    }
}
unsafe fn CreateFogHorizontalSprites() {
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogHSpritesCreated
        == 0
    {
        let mut fogHorizontalSpriteSheet: SpriteSheet = zeroed();
        fogHorizontalSpriteSheet.data =
            gWeatherFogHorizontalTiles.as_ptr().cast_mut() as *mut c_void;
        fogHorizontalSpriteSheet.size = 2048;
        fogHorizontalSpriteSheet.tag = GFXTAG_FOG_H;
        LoadSpriteSheet(&raw mut fogHorizontalSpriteSheet);
        for i in 0..NUM_FOG_HORIZONTAL_SPRITES {
            spriteId = CreateSpriteAtEnd(
                (&raw const *sFogHorizontalSpriteTemplate).cast_mut(),
                0,
                0,
                0xFF,
            );
            if spriteId != MAX_SPRITES {
                sprite = &raw mut gSprites[spriteId];
                (*sprite).data[0] = (i as i32 % 5) as i16;
                (*sprite).x = (i as i32 % 5) as i16 * 64 + 32;
                (*sprite).y = (i as i32 / 5) as i16 * 64 + 32;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .fogHSprites[i] = sprite;
            } else {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .fogHSprites[i] = null_mut();
            }
        }
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHSpritesCreated = TRUE;
    }
}
unsafe fn DestroyFogHorizontalSprites() {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogHSpritesCreated
        != 0
    {
        for i in 0..NUM_FOG_HORIZONTAL_SPRITES {
            if !(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s2
                .fogHSprites[i]
                .is_null()
            {
                DestroySprite(
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .fogHSprites[i],
                );
            }
        }
        FreeSpriteTilesByTag(GFXTAG_FOG_H);
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogHSpritesCreated = 0;
    }
}
pub unsafe fn Ash_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = FALSE;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).ashUnused =
        20;
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .ashSpritesCreated
        == 0
    {
        Weather_SetBlendCoeffs(0, 16);
        SetGpuReg(REG_OFFSET_BLDALPHA, 16192);
    }
}
pub unsafe fn Ash_InitAll() {
    Ash_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == FALSE
    {
        Ash_Main();
    }
}
pub unsafe fn Ash_Main() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .ashBaseSpritesX = gSpriteCoordOffsetX as u16 & 0x1FF;
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .ashBaseSpritesX
        >= DISPLAY_WIDTH
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .ashBaseSpritesX -= DISPLAY_WIDTH;
    }
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep
    {
        0 => {
            LoadAshSpriteSheet();
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        1 => {
            if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .ashSpritesCreated
                == 0
            {
                CreateAshSprites();
            }
            Weather_SetTargetBlendCoeffs(16, 0, 1);
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        2 => {
            if Weather_UpdateBlend() != 0 {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .weatherGfxLoaded = TRUE;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
        }
        _ => {
            Weather_UpdateBlend();
        }
    }
}
pub unsafe fn Ash_Finish() -> u8 {
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .finishStep
    {
        0 => {
            Weather_SetTargetBlendCoeffs(0, 16, 1);
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .finishStep += 1;
        }
        1 => {
            if Weather_UpdateBlend() != 0 {
                DestroyAshSprites();
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
            }
        }
        2 => {
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .finishStep += 1;
            return FALSE;
        }
        _ => {
            return FALSE;
        }
    }
    TRUE
}
unsafe fn LoadAshSpriteSheet() {
    LoadSpriteSheet((&raw const *sAshSpriteSheet).cast_mut());
}
unsafe fn CreateAshSprites() {
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .ashSpritesCreated
        == 0
    {
        for i in 0..NUM_ASH_SPRITES {
            spriteId = CreateSpriteAtEnd((&raw const *sAshSpriteTemplate).cast_mut(), 0, 0, 0x4E);
            if spriteId != MAX_SPRITES {
                sprite = &raw mut gSprites[spriteId];
                (*sprite).data[tCounterY] = 0;
                (*sprite).data[2] = (i as i32 % 5) as u8 as i16;
                (*sprite).data[3] = (i as i32 / 5) as u8 as i16;
                (*sprite).data[tOffsetY] = (*sprite).data[3] * 64 + 32;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .ashSprites[i] = sprite;
            } else {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .ashSprites[i] = null_mut();
            }
        }
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .ashSpritesCreated = TRUE;
    }
}
unsafe fn DestroyAshSprites() {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .ashSpritesCreated
        != 0
    {
        for i in 0..(NUM_ASH_SPRITES as u16) {
            if !(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s2
                .ashSprites[i]
                .is_null()
            {
                DestroySprite(
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .ashSprites[i],
                );
            }
        }
        FreeSpriteTilesByTag(GFXTAG_ASH);
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .ashSpritesCreated = FALSE;
    }
}
pub(crate) unsafe fn UpdateAshSprite(sprite: *mut Sprite) {
    if ({
        (*sprite).data[tCounterY] += 1;
        (*sprite).data[tCounterY]
    }) > 5
    {
        (*sprite).data[tCounterY] = 0;
        (*sprite).data[tOffsetY] += 1;
    }
    (*sprite).y = gSpriteCoordOffsetY + (*sprite).data[tOffsetY];
    (*sprite).x = (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .ashBaseSpritesX as i16
        + 32
        + (*sprite).data[2] * 64;
    if (*sprite).x >= 272 {
        (*sprite).x = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .ashBaseSpritesX as i16
            + 480
            - (4 - (*sprite).data[2]) * 64;
        (*sprite).x &= 0x1FF;
    }
}
pub unsafe fn FogDiagonal_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogHScrollCounter = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogHScrollOffset = 1;
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogDSpritesCreated
        == 0
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDScrollXCounter = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDScrollYCounter = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDXOffset = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDYOffset = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDBaseSpritesX = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDPosY = 0;
        Weather_SetBlendCoeffs(0, 16);
    }
}
pub unsafe fn FogDiagonal_InitAll() {
    FogDiagonal_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == FALSE
    {
        FogDiagonal_Main();
    }
}
pub unsafe fn FogDiagonal_Main() {
    UpdateFogDiagonalMovement();
    'l1: {
        match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .initStep
        {
            0 => {
                CreateFogDiagonalSprites();
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
            1 => {
                Weather_SetTargetBlendCoeffs(12, 8, 8);
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
            2 => {
                if Weather_UpdateBlend() == 0 {
                    break 'l1;
                }
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .weatherGfxLoaded = TRUE;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .initStep += 1;
            }
            _ => {}
        }
    }
}
pub unsafe fn FogDiagonal_Finish() -> u8 {
    UpdateFogDiagonalMovement();
    'l1: {
        match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .finishStep
        {
            0 => {
                Weather_SetTargetBlendCoeffs(0, 16, 1);
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
            }
            1 => {
                if Weather_UpdateBlend() == 0 {
                    break 'l1;
                }
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
            }
            2 => {
                DestroyFogDiagonalSprites();
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
            }
            _ => {
                return FALSE;
            }
        }
    }
    TRUE
}
unsafe fn UpdateFogDiagonalMovement() {
    if ({
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDScrollXCounter += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDScrollXCounter
    }) > 2
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDXOffset += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDScrollXCounter = 0;
    }
    if ({
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDScrollYCounter += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDScrollYCounter
    }) > 4
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDYOffset += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDScrollYCounter = 0;
    }
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogDBaseSpritesX = (gSpriteCoordOffsetX as u16
        - (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDXOffset)
        & 0xFF;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).fogDPosY =
        gSpriteCoordOffsetY as u16
            + (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .fogDYOffset;
}
unsafe fn CreateFogDiagonalSprites() {
    let mut fogDiagonalSpriteSheet: SpriteSheet = zeroed();
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogDSpritesCreated
        == 0
    {
        fogDiagonalSpriteSheet = *sFogDiagonalSpriteSheet;
        LoadSpriteSheet(&raw mut fogDiagonalSpriteSheet);
        for i in 0..NUM_FOG_DIAGONAL_SPRITES {
            spriteId = CreateSpriteAtEnd(
                (&raw const *sFogDiagonalSpriteTemplate).cast_mut(),
                0,
                (i as i32 / 5) as i16 * 64,
                0xFF,
            );
            if spriteId != MAX_SPRITES {
                sprite = &raw mut gSprites[spriteId];
                (*sprite).data[0] = (i as i32 % 5) as i16;
                (*sprite).data[1] = (i as i32 / 5) as i16;
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .fogDSprites[i] = sprite;
            } else {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .fogDSprites[i] = null_mut();
            }
        }
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDSpritesCreated = TRUE;
    }
}
unsafe fn DestroyFogDiagonalSprites() {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogDSpritesCreated
        != 0
    {
        for i in 0..NUM_FOG_DIAGONAL_SPRITES {
            if !(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s2
                .fogDSprites[i]
                .is_null()
            {
                DestroySprite(
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .fogDSprites[i],
                );
            }
        }
        FreeSpriteTilesByTag(GFXTAG_FOG_D);
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .fogDSpritesCreated = FALSE;
    }
}
pub(crate) unsafe fn UpdateFogDiagonalSprite(sprite: *mut Sprite) {
    (*sprite).y2 = (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogDPosY as i16;
    (*sprite).x = (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .fogDBaseSpritesX as i16
        + 32
        + (*sprite).data[0] * 64;
    if (*sprite).x >= 272 {
        (*sprite).x = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .fogDBaseSpritesX as i16
            + 480
            - (4 - (*sprite).data[0]) * 64;
        (*sprite).x &= 0x1FF;
    }
}
pub unsafe fn Sandstorm_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormSpritesCreated
        == 0
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormXOffset = {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sandstormYOffset = 0;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sandstormYOffset
        };
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveIndex = 8;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveCounter = 0;
        if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveIndex
            >= 96
        {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sandstormWaveIndex = 0x80
                - (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sandstormWaveIndex;
        }
        Weather_SetBlendCoeffs(0, 16);
    }
}
pub unsafe fn Sandstorm_InitAll() {
    Sandstorm_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == 0
    {
        Sandstorm_Main();
    }
}
pub unsafe fn Sandstorm_Main() {
    UpdateSandstormMovement();
    UpdateSandstormWaveIndex();
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormWaveIndex
        >= 96
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveIndex = MIN_SANDSTORM_WAVE_INDEX;
    }
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep
    {
        0 => {
            CreateSandstormSprites();
            CreateSwirlSandstormSprites();
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        1 => {
            Weather_SetTargetBlendCoeffs(16, 0, 0);
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        2 if Weather_UpdateBlend() != 0 => {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .weatherGfxLoaded = TRUE;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .initStep += 1;
        }
        _ => {}
    }
}
pub unsafe fn Sandstorm_Finish() -> u8 {
    UpdateSandstormMovement();
    UpdateSandstormWaveIndex();
    match (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .finishStep
    {
        0 => {
            Weather_SetTargetBlendCoeffs(0, 16, 0);
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .finishStep += 1;
        }
        1 => {
            if Weather_UpdateBlend() != 0 {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .finishStep += 1;
            }
        }
        2 => {
            DestroySandstormSprites();
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .finishStep += 1;
        }
        _ => {
            return FALSE;
        }
    }
    TRUE
}
unsafe fn UpdateSandstormWaveIndex() {
    if ({
        let t1 = (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveCounter;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveCounter += 1;
        t1
    }) > 4
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveIndex += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveCounter = 0;
    }
}
unsafe fn UpdateSandstormMovement() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormXOffset -= (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
        [(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveIndex] as u32
        * 4;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormYOffset -= (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
        [(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormWaveIndex] as u32;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormBaseSpritesX = (gSpriteCoordOffsetX as u16
        + ((*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormXOffset
            >> 8) as u16)
        & 0xFF;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormPosY = gSpriteCoordOffsetY as u16
        + ((*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormYOffset
            >> 8) as u16;
}
unsafe fn DestroySandstormSprites() {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormSpritesCreated
        != 0
    {
        for i in 0..NUM_SANDSTORM_SPRITES {
            if !(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites1[i]
                .is_null()
            {
                DestroySprite(
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .sandstormSprites1[i],
                );
            }
        }
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormSpritesCreated = FALSE;
        FreeSpriteTilesByTag(GFXTAG_SANDSTORM);
    }
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormSwirlSpritesCreated
        != 0
    {
        for i in 0..NUM_SWIRL_SANDSTORM_SPRITES {
            if !(*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites2[i]
                .is_null()
            {
                DestroySprite(
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .sandstormSprites2[i],
                );
            }
        }
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormSwirlSpritesCreated = FALSE;
    }
}
unsafe fn CreateSandstormSprites() {
    let mut spriteId: u8 = 0;
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormSpritesCreated
        == 0
    {
        LoadSpriteSheet((&raw const *sSandstormSpriteSheet).cast_mut());
        LoadCustomWeatherSpritePalette(gSandstormWeatherPalette.as_ptr().cast_mut());
        for i in 0..NUM_SANDSTORM_SPRITES {
            spriteId = CreateSpriteAtEnd(
                (&raw const *sSandstormSpriteTemplate).cast_mut(),
                0,
                (i as i32 / 5) as i16 * 64,
                1,
            );
            if spriteId != MAX_SPRITES {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .sandstormSprites1[i] = &raw mut gSprites[spriteId];
                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites1[i])
                    .data[0] = (i as i32 % 5) as i16;
                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites1[i])
                    .data[1] = (i as i32 / 5) as i16;
            } else {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .sandstormSprites1[i] = null_mut();
            }
        }
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .sandstormSpritesCreated = TRUE;
    }
}
unsafe fn CreateSwirlSandstormSprites() {
    let mut spriteId: u8 = 0;
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormSwirlSpritesCreated
        == 0
    {
        for i in 0..NUM_SWIRL_SANDSTORM_SPRITES {
            spriteId = CreateSpriteAtEnd(
                (&raw const *sSandstormSpriteTemplate).cast_mut(),
                i as i16 * 48 + 24,
                208,
                1,
            );
            if spriteId != MAX_SPRITES {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .sandstormSprites2[i] = &raw mut gSprites[spriteId];
                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites2[i])
                    .oam
                    .set_size(ST_OAM_SIZE_2);
                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites2[i])
                    .data[1] = i as i16 * 51;
                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites2[i])
                    .data[tRadius] = 8;
                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites2[i])
                    .data[tRadiusCounter] = 0;
                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites2[i])
                    .data[4] = 0x6730;
                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites2[i])
                    .data[tEntranceDelay] = sSwirlEntranceDelays[i] as i16;
                StartSpriteAnim(
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .sandstormSprites2[i],
                    1,
                );
                CalcCenterToCornerVec(
                    (*(*(&raw const crate::data::field_weather::gWeatherPtr)
                        .cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .sandstormSprites2[i],
                    ST_OAM_AFFINE_OFF as u8,
                    2,
                    ST_OAM_AFFINE_OFF as u8,
                );
                (*(*(*(&raw const crate::data::field_weather::gWeatherPtr)
                    .cast::<*mut Weather>()))
                .sprites
                .s2
                .sandstormSprites2[i])
                    .callback = Some(WaitSandSwirlSpriteEntrance);
            } else {
                (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                    .sprites
                    .s2
                    .sandstormSprites2[i] = null_mut();
            }
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .sandstormSwirlSpritesCreated = TRUE;
        }
    }
}
pub(crate) unsafe fn UpdateSandstormSprite(sprite: *mut Sprite) {
    (*sprite).y2 = (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormPosY as i16;
    (*sprite).x = (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .sandstormBaseSpritesX as i16
        + 32
        + (*sprite).data[0] * 64;
    if (*sprite).x >= 272 {
        (*sprite).x = (*(*(&raw const crate::data::field_weather::gWeatherPtr)
            .cast::<*mut Weather>()))
        .sandstormBaseSpritesX as i16
            + 480
            - (4 - (*sprite).data[0]) * 64;
        (*sprite).x &= 0x1FF;
    }
}
pub(crate) unsafe fn WaitSandSwirlSpriteEntrance(sprite: *mut Sprite) {
    if ({
        (*sprite).data[tEntranceDelay] -= 1;
        (*sprite).data[tEntranceDelay]
    }) == -1
    {
        (*sprite).callback = Some(UpdateSandstormSwirlSprite);
    }
}
pub(crate) unsafe fn UpdateSandstormSwirlSprite(sprite: *mut Sprite) {
    let mut y: u32 = 0;
    if ({
        (*sprite).y -= 1;
        (*sprite).y
    }) < -48
    {
        (*sprite).y = 208;
        (*sprite).data[tRadius] = 4;
    }
    let x: u32 = (*sprite).data[tRadius] as u32
        * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[(*sprite).data[1]]
            as u32;
    y = (*sprite).data[tRadius] as u32
        * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [(*sprite).data[1] as i32 + 0x40] as u32;
    (*sprite).x2 = (x >> 8) as i16;
    (*sprite).y2 = (y >> 8) as i16;
    (*sprite).data[1] = ((*sprite).data[1] + 10) & 0xFF;
    if ({
        (*sprite).data[tRadiusCounter] += 1;
        (*sprite).data[tRadiusCounter]
    }) > 8
    {
        (*sprite).data[tRadiusCounter] = 0;
        (*sprite).data[tRadius] += 1;
    }
}
pub unsafe fn Shade_InitVars() {
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>())).initStep = 0;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .targetColorMapIndex = 3;
    (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .colorMapStepDelay = 20;
}
pub unsafe fn Shade_InitAll() {
    Shade_InitVars();
}
pub fn Shade_Main() {}
pub fn Shade_Finish() -> u8 {
    FALSE
}
pub unsafe fn Bubbles_InitVars() {
    FogHorizontal_InitVars();
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .bubblesSpritesCreated
        == 0
    {
        LoadSpriteSheet((&raw const *sWeatherBubbleSpriteSheet).cast_mut());
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .bubblesDelayIndex = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .bubblesDelayCounter = sBubbleStartDelays[0] as u16;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .bubblesCoordsIndex = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .bubblesSpriteCount = 0;
    }
}
pub unsafe fn Bubbles_InitAll() {
    Bubbles_InitVars();
    while (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .weatherGfxLoaded
        == 0
    {
        Bubbles_Main();
    }
}
pub unsafe fn Bubbles_Main() {
    FogHorizontal_Main();
    if ({
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .bubblesDelayCounter += 1;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .bubblesDelayCounter
    }) > sBubbleStartDelays[(*(*(&raw const crate::data::field_weather::gWeatherPtr)
        .cast::<*mut Weather>()))
    .bubblesDelayIndex] as u16
    {
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .bubblesDelayCounter = 0;
        if ({
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .bubblesDelayIndex += 1;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .bubblesDelayIndex
        }) > 7
        {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .bubblesDelayIndex = 0;
        }
        CreateBubbleSprite(
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .bubblesCoordsIndex,
        );
        if ({
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .bubblesCoordsIndex += 1;
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .bubblesCoordsIndex
        }) > 12
        {
            (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
                .bubblesCoordsIndex = 0;
        }
    }
}
pub unsafe fn Bubbles_Finish() -> u8 {
    if FogHorizontal_Finish() == 0 {
        DestroyBubbleSprites();
        return FALSE;
    }
    TRUE
}
unsafe fn CreateBubbleSprite(coordsIndex: u16) {
    let x: i16 = sBubbleStartCoords[coordsIndex][0];
    let y: i16 = sBubbleStartCoords[coordsIndex][1] - gSpriteCoordOffsetY;
    let spriteId: u8 = CreateSpriteAtEnd((&raw const *sBubbleSpriteTemplate).cast_mut(), x, y, 0);
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].oam.set_priority(1);
        gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
        gSprites[spriteId].data[tScrollXCounter] = 0;
        gSprites[spriteId].data[tScrollXDir] = 0;
        gSprites[spriteId].data[2] = 0;
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .bubblesSpriteCount += 1;
    }
}
unsafe fn DestroyBubbleSprites() {
    if (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
        .bubblesSpriteCount
        != 0
    {
        for i in 0..(MAX_SPRITES as u16) {
            if gSprites[i].template == (&raw const *sBubbleSpriteTemplate).cast_mut() {
                DestroySprite(&raw mut gSprites[i]);
            }
        }
        FreeSpriteTilesByTag(GFXTAG_BUBBLE);
        (*(*(&raw const crate::data::field_weather::gWeatherPtr).cast::<*mut Weather>()))
            .bubblesSpriteCount = 0;
    }
}
pub(crate) unsafe fn UpdateBubbleSprite(sprite: *mut Sprite) {
    (*sprite).data[tScrollXCounter] += 1;
    if ({
        (*sprite).data[tScrollXCounter] += 1;
        (*sprite).data[tScrollXCounter]
    }) > 8
    {
        (*sprite).data[tScrollXCounter] = 0;
        if (*sprite).data[tScrollXDir] == 0 {
            if ({
                (*sprite).x2 += 1;
                (*sprite).x2
            }) > 4
            {
                (*sprite).data[tScrollXDir] = 1;
            }
        } else {
            if ({
                (*sprite).x2 -= 1;
                (*sprite).x2
            }) <= 0
            {
                (*sprite).data[tScrollXDir] = 0;
            }
        }
    }
    (*sprite).y -= 3;
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) >= 120
    {
        DestroySprite(sprite);
    }
}
fn UnusedSetCurrentAbnormalWeather(weather: u32, unknown: u32) {
    sCurrentAbnormalWeather.set(weather as u8);
    sUnusedWeatherRelated.set(unknown as u16);
}
pub(crate) unsafe fn Task_DoAbnormalWeather(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            if ({
                let t1 = *data.at(15);
                *data.at(15) -= 1;
                t1
            }) <= 0
            {
                SetNextWeather(*data.at(1) as u8);
                sCurrentAbnormalWeather.set(*data.at(1) as u8);
                *data.at(15) = 600;
                *data += 1;
            }
        }
        1 if ({
            let t2 = *data.at(15);
            *data.at(15) -= 1;
            t2
        }) <= 0 =>
        {
            SetNextWeather(*data.at(2) as u8);
            sCurrentAbnormalWeather.set(*data.at(2) as u8);
            *data.at(15) = 600;
            *data = 0;
        }
        _ => {}
    }
}
unsafe fn CreateAbnormalWeatherTask() {
    let taskId: u8 = CreateTask(Some(Task_DoAbnormalWeather), 0);
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(15) = 600;
    if sCurrentAbnormalWeather.get() == WEATHER_DOWNPOUR {
        *data.at(1) = WEATHER_DROUGHT as i16;
        *data.at(2) = WEATHER_DOWNPOUR as i16;
    } else if sCurrentAbnormalWeather.get() == WEATHER_DROUGHT {
        *data.at(1) = WEATHER_DOWNPOUR as i16;
        *data.at(2) = WEATHER_DROUGHT as i16;
    } else {
        sCurrentAbnormalWeather.set(WEATHER_DOWNPOUR);
        *data.at(1) = WEATHER_DROUGHT as i16;
        *data.at(2) = WEATHER_DOWNPOUR as i16;
    }
}
pub unsafe fn SetSavedWeather(weather: u32) {
    let oldWeather: u8 = (*gSaveBlock1Ptr).weather;
    (*gSaveBlock1Ptr).weather = TranslateWeatherNum(weather as u8);
    UpdateRainCounter((*gSaveBlock1Ptr).weather, oldWeather);
}
pub unsafe fn GetSavedWeather() -> u8 {
    (*gSaveBlock1Ptr).weather
}
pub unsafe fn SetSavedWeatherFromCurrMapHeader() {
    let oldWeather: u8 = (*gSaveBlock1Ptr).weather;
    (*gSaveBlock1Ptr).weather = TranslateWeatherNum(gMapHeader.weather);
    UpdateRainCounter((*gSaveBlock1Ptr).weather, oldWeather);
}
#[unsafe(no_mangle)]
pub unsafe fn SetWeather(weather: u32) {
    SetSavedWeather(weather);
    SetNextWeather(GetSavedWeather());
}
pub unsafe fn SetWeather_Unused(weather: u32) {
    SetSavedWeather(weather);
    SetCurrentAndNextWeather(GetSavedWeather());
}
pub unsafe fn DoCurrentWeather() {
    let mut weather: u8 = GetSavedWeather();
    if weather == WEATHER_ABNORMAL {
        if FuncIsActiveTask(Some(Task_DoAbnormalWeather)) == 0 {
            CreateAbnormalWeatherTask();
        }
        weather = sCurrentAbnormalWeather.get();
    } else {
        if FuncIsActiveTask(Some(Task_DoAbnormalWeather)) != 0 {
            DestroyTask(FindTaskIdByFunc(Some(Task_DoAbnormalWeather)));
        }
        sCurrentAbnormalWeather.set(WEATHER_DOWNPOUR);
    }
    SetNextWeather(weather);
}
pub unsafe fn ResumePausedWeather() {
    let mut weather: u8 = GetSavedWeather();
    if weather == WEATHER_ABNORMAL {
        if FuncIsActiveTask(Some(Task_DoAbnormalWeather)) == 0 {
            CreateAbnormalWeatherTask();
        }
        weather = sCurrentAbnormalWeather.get();
    } else {
        if FuncIsActiveTask(Some(Task_DoAbnormalWeather)) != 0 {
            DestroyTask(FindTaskIdByFunc(Some(Task_DoAbnormalWeather)));
        }
        sCurrentAbnormalWeather.set(WEATHER_DOWNPOUR);
    }
    SetCurrentAndNextWeather(weather);
}
unsafe fn TranslateWeatherNum(weather: u8) -> u8 {
    match weather {
        WEATHER_NONE => {
            return WEATHER_NONE;
        }
        WEATHER_SUNNY_CLOUDS => {
            return WEATHER_SUNNY_CLOUDS;
        }
        WEATHER_SUNNY => {
            return WEATHER_SUNNY;
        }
        WEATHER_RAIN => {
            return WEATHER_RAIN;
        }
        WEATHER_SNOW => {
            return WEATHER_SNOW;
        }
        WEATHER_RAIN_THUNDERSTORM => {
            return WEATHER_RAIN_THUNDERSTORM;
        }
        WEATHER_FOG_HORIZONTAL => {
            return WEATHER_FOG_HORIZONTAL;
        }
        WEATHER_VOLCANIC_ASH => {
            return WEATHER_VOLCANIC_ASH;
        }
        WEATHER_SANDSTORM => {
            return WEATHER_SANDSTORM;
        }
        WEATHER_FOG_DIAGONAL => {
            return WEATHER_FOG_DIAGONAL;
        }
        WEATHER_UNDERWATER => {
            return WEATHER_UNDERWATER;
        }
        WEATHER_SHADE => {
            return WEATHER_SHADE;
        }
        WEATHER_DROUGHT => {
            return WEATHER_DROUGHT;
        }
        WEATHER_DOWNPOUR => {
            return WEATHER_DOWNPOUR;
        }
        WEATHER_UNDERWATER_BUBBLES => {
            return WEATHER_UNDERWATER_BUBBLES;
        }
        WEATHER_ABNORMAL => {
            return WEATHER_ABNORMAL;
        }
        20 => {
            return sWeatherCycleRoute119[(*gSaveBlock1Ptr).weatherCycleStage];
        }
        21 => {
            return sWeatherCycleRoute123[(*gSaveBlock1Ptr).weatherCycleStage];
        }
        _ => {
            return WEATHER_NONE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn UpdateWeatherPerDay(increment: u16) {
    let mut weatherStage: u16 = (*gSaveBlock1Ptr).weatherCycleStage as u16 + increment;
    weatherStage = (weatherStage as i32 % 4) as u16;
    (*gSaveBlock1Ptr).weatherCycleStage = weatherStage as u8;
}
unsafe fn UpdateRainCounter(newWeather: u8, oldWeather: u8) {
    if newWeather != oldWeather
        && (newWeather == WEATHER_RAIN || newWeather == WEATHER_RAIN_THUNDERSTORM)
    {
        IncrementGameStat(GAME_STAT_GOT_RAINED_ON);
    }
}
