//! Translated from `src/field_weather_effect.c` by tools/rustport/c2rs.py.
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
pub(crate) static mut sCurrentAbnormalWeather: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedWeatherRelated: u16 = 0;

unsafe extern "C" {
    static mut gMapHeader: MapHeader;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static gSineTable: CArray<i16, 0>;
    static mut gSpriteCoordOffsetX: i16;
    static mut gSpriteCoordOffsetY: i16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gWeatherPtr: *mut Weather;
    fn ApplyWeatherColorMapIfIdle(a0: i8);
    fn ApplyWeatherColorMapIfIdle_Gradual(a0: u8, a1: u8, a2: u8);
    fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DroughtStateInit();
    fn DroughtStateRun();
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn IncrementGameStat(a0: u8);
    fn IsSEPlaying() -> u8;
    fn LoadCustomWeatherSpritePalette(a0: *mut u16);
    fn LoadDroughtWeatherPalettes() -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn ResetDroughtWeatherPaletteLoading();
    fn ScriptContext_Enable();
    fn SetCurrentAndNextWeather(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetNextWeather(a0: u8);
    fn SetRainStrengthFromSoundEffect(a0: u16);
    fn SetSpritePosToMapCoords(a0: i16, a1: i16, a2: *mut i16, a3: *mut i16);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn Weather_SetBlendCoeffs(a0: u8, a1: u8);
    fn Weather_SetTargetBlendCoeffs(a0: u8, a1: u8, a2: i32);
    fn Weather_UpdateBlend() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clouds_InitVars() {
    (*gWeatherPtr).targetColorMapIndex = 0;
    (*gWeatherPtr).colorMapStepDelay = 20;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    (*gWeatherPtr).initStep = 0;
    if (*gWeatherPtr).cloudSpritesCreated == FALSE {
        Weather_SetBlendCoeffs(0, 16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clouds_InitAll() {
    Clouds_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == FALSE {
        Clouds_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clouds_Main() {
    match (*gWeatherPtr).initStep {
        0 => {
            CreateCloudSprites();
            (*gWeatherPtr).initStep += 1;
        }
        1 => {
            Weather_SetTargetBlendCoeffs(12, 8, 1);
            (*gWeatherPtr).initStep += 1;
        }
        2 => {
            if Weather_UpdateBlend() != 0 {
                (*gWeatherPtr).weatherGfxLoaded = TRUE;
                (*gWeatherPtr).initStep += 1;
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clouds_Finish() -> u8 {
    match (*gWeatherPtr).finishStep {
        0 => {
            Weather_SetTargetBlendCoeffs(0, 16, 1);
            (*gWeatherPtr).finishStep += 1;
            return TRUE;
        }
        1 => {
            if Weather_UpdateBlend() != 0 {
                DestroyCloudSprites();
                (*gWeatherPtr).finishStep += 1;
            }
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sunny_InitVars() {
    (*gWeatherPtr).targetColorMapIndex = 0;
    (*gWeatherPtr).colorMapStepDelay = 20;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sunny_InitAll() {
    Sunny_InitVars();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sunny_Main() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sunny_Finish() -> u8 {
    return FALSE;
}
pub(crate) unsafe extern "C" fn CreateCloudSprites() {
    let mut i: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if (*gWeatherPtr).cloudSpritesCreated == TRUE {
        return;
    }
    LoadSpriteSheet((&raw const *sCloudSpriteSheet).cast_mut());
    LoadCustomWeatherSpritePalette(gCloudsWeatherPalette.as_ptr().cast_mut());
    i = 0;
    while i < NUM_CLOUD_SPRITES {
        spriteId = CreateSprite((&raw const *sCloudSpriteTemplate).cast_mut(), 0, 0, 0xFF);
        if spriteId != MAX_SPRITES {
            (*gWeatherPtr).sprites.s1.cloudSprites[i] = &raw mut gSprites[spriteId];
            sprite = (*gWeatherPtr).sprites.s1.cloudSprites[i];
            SetSpritePosToMapCoords(
                sCloudSpriteMapCoords[i].x + MAP_OFFSET as i16,
                sCloudSpriteMapCoords[i].y + MAP_OFFSET as i16,
                &raw mut (*sprite).x,
                &raw mut (*sprite).y,
            );
            (*sprite).set_coordOffsetEnabled(TRUE as u16);
        } else {
            (*gWeatherPtr).sprites.s1.cloudSprites[i] = null_mut();
        }
        i += 1;
    }
    (*gWeatherPtr).cloudSpritesCreated = TRUE;
}
pub(crate) unsafe extern "C" fn DestroyCloudSprites() {
    let mut i: u16 = 0;
    if (*gWeatherPtr).cloudSpritesCreated == 0 {
        return;
    }
    i = 0;
    while i < NUM_CLOUD_SPRITES {
        if !(*gWeatherPtr).sprites.s1.cloudSprites[i].is_null() {
            DestroySprite((*gWeatherPtr).sprites.s1.cloudSprites[i]);
        }
        i += 1;
    }
    FreeSpriteTilesByTag(GFXTAG_CLOUD);
    (*gWeatherPtr).cloudSpritesCreated = FALSE;
}
pub(crate) unsafe extern "C" fn UpdateCloudSprite(sprite: *mut Sprite) {
    (*sprite).data[0] = (*sprite).data[0] + 1 & 1;
    if (*sprite).data[0] != 0 {
        (*sprite).x -= 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Drought_InitVars() {
    (*gWeatherPtr).initStep = 0;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    (*gWeatherPtr).targetColorMapIndex = 0;
    (*gWeatherPtr).colorMapStepDelay = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Drought_InitAll() {
    Drought_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == FALSE {
        Drought_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Drought_Main() {
    match (*gWeatherPtr).initStep {
        0 => {
            if (*gWeatherPtr).palProcessingState != WEATHER_PAL_STATE_CHANGING_WEATHER {
                (*gWeatherPtr).initStep += 1;
            }
        }
        1 => {
            ResetDroughtWeatherPaletteLoading();
            (*gWeatherPtr).initStep += 1;
        }
        2 => {
            if LoadDroughtWeatherPalettes() == FALSE {
                (*gWeatherPtr).initStep += 1;
            }
        }
        3 => {
            DroughtStateInit();
            (*gWeatherPtr).initStep += 1;
        }
        4 => {
            DroughtStateRun();
            if (*gWeatherPtr).droughtBrightnessStage == 6 {
                (*gWeatherPtr).weatherGfxLoaded = TRUE;
                (*gWeatherPtr).initStep += 1;
            }
        }
        _ => {
            DroughtStateRun();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Drought_Finish() -> u8 {
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartDroughtWeatherBlend() {
    CreateTask(Some(UpdateDroughtBlend), 80);
}
pub(crate) unsafe extern "C" fn UpdateDroughtBlend(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    'l1: {
        let sw1: i16 = (*task).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*task).data[1] = 0;
            (*task).data[2] = 0;
            (*task).data[3] = (67108936 as usize as *mut u16).read_volatile() as i16;
            SetGpuReg(REG_OFFSET_WININ, 16191);
            SetGpuReg(REG_OFFSET_BLDCNT, 158);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            (*task).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*task).data[1] += 3;
            if (*task).data[1] > 16 {
                (*task).data[1] = 16;
            }
            SetGpuReg(REG_OFFSET_BLDY, (*task).data[1] as u16);
            if (*task).data[1] >= 16 {
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            (*task).data[2] += 1;
            if (*task).data[2] > 9 {
                (*task).data[2] = 0;
                (*task).data[1] -= 1;
                if (*task).data[1] <= 0 {
                    (*task).data[1] = 0;
                    (*task).data[0] += 1;
                }
                SetGpuReg(REG_OFFSET_BLDY, (*task).data[1] as u16);
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_WININ, (*task).data[3] as u16);
            (*task).data[0] += 1;
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            ScriptContext_Enable();
            DestroyTask(taskId);
            break 'l1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rain_InitVars() {
    (*gWeatherPtr).initStep = 0;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    (*gWeatherPtr).rainSpriteVisibleCounter = 0;
    (*gWeatherPtr).rainSpriteVisibleDelay = 8;
    (*gWeatherPtr).isDownpour = FALSE;
    (*gWeatherPtr).targetRainSpriteCount = 10;
    (*gWeatherPtr).targetColorMapIndex = 3;
    (*gWeatherPtr).colorMapStepDelay = 20;
    SetRainStrengthFromSoundEffect(SE_RAIN);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rain_InitAll() {
    Rain_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == 0 {
        Rain_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rain_Main() {
    match (*gWeatherPtr).initStep {
        0 => {
            LoadRainSpriteSheet();
            (*gWeatherPtr).initStep += 1;
        }
        1 => {
            if CreateRainSprite() == 0 {
                (*gWeatherPtr).initStep += 1;
            }
        }
        2 => {
            if UpdateVisibleRainSprites() == 0 {
                (*gWeatherPtr).weatherGfxLoaded = TRUE;
                (*gWeatherPtr).initStep += 1;
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rain_Finish() -> u8 {
    'l1: {
        let sw1: u16 = (*gWeatherPtr).finishStep;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if (*gWeatherPtr).nextWeather == WEATHER_RAIN
                || (*gWeatherPtr).nextWeather == WEATHER_RAIN_THUNDERSTORM
                || (*gWeatherPtr).nextWeather == WEATHER_DOWNPOUR
            {
                (*gWeatherPtr).finishStep = 0xFF;
                return FALSE;
            } else {
                (*gWeatherPtr).targetRainSpriteCount = 0;
                (*gWeatherPtr).finishStep += 1;
            }
        }
        if fall || sw1 == 1 {
            fall = true;
            if UpdateVisibleRainSprites() == 0 {
                DestroyRainSprites();
                (*gWeatherPtr).finishStep += 1;
                return FALSE;
            }
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartRainSpriteFall(sprite: *mut Sprite) {
    let mut rand: u32 = 0;
    let mut numFallingFrames: u16 = 0;
    let mut tileX: i32 = 0;
    let mut tileY: i32 = 0;
    if (*sprite).data[1] == 0 {
        (*sprite).data[1] = 361;
    }
    rand = 0x41c64e6d * (*sprite).data[1] as u32 + 12345;
    (*sprite).data[1] = (((rand & 0x7FFF0000) >> 16) % 600) as i16;
    numFallingFrames = sRainSpriteFallingDurations[(*gWeatherPtr).isDownpour][0];
    tileX = ((*sprite).data[1] % 30) as i32;
    (*sprite).data[2] = tileX as i16 * 8;
    tileY = ((*sprite).data[1] / 30) as i32;
    (*sprite).data[3] = tileY as i16 * 8;
    (*sprite).data[2] = tileX as i16;
    (*sprite).data[2] <<= 7;
    (*sprite).data[3] = tileY as i16;
    (*sprite).data[3] <<= 7;
    (*sprite).data[2] -=
        sRainSpriteMovement[(*gWeatherPtr).isDownpour][0] * numFallingFrames as i16;
    (*sprite).data[3] -=
        sRainSpriteMovement[(*gWeatherPtr).isDownpour][1] * numFallingFrames as i16;
    StartSpriteAnim(sprite, 0);
    (*sprite).data[4] = 0;
    (*sprite).set_coordOffsetEnabled(FALSE as u16);
    (*sprite).data[0] = numFallingFrames as i16;
}
pub(crate) unsafe extern "C" fn UpdateRainSprite(sprite: *mut Sprite) {
    if (*sprite).data[4] == 0 {
        (*sprite).data[2] += sRainSpriteMovement[(*gWeatherPtr).isDownpour][0];
        (*sprite).data[3] += sRainSpriteMovement[(*gWeatherPtr).isDownpour][1];
        (*sprite).x = (*sprite).data[2] >> 4;
        (*sprite).y = (*sprite).data[3] >> 4;
        if (*sprite).data[5] != 0
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
            StartSpriteAnim(sprite, (*gWeatherPtr).isDownpour + 1);
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
pub(crate) unsafe extern "C" fn WaitRainSprite(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        StartRainSpriteFall(sprite);
        (*sprite).callback = Some(UpdateRainSprite);
    } else {
        (*sprite).data[0] -= 1;
    }
}
pub(crate) unsafe extern "C" fn InitRainSpriteMovement(sprite: *mut Sprite, val: u16) {
    let mut numFallingFrames: u16 = sRainSpriteFallingDurations[(*gWeatherPtr).isDownpour][0];
    let mut numAdvanceRng: u16 = div_i32(
        val as i32,
        sRainSpriteFallingDurations[(*gWeatherPtr).isDownpour][1] as i32 + numFallingFrames as i32,
    ) as u16;
    let mut frameVal: u16 = rem_i32(
        val as i32,
        sRainSpriteFallingDurations[(*gWeatherPtr).isDownpour][1] as i32 + numFallingFrames as i32,
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
        (*sprite).data[6] = 0;
    } else {
        (*sprite).data[0] = frameVal as i16 - numFallingFrames as i16;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).data[6] = 1;
    }
}
pub(crate) unsafe extern "C" fn LoadRainSpriteSheet() {
    LoadSpriteSheet((&raw const *sRainSpriteSheet).cast_mut());
}
pub(crate) unsafe extern "C" fn CreateRainSprite() -> u8 {
    let mut spriteIndex: u8 = 0;
    let mut spriteId: u8 = 0;
    if (*gWeatherPtr).rainSpriteCount == MAX_RAIN_SPRITES {
        return FALSE;
    }
    spriteIndex = (*gWeatherPtr).rainSpriteCount;
    spriteId = CreateSpriteAtEnd(
        (&raw const *sRainSpriteTemplate).cast_mut(),
        sRainSpriteCoords[spriteIndex].x,
        sRainSpriteCoords[spriteIndex].y,
        78,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[5] = FALSE as i16;
        gSprites[spriteId].data[1] = spriteIndex as i16 * 145;
        while gSprites[spriteId].data[1] >= 600 {
            gSprites[spriteId].data[1] -= 600;
        }
        StartRainSpriteFall(&raw mut gSprites[spriteId]);
        InitRainSpriteMovement(&raw mut gSprites[spriteId], spriteIndex as u16 * 9);
        gSprites[spriteId].set_invisible(TRUE as u16);
        (*gWeatherPtr).sprites.s1.rainSprites[spriteIndex] = &raw mut gSprites[spriteId];
    } else {
        (*gWeatherPtr).sprites.s1.rainSprites[spriteIndex] = null_mut();
    }
    if ({
        (*gWeatherPtr).rainSpriteCount += 1;
        (*gWeatherPtr).rainSpriteCount
    }) == MAX_RAIN_SPRITES
    {
        let mut i: u16 = 0;
        i = 0;
        while i < MAX_RAIN_SPRITES as u16 {
            if !(*gWeatherPtr).sprites.s1.rainSprites[i].is_null() {
                if (*(*gWeatherPtr).sprites.s1.rainSprites[i]).data[6] == 0 {
                    (*(*gWeatherPtr).sprites.s1.rainSprites[i]).callback = Some(UpdateRainSprite);
                } else {
                    (*(*gWeatherPtr).sprites.s1.rainSprites[i]).callback = Some(WaitRainSprite);
                }
            }
            i += 1;
        }
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn UpdateVisibleRainSprites() -> u8 {
    if (*gWeatherPtr).curRainSpriteIndex == (*gWeatherPtr).targetRainSpriteCount {
        return FALSE;
    }
    if ({
        (*gWeatherPtr).rainSpriteVisibleCounter += 1;
        (*gWeatherPtr).rainSpriteVisibleCounter
    }) > (*gWeatherPtr).rainSpriteVisibleDelay as u16
    {
        (*gWeatherPtr).rainSpriteVisibleCounter = 0;
        if (*gWeatherPtr).curRainSpriteIndex < (*gWeatherPtr).targetRainSpriteCount {
            (*(*gWeatherPtr).sprites.s1.rainSprites[{
                let t2 = (*gWeatherPtr).curRainSpriteIndex;
                (*gWeatherPtr).curRainSpriteIndex += 1;
                t2
            }])
            .data[5] = TRUE as i16;
        } else {
            (*gWeatherPtr).curRainSpriteIndex -= 1;
            (*(*gWeatherPtr).sprites.s1.rainSprites[(*gWeatherPtr).curRainSpriteIndex]).data[5] =
                FALSE as i16;
            (*(*gWeatherPtr).sprites.s1.rainSprites[(*gWeatherPtr).curRainSpriteIndex])
                .set_invisible(TRUE as u16);
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn DestroyRainSprites() {
    let mut i: u16 = 0;
    i = 0;
    while i < (*gWeatherPtr).rainSpriteCount as u16 {
        if !(*gWeatherPtr).sprites.s1.rainSprites[i].is_null() {
            DestroySprite((*gWeatherPtr).sprites.s1.rainSprites[i]);
        }
        i += 1;
    }
    (*gWeatherPtr).rainSpriteCount = 0;
    FreeSpriteTilesByTag(GFXTAG_RAIN);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Snow_InitVars() {
    (*gWeatherPtr).initStep = 0;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    (*gWeatherPtr).targetColorMapIndex = 3;
    (*gWeatherPtr).colorMapStepDelay = 20;
    (*gWeatherPtr).targetSnowflakeSpriteCount = NUM_SNOWFLAKE_SPRITES;
    (*gWeatherPtr).snowflakeVisibleCounter = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Snow_InitAll() {
    let mut i: u16 = 0;
    Snow_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == FALSE {
        Snow_Main();
        i = 0;
        while i < (*gWeatherPtr).snowflakeSpriteCount as u16 {
            UpdateSnowflakeSprite((*gWeatherPtr).sprites.s1.snowflakeSprites[i]);
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Snow_Main() {
    if (*gWeatherPtr).initStep == 0 && UpdateVisibleSnowflakeSprites() == 0 {
        (*gWeatherPtr).weatherGfxLoaded = TRUE;
        (*gWeatherPtr).initStep += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Snow_Finish() -> u8 {
    'l1: {
        let sw1: u16 = (*gWeatherPtr).finishStep;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*gWeatherPtr).targetSnowflakeSpriteCount = 0;
            (*gWeatherPtr).snowflakeVisibleCounter = 0;
            (*gWeatherPtr).finishStep += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if UpdateVisibleSnowflakeSprites() == 0 {
                (*gWeatherPtr).finishStep += 1;
                return FALSE;
            }
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn UpdateVisibleSnowflakeSprites() -> u8 {
    if (*gWeatherPtr).snowflakeSpriteCount == (*gWeatherPtr).targetSnowflakeSpriteCount {
        return FALSE;
    }
    if ({
        (*gWeatherPtr).snowflakeVisibleCounter += 1;
        (*gWeatherPtr).snowflakeVisibleCounter
    }) > 36
    {
        (*gWeatherPtr).snowflakeVisibleCounter = 0;
        if (*gWeatherPtr).snowflakeSpriteCount < (*gWeatherPtr).targetSnowflakeSpriteCount {
            CreateSnowflakeSprite();
        } else {
            DestroySnowflakeSprite();
        }
    }
    return ((*gWeatherPtr).snowflakeSpriteCount != (*gWeatherPtr).targetSnowflakeSpriteCount)
        as u8;
}
pub(crate) unsafe extern "C" fn CreateSnowflakeSprite() -> u8 {
    let mut spriteId: u8 =
        CreateSpriteAtEnd((&raw const *sSnowflakeSpriteTemplate).cast_mut(), 0, 0, 78);
    if spriteId == MAX_SPRITES {
        return FALSE;
    }
    gSprites[spriteId].data[4] = (*gWeatherPtr).snowflakeSpriteCount as i16;
    InitSnowflakeSpriteMovement(&raw mut gSprites[spriteId]);
    gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
    (*gWeatherPtr).sprites.s1.snowflakeSprites[{
        let t1 = (*gWeatherPtr).snowflakeSpriteCount;
        (*gWeatherPtr).snowflakeSpriteCount += 1;
        t1
    }] = &raw mut gSprites[spriteId];
    return TRUE;
}
pub(crate) unsafe extern "C" fn DestroySnowflakeSprite() -> u8 {
    if (*gWeatherPtr).snowflakeSpriteCount != 0 {
        DestroySprite(
            (*gWeatherPtr).sprites.s1.snowflakeSprites[{
                (*gWeatherPtr).snowflakeSpriteCount -= 1;
                (*gWeatherPtr).snowflakeSpriteCount
            }],
        );
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn InitSnowflakeSpriteMovement(sprite: *mut Sprite) {
    let mut rand: u16 = 0;
    let mut x: u16 = ((*sprite).data[4] as u16 * 5 & 7) * 30 + (Random() as i32 % 30) as u16;
    (*sprite).y = -3 - (gSpriteCoordOffsetY + (*sprite).centerToCornerVecY as i16);
    (*sprite).x = x as i16 - (gSpriteCoordOffsetX + (*sprite).centerToCornerVecX as i16);
    (*sprite).data[0] = (*sprite).y * 128;
    (*sprite).x2 = 0;
    rand = Random();
    (*sprite).data[1] = (rand as i16 & 3) * 5 + 64;
    (*sprite).data[7] = (*sprite).data[1];
    StartSpriteAnim(sprite, (if rand as i32 & 1 != 0 { 0 } else { 1 }) as u8);
    (*sprite).data[3] = 0;
    (*sprite).data[2] = (if rand as i32 & 3 == 0 { 2 } else { 1 }) as i16;
    (*sprite).data[6] = (rand as i16 & 0x1F) + 210;
    (*sprite).data[5] = 0;
}
pub(crate) unsafe extern "C" fn WaitSnowflakeSprite(sprite: *mut Sprite) {
    if (*gWeatherPtr).snowflakeTimer > 18 {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).callback = Some(UpdateSnowflakeSprite);
        (*sprite).y = 250 - (gSpriteCoordOffsetY + (*sprite).centerToCornerVecY as i16);
        (*sprite).data[0] = (*sprite).y * 128;
        (*gWeatherPtr).snowflakeTimer = 0;
    }
}
pub(crate) unsafe extern "C" fn UpdateSnowflakeSprite(sprite: *mut Sprite) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    (*sprite).data[0] += (*sprite).data[1];
    (*sprite).y = (*sprite).data[0] >> 7;
    (*sprite).data[3] += (*sprite).data[2];
    (*sprite).data[3] &= 0xFF;
    (*sprite).x2 = gSineTable[(*sprite).data[3]] / 64;
    x = (*sprite).x + (*sprite).centerToCornerVecX as i16 + gSpriteCoordOffsetX & 0x1FF;
    if x as i32 & 0x100 != 0 {
        x |= -256;
    }
    if x < -3 {
        (*sprite).x = 242 - (gSpriteCoordOffsetX + (*sprite).centerToCornerVecX as i16);
    } else if x > 242 {
        (*sprite).x = -3 - (gSpriteCoordOffsetX + (*sprite).centerToCornerVecX as i16);
    }
    y = (*sprite).y + (*sprite).centerToCornerVecY as i16 + gSpriteCoordOffsetY & 0xFF;
    if y > 163 && y < 171 {
        (*sprite).y = 250 - (gSpriteCoordOffsetY + (*sprite).centerToCornerVecY as i16);
        (*sprite).data[0] = (*sprite).y * 128;
        (*sprite).data[5] = 0;
        (*sprite).data[6] = 220;
    } else if y > 242 && y < 250 {
        (*sprite).y = 163;
        (*sprite).data[0] = (*sprite).y * 128;
        (*sprite).data[5] = 0;
        (*sprite).data[6] = 220;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(WaitSnowflakeSprite);
    }
    if ({
        (*sprite).data[5] += 1;
        (*sprite).data[5]
    }) == (*sprite).data[6]
    {
        InitSnowflakeSpriteMovement(sprite);
        (*sprite).y = 250;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(WaitSnowflakeSprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Thunderstorm_InitVars() {
    (*gWeatherPtr).initStep = THUNDER_STATE_LOAD_RAIN;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    (*gWeatherPtr).rainSpriteVisibleCounter = 0;
    (*gWeatherPtr).rainSpriteVisibleDelay = 4;
    (*gWeatherPtr).isDownpour = FALSE;
    (*gWeatherPtr).targetRainSpriteCount = 16;
    (*gWeatherPtr).targetColorMapIndex = 3;
    (*gWeatherPtr).colorMapStepDelay = 20;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    (*gWeatherPtr).thunderEnqueued = FALSE;
    SetRainStrengthFromSoundEffect(SE_THUNDERSTORM);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Thunderstorm_InitAll() {
    Thunderstorm_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == FALSE {
        Thunderstorm_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Downpour_InitVars() {
    (*gWeatherPtr).initStep = THUNDER_STATE_LOAD_RAIN;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    (*gWeatherPtr).rainSpriteVisibleCounter = 0;
    (*gWeatherPtr).rainSpriteVisibleDelay = 4;
    (*gWeatherPtr).isDownpour = TRUE;
    (*gWeatherPtr).targetRainSpriteCount = 24;
    (*gWeatherPtr).targetColorMapIndex = 3;
    (*gWeatherPtr).colorMapStepDelay = 20;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    SetRainStrengthFromSoundEffect(SE_DOWNPOUR);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Downpour_InitAll() {
    Downpour_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == FALSE {
        Thunderstorm_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Thunderstorm_Main() {
    UpdateThunderSound();
    'l1: {
        let sw1: u16 = (*gWeatherPtr).initStep;
        let mut fall = false;
        if sw1 == THUNDER_STATE_LOAD_RAIN {
            fall = true;
            LoadRainSpriteSheet();
            (*gWeatherPtr).initStep += 1;
            break 'l1;
        }
        if sw1 == THUNDER_STATE_CREATE_RAIN {
            fall = true;
            if CreateRainSprite() == 0 {
                (*gWeatherPtr).initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_INIT_RAIN {
            fall = true;
            if UpdateVisibleRainSprites() == 0 {
                (*gWeatherPtr).weatherGfxLoaded = TRUE;
                (*gWeatherPtr).initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_WAIT_CHANGE {
            fall = true;
            if (*gWeatherPtr).palProcessingState != WEATHER_PAL_STATE_CHANGING_WEATHER {
                (*gWeatherPtr).initStep = THUNDER_STATE_INIT_CYCLE_1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_NEW_CYCLE {
            fall = true;
            (*gWeatherPtr).thunderAllowEnd = TRUE;
            (*gWeatherPtr).thunderTimer = (Random() as i32 % 360) as u16 + 360;
            (*gWeatherPtr).initStep += 1;
        }
        if fall || sw1 == THUNDER_STATE_NEW_CYCLE_WAIT {
            fall = true;
            if ({
                (*gWeatherPtr).thunderTimer -= 1;
                (*gWeatherPtr).thunderTimer
            }) == 0
            {
                (*gWeatherPtr).initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_INIT_CYCLE_1 {
            fall = true;
            (*gWeatherPtr).thunderAllowEnd = TRUE;
            (*gWeatherPtr).thunderLongBolt = (Random() as i32 % 2) as u8;
            (*gWeatherPtr).initStep += 1;
            break 'l1;
        }
        if sw1 == THUNDER_STATE_INIT_CYCLE_2 {
            fall = true;
            (*gWeatherPtr).thunderShortBolts = (Random() as u8 & 1) + 1;
            (*gWeatherPtr).initStep += 1;
        }
        if fall || sw1 == THUNDER_STATE_SHORT_BOLT {
            fall = true;
            ApplyWeatherColorMapIfIdle(19);
            if (*gWeatherPtr).thunderLongBolt == 0 && (*gWeatherPtr).thunderShortBolts == 1 {
                EnqueueThunder(20);
            }
            (*gWeatherPtr).thunderTimer = (Random() as i32 % 3) as u16 + 6;
            (*gWeatherPtr).initStep += 1;
            break 'l1;
        }
        if sw1 == THUNDER_STATE_TRY_NEW_BOLT {
            fall = true;
            if ({
                (*gWeatherPtr).thunderTimer -= 1;
                (*gWeatherPtr).thunderTimer
            }) == 0
            {
                ApplyWeatherColorMapIfIdle(3);
                (*gWeatherPtr).thunderAllowEnd = TRUE;
                if ({
                    (*gWeatherPtr).thunderShortBolts -= 1;
                    (*gWeatherPtr).thunderShortBolts
                }) != 0
                {
                    (*gWeatherPtr).thunderTimer = (Random() as i32 % 16) as u16 + 60;
                    (*gWeatherPtr).initStep = THUNDER_STATE_WAIT_BOLT_SHORT;
                } else if (*gWeatherPtr).thunderLongBolt == 0 {
                    (*gWeatherPtr).initStep = THUNDER_STATE_NEW_CYCLE;
                } else {
                    (*gWeatherPtr).initStep = THUNDER_STATE_INIT_BOLT_LONG;
                }
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_WAIT_BOLT_SHORT {
            fall = true;
            if ({
                (*gWeatherPtr).thunderTimer -= 1;
                (*gWeatherPtr).thunderTimer
            }) == 0
            {
                (*gWeatherPtr).initStep = THUNDER_STATE_SHORT_BOLT;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_INIT_BOLT_LONG {
            fall = true;
            (*gWeatherPtr).thunderTimer = (Random() as i32 % 16) as u16 + 60;
            (*gWeatherPtr).initStep += 1;
            break 'l1;
        }
        if sw1 == THUNDER_STATE_WAIT_BOLT_LONG {
            fall = true;
            if ({
                (*gWeatherPtr).thunderTimer -= 1;
                (*gWeatherPtr).thunderTimer
            }) == 0
            {
                EnqueueThunder(100);
                ApplyWeatherColorMapIfIdle(19);
                (*gWeatherPtr).thunderTimer = (Random() & 0xF) + 30;
                (*gWeatherPtr).initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_FADE_BOLT_LONG {
            fall = true;
            if ({
                (*gWeatherPtr).thunderTimer -= 1;
                (*gWeatherPtr).thunderTimer
            }) == 0
            {
                ApplyWeatherColorMapIfIdle_Gradual(19, 3, 5);
                (*gWeatherPtr).initStep += 1;
            }
            break 'l1;
        }
        if sw1 == THUNDER_STATE_END_BOLT_LONG {
            fall = true;
            if (*gWeatherPtr).palProcessingState == WEATHER_PAL_STATE_IDLE {
                (*gWeatherPtr).thunderAllowEnd = TRUE;
                (*gWeatherPtr).initStep = THUNDER_STATE_NEW_CYCLE;
            }
            break 'l1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Thunderstorm_Finish() -> u8 {
    'l1: {
        let sw1: u16 = (*gWeatherPtr).finishStep;
        let matched = sw1 == 0 || sw1 == 1 || sw1 == 2;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*gWeatherPtr).thunderAllowEnd = FALSE;
            (*gWeatherPtr).finishStep += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            Thunderstorm_Main();
            if (*gWeatherPtr).thunderAllowEnd != 0 {
                if (*gWeatherPtr).nextWeather == WEATHER_RAIN
                    || (*gWeatherPtr).nextWeather == WEATHER_RAIN_THUNDERSTORM
                    || (*gWeatherPtr).nextWeather == WEATHER_DOWNPOUR
                {
                    return FALSE;
                }
                (*gWeatherPtr).targetRainSpriteCount = 0;
                (*gWeatherPtr).finishStep += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if UpdateVisibleRainSprites() == 0 {
                DestroyRainSprites();
                (*gWeatherPtr).thunderEnqueued = FALSE;
                (*gWeatherPtr).finishStep += 1;
                return FALSE;
            }
            break 'l1;
        }
        if !matched {
            fall = true;
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn EnqueueThunder(waitFrames: u16) {
    if (*gWeatherPtr).thunderEnqueued == 0 {
        (*gWeatherPtr).thunderSETimer = rem_i32(Random() as i32, waitFrames as i32) as u16;
        (*gWeatherPtr).thunderEnqueued = TRUE;
    }
}
pub(crate) unsafe extern "C" fn UpdateThunderSound() {
    if (*gWeatherPtr).thunderEnqueued == TRUE {
        if (*gWeatherPtr).thunderSETimer == 0 {
            if IsSEPlaying() != 0 {
                return;
            }
            if Random() as i32 & 1 != 0 {
                PlaySE(SE_THUNDER);
            } else {
                PlaySE(SE_THUNDER2);
            }
            (*gWeatherPtr).thunderEnqueued = FALSE;
        } else {
            (*gWeatherPtr).thunderSETimer -= 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogHorizontal_InitVars() {
    (*gWeatherPtr).initStep = 0;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    (*gWeatherPtr).targetColorMapIndex = 0;
    (*gWeatherPtr).colorMapStepDelay = 20;
    if (*gWeatherPtr).fogHSpritesCreated == 0 {
        (*gWeatherPtr).fogHScrollCounter = 0;
        (*gWeatherPtr).fogHScrollOffset = 0;
        (*gWeatherPtr).fogHScrollPosX = 0;
        Weather_SetBlendCoeffs(0, 16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogHorizontal_InitAll() {
    FogHorizontal_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == FALSE {
        FogHorizontal_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogHorizontal_Main() {
    (*gWeatherPtr).fogHScrollPosX =
        gSpriteCoordOffsetX as u16 - (*gWeatherPtr).fogHScrollOffset & 0xFF;
    if ({
        (*gWeatherPtr).fogHScrollCounter += 1;
        (*gWeatherPtr).fogHScrollCounter
    }) > 3
    {
        (*gWeatherPtr).fogHScrollCounter = 0;
        (*gWeatherPtr).fogHScrollOffset += 1;
    }
    match (*gWeatherPtr).initStep {
        0 => {
            CreateFogHorizontalSprites();
            if (*gWeatherPtr).currWeather == WEATHER_FOG_HORIZONTAL {
                Weather_SetTargetBlendCoeffs(12, 8, 3);
            } else {
                Weather_SetTargetBlendCoeffs(4, 16, 0);
            }
            (*gWeatherPtr).initStep += 1;
        }
        1 => {
            if Weather_UpdateBlend() != 0 {
                (*gWeatherPtr).weatherGfxLoaded = TRUE;
                (*gWeatherPtr).initStep += 1;
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogHorizontal_Finish() -> u8 {
    (*gWeatherPtr).fogHScrollPosX =
        gSpriteCoordOffsetX as u16 - (*gWeatherPtr).fogHScrollOffset & 0xFF;
    if ({
        (*gWeatherPtr).fogHScrollCounter += 1;
        (*gWeatherPtr).fogHScrollCounter
    }) > 3
    {
        (*gWeatherPtr).fogHScrollCounter = 0;
        (*gWeatherPtr).fogHScrollOffset += 1;
    }
    match (*gWeatherPtr).finishStep {
        0 => {
            Weather_SetTargetBlendCoeffs(0, 16, 3);
            (*gWeatherPtr).finishStep += 1;
        }
        1 => {
            if Weather_UpdateBlend() != 0 {
                (*gWeatherPtr).finishStep += 1;
            }
        }
        2 => {
            DestroyFogHorizontalSprites();
            (*gWeatherPtr).finishStep += 1;
        }
        _ => {
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn FogHorizontalSpriteCallback(sprite: *mut Sprite) {
    (*sprite).y2 = gSpriteCoordOffsetY as u8 as i16;
    (*sprite).x = (*gWeatherPtr).fogHScrollPosX as i16 + 32 + (*sprite).data[0] * 64;
    if (*sprite).x >= 272 {
        (*sprite).x = 480 + (*gWeatherPtr).fogHScrollPosX as i16 - (4 - (*sprite).data[0]) * 64;
        (*sprite).x &= 0x1FF;
    }
}
pub(crate) unsafe extern "C" fn CreateFogHorizontalSprites() {
    let mut i: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if (*gWeatherPtr).fogHSpritesCreated == 0 {
        let mut fogHorizontalSpriteSheet: SpriteSheet = zeroed();
        fogHorizontalSpriteSheet.data =
            gWeatherFogHorizontalTiles.as_ptr().cast_mut() as *mut c_void;
        fogHorizontalSpriteSheet.size = 2048;
        fogHorizontalSpriteSheet.tag = GFXTAG_FOG_H;
        LoadSpriteSheet(&raw mut fogHorizontalSpriteSheet);
        i = 0;
        while i < NUM_FOG_HORIZONTAL_SPRITES {
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
                (*gWeatherPtr).sprites.s2.fogHSprites[i] = sprite;
            } else {
                (*gWeatherPtr).sprites.s2.fogHSprites[i] = null_mut();
            }
            i += 1;
        }
        (*gWeatherPtr).fogHSpritesCreated = TRUE;
    }
}
pub(crate) unsafe extern "C" fn DestroyFogHorizontalSprites() {
    let mut i: u16 = 0;
    if (*gWeatherPtr).fogHSpritesCreated != 0 {
        i = 0;
        while i < NUM_FOG_HORIZONTAL_SPRITES {
            if !(*gWeatherPtr).sprites.s2.fogHSprites[i].is_null() {
                DestroySprite((*gWeatherPtr).sprites.s2.fogHSprites[i]);
            }
            i += 1;
        }
        FreeSpriteTilesByTag(GFXTAG_FOG_H);
        (*gWeatherPtr).fogHSpritesCreated = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Ash_InitVars() {
    (*gWeatherPtr).initStep = 0;
    (*gWeatherPtr).weatherGfxLoaded = FALSE;
    (*gWeatherPtr).targetColorMapIndex = 0;
    (*gWeatherPtr).colorMapStepDelay = 20;
    (*gWeatherPtr).ashUnused = 20;
    if (*gWeatherPtr).ashSpritesCreated == 0 {
        Weather_SetBlendCoeffs(0, 16);
        SetGpuReg(REG_OFFSET_BLDALPHA, 16192);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Ash_InitAll() {
    Ash_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == FALSE {
        Ash_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Ash_Main() {
    (*gWeatherPtr).ashBaseSpritesX = gSpriteCoordOffsetX as u16 & 0x1FF;
    while (*gWeatherPtr).ashBaseSpritesX >= DISPLAY_WIDTH {
        (*gWeatherPtr).ashBaseSpritesX -= DISPLAY_WIDTH;
    }
    match (*gWeatherPtr).initStep {
        0 => {
            LoadAshSpriteSheet();
            (*gWeatherPtr).initStep += 1;
        }
        1 => {
            if (*gWeatherPtr).ashSpritesCreated == 0 {
                CreateAshSprites();
            }
            Weather_SetTargetBlendCoeffs(16, 0, 1);
            (*gWeatherPtr).initStep += 1;
        }
        2 => {
            if Weather_UpdateBlend() != 0 {
                (*gWeatherPtr).weatherGfxLoaded = TRUE;
                (*gWeatherPtr).initStep += 1;
            }
        }
        _ => {
            Weather_UpdateBlend();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Ash_Finish() -> u8 {
    match (*gWeatherPtr).finishStep {
        0 => {
            Weather_SetTargetBlendCoeffs(0, 16, 1);
            (*gWeatherPtr).finishStep += 1;
        }
        1 => {
            if Weather_UpdateBlend() != 0 {
                DestroyAshSprites();
                (*gWeatherPtr).finishStep += 1;
            }
        }
        2 => {
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            (*gWeatherPtr).finishStep += 1;
            return FALSE;
        }
        _ => {
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn LoadAshSpriteSheet() {
    LoadSpriteSheet((&raw const *sAshSpriteSheet).cast_mut());
}
pub(crate) unsafe extern "C" fn CreateAshSprites() {
    let mut i: u8 = 0;
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if (*gWeatherPtr).ashSpritesCreated == 0 {
        i = 0;
        while i < NUM_ASH_SPRITES {
            spriteId = CreateSpriteAtEnd((&raw const *sAshSpriteTemplate).cast_mut(), 0, 0, 0x4E);
            if spriteId != MAX_SPRITES {
                sprite = &raw mut gSprites[spriteId];
                (*sprite).data[1] = 0;
                (*sprite).data[2] = (i as i32 % 5) as u8 as i16;
                (*sprite).data[3] = (i as i32 / 5) as u8 as i16;
                (*sprite).data[0] = (*sprite).data[3] * 64 + 32;
                (*gWeatherPtr).sprites.s2.ashSprites[i] = sprite;
            } else {
                (*gWeatherPtr).sprites.s2.ashSprites[i] = null_mut();
            }
            i += 1;
        }
        (*gWeatherPtr).ashSpritesCreated = TRUE;
    }
}
pub(crate) unsafe extern "C" fn DestroyAshSprites() {
    let mut i: u16 = 0;
    if (*gWeatherPtr).ashSpritesCreated != 0 {
        i = 0;
        while i < NUM_ASH_SPRITES as u16 {
            if !(*gWeatherPtr).sprites.s2.ashSprites[i].is_null() {
                DestroySprite((*gWeatherPtr).sprites.s2.ashSprites[i]);
            }
            i += 1;
        }
        FreeSpriteTilesByTag(GFXTAG_ASH);
        (*gWeatherPtr).ashSpritesCreated = FALSE;
    }
}
pub(crate) unsafe extern "C" fn UpdateAshSprite(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 5
    {
        (*sprite).data[1] = 0;
        (*sprite).data[0] += 1;
    }
    (*sprite).y = gSpriteCoordOffsetY + (*sprite).data[0];
    (*sprite).x = (*gWeatherPtr).ashBaseSpritesX as i16 + 32 + (*sprite).data[2] * 64;
    if (*sprite).x >= 272 {
        (*sprite).x = (*gWeatherPtr).ashBaseSpritesX as i16 + 480 - (4 - (*sprite).data[2]) * 64;
        (*sprite).x &= 0x1FF;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogDiagonal_InitVars() {
    (*gWeatherPtr).initStep = 0;
    (*gWeatherPtr).weatherGfxLoaded = 0;
    (*gWeatherPtr).targetColorMapIndex = 0;
    (*gWeatherPtr).colorMapStepDelay = 20;
    (*gWeatherPtr).fogHScrollCounter = 0;
    (*gWeatherPtr).fogHScrollOffset = 1;
    if (*gWeatherPtr).fogDSpritesCreated == 0 {
        (*gWeatherPtr).fogDScrollXCounter = 0;
        (*gWeatherPtr).fogDScrollYCounter = 0;
        (*gWeatherPtr).fogDXOffset = 0;
        (*gWeatherPtr).fogDYOffset = 0;
        (*gWeatherPtr).fogDBaseSpritesX = 0;
        (*gWeatherPtr).fogDPosY = 0;
        Weather_SetBlendCoeffs(0, 16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogDiagonal_InitAll() {
    FogDiagonal_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == FALSE {
        FogDiagonal_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogDiagonal_Main() {
    UpdateFogDiagonalMovement();
    'l1: {
        match (*gWeatherPtr).initStep {
            0 => {
                CreateFogDiagonalSprites();
                (*gWeatherPtr).initStep += 1;
            }
            1 => {
                Weather_SetTargetBlendCoeffs(12, 8, 8);
                (*gWeatherPtr).initStep += 1;
            }
            2 => {
                if Weather_UpdateBlend() == 0 {
                    break 'l1;
                }
                (*gWeatherPtr).weatherGfxLoaded = TRUE;
                (*gWeatherPtr).initStep += 1;
            }
            _ => {}
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FogDiagonal_Finish() -> u8 {
    UpdateFogDiagonalMovement();
    'l1: {
        match (*gWeatherPtr).finishStep {
            0 => {
                Weather_SetTargetBlendCoeffs(0, 16, 1);
                (*gWeatherPtr).finishStep += 1;
            }
            1 => {
                if Weather_UpdateBlend() == 0 {
                    break 'l1;
                }
                (*gWeatherPtr).finishStep += 1;
            }
            2 => {
                DestroyFogDiagonalSprites();
                (*gWeatherPtr).finishStep += 1;
            }
            _ => {
                return FALSE;
            }
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn UpdateFogDiagonalMovement() {
    if ({
        (*gWeatherPtr).fogDScrollXCounter += 1;
        (*gWeatherPtr).fogDScrollXCounter
    }) > 2
    {
        (*gWeatherPtr).fogDXOffset += 1;
        (*gWeatherPtr).fogDScrollXCounter = 0;
    }
    if ({
        (*gWeatherPtr).fogDScrollYCounter += 1;
        (*gWeatherPtr).fogDScrollYCounter
    }) > 4
    {
        (*gWeatherPtr).fogDYOffset += 1;
        (*gWeatherPtr).fogDScrollYCounter = 0;
    }
    (*gWeatherPtr).fogDBaseSpritesX =
        gSpriteCoordOffsetX as u16 - (*gWeatherPtr).fogDXOffset & 0xFF;
    (*gWeatherPtr).fogDPosY = gSpriteCoordOffsetY as u16 + (*gWeatherPtr).fogDYOffset;
}
pub(crate) unsafe extern "C" fn CreateFogDiagonalSprites() {
    let mut i: u16 = 0;
    let mut fogDiagonalSpriteSheet: SpriteSheet = zeroed();
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if (*gWeatherPtr).fogDSpritesCreated == 0 {
        fogDiagonalSpriteSheet = *sFogDiagonalSpriteSheet;
        LoadSpriteSheet(&raw mut fogDiagonalSpriteSheet);
        i = 0;
        while i < NUM_FOG_DIAGONAL_SPRITES {
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
                (*gWeatherPtr).sprites.s2.fogDSprites[i] = sprite;
            } else {
                (*gWeatherPtr).sprites.s2.fogDSprites[i] = null_mut();
            }
            i += 1;
        }
        (*gWeatherPtr).fogDSpritesCreated = TRUE;
    }
}
pub(crate) unsafe extern "C" fn DestroyFogDiagonalSprites() {
    let mut i: u16 = 0;
    if (*gWeatherPtr).fogDSpritesCreated != 0 {
        i = 0;
        while i < NUM_FOG_DIAGONAL_SPRITES {
            if !(*gWeatherPtr).sprites.s2.fogDSprites[i].is_null() {
                DestroySprite((*gWeatherPtr).sprites.s2.fogDSprites[i]);
            }
            i += 1;
        }
        FreeSpriteTilesByTag(GFXTAG_FOG_D);
        (*gWeatherPtr).fogDSpritesCreated = FALSE;
    }
}
pub(crate) unsafe extern "C" fn UpdateFogDiagonalSprite(sprite: *mut Sprite) {
    (*sprite).y2 = (*gWeatherPtr).fogDPosY as i16;
    (*sprite).x = (*gWeatherPtr).fogDBaseSpritesX as i16 + 32 + (*sprite).data[0] * 64;
    if (*sprite).x >= 272 {
        (*sprite).x = (*gWeatherPtr).fogDBaseSpritesX as i16 + 480 - (4 - (*sprite).data[0]) * 64;
        (*sprite).x &= 0x1FF;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sandstorm_InitVars() {
    (*gWeatherPtr).initStep = 0;
    (*gWeatherPtr).weatherGfxLoaded = 0;
    (*gWeatherPtr).targetColorMapIndex = 0;
    (*gWeatherPtr).colorMapStepDelay = 20;
    if (*gWeatherPtr).sandstormSpritesCreated == 0 {
        (*gWeatherPtr).sandstormXOffset = {
            (*gWeatherPtr).sandstormYOffset = 0;
            (*gWeatherPtr).sandstormYOffset
        };
        (*gWeatherPtr).sandstormWaveIndex = 8;
        (*gWeatherPtr).sandstormWaveCounter = 0;
        if (*gWeatherPtr).sandstormWaveIndex >= 96 {
            (*gWeatherPtr).sandstormWaveIndex = 0x80 - (*gWeatherPtr).sandstormWaveIndex;
        }
        Weather_SetBlendCoeffs(0, 16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sandstorm_InitAll() {
    Sandstorm_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == 0 {
        Sandstorm_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sandstorm_Main() {
    UpdateSandstormMovement();
    UpdateSandstormWaveIndex();
    if (*gWeatherPtr).sandstormWaveIndex >= 96 {
        (*gWeatherPtr).sandstormWaveIndex = MIN_SANDSTORM_WAVE_INDEX;
    }
    match (*gWeatherPtr).initStep {
        0 => {
            CreateSandstormSprites();
            CreateSwirlSandstormSprites();
            (*gWeatherPtr).initStep += 1;
        }
        1 => {
            Weather_SetTargetBlendCoeffs(16, 0, 0);
            (*gWeatherPtr).initStep += 1;
        }
        2 => {
            if Weather_UpdateBlend() != 0 {
                (*gWeatherPtr).weatherGfxLoaded = TRUE;
                (*gWeatherPtr).initStep += 1;
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sandstorm_Finish() -> u8 {
    UpdateSandstormMovement();
    UpdateSandstormWaveIndex();
    match (*gWeatherPtr).finishStep {
        0 => {
            Weather_SetTargetBlendCoeffs(0, 16, 0);
            (*gWeatherPtr).finishStep += 1;
        }
        1 => {
            if Weather_UpdateBlend() != 0 {
                (*gWeatherPtr).finishStep += 1;
            }
        }
        2 => {
            DestroySandstormSprites();
            (*gWeatherPtr).finishStep += 1;
        }
        _ => {
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn UpdateSandstormWaveIndex() {
    if ({
        let t1 = (*gWeatherPtr).sandstormWaveCounter;
        (*gWeatherPtr).sandstormWaveCounter += 1;
        t1
    }) > 4
    {
        (*gWeatherPtr).sandstormWaveIndex += 1;
        (*gWeatherPtr).sandstormWaveCounter = 0;
    }
}
pub(crate) unsafe extern "C" fn UpdateSandstormMovement() {
    (*gWeatherPtr).sandstormXOffset -= gSineTable[(*gWeatherPtr).sandstormWaveIndex] as u32 * 4;
    (*gWeatherPtr).sandstormYOffset -= gSineTable[(*gWeatherPtr).sandstormWaveIndex] as u32;
    (*gWeatherPtr).sandstormBaseSpritesX =
        gSpriteCoordOffsetX as u16 + ((*gWeatherPtr).sandstormXOffset >> 8) as u16 & 0xFF;
    (*gWeatherPtr).sandstormPosY =
        gSpriteCoordOffsetY as u16 + ((*gWeatherPtr).sandstormYOffset >> 8) as u16;
}
pub(crate) unsafe extern "C" fn DestroySandstormSprites() {
    let mut i: u16 = 0;
    if (*gWeatherPtr).sandstormSpritesCreated != 0 {
        i = 0;
        while i < NUM_SANDSTORM_SPRITES {
            if !(*gWeatherPtr).sprites.s2.sandstormSprites1[i].is_null() {
                DestroySprite((*gWeatherPtr).sprites.s2.sandstormSprites1[i]);
            }
            i += 1;
        }
        (*gWeatherPtr).sandstormSpritesCreated = FALSE;
        FreeSpriteTilesByTag(GFXTAG_SANDSTORM);
    }
    if (*gWeatherPtr).sandstormSwirlSpritesCreated != 0 {
        i = 0;
        while i < NUM_SWIRL_SANDSTORM_SPRITES {
            if !(*gWeatherPtr).sprites.s2.sandstormSprites2[i].is_null() {
                DestroySprite((*gWeatherPtr).sprites.s2.sandstormSprites2[i]);
            }
            i += 1;
        }
        (*gWeatherPtr).sandstormSwirlSpritesCreated = FALSE;
    }
}
pub(crate) unsafe extern "C" fn CreateSandstormSprites() {
    let mut i: u16 = 0;
    let mut spriteId: u8 = 0;
    if (*gWeatherPtr).sandstormSpritesCreated == 0 {
        LoadSpriteSheet((&raw const *sSandstormSpriteSheet).cast_mut());
        LoadCustomWeatherSpritePalette(gSandstormWeatherPalette.as_ptr().cast_mut());
        i = 0;
        while i < NUM_SANDSTORM_SPRITES {
            spriteId = CreateSpriteAtEnd(
                (&raw const *sSandstormSpriteTemplate).cast_mut(),
                0,
                (i as i32 / 5) as i16 * 64,
                1,
            );
            if spriteId != MAX_SPRITES {
                (*gWeatherPtr).sprites.s2.sandstormSprites1[i] = &raw mut gSprites[spriteId];
                (*(*gWeatherPtr).sprites.s2.sandstormSprites1[i]).data[0] = (i as i32 % 5) as i16;
                (*(*gWeatherPtr).sprites.s2.sandstormSprites1[i]).data[1] = (i as i32 / 5) as i16;
            } else {
                (*gWeatherPtr).sprites.s2.sandstormSprites1[i] = null_mut();
            }
            i += 1;
        }
        (*gWeatherPtr).sandstormSpritesCreated = TRUE;
    }
}
pub(crate) unsafe extern "C" fn CreateSwirlSandstormSprites() {
    let mut i: u16 = 0;
    let mut spriteId: u8 = 0;
    if (*gWeatherPtr).sandstormSwirlSpritesCreated == 0 {
        i = 0;
        while i < NUM_SWIRL_SANDSTORM_SPRITES {
            spriteId = CreateSpriteAtEnd(
                (&raw const *sSandstormSpriteTemplate).cast_mut(),
                i as i16 * 48 + 24,
                208,
                1,
            );
            if spriteId != MAX_SPRITES {
                (*gWeatherPtr).sprites.s2.sandstormSprites2[i] = &raw mut gSprites[spriteId];
                (*(*gWeatherPtr).sprites.s2.sandstormSprites2[i])
                    .oam
                    .set_size(ST_OAM_SIZE_2);
                (*(*gWeatherPtr).sprites.s2.sandstormSprites2[i]).data[1] = i as i16 * 51;
                (*(*gWeatherPtr).sprites.s2.sandstormSprites2[i]).data[0] = 8;
                (*(*gWeatherPtr).sprites.s2.sandstormSprites2[i]).data[2] = 0;
                (*(*gWeatherPtr).sprites.s2.sandstormSprites2[i]).data[4] = 0x6730;
                (*(*gWeatherPtr).sprites.s2.sandstormSprites2[i]).data[3] =
                    sSwirlEntranceDelays[i] as i16;
                StartSpriteAnim((*gWeatherPtr).sprites.s2.sandstormSprites2[i], 1);
                CalcCenterToCornerVec(
                    (*gWeatherPtr).sprites.s2.sandstormSprites2[i],
                    ST_OAM_AFFINE_OFF as u8,
                    2,
                    ST_OAM_AFFINE_OFF as u8,
                );
                (*(*gWeatherPtr).sprites.s2.sandstormSprites2[i]).callback =
                    Some(WaitSandSwirlSpriteEntrance);
            } else {
                (*gWeatherPtr).sprites.s2.sandstormSprites2[i] = null_mut();
            }
            (*gWeatherPtr).sandstormSwirlSpritesCreated = TRUE;
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateSandstormSprite(sprite: *mut Sprite) {
    (*sprite).y2 = (*gWeatherPtr).sandstormPosY as i16;
    (*sprite).x = (*gWeatherPtr).sandstormBaseSpritesX as i16 + 32 + (*sprite).data[0] * 64;
    if (*sprite).x >= 272 {
        (*sprite).x =
            (*gWeatherPtr).sandstormBaseSpritesX as i16 + 480 - (4 - (*sprite).data[0]) * 64;
        (*sprite).x &= 0x1FF;
    }
}
pub(crate) unsafe extern "C" fn WaitSandSwirlSpriteEntrance(sprite: *mut Sprite) {
    if ({
        (*sprite).data[3] -= 1;
        (*sprite).data[3]
    }) == -1
    {
        (*sprite).callback = Some(UpdateSandstormSwirlSprite);
    }
}
pub(crate) unsafe extern "C" fn UpdateSandstormSwirlSprite(sprite: *mut Sprite) {
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    if ({
        (*sprite).y -= 1;
        (*sprite).y
    }) < -48
    {
        (*sprite).y = 208;
        (*sprite).data[0] = 4;
    }
    x = (*sprite).data[0] as u32 * gSineTable[(*sprite).data[1]] as u32;
    y = (*sprite).data[0] as u32 * gSineTable[(*sprite).data[1] as i32 + 0x40] as u32;
    (*sprite).x2 = (x >> 8) as i16;
    (*sprite).y2 = (y >> 8) as i16;
    (*sprite).data[1] = (*sprite).data[1] + 10 & 0xFF;
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) > 8
    {
        (*sprite).data[2] = 0;
        (*sprite).data[0] += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Shade_InitVars() {
    (*gWeatherPtr).initStep = 0;
    (*gWeatherPtr).targetColorMapIndex = 3;
    (*gWeatherPtr).colorMapStepDelay = 20;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Shade_InitAll() {
    Shade_InitVars();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Shade_Main() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Shade_Finish() -> u8 {
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bubbles_InitVars() {
    FogHorizontal_InitVars();
    if (*gWeatherPtr).bubblesSpritesCreated == 0 {
        LoadSpriteSheet((&raw const *sWeatherBubbleSpriteSheet).cast_mut());
        (*gWeatherPtr).bubblesDelayIndex = 0;
        (*gWeatherPtr).bubblesDelayCounter = sBubbleStartDelays[0] as u16;
        (*gWeatherPtr).bubblesCoordsIndex = 0;
        (*gWeatherPtr).bubblesSpriteCount = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bubbles_InitAll() {
    Bubbles_InitVars();
    while (*gWeatherPtr).weatherGfxLoaded == 0 {
        Bubbles_Main();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bubbles_Main() {
    FogHorizontal_Main();
    if ({
        (*gWeatherPtr).bubblesDelayCounter += 1;
        (*gWeatherPtr).bubblesDelayCounter
    }) > sBubbleStartDelays[(*gWeatherPtr).bubblesDelayIndex] as u16
    {
        (*gWeatherPtr).bubblesDelayCounter = 0;
        if ({
            (*gWeatherPtr).bubblesDelayIndex += 1;
            (*gWeatherPtr).bubblesDelayIndex
        }) > 7
        {
            (*gWeatherPtr).bubblesDelayIndex = 0;
        }
        CreateBubbleSprite((*gWeatherPtr).bubblesCoordsIndex);
        if ({
            (*gWeatherPtr).bubblesCoordsIndex += 1;
            (*gWeatherPtr).bubblesCoordsIndex
        }) > 12
        {
            (*gWeatherPtr).bubblesCoordsIndex = 0;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bubbles_Finish() -> u8 {
    if FogHorizontal_Finish() == 0 {
        DestroyBubbleSprites();
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn CreateBubbleSprite(coordsIndex: u16) {
    let mut x: i16 = sBubbleStartCoords[coordsIndex][0];
    let mut y: i16 = sBubbleStartCoords[coordsIndex][1] - gSpriteCoordOffsetY;
    let mut spriteId: u8 =
        CreateSpriteAtEnd((&raw const *sBubbleSpriteTemplate).cast_mut(), x, y, 0);
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].oam.set_priority(1);
        gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
        gSprites[spriteId].data[0] = 0;
        gSprites[spriteId].data[1] = 0;
        gSprites[spriteId].data[2] = 0;
        (*gWeatherPtr).bubblesSpriteCount += 1;
    }
}
pub(crate) unsafe extern "C" fn DestroyBubbleSprites() {
    let mut i: u16 = 0;
    if (*gWeatherPtr).bubblesSpriteCount != 0 {
        i = 0;
        while i < MAX_SPRITES as u16 {
            if gSprites[i].template == (&raw const *sBubbleSpriteTemplate).cast_mut() {
                DestroySprite(&raw mut gSprites[i]);
            }
            i += 1;
        }
        FreeSpriteTilesByTag(GFXTAG_BUBBLE);
        (*gWeatherPtr).bubblesSpriteCount = 0;
    }
}
pub(crate) unsafe extern "C" fn UpdateBubbleSprite(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 8
    {
        (*sprite).data[0] = 0;
        if (*sprite).data[1] == 0 {
            if ({
                (*sprite).x2 += 1;
                (*sprite).x2
            }) > 4
            {
                (*sprite).data[1] = 1;
            }
        } else {
            if ({
                (*sprite).x2 -= 1;
                (*sprite).x2
            }) <= 0
            {
                (*sprite).data[1] = 0;
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
pub(crate) unsafe extern "C" fn UnusedSetCurrentAbnormalWeather(weather: u32, unknown: u32) {
    sCurrentAbnormalWeather = weather as u8;
    sUnusedWeatherRelated = unknown as u16;
}
pub(crate) unsafe extern "C" fn Task_DoAbnormalWeather(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            if ({
                let t1 = *data.at(15);
                *data.at(15) -= 1;
                t1
            }) <= 0
            {
                SetNextWeather(*data.at(1) as u8);
                sCurrentAbnormalWeather = *data.at(1) as u8;
                *data.at(15) = 600;
                *data += 1;
            }
        }
        1 => {
            if ({
                let t2 = *data.at(15);
                *data.at(15) -= 1;
                t2
            }) <= 0
            {
                SetNextWeather(*data.at(2) as u8);
                sCurrentAbnormalWeather = *data.at(2) as u8;
                *data.at(15) = 600;
                *data = 0;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateAbnormalWeatherTask() {
    let mut taskId: u8 = CreateTask(Some(Task_DoAbnormalWeather), 0);
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(15) = 600;
    if sCurrentAbnormalWeather == WEATHER_DOWNPOUR {
        *data.at(1) = WEATHER_DROUGHT as i16;
        *data.at(2) = WEATHER_DOWNPOUR as i16;
    } else if sCurrentAbnormalWeather == WEATHER_DROUGHT {
        *data.at(1) = WEATHER_DOWNPOUR as i16;
        *data.at(2) = WEATHER_DROUGHT as i16;
    } else {
        sCurrentAbnormalWeather = WEATHER_DOWNPOUR;
        *data.at(1) = WEATHER_DROUGHT as i16;
        *data.at(2) = WEATHER_DOWNPOUR as i16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSavedWeather(weather: u32) {
    let mut oldWeather: u8 = (*gSaveBlock1Ptr).weather;
    (*gSaveBlock1Ptr).weather = TranslateWeatherNum(weather as u8);
    UpdateRainCounter((*gSaveBlock1Ptr).weather, oldWeather);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedWeather() -> u8 {
    return (*gSaveBlock1Ptr).weather;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSavedWeatherFromCurrMapHeader() {
    let mut oldWeather: u8 = (*gSaveBlock1Ptr).weather;
    (*gSaveBlock1Ptr).weather = TranslateWeatherNum(gMapHeader.weather);
    UpdateRainCounter((*gSaveBlock1Ptr).weather, oldWeather);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWeather(weather: u32) {
    SetSavedWeather(weather);
    SetNextWeather(GetSavedWeather());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWeather_Unused(weather: u32) {
    SetSavedWeather(weather);
    SetCurrentAndNextWeather(GetSavedWeather());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoCurrentWeather() {
    let mut weather: u8 = GetSavedWeather();
    if weather == WEATHER_ABNORMAL {
        if FuncIsActiveTask(Some(Task_DoAbnormalWeather)) == 0 {
            CreateAbnormalWeatherTask();
        }
        weather = sCurrentAbnormalWeather;
    } else {
        if FuncIsActiveTask(Some(Task_DoAbnormalWeather)) != 0 {
            DestroyTask(FindTaskIdByFunc(Some(Task_DoAbnormalWeather)));
        }
        sCurrentAbnormalWeather = WEATHER_DOWNPOUR;
    }
    SetNextWeather(weather);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResumePausedWeather() {
    let mut weather: u8 = GetSavedWeather();
    if weather == WEATHER_ABNORMAL {
        if FuncIsActiveTask(Some(Task_DoAbnormalWeather)) == 0 {
            CreateAbnormalWeatherTask();
        }
        weather = sCurrentAbnormalWeather;
    } else {
        if FuncIsActiveTask(Some(Task_DoAbnormalWeather)) != 0 {
            DestroyTask(FindTaskIdByFunc(Some(Task_DoAbnormalWeather)));
        }
        sCurrentAbnormalWeather = WEATHER_DOWNPOUR;
    }
    SetCurrentAndNextWeather(weather);
}
pub(crate) unsafe extern "C" fn TranslateWeatherNum(weather: u8) -> u8 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateWeatherPerDay(increment: u16) {
    let mut weatherStage: u16 = (*gSaveBlock1Ptr).weatherCycleStage as u16 + increment;
    weatherStage = (weatherStage as i32 % 4) as u16;
    (*gSaveBlock1Ptr).weatherCycleStage = weatherStage as u8;
}
pub(crate) unsafe extern "C" fn UpdateRainCounter(newWeather: u8, oldWeather: u8) {
    if newWeather != oldWeather
        && (newWeather == WEATHER_RAIN || newWeather == WEATHER_RAIN_THUNDERSTORM)
    {
        IncrementGameStat(GAME_STAT_GOT_RAINED_ON);
    }
}
