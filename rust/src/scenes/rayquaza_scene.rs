//! Translated from `src/rayquaza_scene.c` by tools/rustport/c2rs.py.
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
    clippy::useless_transmute,
    dead_code,
    unused_assignments
)]

use crate::agb_main::{SetHBlankCallback, SetVBlankCallback};
use crate::bg::{ChangeBgX, ChangeBgY, GetBgY, ResetBgsAndClearDma3BusyFlags, SetBgAffine, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::{ClearGpuRegBits, EnableInterrupts, GetGpuReg, SetGpuReg, SetGpuRegBits};
use crate::menu::{
    ClearScheduledBgCopiesToVram, DecompressAndCopyTileDataToVram,
    DoScheduledBgTilemapCopiesToVram, FreeTempTileDataBuffersIfPossible, ResetTempTileDataBuffers,
    ScheduleBgCopyTilemapToVram,
};
use crate::menu_helpers::{
    ResetAllBgsCoordinates, ResetVramOamAndBgCntRegs, SetVBlankHBlankCallbacksToNull,
};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, BlendPalettesGradually, FillPalette,
    LoadCompressedPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::random::Random;
use crate::scanline_effect::{
    ScanlineEffect_Clear, ScanlineEffect_InitHBlankDmaTransfer, ScanlineEffect_Stop,
};
use crate::sound::{PlayCry_Normal, PlayNewMapMusic, PlaySE, StopMapMusic};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_set, task_set_func};
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
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
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
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn InitSpriteAffineAnim(a0: *mut Sprite) {
    unsafe {
        crate::sprite::InitSpriteAffineAnim(a0 as _);
    }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
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
const sBodyPartSpriteId1: usize = 0;
const sGroudonBodySpriteId: usize = 0;
const sSmokeId: usize = 0;
const sTailSpriteId: usize = 0;
const sBodyPartSpriteId2: usize = 1;
const sDecel: usize = 5;
const sFloatTimer: usize = 6;
const sSpeed: usize = 6;
const sIsKyogre: usize = 7;
// Data tables (translate with cdata.py): sTasksForAnimations sOam_64x64 sOam_32x32 sOam_64x32 sOam_32x16 sOam_16x8 sOam_16x32 sOam_16x16 sOam_32x8 sAnim_DuoFightPre_Groudon_Head sAnim_DuoFightPre_Groudon_Body sAnims_DuoFightPre_Groudon sSpriteTemplate_DuoFightPre_Groudon sAnim_DuoFightPre_GroudonShoulderKyogreDorsalFin sAnims_DuoFightPre_GroudonShoulderKyogreDorsalFin sSpriteTemplate_DuoFightPre_GroudonShoulder sAnim_DuoFightPre_GroudonClaw sAnims_DuoFightPre_GroudonClaw sSpriteTemplate_DuoFightPre_GroudonClaw sAnim_DuoFightPre_Kyogre_TopLeft sAnim_DuoFightPre_Kyogre_TopRight sAnim_DuoFightPre_Kyogre_FaceLeft sAnim_DuoFightPre_Kyogre_FaceRight sAnim_DuoFightPre_Kyogre_ChinLeft sAnim_DuoFightPre_Kyogre_ChinRight sAnim_DuoFightPre_Kyogre_LeftPectoralFin sAnim_DuoFightPre_Kyogre_LeftShoulder sAnim_DuoFightPre_Kyogre_RightShoulder sAnims_DuoFightPre_Kyogre sSpriteTemplate_DuoFightPre_Kyogre sAnim_DuoFightPre_KyogrePectoralFin sAnims_DuoFightPre_KyogrePectoralFin sSpriteTemplate_DuoFightPre_KyogrePectoralFin sSpriteTemplate_DuoFightPre_KyogreDorsalFin sScanlineParams_DuoFight_Clouds sBgTemplates_DuoFight sAnim_DuoFight_Groudon_Head sAnim_DuoFight_Groudon_Body sAnims_DuoFight_Groudon sSpriteSheet_DuoFight_Groudon sSpritePal_DuoFight_Groudon sSpriteTemplate_DuoFight_Groudon sAnim_DuoFight_GroudonShoulderKyogreDorsalFin sAnims_DuoFight_GroudonShoulderKyogreDorsalFin sSpriteSheet_DuoFight_GroudonShoulder sSpriteTemplate_DuoFight_GroudonShoulder sAnim_DuoFight_GroudonClaw sAnims_DuoFight_GroudonClaw sSpriteSheet_DuoFight_GroudonClaw sSpriteTemplate_DuoFight_GroudonClaw sAnim_DuoFight_Kyogre_TopLeft sAnim_DuoFight_Kyogre_TopRight sAnim_DuoFight_Kyogre_FaceLeft sAnim_DuoFight_Kyogre_FaceRight sAnim_DuoFight_Kyogre_ChinLeft sAnim_DuoFight_Kyogre_ChinRight sAnim_DuoFight_Kyogre_LeftPectoralFin sAnim_DuoFight_Kyogre_LeftShoulder sAnim_DuoFight_Kyogre_RightShoulder sAnims_DuoFight_Kyogre sSpriteSheet_DuoFight_Kyogre sSpritePal_DuoFight_Kyogre sSpriteTemplate_DuoFight_Kyogre sAnim_DuoFight_KyogrePectoralFin sAnims_DuoFight_KyogrePectoralFin sSpriteSheet_DuoFight_KyogrePectoralFin sSpriteTemplate_DuoFight_KyogrePectoralFin sSpriteSheet_DuoFight_KyogreDorsalFin sSpriteTemplate_DuoFight_KyogreDorsalFin sBgTemplates_TakesFlight sAnim_TakesFlight_Smoke sAnims_TakesFlight_Smoke sAffineAnim_TakesFlight_Smoke sAffineAnims_TakesFlight_Smoke sSpriteSheet_TakesFlight_Smoke sSpritePal_TakesFlight_Smoke sSpriteTemplate_TakesFlight_Smoke sTakesFlight_SmokeCoords sBgTemplates_Descends sAnim_Descends_Rayquaza sAnims_Descends_Rayquaza sAnim_Descends_RayquazaTail sAnims_Descends_RayquazaTail sSpriteSheet_Descends_Rayquaza sSpriteSheet_Descends_RayquazaTail sSpritePal_Descends_Rayquaza sSpriteTemplate_Descends_Rayquaza sSpriteTemplate_Descends_RayquazaTail sBgTemplates_Charges sAnim_ChasesAway_Groudon_Still sAnim_ChasesAway_Groudon_Moving sAnims_ChasesAway_Groudon sAnim_ChasesAway_GroudonTail sAnims_ChasesAway_GroudonTail sAnim_ChasesAway_Kyogre_Front sAnim_ChasesAway_Kyogre_Back sAnim_ChasesAway_Kyogre_Tail sAnims_ChasesAway_Kyogre sAnim_ChasesAway_Rayquaza_FlyingDown sAnim_ChasesAway_Rayquaza_Arriving sAnim_ChasesAway_Rayquaza_Floating sAnim_ChasesAway_Rayquaza_Shouting sAnims_ChasesAway_Rayquaza sAnim_ChasesAway_RayquazaTail_FlyingDown sAnim_ChasesAway_RayquazaTail_Arriving sAnim_ChasesAway_RayquazaTail_Floating sAnim_ChasesAway_RayquazaTail_Shouting sAnims_ChasesAway_RayquazaTail sAnim_ChasesAway_KyogreSplash sAnims_ChasesAway_KyogreSplash sSpriteSheet_ChasesAway_Groudon sSpriteSheet_ChasesAway_GroudonTail sSpriteSheet_ChasesAway_Kyogre sSpriteSheet_ChasesAway_Rayquaza sSpriteSheet_ChasesAway_RayquazaTail sSpriteSheet_ChasesAway_KyogreSplash sSpritePal_ChasesAway_Groudon sSpritePal_ChasesAway_Kyogre sSpritePal_ChasesAway_Rayquaza sSpritePal_ChasesAway_KyogreSplash sSpriteTemplate_ChasesAway_Groudon sSpriteTemplate_ChasesAway_GroudonTail sSpriteTemplate_ChasesAway_Kyogre sSpriteTemplate_ChasesAway_Rayquaza sSpriteTemplate_ChasesAway_RayquazaTail sSpriteTemplate_ChasesAway_KyogreSplash sBgTemplates_ChasesAway

/// `struct RayquazaScene`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RayquazaScene {
    pub exitCallback: Option<unsafe fn()>,
    pub tilemapBuffers: CArray<CArray<u8, 2048>, 4>,
    pub unk: u16,
    pub animId: u8,
    pub endEarly: u8,
    pub revealedLightLine: i16,
    pub revealedLightTimer: i16,
    pub unused: CArray<u8, 12>,
}

unsafe impl Sync for RayquazaScene {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<RayquazaScene>() == 8216);
    assert!(offset_of!(RayquazaScene, exitCallback) == 0);
    assert!(offset_of!(RayquazaScene, tilemapBuffers) == 4);
    assert!(offset_of!(RayquazaScene, unk) == 8196);
    assert!(offset_of!(RayquazaScene, animId) == 8198);
    assert!(offset_of!(RayquazaScene, endEarly) == 8199);
    assert!(offset_of!(RayquazaScene, revealedLightLine) == 8200);
    assert!(offset_of!(RayquazaScene, revealedLightTimer) == 8202);
    assert!(offset_of!(RayquazaScene, unused) == 8204);
};

const MAX_SMOKE: i32 = 10;
const RAY_ANIM_DUO_FIGHT_PRE: u8 = 0;

static sBgTemplates_Charges: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::rayquaza_scene::sBgTemplates_Charges).cast());
static sBgTemplates_ChasesAway: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::rayquaza_scene::sBgTemplates_ChasesAway).cast());
static sBgTemplates_Descends: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::rayquaza_scene::sBgTemplates_Descends).cast());
static sBgTemplates_DuoFight: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::rayquaza_scene::sBgTemplates_DuoFight).cast());
static sBgTemplates_TakesFlight: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::rayquaza_scene::sBgTemplates_TakesFlight).cast());
static sScanlineParams_DuoFight_Clouds: Table<ScanlineEffectParams> =
    Table((&raw const crate::data::rayquaza_scene::sScanlineParams_DuoFight_Clouds).cast());
static sSpritePal_ChasesAway_Groudon: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::rayquaza_scene::sSpritePal_ChasesAway_Groudon).cast());
static sSpritePal_ChasesAway_Kyogre: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::rayquaza_scene::sSpritePal_ChasesAway_Kyogre).cast());
static sSpritePal_ChasesAway_KyogreSplash: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::rayquaza_scene::sSpritePal_ChasesAway_KyogreSplash).cast());
static sSpritePal_ChasesAway_Rayquaza: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::rayquaza_scene::sSpritePal_ChasesAway_Rayquaza).cast());
static sSpritePal_Descends_Rayquaza: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::rayquaza_scene::sSpritePal_Descends_Rayquaza).cast());
static sSpritePal_DuoFight_Groudon: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::rayquaza_scene::sSpritePal_DuoFight_Groudon).cast());
static sSpritePal_DuoFight_Kyogre: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::rayquaza_scene::sSpritePal_DuoFight_Kyogre).cast());
static sSpritePal_TakesFlight_Smoke: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::rayquaza_scene::sSpritePal_TakesFlight_Smoke).cast());
static sSpriteSheet_ChasesAway_Groudon: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_ChasesAway_Groudon).cast());
static sSpriteSheet_ChasesAway_GroudonTail: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_ChasesAway_GroudonTail).cast());
static sSpriteSheet_ChasesAway_Kyogre: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_ChasesAway_Kyogre).cast());
static sSpriteSheet_ChasesAway_KyogreSplash: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_ChasesAway_KyogreSplash).cast());
static sSpriteSheet_ChasesAway_Rayquaza: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_ChasesAway_Rayquaza).cast());
static sSpriteSheet_ChasesAway_RayquazaTail: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_ChasesAway_RayquazaTail).cast());
static sSpriteSheet_Descends_Rayquaza: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_Descends_Rayquaza).cast());
static sSpriteSheet_Descends_RayquazaTail: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_Descends_RayquazaTail).cast());
static sSpriteSheet_DuoFight_Groudon: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_DuoFight_Groudon).cast());
static sSpriteSheet_DuoFight_GroudonClaw: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_DuoFight_GroudonClaw).cast());
static sSpriteSheet_DuoFight_GroudonShoulder: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_DuoFight_GroudonShoulder).cast());
static sSpriteSheet_DuoFight_Kyogre: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_DuoFight_Kyogre).cast());
static sSpriteSheet_DuoFight_KyogreDorsalFin: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_DuoFight_KyogreDorsalFin).cast());
static sSpriteSheet_DuoFight_KyogrePectoralFin: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_DuoFight_KyogrePectoralFin).cast());
static sSpriteSheet_TakesFlight_Smoke: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteSheet_TakesFlight_Smoke).cast());
static sSpriteTemplate_ChasesAway_Groudon: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_ChasesAway_Groudon).cast());
static sSpriteTemplate_ChasesAway_GroudonTail: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_ChasesAway_GroudonTail).cast());
static sSpriteTemplate_ChasesAway_Kyogre: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_ChasesAway_Kyogre).cast());
static sSpriteTemplate_ChasesAway_KyogreSplash: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_ChasesAway_KyogreSplash).cast());
static sSpriteTemplate_ChasesAway_Rayquaza: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_ChasesAway_Rayquaza).cast());
static sSpriteTemplate_ChasesAway_RayquazaTail: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_ChasesAway_RayquazaTail).cast());
static sSpriteTemplate_Descends_Rayquaza: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_Descends_Rayquaza).cast());
static sSpriteTemplate_Descends_RayquazaTail: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_Descends_RayquazaTail).cast());
static sSpriteTemplate_DuoFightPre_Groudon: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFightPre_Groudon).cast());
static sSpriteTemplate_DuoFightPre_GroudonClaw: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFightPre_GroudonClaw).cast());
static sSpriteTemplate_DuoFightPre_GroudonShoulder: Table<SpriteTemplate> = Table(
    (&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFightPre_GroudonShoulder).cast(),
);
static sSpriteTemplate_DuoFightPre_Kyogre: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFightPre_Kyogre).cast());
static sSpriteTemplate_DuoFightPre_KyogreDorsalFin: Table<SpriteTemplate> = Table(
    (&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFightPre_KyogreDorsalFin).cast(),
);
static sSpriteTemplate_DuoFightPre_KyogrePectoralFin: Table<SpriteTemplate> = Table(
    (&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFightPre_KyogrePectoralFin).cast(),
);
static sSpriteTemplate_DuoFight_Groudon: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFight_Groudon).cast());
static sSpriteTemplate_DuoFight_GroudonClaw: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFight_GroudonClaw).cast());
static sSpriteTemplate_DuoFight_GroudonShoulder: Table<SpriteTemplate> = Table(
    (&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFight_GroudonShoulder).cast(),
);
static sSpriteTemplate_DuoFight_Kyogre: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFight_Kyogre).cast());
static sSpriteTemplate_DuoFight_KyogreDorsalFin: Table<SpriteTemplate> = Table(
    (&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFight_KyogreDorsalFin).cast(),
);
static sSpriteTemplate_DuoFight_KyogrePectoralFin: Table<SpriteTemplate> = Table(
    (&raw const crate::data::rayquaza_scene::sSpriteTemplate_DuoFight_KyogrePectoralFin).cast(),
);
static sSpriteTemplate_TakesFlight_Smoke: Table<SpriteTemplate> =
    Table((&raw const crate::data::rayquaza_scene::sSpriteTemplate_TakesFlight_Smoke).cast());
static sTakesFlight_SmokeCoords: Table<CArray<CArray<i8, 2>, 10>> =
    Table((&raw const crate::data::rayquaza_scene::sTakesFlight_SmokeCoords).cast());
static sTasksForAnimations: Table<CArray<Option<unsafe fn(u8)>, 7>> =
    Table((&raw const crate::data::rayquaza_scene::sTasksForAnimations).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sRayScene: crate::global::Global<*mut RayquazaScene> =
    crate::global::Global::new(null_mut());

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuFastSet` with this module's view of its types.
#[inline]
unsafe fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuFastSet(a0 as _, a1 as _, a2);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn DoRayquazaScene(animId: u8, endEarly: u8, exitCallback: Option<unsafe fn()>) {
    crate::gl!(sRayScene = AllocZeroed(8216) as *mut RayquazaScene);
    (*crate::gl!(sRayScene)).animId = animId;
    (*crate::gl!(sRayScene)).exitCallback = exitCallback;
    (*crate::gl!(sRayScene)).endEarly = endEarly;
    SetMainCallback2(Some(CB2_InitRayquazaScene));
}
pub(crate) unsafe fn CB2_InitRayquazaScene() {
    SetVBlankHBlankCallbacksToNull();
    ClearScheduledBgCopiesToVram();
    ScanlineEffect_Stop();
    FreeAllSpritePalettes();
    ResetPaletteFade();
    ResetSpriteData();
    ResetTasks();
    FillPalette(0, 240, 32);
    CreateTask(sTasksForAnimations[(*crate::gl!(sRayScene)).animId], 0);
    SetMainCallback2(Some(CB2_RayquazaScene));
}
pub(crate) unsafe fn CB2_RayquazaScene() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_RayquazaScene() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn Task_EndAfterFadeScreen(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        ResetSpriteData();
        FreeAllSpritePalettes();
        SetMainCallback2((*crate::gl!(sRayScene)).exitCallback);
        Free(crate::gl!(sRayScene) as *mut c_void);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_SetNextAnim(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if (*crate::gl!(sRayScene)).endEarly == TRUE {
            task_set_func(taskId, Some(Task_EndAfterFadeScreen));
        } else {
            (*crate::gl!(sRayScene)).animId += 1;
            (*crate::gl!(sRayScene)).unk = 0;
            task_set_func(taskId, sTasksForAnimations[(*crate::gl!(sRayScene)).animId]);
        }
    }
}
unsafe fn SetWindowsHideVertBorders() {
    SetGpuReg(REG_OFFSET_WININ, WININ_WIN0_ALL);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
    SetGpuReg(REG_OFFSET_WIN0V, 6280);
    crate::gl!(gPlttBufferUnfaded[0] = 0);
    crate::gl!(gPlttBufferFaded[0] = 0);
}
unsafe fn ResetWindowDimensions() {
    SetGpuReg(REG_OFFSET_WININ, WININ_WIN0_ALL);
    SetGpuReg(REG_OFFSET_WINOUT, WINOUT_WIN01_ALL);
}
pub(crate) unsafe fn Task_HandleDuoFightPre(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    DuoFight_AnimateRain();
    if (*gPaletteFade.as_ptr()).active() == 0 {
        let frame: i16 = *data;
        if frame == 64 {
            DuoFight_Lightning1();
        } else if frame == 144 {
            DuoFight_Lightning2();
        } else {
            match frame {
                328 => {
                    DuoFightEnd(taskId, 0);
                    return;
                }
                148 => {
                    DuoFight_LightningLong();
                }
                _ => {}
            }
        }
        *data += 1;
    }
}
unsafe fn DuoFightPre_CreateGroudonSprites() -> u8 {
    let mut data: *mut i16 = null_mut();
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Groudon).cast_mut(),
        88,
        72,
        3,
    );
    crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_DuoFightPre_Groudon));
    data = (*gSprites.as_ptr())[spriteId].data.as_mut_ptr();
    *data = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Groudon).cast_mut(),
        56,
        104,
        3,
    ) as i16;
    *data.at(1) = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_GroudonShoulder).cast_mut(),
        75,
        101,
        0,
    ) as i16;
    *data.at(2) = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_GroudonClaw).cast_mut(),
        109,
        114,
        1,
    ) as i16;
    StartSpriteAnim(
        {
            let gl_i0 = *data;
            crate::gl!(&raw gSprites[gl_i0])
        },
        1,
    );
    spriteId
}
pub(crate) unsafe fn SpriteCB_DuoFightPre_Groudon(sprite: *mut Sprite) {
    let data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(5) += 1;
    *data.at(5) &= 0x1F;
    if *data.at(5) == 0 && (*sprite).x != 72 {
        (*sprite).x -= 1;
        {
            let gl_i1 = (*sprite).data[sGroudonBodySpriteId];
            crate::gl!(gSprites[gl_i1].x -= 1)
        };
        {
            let gl_i2 = *data.at(1);
            crate::gl!(gSprites[gl_i2].x -= 1)
        };
        {
            let gl_i3 = *data.at(2);
            crate::gl!(gSprites[gl_i3].x -= 1)
        };
    }
    match (*sprite).animCmdIndex {
        0 => {
            {
                let gl_i4 = *data.at(1);
                crate::gl!(gSprites[gl_i4].x2 = 0)
            };
            {
                let gl_i5 = *data.at(1);
                crate::gl!(gSprites[gl_i5].y2 = 0)
            };
            {
                let gl_i6 = *data.at(2);
                crate::gl!(gSprites[gl_i6].x2 = 0)
            };
            {
                let gl_i7 = *data.at(2);
                crate::gl!(gSprites[gl_i7].y2 = 0)
            };
        }
        1 | 3 => {
            {
                let gl_i8 = *data.at(1);
                crate::gl!(gSprites[gl_i8].x2 = -1)
            };
            {
                let gl_i9 = *data.at(1);
                crate::gl!(gSprites[gl_i9].y2 = 0)
            };
            {
                let gl_i10 = *data.at(2);
                crate::gl!(gSprites[gl_i10].x2 = -1)
            };
            {
                let gl_i11 = *data.at(2);
                crate::gl!(gSprites[gl_i11].y2 = 0)
            };
        }
        2 => {
            {
                let gl_i12 = *data.at(1);
                crate::gl!(gSprites[gl_i12].x2 = -1)
            };
            {
                let gl_i13 = *data.at(1);
                crate::gl!(gSprites[gl_i13].y2 = 1)
            };
            {
                let gl_i14 = *data.at(2);
                crate::gl!(gSprites[gl_i14].x2 = -2)
            };
            {
                let gl_i15 = *data.at(2);
                crate::gl!(gSprites[gl_i15].y2 = 1)
            };
        }
        _ => {}
    }
}
unsafe fn DuoFightPre_CreateKyogreSprites() -> u8 {
    let mut data: *mut i16 = null_mut();
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        136,
        96,
        1,
    );
    crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_DuoFightPre_Kyogre));
    data = (*gSprites.as_ptr())[spriteId].data.as_mut_ptr();
    *data = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        168,
        96,
        1,
    ) as i16)
        << 8;
    *data |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        136,
        112,
        1,
    ) as i16;
    *data.at(1) = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        168,
        112,
        1,
    ) as i16)
        << 8;
    *data.at(1) |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        136,
        128,
        1,
    ) as i16;
    *data.at(2) = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        168,
        128,
        1,
    ) as i16)
        << 8;
    *data.at(2) |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        104,
        128,
        2,
    ) as i16;
    *data.at(3) = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        136,
        128,
        2,
    ) as i16)
        << 8;
    *data.at(3) |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        184,
        128,
        0,
    ) as i16;
    *data.at(4) = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_KyogrePectoralFin).cast_mut(),
        208,
        132,
        0,
    ) as i16)
        << 8;
    *data.at(4) |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_KyogreDorsalFin).cast_mut(),
        200,
        120,
        1,
    ) as i16;
    StartSpriteAnim(
        {
            let gl_i16 = *data >> 8;
            crate::gl!(&raw gSprites[gl_i16])
        },
        1,
    );
    StartSpriteAnim(
        {
            let gl_i17 = *data as i32 & 0xFF;
            crate::gl!(&raw gSprites[gl_i17])
        },
        2,
    );
    StartSpriteAnim(
        {
            let gl_i18 = *data.at(1) >> 8;
            crate::gl!(&raw gSprites[gl_i18])
        },
        3,
    );
    StartSpriteAnim(
        {
            let gl_i19 = *data.at(1) as i32 & 0xFF;
            crate::gl!(&raw gSprites[gl_i19])
        },
        4,
    );
    StartSpriteAnim(
        {
            let gl_i20 = *data.at(2) >> 8;
            crate::gl!(&raw gSprites[gl_i20])
        },
        5,
    );
    StartSpriteAnim(
        {
            let gl_i21 = *data.at(2) as i32 & 0xFF;
            crate::gl!(&raw gSprites[gl_i21])
        },
        6,
    );
    StartSpriteAnim(
        {
            let gl_i22 = *data.at(3) >> 8;
            crate::gl!(&raw gSprites[gl_i22])
        },
        7,
    );
    StartSpriteAnim(
        {
            let gl_i23 = *data.at(3) as i32 & 0xFF;
            crate::gl!(&raw gSprites[gl_i23])
        },
        8,
    );
    spriteId
}
pub(crate) unsafe fn SpriteCB_DuoFightPre_Kyogre(sprite: *mut Sprite) {
    let data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(5) += 1;
    *data.at(5) &= 0x1F;
    if *data.at(5) == 0 && (*sprite).x != 152 {
        (*sprite).x += 1;
        {
            let gl_i24 = (*sprite).data[0] >> 8;
            crate::gl!(gSprites[gl_i24].x += 1)
        };
        {
            let gl_i25 = (*sprite).data[0] as i32 & 0xFF;
            crate::gl!(gSprites[gl_i25].x += 1)
        };
        {
            let gl_i26 = *data.at(1) >> 8;
            crate::gl!(gSprites[gl_i26].x += 1)
        };
        {
            let gl_i27 = *data.at(1) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i27].x += 1)
        };
        {
            let gl_i28 = *data.at(2) >> 8;
            crate::gl!(gSprites[gl_i28].x += 1)
        };
        {
            let gl_i29 = *data.at(2) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i29].x += 1)
        };
        {
            let gl_i30 = *data.at(3) >> 8;
            crate::gl!(gSprites[gl_i30].x += 1)
        };
        {
            let gl_i31 = *data.at(3) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i31].x += 1)
        };
        {
            let gl_i32 = *data.at(4) >> 8;
            crate::gl!(gSprites[gl_i32].x += 1)
        };
        {
            let gl_i33 = *data.at(4) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i33].x += 1)
        };
    }
    match {
        let gl_i34 = *data.at(2) as i32 & 0xFF;
        crate::gl!(gSprites[gl_i34].animCmdIndex)
    } {
        0 => {
            (*sprite).y2 = 0;
            {
                let gl_i35 = *data >> 8;
                crate::gl!(gSprites[gl_i35].y2 = 0)
            };
            {
                let gl_i36 = *data as i32 & 0xFF;
                crate::gl!(gSprites[gl_i36].y2 = 0)
            };
            {
                let gl_i37 = *data.at(1) >> 8;
                crate::gl!(gSprites[gl_i37].y2 = 0)
            };
            {
                let gl_i38 = *data.at(1) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i38].y2 = 0)
            };
            {
                let gl_i39 = *data.at(2) >> 8;
                crate::gl!(gSprites[gl_i39].y2 = 0)
            };
            {
                let gl_i40 = *data.at(2) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i40].y2 = 0)
            };
            {
                let gl_i41 = *data.at(3) >> 8;
                crate::gl!(gSprites[gl_i41].y2 = 0)
            };
            {
                let gl_i42 = *data.at(3) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i42].y2 = 0)
            };
            {
                let gl_i43 = *data.at(4) >> 8;
                crate::gl!(gSprites[gl_i43].y2 = 0)
            };
            {
                let gl_i44 = *data.at(4) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i44].y2 = 0)
            };
        }
        1 | 3 => {
            (*sprite).y2 = 1;
            {
                let gl_i45 = *data >> 8;
                crate::gl!(gSprites[gl_i45].y2 = 1)
            };
            {
                let gl_i46 = *data as i32 & 0xFF;
                crate::gl!(gSprites[gl_i46].y2 = 1)
            };
            {
                let gl_i47 = *data.at(1) >> 8;
                crate::gl!(gSprites[gl_i47].y2 = 1)
            };
            {
                let gl_i48 = *data.at(1) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i48].y2 = 1)
            };
            {
                let gl_i49 = *data.at(2) >> 8;
                crate::gl!(gSprites[gl_i49].y2 = 1)
            };
            {
                let gl_i50 = *data.at(2) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i50].y2 = 1)
            };
            {
                let gl_i51 = *data.at(3) >> 8;
                crate::gl!(gSprites[gl_i51].y2 = 1)
            };
            {
                let gl_i52 = *data.at(3) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i52].y2 = 1)
            };
            {
                let gl_i53 = *data.at(4) >> 8;
                crate::gl!(gSprites[gl_i53].y2 = 1)
            };
            {
                let gl_i54 = *data.at(4) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i54].y2 = 1)
            };
        }
        2 => {
            (*sprite).y2 = 2;
            {
                let gl_i55 = *data >> 8;
                crate::gl!(gSprites[gl_i55].y2 = 2)
            };
            {
                let gl_i56 = *data as i32 & 0xFF;
                crate::gl!(gSprites[gl_i56].y2 = 2)
            };
            {
                let gl_i57 = *data.at(1) >> 8;
                crate::gl!(gSprites[gl_i57].y2 = 2)
            };
            {
                let gl_i58 = *data.at(1) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i58].y2 = 2)
            };
            {
                let gl_i59 = *data.at(2) >> 8;
                crate::gl!(gSprites[gl_i59].y2 = 2)
            };
            {
                let gl_i60 = *data.at(4) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i60].y2 = 2)
            };
        }
        _ => {}
    }
}
pub(crate) unsafe fn VBlankCB_DuoFight() {
    VBlankCB_RayquazaScene();
    ScanlineEffect_InitHBlankDmaTransfer();
}
unsafe fn InitDuoFightSceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates_DuoFight.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        0,
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn LoadDuoFightSceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        0,
        (*(&raw const crate::data::graphics::gRaySceneDuoFight_Clouds_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneDuoFight_Clouds2_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneDuoFight_Clouds1_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneDuoFight_Clouds3_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gRaySceneDuoFight_Clouds_Pal)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        0,
        64,
    );
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_Groudon).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_GroudonShoulder).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_GroudonClaw).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_Kyogre).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_KyogrePectoralFin).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_KyogreDorsalFin).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_DuoFight_Groudon).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_DuoFight_Kyogre).cast_mut());
}
pub(crate) unsafe fn Task_DuoFightAnim(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ScanlineEffect_Clear();
    InitDuoFightSceneBgs();
    LoadDuoFightSceneGfx();
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())
            .as_mut_ptr() as *mut c_void,
            0x10003c0,
        );
    }
    ScanlineEffect_SetParams(*sScanlineParams_DuoFight_Clouds);
    *data = 0;
    *data.at(1) = CreateTask(Some(Task_DuoFight_AnimateClouds), 0) as i16;
    if (*crate::gl!(sRayScene)).animId == RAY_ANIM_DUO_FIGHT_PRE {
        *data.at(2) = DuoFightPre_CreateGroudonSprites() as i16;
        *data.at(3) = DuoFightPre_CreateKyogreSprites() as i16;
        task_set_func(taskId, Some(Task_HandleDuoFightPre));
    } else {
        *data.at(2) = DuoFight_CreateGroudonSprites() as i16;
        *data.at(3) = DuoFight_CreateKyogreSprites() as i16;
        task_set_func(taskId, Some(Task_HandleDuoFight));
        StopMapMusic();
    }
    BlendPalettes(PALETTES_ALL, 0x10, 0);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
    SetVBlankCallback(Some(VBlankCB_DuoFight));
    PlaySE(SE_DOWNPOUR);
}
pub(crate) unsafe fn Task_DuoFight_AnimateClouds(taskId: u8) {
    let data: *mut u16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut u16;
    for i in 24..92i16 {
        if i <= 47 {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = *data >> 8;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = *data >> 8;
        } else if i <= 63 {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = *data.at(1) >> 8;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = *data.at(1) >> 8;
        } else if i <= 75 {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = *data.at(2) >> 8;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = *data.at(2) >> 8;
        } else if i <= 83 {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = *data.at(3) >> 8;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = *data.at(3) >> 8;
        } else if i <= 87 {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = *data.at(4) >> 8;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = *data.at(4) >> 8;
        } else {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = *data.at(5) >> 8;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = *data.at(5) >> 8;
        }
    }
    if (*crate::gl!(sRayScene)).animId == RAY_ANIM_DUO_FIGHT_PRE {
        *data += 448;
        *data.at(1) += 384;
        *data.at(2) += 320;
        *data.at(3) += 256;
        *data.at(4) += 192;
        *data.at(5) += 128;
    } else {
        *data += 768;
        *data.at(1) += 640;
        *data.at(2) += 512;
        *data.at(3) += 384;
        *data.at(4) += 256;
        *data.at(5) += 128;
    }
}
pub(crate) unsafe fn Task_HandleDuoFight(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    DuoFight_AnimateRain();
    if (*gPaletteFade.as_ptr()).active() == 0 {
        let frame: i16 = *data;
        if frame == 32 || frame == 112 {
            DuoFight_Lightning1();
        } else if frame == 216 {
            DuoFight_Lightning2();
        } else if frame == 220 {
            DuoFight_LightningLong();
        } else {
            match frame {
                412 => {
                    DuoFightEnd(taskId, 2);
                    return;
                }
                380 => {
                    SetGpuReg(REG_OFFSET_BLDCNT, 580);
                    task_set_func(*data.at(1), Some(DuoFight_PanOffScene));
                    task_set(*data.at(1), 0, 0);
                    task_set(*data.at(1), 2, *data.at(2));
                    task_set(*data.at(1), 3, *data.at(3));
                    ScanlineEffect_Stop();
                }
                _ => {}
            }
        }
        *data += 1;
    }
}
unsafe fn DuoFight_Lightning1() {
    PlaySE(SE_THUNDER);
    BlendPalettesGradually(32767, 0, 16, 0, PALETTES_BG as u16, 0, 0);
    BlendPalettesGradually(PALETTES_OBJECTS, 0, 16, 0, 0, 0, 1);
}
unsafe fn DuoFight_Lightning2() {
    PlaySE(SE_THUNDER);
    BlendPalettesGradually(32767, 0, 16, 16, PALETTES_BG as u16, 0, 0);
    BlendPalettesGradually(PALETTES_OBJECTS, 0, 16, 16, 0, 0, 1);
}
unsafe fn DuoFight_LightningLong() {
    BlendPalettesGradually(32767, 4, 16, 0, PALETTES_BG as u16, 0, 0);
    BlendPalettesGradually(PALETTES_OBJECTS, 4, 16, 0, 0, 0, 1);
}
unsafe fn DuoFight_AnimateRain() {
    ChangeBgX(2, 0x400, BG_COORD_ADD);
    ChangeBgY(2, 0x800, BG_COORD_SUB);
}
pub(crate) unsafe fn DuoFight_PanOffScene(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    DuoFight_SlideGroudonDown({
        let gl_i61 = *data.at(2);
        crate::gl!(&raw gSprites[gl_i61])
    });
    DuoFight_SlideKyogreDown({
        let gl_i62 = *data.at(3);
        crate::gl!(&raw gSprites[gl_i62])
    });
    let bgY: u16 = GetBgY(1) as u16;
    if GetBgY(1) == 0 || bgY > 0x8000 {
        ChangeBgY(1, 0x400, BG_COORD_SUB);
    }
    if *data != 16 {
        *data += 1;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            ((*data as u16) << 8) | (16 - *data as u16),
        );
    }
}
unsafe fn DuoFightEnd(taskId: u8, palDelay: i8) {
    PlaySE(SE_DOWNPOUR_STOP);
    BeginNormalPaletteFade(PALETTES_ALL, palDelay, 0, 0x10, 0);
    task_set_func(taskId, Some(Task_DuoFightEnd));
}
pub(crate) unsafe fn Task_DuoFightEnd(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    DuoFight_AnimateRain();
    if (*gPaletteFade.as_ptr()).active() == 0 {
        DestroyTask(*data.at(1) as u8);
        ChangeBgY(1, 0, BG_COORD_SET);
        SetVBlankCallback(None);
        ScanlineEffect_Stop();
        ResetSpriteData();
        FreeAllSpritePalettes();
        *data = 0;
        task_set_func(taskId, Some(Task_SetNextAnim));
    }
}
unsafe fn DuoFight_CreateGroudonSprites() -> u8 {
    let mut data: *mut i16 = null_mut();
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Groudon).cast_mut(),
        98,
        72,
        3,
    );
    crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_DuoFight_Groudon));
    data = (*gSprites.as_ptr())[spriteId].data.as_mut_ptr();
    *data = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Groudon).cast_mut(),
        66,
        104,
        3,
    ) as i16;
    *data.at(1) = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_GroudonShoulder).cast_mut(),
        85,
        101,
        0,
    ) as i16;
    *data.at(2) = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_GroudonClaw).cast_mut(),
        119,
        114,
        1,
    ) as i16;
    StartSpriteAnim(
        {
            let gl_i63 = *data;
            crate::gl!(&raw gSprites[gl_i63])
        },
        1,
    );
    spriteId
}
pub(crate) unsafe fn SpriteCB_DuoFight_Groudon(sprite: *mut Sprite) {
    let data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(5) += 1;
    *data.at(5) &= 0xF;
    if *data.at(5) as i32 & 7 == 0 && (*sprite).x != 72 {
        (*sprite).x -= 1;
        {
            let gl_i64 = (*sprite).data[sGroudonBodySpriteId];
            crate::gl!(gSprites[gl_i64].x -= 1)
        };
        {
            let gl_i65 = *data.at(1);
            crate::gl!(gSprites[gl_i65].x -= 1)
        };
        {
            let gl_i66 = *data.at(2);
            crate::gl!(gSprites[gl_i66].x -= 1)
        };
    }
    match (*sprite).animCmdIndex {
        0 => {
            {
                let gl_i67 = *data.at(1);
                crate::gl!(gSprites[gl_i67].x2 = 0)
            };
            {
                let gl_i68 = *data.at(1);
                crate::gl!(gSprites[gl_i68].y2 = 0)
            };
            {
                let gl_i69 = *data.at(2);
                crate::gl!(gSprites[gl_i69].x2 = 0)
            };
            {
                let gl_i70 = *data.at(2);
                crate::gl!(gSprites[gl_i70].y2 = 0)
            };
        }
        1 | 3 => {
            {
                let gl_i71 = *data.at(1);
                crate::gl!(gSprites[gl_i71].x2 = -1)
            };
            {
                let gl_i72 = *data.at(1);
                crate::gl!(gSprites[gl_i72].y2 = 0)
            };
            {
                let gl_i73 = *data.at(2);
                crate::gl!(gSprites[gl_i73].x2 = -1)
            };
            {
                let gl_i74 = *data.at(2);
                crate::gl!(gSprites[gl_i74].y2 = 0)
            };
        }
        2 => {
            {
                let gl_i75 = *data.at(1);
                crate::gl!(gSprites[gl_i75].x2 = -1)
            };
            {
                let gl_i76 = *data.at(1);
                crate::gl!(gSprites[gl_i76].y2 = 1)
            };
            {
                let gl_i77 = *data.at(2);
                crate::gl!(gSprites[gl_i77].x2 = -2)
            };
            {
                let gl_i78 = *data.at(2);
                crate::gl!(gSprites[gl_i78].y2 = 1)
            };
        }
        _ => {}
    }
}
unsafe fn DuoFight_SlideGroudonDown(sprite: *mut Sprite) {
    let data: *mut i16 = (*sprite).data.as_mut_ptr();
    if (*sprite).y <= DISPLAY_HEIGHT as i16 {
        (*sprite).y += 8;
        {
            let gl_i79 = (*sprite).data[sGroudonBodySpriteId];
            crate::gl!(gSprites[gl_i79].y += 8)
        };
        {
            let gl_i80 = *data.at(1);
            crate::gl!(gSprites[gl_i80].y += 8)
        };
        {
            let gl_i81 = *data.at(2);
            crate::gl!(gSprites[gl_i81].y += 8)
        };
    }
}
unsafe fn DuoFight_CreateKyogreSprites() -> u8 {
    let mut data: *mut i16 = null_mut();
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        126,
        96,
        1,
    );
    crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_DuoFight_Kyogre));
    data = (*gSprites.as_ptr())[spriteId].data.as_mut_ptr();
    *data = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        158,
        96,
        1,
    ) as i16)
        << 8;
    *data |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        126,
        112,
        1,
    ) as i16;
    *data.at(1) = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        158,
        112,
        1,
    ) as i16)
        << 8;
    *data.at(1) |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        126,
        128,
        1,
    ) as i16;
    *data.at(2) = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        158,
        128,
        1,
    ) as i16)
        << 8;
    *data.at(2) |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        94,
        128,
        2,
    ) as i16;
    *data.at(3) = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        126,
        128,
        2,
    ) as i16)
        << 8;
    *data.at(3) |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        174,
        128,
        0,
    ) as i16;
    *data.at(4) = (CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_KyogrePectoralFin).cast_mut(),
        198,
        132,
        0,
    ) as i16)
        << 8;
    *data.at(4) |= CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_KyogreDorsalFin).cast_mut(),
        190,
        120,
        1,
    ) as i16;
    StartSpriteAnim(
        {
            let gl_i82 = *data >> 8;
            crate::gl!(&raw gSprites[gl_i82])
        },
        1,
    );
    StartSpriteAnim(
        {
            let gl_i83 = *data as i32 & 0xFF;
            crate::gl!(&raw gSprites[gl_i83])
        },
        2,
    );
    StartSpriteAnim(
        {
            let gl_i84 = *data.at(1) >> 8;
            crate::gl!(&raw gSprites[gl_i84])
        },
        3,
    );
    StartSpriteAnim(
        {
            let gl_i85 = *data.at(1) as i32 & 0xFF;
            crate::gl!(&raw gSprites[gl_i85])
        },
        4,
    );
    StartSpriteAnim(
        {
            let gl_i86 = *data.at(2) >> 8;
            crate::gl!(&raw gSprites[gl_i86])
        },
        5,
    );
    StartSpriteAnim(
        {
            let gl_i87 = *data.at(2) as i32 & 0xFF;
            crate::gl!(&raw gSprites[gl_i87])
        },
        6,
    );
    StartSpriteAnim(
        {
            let gl_i88 = *data.at(3) >> 8;
            crate::gl!(&raw gSprites[gl_i88])
        },
        7,
    );
    StartSpriteAnim(
        {
            let gl_i89 = *data.at(3) as i32 & 0xFF;
            crate::gl!(&raw gSprites[gl_i89])
        },
        8,
    );
    spriteId
}
pub(crate) unsafe fn SpriteCB_DuoFight_Kyogre(sprite: *mut Sprite) {
    let data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(5) += 1;
    *data.at(5) &= 0xF;
    if *data.at(5) as i32 & 7 == 0 && (*sprite).x != 152 {
        (*sprite).x += 1;
        {
            let gl_i90 = (*sprite).data[0] >> 8;
            crate::gl!(gSprites[gl_i90].x += 1)
        };
        {
            let gl_i91 = (*sprite).data[0] as i32 & 0xFF;
            crate::gl!(gSprites[gl_i91].x += 1)
        };
        {
            let gl_i92 = *data.at(1) >> 8;
            crate::gl!(gSprites[gl_i92].x += 1)
        };
        {
            let gl_i93 = *data.at(1) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i93].x += 1)
        };
        {
            let gl_i94 = *data.at(2) >> 8;
            crate::gl!(gSprites[gl_i94].x += 1)
        };
        {
            let gl_i95 = *data.at(2) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i95].x += 1)
        };
        {
            let gl_i96 = *data.at(3) >> 8;
            crate::gl!(gSprites[gl_i96].x += 1)
        };
        {
            let gl_i97 = *data.at(3) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i97].x += 1)
        };
        {
            let gl_i98 = *data.at(4) >> 8;
            crate::gl!(gSprites[gl_i98].x += 1)
        };
        {
            let gl_i99 = *data.at(4) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i99].x += 1)
        };
    }
    match {
        let gl_i100 = *data.at(2) as i32 & 0xFF;
        crate::gl!(gSprites[gl_i100].animCmdIndex)
    } {
        0 => {
            (*sprite).y2 = 0;
            {
                let gl_i101 = *data >> 8;
                crate::gl!(gSprites[gl_i101].y2 = 0)
            };
            {
                let gl_i102 = *data as i32 & 0xFF;
                crate::gl!(gSprites[gl_i102].y2 = 0)
            };
            {
                let gl_i103 = *data.at(1) >> 8;
                crate::gl!(gSprites[gl_i103].y2 = 0)
            };
            {
                let gl_i104 = *data.at(1) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i104].y2 = 0)
            };
            {
                let gl_i105 = *data.at(2) >> 8;
                crate::gl!(gSprites[gl_i105].y2 = 0)
            };
            {
                let gl_i106 = *data.at(2) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i106].y2 = 0)
            };
            {
                let gl_i107 = *data.at(3) >> 8;
                crate::gl!(gSprites[gl_i107].y2 = 0)
            };
            {
                let gl_i108 = *data.at(3) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i108].y2 = 0)
            };
            {
                let gl_i109 = *data.at(4) >> 8;
                crate::gl!(gSprites[gl_i109].y2 = 0)
            };
            {
                let gl_i110 = *data.at(4) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i110].y2 = 0)
            };
        }
        1 | 3 => {
            (*sprite).y2 = 1;
            {
                let gl_i111 = *data >> 8;
                crate::gl!(gSprites[gl_i111].y2 = 1)
            };
            {
                let gl_i112 = *data as i32 & 0xFF;
                crate::gl!(gSprites[gl_i112].y2 = 1)
            };
            {
                let gl_i113 = *data.at(1) >> 8;
                crate::gl!(gSprites[gl_i113].y2 = 1)
            };
            {
                let gl_i114 = *data.at(1) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i114].y2 = 1)
            };
            {
                let gl_i115 = *data.at(2) >> 8;
                crate::gl!(gSprites[gl_i115].y2 = 1)
            };
            {
                let gl_i116 = *data.at(2) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i116].y2 = 1)
            };
            {
                let gl_i117 = *data.at(3) >> 8;
                crate::gl!(gSprites[gl_i117].y2 = 1)
            };
            {
                let gl_i118 = *data.at(3) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i118].y2 = 1)
            };
            {
                let gl_i119 = *data.at(4) >> 8;
                crate::gl!(gSprites[gl_i119].y2 = 1)
            };
            {
                let gl_i120 = *data.at(4) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i120].y2 = 1)
            };
        }
        2 => {
            (*sprite).y2 = 2;
            {
                let gl_i121 = *data >> 8;
                crate::gl!(gSprites[gl_i121].y2 = 2)
            };
            {
                let gl_i122 = *data as i32 & 0xFF;
                crate::gl!(gSprites[gl_i122].y2 = 2)
            };
            {
                let gl_i123 = *data.at(1) >> 8;
                crate::gl!(gSprites[gl_i123].y2 = 2)
            };
            {
                let gl_i124 = *data.at(1) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i124].y2 = 2)
            };
            {
                let gl_i125 = *data.at(2) >> 8;
                crate::gl!(gSprites[gl_i125].y2 = 2)
            };
            {
                let gl_i126 = *data.at(4) as i32 & 0xFF;
                crate::gl!(gSprites[gl_i126].y2 = 2)
            };
        }
        _ => {}
    }
}
unsafe fn DuoFight_SlideKyogreDown(sprite: *mut Sprite) {
    let data: *mut i16 = (*sprite).data.as_mut_ptr();
    if (*sprite).y <= DISPLAY_HEIGHT as i16 {
        (*sprite).y += 8;
        {
            let gl_i127 = (*sprite).data[0] >> 8;
            crate::gl!(gSprites[gl_i127].y += 8)
        };
        {
            let gl_i128 = (*sprite).data[0] as i32 & 0xFF;
            crate::gl!(gSprites[gl_i128].y += 8)
        };
        {
            let gl_i129 = *data.at(1) >> 8;
            crate::gl!(gSprites[gl_i129].y += 8)
        };
        {
            let gl_i130 = *data.at(1) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i130].y += 8)
        };
        {
            let gl_i131 = *data.at(2) >> 8;
            crate::gl!(gSprites[gl_i131].y += 8)
        };
        {
            let gl_i132 = *data.at(2) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i132].y += 8)
        };
        {
            let gl_i133 = *data.at(3) >> 8;
            crate::gl!(gSprites[gl_i133].y += 8)
        };
        {
            let gl_i134 = *data.at(3) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i134].y += 8)
        };
        {
            let gl_i135 = *data.at(4) >> 8;
            crate::gl!(gSprites[gl_i135].y += 8)
        };
        {
            let gl_i136 = *data.at(4) as i32 & 0xFF;
            crate::gl!(gSprites[gl_i136].y += 8)
        };
    }
}
unsafe fn InitTakesFlightSceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(1, sBgTemplates_TakesFlight.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        0,
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn LoadTakesFlightSceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        0,
        (*(&raw const crate::data::graphics::gRaySceneDuoFight_Clouds_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        1,
        (*(&raw const crate::data::graphics::gRaySceneTakesFlight_Bg_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        2,
        (*(&raw const crate::data::graphics::gRaySceneTakesFlight_Rayquaza_Gfx)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneDuoFight_Clouds2_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneTakesFlight_Bg_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneTakesFlight_Rayquaza_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gRaySceneTakesFlight_Rayquaza_Pal)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        0,
        64,
    );
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_TakesFlight_Smoke).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_TakesFlight_Smoke).cast_mut());
}
pub(crate) unsafe fn Task_RayTakesFlightAnim(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    PlayNewMapMusic(MUS_RAYQUAZA_APPEARS);
    InitTakesFlightSceneBgs();
    LoadTakesFlightSceneGfx();
    SetGpuReg(REG_OFFSET_BLDCNT, 592);
    SetGpuReg(REG_OFFSET_BLDALPHA, 2056);
    BlendPalettes(PALETTES_ALL, 16, 0);
    SetVBlankCallback(Some(VBlankCB_RayquazaScene));
    CreateTask(Some(Task_TakesFlight_CreateSmoke), 0);
    *data = 0;
    *data.at(1) = 0;
    task_set_func(taskId, Some(Task_HandleRayTakesFlight));
}
pub(crate) unsafe fn Task_HandleRayTakesFlight(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            if *data.at(1) == 8 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
                *data.at(2) = 0;
                *data.at(3) = 30;
                *data.at(4) = 0;
                *data.at(5) = 7;
                *data.at(1) = 0;
                *data += 1;
            } else {
                *data.at(1) += 1;
            }
        }
        1 => {
            *data.at(2) += *data.at(3);
            *data.at(4) += *data.at(5);
            if *data.at(3) > 3 {
                *data.at(3) -= 3;
            }
            if *data.at(5) != 0 {
                *data.at(5) -= 1;
            }
            if *data.at(2) > 255 {
                *data.at(2) = 256;
                *data.at(3) = 0;
                *data.at(6) = 12;
                *data.at(7) = -1;
                *data.at(1) = 0;
                *data += 1;
            }
            SetBgAffine(
                2,
                0x7800,
                0x1800,
                120,
                *data.at(4) + 32,
                *data.at(2),
                *data.at(2),
                0,
            );
        }
        2 => {
            *data.at(1) += 1;
            SetBgAffine(
                2,
                0x7800,
                0x1800,
                120,
                *data.at(4) + 32 + (*data.at(6) >> 2),
                *data.at(2),
                *data.at(2),
                0,
            );
            *data.at(6) += *data.at(7);
            if *data.at(6) == 12 || *data.at(6) == -12 {
                *data.at(7) *= -1;
                if *data.at(1) > 295 {
                    *data += 1;
                    BeginNormalPaletteFade(PALETTES_ALL, 6, 0, 0x10, 0);
                }
            }
        }
        3 => {
            *data.at(2) += 16;
            SetBgAffine(
                2,
                0x7800,
                0x1800,
                120,
                *data.at(4) + 32,
                *data.at(2),
                *data.at(2),
                0,
            );
            Task_RayTakesFlightEnd(taskId);
        }
        _ => {}
    }
}
unsafe fn Task_RayTakesFlightEnd(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        SetVBlankCallback(None);
        ResetSpriteData();
        FreeAllSpritePalettes();
        task_set_func(taskId, Some(Task_SetNextAnim));
    }
}
pub(crate) unsafe fn Task_TakesFlight_CreateSmoke(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(1) as i32 & 3 == 0 {
        let spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_TakesFlight_Smoke).cast_mut(),
            sTakesFlight_SmokeCoords[*data][0] as i16 * 4 + 120,
            sTakesFlight_SmokeCoords[*data][1] as i16 * 4 + 80,
            0,
        );
        crate::gl!(gSprites[spriteId].data[0] = *data as i8 as i16);
        (*gSprites.as_ptr())[spriteId]
            .oam
            .set_objMode(ST_OAM_OBJ_BLEND);
        (*gSprites.as_ptr())[spriteId]
            .oam
            .set_affineMode(ST_OAM_AFFINE_DOUBLE);
        (*gSprites.as_ptr())[spriteId].oam.set_priority(2);
        InitSpriteAffineAnim(crate::gl!(&raw gSprites[spriteId]));
        if *data == 9 {
            DestroyTask(taskId);
            return;
        } else {
            *data += 1;
        }
    }
    *data.at(1) += 1;
}
pub(crate) unsafe fn SpriteCB_TakesFlight_Smoke(sprite: *mut Sprite) {
    if (*sprite).data[1] == 0 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
    } else {
        (*sprite).x2 += sTakesFlight_SmokeCoords[(*sprite).data[sSmokeId]][0] as i16;
        (*sprite).y2 += sTakesFlight_SmokeCoords[(*sprite).data[sSmokeId]][1] as i16;
    }
    (*sprite).data[1] += 1;
    (*sprite).data[1] &= 0xF;
}
unsafe fn InitDescendsSceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates_Descends.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(
        0,
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        3,
        (*crate::gl!(sRayScene)).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    ScheduleBgCopyTilemapToVram(3);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn LoadDescendsSceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        0,
        (*(&raw const crate::data::graphics::gRaySceneDescends_Light_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        1,
        (*(&raw const crate::data::graphics::gRaySceneDescends_Bg_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneDescends_Light_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneDescends_Bg_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
    );
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
            0x1000200,
        );
    }
    CpuFastSet(
        (*crate::gl!(sRayScene)).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
        512,
    );
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            &raw mut (*crate::gl!(sRayScene)).tilemapBuffers[1][256] as *mut c_void,
            0x10000d0,
        );
    }
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gRaySceneDescends_Bg_Pal).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        64,
    );
    crate::gl!(gPlttBufferUnfaded[0] = 32767);
    crate::gl!(gPlttBufferFaded[0] = 32767);
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Descends_Rayquaza).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Descends_RayquazaTail).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_Descends_Rayquaza).cast_mut());
}
pub(crate) unsafe fn HBlankCB_RayDescends() {
    let vcount: u16 = GetGpuReg(REG_OFFSET_VCOUNT);
    if (24..=135).contains(&vcount)
        && vcount as i32 - 24 <= (*crate::gl!(sRayScene)).revealedLightLine as i32
    {
        volatile_write(67108946_usize as *mut u16, 0xD08);
    } else {
        volatile_write(67108946_usize as *mut u16, 0x1000);
    }
    if vcount == 0 {
        if (*crate::gl!(sRayScene)).revealedLightLine <= 0x1FFF {
            if (*crate::gl!(sRayScene)).revealedLightLine <= 39 {
                (*crate::gl!(sRayScene)).revealedLightLine += 4;
            } else if (*crate::gl!(sRayScene)).revealedLightLine <= 79 {
                (*crate::gl!(sRayScene)).revealedLightLine += 2;
            } else {
                (*crate::gl!(sRayScene)).revealedLightLine += 1;
            }
        }
        (*crate::gl!(sRayScene)).revealedLightTimer += 1;
    }
}
pub(crate) unsafe fn Task_RayDescendsAnim(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    InitDescendsSceneBgs();
    LoadDescendsSceneGfx();
    SetGpuRegBits(REG_OFFSET_BLDCNT, 7745);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    BlendPalettes(PALETTES_ALL, 0x10, 0);
    SetVBlankCallback(Some(VBlankCB_RayquazaScene));
    (*crate::gl!(sRayScene)).revealedLightLine = 0;
    (*crate::gl!(sRayScene)).revealedLightTimer = 0;
    *data = 0;
    *data.at(1) = 0;
    *data.at(2) = 0;
    *data.at(3) = 0;
    *data.at(4) = 0x1000;
    task_set_func(taskId, Some(Task_HandleRayDescends));
}
pub(crate) unsafe fn Task_HandleRayDescends(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            if *data.at(1) == 8 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
                *data.at(1) = 0;
                *data += 1;
            } else {
                *data.at(1) += 1;
            }
        }
        1 => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                if *data.at(1) == 10 {
                    *data.at(1) = 0;
                    *data += 1;
                    SetHBlankCallback(Some(HBlankCB_RayDescends));
                    EnableInterrupts(3);
                } else {
                    *data.at(1) += 1;
                }
            }
        }
        2 => {
            if *data.at(1) == 80 {
                *data.at(1) = 0;
                *data += 1;
                CreateDescendsRayquazaSprite();
            } else {
                *data.at(1) += 1;
            }
        }
        3 => {
            if ({
                *data.at(1) += 1;
                *data.at(1)
            }) == 368
            {
                *data.at(1) = 0;
                *data += 1;
            }
        }
        4 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            task_set_func(taskId, Some(Task_RayDescendsEnd));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_RayDescendsEnd(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        SetVBlankCallback(None);
        SetHBlankCallback(None);
        ResetSpriteData();
        FreeAllSpritePalettes();
        task_set_func(taskId, Some(Task_SetNextAnim));
    }
}
unsafe fn CreateDescendsRayquazaSprite() -> u8 {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_Descends_Rayquaza).cast_mut(),
        160,
        0,
        0,
    );
    let data: *mut i16 = (*gSprites.as_ptr())[spriteId].data.as_mut_ptr();
    *data = CreateSprite(
        (&raw const *sSpriteTemplate_Descends_RayquazaTail).cast_mut(),
        184,
        -48,
        0,
    ) as i16;
    crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_Descends_Rayquaza));
    (*gSprites.as_ptr())[spriteId].oam.set_priority(3);
    (*gSprites.as_ptr())[*data].oam.set_priority(3);
    spriteId
}
pub(crate) unsafe fn SpriteCB_Descends_Rayquaza(sprite: *mut Sprite) {
    let data: *mut i16 = (*sprite).data.as_mut_ptr();
    let frame: i16 = *data.at(2);
    if frame == 0 {
        *data.at(3) = 12;
        *data.at(4) = 8;
    } else if frame == 256 {
        *data.at(3) = 9;
        *data.at(4) = 7;
    } else if frame == 268 {
        *data.at(3) = 8;
        *data.at(4) = 6;
    } else if frame == 280 {
        *data.at(3) = 7;
        *data.at(4) = 5;
    } else if frame == 292 {
        *data.at(3) = 6;
        *data.at(4) = 4;
    } else if frame == 304 {
        *data.at(3) = 5;
        *data.at(4) = 3;
    } else if frame == 320 {
        *data.at(3) = 4;
        *data.at(4) = 2;
    }
    if rem_i32(*data.at(2) as i32, *data.at(3) as i32) == 0 {
        (*sprite).x2 -= 1;
        {
            let gl_i137 = *data;
            crate::gl!(gSprites[gl_i137].x2 -= 1)
        };
    }
    if rem_i32(*data.at(2) as i32, *data.at(4) as i32) == 0 {
        (*sprite).y2 += 1;
        {
            let gl_i138 = *data;
            crate::gl!(gSprites[gl_i138].y2 += 1)
        };
    }
    *data.at(2) += 1;
}
unsafe fn InitChargesSceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates_Charges.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(
        0,
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        3,
        (*crate::gl!(sRayScene)).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    ScheduleBgCopyTilemapToVram(3);
    SetGpuReg(REG_OFFSET_DISPCNT, 12352);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn LoadChargesSceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        1,
        (*(&raw const crate::data::graphics::gRaySceneCharges_Rayquaza_Gfx)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        2,
        (*(&raw const crate::data::graphics::gRaySceneCharges_Streaks_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        3,
        (*(&raw const crate::data::graphics::gRaySceneCharges_Bg_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneCharges_Orbs_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneCharges_Rayquaza_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneCharges_Streaks_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneCharges_Bg_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gRaySceneCharges_Bg_Pal).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        128,
    );
}
pub(crate) unsafe fn Task_RayChargesAnim(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    InitChargesSceneBgs();
    LoadChargesSceneGfx();
    SetWindowsHideVertBorders();
    BlendPalettes(PALETTES_ALL, 0x10, 0);
    SetVBlankCallback(Some(VBlankCB_RayquazaScene));
    *data = 0;
    *data.at(1) = 0;
    *data.at(2) = CreateTask(Some(Task_RayCharges_ShakeRayquaza), 0) as i16;
    task_set_func(taskId, Some(Task_HandleRayCharges));
}
pub(crate) unsafe fn Task_HandleRayCharges(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    RayCharges_AnimateBg();
    if *data.at(3) as i32 & 7 == 0 && *data <= 1 && *data.at(1) <= 89 {
        PlaySE(SE_INTRO_BLAST);
    }
    *data.at(3) += 1;
    match *data {
        0 => {
            if *data.at(1) == 8 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
                *data.at(1) = 0;
                *data += 1;
            } else {
                *data.at(1) += 1;
            }
        }
        1 => {
            if *data.at(1) == 127 {
                *data.at(1) = 0;
                *data += 1;
                task_set_func(*data.at(2), Some(Task_RayCharges_FlyOffscreen));
            } else {
                *data.at(1) += 1;
            }
        }
        2 => {
            if *data.at(1) == 12 {
                *data.at(1) = 0;
                *data += 1;
            } else {
                *data.at(1) += 1;
            }
        }
        3 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            task_set_func(taskId, Some(Task_RayChargesEnd));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_RayCharges_ShakeRayquaza(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(15) as i32 & 3 == 0 {
        ChangeBgX(1, (Random() as i32 % 8 - 4) << 8, BG_COORD_SET);
        ChangeBgY(1, (Random() as i32 % 8 - 4) << 8, BG_COORD_SET);
    }
    *data.at(15) += 1;
}
pub(crate) unsafe fn Task_RayCharges_FlyOffscreen(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data == 0 {
        ChangeBgX(1, 0, BG_COORD_SET);
        ChangeBgY(1, 0, BG_COORD_SET);
        *data += 1;
        *data.at(1) = 10;
        *data.at(2) = -1;
    } else if *data == 1 {
        ChangeBgX(1, (*data.at(1) as i32) << 8, BG_COORD_SUB);
        ChangeBgY(1, (*data.at(1) as i32) << 8, BG_COORD_ADD);
        *data.at(1) += *data.at(2);
        if *data.at(1) == -10 {
            *data.at(2) *= -1;
        }
    }
}
unsafe fn RayCharges_AnimateBg() {
    ChangeBgX(2, 0x400, BG_COORD_SUB);
    ChangeBgY(2, 0x400, BG_COORD_ADD);
    ChangeBgX(0, 0x800, BG_COORD_SUB);
    ChangeBgY(0, 0x800, BG_COORD_ADD);
}
pub(crate) unsafe fn Task_RayChargesEnd(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    RayCharges_AnimateBg();
    if (*gPaletteFade.as_ptr()).active() == 0 {
        SetVBlankCallback(None);
        ResetWindowDimensions();
        DestroyTask(*data.at(2) as u8);
        task_set_func(taskId, Some(Task_SetNextAnim));
    }
}
unsafe fn InitChasesAwaySceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(1, sBgTemplates_ChasesAway.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        0,
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    SetGpuReg(REG_OFFSET_DISPCNT, 12352);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn LoadChasesAwaySceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        2,
        (*(&raw const crate::data::graphics::gRaySceneChasesAway_Ring_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        0,
        (*(&raw const crate::data::graphics::gRaySceneChasesAway_Light_Gfx)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneChasesAway_Bg_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneChasesAway_Light_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gRaySceneChasesAway_Ring_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sRayScene)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gRaySceneChasesAway_Bg_Pal).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        96,
    );
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_ChasesAway_Groudon).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_ChasesAway_GroudonTail).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_ChasesAway_Kyogre).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_ChasesAway_Rayquaza).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_ChasesAway_RayquazaTail).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_ChasesAway_KyogreSplash).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_ChasesAway_Groudon).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_ChasesAway_Kyogre).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_ChasesAway_Rayquaza).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_ChasesAway_KyogreSplash).cast_mut());
}
pub(crate) unsafe fn Task_RayChasesAwayAnim(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    InitChasesAwaySceneBgs();
    LoadChasesAwaySceneGfx();
    SetWindowsHideVertBorders();
    ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
    SetGpuReg(REG_OFFSET_BLDCNT, 577);
    SetGpuReg(REG_OFFSET_BLDALPHA, 3593);
    BlendPalettes(PALETTES_ALL, 0x10, 0);
    SetVBlankCallback(Some(VBlankCB_RayquazaScene));
    *data = 0;
    *data.at(1) = 0;
    task_set_func(taskId, Some(Task_HandleRayChasesAway));
    *data.at(2) = CreateTask(Some(Task_ChasesAway_AnimateBg), 0) as i16;
    task_set(*data.at(2), 0, 0);
    task_set(*data.at(2), 1, 0);
    task_set(*data.at(2), 2, 0);
    task_set(*data.at(2), 3, 1);
    task_set(*data.at(2), 4, 1);
}
pub(crate) unsafe fn Task_HandleRayChasesAway(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            if *data.at(1) == 8 {
                ChasesAway_CreateTrioSprites(taskId);
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
                *data.at(1) = 0;
                *data += 1;
            } else {
                *data.at(1) += 1;
            }
        }
        1 => {
            if ({
                let gl_i139 = *data.at(5);
                crate::gl!(gSprites[gl_i139].callback)
            }) == Some(SpriteCB_ChasesAway_RayquazaFloat as unsafe fn(*mut Sprite))
            {
                if *data.at(1) == 64 {
                    ChasesAway_KyogreStartLeave(taskId);
                    ChasesAway_GroudonStartLeave(taskId);
                    *data.at(1) = 0;
                    *data += 1;
                } else {
                    *data.at(1) += 1;
                }
            }
        }
        2 => {
            if *data.at(1) == 448 {
                *data.at(1) = 0;
                *data += 1;
            } else {
                *data.at(1) += 1;
                if *data.at(1) % 144 == 0 {
                    BlendPalettesGradually(65534, 0, 16, 0, PALETTES_BG as u16, 0, 0);
                    BlendPalettesGradually(PALETTES_OBJECTS, 0, 16, 0, 0, 0, 1);
                }
            }
        }
        3 => {
            BeginNormalPaletteFade(PALETTES_ALL, 4, 0, 0x10, 0);
            task_set_func(taskId, Some(Task_RayChasesAwayEnd));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_ChasesAway_AnimateBg(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data as i32 & 0xF == 0 {
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (*data.at(1) as u16 + 14) << 8 & 0x1F00 | (*data.at(2) as u16 + 9) & 0xF,
        );
        *data.at(1) -= *data.at(3);
        *data.at(2) += *data.at(4);
        if *data.at(1) == -3 || *data.at(1) == 0 {
            *data.at(3) *= -1;
        }
        if *data.at(2) == 3 || *data.at(2) == 0 {
            *data.at(4) *= -1;
        }
    }
    *data += 1;
}
pub(crate) unsafe fn Task_RayChasesAwayEnd(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if (*gPaletteFade.as_ptr()).active() == 0 {
        StopMapMusic();
        if *data.at(1) == 0 {
            SetVBlankCallback(None);
            ResetWindowDimensions();
            ResetSpriteData();
            FreeAllSpritePalettes();
            DestroyTask(*data.at(2) as u8);
        }
        if *data.at(1) == 32 {
            *data.at(1) = 0;
            task_set_func(taskId, Some(Task_SetNextAnim));
        } else {
            *data.at(1) += 1;
        }
    }
}
unsafe fn ChasesAway_CreateTrioSprites(taskId: u8) {
    let taskData: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *taskData.at(3) = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_Groudon).cast_mut(),
        64,
        120,
        0,
    ) as i16;
    let mut spriteData: *mut i16 = (*gSprites.as_ptr())[*taskData.at(3)].data.as_mut_ptr();
    *spriteData = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_GroudonTail).cast_mut(),
        16,
        130,
        0,
    ) as i16;
    (*gSprites.as_ptr())[*taskData.at(3)].oam.set_priority(1);
    (*gSprites.as_ptr())[*spriteData].oam.set_priority(1);
    *taskData.at(4) = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_Kyogre).cast_mut(),
        160,
        128,
        1,
    ) as i16;
    spriteData = (*gSprites.as_ptr())[*taskData.at(4)].data.as_mut_ptr();
    *spriteData = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_Kyogre).cast_mut(),
        192,
        128,
        1,
    ) as i16;
    *spriteData.at(1) = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_Kyogre).cast_mut(),
        224,
        128,
        1,
    ) as i16;
    (*gSprites.as_ptr())[*taskData.at(4)].oam.set_priority(1);
    (*gSprites.as_ptr())[*spriteData].oam.set_priority(1);
    (*gSprites.as_ptr())[*spriteData.at(1)].oam.set_priority(1);
    StartSpriteAnim(
        {
            let gl_i140 = *spriteData;
            crate::gl!(&raw gSprites[gl_i140])
        },
        1,
    );
    StartSpriteAnim(
        {
            let gl_i141 = *spriteData.at(1);
            crate::gl!(&raw gSprites[gl_i141])
        },
        2,
    );
    *taskData.at(5) = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_Rayquaza).cast_mut(),
        120,
        -65,
        0,
    ) as i16;
    spriteData = (*gSprites.as_ptr())[*taskData.at(5)].data.as_mut_ptr();
    *spriteData = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_RayquazaTail).cast_mut(),
        120,
        -113,
        0,
    ) as i16;
    (*gSprites.as_ptr())[*taskData.at(5)].oam.set_priority(1);
    (*gSprites.as_ptr())[*spriteData].oam.set_priority(1);
}
unsafe fn ChasesAway_PushDuoBack(taskId: u8) {
    let taskData: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    {
        let gl_i142 = *taskData.at(3);
        crate::gl!(gSprites[gl_i142].callback = Some(SpriteCB_ChasesAway_DuoRingPush))
    };
    {
        let gl_i143 = *taskData.at(3);
        crate::gl!(gSprites[gl_i143].data[4] = 0)
    };
    {
        let gl_i144 = *taskData.at(3);
        let gl_i145 = sDecel;
        crate::gl!(gSprites[gl_i144].data[gl_i145] = 0)
    };
    {
        let gl_i146 = *taskData.at(3);
        let gl_i147 = sSpeed;
        crate::gl!(gSprites[gl_i146].data[gl_i147] = 4)
    };
    {
        let gl_i148 = *taskData.at(3);
        let gl_i149 = sIsKyogre;
        crate::gl!(gSprites[gl_i148].data[gl_i149] = FALSE as i16)
    };
    {
        let gl_i150 = *taskData.at(4);
        crate::gl!(gSprites[gl_i150].callback = Some(SpriteCB_ChasesAway_DuoRingPush))
    };
    {
        let gl_i151 = *taskData.at(4);
        crate::gl!(gSprites[gl_i151].data[4] = 0)
    };
    {
        let gl_i152 = *taskData.at(4);
        let gl_i153 = sDecel;
        crate::gl!(gSprites[gl_i152].data[gl_i153] = 0)
    };
    {
        let gl_i154 = *taskData.at(4);
        let gl_i155 = sSpeed;
        crate::gl!(gSprites[gl_i154].data[gl_i155] = 4)
    };
    {
        let gl_i156 = *taskData.at(4);
        let gl_i157 = sIsKyogre;
        crate::gl!(gSprites[gl_i156].data[gl_i157] = TRUE as i16)
    };
}
pub(crate) unsafe fn SpriteCB_ChasesAway_DuoRingPush(sprite: *mut Sprite) {
    if (*sprite).data[4] as i32 & 7 == 0 {
        if (*sprite).data[sIsKyogre] == 0 {
            (*sprite).x -= (*sprite).data[sSpeed];
            {
                let gl_i158 = (*sprite).data[sBodyPartSpriteId1];
                crate::gl!(gSprites[gl_i158].x -= (*sprite).data[sSpeed])
            };
        } else {
            (*sprite).x += (*sprite).data[sSpeed];
            {
                let gl_i159 = (*sprite).data[sBodyPartSpriteId1];
                crate::gl!(gSprites[gl_i159].x += (*sprite).data[sSpeed])
            };
            {
                let gl_i160 = (*sprite).data[sBodyPartSpriteId2];
                crate::gl!(gSprites[gl_i160].x += (*sprite).data[sSpeed])
            };
        }
        (*sprite).data[sDecel] += 1;
        (*sprite).data[sSpeed] -= (*sprite).data[sDecel];
        if (*sprite).data[sDecel] == 3 {
            (*sprite).data[4] = 0;
            (*sprite).data[sDecel] = 0;
            (*sprite).data[sSpeed] = 0;
            (*sprite).callback = Some(SpriteCallbackDummy);
            return;
        }
    }
    (*sprite).data[4] += 1;
}
unsafe fn ChasesAway_GroudonStartLeave(taskId: u8) {
    let taskData: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    {
        let gl_i161 = *taskData.at(3);
        crate::gl!(gSprites[gl_i161].callback = Some(SpriteCB_ChasesAway_GroudonLeave))
    };
    StartSpriteAnim(
        {
            let gl_i162 = *taskData.at(3);
            crate::gl!(&raw gSprites[gl_i162])
        },
        1,
    );
}
pub(crate) unsafe fn SpriteCB_ChasesAway_GroudonLeave(sprite: *mut Sprite) {
    match (*sprite).animCmdIndex {
        0 | 2 => {
            if (*sprite).animDelayCounter() as i32 % 12 == 0 {
                (*sprite).x -= 2;
                {
                    let gl_i163 = (*sprite).data[0];
                    crate::gl!(gSprites[gl_i163].x -= 2)
                };
            }
            {
                let gl_i164 = (*sprite).data[0];
                crate::gl!(gSprites[gl_i164].y2 = 0)
            };
        }
        1 | 3 => {
            {
                let gl_i165 = (*sprite).data[0];
                crate::gl!(gSprites[gl_i165].y2 = -2)
            };
            if (*sprite).animDelayCounter() as i32 & 15 == 0 {
                (*sprite).y += 1;
                {
                    let gl_i166 = (*sprite).data[0];
                    crate::gl!(gSprites[gl_i166].y += 1)
                };
            }
        }
        _ => {}
    }
}
unsafe fn ChasesAway_KyogreStartLeave(taskId: u8) {
    let taskData: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let spriteData: *mut i16 = (*gSprites.as_ptr())[*taskData.at(4)].data.as_mut_ptr();
    {
        let gl_i167 = *taskData.at(4);
        crate::gl!(gSprites[gl_i167].callback = Some(SpriteCB_ChasesAway_KyogreLeave))
    };
    {
        let gl_i168 = *spriteData;
        crate::gl!(gSprites[gl_i168].callback = Some(SpriteCB_ChasesAway_KyogreLeave))
    };
    {
        let gl_i169 = *spriteData.at(1);
        crate::gl!(gSprites[gl_i169].callback = Some(SpriteCB_ChasesAway_KyogreLeave))
    };
}
pub(crate) unsafe fn SpriteCB_ChasesAway_KyogreLeave(sprite: *mut Sprite) {
    if (*sprite).data[4] as i32 & 3 == 0 {
        if (*sprite).x2 == 1 {
            (*sprite).x2 = -1;
        } else {
            (*sprite).x2 = 1;
        }
    }
    if (*sprite).data[5] == 128 {
        (*sprite).data[7] = CreateSprite(
            (&raw const *sSpriteTemplate_ChasesAway_KyogreSplash).cast_mut(),
            152,
            132,
            0,
        ) as i16;
        (*gSprites.as_ptr())[(*sprite).data[7]].oam.set_priority(1);
        (*sprite).data[7] = CreateSprite(
            (&raw const *sSpriteTemplate_ChasesAway_KyogreSplash).cast_mut(),
            224,
            132,
            0,
        ) as i16;
        (*gSprites.as_ptr())[(*sprite).data[7]].oam.set_priority(1);
        (*gSprites.as_ptr())[(*sprite).data[7]].set_hFlip(1);
        (*sprite).data[5] += 1;
    }
    if (*sprite).data[5] > 127 {
        if (*sprite).y2 != 32 {
            (*sprite).data[6] += 1;
            (*sprite).y2 = (*sprite).data[6] >> 4;
        }
    } else {
        (*sprite).data[5] += 1;
    }
    if (*sprite).data[4] % 64 == 0 {
        PlaySE(SE_M_WHIRLPOOL);
    }
    (*sprite).data[4] += 1;
}
pub(crate) unsafe fn SpriteCB_ChasesAway_Rayquaza(sprite: *mut Sprite) {
    let frame: i16 = (*sprite).data[7];
    if frame <= 64 {
        (*sprite).y2 += 2;
        {
            let gl_i170 = (*sprite).data[sTailSpriteId];
            crate::gl!(gSprites[gl_i170].y2 += 2)
        };
        if (*sprite).data[7] == 64 {
            ChasesAway_SetRayquazaAnim(sprite, 1, 0, -48);
            (*sprite).data[4] = 5;
            (*sprite).data[5] = -1;
            {
                let gl_i171 = (*sprite).data[sTailSpriteId];
                crate::gl!(gSprites[gl_i171].data[4] = 3)
            };
            {
                let gl_i172 = (*sprite).data[sTailSpriteId];
                crate::gl!(gSprites[gl_i172].data[5] = 5)
            };
        }
    } else if frame <= 111 {
        SpriteCB_ChasesAway_RayquazaFloat(sprite);
        if (*sprite).data[4] == 0 {
            PlaySE(SE_MUGSHOT);
        }
        if (*sprite).data[4] == -3 {
            ChasesAway_SetRayquazaAnim(sprite, 2, 48, 16);
        }
    } else if frame == 112 {
        {
            let gl_i173 = (*sprite).data[sTailSpriteId];
            crate::gl!(gSprites[gl_i173].data[4] = 7)
        };
        {
            let gl_i174 = (*sprite).data[sTailSpriteId];
            crate::gl!(gSprites[gl_i174].data[5] = 3)
        };
        SpriteCB_ChasesAway_RayquazaFloat(sprite);
    } else if frame <= 327 {
        SpriteCB_ChasesAway_RayquazaFloat(sprite);
    } else if frame == 328 {
        SpriteCB_ChasesAway_RayquazaFloat(sprite);
        ChasesAway_SetRayquazaAnim(sprite, 3, 48, 16);
        (*sprite).x2 = 1;
        {
            let gl_i175 = (*sprite).data[sTailSpriteId];
            crate::gl!(gSprites[gl_i175].x2 = 1)
        };
        PlayCry_Normal(SPECIES_RAYQUAZA as u16, 0);
        CreateTask(Some(Task_ChasesAway_AnimateRing), 0);
    } else {
        match frame {
            376 => {
                (*sprite).x2 = 0;
                {
                    let gl_i176 = (*sprite).data[sTailSpriteId];
                    crate::gl!(gSprites[gl_i176].x2 = 0)
                };
                SpriteCB_ChasesAway_RayquazaFloat(sprite);
                ChasesAway_SetRayquazaAnim(sprite, 2, 48, 16);
                (*sprite).callback = Some(SpriteCB_ChasesAway_RayquazaFloat);
                return;
            }
            352 => {
                ChasesAway_PushDuoBack(FindTaskIdByFunc(Some(Task_HandleRayChasesAway)));
            }
            _ => {}
        }
    }
    if (*sprite).data[7] > 328 && (*sprite).data[7] as i32 & 1 == 0 {
        (*sprite).x2 *= -1;
        {
            let gl_i177 = (*sprite).data[sTailSpriteId];
            crate::gl!(gSprites[gl_i177].x2 = (*sprite).x2)
        };
    }
    (*sprite).data[7] += 1;
}
pub(crate) unsafe fn SpriteCB_ChasesAway_RayquazaFloat(body: *mut Sprite) {
    let tail: *mut Sprite = {
        let gl_i178 = (*body).data[sTailSpriteId];
        crate::gl!(&raw gSprites[gl_i178])
    };
    if (*body).data[sFloatTimer] as i32 & (*tail).data[4] as i32 == 0 {
        (*body).y2 += (*body).data[4];
        {
            let gl_i179 = (*body).data[sTailSpriteId];
            crate::gl!(gSprites[gl_i179].y2 += (*body).data[4])
        };
        (*body).data[4] += (*body).data[5];
        if (*body).data[4] >= (*tail).data[5] || (*body).data[4] as i32 <= -((*tail).data[5] as i32)
        {
            if (*body).data[4] > (*tail).data[5] {
                (*body).data[4] = (*tail).data[5];
            } else if ((*body).data[4] as i32) < -((*tail).data[5] as i32) {
                (*body).data[4] = -(*tail).data[5];
            }
            (*body).data[5] *= -1;
        }
    }
    (*body).data[sFloatTimer] += 1;
}
unsafe fn ChasesAway_SetRayquazaAnim(body: *mut Sprite, animNum: u8, x: i16, y: i16) {
    let tail: *mut Sprite = {
        let gl_i180 = (*body).data[sTailSpriteId];
        crate::gl!(&raw gSprites[gl_i180])
    };
    (*tail).x = (*body).x + x;
    (*tail).y = (*body).y + y;
    (*tail).x2 = (*body).x2;
    (*tail).y2 = (*body).y2;
    StartSpriteAnim(body, animNum);
    StartSpriteAnim(tail, animNum);
}
pub(crate) unsafe fn Task_ChasesAway_AnimateRing(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            SetBgAffine(2, 0x4000, 0x4000, 120, 64, 256, 256, 0);
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
            *data.at(4) = 16;
            *data += 1;
        }
        1 => {
            if *data.at(5) == 8 {
                PlaySE(SE_SLIDING_DOOR as u16);
            }
            if *data.at(2) == 2 {
                *data += 1;
            } else {
                *data.at(1) += *data.at(4);
                *data.at(5) += 1;
                if *data.at(3) % 3 == 0 && *data.at(4) != 4 {
                    *data.at(4) -= 2;
                }
                *data.at(3) += 1;
                SetBgAffine(
                    2,
                    0x4000,
                    0x4000,
                    120,
                    64,
                    256 - *data.at(1),
                    256 - *data.at(1),
                    0,
                );
                if *data.at(1) > 255 {
                    *data.at(1) = 0;
                    *data.at(3) = 0;
                    *data.at(5) = 0;
                    *data.at(4) = 16;
                    *data.at(2) += 1;
                }
            }
        }
        2 => {
            ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
