//! Translated from `src/rayquaza_scene.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sTasksForAnimations sOam_64x64 sOam_32x32 sOam_64x32 sOam_32x16 sOam_16x8 sOam_16x32 sOam_16x16 sOam_32x8 sAnim_DuoFightPre_Groudon_Head sAnim_DuoFightPre_Groudon_Body sAnims_DuoFightPre_Groudon sSpriteTemplate_DuoFightPre_Groudon sAnim_DuoFightPre_GroudonShoulderKyogreDorsalFin sAnims_DuoFightPre_GroudonShoulderKyogreDorsalFin sSpriteTemplate_DuoFightPre_GroudonShoulder sAnim_DuoFightPre_GroudonClaw sAnims_DuoFightPre_GroudonClaw sSpriteTemplate_DuoFightPre_GroudonClaw sAnim_DuoFightPre_Kyogre_TopLeft sAnim_DuoFightPre_Kyogre_TopRight sAnim_DuoFightPre_Kyogre_FaceLeft sAnim_DuoFightPre_Kyogre_FaceRight sAnim_DuoFightPre_Kyogre_ChinLeft sAnim_DuoFightPre_Kyogre_ChinRight sAnim_DuoFightPre_Kyogre_LeftPectoralFin sAnim_DuoFightPre_Kyogre_LeftShoulder sAnim_DuoFightPre_Kyogre_RightShoulder sAnims_DuoFightPre_Kyogre sSpriteTemplate_DuoFightPre_Kyogre sAnim_DuoFightPre_KyogrePectoralFin sAnims_DuoFightPre_KyogrePectoralFin sSpriteTemplate_DuoFightPre_KyogrePectoralFin sSpriteTemplate_DuoFightPre_KyogreDorsalFin sScanlineParams_DuoFight_Clouds sBgTemplates_DuoFight sAnim_DuoFight_Groudon_Head sAnim_DuoFight_Groudon_Body sAnims_DuoFight_Groudon sSpriteSheet_DuoFight_Groudon sSpritePal_DuoFight_Groudon sSpriteTemplate_DuoFight_Groudon sAnim_DuoFight_GroudonShoulderKyogreDorsalFin sAnims_DuoFight_GroudonShoulderKyogreDorsalFin sSpriteSheet_DuoFight_GroudonShoulder sSpriteTemplate_DuoFight_GroudonShoulder sAnim_DuoFight_GroudonClaw sAnims_DuoFight_GroudonClaw sSpriteSheet_DuoFight_GroudonClaw sSpriteTemplate_DuoFight_GroudonClaw sAnim_DuoFight_Kyogre_TopLeft sAnim_DuoFight_Kyogre_TopRight sAnim_DuoFight_Kyogre_FaceLeft sAnim_DuoFight_Kyogre_FaceRight sAnim_DuoFight_Kyogre_ChinLeft sAnim_DuoFight_Kyogre_ChinRight sAnim_DuoFight_Kyogre_LeftPectoralFin sAnim_DuoFight_Kyogre_LeftShoulder sAnim_DuoFight_Kyogre_RightShoulder sAnims_DuoFight_Kyogre sSpriteSheet_DuoFight_Kyogre sSpritePal_DuoFight_Kyogre sSpriteTemplate_DuoFight_Kyogre sAnim_DuoFight_KyogrePectoralFin sAnims_DuoFight_KyogrePectoralFin sSpriteSheet_DuoFight_KyogrePectoralFin sSpriteTemplate_DuoFight_KyogrePectoralFin sSpriteSheet_DuoFight_KyogreDorsalFin sSpriteTemplate_DuoFight_KyogreDorsalFin sBgTemplates_TakesFlight sAnim_TakesFlight_Smoke sAnims_TakesFlight_Smoke sAffineAnim_TakesFlight_Smoke sAffineAnims_TakesFlight_Smoke sSpriteSheet_TakesFlight_Smoke sSpritePal_TakesFlight_Smoke sSpriteTemplate_TakesFlight_Smoke sTakesFlight_SmokeCoords sBgTemplates_Descends sAnim_Descends_Rayquaza sAnims_Descends_Rayquaza sAnim_Descends_RayquazaTail sAnims_Descends_RayquazaTail sSpriteSheet_Descends_Rayquaza sSpriteSheet_Descends_RayquazaTail sSpritePal_Descends_Rayquaza sSpriteTemplate_Descends_Rayquaza sSpriteTemplate_Descends_RayquazaTail sBgTemplates_Charges sAnim_ChasesAway_Groudon_Still sAnim_ChasesAway_Groudon_Moving sAnims_ChasesAway_Groudon sAnim_ChasesAway_GroudonTail sAnims_ChasesAway_GroudonTail sAnim_ChasesAway_Kyogre_Front sAnim_ChasesAway_Kyogre_Back sAnim_ChasesAway_Kyogre_Tail sAnims_ChasesAway_Kyogre sAnim_ChasesAway_Rayquaza_FlyingDown sAnim_ChasesAway_Rayquaza_Arriving sAnim_ChasesAway_Rayquaza_Floating sAnim_ChasesAway_Rayquaza_Shouting sAnims_ChasesAway_Rayquaza sAnim_ChasesAway_RayquazaTail_FlyingDown sAnim_ChasesAway_RayquazaTail_Arriving sAnim_ChasesAway_RayquazaTail_Floating sAnim_ChasesAway_RayquazaTail_Shouting sAnims_ChasesAway_RayquazaTail sAnim_ChasesAway_KyogreSplash sAnims_ChasesAway_KyogreSplash sSpriteSheet_ChasesAway_Groudon sSpriteSheet_ChasesAway_GroudonTail sSpriteSheet_ChasesAway_Kyogre sSpriteSheet_ChasesAway_Rayquaza sSpriteSheet_ChasesAway_RayquazaTail sSpriteSheet_ChasesAway_KyogreSplash sSpritePal_ChasesAway_Groudon sSpritePal_ChasesAway_Kyogre sSpritePal_ChasesAway_Rayquaza sSpritePal_ChasesAway_KyogreSplash sSpriteTemplate_ChasesAway_Groudon sSpriteTemplate_ChasesAway_GroudonTail sSpriteTemplate_ChasesAway_Kyogre sSpriteTemplate_ChasesAway_Rayquaza sSpriteTemplate_ChasesAway_RayquazaTail sSpriteTemplate_ChasesAway_KyogreSplash sBgTemplates_ChasesAway

/// `struct RayquazaScene`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RayquazaScene {
    pub exitCallback: Option<unsafe extern "C" fn()>,
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
static sTasksForAnimations: Table<CArray<Option<unsafe extern "C" fn(u8)>, 7>> =
    Table((&raw const crate::data::rayquaza_scene::sTasksForAnimations).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRayScene: *mut RayquazaScene = null_mut();

unsafe extern "C" {
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static gRaySceneCharges_Bg_Gfx: CArray<u32, 0>;
    static gRaySceneCharges_Bg_Pal: CArray<u32, 0>;
    static gRaySceneCharges_Bg_Tilemap: CArray<u32, 0>;
    static gRaySceneCharges_Orbs_Tilemap: CArray<u32, 0>;
    static gRaySceneCharges_Rayquaza_Gfx: CArray<u32, 0>;
    static gRaySceneCharges_Rayquaza_Tilemap: CArray<u32, 0>;
    static gRaySceneCharges_Streaks_Gfx: CArray<u32, 0>;
    static gRaySceneCharges_Streaks_Tilemap: CArray<u32, 0>;
    static gRaySceneChasesAway_Bg_Pal: CArray<u32, 0>;
    static gRaySceneChasesAway_Bg_Tilemap: CArray<u32, 0>;
    static gRaySceneChasesAway_Light_Gfx: CArray<u32, 0>;
    static gRaySceneChasesAway_Light_Tilemap: CArray<u32, 0>;
    static gRaySceneChasesAway_Ring_Gfx: CArray<u32, 0>;
    static gRaySceneChasesAway_Ring_Tilemap: CArray<u32, 0>;
    static gRaySceneDescends_Bg_Gfx: CArray<u32, 0>;
    static gRaySceneDescends_Bg_Pal: CArray<u32, 0>;
    static gRaySceneDescends_Bg_Tilemap: CArray<u32, 0>;
    static gRaySceneDescends_Light_Gfx: CArray<u32, 0>;
    static gRaySceneDescends_Light_Tilemap: CArray<u32, 0>;
    static gRaySceneDuoFight_Clouds1_Tilemap: CArray<u32, 0>;
    static gRaySceneDuoFight_Clouds2_Tilemap: CArray<u32, 0>;
    static gRaySceneDuoFight_Clouds3_Tilemap: CArray<u32, 0>;
    static gRaySceneDuoFight_Clouds_Gfx: CArray<u32, 0>;
    static gRaySceneDuoFight_Clouds_Pal: CArray<u32, 0>;
    static gRaySceneTakesFlight_Bg_Gfx: CArray<u32, 0>;
    static gRaySceneTakesFlight_Bg_Tilemap: CArray<u32, 0>;
    static gRaySceneTakesFlight_Rayquaza_Gfx: CArray<u32, 0>;
    static gRaySceneTakesFlight_Rayquaza_Pal: CArray<u32, 0>;
    static gRaySceneTakesFlight_Rayquaza_Tilemap: CArray<u32, 0>;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlendPalettesGradually(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16, a5: u8, a6: u8);
    fn BuildOamBuffer();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearScheduledBgCopiesToVram();
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroyTask(a0: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn EnableInterrupts(a0: u16);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBgY(a0: u8) -> i32;
    fn GetGpuReg(a0: u8) -> u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBgAffine(a0: u8, a1: i32, a2: i32, a3: i16, a4: i16, a5: i16, a6: i16, a7: u16);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StopMapMusic();
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoRayquazaScene(
    animId: u8,
    endEarly: u8,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    sRayScene = AllocZeroed(8216) as *mut RayquazaScene;
    (*sRayScene).animId = animId;
    (*sRayScene).exitCallback = exitCallback;
    (*sRayScene).endEarly = endEarly;
    SetMainCallback2(Some(CB2_InitRayquazaScene));
}
pub(crate) unsafe extern "C" fn CB2_InitRayquazaScene() {
    SetVBlankHBlankCallbacksToNull();
    ClearScheduledBgCopiesToVram();
    ScanlineEffect_Stop();
    FreeAllSpritePalettes();
    ResetPaletteFade();
    ResetSpriteData();
    ResetTasks();
    FillPalette(0, 240, 32);
    CreateTask(sTasksForAnimations[(*sRayScene).animId], 0);
    SetMainCallback2(Some(CB2_RayquazaScene));
}
pub(crate) unsafe extern "C" fn CB2_RayquazaScene() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB_RayquazaScene() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn Task_EndAfterFadeScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        ResetSpriteData();
        FreeAllSpritePalettes();
        SetMainCallback2((*sRayScene).exitCallback);
        Free(sRayScene as *mut c_void);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_SetNextAnim(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if (*sRayScene).endEarly == TRUE {
            gTasks[taskId].func = Some(Task_EndAfterFadeScreen);
        } else {
            (*sRayScene).animId += 1;
            (*sRayScene).unk = 0;
            gTasks[taskId].func = sTasksForAnimations[(*sRayScene).animId];
        }
    }
}
pub(crate) unsafe extern "C" fn SetWindowsHideVertBorders() {
    SetGpuReg(REG_OFFSET_WININ, WININ_WIN0_ALL);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
    SetGpuReg(REG_OFFSET_WIN0V, 6280);
    gPlttBufferUnfaded[0] = 0;
    gPlttBufferFaded[0] = 0;
}
pub(crate) unsafe extern "C" fn ResetWindowDimensions() {
    SetGpuReg(REG_OFFSET_WININ, WININ_WIN0_ALL);
    SetGpuReg(REG_OFFSET_WINOUT, WINOUT_WIN01_ALL);
}
pub(crate) unsafe extern "C" fn Task_HandleDuoFightPre(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    DuoFight_AnimateRain();
    if gPaletteFade.active() == 0 {
        let mut frame: i16 = *data;
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
pub(crate) unsafe extern "C" fn DuoFightPre_CreateGroudonSprites() -> u8 {
    let mut spriteId: u8 = 0;
    let mut data: *mut i16 = null_mut();
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Groudon).cast_mut(),
        88,
        72,
        3,
    );
    gSprites[spriteId].callback = Some(SpriteCB_DuoFightPre_Groudon);
    data = gSprites[spriteId].data.as_mut_ptr();
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
    StartSpriteAnim(&raw mut gSprites[*data], 1);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_DuoFightPre_Groudon(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(5) += 1;
    *data.at(5) &= 0x1F;
    if *data.at(5) == 0 && (*sprite).x != 72 {
        (*sprite).x -= 1;
        gSprites[(*sprite).data[0]].x -= 1;
        gSprites[*data.at(1)].x -= 1;
        gSprites[*data.at(2)].x -= 1;
    }
    match (*sprite).animCmdIndex {
        0 => {
            gSprites[*data.at(1)].x2 = 0;
            gSprites[*data.at(1)].y2 = 0;
            gSprites[*data.at(2)].x2 = 0;
            gSprites[*data.at(2)].y2 = 0;
        }
        1 | 3 => {
            gSprites[*data.at(1)].x2 = -1;
            gSprites[*data.at(1)].y2 = 0;
            gSprites[*data.at(2)].x2 = -1;
            gSprites[*data.at(2)].y2 = 0;
        }
        2 => {
            gSprites[*data.at(1)].x2 = -1;
            gSprites[*data.at(1)].y2 = 1;
            gSprites[*data.at(2)].x2 = -2;
            gSprites[*data.at(2)].y2 = 1;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DuoFightPre_CreateKyogreSprites() -> u8 {
    let mut spriteId: u8 = 0;
    let mut data: *mut i16 = null_mut();
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFightPre_Kyogre).cast_mut(),
        136,
        96,
        1,
    );
    gSprites[spriteId].callback = Some(SpriteCB_DuoFightPre_Kyogre);
    data = gSprites[spriteId].data.as_mut_ptr();
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
    StartSpriteAnim(&raw mut gSprites[*data >> 8], 1);
    StartSpriteAnim(&raw mut gSprites[*data as i32 & 0xFF], 2);
    StartSpriteAnim(&raw mut gSprites[*data.at(1) >> 8], 3);
    StartSpriteAnim(&raw mut gSprites[*data.at(1) as i32 & 0xFF], 4);
    StartSpriteAnim(&raw mut gSprites[*data.at(2) >> 8], 5);
    StartSpriteAnim(&raw mut gSprites[*data.at(2) as i32 & 0xFF], 6);
    StartSpriteAnim(&raw mut gSprites[*data.at(3) >> 8], 7);
    StartSpriteAnim(&raw mut gSprites[*data.at(3) as i32 & 0xFF], 8);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_DuoFightPre_Kyogre(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(5) += 1;
    *data.at(5) &= 0x1F;
    if *data.at(5) == 0 && (*sprite).x != 152 {
        (*sprite).x += 1;
        gSprites[(*sprite).data[0] >> 8].x += 1;
        gSprites[(*sprite).data[0] as i32 & 0xFF].x += 1;
        gSprites[*data.at(1) >> 8].x += 1;
        gSprites[*data.at(1) as i32 & 0xFF].x += 1;
        gSprites[*data.at(2) >> 8].x += 1;
        gSprites[*data.at(2) as i32 & 0xFF].x += 1;
        gSprites[*data.at(3) >> 8].x += 1;
        gSprites[*data.at(3) as i32 & 0xFF].x += 1;
        gSprites[*data.at(4) >> 8].x += 1;
        gSprites[*data.at(4) as i32 & 0xFF].x += 1;
    }
    match gSprites[*data.at(2) as i32 & 0xFF].animCmdIndex {
        0 => {
            (*sprite).y2 = 0;
            gSprites[*data >> 8].y2 = 0;
            gSprites[*data as i32 & 0xFF].y2 = 0;
            gSprites[*data.at(1) >> 8].y2 = 0;
            gSprites[*data.at(1) as i32 & 0xFF].y2 = 0;
            gSprites[*data.at(2) >> 8].y2 = 0;
            gSprites[*data.at(2) as i32 & 0xFF].y2 = 0;
            gSprites[*data.at(3) >> 8].y2 = 0;
            gSprites[*data.at(3) as i32 & 0xFF].y2 = 0;
            gSprites[*data.at(4) >> 8].y2 = 0;
            gSprites[*data.at(4) as i32 & 0xFF].y2 = 0;
        }
        1 | 3 => {
            (*sprite).y2 = 1;
            gSprites[*data >> 8].y2 = 1;
            gSprites[*data as i32 & 0xFF].y2 = 1;
            gSprites[*data.at(1) >> 8].y2 = 1;
            gSprites[*data.at(1) as i32 & 0xFF].y2 = 1;
            gSprites[*data.at(2) >> 8].y2 = 1;
            gSprites[*data.at(2) as i32 & 0xFF].y2 = 1;
            gSprites[*data.at(3) >> 8].y2 = 1;
            gSprites[*data.at(3) as i32 & 0xFF].y2 = 1;
            gSprites[*data.at(4) >> 8].y2 = 1;
            gSprites[*data.at(4) as i32 & 0xFF].y2 = 1;
        }
        2 => {
            (*sprite).y2 = 2;
            gSprites[*data >> 8].y2 = 2;
            gSprites[*data as i32 & 0xFF].y2 = 2;
            gSprites[*data.at(1) >> 8].y2 = 2;
            gSprites[*data.at(1) as i32 & 0xFF].y2 = 2;
            gSprites[*data.at(2) >> 8].y2 = 2;
            gSprites[*data.at(4) as i32 & 0xFF].y2 = 2;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_DuoFight() {
    VBlankCB_RayquazaScene();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn InitDuoFightSceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates_DuoFight.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        0,
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
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
pub(crate) unsafe extern "C" fn LoadDuoFightSceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        0,
        gRaySceneDuoFight_Clouds_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        gRaySceneDuoFight_Clouds2_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneDuoFight_Clouds1_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneDuoFight_Clouds3_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    LoadCompressedPalette(gRaySceneDuoFight_Clouds_Pal.as_ptr().cast_mut(), 0, 64);
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_Groudon).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_GroudonShoulder).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_GroudonClaw).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_Kyogre).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_KyogrePectoralFin).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_DuoFight_KyogreDorsalFin).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_DuoFight_Groudon).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_DuoFight_Kyogre).cast_mut());
}
pub(crate) unsafe extern "C" fn Task_DuoFightAnim(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    ScanlineEffect_Clear();
    InitDuoFightSceneBgs();
    LoadDuoFightSceneGfx();
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            gScanlineEffectRegBuffers.as_mut_ptr() as *mut c_void,
            0x10003c0,
        );
    }
    ScanlineEffect_SetParams(*sScanlineParams_DuoFight_Clouds);
    *data = 0;
    *data.at(1) = CreateTask(Some(Task_DuoFight_AnimateClouds), 0) as i16;
    if (*sRayScene).animId == RAY_ANIM_DUO_FIGHT_PRE {
        *data.at(2) = DuoFightPre_CreateGroudonSprites() as i16;
        *data.at(3) = DuoFightPre_CreateKyogreSprites() as i16;
        gTasks[taskId].func = Some(Task_HandleDuoFightPre);
    } else {
        *data.at(2) = DuoFight_CreateGroudonSprites() as i16;
        *data.at(3) = DuoFight_CreateKyogreSprites() as i16;
        gTasks[taskId].func = Some(Task_HandleDuoFight);
        StopMapMusic();
    }
    BlendPalettes(PALETTES_ALL, 0x10, 0);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
    SetVBlankCallback(Some(VBlankCB_DuoFight));
    PlaySE(SE_DOWNPOUR);
}
pub(crate) unsafe extern "C" fn Task_DuoFight_AnimateClouds(taskId: u8) {
    let mut i: i16 = 0;
    let mut data: *mut u16 = gTasks[taskId].data.as_mut_ptr() as *mut u16;
    i = 24;
    while i < 92 {
        if i <= 47 {
            gScanlineEffectRegBuffers[0][i] = *data >> 8;
            gScanlineEffectRegBuffers[1][i] = *data >> 8;
        } else if i <= 63 {
            gScanlineEffectRegBuffers[0][i] = *data.at(1) >> 8;
            gScanlineEffectRegBuffers[1][i] = *data.at(1) >> 8;
        } else if i <= 75 {
            gScanlineEffectRegBuffers[0][i] = *data.at(2) >> 8;
            gScanlineEffectRegBuffers[1][i] = *data.at(2) >> 8;
        } else if i <= 83 {
            gScanlineEffectRegBuffers[0][i] = *data.at(3) >> 8;
            gScanlineEffectRegBuffers[1][i] = *data.at(3) >> 8;
        } else if i <= 87 {
            gScanlineEffectRegBuffers[0][i] = *data.at(4) >> 8;
            gScanlineEffectRegBuffers[1][i] = *data.at(4) >> 8;
        } else {
            gScanlineEffectRegBuffers[0][i] = *data.at(5) >> 8;
            gScanlineEffectRegBuffers[1][i] = *data.at(5) >> 8;
        }
        i += 1;
    }
    if (*sRayScene).animId == RAY_ANIM_DUO_FIGHT_PRE {
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
pub(crate) unsafe extern "C" fn Task_HandleDuoFight(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    DuoFight_AnimateRain();
    if gPaletteFade.active() == 0 {
        let mut frame: i16 = *data;
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
                    gTasks[*data.at(1)].func = Some(DuoFight_PanOffScene);
                    gTasks[*data.at(1)].data[0] = 0;
                    gTasks[*data.at(1)].data[2] = *data.at(2);
                    gTasks[*data.at(1)].data[3] = *data.at(3);
                    ScanlineEffect_Stop();
                }
                _ => {}
            }
        }
        *data += 1;
    }
}
pub(crate) unsafe extern "C" fn DuoFight_Lightning1() {
    PlaySE(SE_THUNDER);
    BlendPalettesGradually(32767, 0, 16, 0, PALETTES_BG as u16, 0, 0);
    BlendPalettesGradually(PALETTES_OBJECTS, 0, 16, 0, 0, 0, 1);
}
pub(crate) unsafe extern "C" fn DuoFight_Lightning2() {
    PlaySE(SE_THUNDER);
    BlendPalettesGradually(32767, 0, 16, 16, PALETTES_BG as u16, 0, 0);
    BlendPalettesGradually(PALETTES_OBJECTS, 0, 16, 16, 0, 0, 1);
}
pub(crate) unsafe extern "C" fn DuoFight_LightningLong() {
    BlendPalettesGradually(32767, 4, 16, 0, PALETTES_BG as u16, 0, 0);
    BlendPalettesGradually(PALETTES_OBJECTS, 4, 16, 0, 0, 0, 1);
}
pub(crate) unsafe extern "C" fn DuoFight_AnimateRain() {
    ChangeBgX(2, 0x400, BG_COORD_ADD);
    ChangeBgY(2, 0x800, BG_COORD_SUB);
}
pub(crate) unsafe extern "C" fn DuoFight_PanOffScene(taskId: u8) {
    let mut bgY: u16 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    DuoFight_SlideGroudonDown(&raw mut gSprites[*data.at(2)]);
    DuoFight_SlideKyogreDown(&raw mut gSprites[*data.at(3)]);
    bgY = GetBgY(1) as u16;
    if GetBgY(1) == 0 || bgY > 0x8000 {
        ChangeBgY(1, 0x400, BG_COORD_SUB);
    }
    if *data != 16 {
        *data += 1;
        SetGpuReg(REG_OFFSET_BLDALPHA, (*data as u16) << 8 | 16 - *data as u16);
    }
}
pub(crate) unsafe extern "C" fn DuoFightEnd(taskId: u8, palDelay: i8) {
    PlaySE(SE_DOWNPOUR_STOP);
    BeginNormalPaletteFade(PALETTES_ALL, palDelay, 0, 0x10, 0);
    gTasks[taskId].func = Some(Task_DuoFightEnd);
}
pub(crate) unsafe extern "C" fn Task_DuoFightEnd(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    DuoFight_AnimateRain();
    if gPaletteFade.active() == 0 {
        DestroyTask(*data.at(1) as u8);
        ChangeBgY(1, 0, BG_COORD_SET);
        SetVBlankCallback(None);
        ScanlineEffect_Stop();
        ResetSpriteData();
        FreeAllSpritePalettes();
        *data = 0;
        gTasks[taskId].func = Some(Task_SetNextAnim);
    }
}
pub(crate) unsafe extern "C" fn DuoFight_CreateGroudonSprites() -> u8 {
    let mut spriteId: u8 = 0;
    let mut data: *mut i16 = null_mut();
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Groudon).cast_mut(),
        98,
        72,
        3,
    );
    gSprites[spriteId].callback = Some(SpriteCB_DuoFight_Groudon);
    data = gSprites[spriteId].data.as_mut_ptr();
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
    StartSpriteAnim(&raw mut gSprites[*data], 1);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_DuoFight_Groudon(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(5) += 1;
    *data.at(5) &= 0xF;
    if *data.at(5) as i32 & 7 == 0 && (*sprite).x != 72 {
        (*sprite).x -= 1;
        gSprites[(*sprite).data[0]].x -= 1;
        gSprites[*data.at(1)].x -= 1;
        gSprites[*data.at(2)].x -= 1;
    }
    match (*sprite).animCmdIndex {
        0 => {
            gSprites[*data.at(1)].x2 = 0;
            gSprites[*data.at(1)].y2 = 0;
            gSprites[*data.at(2)].x2 = 0;
            gSprites[*data.at(2)].y2 = 0;
        }
        1 | 3 => {
            gSprites[*data.at(1)].x2 = -1;
            gSprites[*data.at(1)].y2 = 0;
            gSprites[*data.at(2)].x2 = -1;
            gSprites[*data.at(2)].y2 = 0;
        }
        2 => {
            gSprites[*data.at(1)].x2 = -1;
            gSprites[*data.at(1)].y2 = 1;
            gSprites[*data.at(2)].x2 = -2;
            gSprites[*data.at(2)].y2 = 1;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DuoFight_SlideGroudonDown(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    if (*sprite).y <= DISPLAY_HEIGHT as i16 {
        (*sprite).y += 8;
        gSprites[(*sprite).data[0]].y += 8;
        gSprites[*data.at(1)].y += 8;
        gSprites[*data.at(2)].y += 8;
    }
}
pub(crate) unsafe extern "C" fn DuoFight_CreateKyogreSprites() -> u8 {
    let mut spriteId: u8 = 0;
    let mut data: *mut i16 = null_mut();
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_DuoFight_Kyogre).cast_mut(),
        126,
        96,
        1,
    );
    gSprites[spriteId].callback = Some(SpriteCB_DuoFight_Kyogre);
    data = gSprites[spriteId].data.as_mut_ptr();
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
    StartSpriteAnim(&raw mut gSprites[*data >> 8], 1);
    StartSpriteAnim(&raw mut gSprites[*data as i32 & 0xFF], 2);
    StartSpriteAnim(&raw mut gSprites[*data.at(1) >> 8], 3);
    StartSpriteAnim(&raw mut gSprites[*data.at(1) as i32 & 0xFF], 4);
    StartSpriteAnim(&raw mut gSprites[*data.at(2) >> 8], 5);
    StartSpriteAnim(&raw mut gSprites[*data.at(2) as i32 & 0xFF], 6);
    StartSpriteAnim(&raw mut gSprites[*data.at(3) >> 8], 7);
    StartSpriteAnim(&raw mut gSprites[*data.at(3) as i32 & 0xFF], 8);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_DuoFight_Kyogre(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(5) += 1;
    *data.at(5) &= 0xF;
    if *data.at(5) as i32 & 7 == 0 && (*sprite).x != 152 {
        (*sprite).x += 1;
        gSprites[(*sprite).data[0] >> 8].x += 1;
        gSprites[(*sprite).data[0] as i32 & 0xFF].x += 1;
        gSprites[*data.at(1) >> 8].x += 1;
        gSprites[*data.at(1) as i32 & 0xFF].x += 1;
        gSprites[*data.at(2) >> 8].x += 1;
        gSprites[*data.at(2) as i32 & 0xFF].x += 1;
        gSprites[*data.at(3) >> 8].x += 1;
        gSprites[*data.at(3) as i32 & 0xFF].x += 1;
        gSprites[*data.at(4) >> 8].x += 1;
        gSprites[*data.at(4) as i32 & 0xFF].x += 1;
    }
    match gSprites[*data.at(2) as i32 & 0xFF].animCmdIndex {
        0 => {
            (*sprite).y2 = 0;
            gSprites[*data >> 8].y2 = 0;
            gSprites[*data as i32 & 0xFF].y2 = 0;
            gSprites[*data.at(1) >> 8].y2 = 0;
            gSprites[*data.at(1) as i32 & 0xFF].y2 = 0;
            gSprites[*data.at(2) >> 8].y2 = 0;
            gSprites[*data.at(2) as i32 & 0xFF].y2 = 0;
            gSprites[*data.at(3) >> 8].y2 = 0;
            gSprites[*data.at(3) as i32 & 0xFF].y2 = 0;
            gSprites[*data.at(4) >> 8].y2 = 0;
            gSprites[*data.at(4) as i32 & 0xFF].y2 = 0;
        }
        1 | 3 => {
            (*sprite).y2 = 1;
            gSprites[*data >> 8].y2 = 1;
            gSprites[*data as i32 & 0xFF].y2 = 1;
            gSprites[*data.at(1) >> 8].y2 = 1;
            gSprites[*data.at(1) as i32 & 0xFF].y2 = 1;
            gSprites[*data.at(2) >> 8].y2 = 1;
            gSprites[*data.at(2) as i32 & 0xFF].y2 = 1;
            gSprites[*data.at(3) >> 8].y2 = 1;
            gSprites[*data.at(3) as i32 & 0xFF].y2 = 1;
            gSprites[*data.at(4) >> 8].y2 = 1;
            gSprites[*data.at(4) as i32 & 0xFF].y2 = 1;
        }
        2 => {
            (*sprite).y2 = 2;
            gSprites[*data >> 8].y2 = 2;
            gSprites[*data as i32 & 0xFF].y2 = 2;
            gSprites[*data.at(1) >> 8].y2 = 2;
            gSprites[*data.at(1) as i32 & 0xFF].y2 = 2;
            gSprites[*data.at(2) >> 8].y2 = 2;
            gSprites[*data.at(4) as i32 & 0xFF].y2 = 2;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DuoFight_SlideKyogreDown(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    if (*sprite).y <= DISPLAY_HEIGHT as i16 {
        (*sprite).y += 8;
        gSprites[(*sprite).data[0] >> 8].y += 8;
        gSprites[(*sprite).data[0] as i32 & 0xFF].y += 8;
        gSprites[*data.at(1) >> 8].y += 8;
        gSprites[*data.at(1) as i32 & 0xFF].y += 8;
        gSprites[*data.at(2) >> 8].y += 8;
        gSprites[*data.at(2) as i32 & 0xFF].y += 8;
        gSprites[*data.at(3) >> 8].y += 8;
        gSprites[*data.at(3) as i32 & 0xFF].y += 8;
        gSprites[*data.at(4) >> 8].y += 8;
        gSprites[*data.at(4) as i32 & 0xFF].y += 8;
    }
}
pub(crate) unsafe extern "C" fn InitTakesFlightSceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(1, sBgTemplates_TakesFlight.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        0,
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
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
pub(crate) unsafe extern "C" fn LoadTakesFlightSceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        0,
        gRaySceneDuoFight_Clouds_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        1,
        gRaySceneTakesFlight_Bg_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        2,
        gRaySceneTakesFlight_Rayquaza_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        gRaySceneDuoFight_Clouds2_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneTakesFlight_Bg_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneTakesFlight_Rayquaza_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    LoadCompressedPalette(gRaySceneTakesFlight_Rayquaza_Pal.as_ptr().cast_mut(), 0, 64);
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_TakesFlight_Smoke).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_TakesFlight_Smoke).cast_mut());
}
pub(crate) unsafe extern "C" fn Task_RayTakesFlightAnim(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
    gTasks[taskId].func = Some(Task_HandleRayTakesFlight);
}
pub(crate) unsafe extern "C" fn Task_HandleRayTakesFlight(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn Task_RayTakesFlightEnd(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetVBlankCallback(None);
        ResetSpriteData();
        FreeAllSpritePalettes();
        gTasks[taskId].func = Some(Task_SetNextAnim);
    }
}
pub(crate) unsafe extern "C" fn Task_TakesFlight_CreateSmoke(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(1) as i32 & 3 == 0 {
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_TakesFlight_Smoke).cast_mut(),
            sTakesFlight_SmokeCoords[*data][0] as i16 * 4 + 120,
            sTakesFlight_SmokeCoords[*data][1] as i16 * 4 + 80,
            0,
        );
        gSprites[spriteId].data[0] = *data as i8 as i16;
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
        gSprites[spriteId].oam.set_priority(2);
        InitSpriteAffineAnim(&raw mut gSprites[spriteId]);
        if *data == 9 {
            DestroyTask(taskId);
            return;
        } else {
            *data += 1;
        }
    }
    *data.at(1) += 1;
}
pub(crate) unsafe extern "C" fn SpriteCB_TakesFlight_Smoke(sprite: *mut Sprite) {
    if (*sprite).data[1] == 0 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
    } else {
        (*sprite).x2 += sTakesFlight_SmokeCoords[(*sprite).data[0]][0] as i16;
        (*sprite).y2 += sTakesFlight_SmokeCoords[(*sprite).data[0]][1] as i16;
    }
    (*sprite).data[1] += 1;
    (*sprite).data[1] &= 0xF;
}
pub(crate) unsafe extern "C" fn InitDescendsSceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates_Descends.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(
        0,
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        3,
        (*sRayScene).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
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
pub(crate) unsafe extern "C" fn LoadDescendsSceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        0,
        gRaySceneDescends_Light_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        1,
        gRaySceneDescends_Bg_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        gRaySceneDescends_Light_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneDescends_Bg_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
    );
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
            0x1000200,
        );
    }
    CpuFastSet(
        (*sRayScene).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
        512,
    );
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            &raw mut (*sRayScene).tilemapBuffers[1][256] as *mut c_void,
            0x10000d0,
        );
    }
    LoadCompressedPalette(gRaySceneDescends_Bg_Pal.as_ptr().cast_mut(), 0, 64);
    gPlttBufferUnfaded[0] = 32767;
    gPlttBufferFaded[0] = 32767;
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Descends_Rayquaza).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Descends_RayquazaTail).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePal_Descends_Rayquaza).cast_mut());
}
pub(crate) unsafe extern "C" fn HBlankCB_RayDescends() {
    let mut vcount: u16 = GetGpuReg(REG_OFFSET_VCOUNT);
    if vcount >= 24 && vcount <= 135 && vcount as i32 - 24 <= (*sRayScene).revealedLightLine as i32
    {
        volatile_write(67108946 as usize as *mut u16, 0xD08);
    } else {
        volatile_write(67108946 as usize as *mut u16, 0x1000);
    }
    if vcount == 0 {
        if (*sRayScene).revealedLightLine <= 0x1FFF {
            if (*sRayScene).revealedLightLine <= 39 {
                (*sRayScene).revealedLightLine += 4;
            } else if (*sRayScene).revealedLightLine <= 79 {
                (*sRayScene).revealedLightLine += 2;
            } else {
                (*sRayScene).revealedLightLine += 1;
            }
        }
        (*sRayScene).revealedLightTimer += 1;
    }
}
pub(crate) unsafe extern "C" fn Task_RayDescendsAnim(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    InitDescendsSceneBgs();
    LoadDescendsSceneGfx();
    SetGpuRegBits(REG_OFFSET_BLDCNT, 7745);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    BlendPalettes(PALETTES_ALL, 0x10, 0);
    SetVBlankCallback(Some(VBlankCB_RayquazaScene));
    (*sRayScene).revealedLightLine = 0;
    (*sRayScene).revealedLightTimer = 0;
    *data = 0;
    *data.at(1) = 0;
    *data.at(2) = 0;
    *data.at(3) = 0;
    *data.at(4) = 0x1000;
    gTasks[taskId].func = Some(Task_HandleRayDescends);
}
pub(crate) unsafe extern "C" fn Task_HandleRayDescends(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
            if gPaletteFade.active() == 0 {
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
            gTasks[taskId].func = Some(Task_RayDescendsEnd);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_RayDescendsEnd(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetVBlankCallback(None);
        SetHBlankCallback(None);
        ResetSpriteData();
        FreeAllSpritePalettes();
        gTasks[taskId].func = Some(Task_SetNextAnim);
    }
}
pub(crate) unsafe extern "C" fn CreateDescendsRayquazaSprite() -> u8 {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_Descends_Rayquaza).cast_mut(),
        160,
        0,
        0,
    );
    let mut data: *mut i16 = gSprites[spriteId].data.as_mut_ptr();
    *data = CreateSprite(
        (&raw const *sSpriteTemplate_Descends_RayquazaTail).cast_mut(),
        184,
        -48,
        0,
    ) as i16;
    gSprites[spriteId].callback = Some(SpriteCB_Descends_Rayquaza);
    gSprites[spriteId].oam.set_priority(3);
    gSprites[*data].oam.set_priority(3);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_Descends_Rayquaza(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    let mut frame: i16 = *data.at(2);
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
        gSprites[*data].x2 -= 1;
    }
    if rem_i32(*data.at(2) as i32, *data.at(4) as i32) == 0 {
        (*sprite).y2 += 1;
        gSprites[*data].y2 += 1;
    }
    *data.at(2) += 1;
}
pub(crate) unsafe extern "C" fn InitChargesSceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates_Charges.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(
        0,
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        3,
        (*sRayScene).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
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
pub(crate) unsafe extern "C" fn LoadChargesSceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        1,
        gRaySceneCharges_Rayquaza_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        2,
        gRaySceneCharges_Streaks_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        3,
        gRaySceneCharges_Bg_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        gRaySceneCharges_Orbs_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneCharges_Rayquaza_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneCharges_Streaks_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneCharges_Bg_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
    );
    LoadCompressedPalette(gRaySceneCharges_Bg_Pal.as_ptr().cast_mut(), 0, 128);
}
pub(crate) unsafe extern "C" fn Task_RayChargesAnim(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    InitChargesSceneBgs();
    LoadChargesSceneGfx();
    SetWindowsHideVertBorders();
    BlendPalettes(PALETTES_ALL, 0x10, 0);
    SetVBlankCallback(Some(VBlankCB_RayquazaScene));
    *data = 0;
    *data.at(1) = 0;
    *data.at(2) = CreateTask(Some(Task_RayCharges_ShakeRayquaza), 0) as i16;
    gTasks[taskId].func = Some(Task_HandleRayCharges);
}
pub(crate) unsafe extern "C" fn Task_HandleRayCharges(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
                gTasks[*data.at(2)].func = Some(Task_RayCharges_FlyOffscreen);
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
            gTasks[taskId].func = Some(Task_RayChargesEnd);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_RayCharges_ShakeRayquaza(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(15) as i32 & 3 == 0 {
        ChangeBgX(1, Random() as i32 % 8 - 4 << 8, BG_COORD_SET);
        ChangeBgY(1, Random() as i32 % 8 - 4 << 8, BG_COORD_SET);
    }
    *data.at(15) += 1;
}
pub(crate) unsafe extern "C" fn Task_RayCharges_FlyOffscreen(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn RayCharges_AnimateBg() {
    ChangeBgX(2, 0x400, BG_COORD_SUB);
    ChangeBgY(2, 0x400, BG_COORD_ADD);
    ChangeBgX(0, 0x800, BG_COORD_SUB);
    ChangeBgY(0, 0x800, BG_COORD_ADD);
}
pub(crate) unsafe extern "C" fn Task_RayChargesEnd(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    RayCharges_AnimateBg();
    if gPaletteFade.active() == 0 {
        SetVBlankCallback(None);
        ResetWindowDimensions();
        DestroyTask(*data.at(2) as u8);
        gTasks[taskId].func = Some(Task_SetNextAnim);
    }
}
pub(crate) unsafe extern "C" fn InitChasesAwaySceneBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(1, sBgTemplates_ChasesAway.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        0,
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
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
pub(crate) unsafe extern "C" fn LoadChasesAwaySceneGfx() {
    ResetTempTileDataBuffers();
    DecompressAndCopyTileDataToVram(
        2,
        gRaySceneChasesAway_Ring_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    DecompressAndCopyTileDataToVram(
        0,
        gRaySceneChasesAway_Light_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    while FreeTempTileDataBuffersIfPossible() != 0 {}
    LZDecompressWram(
        gRaySceneChasesAway_Bg_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneChasesAway_Light_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LZDecompressWram(
        gRaySceneChasesAway_Ring_Tilemap.as_ptr().cast_mut(),
        (*sRayScene).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    LoadCompressedPalette(gRaySceneChasesAway_Bg_Pal.as_ptr().cast_mut(), 0, 96);
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
pub(crate) unsafe extern "C" fn Task_RayChasesAwayAnim(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
    gTasks[taskId].func = Some(Task_HandleRayChasesAway);
    *data.at(2) = CreateTask(Some(Task_ChasesAway_AnimateBg), 0) as i16;
    gTasks[*data.at(2)].data[0] = 0;
    gTasks[*data.at(2)].data[1] = 0;
    gTasks[*data.at(2)].data[2] = 0;
    gTasks[*data.at(2)].data[3] = 1;
    gTasks[*data.at(2)].data[4] = 1;
}
pub(crate) unsafe extern "C" fn Task_HandleRayChasesAway(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
            if gSprites[*data.at(5)].callback
                == Some(SpriteCB_ChasesAway_RayquazaFloat as unsafe extern "C" fn(*mut Sprite))
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
            gTasks[taskId].func = Some(Task_RayChasesAwayEnd);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ChasesAway_AnimateBg(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data as i32 & 0xF == 0 {
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            *data.at(1) as u16 + 14 << 8 & 0x1F00 | *data.at(2) as u16 + 9 & 0xF,
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
pub(crate) unsafe extern "C" fn Task_RayChasesAwayEnd(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
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
            gTasks[taskId].func = Some(Task_SetNextAnim);
        } else {
            *data.at(1) += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn ChasesAway_CreateTrioSprites(taskId: u8) {
    let mut taskData: *mut i16 = null_mut();
    let mut spriteData: *mut i16 = null_mut();
    taskData = gTasks[taskId].data.as_mut_ptr();
    *taskData.at(3) = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_Groudon).cast_mut(),
        64,
        120,
        0,
    ) as i16;
    spriteData = gSprites[*taskData.at(3)].data.as_mut_ptr();
    *spriteData = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_GroudonTail).cast_mut(),
        16,
        130,
        0,
    ) as i16;
    gSprites[*taskData.at(3)].oam.set_priority(1);
    gSprites[*spriteData].oam.set_priority(1);
    *taskData.at(4) = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_Kyogre).cast_mut(),
        160,
        128,
        1,
    ) as i16;
    spriteData = gSprites[*taskData.at(4)].data.as_mut_ptr();
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
    gSprites[*taskData.at(4)].oam.set_priority(1);
    gSprites[*spriteData].oam.set_priority(1);
    gSprites[*spriteData.at(1)].oam.set_priority(1);
    StartSpriteAnim(&raw mut gSprites[*spriteData], 1);
    StartSpriteAnim(&raw mut gSprites[*spriteData.at(1)], 2);
    *taskData.at(5) = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_Rayquaza).cast_mut(),
        120,
        -65,
        0,
    ) as i16;
    spriteData = gSprites[*taskData.at(5)].data.as_mut_ptr();
    *spriteData = CreateSprite(
        (&raw const *sSpriteTemplate_ChasesAway_RayquazaTail).cast_mut(),
        120,
        -113,
        0,
    ) as i16;
    gSprites[*taskData.at(5)].oam.set_priority(1);
    gSprites[*spriteData].oam.set_priority(1);
}
pub(crate) unsafe extern "C" fn ChasesAway_PushDuoBack(taskId: u8) {
    let mut taskData: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    gSprites[*taskData.at(3)].callback = Some(SpriteCB_ChasesAway_DuoRingPush);
    gSprites[*taskData.at(3)].data[4] = 0;
    gSprites[*taskData.at(3)].data[5] = 0;
    gSprites[*taskData.at(3)].data[6] = 4;
    gSprites[*taskData.at(3)].data[7] = FALSE as i16;
    gSprites[*taskData.at(4)].callback = Some(SpriteCB_ChasesAway_DuoRingPush);
    gSprites[*taskData.at(4)].data[4] = 0;
    gSprites[*taskData.at(4)].data[5] = 0;
    gSprites[*taskData.at(4)].data[6] = 4;
    gSprites[*taskData.at(4)].data[7] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_DuoRingPush(sprite: *mut Sprite) {
    if (*sprite).data[4] as i32 & 7 == 0 {
        if (*sprite).data[7] == 0 {
            (*sprite).x -= (*sprite).data[6];
            gSprites[(*sprite).data[0]].x -= (*sprite).data[6];
        } else {
            (*sprite).x += (*sprite).data[6];
            gSprites[(*sprite).data[0]].x += (*sprite).data[6];
            gSprites[(*sprite).data[1]].x += (*sprite).data[6];
        }
        (*sprite).data[5] += 1;
        (*sprite).data[6] -= (*sprite).data[5];
        if (*sprite).data[5] == 3 {
            (*sprite).data[4] = 0;
            (*sprite).data[5] = 0;
            (*sprite).data[6] = 0;
            (*sprite).callback = Some(SpriteCallbackDummy);
            return;
        }
    }
    (*sprite).data[4] += 1;
}
pub(crate) unsafe extern "C" fn ChasesAway_GroudonStartLeave(taskId: u8) {
    let mut taskData: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    gSprites[*taskData.at(3)].callback = Some(SpriteCB_ChasesAway_GroudonLeave);
    StartSpriteAnim(&raw mut gSprites[*taskData.at(3)], 1);
}
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_GroudonLeave(sprite: *mut Sprite) {
    match (*sprite).animCmdIndex {
        0 | 2 => {
            if (*sprite).animDelayCounter() as i32 % 12 == 0 {
                (*sprite).x -= 2;
                gSprites[(*sprite).data[0]].x -= 2;
            }
            gSprites[(*sprite).data[0]].y2 = 0;
        }
        1 | 3 => {
            gSprites[(*sprite).data[0]].y2 = -2;
            if (*sprite).animDelayCounter() as i32 & 15 == 0 {
                (*sprite).y += 1;
                gSprites[(*sprite).data[0]].y += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ChasesAway_KyogreStartLeave(taskId: u8) {
    let mut taskData: *mut i16 = null_mut();
    let mut spriteData: *mut i16 = null_mut();
    taskData = gTasks[taskId].data.as_mut_ptr();
    spriteData = gSprites[*taskData.at(4)].data.as_mut_ptr();
    gSprites[*taskData.at(4)].callback = Some(SpriteCB_ChasesAway_KyogreLeave);
    gSprites[*spriteData].callback = Some(SpriteCB_ChasesAway_KyogreLeave);
    gSprites[*spriteData.at(1)].callback = Some(SpriteCB_ChasesAway_KyogreLeave);
}
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_KyogreLeave(sprite: *mut Sprite) {
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
        gSprites[(*sprite).data[7]].oam.set_priority(1);
        (*sprite).data[7] = CreateSprite(
            (&raw const *sSpriteTemplate_ChasesAway_KyogreSplash).cast_mut(),
            224,
            132,
            0,
        ) as i16;
        gSprites[(*sprite).data[7]].oam.set_priority(1);
        gSprites[(*sprite).data[7]].set_hFlip(1);
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
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_Rayquaza(sprite: *mut Sprite) {
    let mut frame: i16 = (*sprite).data[7];
    if frame <= 64 {
        (*sprite).y2 += 2;
        gSprites[(*sprite).data[0]].y2 += 2;
        if (*sprite).data[7] == 64 {
            ChasesAway_SetRayquazaAnim(sprite, 1, 0, -48);
            (*sprite).data[4] = 5;
            (*sprite).data[5] = -1;
            gSprites[(*sprite).data[0]].data[4] = 3;
            gSprites[(*sprite).data[0]].data[5] = 5;
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
        gSprites[(*sprite).data[0]].data[4] = 7;
        gSprites[(*sprite).data[0]].data[5] = 3;
        SpriteCB_ChasesAway_RayquazaFloat(sprite);
    } else if frame <= 327 {
        SpriteCB_ChasesAway_RayquazaFloat(sprite);
    } else if frame == 328 {
        SpriteCB_ChasesAway_RayquazaFloat(sprite);
        ChasesAway_SetRayquazaAnim(sprite, 3, 48, 16);
        (*sprite).x2 = 1;
        gSprites[(*sprite).data[0]].x2 = 1;
        PlayCry_Normal(SPECIES_RAYQUAZA as u16, 0);
        CreateTask(Some(Task_ChasesAway_AnimateRing), 0);
    } else {
        match frame {
            376 => {
                (*sprite).x2 = 0;
                gSprites[(*sprite).data[0]].x2 = 0;
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
        gSprites[(*sprite).data[0]].x2 = (*sprite).x2;
    }
    (*sprite).data[7] += 1;
}
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_RayquazaFloat(body: *mut Sprite) {
    let mut tail: *mut Sprite = &raw mut gSprites[(*body).data[0]];
    if (*body).data[6] as i32 & (*tail).data[4] as i32 == 0 {
        (*body).y2 += (*body).data[4];
        gSprites[(*body).data[0]].y2 += (*body).data[4];
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
    (*body).data[6] += 1;
}
pub(crate) unsafe extern "C" fn ChasesAway_SetRayquazaAnim(
    body: *mut Sprite,
    animNum: u8,
    x: i16,
    y: i16,
) {
    let mut tail: *mut Sprite = &raw mut gSprites[(*body).data[0]];
    (*tail).x = (*body).x + x;
    (*tail).y = (*body).y + y;
    (*tail).x2 = (*body).x2;
    (*tail).y2 = (*body).y2;
    StartSpriteAnim(body, animNum);
    StartSpriteAnim(tail, animNum);
}
pub(crate) unsafe extern "C" fn Task_ChasesAway_AnimateRing(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
