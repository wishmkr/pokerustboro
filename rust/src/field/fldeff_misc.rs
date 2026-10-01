//! Translated from `src/fldeff_misc.c` by tools/rustport/c2rs.py.
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
    clippy::unnecessary_cast,
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{VarGet, VarSet};
use crate::event_object_movement::{
    GetMapCoordsFromSpritePos, GetWalkInPlaceNormalMovementAction,
    ObjectEventClearHeldMovementIfFinished, ObjectEventIsMovementOverridden,
    ObjectEventSetHeldMovement, SetSpritePosToOffsetMapCoords,
};
use crate::ffi::gSpecialVar_Result;
use crate::field_camera::CurrentMapDrawMetatileAt;
use crate::field_effect::{
    FieldEffectActiveListRemove, FieldEffectStart, FieldEffectStop, gFieldEffectArguments,
};
use crate::field_player_avatar::{
    GetPlayerAvatarFlags, GetPlayerFacingDirection, GetXYCoordsOneStepInFrontOfPlayer,
    SetPlayerAvatarTransitionFlags, SetPlayerAvatarWatering, gObjectEvents, gPlayerAvatar,
};
use crate::fieldmap::{
    MapGridGetMetatileBehaviorAt, MapGridGetMetatileIdAt, MapGridSetMetatileIdAt, gMapHeader,
};
use crate::fldeff_rocksmash::CreateFieldMoveTask;
use crate::gpu_regs::{ClearGpuRegBits, GetGpuReg, SetGpuReg, SetGpuRegBits};
use crate::metatile_behavior::{
    MetatileBehavior_IsSecretBaseCave, MetatileBehavior_IsSecretBaseShrub,
    MetatileBehavior_IsSecretBaseTree,
};
use crate::overworld::gFieldCallback2;
use crate::palette::BlendPalettes;
use crate::palette::gPlttBufferFaded;
use crate::party_menu::{
    FieldCallback_PrepareFadeInFromMenu, GetCursorSelectionMonId, gPostMenuFieldCallback,
};
use crate::script::{LockPlayerFieldControls, ScriptContext_Enable};
use crate::secret_base::{
    CheckPlayerHasSecretBase, CurMapIsSecretBase, SetCurSecretBaseIdFromPosition,
    ToggleSecretBaseEntranceMetatile, TrySetCurSecretBaseIndex,
};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::string_util::{gStringVar1, gStringVar2};
use crate::task::DestroyTask;
use crate::task::{gTasks, task_func, task_get, task_set, task_set_func};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
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
/// `FreeSpritePalette` with this module's view of its types.
#[inline]
unsafe fn FreeSpritePalette(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpritePalette(a0 as _);
    }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `ScriptContext_SetupScript` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_SetupScript(a0: *mut u8) {
    unsafe {
        crate::script::ScriptContext_SetupScript(a0 as _);
    }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tMetatileID: usize = 0;
const tX: usize = 0;
const tHorzIncrement: usize = 1;
const tY: usize = 1;
const tVertIncrement: usize = 2;
const tWinLeft: usize = 3;
const tWinRight: usize = 4;
const tWinTop: usize = 5;
const tWinBottom: usize = 6;
const tBlendCnt: usize = 7;
const tBlendY: usize = 8;
// Data tables (translate with cdata.py): sSecretPowerCave_Gfx sFiller sSecretPowerCave_Pal sSecretPowerShrub_Gfx sSecretPowerTree_Gfx sSecretPowerPlant_Pal sSandPillar0_Gfx sSandPillar1_Gfx sSandPillar2_Gfx sOam_SecretPower sAnim_SecretPowerCave sAnim_VineDropLeft sAnim_VineRiseLeft sAnim_VineDropRight sAnim_VineRiseRight sAnim_SecretPowerShrub sAnimTable_SecretPowerCave sAnimTable_SecretPowerTree sAnimTable_SecretPowerShrub sPicTable_SecretPowerCave sPicTable_SecretPowerTree sPicTable_SecretPowerShrub sSpriteTemplate_SecretPowerCave sSpriteTemplate_SecretPowerTree sSpriteTemplate_SecretPowerShrub gSpritePalette_SecretPower_Cave gSpritePalette_SecretPower_Plant sOam_SandPillar sAnim_SandPillar sAnimTable_SandPillar sPicTable_SandPillar sSpriteTemplate_SandPillar gSpritePalette_SandPillar sRecordMixLights_Gfx sRecordMixLights_Pal sPicTable_RecordMixLights sSpritePalette_RecordMixLights sAnim_RecordMixLights sAnimTable_RecordMixLights sSpriteTemplate_RecordMixLights

static sSpritePalette_RecordMixLights: Table<SpritePalette> =
    Table((&raw const crate::data::fldeff_misc::sSpritePalette_RecordMixLights).cast());
static sSpriteTemplate_RecordMixLights: Table<SpriteTemplate> =
    Table((&raw const crate::data::fldeff_misc::sSpriteTemplate_RecordMixLights).cast());
static sSpriteTemplate_SandPillar: Table<SpriteTemplate> =
    Table((&raw const crate::data::fldeff_misc::sSpriteTemplate_SandPillar).cast());
static sSpriteTemplate_SecretPowerCave: Table<SpriteTemplate> =
    Table((&raw const crate::data::fldeff_misc::sSpriteTemplate_SecretPowerCave).cast());
static sSpriteTemplate_SecretPowerShrub: Table<SpriteTemplate> =
    Table((&raw const crate::data::fldeff_misc::sSpriteTemplate_SecretPowerShrub).cast());
static sSpriteTemplate_SecretPowerTree: Table<SpriteTemplate> =
    Table((&raw const crate::data::fldeff_misc::sSpriteTemplate_SecretPowerTree).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerFacingPosition: MapPosition = unsafe { zeroed() };

pub unsafe fn ComputerScreenOpenEffect(increment: u16, unused: u16, priority: u8) {
    CreateComputerScreenEffectTask(
        Some(Task_ComputerScreenOpenEffect),
        increment,
        unused,
        priority,
    );
}
pub unsafe fn ComputerScreenCloseEffect(increment: u16, unused: u16, priority: u8) {
    CreateComputerScreenEffectTask(
        Some(Task_ComputerScreenCloseEffect),
        increment,
        unused,
        priority,
    );
}
pub unsafe fn IsComputerScreenOpenEffectActive() -> u8 {
    FuncIsActiveTask(Some(Task_ComputerScreenOpenEffect))
}
pub unsafe fn IsComputerScreenCloseEffectActive() -> u8 {
    FuncIsActiveTask(Some(Task_ComputerScreenCloseEffect))
}
unsafe fn CreateComputerScreenEffectTask(
    func: Option<unsafe fn(u8)>,
    increment: u16,
    unused: u16,
    priority: u8,
) {
    let taskId: u8 = CreateTask(func, priority);
    task_set(taskId, 0, 0);
    task_set(
        taskId,
        tHorzIncrement,
        (if increment == 0 { 16 } else { increment as i32 }) as i16,
    );
    task_set(
        taskId,
        tVertIncrement,
        (if increment == 0 { 20 } else { increment as i32 }) as i16,
    );
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn Task_ComputerScreenOpenEffect(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[tWinLeft] = 120;
            (*task).data[tWinRight] = 120;
            (*task).data[tWinTop] = 80;
            (*task).data[tWinBottom] = 81;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[tWinLeft] as u16) << 8 | (*task).data[tWinRight] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 0);
        }
        1 => {
            (*task).data[tBlendCnt] = GetGpuReg(REG_OFFSET_BLDCNT) as i16;
            (*task).data[tBlendY] = GetGpuReg(REG_OFFSET_BLDY) as i16;
            SetGpuReg(REG_OFFSET_BLDCNT, 191);
            SetGpuReg(REG_OFFSET_BLDY, 16);
        }
        2 => {
            (*task).data[tWinLeft] -= (*task).data[tHorzIncrement];
            (*task).data[tWinRight] += (*task).data[tHorzIncrement];
            if (*task).data[tWinLeft] < 1 || (*task).data[tWinRight] > 239 {
                (*task).data[tWinLeft] = 0;
                (*task).data[tWinRight] = DISPLAY_WIDTH as i16;
                SetGpuReg(REG_OFFSET_BLDY, 0);
                SetGpuReg(REG_OFFSET_BLDCNT, (*task).data[tBlendCnt] as u16);
                BlendPalettes(PALETTES_ALL, 0, 0);
                gPlttBufferFaded[0] = 0;
            }
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[tWinLeft] as u16) << 8 | (*task).data[tWinRight] as u16,
            );
            if (*task).data[tWinLeft] != 0 {
                return;
            }
        }
        3 => {
            (*task).data[tWinTop] -= (*task).data[tVertIncrement];
            (*task).data[tWinBottom] += (*task).data[tVertIncrement];
            if (*task).data[tWinTop] < 1 || (*task).data[tWinBottom] > 159 {
                (*task).data[tWinTop] = 0;
                (*task).data[tWinBottom] = DISPLAY_HEIGHT as i16;
                ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            if (*task).data[tWinTop] != 0 {
                return;
            }
        }
        _ => {
            SetGpuReg(REG_OFFSET_BLDCNT, (*task).data[tBlendCnt] as u16);
            DestroyTask(taskId);
            return;
        }
    }
    (*task).data[0] += 1;
}
pub(crate) unsafe fn Task_ComputerScreenCloseEffect(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            gPlttBufferFaded[0] = 0;
        }
        1 => {
            (*task).data[tWinLeft] = 0;
            (*task).data[tWinRight] = DISPLAY_WIDTH as i16;
            (*task).data[tWinTop] = 0;
            (*task).data[tWinBottom] = DISPLAY_HEIGHT as i16;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[tWinLeft] as u16) << 8 | (*task).data[tWinRight] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 0);
        }
        2 => {
            (*task).data[tWinTop] += (*task).data[tVertIncrement];
            (*task).data[tWinBottom] -= (*task).data[tVertIncrement];
            if (*task).data[tWinTop] >= 80 || (*task).data[tWinBottom] <= 81 {
                (*task).data[tWinTop] = 80;
                (*task).data[tWinBottom] = 81;
                SetGpuReg(REG_OFFSET_BLDCNT, 191);
                SetGpuReg(REG_OFFSET_BLDY, 16);
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            if (*task).data[tWinTop] != 80 {
                return;
            }
        }
        3 => {
            (*task).data[tWinLeft] += (*task).data[tHorzIncrement];
            (*task).data[tWinRight] -= (*task).data[tHorzIncrement];
            if (*task).data[tWinLeft] >= 120 || (*task).data[tWinRight] <= 120 {
                (*task).data[tWinLeft] = 120;
                (*task).data[tWinRight] = 120;
                BlendPalettes(PALETTES_ALL, 16, 0);
                gPlttBufferFaded[0] = 0;
            }
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[tWinLeft] as u16) << 8 | (*task).data[tWinRight] as u16,
            );
            if (*task).data[tWinLeft] != 120 {
                return;
            }
        }
        _ => {
            ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            DestroyTask(taskId);
            return;
        }
    }
    (*task).data[0] += 1;
}
unsafe fn SetCurrentSecretBase() {
    SetCurSecretBaseIdFromPosition(&raw mut gPlayerFacingPosition, gMapHeader.events);
    TrySetCurSecretBaseIndex();
}
unsafe fn AdjustSecretPowerSpritePixelOffsets() {
    if gPlayerAvatar.flags as i32 & 6 != 0 {
        match gFieldEffectArguments[1] {
            1 => {
                gFieldEffectArguments[5] = 16;
                gFieldEffectArguments[6] = 40;
            }
            2 => {
                gFieldEffectArguments[5] = 16;
                gFieldEffectArguments[6] = 8;
            }
            3 => {
                gFieldEffectArguments[5] = -8;
                gFieldEffectArguments[6] = 24;
            }
            4 => {
                gFieldEffectArguments[5] = 24;
                gFieldEffectArguments[6] = 24;
            }
            _ => {}
        }
    } else {
        match gFieldEffectArguments[1] {
            1 => {
                gFieldEffectArguments[5] = 8;
                gFieldEffectArguments[6] = 40;
            }
            2 => {
                gFieldEffectArguments[5] = 8;
                gFieldEffectArguments[6] = 8;
            }
            3 => {
                gFieldEffectArguments[5] = -8;
                gFieldEffectArguments[6] = 24;
            }
            4 => {
                gFieldEffectArguments[5] = 24;
                gFieldEffectArguments[6] = 24;
            }
            _ => {}
        }
    }
}
pub unsafe fn SetUpFieldMove_SecretPower() -> u8 {
    CheckPlayerHasSecretBase();
    if gSpecialVar_Result == 1 || GetPlayerFacingDirection() != DIR_NORTH {
        return FALSE;
    }
    GetXYCoordsOneStepInFrontOfPlayer(
        &raw mut gPlayerFacingPosition.x,
        &raw mut gPlayerFacingPosition.y,
    );
    let mb: u8 = MapGridGetMetatileBehaviorAt(
        gPlayerFacingPosition.x as i32,
        gPlayerFacingPosition.y as i32,
    ) as u8;
    if MetatileBehavior_IsSecretBaseCave(mb) == TRUE {
        SetCurrentSecretBase();
        gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
        gPostMenuFieldCallback = Some(FieldCallback_SecretBaseCave);
        return TRUE;
    }
    if MetatileBehavior_IsSecretBaseTree(mb) == TRUE {
        SetCurrentSecretBase();
        gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
        gPostMenuFieldCallback = Some(FieldCallback_SecretBaseTree);
        return TRUE;
    }
    if MetatileBehavior_IsSecretBaseShrub(mb) == TRUE {
        SetCurrentSecretBase();
        gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
        gPostMenuFieldCallback = Some(FieldCallback_SecretBaseShrub);
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn FieldCallback_SecretBaseCave() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    ScriptContext_SetupScript(
        (*crate::asmdata::SecretBase_EventScript_CaveUseSecretPower.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_UseSecretPowerCave() -> u8 {
    let taskId: u8 = CreateFieldMoveTask();
    task_set(
        taskId,
        8,
        (StartSecretBaseCaveFieldEffect as *const () as usize as u32 >> 16) as i16,
    );
    task_set(
        taskId,
        9,
        StartSecretBaseCaveFieldEffect as *const () as usize as u32 as i16,
    );
    FALSE
}
pub(crate) unsafe fn StartSecretBaseCaveFieldEffect() {
    FieldEffectActiveListRemove(FLDEFF_USE_SECRET_POWER_CAVE);
    FieldEffectStart(FLDEFF_SECRET_POWER_CAVE);
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_SecretPowerCave() -> u8 {
    AdjustSecretPowerSpritePixelOffsets();
    CreateSprite(
        (&raw const *sSpriteTemplate_SecretPowerCave).cast_mut(),
        gSprites[gPlayerAvatar.spriteId].oam.x() as i16 + gFieldEffectArguments[5] as i16,
        gSprites[gPlayerAvatar.spriteId].oam.y() as i16 + gFieldEffectArguments[6] as i16,
        148,
    );
    FALSE
}
pub(crate) unsafe fn SpriteCB_CaveEntranceInit(sprite: *mut Sprite) {
    PlaySE(SE_M_ROCK_THROW);
    (*sprite).data[0] = 0;
    (*sprite).callback = Some(SpriteCB_CaveEntranceOpen);
}
pub(crate) unsafe fn SpriteCB_CaveEntranceOpen(sprite: *mut Sprite) {
    if (*sprite).data[0] < 40 {
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) == 20
        {
            ToggleSecretBaseEntranceMetatile();
        }
    } else {
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(SpriteCB_CaveEntranceEnd);
    }
}
pub(crate) unsafe fn SpriteCB_CaveEntranceEnd(sprite: *mut Sprite) {
    FieldEffectStop(sprite, FLDEFF_SECRET_POWER_CAVE);
    ScriptContext_Enable();
}
pub(crate) unsafe fn FieldCallback_SecretBaseTree() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    ScriptContext_SetupScript(
        (*crate::asmdata::SecretBase_EventScript_TreeUseSecretPower.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_UseSecretPowerTree() -> u8 {
    let taskId: u8 = CreateFieldMoveTask();
    task_set(
        taskId,
        8,
        (StartSecretBaseTreeFieldEffect as *const () as usize as u32 >> 16) as i16,
    );
    task_set(
        taskId,
        9,
        StartSecretBaseTreeFieldEffect as *const () as usize as u32 as i16,
    );
    FALSE
}
pub(crate) unsafe fn StartSecretBaseTreeFieldEffect() {
    FieldEffectActiveListRemove(FLDEFF_USE_SECRET_POWER_TREE);
    FieldEffectStart(FLDEFF_SECRET_POWER_TREE);
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_SecretPowerTree() -> u8 {
    let mb: i16 = MapGridGetMetatileBehaviorAt(
        gPlayerFacingPosition.x as i32,
        gPlayerFacingPosition.y as i32,
    ) as i16
        & 0xFFF;
    if mb == MB_SECRET_BASE_SPOT_TREE_LEFT {
        gFieldEffectArguments[7] = 0;
    }
    if mb == MB_SECRET_BASE_SPOT_TREE_RIGHT {
        gFieldEffectArguments[7] = 2;
    }
    AdjustSecretPowerSpritePixelOffsets();
    CreateSprite(
        (&raw const *sSpriteTemplate_SecretPowerTree).cast_mut(),
        gSprites[gPlayerAvatar.spriteId].oam.x() as i16 + gFieldEffectArguments[5] as i16,
        gSprites[gPlayerAvatar.spriteId].oam.y() as i16 + gFieldEffectArguments[6] as i16,
        148,
    );
    if gFieldEffectArguments[7] == 1 || gFieldEffectArguments[7] == 3 {
        ToggleSecretBaseEntranceMetatile();
    }
    FALSE
}
pub(crate) unsafe fn SpriteCB_TreeEntranceInit(sprite: *mut Sprite) {
    PlaySE(SE_M_SCRATCH);
    (*sprite).animNum = gFieldEffectArguments[7] as u8;
    (*sprite).data[0] = 0;
    (*sprite).callback = Some(SpriteCB_TreeEntranceOpen);
}
pub(crate) unsafe fn SpriteCB_TreeEntranceOpen(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    if (*sprite).data[0] >= 40 {
        if gFieldEffectArguments[7] == 0 || gFieldEffectArguments[7] == 2 {
            ToggleSecretBaseEntranceMetatile();
        }
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(SpriteCB_TreeEntranceEnd);
    }
}
pub(crate) unsafe fn SpriteCB_TreeEntranceEnd(sprite: *mut Sprite) {
    FieldEffectStop(sprite, FLDEFF_SECRET_POWER_TREE);
    ScriptContext_Enable();
}
pub(crate) unsafe fn FieldCallback_SecretBaseShrub() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    ScriptContext_SetupScript(
        (*crate::asmdata::SecretBase_EventScript_ShrubUseSecretPower.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_UseSecretPowerShrub() -> u8 {
    let taskId: u8 = CreateFieldMoveTask();
    task_set(
        taskId,
        8,
        (StartSecretBaseShrubFieldEffect as *const () as usize as u32 >> 16) as i16,
    );
    task_set(
        taskId,
        9,
        StartSecretBaseShrubFieldEffect as *const () as usize as u32 as i16,
    );
    FALSE
}
pub(crate) unsafe fn StartSecretBaseShrubFieldEffect() {
    FieldEffectActiveListRemove(FLDEFF_USE_SECRET_POWER_SHRUB);
    FieldEffectStart(FLDEFF_SECRET_POWER_SHRUB);
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_SecretPowerShrub() -> u8 {
    AdjustSecretPowerSpritePixelOffsets();
    CreateSprite(
        (&raw const *sSpriteTemplate_SecretPowerShrub).cast_mut(),
        gSprites[gPlayerAvatar.spriteId].oam.x() as i16 + gFieldEffectArguments[5] as i16,
        gSprites[gPlayerAvatar.spriteId].oam.y() as i16 + gFieldEffectArguments[6] as i16,
        148,
    );
    FALSE
}
pub(crate) unsafe fn SpriteCB_ShrubEntranceInit(sprite: *mut Sprite) {
    PlaySE(SE_M_POISON_POWDER);
    (*sprite).data[0] = 0;
    (*sprite).callback = Some(SpriteCB_ShrubEntranceOpen);
}
pub(crate) unsafe fn SpriteCB_ShrubEntranceOpen(sprite: *mut Sprite) {
    if (*sprite).data[0] < 40 {
        (*sprite).data[0] += 1;
        if (*sprite).data[0] == 20 {
            ToggleSecretBaseEntranceMetatile();
        }
    } else {
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(SpriteCB_ShrubEntranceEnd);
    }
}
pub(crate) unsafe fn SpriteCB_ShrubEntranceEnd(sprite: *mut Sprite) {
    FieldEffectStop(sprite, FLDEFF_SECRET_POWER_SHRUB);
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_SecretBasePCTurnOn() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    let taskId: u8 = CreateTask(Some(Task_SecretBasePCTurnOn), 0);
    task_set(taskId, tX, x);
    task_set(taskId, tY, y);
    task_set(taskId, 2, 0);
    FALSE
}
pub(crate) unsafe fn Task_SecretBasePCTurnOn(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data.at(2) {
        4 | 12 => {
            MapGridSetMetatileIdAt(*data as i32, *data.at(1) as i32, METATILE_SecretBase_PC_On);
            CurrentMapDrawMetatileAt(*data as i32, *data.at(1) as i32);
        }
        8 | 16 => {
            MapGridSetMetatileIdAt(*data as i32, *data.at(1) as i32, METATILE_SecretBase_PC);
            CurrentMapDrawMetatileAt(*data as i32, *data.at(1) as i32);
        }
        20 => {
            MapGridSetMetatileIdAt(*data as i32, *data.at(1) as i32, METATILE_SecretBase_PC_On);
            CurrentMapDrawMetatileAt(*data as i32, *data.at(1) as i32);
            FieldEffectActiveListRemove(FLDEFF_PCTURN_ON);
            ScriptContext_Enable();
            DestroyTask(taskId);
            return;
        }
        _ => {}
    }
    *data.at(2) += 1;
}
#[unsafe(no_mangle)]
pub unsafe fn DoSecretBasePCTurnOffEffect() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    PlaySE(SE_PC_OFF);
    if VarGet(VAR_CURRENT_SECRET_BASE) == 0 {
        MapGridSetMetatileIdAt(x as i32, y as i32, 3616);
    } else {
        MapGridSetMetatileIdAt(x as i32, y as i32, 3617);
    }
    CurrentMapDrawMetatileAt(x as i32, y as i32);
}
pub unsafe fn PopSecretBaseBalloon(metatileId: i16, x: i16, y: i16) {
    let taskId: u8 = CreateTask(Some(Task_PopSecretBaseBalloon), 0);
    task_set(taskId, 0, metatileId);
    task_set(taskId, 1, x);
    task_set(taskId, 2, y);
    task_set(taskId, 3, 0);
    task_set(taskId, 4, 1);
}
pub(crate) unsafe fn Task_PopSecretBaseBalloon(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(3) == 6 {
        *data.at(3) = 0;
    } else {
        *data.at(3) += 1;
    }
    if *data.at(3) == 0 {
        if *data.at(4) == 2 {
            DoBalloonSoundEffect(*data);
        }
        MapGridSetMetatileIdAt(
            *data.at(1) as i32,
            *data.at(2) as i32,
            *data as u16 + *data.at(4) as u16,
        );
        CurrentMapDrawMetatileAt(*data.at(1) as i32, *data.at(2) as i32);
        if *data.at(4) == 3 {
            DestroyTask(taskId);
        } else {
            *data.at(4) += 1;
        }
    }
}
unsafe fn DoBalloonSoundEffect(metatileId: i16) {
    match metatileId {
        METATILE_SecretBase_RedBalloon => {
            PlaySE(SE_BALLOON_RED);
        }
        METATILE_SecretBase_BlueBalloon => {
            PlaySE(SE_BALLOON_BLUE);
        }
        METATILE_SecretBase_YellowBalloon => {
            PlaySE(SE_BALLOON_YELLOW);
        }
        METATILE_SecretBase_MudBall => {
            PlaySE(SE_MUD_BALL);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub fn FldEff_Nop47() -> u8 {
    FALSE
}
#[unsafe(no_mangle)]
pub fn FldEff_Nop48() -> u8 {
    FALSE
}
unsafe fn DoSecretBaseBreakableDoorEffect(x: i16, y: i16) {
    PlaySE(SE_BREAKABLE_DOOR);
    MapGridSetMetatileIdAt(
        x as i32,
        y as i32,
        METATILE_SecretBase_BreakableDoor_BottomOpen,
    );
    MapGridSetMetatileIdAt(
        x as i32,
        y as i32 - 1,
        METATILE_SecretBase_BreakableDoor_TopOpen,
    );
    CurrentMapDrawMetatileAt(x as i32, y as i32);
    CurrentMapDrawMetatileAt(x as i32, y as i32 - 1);
}
pub(crate) unsafe fn Task_ShatterSecretBaseBreakableDoor(taskId: u8) {
    if task_get(taskId, 0) == 7 {
        DoSecretBaseBreakableDoorEffect(task_get(taskId, 1), task_get(taskId, 2));
        DestroyTask(taskId);
    } else {
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    }
}
pub unsafe fn ShatterSecretBaseBreakableDoor(x: i16, y: i16) {
    let dir: u8 = GetPlayerFacingDirection();
    if dir == DIR_SOUTH {
        DoSecretBaseBreakableDoorEffect(x, y);
    } else if dir == DIR_NORTH {
        let taskId: u8 = CreateTask(Some(Task_ShatterSecretBaseBreakableDoor), 5);
        task_set(taskId, 0, 0);
        task_set(taskId, 1, x);
        task_set(taskId, 2, y);
    }
}
pub(crate) unsafe fn Task_SecretBaseMusicNoteMatSound(taskId: u8) {
    if task_get(taskId, 1) == 7 {
        match task_get(taskId, tMetatileID) {
            METATILE_SecretBase_NoteMat_C_Low => {
                PlaySE(SE_NOTE_C);
            }
            METATILE_SecretBase_NoteMat_D => {
                PlaySE(SE_NOTE_D);
            }
            METATILE_SecretBase_NoteMat_E => {
                PlaySE(SE_NOTE_E);
            }
            METATILE_SecretBase_NoteMat_F => {
                PlaySE(SE_NOTE_F);
            }
            METATILE_SecretBase_NoteMat_G => {
                PlaySE(SE_NOTE_G);
            }
            METATILE_SecretBase_NoteMat_A => {
                PlaySE(SE_NOTE_A);
            }
            METATILE_SecretBase_NoteMat_B => {
                PlaySE(SE_NOTE_B);
            }
            METATILE_SecretBase_NoteMat_C_High => {
                PlaySE(SE_NOTE_C_HIGH);
            }
            _ => {}
        }
        DestroyTask(taskId);
    } else {
        task_set(taskId, 1, task_get(taskId, 1) + 1);
    }
}
pub unsafe fn PlaySecretBaseMusicNoteMatSound(metatileId: i16) {
    let taskId: u8 = CreateTask(Some(Task_SecretBaseMusicNoteMatSound), 5);
    task_set(taskId, tMetatileID, metatileId);
    task_set(taskId, 1, 0);
}
pub(crate) unsafe fn SpriteCB_GlitterMatSparkle(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    if (*sprite).data[0] == 8 {
        PlaySE(SE_M_HEAL_BELL);
    }
    if (*sprite).data[0] >= 32 {
        DestroySprite(sprite);
    }
}
pub unsafe fn DoSecretBaseGlitterMatSparkle() {
    let mut x: i16 = gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x;
    let mut y: i16 = gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y;
    SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8, 4);
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[22],
        x,
        y,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
        gSprites[spriteId].oam.set_priority(1);
        gSprites[spriteId].oam.set_paletteNum(5);
        gSprites[spriteId].callback = Some(SpriteCB_GlitterMatSparkle);
        gSprites[spriteId].data[0] = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_SandPillar() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    LockPlayerFieldControls();
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    gFieldEffectArguments[5] = x as i32;
    gFieldEffectArguments[6] = y as i32;
    match GetPlayerFacingDirection() {
        DIR_SOUTH => {
            CreateSprite(
                (&raw const *sSpriteTemplate_SandPillar).cast_mut(),
                gSprites[gPlayerAvatar.spriteId].oam.x() as i16 + 8,
                gSprites[gPlayerAvatar.spriteId].oam.y() as i16 + 32,
                0,
            );
        }
        DIR_NORTH => {
            CreateSprite(
                (&raw const *sSpriteTemplate_SandPillar).cast_mut(),
                gSprites[gPlayerAvatar.spriteId].oam.x() as i16 + 8,
                gSprites[gPlayerAvatar.spriteId].oam.y() as i16,
                148,
            );
        }
        DIR_WEST => {
            CreateSprite(
                (&raw const *sSpriteTemplate_SandPillar).cast_mut(),
                gSprites[gPlayerAvatar.spriteId].oam.x() as i16 - 8,
                gSprites[gPlayerAvatar.spriteId].oam.y() as i16 + 16,
                148,
            );
        }
        DIR_EAST => {
            CreateSprite(
                (&raw const *sSpriteTemplate_SandPillar).cast_mut(),
                gSprites[gPlayerAvatar.spriteId].oam.x() as i16 + 24,
                gSprites[gPlayerAvatar.spriteId].oam.y() as i16 + 16,
                148,
            );
        }
        _ => {}
    }
    FALSE
}
pub(crate) unsafe fn SpriteCB_SandPillar_BreakTop(sprite: *mut Sprite) {
    PlaySE(SE_M_ROCK_THROW);
    if MapGridGetMetatileIdAt(gFieldEffectArguments[5], gFieldEffectArguments[6] - 1)
        == METATILE_SecretBase_SandOrnament_TopWall
    {
        MapGridSetMetatileIdAt(gFieldEffectArguments[5], gFieldEffectArguments[6] - 1, 3586);
    } else {
        MapGridSetMetatileIdAt(
            gFieldEffectArguments[5],
            gFieldEffectArguments[6] - 1,
            METATILE_SecretBase_SandOrnament_BrokenTop,
        );
    }
    MapGridSetMetatileIdAt(
        gFieldEffectArguments[5],
        gFieldEffectArguments[6],
        METATILE_SecretBase_Ground,
    );
    CurrentMapDrawMetatileAt(gFieldEffectArguments[5], gFieldEffectArguments[6] - 1);
    CurrentMapDrawMetatileAt(gFieldEffectArguments[5], gFieldEffectArguments[6]);
    (*sprite).data[0] = 0;
    (*sprite).callback = Some(SpriteCB_SandPillar_BreakBase);
}
pub(crate) unsafe fn SpriteCB_SandPillar_BreakBase(sprite: *mut Sprite) {
    if (*sprite).data[0] < 18 {
        (*sprite).data[0] += 1;
    } else {
        MapGridSetMetatileIdAt(gFieldEffectArguments[5], gFieldEffectArguments[6], 3724);
        CurrentMapDrawMetatileAt(gFieldEffectArguments[5], gFieldEffectArguments[6]);
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(SpriteCB_SandPillar_End);
    }
}
pub(crate) unsafe fn SpriteCB_SandPillar_End(sprite: *mut Sprite) {
    FieldEffectStop(sprite, FLDEFF_SAND_PILLAR);
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe fn InteractWithShieldOrTVDecoration() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    let metatileId: i32 = MapGridGetMetatileIdAt(x as i32, y as i32);
    match metatileId {
        METATILE_SecretBase_GoldShield_Base1 => {
            ConvertIntToDecimalStringN(gStringVar1.as_mut_ptr(), 100, STR_CONV_MODE_LEFT_ALIGN, 3);
            StringCopy(
                gStringVar2.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_Gold).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            gSpecialVar_Result = 0;
            if VarGet(VAR_CURRENT_SECRET_BASE) == 0 {
                return;
            }
            VarSet(
                VAR_SECRET_BASE_LOW_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_GOLD_SHIELD,
            );
        }
        METATILE_SecretBase_SilverShield_Base1 => {
            ConvertIntToDecimalStringN(gStringVar1.as_mut_ptr(), 50, STR_CONV_MODE_LEFT_ALIGN, 2);
            StringCopy(
                gStringVar2.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_Silver).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            gSpecialVar_Result = 0;
            if VarGet(VAR_CURRENT_SECRET_BASE) == 0 {
                return;
            }
            VarSet(
                VAR_SECRET_BASE_LOW_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_SILVER_SHIELD,
            );
        }
        METATILE_SecretBase_TV => {
            gSpecialVar_Result = 1;
            if VarGet(VAR_CURRENT_SECRET_BASE) == 0 {
                return;
            }
            VarSet(
                VAR_SECRET_BASE_LOW_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_TV,
            );
        }
        METATILE_SecretBase_RoundTV => {
            gSpecialVar_Result = 2;
            if VarGet(VAR_CURRENT_SECRET_BASE) == 0 {
                return;
            }
            VarSet(
                VAR_SECRET_BASE_LOW_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_TV,
            );
        }
        METATILE_SecretBase_CuteTV => {
            gSpecialVar_Result = 3;
            if VarGet(VAR_CURRENT_SECRET_BASE) == 0 {
                return;
            }
            VarSet(
                VAR_SECRET_BASE_LOW_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_TV,
            );
        }
        _ => {}
    }
}
pub unsafe fn IsLargeBreakableDecoration(metatileId: u16, checkBase: u8) -> u8 {
    if CurMapIsSecretBase() == 0 {
        return FALSE;
    }
    if checkBase == 0 {
        if metatileId == METATILE_SecretBase_SandOrnament_Top
            || metatileId == METATILE_SecretBase_SandOrnament_TopWall as u16
        {
            return TRUE;
        }
        if metatileId == METATILE_SecretBase_BreakableDoor_TopClosed {
            return TRUE;
        }
    } else {
        if metatileId == METATILE_SecretBase_SandOrnament_Base1 {
            return TRUE;
        }
        if metatileId == METATILE_SecretBase_BreakableDoor_BottomClosed {
            return TRUE;
        }
    }
    FALSE
}
pub(crate) unsafe fn Task_FieldPoisonEffect(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            *data.at(1) += 2;
            if *data.at(1) > 8 {
                *data += 1;
            }
        }
        1 => {
            *data.at(1) -= 2;
            if *data.at(1) == 0 {
                *data += 1;
            }
        }
        2 => {
            DestroyTask(taskId);
            return;
        }
        _ => {}
    }
    SetGpuReg(
        REG_OFFSET_MOSAIC,
        (*data.at(1) as u16) << 4 | *data.at(1) as u16,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn FldEffPoison_Start() {
    PlaySE(SE_FIELD_POISON);
    CreateTask(Some(Task_FieldPoisonEffect), 80);
}
pub unsafe fn FldEffPoison_IsActive() -> u32 {
    FuncIsActiveTask(Some(Task_FieldPoisonEffect)) as u32
}
pub(crate) unsafe fn Task_WateringBerryTreeAnim(taskId: u8) {
    task_set_func(taskId, Some(Task_WateringBerryTreeAnim_Start));
}
pub(crate) unsafe fn Task_WateringBerryTreeAnim_Start(taskId: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(playerObjEvent) == 0
        || ObjectEventClearHeldMovementIfFinished(playerObjEvent) != 0
    {
        SetPlayerAvatarWatering(GetPlayerFacingDirection());
        ObjectEventSetHeldMovement(
            playerObjEvent,
            GetWalkInPlaceNormalMovementAction(GetPlayerFacingDirection() as u32),
        );
        task_set_func(taskId, Some(Task_WateringBerryTreeAnim_Continue));
    }
}
pub(crate) unsafe fn Task_WateringBerryTreeAnim_Continue(taskId: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventClearHeldMovementIfFinished(playerObjEvent) != 0 {
        let value: i16 = {
            let t1 = task_get(taskId, 1);
            task_set(taskId, 1, task_get(taskId, 1) + 1);
            t1
        };
        if value < 10 {
            ObjectEventSetHeldMovement(
                playerObjEvent,
                GetWalkInPlaceNormalMovementAction(GetPlayerFacingDirection() as u32),
            );
        } else {
            task_set_func(taskId, Some(Task_WateringBerryTreeAnim_End));
        }
    }
}
pub(crate) unsafe fn Task_WateringBerryTreeAnim_End(taskId: u8) {
    SetPlayerAvatarTransitionFlags(GetPlayerAvatarFlags() as u16);
    DestroyTask(taskId);
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe fn DoWateringBerryTreeAnim() {
    CreateTask(Some(Task_WateringBerryTreeAnim), 80);
}
pub unsafe fn CreateRecordMixingLights() -> u8 {
    LoadSpritePalette((&raw const *sSpritePalette_RecordMixLights).cast_mut());
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_RecordMixLights).cast_mut(),
        0,
        0,
        82,
    );
    if spriteId == MAX_SPRITES {
        return MAX_SPRITES;
    } else {
        let sprite: *mut Sprite = &raw mut gSprites[spriteId];
        GetMapCoordsFromSpritePos(16, 13, &raw mut (*sprite).x, &raw mut (*sprite).y);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).x += 16;
        (*sprite).y += 2;
    }
    spriteId
}
pub unsafe fn DestroyRecordMixingLights() {
    for i in 0..(MAX_SPRITES as i32) {
        if gSprites[i].template == (&raw const *sSpriteTemplate_RecordMixLights).cast_mut() {
            FreeSpritePalette(&raw mut gSprites[i]);
            DestroySprite(&raw mut gSprites[i]);
        }
    }
}
