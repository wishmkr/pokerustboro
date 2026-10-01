//! Translated from `src/mirage_tower.c` by tools/rustport/c2rs.py.
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
    unused_variables
)]

use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, SetBgAttribute, ShowBg, UnsetBgTilemapBuffer,
};
use crate::bg::{CopyToBgTilemapBufferRect_ChangePalette, LoadBgTiles};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagClear, FlagGet, FlagSet, VarGet};
use crate::event_object_movement::TryGetObjectEventIdByLocalIdAndMap;
use crate::field_camera::{
    DrawWholeMapView, InstallCameraPanAheadCallback, SetCameraPanning, SetCameraPanningCallback,
};
use crate::field_player_avatar::{gObjectEvents, gPlayerAvatar};
use crate::fieldmap::MapGridSetMetatileIdAt;
use crate::gpu_regs::{SetGpuReg, SetGpuRegBits};
use crate::load_save::gSaveBlock1Ptr;
use crate::menu::InitStandardTextBoxWindows;
use crate::palette_util::{
    InitPulseBlend, InitPulseBlendPaletteSettings, MarkUsedPulseBlendPalettes,
    UnloadUsedPulseBlendPalettes, UnmarkUsedPulseBlendPalettes, UpdatePulseBlend,
};
use crate::random::Random;
use crate::script::ScriptContext_Enable;
use crate::sound::PlaySE;
use crate::sprite::FreeSpriteTilesByTag;
use crate::sprite::gSprites;
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::{task_get, task_set, task_set_func};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::FreeAllWindowBuffers;
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `LoadSpriteSheets` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheets(a0: *mut SpriteSheet) {
    unsafe {
        crate::sprite::LoadSpriteSheets(a0 as _);
    }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
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
const sIndex: usize = 0;
const tState: usize = 0;
const tXShakeOffset: usize = 0;
const sYOffset: usize = 1;
const tTimer: usize = 1;
const tNumShakes: usize = 2;
const tShakeDelay: usize = 3;
const tYShakeOffset: usize = 4;
// Data tables (translate with cdata.py): sMirageTower_Gfx sMirageTowerTilemap sFossil_Pal sFossil_Gfx sMirageTowerCrumbles_Gfx sMirageTowerCrumbles_Palette sCeilingCrumblePositions sCeilingCrumbleSpriteSheets sInvisibleMirageTowerMetatiles sAnim_FallingFossil sOamData_FallingFossil sAnims_FallingFossil sSpriteTemplate_FallingFossil gMirageTowerPulseBlendSettings sAnim_CeilingCrumbleSmall sAnims_CeilingCrumbleSmall sOamData_CeilingCrumbleSmall sSpriteTemplate_CeilingCrumbleSmall sAnim_CeilingCrumbleLarge sAnims_CeilingCrumbleLarge sOamData_CeilingCrumbleLarge sSpriteTemplate_CeilingCrumbleLarge

/// `struct FallAnim_Fossil`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FallAnim_Fossil {
    pub frameImageTiles: *mut u8,
    pub frameImage: *mut SpriteFrameImage,
    pub spriteId: u8,
    pub disintegrateRand: *mut u16,
    pub disintegrateIdx: u16,
}

unsafe impl Sync for FallAnim_Fossil {}

/// `struct FallAnim_Tower`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FallAnim_Tower {
    pub disintegrateRand: *mut u8,
    pub disintegrateIdx: u8,
}

unsafe impl Sync for FallAnim_Tower {}

/// `struct BgRegOffsets`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BgRegOffsets {
    pub bgHOFS: u16,
    pub bgVOFS: u16,
}

unsafe impl Sync for BgRegOffsets {}

/// `struct MirageTowerPulseBlend`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MirageTowerPulseBlend {
    pub taskId: u8,
    pub pulseBlend: PulseBlend,
}

unsafe impl Sync for MirageTowerPulseBlend {}

/// `struct MetatileCoords`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct MetatileCoords {
    pub x: u8,
    pub y: u8,
    pub metatileId: u16,
}

unsafe impl Sync for MetatileCoords {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<FallAnim_Fossil>() == 20);
    assert!(offset_of!(FallAnim_Fossil, frameImageTiles) == 0);
    assert!(offset_of!(FallAnim_Fossil, frameImage) == 4);
    assert!(offset_of!(FallAnim_Fossil, spriteId) == 8);
    assert!(offset_of!(FallAnim_Fossil, disintegrateRand) == 12);
    assert!(offset_of!(FallAnim_Fossil, disintegrateIdx) == 16);
    assert!(size_of::<FallAnim_Tower>() == 8);
    assert!(offset_of!(FallAnim_Tower, disintegrateRand) == 0);
    assert!(offset_of!(FallAnim_Tower, disintegrateIdx) == 4);
    assert!(size_of::<BgRegOffsets>() == 4);
    assert!(offset_of!(BgRegOffsets, bgHOFS) == 0);
    assert!(offset_of!(BgRegOffsets, bgVOFS) == 2);
    assert!(size_of::<MirageTowerPulseBlend>() == 200);
    assert!(offset_of!(MirageTowerPulseBlend, taskId) == 0);
    assert!(offset_of!(MirageTowerPulseBlend, pulseBlend) == 4);
    assert!(size_of::<MetatileCoords>() == 4);
    assert!(offset_of!(MetatileCoords, x) == 0);
    assert!(offset_of!(MetatileCoords, y) == 1);
    assert!(offset_of!(MetatileCoords, metatileId) == 2);
};

const FOSSIL_DISINTEGRATE_LENGTH: u16 = 256;
const INNER_BUFFER_LENGTH: u32 = 48;
const OUTER_BUFFER_LENGTH: i32 = 96;
const TAG_CEILING_CRUMBLE: u16 = 4000;

static gMirageTowerPulseBlendSettings: Table<PulseBlendSettings> =
    Table((&raw const crate::data::mirage_tower::gMirageTowerPulseBlendSettings).cast());
static sCeilingCrumblePositions: Table<CArray<CArray<i16, 3>, 8>> =
    Table((&raw const crate::data::mirage_tower::sCeilingCrumblePositions).cast());
static sCeilingCrumbleSpriteSheets: Table<CArray<SpriteSheet, 2>> =
    Table((&raw const crate::data::mirage_tower::sCeilingCrumbleSpriteSheets).cast());
static sFossil_Gfx: Table<CArray<u8, 128>> =
    Table((&raw const crate::data::mirage_tower::sFossil_Gfx).cast());
static sInvisibleMirageTowerMetatiles: Table<CArray<MetatileCoords, 18>> =
    Table((&raw const crate::data::mirage_tower::sInvisibleMirageTowerMetatiles).cast());
static sMirageTowerTilemap: Table<CArray<u16, 72>> =
    Table((&raw const crate::data::mirage_tower::sMirageTowerTilemap).cast());
static sMirageTower_Gfx: Table<CArray<u8, 2336>> =
    Table((&raw const crate::data::mirage_tower::sMirageTower_Gfx).cast());
static sSpriteTemplate_CeilingCrumbleLarge: Table<SpriteTemplate> =
    Table((&raw const crate::data::mirage_tower::sSpriteTemplate_CeilingCrumbleLarge).cast());
static sSpriteTemplate_CeilingCrumbleSmall: Table<SpriteTemplate> =
    Table((&raw const crate::data::mirage_tower::sSpriteTemplate_CeilingCrumbleSmall).cast());
static sSpriteTemplate_FallingFossil: Table<SpriteTemplate> =
    Table((&raw const crate::data::mirage_tower::sSpriteTemplate_FallingFossil).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMirageTowerGfxBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMirageTowerTilemapBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFallingFossil: *mut FallAnim_Fossil = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFallingTower: *mut FallAnim_Tower = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBgShakeOffsets: *mut BgRegOffsets = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMirageTowerPulseBlend: *mut MirageTowerPulseBlend = null_mut();
pub(crate) static mut sDebug_DisintegrationData: Aligned<CArray<u16, 8>> =
    Aligned(unsafe { zeroed() });

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub unsafe fn IsMirageTowerVisible() -> u8 {
    if !((*gSaveBlock1Ptr).location.mapGroup == 0 && (*gSaveBlock1Ptr).location.mapNum == 26) {
        return FALSE;
    }
    FlagGet(FLAG_MIRAGE_TOWER_VISIBLE)
}
pub(crate) unsafe fn UpdateMirageTowerPulseBlend(taskId: u8) {
    UpdatePulseBlend(&raw mut (*sMirageTowerPulseBlend).pulseBlend);
}
pub unsafe fn ClearMirageTowerPulseBlend() {
    sMirageTowerPulseBlend = null_mut();
}
#[unsafe(no_mangle)]
pub unsafe fn TryStartMirageTowerPulseBlendEffect() {
    if !sMirageTowerPulseBlend.is_null() {
        sMirageTowerPulseBlend = null_mut();
        return;
    }
    if (*gSaveBlock1Ptr).location.mapGroup != 0
        || (*gSaveBlock1Ptr).location.mapNum != 26
        || FlagGet(FLAG_MIRAGE_TOWER_VISIBLE) == 0
    {
        return;
    }
    sMirageTowerPulseBlend = AllocZeroed(200) as *mut MirageTowerPulseBlend;
    InitPulseBlend(&raw mut (*sMirageTowerPulseBlend).pulseBlend);
    InitPulseBlendPaletteSettings(
        &raw mut (*sMirageTowerPulseBlend).pulseBlend,
        (&raw const *gMirageTowerPulseBlendSettings).cast_mut(),
    );
    MarkUsedPulseBlendPalettes(&raw mut (*sMirageTowerPulseBlend).pulseBlend, 0x1, 1);
    (*sMirageTowerPulseBlend).taskId = CreateTask(Some(UpdateMirageTowerPulseBlend), 0xFF);
}
#[unsafe(no_mangle)]
pub unsafe fn ClearMirageTowerPulseBlendEffect() {
    if (*gSaveBlock1Ptr).location.mapGroup != 0
        || (*gSaveBlock1Ptr).location.mapNum != 26
        || FlagGet(FLAG_MIRAGE_TOWER_VISIBLE) == 0
        || sMirageTowerPulseBlend.is_null()
    {
        return;
    }
    if FuncIsActiveTask(Some(UpdateMirageTowerPulseBlend)) != 0 {
        DestroyTask((*sMirageTowerPulseBlend).taskId);
    }
    UnmarkUsedPulseBlendPalettes(&raw mut (*sMirageTowerPulseBlend).pulseBlend, 0x1, 1);
    UnloadUsedPulseBlendPalettes(&raw mut (*sMirageTowerPulseBlend).pulseBlend, 0x1, 1);
    Free(sMirageTowerPulseBlend as *mut c_void);
    sMirageTowerPulseBlend = null_mut();
}
#[unsafe(no_mangle)]
pub unsafe fn SetMirageTowerVisibility() {
    if VarGet(VAR_MIRAGE_TOWER_STATE) != 0 {
        FlagClear(FLAG_MIRAGE_TOWER_VISIBLE);
        return;
    }
    let rand: u16 = Random();
    let mut visible: u8 = rand as u8 & 1;
    if FlagGet(FLAG_FORCE_MIRAGE_TOWER_VISIBLE) == TRUE {
        visible = TRUE;
    }
    if visible != 0 {
        FlagSet(FLAG_MIRAGE_TOWER_VISIBLE);
        TryStartMirageTowerPulseBlendEffect();
        return;
    }
    FlagClear(FLAG_MIRAGE_TOWER_VISIBLE);
}
#[unsafe(no_mangle)]
pub unsafe fn StartPlayerDescendMirageTower() {
    CreateTask(Some(PlayerDescendMirageTower), 8);
}
pub(crate) unsafe fn PlayerDescendMirageTower(taskId: u8) {
    let mut objectEventId: u8 = 0;
    TryGetObjectEventIdByLocalIdAndMap(
        LOCALID_ROUTE111_PLAYER_FALLING,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        &raw mut objectEventId,
    );
    let fallingPlayer: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
    gSprites[(*fallingPlayer).spriteId].y2 += 4;
    let player: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if gSprites[(*fallingPlayer).spriteId].y as i32 + gSprites[(*fallingPlayer).spriteId].y2 as i32
        >= gSprites[(*player).spriteId].y as i32 + gSprites[(*player).spriteId].y2 as i32
    {
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
unsafe fn StartScreenShake(yShakeOffset: u8, xShakeOffset: u8, numShakes: u8, shakeDelay: u8) {
    let taskId: u8 = CreateTask(Some(DoScreenShake), 9);
    task_set(taskId, tXShakeOffset, xShakeOffset as i16);
    task_set(taskId, tTimer, 0);
    task_set(taskId, tNumShakes, numShakes as i16);
    task_set(taskId, tShakeDelay, shakeDelay as i16);
    task_set(taskId, tYShakeOffset, yShakeOffset as i16);
    SetCameraPanningCallback(None);
    PlaySE(SE_M_STRENGTH);
}
pub(crate) unsafe fn DoScreenShake(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(1) += 1;
    if rem_i32(*data.at(1) as i32, *data.at(3) as i32) == 0 {
        *data.at(1) = 0;
        *data.at(2) -= 1;
        *data = -*data;
        *data.at(4) = -*data.at(4);
        SetCameraPanning(*data, *data.at(4));
        if *data.at(2) == 0 {
            IncrementCeilingCrumbleFinishedCount();
            DestroyTask(taskId);
            InstallCameraPanAheadCallback();
        }
    }
}
unsafe fn IncrementCeilingCrumbleFinishedCount() {
    let taskId: u8 = FindTaskIdByFunc(Some(WaitCeilingCrumble));
    if taskId != TASK_NONE {
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DoMirageTowerCeilingCrumble() {
    LoadSpriteSheets(sCeilingCrumbleSpriteSheets.as_ptr().cast_mut());
    CreateCeilingCrumbleSprites();
    CreateTask(Some(WaitCeilingCrumble), 8);
    StartScreenShake(2, 1, 16, 3);
}
pub(crate) unsafe fn WaitCeilingCrumble(taskId: u8) {
    let data: *mut u16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut u16;
    *data.at(1) += 1;
    if *data.at(1) == 1000 || *data == 17 {
        task_set_func(taskId, Some(FinishCeilingCrumbleTask));
    }
}
pub(crate) unsafe fn FinishCeilingCrumbleTask(taskId: u8) {
    FreeSpriteTilesByTag(TAG_CEILING_CRUMBLE);
    DestroyTask(taskId);
    ScriptContext_Enable();
}
unsafe fn CreateCeilingCrumbleSprites() {
    let mut spriteId: u8 = 0;
    for i in 0..8u8 {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_CeilingCrumbleLarge).cast_mut(),
            sCeilingCrumblePositions[i][0] + 120,
            sCeilingCrumblePositions[i][1],
            8,
        );
        gSprites[spriteId].oam.set_priority(0);
        gSprites[spriteId].oam.set_paletteNum(PALSLOT_PLAYER as u16);
        gSprites[spriteId].data[sIndex] = i as i16;
    }
    for i in 0..8u8 {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_CeilingCrumbleSmall).cast_mut(),
            sCeilingCrumblePositions[i][0] + 115,
            sCeilingCrumblePositions[i][1] - 3,
            8,
        );
        gSprites[spriteId].oam.set_priority(0);
        gSprites[spriteId].oam.set_paletteNum(PALSLOT_PLAYER as u16);
        gSprites[spriteId].data[sIndex] = i as i16;
    }
}
pub(crate) unsafe fn SpriteCB_CeilingCrumble(sprite: *mut Sprite) {
    (*sprite).data[sYOffset] += 2;
    (*sprite).y2 = (*sprite).data[sYOffset] / 2;
    if (*sprite).y as i32 + (*sprite).y2 as i32
        > sCeilingCrumblePositions[(*sprite).data[sIndex]][2] as i32
    {
        DestroySprite(sprite);
        IncrementCeilingCrumbleFinishedCount();
    }
}
unsafe fn SetInvisibleMirageTowerMetatiles() {
    for i in 0..18u8 {
        MapGridSetMetatileIdAt(
            sInvisibleMirageTowerMetatiles[i].x as i32 + MAP_OFFSET,
            sInvisibleMirageTowerMetatiles[i].y as i32 + MAP_OFFSET,
            sInvisibleMirageTowerMetatiles[i].metatileId,
        );
    }
    DrawWholeMapView();
}
#[unsafe(no_mangle)]
pub unsafe fn StartMirageTowerDisintegration() {
    CreateTask(Some(DoMirageTowerDisintegration), 9);
}
#[unsafe(no_mangle)]
pub unsafe fn StartMirageTowerShake() {
    CreateTask(Some(InitMirageTowerShake), 9);
}
#[unsafe(no_mangle)]
pub unsafe fn StartMirageTowerFossilFallAndSink() {
    CreateTask(Some(Task_FossilFallAndSink), 9);
}
unsafe fn SetBgShakeOffsets() {
    SetGpuReg(REG_OFFSET_BG0HOFS, (*sBgShakeOffsets).bgHOFS);
    SetGpuReg(REG_OFFSET_BG0VOFS, (*sBgShakeOffsets).bgVOFS);
}
pub(crate) unsafe fn UpdateBgShake(taskId: u8) {
    if task_get(taskId, 0) == 0 {
        (*sBgShakeOffsets).bgHOFS = (*sBgShakeOffsets).bgHOFS.wrapping_neg();
        task_set(taskId, 0, 2);
        SetBgShakeOffsets();
    } else {
        task_set(taskId, 0, task_get(taskId, 0) - 1);
    }
}
pub(crate) unsafe fn InitMirageTowerShake(taskId: u8) {
    let mut zero: u8 = 0;
    match task_get(taskId, tState) {
        0 => {
            FreeAllWindowBuffers();
            SetBgAttribute(0, BG_ATTR_PRIORITY, 2);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 => {
            sMirageTowerGfxBuffer = AllocZeroed(2336) as *mut u8;
            sMirageTowerTilemapBuffer = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
            ChangeBgX(0, 0, BG_COORD_SET);
            ChangeBgY(0, 0, BG_COORD_SET);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        2 => {
            CpuSet(
                sMirageTower_Gfx.as_ptr().cast_mut() as *mut c_void,
                sMirageTowerGfxBuffer as *mut c_void,
                1168,
            );
            LoadBgTiles(0, sMirageTowerGfxBuffer as *mut c_void, 2336, 0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        3 => {
            SetBgTilemapBuffer(0, sMirageTowerTilemapBuffer as *mut c_void);
            CopyToBgTilemapBufferRect_ChangePalette(
                0,
                (&raw const *sMirageTowerTilemap).cast_mut() as *mut c_void,
                12,
                29,
                6,
                12,
                17,
            );
            CopyBgTilemapBufferToVram(0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        4 => {
            ShowBg(0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        5 => {
            SetInvisibleMirageTowerMetatiles();
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        6 => {
            sBgShakeOffsets = Alloc(4) as *mut BgRegOffsets;
            zero = 0;
            (*sBgShakeOffsets).bgHOFS = 2;
            (*sBgShakeOffsets).bgVOFS = zero as u16;
            CreateTask(Some(UpdateBgShake), 10);
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
        _ => {}
    }
}
pub(crate) unsafe fn DoMirageTowerDisintegration(taskId: u8) {
    let mut bgShakeTaskId: u8 = 0;
    let mut index: u8 = 0;
    'l1: {
        match task_get(taskId, tState) {
            1 => {
                sFallingTower = AllocZeroed(768) as *mut FallAnim_Tower;
            }
            3 => {
                if task_get(taskId, 3) <= 95 {
                    if task_get(taskId, 1) > 1 {
                        index = task_get(taskId, 3) as u8;
                        (*sFallingTower.at(index)).disintegrateRand =
                            Alloc(INNER_BUFFER_LENGTH) as *mut u8;
                        for i in 0..=47u16 {
                            *(*sFallingTower.at(index)).disintegrateRand.at(i) = i as u8;
                        }
                        for i in 0..=47u16 {
                            let rand1: u16 = (Random() as i32 % 48) as u16;
                            let rand2: u16 = (Random() as i32 % 48) as u16;
                            let temp: u16 =
                                *(*sFallingTower.at(index)).disintegrateRand.at(rand2) as u16;
                            *(*sFallingTower.at(index)).disintegrateRand.at(rand2) =
                                *(*sFallingTower.at(index)).disintegrateRand.at(rand1);
                            *(*sFallingTower.at(index)).disintegrateRand.at(rand1) = temp as u8;
                        }
                        if task_get(taskId, 3) <= 95 {
                            task_set(taskId, 3, task_get(taskId, 3) + 1);
                        }
                        task_set(taskId, 1, 0);
                    }
                    task_set(taskId, 1, task_get(taskId, 1) + 1);
                }
                index = task_get(taskId, 3) as u8;
                for i in (task_get(taskId, 2) as u8 as u16)..(index as u16) {
                    for j in 0..1u8 {
                        UpdateDisintegrationEffect(
                            sMirageTowerGfxBuffer,
                            (95 - i) * INNER_BUFFER_LENGTH as u16
                                + *(*sFallingTower.at(i)).disintegrateRand.at({
                                    let t1 = (*sFallingTower.at(i)).disintegrateIdx;
                                    (*sFallingTower.at(i)).disintegrateIdx += 1;
                                    t1
                                }) as u16,
                            0,
                            INNER_BUFFER_LENGTH as u8,
                            1,
                        );
                    }
                    if (*sFallingTower.at(i)).disintegrateIdx > 47 {
                        Free((*sFallingTower.at(i)).disintegrateRand as *mut c_void);
                        (*sFallingTower.at(i)).disintegrateRand = null_mut();
                        task_set(taskId, 2, task_get(taskId, 2) + 1);
                        if i as i32 % 2 == 1 {
                            (*sBgShakeOffsets).bgVOFS -= 1;
                        }
                    }
                }
                LoadBgTiles(0, sMirageTowerGfxBuffer as *mut c_void, 2336, 0);
                if (*sFallingTower.at(95)).disintegrateIdx > 47 {
                    break 'l1;
                }
                return;
            }
            4 => {
                UnsetBgTilemapBuffer(0);
                bgShakeTaskId = FindTaskIdByFunc(Some(UpdateBgShake));
                if bgShakeTaskId != TASK_NONE {
                    DestroyTask(bgShakeTaskId);
                }
                (*sBgShakeOffsets).bgVOFS = {
                    (*sBgShakeOffsets).bgHOFS = 0;
                    (*sBgShakeOffsets).bgHOFS
                };
                SetBgShakeOffsets();
            }
            5 => {
                Free(sBgShakeOffsets as *mut c_void);
                sBgShakeOffsets = null_mut();
                Free(sFallingTower as *mut c_void);
                sFallingTower = null_mut();
                Free(sMirageTowerGfxBuffer as *mut c_void);
                sMirageTowerGfxBuffer = null_mut();
                Free(sMirageTowerTilemapBuffer as *mut c_void);
                sMirageTowerTilemapBuffer = null_mut();
            }
            6 => {
                SetGpuRegBits(REG_OFFSET_BG2CNT, 2);
                SetGpuRegBits(REG_OFFSET_BG0CNT, 0);
                SetBgAttribute(0, BG_ATTR_PRIORITY, 0);
                InitStandardTextBoxWindows();
            }
            7 => {
                ShowBg(0);
            }
            8 => {
                DestroyTask(taskId);
                ScriptContext_Enable();
            }
            _ => {}
        }
    }
    task_set(taskId, tState, task_get(taskId, tState) + 1);
}
pub(crate) unsafe fn Task_FossilFallAndSink(taskId: u8) {
    let mut i: u16 = 0;
    let mut buffer: *mut u8 = null_mut();
    'l1: {
        let sw1: i16 = task_get(taskId, 0);
        let mut fall = false;
        if sw1 == 1 {
            sFallingFossil = AllocZeroed(20) as *mut FallAnim_Fossil;
            (*sFallingFossil).frameImageTiles = AllocZeroed(128) as *mut u8;
            (*sFallingFossil).frameImage = AllocZeroed(8) as *mut SpriteFrameImage;
            (*sFallingFossil).disintegrateRand = AllocZeroed(512) as *mut u16;
            (*sFallingFossil).disintegrateIdx = 0;
            break 'l1;
        }
        if sw1 == 2 {
            buffer = (*sFallingFossil).frameImageTiles;
            i = 0;
            while i < 128 {
                *buffer = sFossil_Gfx[i];
                i += 1;
                buffer = buffer.at(1);
            }
            break 'l1;
        }
        if sw1 == 3 {
            (*(*sFallingFossil).frameImage).data = (*sFallingFossil).frameImageTiles as *mut c_void;
            (*(*sFallingFossil).frameImage).size = 128;
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            {
                let mut fossilTemplate: SpriteTemplate = *sSpriteTemplate_FallingFossil;
                fossilTemplate.images = (*sFallingFossil).frameImage;
                (*sFallingFossil).spriteId = CreateSprite(&raw mut fossilTemplate, 128, -16, 1);
                gSprites[(*sFallingFossil).spriteId].centerToCornerVecX = 0;
                gSprites[(*sFallingFossil).spriteId].data[0] =
                    gSprites[(*sFallingFossil).spriteId].x;
                gSprites[(*sFallingFossil).spriteId].data[1] = 1;
            }
        }
        if fall || sw1 == 5 {
            for i in 0..FOSSIL_DISINTEGRATE_LENGTH {
                *(*sFallingFossil).disintegrateRand.at(i) = i;
            }
            break 'l1;
        }
        if sw1 == 6 {
            for i in 0..512u16 {
                let rand1: u16 = (Random() as i32 % 256) as u16;
                let rand2: u16 = (Random() as i32 % 256) as u16;
                let temp: u16 = *(*sFallingFossil).disintegrateRand.at(rand2);
                *(*sFallingFossil).disintegrateRand.at(rand2) =
                    *(*sFallingFossil).disintegrateRand.at(rand1);
                *(*sFallingFossil).disintegrateRand.at(rand1) = temp;
            }
            gSprites[(*sFallingFossil).spriteId].callback = Some(SpriteCB_FallingFossil);
            break 'l1;
        }
        if sw1 == 7 {
            if gSprites[(*sFallingFossil).spriteId].callback
                != Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                return;
            }
            DestroySprite(&raw mut gSprites[(*sFallingFossil).spriteId]);
            Free((*sFallingFossil).disintegrateRand as *mut c_void);
            (*sFallingFossil).disintegrateRand = null_mut();
            Free((*sFallingFossil).frameImage as *mut c_void);
            (*sFallingFossil).frameImage = null_mut();
            Free((*sFallingFossil).frameImageTiles as *mut c_void);
            (*sFallingFossil).frameImageTiles = null_mut();
            Free(sFallingFossil as *mut c_void);
            sFallingFossil = null_mut();
            break 'l1;
        }
        if sw1 == 8 {
            ScriptContext_Enable();
            break 'l1;
        }
    }
    task_set(taskId, 0, task_get(taskId, 0) + 1);
}
pub(crate) unsafe fn SpriteCB_FallingFossil(sprite: *mut Sprite) {
    if (*sFallingFossil).disintegrateIdx >= FOSSIL_DISINTEGRATE_LENGTH {
        (*sprite).callback = Some(SpriteCallbackDummy);
    } else if (*sprite).y >= 96 {
        for i in 0..2u8 {
            UpdateDisintegrationEffect(
                (*sFallingFossil).frameImageTiles,
                *(*sFallingFossil).disintegrateRand.at({
                    let t1 = (*sFallingFossil).disintegrateIdx;
                    (*sFallingFossil).disintegrateIdx += 1;
                    t1
                }),
                0,
                16,
                0,
            );
        }
        StartSpriteAnim(sprite, 0);
    } else {
        (*sprite).y += 1;
    }
}
unsafe fn UpdateDisintegrationEffect(tiles: *mut u8, randId: u16, c: u8, size: u8, offset: u8) {
    let height: u8 = div_i32(randId as i32, size as i32) as u8;
    sDebug_DisintegrationData[0] = height as u16;
    let width: u8 = rem_i32(randId as i32, size as i32) as u8;
    sDebug_DisintegrationData[1] = width as u16;
    let row: u8 = height & 7;
    let col: u8 = width & 7;
    sDebug_DisintegrationData[2] = height as u16 & 7;
    sDebug_DisintegrationData[3] = width as u16 & 7;
    let widthTiles: u8 = (width as i32 / 8) as u8;
    let heightTiles: u8 = (height as i32 / 8) as u8;
    sDebug_DisintegrationData[4] = (width as i32 / 8) as u16;
    sDebug_DisintegrationData[5] = (height as i32 / 8) as u16;
    let var: u16 = (size as i32 / 8) as u16 * (heightTiles as u16 * 64) + widthTiles as u16 * 64;
    sDebug_DisintegrationData[6] = var;
    let mut baseOffset: u16 = var + (row as u16 * 8 + col as u16);
    baseOffset = (baseOffset as i32 / 2) as u16;
    sDebug_DisintegrationData[7] = var + (row as u16 * 8 + col as u16);
    let flag: u8 = (randId as i32 % 2) as u8 ^ 1;
    let tileMask: u8 =
        shl_i32(c as i32, (flag as u32) << 2) as u8 | shl_i32(15, (flag as u32 ^ 1) << 2) as u8;
    *tiles.at(baseOffset as i32 + offset as i32 * 32) &= tileMask;
}
