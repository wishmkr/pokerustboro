//! Translated from `src/battle_transition.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sBigPokeball_Tileset sPokeballTrail_Tileset sPokeball_Gfx sEliteFour_Tileset sUnusedBrendan_Gfx sUnusedLass_Gfx sShrinkingBoxTileset sEvilTeam_Palette sTeamAqua_Tileset sTeamAqua_Tilemap sTeamMagma_Tileset sTeamMagma_Tilemap sRegis_Tileset sRegice_Palette sRegisteel_Palette sRegirock_Palette sRegice_Tilemap sRegisteel_Tilemap sRegirock_Tilemap sUnused_Palette sKyogre_Tileset sKyogre_Tilemap sGroudon_Tileset sGroudon_Tilemap sKyogre1_Palette sKyogre2_Palette sGroudon1_Palette sGroudon2_Palette sRayquaza_Palette sRayquaza_Tileset sRayquaza_Tilemap sFrontierLogo_Palette sFrontierLogo_Tileset sFrontierLogo_Tilemap sFrontierSquares_Palette sFrontierSquares_FilledBg_Tileset sFrontierSquares_EmptyBg_Tileset sFrontierSquares_Shrink1_Tileset sFrontierSquares_Shrink2_Tileset sFrontierSquares_Tilemap sTasks_Intro sTasks_Main sTaskHandlers sBlur_Funcs sSwirl_Funcs sShuffle_Funcs sAqua_Funcs sMagma_Funcs sBigPokeball_Funcs sRegice_Funcs sRegisteel_Funcs sRegirock_Funcs sKyogre_Funcs sPokeballsTrail_Funcs sPokeballsTrail_StartXCoords sPokeballsTrail_Delays sPokeballsTrail_Speeds sClockwiseWipe_Funcs sRipple_Funcs sWave_Funcs sMugshot_Funcs sMugshotsTrainerPicIDsTable sMugshotsOpponentRotationScales sMugshotsOpponentCoords sMugshotTrainerPicFuncs sTrainerPicSlideSpeeds sTrainerPicSlideAccels sSlice_Funcs sShredSplit_Funcs sShredSplit_SectionYCoords sShredSplit_SectionMoveDirs sBlackhole_Funcs sBlackholePulsate_Funcs sBlackhole_Vibrations sRectangularSpiral_Funcs sRectangularSpiral_Major_InwardRight sRectangularSpiral_Major_InwardLeft sRectangularSpiral_Major_InwardUp sRectangularSpiral_Major_InwardDown sRectangularSpiral_Minor_InwardRight sRectangularSpiral_Minor_InwardLeft sRectangularSpiral_Minor_InwardUp sRectangularSpiral_Minor_InwardDown sRectangularSpiral_Minor_OutwardRight sRectangularSpiral_Minor_OutwardLeft sRectangularSpiral_Minor_OutwardUp sRectangularSpiral_Minor_OutwardDown sRectangularSpiral_Major_OutwardRight sRectangularSpiral_Major_OutwardLeft sRectangularSpiral_Major_OutwardUp sRectangularSpiral_Major_OutwardDown sRectangularSpiral_MoveDataTable_MajorDiagonal sRectangularSpiral_MoveDataTable_MinorDiagonal sRectangularSpiral_MoveDataTables sGroudon_Funcs sRayquaza_Funcs sWhiteBarsFade_Funcs sWhiteBarsFade_StartDelays sGridSquares_Funcs sAngledWipes_Funcs sAngledWipes_MoveData sAngledWipes_EndDelays sTransitionIntroFuncs sSpriteImage_Pokeball sSpriteAnim_Pokeball sSpriteAnimTable_Pokeball sSpriteAffineAnim_Pokeball1 sSpriteAffineAnim_Pokeball2 sSpriteAffineAnimTable_Pokeball sSpriteTemplate_Pokeball sOam_UnusedBrendanLass sImageTable_UnusedBrendan sImageTable_UnusedLass sSpriteAnim_UnusedBrendanLass sSpriteAnimTable_UnusedBrendanLass sSpriteTemplate_UnusedBrendan sSpriteTemplate_UnusedLass sFieldEffectPal_Pokeball gSpritePalette_Pokeball sMugshotPal_Sidney sMugshotPal_Phoebe sMugshotPal_Glacia sMugshotPal_Drake sMugshotPal_Champion sMugshotPal_Brendan sMugshotPal_May sOpponentMugshotsPals sPlayerMugshotsPals sUnusedTrainerPalette sSpritePalette_UnusedTrainer sBigPokeball_Tilemap sMugshotsTilemap sFrontierLogoWiggle_Funcs sFrontierLogoWave_Funcs sFrontierSquares_Funcs sFrontierSquaresSpiral_Funcs sFrontierSquaresScroll_Funcs sFrontierSquaresSpiral_Positions sFrontierSquaresScroll_Positions

/// `struct RectangularSpiralLine`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct RectangularSpiralLine {
    pub state: u8,
    pub position: i16,
    pub moveIndex: u8,
    pub reboundPosition: i16,
    pub outward: u8,
}

unsafe impl Sync for RectangularSpiralLine {}

/// `struct TransitionData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct TransitionData {
    pub VBlank_DMA: u8,
    pub WININ: u16,
    pub WINOUT: u16,
    pub WIN0H: u16,
    pub WIN0V: u16,
    pub unused1: u16,
    pub unused2: u16,
    pub BLDCNT: u16,
    pub BLDALPHA: u16,
    pub BLDY: u16,
    pub cameraX: i16,
    pub cameraY: i16,
    pub BG0HOFS_Lower: i16,
    pub BG0HOFS_Upper: i16,
    pub BG0VOFS: i16,
    pub unused3: i16,
    pub counter: i16,
    pub unused4: i16,
    pub data: CArray<i16, 11>,
}

unsafe impl Sync for TransitionData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<RectangularSpiralLine>() == 12);
    assert!(offset_of!(RectangularSpiralLine, state) == 0);
    assert!(offset_of!(RectangularSpiralLine, position) == 2);
    assert!(offset_of!(RectangularSpiralLine, moveIndex) == 4);
    assert!(offset_of!(RectangularSpiralLine, reboundPosition) == 6);
    assert!(offset_of!(RectangularSpiralLine, outward) == 8);
    assert!(size_of::<TransitionData>() == 60);
    assert!(offset_of!(TransitionData, VBlank_DMA) == 0);
    assert!(offset_of!(TransitionData, WININ) == 2);
    assert!(offset_of!(TransitionData, WINOUT) == 4);
    assert!(offset_of!(TransitionData, WIN0H) == 6);
    assert!(offset_of!(TransitionData, WIN0V) == 8);
    assert!(offset_of!(TransitionData, unused1) == 10);
    assert!(offset_of!(TransitionData, unused2) == 12);
    assert!(offset_of!(TransitionData, BLDCNT) == 14);
    assert!(offset_of!(TransitionData, BLDALPHA) == 16);
    assert!(offset_of!(TransitionData, BLDY) == 18);
    assert!(offset_of!(TransitionData, cameraX) == 20);
    assert!(offset_of!(TransitionData, cameraY) == 22);
    assert!(offset_of!(TransitionData, BG0HOFS_Lower) == 24);
    assert!(offset_of!(TransitionData, BG0HOFS_Upper) == 26);
    assert!(offset_of!(TransitionData, BG0VOFS) == 28);
    assert!(offset_of!(TransitionData, unused3) == 30);
    assert!(offset_of!(TransitionData, counter) == 32);
    assert!(offset_of!(TransitionData, unused4) == 34);
    assert!(offset_of!(TransitionData, data) == 36);
};

const B_TRANS_DMA_FLAGS: u32 = 0xa2400001;
const FADE_TARGET: i16 = 4096;
const MARGIN_SIZE: u8 = 1;
const MOVE_DOWN: i16 = 4;
const MOVE_LEFT: i16 = 2;
const MOVE_RIGHT: i16 = 1;
const MOVE_UP: i16 = 3;
const NUM_ANGLED_WIPES: i16 = 7;
const NUM_POKEBALL_TRAILS: i16 = 5;
const NUM_SQUARES: i16 = 35;
const NUM_SQUARES_PER_COL: i32 = 5;
const NUM_SQUARES_PER_ROW: i16 = 7;
const NUM_WHITE_BARS: i16 = 8;
const SPIRAL_END: i16 = -1;
const SPIRAL_INWARD_END: u8 = 3;
const SPIRAL_INWARD_START: u8 = 0;
const SPIRAL_OUTWARD_END: u8 = 7;
const SPIRAL_OUTWARD_START: u8 = 4;
const SPIRAL_REBOUND: i16 = -2;
const SQUARE_SIZE: u8 = 4;

static sAngledWipes_EndDelays: Table<CArray<i16, 7>> =
    Table((&raw const crate::data::battle_transition::sAngledWipes_EndDelays).cast());
static sAngledWipes_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>> =
    Table((&raw const crate::data::battle_transition::sAngledWipes_Funcs).cast());
static sAngledWipes_MoveData: Table<CArray<CArray<i16, 5>, 7>> =
    Table((&raw const crate::data::battle_transition::sAngledWipes_MoveData).cast());
static sAqua_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 7>> =
    Table((&raw const crate::data::battle_transition::sAqua_Funcs).cast());
static sBigPokeball_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 6>> =
    Table((&raw const crate::data::battle_transition::sBigPokeball_Funcs).cast());
static sBigPokeball_Tilemap: Table<CArray<u16, 600>> =
    Table((&raw const crate::data::battle_transition::sBigPokeball_Tilemap).cast());
static sBigPokeball_Tileset: Table<CArray<u32, 352>> =
    Table((&raw const crate::data::battle_transition::sBigPokeball_Tileset).cast());
static sBlackholePulsate_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 2>> =
    Table((&raw const crate::data::battle_transition::sBlackholePulsate_Funcs).cast());
static sBlackhole_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::battle_transition::sBlackhole_Funcs).cast());
static sBlackhole_Vibrations: Table<CArray<i16, 2>> =
    Table((&raw const crate::data::battle_transition::sBlackhole_Vibrations).cast());
static sBlur_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::battle_transition::sBlur_Funcs).cast());
static sClockwiseWipe_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 7>> =
    Table((&raw const crate::data::battle_transition::sClockwiseWipe_Funcs).cast());
static sEliteFour_Tileset: Table<CArray<u32, 120>> =
    Table((&raw const crate::data::battle_transition::sEliteFour_Tileset).cast());
static sEvilTeam_Palette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_transition::sEvilTeam_Palette).cast());
static sFieldEffectPal_Pokeball: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_transition::sFieldEffectPal_Pokeball).cast());
static sFrontierLogoWave_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 4>> =
    Table((&raw const crate::data::battle_transition::sFrontierLogoWave_Funcs).cast());
static sFrontierLogoWiggle_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 6>> =
    Table((&raw const crate::data::battle_transition::sFrontierLogoWiggle_Funcs).cast());
static sFrontierLogo_Palette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_transition::sFrontierLogo_Palette).cast());
static sFrontierLogo_Tilemap: Table<CArray<u32, 187>> =
    Table((&raw const crate::data::battle_transition::sFrontierLogo_Tilemap).cast());
static sFrontierLogo_Tileset: Table<CArray<u32, 419>> =
    Table((&raw const crate::data::battle_transition::sFrontierLogo_Tileset).cast());
static sFrontierSquaresScroll_Funcs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>,
> = Table((&raw const crate::data::battle_transition::sFrontierSquaresScroll_Funcs).cast());
static sFrontierSquaresScroll_Positions: Table<CArray<u8, 64>> =
    Table((&raw const crate::data::battle_transition::sFrontierSquaresScroll_Positions).cast());
static sFrontierSquaresSpiral_Funcs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>,
> = Table((&raw const crate::data::battle_transition::sFrontierSquaresSpiral_Funcs).cast());
static sFrontierSquaresSpiral_Positions: Table<CArray<u8, 35>> =
    Table((&raw const crate::data::battle_transition::sFrontierSquaresSpiral_Positions).cast());
static sFrontierSquares_EmptyBg_Tileset: Table<CArray<u32, 64>> =
    Table((&raw const crate::data::battle_transition::sFrontierSquares_EmptyBg_Tileset).cast());
static sFrontierSquares_FilledBg_Tileset: Table<CArray<u32, 87>> =
    Table((&raw const crate::data::battle_transition::sFrontierSquares_FilledBg_Tileset).cast());
static sFrontierSquares_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 4>> =
    Table((&raw const crate::data::battle_transition::sFrontierSquares_Funcs).cast());
static sFrontierSquares_Palette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_transition::sFrontierSquares_Palette).cast());
static sFrontierSquares_Shrink1_Tileset: Table<CArray<u32, 44>> =
    Table((&raw const crate::data::battle_transition::sFrontierSquares_Shrink1_Tileset).cast());
static sFrontierSquares_Shrink2_Tileset: Table<CArray<u32, 33>> =
    Table((&raw const crate::data::battle_transition::sFrontierSquares_Shrink2_Tileset).cast());
static sFrontierSquares_Tilemap: Table<CArray<u32, 8>> =
    Table((&raw const crate::data::battle_transition::sFrontierSquares_Tilemap).cast());
static sGridSquares_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::battle_transition::sGridSquares_Funcs).cast());
static sGroudon1_Palette: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::battle_transition::sGroudon1_Palette).cast());
static sGroudon2_Palette: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::battle_transition::sGroudon2_Palette).cast());
static sGroudon_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 8>> =
    Table((&raw const crate::data::battle_transition::sGroudon_Funcs).cast());
static sGroudon_Tilemap: Table<CArray<u32, 198>> =
    Table((&raw const crate::data::battle_transition::sGroudon_Tilemap).cast());
static sGroudon_Tileset: Table<CArray<u32, 347>> =
    Table((&raw const crate::data::battle_transition::sGroudon_Tileset).cast());
static sKyogre1_Palette: Table<CArray<u16, 160>> =
    Table((&raw const crate::data::battle_transition::sKyogre1_Palette).cast());
static sKyogre2_Palette: Table<CArray<u16, 224>> =
    Table((&raw const crate::data::battle_transition::sKyogre2_Palette).cast());
static sKyogre_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 8>> =
    Table((&raw const crate::data::battle_transition::sKyogre_Funcs).cast());
static sKyogre_Tilemap: Table<CArray<u32, 205>> =
    Table((&raw const crate::data::battle_transition::sKyogre_Tilemap).cast());
static sKyogre_Tileset: Table<CArray<u32, 429>> =
    Table((&raw const crate::data::battle_transition::sKyogre_Tileset).cast());
static sMagma_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 7>> =
    Table((&raw const crate::data::battle_transition::sMagma_Funcs).cast());
static sMugshotTrainerPicFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Sprite) -> u8>, 7>> =
    Table((&raw const crate::data::battle_transition::sMugshotTrainerPicFuncs).cast());
static sMugshot_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 10>> =
    Table((&raw const crate::data::battle_transition::sMugshot_Funcs).cast());
static sMugshotsOpponentCoords: Table<CArray<CArray<i16, 2>, 5>> =
    Table((&raw const crate::data::battle_transition::sMugshotsOpponentCoords).cast());
static sMugshotsOpponentRotationScales: Table<CArray<CArray<i16, 2>, 5>> =
    Table((&raw const crate::data::battle_transition::sMugshotsOpponentRotationScales).cast());
static sMugshotsTilemap: Table<CArray<u16, 640>> =
    Table((&raw const crate::data::battle_transition::sMugshotsTilemap).cast());
static sMugshotsTrainerPicIDsTable: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::battle_transition::sMugshotsTrainerPicIDsTable).cast());
static sOpponentMugshotsPals: Table<CArray<*mut u16, 5>> =
    Table((&raw const crate::data::battle_transition::sOpponentMugshotsPals).cast());
static sPlayerMugshotsPals: Table<CArray<*mut u16, 2>> =
    Table((&raw const crate::data::battle_transition::sPlayerMugshotsPals).cast());
static sPokeballTrail_Tileset: Table<CArray<u32, 16>> =
    Table((&raw const crate::data::battle_transition::sPokeballTrail_Tileset).cast());
static sPokeballsTrail_Delays: Table<CArray<i16, 5>> =
    Table((&raw const crate::data::battle_transition::sPokeballsTrail_Delays).cast());
static sPokeballsTrail_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::battle_transition::sPokeballsTrail_Funcs).cast());
static sPokeballsTrail_Speeds: Table<CArray<i16, 2>> =
    Table((&raw const crate::data::battle_transition::sPokeballsTrail_Speeds).cast());
static sPokeballsTrail_StartXCoords: Table<CArray<i16, 2>> =
    Table((&raw const crate::data::battle_transition::sPokeballsTrail_StartXCoords).cast());
static sRayquaza_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 11>> =
    Table((&raw const crate::data::battle_transition::sRayquaza_Funcs).cast());
static sRayquaza_Palette: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::battle_transition::sRayquaza_Palette).cast());
static sRayquaza_Tilemap: Table<CArray<u32, 1024>> =
    Table((&raw const crate::data::battle_transition::sRayquaza_Tilemap).cast());
static sRayquaza_Tileset: Table<CArray<u32, 7504>> =
    Table((&raw const crate::data::battle_transition::sRayquaza_Tileset).cast());
static sRectangularSpiral_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::battle_transition::sRectangularSpiral_Funcs).cast());
static sRectangularSpiral_MoveDataTables: Table<CArray<*mut *mut i16, 2>> =
    Table((&raw const crate::data::battle_transition::sRectangularSpiral_MoveDataTables).cast());
static sRegice_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 6>> =
    Table((&raw const crate::data::battle_transition::sRegice_Funcs).cast());
static sRegice_Palette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_transition::sRegice_Palette).cast());
static sRegice_Tilemap: Table<CArray<u32, 512>> =
    Table((&raw const crate::data::battle_transition::sRegice_Tilemap).cast());
static sRegirock_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 6>> =
    Table((&raw const crate::data::battle_transition::sRegirock_Funcs).cast());
static sRegirock_Palette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_transition::sRegirock_Palette).cast());
static sRegirock_Tilemap: Table<CArray<u32, 512>> =
    Table((&raw const crate::data::battle_transition::sRegirock_Tilemap).cast());
static sRegis_Tileset: Table<CArray<u32, 424>> =
    Table((&raw const crate::data::battle_transition::sRegis_Tileset).cast());
static sRegisteel_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 6>> =
    Table((&raw const crate::data::battle_transition::sRegisteel_Funcs).cast());
static sRegisteel_Palette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_transition::sRegisteel_Palette).cast());
static sRegisteel_Tilemap: Table<CArray<u32, 512>> =
    Table((&raw const crate::data::battle_transition::sRegisteel_Tilemap).cast());
static sRipple_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 2>> =
    Table((&raw const crate::data::battle_transition::sRipple_Funcs).cast());
static sShredSplit_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 4>> =
    Table((&raw const crate::data::battle_transition::sShredSplit_Funcs).cast());
static sShredSplit_SectionMoveDirs: Table<CArray<i16, 2>> =
    Table((&raw const crate::data::battle_transition::sShredSplit_SectionMoveDirs).cast());
static sShredSplit_SectionYCoords: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::battle_transition::sShredSplit_SectionYCoords).cast());
static sShrinkingBoxTileset: Table<CArray<u32, 120>> =
    Table((&raw const crate::data::battle_transition::sShrinkingBoxTileset).cast());
static sShuffle_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 2>> =
    Table((&raw const crate::data::battle_transition::sShuffle_Funcs).cast());
static sSlice_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::battle_transition::sSlice_Funcs).cast());
static sSpriteTemplate_Pokeball: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_transition::sSpriteTemplate_Pokeball).cast());
static sSwirl_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 2>> =
    Table((&raw const crate::data::battle_transition::sSwirl_Funcs).cast());
static sTaskHandlers: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 4>> =
    Table((&raw const crate::data::battle_transition::sTaskHandlers).cast());
static sTasks_Intro: Table<CArray<Option<unsafe extern "C" fn(u8)>, 42>> =
    Table((&raw const crate::data::battle_transition::sTasks_Intro).cast());
static sTasks_Main: Table<CArray<Option<unsafe extern "C" fn(u8)>, 42>> =
    Table((&raw const crate::data::battle_transition::sTasks_Main).cast());
static sTeamAqua_Tilemap: Table<CArray<u32, 151>> =
    Table((&raw const crate::data::battle_transition::sTeamAqua_Tilemap).cast());
static sTeamAqua_Tileset: Table<CArray<u32, 222>> =
    Table((&raw const crate::data::battle_transition::sTeamAqua_Tileset).cast());
static sTeamMagma_Tilemap: Table<CArray<u32, 185>> =
    Table((&raw const crate::data::battle_transition::sTeamMagma_Tilemap).cast());
static sTeamMagma_Tileset: Table<CArray<u32, 291>> =
    Table((&raw const crate::data::battle_transition::sTeamMagma_Tileset).cast());
static sTrainerPicSlideAccels: Table<CArray<i16, 2>> =
    Table((&raw const crate::data::battle_transition::sTrainerPicSlideAccels).cast());
static sTrainerPicSlideSpeeds: Table<CArray<i16, 2>> =
    Table((&raw const crate::data::battle_transition::sTrainerPicSlideSpeeds).cast());
static sTransitionIntroFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 2>> =
    Table((&raw const crate::data::battle_transition::sTransitionIntroFuncs).cast());
static sWave_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::battle_transition::sWave_Funcs).cast());
static sWhiteBarsFade_Funcs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>> =
    Table((&raw const crate::data::battle_transition::sWhiteBarsFade_Funcs).cast());
static sWhiteBarsFade_StartDelays: Table<CArray<i16, 8>> =
    Table((&raw const crate::data::battle_transition::sWhiteBarsFade_StartDelays).cast());

pub(crate) static mut sDebug_RectangularSpiralData: i16 = 0;
pub(crate) static mut sTestingTransitionId: u8 = 0;
pub(crate) static mut sTestingTransitionState: u8 = 0;
pub(crate) static mut sRectangularSpiralLines: CArray<RectangularSpiralLine, 4> =
    unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTransitionData: *mut TransitionData = null_mut();

unsafe extern "C" {
    static mut gBattle_BG0_X: u16;
    static mut gBattle_BG0_Y: u16;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gFieldEffectArguments: CArray<i32, 8>;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn AllocOamMatrix() -> u8;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_OverworldBasic();
    fn CB2_ReturnToField();
    fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8);
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyRectToBgTilemapBufferRect(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u8,
        a8: u8,
        a9: u8,
        a10: u8,
        a11: i16,
        a12: i16,
    );
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateInvisibleSprite(a0: Option<unsafe extern "C" fn(*mut Sprite)>) -> u8;
    fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTrainerSprite(a0: u8, a1: i16, a2: i16, a3: u8, a4: *mut u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn EnableInterrupts(a0: u16);
    fn FieldEffectActiveListContains(a0: u8) -> u8;
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldEffectStop(a0: *mut Sprite, a1: u8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut c_void);
    fn GetCameraOffsetWithPan(a0: *mut i16, a1: *mut i16);
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn PlayerGenderToFrontTrainerPicId(a0: u8) -> u16;
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn RunTasks();
    fn ScanlineEffect_Clear();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetOamMatrixRotationScaling(a0: u8, a1: i16, a2: i16, a3: u16);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWeatherScreenFadeOut();
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn CB2_TestBattleTransition() {
    match sTestingTransitionState {
        0 => {
            LaunchBattleTransitionTask(sTestingTransitionId);
            sTestingTransitionState += 1;
        }
        1 => {
            if IsBattleTransitionDone() != 0 {
                sTestingTransitionState = 0;
                SetMainCallback2(Some(CB2_ReturnToField));
            }
        }
        _ => {}
    }
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn TestBattleTransition(transitionId: u8) {
    sTestingTransitionId = transitionId;
    SetMainCallback2(Some(CB2_TestBattleTransition));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTransition_StartOnField(transitionId: u8) {
    gMain.callback2 = Some(CB2_OverworldBasic);
    LaunchBattleTransitionTask(transitionId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTransition_Start(transitionId: u8) {
    LaunchBattleTransitionTask(transitionId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBattleTransitionDone() -> u8 {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_BattleTransition));
    if gTasks[taskId].data[15] != 0 {
        DestroyTask(taskId);
        Free(sTransitionData as *mut c_void);
        sTransitionData = null_mut();
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn LaunchBattleTransitionTask(transitionId: u8) {
    let mut taskId: u8 = CreateTask(Some(Task_BattleTransition), 2);
    gTasks[taskId].data[1] = transitionId as i16;
    sTransitionData = AllocZeroed(60) as *mut TransitionData;
}
pub(crate) unsafe extern "C" fn Task_BattleTransition(taskId: u8) {
    while sTaskHandlers[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Transition_StartIntro(task: *mut Task) -> u8 {
    SetWeatherScreenFadeOut();
    CpuSet(
        gPlttBufferFaded.as_mut_ptr() as *mut c_void,
        gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
        0x4000100,
    );
    if sTasks_Intro[(*task).data[1]].is_some() {
        CreateTask(sTasks_Intro[(*task).data[1]], 4);
        (*task).data[0] += 1;
        return FALSE;
    } else {
        (*task).data[0] = 2;
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Transition_WaitForIntro(task: *mut Task) -> u8 {
    if FindTaskIdByFunc(sTasks_Intro[(*task).data[1]]) == TASK_NONE {
        (*task).data[0] += 1;
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Transition_StartMain(task: *mut Task) -> u8 {
    CreateTask(sTasks_Main[(*task).data[1]], 0);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Transition_WaitForMain(task: *mut Task) -> u8 {
    (*task).data[15] = FALSE as i16;
    if FindTaskIdByFunc(sTasks_Main[(*task).data[1]]) == TASK_NONE {
        (*task).data[15] = TRUE as i16;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_Intro(taskId: u8) {
    if gTasks[taskId].data[0] == 0 {
        gTasks[taskId].data[0] += 1;
        CreateIntroTask(0, 0, 3, 2, 2);
    } else if IsIntroTaskDone() != 0 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_Blur(taskId: u8) {
    while sBlur_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Blur_Init(task: *mut Task) -> u8 {
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
    SetGpuRegBits(REG_OFFSET_BG1CNT, BGCNT_MOSAIC);
    SetGpuRegBits(REG_OFFSET_BG2CNT, BGCNT_MOSAIC);
    SetGpuRegBits(REG_OFFSET_BG3CNT, BGCNT_MOSAIC);
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn Blur_Main(task: *mut Task) -> u8 {
    if (*task).data[1] != 0 {
        (*task).data[1] -= 1;
    } else {
        (*task).data[1] = 4;
        if ({
            (*task).data[2] += 1;
            (*task).data[2]
        }) == 10
        {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
        }
        SetGpuReg(REG_OFFSET_MOSAIC, ((*task).data[2] as u16 & 15) * 17);
        if (*task).data[2] > 14 {
            (*task).data[0] += 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Blur_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_Blur));
        DestroyTask(taskId);
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_Swirl(taskId: u8) {
    while sSwirl_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Swirl_Init(task: *mut Task) -> u8 {
    InitTransitionData();
    ScanlineEffect_Clear();
    BeginNormalPaletteFade(PALETTES_ALL, 4, 0, 16, 0);
    SetSinWave(
        gScanlineEffectRegBuffers[1].as_mut_ptr() as *mut i16,
        (*sTransitionData).cameraX,
        0,
        2,
        0,
        DISPLAY_HEIGHT as i16,
    );
    SetVBlankCallback(Some(VBlankCB_Swirl));
    SetHBlankCallback(Some(HBlankCB_Swirl));
    EnableInterrupts(3);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Swirl_End(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    (*task).data[1] += 4;
    (*task).data[2] += 8;
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        (*sTransitionData).cameraX,
        (*task).data[1],
        2,
        (*task).data[2],
        DISPLAY_HEIGHT as i16,
    );
    if gPaletteFade.active() == 0 {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_Swirl));
        DestroyTask(taskId);
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_Swirl() {
    VBlankCB_BattleTransition();
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_Swirl() {
    let mut var: u16 =
        gScanlineEffectRegBuffers[1][(67108870 as usize as *mut u16).read_volatile()];
    volatile_write(67108884 as usize as *mut u16, var);
    volatile_write(67108888 as usize as *mut u16, var);
    volatile_write(67108892 as usize as *mut u16, var);
}
pub(crate) unsafe extern "C" fn Task_Shuffle(taskId: u8) {
    while sShuffle_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Shuffle_Init(task: *mut Task) -> u8 {
    InitTransitionData();
    ScanlineEffect_Clear();
    BeginNormalPaletteFade(PALETTES_ALL, 4, 0, 16, 0);
    memset(
        gScanlineEffectRegBuffers[1].as_mut_ptr() as *mut u8,
        (*sTransitionData).cameraY as i32,
        320,
    );
    SetVBlankCallback(Some(VBlankCB_Shuffle));
    SetHBlankCallback(Some(HBlankCB_Shuffle));
    EnableInterrupts(3);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Shuffle_End(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    let mut amplitude: u16 = 0;
    let mut sinVal: u16 = 0;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    sinVal = (*task).data[1] as u16;
    amplitude = ((*task).data[2] >> 8) as u16;
    (*task).data[1] += 4224;
    (*task).data[2] += 384;
    i = 0;
    while i < DISPLAY_HEIGHT as u8 {
        let mut sinIndex: u16 = (sinVal as i32 / 256) as u16;
        gScanlineEffectRegBuffers[0][i] =
            (*sTransitionData).cameraY as u16 + Sin(sinIndex as i16, amplitude as i16) as u16;
        i += 1;
        sinVal += 4224;
    }
    if gPaletteFade.active() == 0 {
        DestroyTask(FindTaskIdByFunc(Some(Task_Shuffle)));
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_Shuffle() {
    VBlankCB_BattleTransition();
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_Shuffle() {
    let mut var: u16 =
        gScanlineEffectRegBuffers[1][(67108870 as usize as *mut u16).read_volatile()];
    volatile_write(67108886 as usize as *mut u16, var);
    volatile_write(67108890 as usize as *mut u16, var);
    volatile_write(67108894 as usize as *mut u16, var);
}
pub(crate) unsafe extern "C" fn Task_BigPokeball(taskId: u8) {
    while sBigPokeball_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn Task_Aqua(taskId: u8) {
    while sAqua_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Task_Magma(taskId: u8) {
    while sMagma_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Task_Regice(taskId: u8) {
    while sRegice_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Task_Registeel(taskId: u8) {
    while sRegisteel_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0
    {
    }
}
pub(crate) unsafe extern "C" fn Task_Regirock(taskId: u8) {
    while sRegirock_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {
    }
}
pub(crate) unsafe extern "C" fn Task_Kyogre(taskId: u8) {
    while sKyogre_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn InitPatternWeaveTransition(task: *mut Task) {
    let mut i: i32 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    (*task).data[1] = 16;
    (*task).data[2] = 0;
    (*task).data[4] = 0;
    (*task).data[5] = 0x4000;
    (*sTransitionData).WININ = WININ_WIN0_ALL;
    (*sTransitionData).WINOUT = 0;
    (*sTransitionData).WIN0H = DISPLAY_WIDTH;
    (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
    (*sTransitionData).BLDCNT = 16193;
    (*sTransitionData).BLDALPHA = ((*task).data[1] as u16) << 8 | (*task).data[2] as u16;
    i = 0;
    while i < DISPLAY_HEIGHT as i32 {
        gScanlineEffectRegBuffers[1][i] = DISPLAY_WIDTH;
        i += 1;
    }
    SetVBlankCallback(Some(VBlankCB_PatternWeave));
}
pub(crate) unsafe extern "C" fn Aqua_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    (*task).data[8] = 60;
    InitPatternWeaveTransition(task);
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    LZ77UnCompVram(
        sTeamAqua_Tileset.as_ptr().cast_mut(),
        tileset as *mut c_void,
    );
    LoadPalette(
        sEvilTeam_Palette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Magma_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    (*task).data[8] = 60;
    InitPatternWeaveTransition(task);
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    LZ77UnCompVram(
        sTeamMagma_Tileset.as_ptr().cast_mut(),
        tileset as *mut c_void,
    );
    LoadPalette(
        sEvilTeam_Palette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Regi_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    (*task).data[8] = 60;
    InitPatternWeaveTransition(task);
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    CpuSet(
        sRegis_Tileset.as_ptr().cast_mut() as *mut c_void,
        tileset as *mut c_void,
        4096,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn BigPokeball_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    InitPatternWeaveTransition(task);
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    CpuSet(
        sBigPokeball_Tileset.as_ptr().cast_mut() as *mut c_void,
        tileset as *mut c_void,
        704,
    );
    LoadPalette(
        sFieldEffectPal_Pokeball.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn BigPokeball_SetGfx(task: *mut Task) -> u8 {
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    let mut bigPokeballMap: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    bigPokeballMap = sBigPokeball_Tilemap.as_ptr().cast_mut();
    i = 0;
    while i < 20 {
        j = 0;
        while j < 30 {
            let mut index: u32 = i as u32 * 32 + j as u32;
            *tilemap.at(index) = *bigPokeballMap | 61440;
            j += 1;
            bigPokeballMap = bigPokeballMap.at(1);
        }
        i += 1;
    }
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5],
        DISPLAY_HEIGHT as i16,
    );
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn Aqua_SetGfx(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LZ77UnCompVram(
        sTeamAqua_Tilemap.as_ptr().cast_mut(),
        tilemap as *mut c_void,
    );
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5],
        DISPLAY_HEIGHT as i16,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Magma_SetGfx(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LZ77UnCompVram(
        sTeamMagma_Tilemap.as_ptr().cast_mut(),
        tilemap as *mut c_void,
    );
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5],
        DISPLAY_HEIGHT as i16,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Regice_SetGfx(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LoadPalette(sRegice_Palette.as_ptr().cast_mut() as *mut c_void, 240, 32);
    CpuSet(
        sRegice_Tilemap.as_ptr().cast_mut() as *mut c_void,
        tilemap as *mut c_void,
        640,
    );
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5],
        DISPLAY_HEIGHT as i16,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Registeel_SetGfx(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LoadPalette(
        sRegisteel_Palette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    CpuSet(
        sRegisteel_Tilemap.as_ptr().cast_mut() as *mut c_void,
        tilemap as *mut c_void,
        640,
    );
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5],
        DISPLAY_HEIGHT as i16,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Regirock_SetGfx(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LoadPalette(
        sRegirock_Palette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    CpuSet(
        sRegirock_Tilemap.as_ptr().cast_mut() as *mut c_void,
        tilemap as *mut c_void,
        640,
    );
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5],
        DISPLAY_HEIGHT as i16,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Kyogre_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    LZ77UnCompVram(sKyogre_Tileset.as_ptr().cast_mut(), tileset as *mut c_void);
    LZ77UnCompVram(sKyogre_Tilemap.as_ptr().cast_mut(), tilemap as *mut c_void);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Kyogre_PaletteFlash(task: *mut Task) -> u8 {
    if (*task).data[1] % 3 == 0 {
        let mut offset: u16 = ((*task).data[1] % 30) as u16;
        offset = (offset as i32 / 3) as u16;
        LoadPalette(
            (&raw const sKyogre1_Palette[offset as i32 * 16]).cast_mut() as *mut c_void,
            240,
            32,
        );
    }
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 58
    {
        (*task).data[0] += 1;
        (*task).data[1] = 0;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Kyogre_PaletteBrighten(task: *mut Task) -> u8 {
    if (*task).data[1] % 5 == 0 {
        let mut offset: i16 = (*task).data[1] / 5;
        LoadPalette(
            (&raw const sKyogre2_Palette[offset as i32 * 16]).cast_mut() as *mut c_void,
            240,
            32,
        );
    }
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 68
    {
        (*task).data[0] += 1;
        (*task).data[1] = 0;
        (*task).data[8] = 30;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn WeatherDuo_FadeOut(task: *mut Task) -> u8 {
    BeginNormalPaletteFade(0xffff8000, 1, 0, 16, 0);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WeatherDuo_End(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        {
            let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
            let _ = (dmaRegs.at(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc((*task).func));
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PatternWeave_Blend1(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    if (*task).data[3] == 0
        || ({
            (*task).data[3] -= 1;
            (*task).data[3]
        }) == 0
    {
        (*task).data[2] += 1;
        (*task).data[3] = 2;
    }
    (*sTransitionData).BLDALPHA = ((*task).data[1] as u16) << 8 | (*task).data[2] as u16;
    if (*task).data[2] > 15 {
        (*task).data[0] += 1;
    }
    (*task).data[4] += 8;
    (*task).data[5] -= 256;
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5] >> 8,
        DISPLAY_HEIGHT as i16,
    );
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn PatternWeave_Blend2(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    if (*task).data[3] == 0
        || ({
            (*task).data[3] -= 1;
            (*task).data[3]
        }) == 0
    {
        (*task).data[1] -= 1;
        (*task).data[3] = 2;
    }
    (*sTransitionData).BLDALPHA = ((*task).data[1] as u16) << 8 | (*task).data[2] as u16;
    if (*task).data[1] == 0 {
        (*task).data[0] += 1;
    }
    (*task).data[4] += 8;
    (*task).data[5] -= 256;
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5] >> 8,
        DISPLAY_HEIGHT as i16,
    );
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn PatternWeave_FinishAppear(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    (*task).data[4] += 8;
    (*task).data[5] -= 256;
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5] >> 8,
        DISPLAY_HEIGHT as i16,
    );
    if (*task).data[5] <= 0 {
        (*task).data[0] += 1;
        (*task).data[1] = DISPLAY_HEIGHT as i16;
        (*task).data[2] = 256;
        (*task).data[3] = FALSE as i16;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn FramesCountdown(task: *mut Task) -> u8 {
    if ({
        (*task).data[8] -= 1;
        (*task).data[8]
    }) == 0
    {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn WeatherTrio_BgFadeBlack(task: *mut Task) -> u8 {
    BeginNormalPaletteFade(PALETTES_BG, 1, 0, 16, 0);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WeatherTrio_WaitFade(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PatternWeave_CircularMask(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    if (*task).data[2] < 1024 {
        (*task).data[2] += 128;
    }
    if (*task).data[1] != 0 {
        (*task).data[1] -= (*task).data[2] >> 8;
        if (*task).data[1] < 0 {
            (*task).data[1] = 0;
        }
    }
    SetCircularMask(
        gScanlineEffectRegBuffers[0].as_mut_ptr(),
        120,
        80,
        (*task).data[1],
    );
    if (*task).data[1] == 0 {
        SetVBlankCallback(None);
        {
            let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
            let _ = (dmaRegs.at(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc((*task).func));
    } else {
        if (*task).data[3] == 0 {
            (*task).data[3] += 1;
            SetVBlankCallback(Some(VBlankCB_CircularMask));
        }
        volatile_write(
            &raw mut (*sTransitionData).VBlank_DMA,
            (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
        );
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_SetWinAndBlend() {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    VBlankCB_BattleTransition();
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    volatile_write(67108936 as usize as *mut u16, (*sTransitionData).WININ);
    volatile_write(67108938 as usize as *mut u16, (*sTransitionData).WINOUT);
    volatile_write(67108932 as usize as *mut u16, (*sTransitionData).WIN0V);
    volatile_write(67108944 as usize as *mut u16, (*sTransitionData).BLDCNT);
    volatile_write(67108946 as usize as *mut u16, (*sTransitionData).BLDALPHA);
}
pub(crate) unsafe extern "C" fn VBlankCB_PatternWeave() {
    VBlankCB_SetWinAndBlend();
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(
                dmaRegs,
                gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
            );
            volatile_write(dmaRegs.at(1), 67108880 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_CircularMask() {
    VBlankCB_SetWinAndBlend();
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(
                dmaRegs,
                gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
            );
            volatile_write(dmaRegs.at(1), 67108928 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PokeballsTrail(taskId: u8) {
    while sPokeballsTrail_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn PokeballsTrail_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    CpuSet(
        sPokeballTrail_Tileset.as_ptr().cast_mut() as *mut c_void,
        tileset as *mut c_void,
        0x20,
    );
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x5000200,
            );
        }
    }
    LoadPalette(
        sFieldEffectPal_Pokeball.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn PokeballsTrail_Main(task: *mut Task) -> u8 {
    let mut i: i16 = 0;
    let mut side: i16 = 0;
    let mut startX: CArray<i16, 2> = zeroed();
    let mut delays: CArray<i16, 5> = zeroed();
    memcpy(
        startX.as_mut_ptr() as *mut u8,
        sPokeballsTrail_StartXCoords.as_ptr().cast_mut() as *mut u8,
        4,
    );
    memcpy(
        delays.as_mut_ptr() as *mut u8,
        sPokeballsTrail_Delays.as_ptr().cast_mut() as *mut u8,
        10,
    );
    side = Random() as i16 & 1;
    i = 0;
    while i < NUM_POKEBALL_TRAILS {
        gFieldEffectArguments[0] = startX[side] as i32;
        gFieldEffectArguments[1] = i as i32 * 32 + 16;
        gFieldEffectArguments[2] = side as i32;
        gFieldEffectArguments[3] = delays[i] as i32;
        FieldEffectStart(FLDEFF_POKEBALL_TRAIL);
        i += 1;
        side ^= 1;
    }
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn PokeballsTrail_End(task: *mut Task) -> u8 {
    if FieldEffectActiveListContains(FLDEFF_POKEBALL_TRAIL) == 0 {
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(Some(Task_PokeballsTrail)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_PokeballTrail() -> u8 {
    let mut spriteId: u8 = CreateSpriteAtEnd(
        (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        0,
    );
    gSprites[spriteId].oam.set_priority(0);
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[spriteId].data[0] = gFieldEffectArguments[2] as i16;
    gSprites[spriteId].data[1] = gFieldEffectArguments[3] as i16;
    gSprites[spriteId].data[2] = -1;
    InitSpriteAffineAnim(&raw mut gSprites[spriteId]);
    StartSpriteAffineAnim(&raw mut gSprites[spriteId], gFieldEffectArguments[2] as u8);
    return FALSE;
}
pub(crate) unsafe extern "C" fn SpriteCB_FldEffPokeballTrail(sprite: *mut Sprite) {
    let mut speeds: CArray<i16, 2> = zeroed();
    memcpy(
        speeds.as_mut_ptr() as *mut u8,
        sPokeballsTrail_Speeds.as_ptr().cast_mut() as *mut u8,
        4,
    );
    if (*sprite).data[1] != 0 {
        (*sprite).data[1] -= 1;
    } else {
        if (*sprite).x >= 0 && (*sprite).x <= DISPLAY_WIDTH as i16 {
            let mut posX: i16 = (*sprite).x >> 3;
            let mut posY: i16 = (*sprite).y >> 3;
            if posX != (*sprite).data[2] {
                let mut var: u32 = 0;
                let mut ptr: *mut u16 = null_mut();
                (*sprite).data[2] = posX;
                var = (((67108872 as usize as *mut u16).read_volatile() >> 8) as u32 & 0x1F) << 11;
                ptr = (BG_VRAM as u32 + var) as usize as *mut u16;
                {
                    let mut index: u32 = (posY as u32 - 2) * 32 + posX as u32;
                    *ptr.at(index) = 61441;
                }
                {
                    let mut index: u32 = (posY as u32 - 1) * 32 + posX as u32;
                    *ptr.at(index) = 61441;
                }
                {
                    let mut index: u32 = (posY as u32 - 0) * 32 + posX as u32;
                    *ptr.at(index) = 61441;
                }
                {
                    let mut index: u32 = (posY as u32 + 1) * 32 + posX as u32;
                    *ptr.at(index) = 61441;
                }
            }
        }
        (*sprite).x += speeds[(*sprite).data[0]];
        if (*sprite).x < -15 || (*sprite).x > 255 {
            FieldEffectStop(sprite, FLDEFF_POKEBALL_TRAIL);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ClockwiseWipe(taskId: u8) {
    while sClockwiseWipe_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_Init(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    (*sTransitionData).WININ = 0;
    (*sTransitionData).WINOUT = WINOUT_WIN01_ALL;
    (*sTransitionData).WIN0H = 61681;
    (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
    i = 0;
    while i < DISPLAY_HEIGHT {
        gScanlineEffectRegBuffers[1][i] = 62452;
        i += 1;
    }
    SetVBlankCallback(Some(VBlankCB_ClockwiseWipe));
    (*sTransitionData).data[4] = 120;
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_TopRight(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    InitBlackWipe(
        (*sTransitionData).data.as_mut_ptr(),
        120,
        80,
        (*sTransitionData).data[4],
        0,
        1,
        1,
    );
    loop {
        gScanlineEffectRegBuffers[0][(*sTransitionData).data[3]] =
            (*sTransitionData).data[2] as u16 + 1 | 30720;
        if UpdateBlackWipe((*sTransitionData).data.as_mut_ptr(), TRUE, TRUE) != 0 {
            break;
        }
    }
    (*sTransitionData).data[4] += 16;
    if (*sTransitionData).data[4] >= DISPLAY_WIDTH as i16 {
        (*sTransitionData).data[5] = 0;
        (*task).data[0] += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_Right(task: *mut Task) -> u8 {
    let mut start: i16 = 0;
    let mut end: i16 = 0;
    let mut finished: u8 = 0;
    volatile_write(&raw mut finished, FALSE);
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    InitBlackWipe(
        (*sTransitionData).data.as_mut_ptr(),
        120,
        80,
        DISPLAY_WIDTH as i16,
        (*sTransitionData).data[5],
        1,
        1,
    );
    loop {
        start = 120;
        end = (*sTransitionData).data[2] + 1;
        if (*sTransitionData).data[5] >= 80 {
            start = (*sTransitionData).data[2];
            end = DISPLAY_WIDTH as i16;
        }
        gScanlineEffectRegBuffers[0][(*sTransitionData).data[3]] = end as u16 | (start as u16) << 8;
        if (&raw mut finished).read_volatile() != 0 {
            break;
        }
        volatile_write(
            &raw mut finished,
            UpdateBlackWipe((*sTransitionData).data.as_mut_ptr(), TRUE, TRUE),
        );
    }
    (*sTransitionData).data[5] += 8;
    if (*sTransitionData).data[5] >= DISPLAY_HEIGHT as i16 {
        (*sTransitionData).data[4] = DISPLAY_WIDTH as i16;
        (*task).data[0] += 1;
    } else {
        while (*sTransitionData).data[3] < (*sTransitionData).data[5] {
            gScanlineEffectRegBuffers[0][{
                (*sTransitionData).data[3] += 1;
                (*sTransitionData).data[3]
            }] = end as u16 | (start as u16) << 8;
        }
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_Bottom(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    InitBlackWipe(
        (*sTransitionData).data.as_mut_ptr(),
        120,
        80,
        (*sTransitionData).data[4],
        DISPLAY_HEIGHT as i16,
        1,
        1,
    );
    loop {
        gScanlineEffectRegBuffers[0][(*sTransitionData).data[3]] =
            ((*sTransitionData).data[2] as u16) << 8 | DISPLAY_WIDTH;
        if UpdateBlackWipe((*sTransitionData).data.as_mut_ptr(), TRUE, TRUE) != 0 {
            break;
        }
    }
    (*sTransitionData).data[4] -= 16;
    if (*sTransitionData).data[4] <= 0 {
        (*sTransitionData).data[5] = DISPLAY_HEIGHT as i16;
        (*task).data[0] += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_Left(task: *mut Task) -> u8 {
    let mut end: i16 = 0;
    let mut start: i16 = 0;
    let mut temp: i16 = 0;
    let mut finished: u8 = 0;
    volatile_write(&raw mut finished, FALSE);
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    InitBlackWipe(
        (*sTransitionData).data.as_mut_ptr(),
        120,
        80,
        0,
        (*sTransitionData).data[5],
        1,
        1,
    );
    loop {
        end = gScanlineEffectRegBuffers[0][(*sTransitionData).data[3]] as i16 & 0xFF;
        start = (*sTransitionData).data[2];
        if (*sTransitionData).data[5] <= 80 {
            start = 120;
            end = (*sTransitionData).data[2];
        }
        temp = end | start << 8;
        gScanlineEffectRegBuffers[0][(*sTransitionData).data[3]] = temp as u16;
        if (&raw mut finished).read_volatile() != 0 {
            break;
        }
        volatile_write(
            &raw mut finished,
            UpdateBlackWipe((*sTransitionData).data.as_mut_ptr(), TRUE, TRUE),
        );
    }
    (*sTransitionData).data[5] -= 8;
    if (*sTransitionData).data[5] <= 0 {
        (*sTransitionData).data[4] = 0;
        (*task).data[0] += 1;
    } else {
        while (*sTransitionData).data[3] > (*sTransitionData).data[5] {
            gScanlineEffectRegBuffers[0][{
                (*sTransitionData).data[3] -= 1;
                (*sTransitionData).data[3]
            }] = end as u16 | (start as u16) << 8;
        }
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_TopLeft(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    InitBlackWipe(
        (*sTransitionData).data.as_mut_ptr(),
        120,
        80,
        (*sTransitionData).data[4],
        0,
        1,
        1,
    );
    loop {
        let mut start: i16 = 0;
        let mut end: i16 = 0;
        start = 120;
        end = (*sTransitionData).data[2];
        if (*sTransitionData).data[2] >= 120 {
            start = 0;
            end = DISPLAY_WIDTH as i16;
        }
        gScanlineEffectRegBuffers[0][(*sTransitionData).data[3]] = end as u16 | (start as u16) << 8;
        if UpdateBlackWipe((*sTransitionData).data.as_mut_ptr(), TRUE, TRUE) != 0 {
            break;
        }
    }
    (*sTransitionData).data[4] += 16;
    if (*sTransitionData).data[2] > 120 {
        (*task).data[0] += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_End(task: *mut Task) -> u8 {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    FadeScreenBlack();
    DestroyTask(FindTaskIdByFunc(Some(Task_ClockwiseWipe)));
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_ClockwiseWipe() {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    VBlankCB_BattleTransition();
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    volatile_write(67108936 as usize as *mut u16, (*sTransitionData).WININ);
    volatile_write(67108938 as usize as *mut u16, (*sTransitionData).WINOUT);
    volatile_write(67108932 as usize as *mut u16, (*sTransitionData).WIN0V);
    volatile_write(
        67108928 as usize as *mut u16,
        gScanlineEffectRegBuffers[1][0],
    );
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(
                dmaRegs,
                gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
            );
            volatile_write(dmaRegs.at(1), 67108928 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Ripple(taskId: u8) {
    while sRipple_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Ripple_Init(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    i = 0;
    while i < DISPLAY_HEIGHT as u8 {
        gScanlineEffectRegBuffers[1][i] = (*sTransitionData).cameraY as u16;
        i += 1;
    }
    SetVBlankCallback(Some(VBlankCB_Ripple));
    SetHBlankCallback(Some(HBlankCB_Ripple));
    EnableInterrupts(INTR_FLAG_HBLANK);
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn Ripple_Main(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    let mut amplitude: i16 = 0;
    let mut sinVal: u16 = 0;
    let mut speed: u16 = 0;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    amplitude = (*task).data[2] >> 8;
    sinVal = (*task).data[1] as u16;
    speed = 0x180;
    (*task).data[1] += 0x400;
    if (*task).data[2] <= 0x1FFF {
        (*task).data[2] += 0x180;
    }
    i = 0;
    while i < DISPLAY_HEIGHT as u8 {
        let mut sinIndex: i16 = (sinVal >> 8) as i16;
        gScanlineEffectRegBuffers[0][i] =
            (*sTransitionData).cameraY as u16 + Sin(sinIndex & -1, amplitude) as u16;
        i += 1;
        sinVal += speed;
    }
    if ({
        (*task).data[3] += 1;
        (*task).data[3]
    }) == 81
    {
        (*task).data[4] += 1;
        BeginNormalPaletteFade(PALETTES_ALL, -2, 0, 16, 0);
    }
    if (*task).data[4] != 0 && gPaletteFade.active() == 0 {
        DestroyTask(FindTaskIdByFunc(Some(Task_Ripple)));
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_Ripple() {
    VBlankCB_BattleTransition();
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_Ripple() {
    let mut var: u16 =
        gScanlineEffectRegBuffers[1][(67108870 as usize as *mut u16).read_volatile()];
    volatile_write(67108886 as usize as *mut u16, var);
    volatile_write(67108890 as usize as *mut u16, var);
    volatile_write(67108894 as usize as *mut u16, var);
}
pub(crate) unsafe extern "C" fn Task_Wave(taskId: u8) {
    while sWave_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Wave_Init(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    (*sTransitionData).WININ = WININ_WIN0_ALL;
    (*sTransitionData).WINOUT = 0;
    (*sTransitionData).WIN0H = DISPLAY_WIDTH;
    (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
    i = 0;
    while i < DISPLAY_HEIGHT as u8 {
        gScanlineEffectRegBuffers[1][i] = 242;
        i += 1;
    }
    SetVBlankCallback(Some(VBlankCB_Wave));
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn Wave_Main(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    let mut sinIndex: u8 = 0;
    let mut toStore: *mut u16 = null_mut();
    let mut finished: u8 = 0;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    toStore = gScanlineEffectRegBuffers[0].as_mut_ptr();
    sinIndex = (*task).data[2] as u8;
    (*task).data[2] += 16;
    (*task).data[1] += 8;
    i = 0;
    finished = TRUE;
    while i < DISPLAY_HEIGHT as u8 {
        let mut x: i16 = (*task).data[1] + Sin(sinIndex as i16, 40);
        if x < 0 {
            x = 0;
        }
        if x > DISPLAY_WIDTH as i16 {
            x = DISPLAY_WIDTH as i16;
        }
        *toStore = (x as u16) << 8 | 241;
        if x < DISPLAY_WIDTH as i16 {
            finished = FALSE;
        }
        i += 1;
        sinIndex += 4;
        toStore = toStore.at(1);
    }
    if finished != 0 {
        (*task).data[0] += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn Wave_End(task: *mut Task) -> u8 {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    FadeScreenBlack();
    DestroyTask(FindTaskIdByFunc(Some(Task_Wave)));
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_Wave() {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    VBlankCB_BattleTransition();
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    volatile_write(67108936 as usize as *mut u16, (*sTransitionData).WININ);
    volatile_write(67108938 as usize as *mut u16, (*sTransitionData).WINOUT);
    volatile_write(67108932 as usize as *mut u16, (*sTransitionData).WIN0V);
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(
                dmaRegs,
                gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
            );
            volatile_write(dmaRegs.at(1), 67108928 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Sidney(taskId: u8) {
    gTasks[taskId].data[15] = MUGSHOT_SIDNEY;
    DoMugshotTransition(taskId);
}
pub(crate) unsafe extern "C" fn Task_Phoebe(taskId: u8) {
    gTasks[taskId].data[15] = MUGSHOT_PHOEBE;
    DoMugshotTransition(taskId);
}
pub(crate) unsafe extern "C" fn Task_Glacia(taskId: u8) {
    gTasks[taskId].data[15] = MUGSHOT_GLACIA;
    DoMugshotTransition(taskId);
}
pub(crate) unsafe extern "C" fn Task_Drake(taskId: u8) {
    gTasks[taskId].data[15] = MUGSHOT_DRAKE;
    DoMugshotTransition(taskId);
}
pub(crate) unsafe extern "C" fn Task_Champion(taskId: u8) {
    gTasks[taskId].data[15] = MUGSHOT_CHAMPION;
    DoMugshotTransition(taskId);
}
pub(crate) unsafe extern "C" fn DoMugshotTransition(taskId: u8) {
    while sMugshot_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Mugshot_Init(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    Mugshots_CreateTrainerPics(task);
    (*task).data[1] = 0;
    (*task).data[2] = 1;
    (*task).data[3] = 239;
    (*sTransitionData).WININ = WININ_WIN0_ALL;
    (*sTransitionData).WINOUT = 62;
    (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
    i = 0;
    while i < DISPLAY_HEIGHT as u8 {
        gScanlineEffectRegBuffers[1][i] = 61681;
        i += 1;
    }
    SetVBlankCallback(Some(VBlankCB_Mugshots));
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Mugshot_SetGfx(task: *mut Task) -> u8 {
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    let mut mugshotsMap: *mut u16 = null_mut();
    mugshotsMap = sMugshotsTilemap.as_ptr().cast_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    CpuSet(
        sEliteFour_Tileset.as_ptr().cast_mut() as *mut c_void,
        tileset as *mut c_void,
        0xF0,
    );
    LoadPalette(
        sOpponentMugshotsPals[(*task).data[15]] as *mut c_void,
        240,
        32,
    );
    LoadPalette(
        sPlayerMugshotsPals[(*gSaveBlock2Ptr).playerGender] as *mut c_void,
        250,
        12,
    );
    i = 0;
    while i < 20 {
        j = 0;
        while j < 32 {
            let mut index: u32 = i as u32 * 32 + j as u32;
            *tilemap.at(index) = *mugshotsMap | 61440;
            j += 1;
            mugshotsMap = mugshotsMap.at(1);
        }
        i += 1;
    }
    EnableInterrupts(INTR_FLAG_HBLANK);
    SetHBlankCallback(Some(HBlankCB_Mugshots));
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Mugshot_ShowBanner(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    let mut sinIndex: u8 = 0;
    let mut toStore: *mut u16 = null_mut();
    let mut x: i16 = 0;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    toStore = gScanlineEffectRegBuffers[0].as_mut_ptr();
    sinIndex = (*task).data[1] as u8;
    (*task).data[1] += 16;
    i = 0;
    while i < 80 {
        x = (*task).data[2] + Sin(sinIndex as i16, 16);
        if x < 0 {
            x = 1;
        }
        if x > DISPLAY_WIDTH as i16 {
            x = DISPLAY_WIDTH as i16;
        }
        *toStore = x as u16;
        i += 1;
        toStore = toStore.at(1);
        sinIndex += 16;
    }
    while i < DISPLAY_HEIGHT as u8 {
        x = (*task).data[3] - Sin(sinIndex as i16, 16);
        if x < 0 {
            x = 0;
        }
        if x > 239 {
            x = 239;
        }
        *toStore = (x as u16) << 8 | DISPLAY_WIDTH;
        i += 1;
        toStore = toStore.at(1);
        sinIndex += 16;
    }
    (*task).data[2] += 8;
    (*task).data[3] -= 8;
    if (*task).data[2] > DISPLAY_WIDTH as i16 {
        (*task).data[2] = DISPLAY_WIDTH as i16;
    }
    if (*task).data[3] < 0 {
        (*task).data[3] = 0;
    }
    if (*task).data[2] == DISPLAY_WIDTH as i16 && (*task).data[3] == 0 {
        (*task).data[0] += 1;
    }
    (*sTransitionData).BG0HOFS_Lower -= 8;
    (*sTransitionData).BG0HOFS_Upper += 8;
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn Mugshot_StartOpponentSlide(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    let mut toStore: *mut u16 = null_mut();
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    i = 0;
    toStore = gScanlineEffectRegBuffers[0].as_mut_ptr();
    while i < DISPLAY_HEIGHT as u8 {
        *toStore = DISPLAY_WIDTH;
        i += 1;
        toStore = toStore.at(1);
    }
    (*task).data[0] += 1;
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    (*task).data[3] = 0;
    (*sTransitionData).BG0HOFS_Lower -= 8;
    (*sTransitionData).BG0HOFS_Upper += 8;
    SetTrainerPicSlideDirection((*task).data[13], 0);
    SetTrainerPicSlideDirection((*task).data[14], 1);
    IncrementTrainerPicState((*task).data[13]);
    PlaySE(SE_MUGSHOT);
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn Mugshot_WaitStartPlayerSlide(task: *mut Task) -> u8 {
    (*sTransitionData).BG0HOFS_Lower -= 8;
    (*sTransitionData).BG0HOFS_Upper += 8;
    if IsTrainerPicSlideDone((*task).data[13]) != 0 {
        (*task).data[0] += 1;
        IncrementTrainerPicState((*task).data[14]);
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Mugshot_WaitPlayerSlide(task: *mut Task) -> u8 {
    (*sTransitionData).BG0HOFS_Lower -= 8;
    (*sTransitionData).BG0HOFS_Upper += 8;
    if IsTrainerPicSlideDone((*task).data[14]) != 0 {
        volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
        SetVBlankCallback(None);
        {
            let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
            let _ = (dmaRegs.at(5)).read_volatile();
        }
        memset(gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut u8, 0, 320);
        memset(gScanlineEffectRegBuffers[1].as_mut_ptr() as *mut u8, 0, 320);
        SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
        SetGpuReg(REG_OFFSET_BLDY, 0);
        (*task).data[0] += 1;
        (*task).data[3] = 0;
        (*task).data[4] = 0;
        (*sTransitionData).BLDCNT = 191;
        SetVBlankCallback(Some(VBlankCB_MugshotsFadeOut));
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Mugshot_GradualWhiteFade(task: *mut Task) -> u8 {
    let mut active: u32 = 0;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    active = TRUE as u32;
    (*sTransitionData).BG0HOFS_Lower -= 8;
    (*sTransitionData).BG0HOFS_Upper += 8;
    if (*task).data[4] < 80 {
        (*task).data[4] += 2;
    }
    if (*task).data[4] > 80 {
        (*task).data[4] = 80;
    }
    if ({
        (*task).data[3] += 1;
        (*task).data[3]
    }) as i32
        & 1
        != 0
    {
        let mut i: i16 = 0;
        i = 0;
        active = 0;
        while i <= (*task).data[4] {
            let mut index1: i16 = 80 - i;
            let mut index2: i16 = 80 + i;
            if gScanlineEffectRegBuffers[0][index1] <= 15 {
                active = TRUE as u32;
                gScanlineEffectRegBuffers[0][index1] += 1;
            }
            if gScanlineEffectRegBuffers[0][index2] <= 15 {
                active = TRUE as u32;
                gScanlineEffectRegBuffers[0][index2] += 1;
            }
            i += 1;
        }
    }
    if (*task).data[4] == 80 && active == 0 {
        (*task).data[0] += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn Mugshot_InitFadeWhiteToBlack(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    BlendPalettes(PALETTES_ALL, 16, 32767);
    (*sTransitionData).BLDCNT = 0xFF;
    (*task).data[3] = 0;
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn Mugshot_FadeToBlack(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    (*task).data[3] += 1;
    memset(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut u8,
        (*task).data[3] as i32,
        320,
    );
    if (*task).data[3] > 15 {
        (*task).data[0] += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn Mugshot_End(task: *mut Task) -> u8 {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    FadeScreenBlack();
    DestroyTask(FindTaskIdByFunc((*task).func));
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_Mugshots() {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    VBlankCB_BattleTransition();
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    volatile_write(
        67108882 as usize as *mut u16,
        (*sTransitionData).BG0VOFS as u16,
    );
    volatile_write(67108936 as usize as *mut u16, (*sTransitionData).WININ);
    volatile_write(67108938 as usize as *mut u16, (*sTransitionData).WINOUT);
    volatile_write(67108932 as usize as *mut u16, (*sTransitionData).WIN0V);
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(
                dmaRegs,
                gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
            );
            volatile_write(dmaRegs.at(1), 67108928 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_MugshotsFadeOut() {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    VBlankCB_BattleTransition();
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    volatile_write(67108944 as usize as *mut u16, (*sTransitionData).BLDCNT);
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(
                dmaRegs,
                gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
            );
            volatile_write(dmaRegs.at(1), 67108948 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_Mugshots() {
    if (67108870 as usize as *mut u16).read_volatile() < 80 {
        volatile_write(
            67108880 as usize as *mut u16,
            (*sTransitionData).BG0HOFS_Lower as u16,
        );
    } else {
        volatile_write(
            67108880 as usize as *mut u16,
            (*sTransitionData).BG0HOFS_Upper as u16,
        );
    }
}
pub(crate) unsafe extern "C" fn Mugshots_CreateTrainerPics(task: *mut Task) {
    let mut opponentSprite: *mut Sprite = null_mut();
    let mut playerSprite: *mut Sprite = null_mut();
    let mut mugshotId: i16 = (*task).data[15];
    (*task).data[13] = CreateTrainerSprite(
        sMugshotsTrainerPicIDsTable[mugshotId],
        sMugshotsOpponentCoords[mugshotId][0] - 32,
        sMugshotsOpponentCoords[mugshotId][1] + 42,
        0,
        gDecompressionBuffer.as_mut_ptr(),
    ) as i16;
    (*task).data[14] = CreateTrainerSprite(
        PlayerGenderToFrontTrainerPicId((*gSaveBlock2Ptr).playerGender) as u8,
        272,
        106,
        0,
        gDecompressionBuffer.as_mut_ptr(),
    ) as i16;
    opponentSprite = &raw mut gSprites[(*task).data[13]];
    playerSprite = &raw mut gSprites[(*task).data[14]];
    (*opponentSprite).callback = Some(SpriteCB_MugshotTrainerPic);
    (*playerSprite).callback = Some(SpriteCB_MugshotTrainerPic);
    (*opponentSprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    (*playerSprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    (*opponentSprite).oam.set_matrixNum(AllocOamMatrix() as u32);
    (*playerSprite).oam.set_matrixNum(AllocOamMatrix() as u32);
    (*opponentSprite).oam.set_shape(1);
    (*playerSprite).oam.set_shape(1);
    (*opponentSprite).oam.set_size(3);
    (*playerSprite).oam.set_size(3);
    CalcCenterToCornerVec(
        opponentSprite,
        1,
        ST_OAM_AFFINE_DOUBLE as u8,
        ST_OAM_AFFINE_DOUBLE as u8,
    );
    CalcCenterToCornerVec(
        playerSprite,
        1,
        ST_OAM_AFFINE_DOUBLE as u8,
        ST_OAM_AFFINE_DOUBLE as u8,
    );
    SetOamMatrixRotationScaling(
        (*opponentSprite).oam.matrixNum() as u8,
        sMugshotsOpponentRotationScales[mugshotId][0],
        sMugshotsOpponentRotationScales[mugshotId][1],
        0,
    );
    SetOamMatrixRotationScaling((*playerSprite).oam.matrixNum() as u8, -512, 512, 0);
}
pub(crate) unsafe extern "C" fn SpriteCB_MugshotTrainerPic(sprite: *mut Sprite) {
    while sMugshotTrainerPicFuncs[(*sprite).data[0]].unwrap_unchecked()(sprite) != 0 {}
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_Pause(sprite: *mut Sprite) -> u8 {
    return FALSE;
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_Init(sprite: *mut Sprite) -> u8 {
    let mut speeds: CArray<i16, 2> = zeroed();
    let mut accels: CArray<i16, 2> = zeroed();
    memcpy(
        speeds.as_mut_ptr() as *mut u8,
        sTrainerPicSlideSpeeds.as_ptr().cast_mut() as *mut u8,
        4,
    );
    memcpy(
        accels.as_mut_ptr() as *mut u8,
        sTrainerPicSlideAccels.as_ptr().cast_mut() as *mut u8,
        4,
    );
    (*sprite).data[0] += 1;
    (*sprite).data[1] = speeds[(*sprite).data[7]];
    (*sprite).data[2] = accels[(*sprite).data[7]];
    return TRUE;
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_Slide(sprite: *mut Sprite) -> u8 {
    (*sprite).x += (*sprite).data[1];
    if (*sprite).data[7] != 0 && (*sprite).x < 133 {
        (*sprite).data[0] += 1;
    } else if (*sprite).data[7] == 0 && (*sprite).x > 103 {
        (*sprite).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_SlideSlow(sprite: *mut Sprite) -> u8 {
    (*sprite).data[1] += (*sprite).data[2];
    (*sprite).x += (*sprite).data[1];
    if (*sprite).data[1] == 0 {
        (*sprite).data[0] += 1;
        (*sprite).data[2] = -(*sprite).data[2];
        (*sprite).data[6] = TRUE as i16;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_SlideOffscreen(sprite: *mut Sprite) -> u8 {
    (*sprite).data[1] += (*sprite).data[2];
    (*sprite).x += (*sprite).data[1];
    if (*sprite).x < -31 || (*sprite).x > 271 {
        (*sprite).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SetTrainerPicSlideDirection(spriteId: i16, dirId: i16) {
    gSprites[spriteId].data[7] = dirId;
}
pub(crate) unsafe extern "C" fn IncrementTrainerPicState(spriteId: i16) {
    gSprites[spriteId].data[0] += 1;
}
pub(crate) unsafe extern "C" fn IsTrainerPicSlideDone(spriteId: i16) -> i16 {
    return gSprites[spriteId].data[6];
}
pub(crate) unsafe extern "C" fn Task_Slice(taskId: u8) {
    while sSlice_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Slice_Init(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    (*task).data[2] = 256;
    (*task).data[3] = 1;
    (*sTransitionData).WININ = WININ_WIN0_ALL;
    (*sTransitionData).WINOUT = 0;
    (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    i = 0;
    while i < DISPLAY_HEIGHT {
        gScanlineEffectRegBuffers[1][i] = (*sTransitionData).cameraX as u16;
        gScanlineEffectRegBuffers[1][DISPLAY_HEIGHT as i32 + i as i32] = DISPLAY_WIDTH;
        i += 1;
    }
    EnableInterrupts(INTR_FLAG_HBLANK);
    SetGpuRegBits(REG_OFFSET_DISPSTAT, DISPSTAT_HBLANK_INTR);
    SetVBlankCallback(Some(VBlankCB_Slice));
    SetHBlankCallback(Some(HBlankCB_Slice));
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn Slice_Main(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    (*task).data[1] += (*task).data[2] >> 8;
    if (*task).data[1] > DISPLAY_WIDTH as i16 {
        (*task).data[1] = DISPLAY_WIDTH as i16;
    }
    if (*task).data[2] <= 0xFFF {
        (*task).data[2] += (*task).data[3];
    }
    if (*task).data[3] < 128 {
        (*task).data[3] <<= 1;
    }
    i = 0;
    while i < DISPLAY_HEIGHT {
        let mut storeLoc1: *mut u16 = &raw mut gScanlineEffectRegBuffers[0][i];
        let mut storeLoc2: *mut u16 =
            &raw mut gScanlineEffectRegBuffers[0][i as i32 + DISPLAY_HEIGHT as i32];
        if i as i32 % 2 != 0 {
            *storeLoc1 = (*sTransitionData).cameraX as u16 + (*task).data[1] as u16;
            *storeLoc2 = DISPLAY_WIDTH - (*task).data[1] as u16;
        } else {
            *storeLoc1 = (*sTransitionData).cameraX as u16 - (*task).data[1] as u16;
            *storeLoc2 = ((*task).data[1] as u16) << 8 | 241;
        }
        i += 1;
    }
    if (*task).data[1] >= DISPLAY_WIDTH as i16 {
        (*task).data[0] += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn Slice_End(task: *mut Task) -> u8 {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    FadeScreenBlack();
    DestroyTask(FindTaskIdByFunc(Some(Task_Slice)));
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_Slice() {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    VBlankCB_BattleTransition();
    volatile_write(67108936 as usize as *mut u16, (*sTransitionData).WININ);
    volatile_write(67108938 as usize as *mut u16, (*sTransitionData).WINOUT);
    volatile_write(67108932 as usize as *mut u16, (*sTransitionData).WIN0V);
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x80000140);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(
                dmaRegs,
                &raw mut gScanlineEffectRegBuffers[1][160] as usize as u32,
            );
            volatile_write(dmaRegs.at(1), 67108928 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_Slice() {
    if (67108870 as usize as *mut u16).read_volatile() < DISPLAY_HEIGHT {
        let mut var: u16 =
            gScanlineEffectRegBuffers[1][(67108870 as usize as *mut u16).read_volatile()];
        volatile_write(67108884 as usize as *mut u16, var);
        volatile_write(67108888 as usize as *mut u16, var);
        volatile_write(67108892 as usize as *mut u16, var);
    }
}
pub(crate) unsafe extern "C" fn Task_ShredSplit(taskId: u8) {
    while sShredSplit_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0
    {
    }
}
pub(crate) unsafe extern "C" fn ShredSplit_Init(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    (*sTransitionData).WININ = WININ_WIN0_ALL;
    (*sTransitionData).WINOUT = 0;
    (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
    i = 0;
    while i < DISPLAY_HEIGHT {
        gScanlineEffectRegBuffers[1][i] = (*sTransitionData).cameraX as u16;
        gScanlineEffectRegBuffers[1][DISPLAY_HEIGHT as i32 + i as i32] = DISPLAY_WIDTH;
        gScanlineEffectRegBuffers[0][i] = (*sTransitionData).cameraX as u16;
        gScanlineEffectRegBuffers[0][DISPLAY_HEIGHT as i32 + i as i32] = DISPLAY_WIDTH;
        gScanlineEffectRegBuffers[0][320 + i as i32] = 0;
        gScanlineEffectRegBuffers[0][480 + i as i32] = 256;
        gScanlineEffectRegBuffers[0][640 + i as i32] = 1;
        i += 1;
    }
    (*task).data[4] = 0;
    (*task).data[5] = 0;
    (*task).data[6] = 7;
    EnableInterrupts(INTR_FLAG_HBLANK);
    SetVBlankCallback(Some(VBlankCB_Slice));
    SetHBlankCallback(Some(HBlankCB_Slice));
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShredSplit_Main(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut k: u16 = 0;
    let mut baseY: CArray<u8, 2> = zeroed();
    let mut moveDirs: CArray<i16, 2> = zeroed();
    let mut linesFinished: u8 = 0;
    let mut ptr4: *mut u16 = null_mut();
    let mut ptr3: *mut u16 = null_mut();
    let mut ptr1: *mut u16 = null_mut();
    let mut ptr2: *mut u16 = null_mut();
    let mut y: i16 = 0;
    memcpy(
        baseY.as_mut_ptr(),
        sShredSplit_SectionYCoords.as_ptr().cast_mut(),
        2,
    );
    memcpy(
        moveDirs.as_mut_ptr() as *mut u8,
        sShredSplit_SectionMoveDirs.as_ptr().cast_mut() as *mut u8,
        4,
    );
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    linesFinished = 0;
    i = 0;
    while i as i32 <= (*task).data[5] as i32 {
        j = 0;
        while j < 2 {
            k = 0;
            while k < 2 {
                y = baseY[j] as i16 + moveDirs[k] * -(i as i16) * 2;
                if y >= 0 && (y != 79 || j != 1) {
                    ptr4 = &raw mut gScanlineEffectRegBuffers[0][y as i32 + 320];
                    ptr3 = &raw mut gScanlineEffectRegBuffers[0][y as i32 + 480];
                    ptr1 = &raw mut gScanlineEffectRegBuffers[0][y as i32 + 640];
                    if *ptr4 >= DISPLAY_WIDTH {
                        *ptr4 = DISPLAY_WIDTH;
                        linesFinished += 1;
                    } else {
                        *ptr4 += *ptr3 >> 8;
                        if *ptr1 <= 0x7F {
                            *ptr1 *= 2;
                        }
                        if *ptr3 <= 0xFFF {
                            *ptr3 += *ptr1;
                        }
                    }
                    ptr2 = &raw mut gScanlineEffectRegBuffers[0][y];
                    ptr3 = &raw mut gScanlineEffectRegBuffers[0][y as i32 + DISPLAY_HEIGHT as i32];
                    *ptr2 = (*sTransitionData).cameraX as u16 + *ptr4;
                    *ptr3 = DISPLAY_WIDTH - *ptr4;
                    if i == 0 {
                        break;
                    }
                }
                k += 1;
            }
            j += 1;
        }
        j = 0;
        while j < 2 {
            k = 0;
            while k < 2 {
                y = baseY[j] as i16 + 1 + moveDirs[k] * -(i as i16) * 2;
                if y <= DISPLAY_HEIGHT as i16 && (y != 80 || j != 1) {
                    ptr4 = &raw mut gScanlineEffectRegBuffers[0][y as i32 + 320];
                    ptr3 = &raw mut gScanlineEffectRegBuffers[0][y as i32 + 480];
                    ptr1 = &raw mut gScanlineEffectRegBuffers[0][y as i32 + 640];
                    if *ptr4 >= DISPLAY_WIDTH {
                        *ptr4 = DISPLAY_WIDTH;
                        linesFinished += 1;
                    } else {
                        *ptr4 += *ptr3 >> 8;
                        if *ptr1 <= 0x7F {
                            *ptr1 *= 2;
                        }
                        if *ptr3 <= 0xFFF {
                            *ptr3 += *ptr1;
                        }
                    }
                    ptr2 = &raw mut gScanlineEffectRegBuffers[0][y];
                    ptr3 = &raw mut gScanlineEffectRegBuffers[0][y as i32 + DISPLAY_HEIGHT as i32];
                    *ptr2 = (*sTransitionData).cameraX as u16 - *ptr4;
                    *ptr3 = *ptr4 << 8 | 241;
                    if i == 0 {
                        break;
                    }
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    if ({
        (*task).data[4] -= 1;
        (*task).data[4]
    }) < 0
    {
        (*task).data[4] = 0;
    }
    if (*task).data[4] <= 0 && (*task).data[5] as i32 + 1 <= 20 {
        (*task).data[4] = (*task).data[6];
        (*task).data[5] += 1;
    }
    if linesFinished >= DISPLAY_HEIGHT as u8 {
        (*task).data[0] += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn ShredSplit_BrokenCheck(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    let mut done: u32 = TRUE as u32;
    let mut checkVar2: u16 = 0xFF10;
    i = 0;
    while i < DISPLAY_HEIGHT {
        if gScanlineEffectRegBuffers[1][i] != DISPLAY_WIDTH
            && gScanlineEffectRegBuffers[1][i] != checkVar2
        {
            done = FALSE as u32;
        }
        i += 1;
    }
    if done == TRUE as u32 {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ShredSplit_End(task: *mut Task) -> u8 {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    FadeScreenBlack();
    DestroyTask(FindTaskIdByFunc(Some(Task_ShredSplit)));
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_Blackhole(taskId: u8) {
    while sBlackhole_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0
    {
    }
}
pub(crate) unsafe extern "C" fn Task_BlackholePulsate(taskId: u8) {
    while sBlackholePulsate_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn Blackhole_Init(task: *mut Task) -> u8 {
    let mut i: i32 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    (*sTransitionData).WININ = 0;
    (*sTransitionData).WINOUT = WINOUT_WIN01_ALL;
    (*sTransitionData).WIN0H = DISPLAY_WIDTH;
    (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
    i = 0;
    while i < DISPLAY_HEIGHT as i32 {
        gScanlineEffectRegBuffers[1][i] = 0;
        i += 1;
    }
    SetVBlankCallback(Some(VBlankCB_CircularMask));
    (*task).data[0] += 1;
    (*task).data[1] = 1;
    (*task).data[2] = 256;
    (*task).data[7] = FALSE as i16;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Blackhole_GrowEnd(task: *mut Task) -> u8 {
    if (*task).data[7] == TRUE as i16 {
        {
            let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
            let _ = (dmaRegs.at(5)).read_volatile();
        }
        SetVBlankCallback(None);
        DestroyTask(FindTaskIdByFunc((*task).func));
    } else {
        volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
        if (*task).data[2] < 1024 {
            (*task).data[2] += 128;
        }
        if (*task).data[1] < DISPLAY_HEIGHT as i16 {
            (*task).data[1] += (*task).data[2] >> 8;
        }
        if (*task).data[1] > DISPLAY_HEIGHT as i16 {
            (*task).data[1] = DISPLAY_HEIGHT as i16;
        }
        SetCircularMask(
            gScanlineEffectRegBuffers[0].as_mut_ptr(),
            120,
            80,
            (*task).data[1],
        );
        if (*task).data[1] == DISPLAY_HEIGHT as i16 {
            (*task).data[7] = TRUE as i16;
            FadeScreenBlack();
        } else {
            volatile_write(
                &raw mut (*sTransitionData).VBlank_DMA,
                (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
            );
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Blackhole_Vibrate(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    if (*task).data[7] == FALSE as i16 {
        (*task).data[7] += 1;
        (*task).data[1] = 48;
        (*task).data[6] = 0;
    }
    (*task).data[1] += sBlackhole_Vibrations[(*task).data[6]];
    (*task).data[6] = (((*task).data[6] as i32 + 1) % 2) as i16;
    SetCircularMask(
        gScanlineEffectRegBuffers[0].as_mut_ptr(),
        120,
        80,
        (*task).data[1],
    );
    if (*task).data[1] < 9 {
        (*task).data[0] += 1;
        (*task).data[7] = FALSE as i16;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn BlackholePulsate_Main(task: *mut Task) -> u8 {
    let mut index: u16 = 0;
    let mut amplitude: i16 = 0;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    if (*task).data[7] == FALSE as i16 {
        (*task).data[7] += 1;
        (*task).data[5] = 2;
        (*task).data[6] = 2;
    }
    if (*task).data[1] > DISPLAY_HEIGHT as i16 {
        (*task).data[1] = DISPLAY_HEIGHT as i16;
    }
    SetCircularMask(
        gScanlineEffectRegBuffers[0].as_mut_ptr(),
        120,
        80,
        (*task).data[1],
    );
    if (*task).data[1] == DISPLAY_HEIGHT as i16 {
        {
            let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
            let _ = (dmaRegs.at(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc((*task).func));
    }
    index = (*task).data[5] as u16;
    if (*task).data[5] as i32 & 0xFF <= 128 {
        amplitude = (*task).data[6];
        (*task).data[5] += 8;
    } else {
        amplitude = (*task).data[6] - 1;
        (*task).data[5] += 16;
    }
    (*task).data[1] += Sin(index as i16 & 0xFF, amplitude);
    if (*task).data[1] <= 0 {
        (*task).data[1] = 1;
    }
    if (*task).data[5] >= 0xFF {
        (*task).data[5] >>= 8;
        (*task).data[6] += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_RectangularSpiral(taskId: u8) {
    while sRectangularSpiral_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn RectangularSpiral_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    CpuSet(
        sShrinkingBoxTileset.as_ptr().cast_mut() as *mut c_void,
        tileset as *mut c_void,
        16,
    );
    CpuSet(
        (&raw const sShrinkingBoxTileset[112]).cast_mut() as *mut c_void,
        tileset.at(32) as *mut c_void,
        16,
    );
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 61440);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    LoadPalette(
        sFieldEffectPal_Pokeball.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*task).data[3] = 1;
    (*task).data[0] += 1;
    sRectangularSpiralLines[0].state = 0;
    sRectangularSpiralLines[0].position = -1;
    sRectangularSpiralLines[0].moveIndex = 1;
    sRectangularSpiralLines[0].reboundPosition = 308;
    sRectangularSpiralLines[0].outward = 0;
    sRectangularSpiralLines[1].state = SPIRAL_INWARD_START;
    sRectangularSpiralLines[1].position = -1;
    sRectangularSpiralLines[1].moveIndex = 1;
    sRectangularSpiralLines[1].reboundPosition = 308;
    sRectangularSpiralLines[1].outward = FALSE;
    sRectangularSpiralLines[2].state = SPIRAL_INWARD_START;
    sRectangularSpiralLines[2].position = -3;
    sRectangularSpiralLines[2].moveIndex = 1;
    sRectangularSpiralLines[2].reboundPosition = 307;
    sRectangularSpiralLines[2].outward = FALSE;
    sRectangularSpiralLines[3].state = SPIRAL_INWARD_START;
    sRectangularSpiralLines[3].position = -3;
    sRectangularSpiralLines[3].moveIndex = 1;
    sRectangularSpiralLines[3].reboundPosition = 307;
    sRectangularSpiralLines[3].outward = FALSE;
    return FALSE;
}
pub(crate) unsafe extern "C" fn RectangularSpiral_Main(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    let mut i: u8 = 0;
    let mut j: u16 = 0;
    let mut done: u32 = TRUE as u32;
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    i = 0;
    while i < 2 {
        j = 0;
        while j < 4 {
            let mut position: i16 = 0;
            let mut x: i16 = 0;
            let mut y: i16 = 0;
            if UpdateRectangularSpiralLine(
                sRectangularSpiral_MoveDataTables[j as i32 / 2],
                &raw mut sRectangularSpiralLines[j],
            ) != 0
            {
                done = FALSE as u32;
                position = sRectangularSpiralLines[j].position;
                if j as i32 % 2 == 1 {
                    position = 637 - position;
                }
                x = position % 32;
                y = position / 32;
                {
                    let mut index: u32 = y as u32 * 32 + x as u32;
                    *tilemap.at(index) = 61442;
                }
            }
            j += 1;
        }
        i += 1;
    }
    if done == TRUE as u32 {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn RectangularSpiral_End(task: *mut Task) -> u8 {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    FadeScreenBlack();
    DestroyTask(FindTaskIdByFunc((*task).func));
    return FALSE;
}
pub(crate) unsafe extern "C" fn UpdateRectangularSpiralLine(
    moveDataTable: *mut *mut i16,
    line: *mut RectangularSpiralLine,
) -> u16 {
    let mut moveData: *mut i16 = *moveDataTable.at((*line).state);
    if *moveData.at((*line).moveIndex) == SPIRAL_END {
        return FALSE as u16;
    }
    sDebug_RectangularSpiralData = *moveData;
    sDebug_RectangularSpiralData = *moveData.at(1);
    sDebug_RectangularSpiralData = *moveData.at(2);
    sDebug_RectangularSpiralData = *moveData.at(3);
    match *moveData {
        MOVE_RIGHT => {
            (*line).position += 1;
        }
        MOVE_LEFT => {
            (*line).position -= 1;
        }
        MOVE_UP => {
            (*line).position -= 32;
        }
        MOVE_DOWN => {
            (*line).position += 32;
        }
        _ => {}
    }
    if (*line).position >= 640 || *moveData.at((*line).moveIndex) == SPIRAL_END {
        return FALSE as u16;
    }
    if (*line).outward == 0 && *moveData.at((*line).moveIndex) == SPIRAL_REBOUND {
        (*line).outward = TRUE;
        (*line).moveIndex = 1;
        (*line).position = (*line).reboundPosition;
        (*line).state = SPIRAL_OUTWARD_START;
    }
    if (*line).position == *moveData.at((*line).moveIndex) {
        (*line).state += 1;
        if (*line).outward == TRUE {
            if (*line).state > SPIRAL_OUTWARD_END {
                (*line).moveIndex += 1;
                (*line).state = SPIRAL_OUTWARD_START;
            }
        } else {
            if (*line).state > SPIRAL_INWARD_END {
                (*line).moveIndex += 1;
                (*line).state = SPIRAL_INWARD_START;
            }
        }
    }
    return TRUE as u16;
}
pub(crate) unsafe extern "C" fn Task_Groudon(taskId: u8) {
    while sGroudon_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn Groudon_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    LZ77UnCompVram(sGroudon_Tileset.as_ptr().cast_mut(), tileset as *mut c_void);
    LZ77UnCompVram(sGroudon_Tilemap.as_ptr().cast_mut(), tilemap as *mut c_void);
    (*task).data[0] += 1;
    (*task).data[1] = 0;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Groudon_PaletteFlash(task: *mut Task) -> u8 {
    if (*task).data[1] % 3 == 0 {
        let mut offset: u16 = ((*task).data[1] % 30 / 3) as u16;
        LoadPalette(
            (&raw const sGroudon1_Palette[offset as i32 * 16]).cast_mut() as *mut c_void,
            240,
            32,
        );
    }
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 58
    {
        (*task).data[0] += 1;
        (*task).data[1] = 0;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Groudon_PaletteBrighten(task: *mut Task) -> u8 {
    if (*task).data[1] % 5 == 0 {
        let mut offset: i16 = (*task).data[1] / 5;
        LoadPalette(
            (&raw const sGroudon2_Palette[offset as i32 * 16]).cast_mut() as *mut c_void,
            240,
            32,
        );
    }
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 68
    {
        (*task).data[0] += 1;
        (*task).data[1] = 0;
        (*task).data[8] = 30;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_Rayquaza(taskId: u8) {
    while sRayquaza_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {
    }
}
pub(crate) unsafe extern "C" fn Rayquaza_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    let mut i: u16 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    SetGpuReg(REG_OFFSET_BG0CNT, 39432);
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    CpuSet(
        sRayquaza_Tileset.as_ptr().cast_mut() as *mut c_void,
        tileset as *mut c_void,
        4096,
    );
    (*sTransitionData).counter = 0;
    (*task).data[0] += 1;
    LoadPalette(
        (&raw const sRayquaza_Palette[80]).cast_mut() as *mut c_void,
        240,
        32,
    );
    i = 0;
    while i < DISPLAY_HEIGHT {
        gScanlineEffectRegBuffers[0][i] = 0;
        gScanlineEffectRegBuffers[1][i] = 0x100;
        i += 1;
    }
    SetVBlankCallback(Some(VBlankCB_Rayquaza));
    return FALSE;
}
pub(crate) unsafe extern "C" fn Rayquaza_SetGfx(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    CpuSet(
        sRayquaza_Tilemap.as_ptr().cast_mut() as *mut c_void,
        tilemap as *mut c_void,
        2048,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Rayquaza_PaletteFlash(task: *mut Task) -> u8 {
    if (*task).data[1] % 4 == 0 {
        let mut value: u16 = ((*task).data[1] / 4) as u16;
        let mut palPtr: *mut u16 =
            (&raw const sRayquaza_Palette[(value as i32 + 5) * 16]).cast_mut();
        LoadPalette(palPtr as *mut c_void, 240, 32);
    }
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 40
    {
        (*task).data[0] += 1;
        (*task).data[1] = 0;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Rayquaza_FadeToBlack(task: *mut Task) -> u8 {
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 20
    {
        (*task).data[0] += 1;
        (*task).data[1] = 0;
        BeginNormalPaletteFade(0xffff8000, 2, 0, 16, 0);
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Rayquaza_WaitFade(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        (*sTransitionData).counter = 1;
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Rayquaza_SetBlack(task: *mut Task) -> u8 {
    BlendPalettes(32767, 8, 0);
    BlendPalettes(0xffff8000, 0, 0);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Rayquaza_TriRing(task: *mut Task) -> u8 {
    if (*task).data[1] % 3 == 0 {
        let mut value: u16 = ((*task).data[1] / 3) as u16;
        let mut palPtr: *mut u16 =
            (&raw const sRayquaza_Palette[(value as i32 + 0) * 16]).cast_mut();
        LoadPalette(palPtr as *mut c_void, 240, 32);
    }
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) >= 40
    {
        let mut i: u16 = 0;
        (*sTransitionData).WININ = 0;
        (*sTransitionData).WINOUT = WINOUT_WIN01_ALL;
        (*sTransitionData).WIN0H = DISPLAY_WIDTH;
        (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
        i = 0;
        while i < DISPLAY_HEIGHT {
            gScanlineEffectRegBuffers[1][i] = 0;
            i += 1;
        }
        SetVBlankCallback(Some(VBlankCB_CircularMask));
        (*task).data[0] += 1;
        (*task).data[2] = 256;
        (*task).data[7] = FALSE as i16;
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG0_ON);
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_Rayquaza() {
    let mut dmaSrc: *mut c_void = null_mut();
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    VBlankCB_BattleTransition();
    if (*sTransitionData).counter == 0 {
        dmaSrc = gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut c_void;
    } else if (*sTransitionData).counter == 1 {
        dmaSrc = gScanlineEffectRegBuffers[1].as_mut_ptr() as *mut c_void;
    } else {
        dmaSrc = gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut c_void;
    }
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(dmaRegs, dmaSrc as usize as u32);
            volatile_write(dmaRegs.at(1), 67108882 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WhiteBarsFade(taskId: u8) {
    while sWhiteBarsFade_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_Init(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    (*sTransitionData).BLDCNT = 191;
    (*sTransitionData).BLDY = 0;
    (*sTransitionData).WININ = 30;
    (*sTransitionData).WINOUT = WINOUT_WIN01_ALL;
    (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
    i = 0;
    while i < DISPLAY_HEIGHT {
        gScanlineEffectRegBuffers[1][i] = 0;
        gScanlineEffectRegBuffers[1][i as i32 + DISPLAY_HEIGHT as i32] = DISPLAY_WIDTH;
        i += 1;
    }
    EnableInterrupts(INTR_FLAG_HBLANK);
    SetHBlankCallback(Some(HBlankCB_WhiteBarsFade));
    SetVBlankCallback(Some(VBlankCB_WhiteBarsFade));
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_StartBars(task: *mut Task) -> u8 {
    let mut i: i16 = 0;
    let mut posY: i16 = 0;
    let mut delays: CArray<i16, 8> = zeroed();
    let mut sprite: *mut Sprite = null_mut();
    memcpy(
        delays.as_mut_ptr() as *mut u8,
        sWhiteBarsFade_StartDelays.as_ptr().cast_mut() as *mut u8,
        16,
    );
    i = 0;
    posY = 0;
    while i < NUM_WHITE_BARS {
        sprite = &raw mut gSprites[CreateInvisibleSprite(Some(SpriteCB_WhiteBarFade))];
        (*sprite).x = DISPLAY_WIDTH as i16;
        (*sprite).y = posY;
        (*sprite).data[5] = delays[i];
        i += 1;
        posY += 20;
    }
    (*sprite).data[6] += 1;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_WaitBars(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, 0);
    if (*sTransitionData).counter >= NUM_WHITE_BARS {
        BlendPalettes(PALETTES_ALL, 16, 32767);
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_BlendToBlack(task: *mut Task) -> u8 {
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, 0);
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    SetVBlankCallback(None);
    SetHBlankCallback(None);
    (*sTransitionData).WIN0H = DISPLAY_WIDTH;
    (*sTransitionData).BLDY = 0;
    (*sTransitionData).BLDCNT = 0xFF;
    (*sTransitionData).WININ = WININ_WIN0_ALL;
    SetVBlankCallback(Some(VBlankCB_WhiteBarsFade_Blend));
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_End(task: *mut Task) -> u8 {
    if ({
        (*sTransitionData).BLDY += 1;
        (*sTransitionData).BLDY
    }) > 16
    {
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(Some(Task_WhiteBarsFade)));
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_WhiteBarsFade() {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    VBlankCB_BattleTransition();
    volatile_write(67108944 as usize as *mut u16, (*sTransitionData).BLDCNT);
    volatile_write(67108936 as usize as *mut u16, (*sTransitionData).WININ);
    volatile_write(67108938 as usize as *mut u16, (*sTransitionData).WINOUT);
    volatile_write(67108932 as usize as *mut u16, (*sTransitionData).WIN0V);
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x80000140);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(
                dmaRegs,
                &raw mut gScanlineEffectRegBuffers[1][160] as usize as u32,
            );
            volatile_write(dmaRegs.at(1), 67108928 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_WhiteBarsFade_Blend() {
    VBlankCB_BattleTransition();
    volatile_write(67108948 as usize as *mut u16, (*sTransitionData).BLDY);
    volatile_write(67108944 as usize as *mut u16, (*sTransitionData).BLDCNT);
    volatile_write(67108936 as usize as *mut u16, (*sTransitionData).WININ);
    volatile_write(67108938 as usize as *mut u16, (*sTransitionData).WINOUT);
    volatile_write(67108928 as usize as *mut u16, (*sTransitionData).WIN0H);
    volatile_write(67108932 as usize as *mut u16, (*sTransitionData).WIN0V);
}
pub(crate) unsafe extern "C" fn HBlankCB_WhiteBarsFade() {
    volatile_write(
        67108948 as usize as *mut u16,
        gScanlineEffectRegBuffers[1][(67108870 as usize as *mut u16).read_volatile()],
    );
}
pub(crate) unsafe extern "C" fn SpriteCB_WhiteBarFade(sprite: *mut Sprite) {
    if (*sprite).data[5] != 0 {
        (*sprite).data[5] -= 1;
        if (*sprite).data[6] != 0 {
            volatile_write(&raw mut (*sTransitionData).VBlank_DMA, 1);
        }
    } else {
        let mut i: u16 = 0;
        let mut ptr1: *mut u16 = &raw mut gScanlineEffectRegBuffers[0][(*sprite).y];
        let mut ptr2: *mut u16 =
            &raw mut gScanlineEffectRegBuffers[0][(*sprite).y as i32 + DISPLAY_HEIGHT as i32];
        i = 0;
        while i < 20 {
            *ptr1.at(i) = ((*sprite).data[0] >> 8) as u16;
            *ptr2.at(i) = (*sprite).x as u8 as u16;
            i += 1;
        }
        if (*sprite).x == 0 && (*sprite).data[0] == FADE_TARGET {
            (*sprite).data[1] = TRUE as i16;
        }
        (*sprite).x -= 16;
        (*sprite).data[0] += 128;
        if (*sprite).x < 0 {
            (*sprite).x = 0;
        }
        if (*sprite).data[0] > FADE_TARGET {
            (*sprite).data[0] = FADE_TARGET;
        }
        if (*sprite).data[6] != 0 {
            volatile_write(&raw mut (*sTransitionData).VBlank_DMA, 1);
        }
        if (*sprite).data[1] != 0 {
            if (*sprite).data[6] == 0
                || (*sTransitionData).counter >= 7
                    && ({
                        let t1 = (*sprite).data[2];
                        (*sprite).data[2] += 1;
                        t1
                    }) > 7
            {
                (*sTransitionData).counter += 1;
                DestroySprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_GridSquares(taskId: u8) {
    while sGridSquares_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn GridSquares_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    CpuSet(
        sShrinkingBoxTileset.as_ptr().cast_mut() as *mut c_void,
        tileset as *mut c_void,
        16,
    );
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 61440);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    LoadPalette(
        sFieldEffectPal_Pokeball.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn GridSquares_Main(task: *mut Task) -> u8 {
    let mut tileset: *mut u16 = null_mut();
    if (*task).data[1] == 0 {
        GetBg0TilemapDst(&raw mut tileset);
        (*task).data[1] = 3;
        (*task).data[2] += 1;
        CpuSet(
            (&raw const sShrinkingBoxTileset[(*task).data[2] as i32 * 8]).cast_mut() as *mut c_void,
            tileset as *mut c_void,
            16,
        );
        if (*task).data[2] > 13 {
            (*task).data[0] += 1;
            (*task).data[1] = 16;
        }
    }
    (*task).data[1] -= 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn GridSquares_End(task: *mut Task) -> u8 {
    if ({
        (*task).data[1] -= 1;
        (*task).data[1]
    }) == 0
    {
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(Some(Task_GridSquares)));
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_AngledWipes(taskId: u8) {
    while sAngledWipes_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn AngledWipes_Init(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    InitTransitionData();
    ScanlineEffect_Clear();
    (*sTransitionData).WININ = WININ_WIN0_ALL;
    (*sTransitionData).WINOUT = 0;
    (*sTransitionData).WIN0V = DISPLAY_HEIGHT;
    i = 0;
    while i < DISPLAY_HEIGHT {
        gScanlineEffectRegBuffers[0][i] = DISPLAY_WIDTH;
        i += 1;
    }
    CpuSet(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut c_void,
        gScanlineEffectRegBuffers[1].as_mut_ptr() as *mut c_void,
        DISPLAY_HEIGHT as u32,
    );
    SetVBlankCallback(Some(VBlankCB_AngledWipes));
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn AngledWipes_SetWipeData(task: *mut Task) -> u8 {
    InitBlackWipe(
        (*sTransitionData).data.as_mut_ptr(),
        sAngledWipes_MoveData[(*task).data[1]][0],
        sAngledWipes_MoveData[(*task).data[1]][1],
        sAngledWipes_MoveData[(*task).data[1]][2],
        sAngledWipes_MoveData[(*task).data[1]][3],
        1,
        1,
    );
    (*task).data[2] = sAngledWipes_MoveData[(*task).data[1]][4];
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn AngledWipes_DoWipe(task: *mut Task) -> u8 {
    let mut i: i16 = 0;
    let mut finished: u8 = 0;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, 0);
    i = 0;
    finished = 0;
    while i < 16 {
        let mut r3: i16 = (gScanlineEffectRegBuffers[0][(*sTransitionData).data[3]] >> 8) as i16;
        let mut r4: i16 = gScanlineEffectRegBuffers[0][(*sTransitionData).data[3]] as i16 & 0xFF;
        if (*task).data[2] == 0 {
            if r3 < (*sTransitionData).data[2] {
                r3 = (*sTransitionData).data[2];
            }
            if r3 > r4 {
                r3 = r4;
            }
        } else {
            if r4 > (*sTransitionData).data[2] {
                r4 = (*sTransitionData).data[2];
            }
            if r4 <= r3 {
                r4 = r3;
            }
        }
        gScanlineEffectRegBuffers[0][(*sTransitionData).data[3]] = r4 as u16 | (r3 as u16) << 8;
        if finished != 0 {
            (*task).data[0] += 1;
            break;
        }
        finished = UpdateBlackWipe((*sTransitionData).data.as_mut_ptr(), TRUE, TRUE);
        i += 1;
    }
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn AngledWipes_TryEnd(task: *mut Task) -> u8 {
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) < NUM_ANGLED_WIPES
    {
        (*task).data[0] += 1;
        (*task).data[3] = sAngledWipes_EndDelays[(*task).data[1] as i32 - 1];
        return TRUE;
    } else {
        {
            let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
            volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
            let _ = (dmaRegs.at(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(Some(Task_AngledWipes)));
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AngledWipes_StartNext(task: *mut Task) -> u8 {
    if ({
        (*task).data[3] -= 1;
        (*task).data[3]
    }) == 0
    {
        (*task).data[0] = 1;
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_AngledWipes() {
    {
        let mut dmaRegs: *mut u16 = 67109040 as usize as *mut u16;
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 50687);
        volatile_write(dmaRegs.at(5), (dmaRegs.at(5)).read_volatile() & 32767);
        let _ = (dmaRegs.at(5)).read_volatile();
    }
    VBlankCB_BattleTransition();
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    volatile_write(67108936 as usize as *mut u16, (*sTransitionData).WININ);
    volatile_write(67108938 as usize as *mut u16, (*sTransitionData).WINOUT);
    volatile_write(67108932 as usize as *mut u16, (*sTransitionData).WIN0V);
    volatile_write(
        67108928 as usize as *mut u16,
        gScanlineEffectRegBuffers[1][0],
    );
    {
        {
            let mut dmaRegs: *mut u32 = 67109040 as usize as *mut u32;
            volatile_write(
                dmaRegs,
                gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
            );
            volatile_write(dmaRegs.at(1), 67108928 as usize as *mut u16 as usize as u32);
            volatile_write(dmaRegs.at(2), B_TRANS_DMA_FLAGS);
            let _ = (dmaRegs.at(2)).read_volatile();
        }
    }
}
pub(crate) unsafe extern "C" fn CreateIntroTask(
    fadeToGrayDelay: i16,
    fadeFromGrayDelay: i16,
    numFades: i16,
    fadeToGrayIncrement: i16,
    fadeFromGrayIncrement: i16,
) {
    let mut taskId: u8 = CreateTask(Some(Task_BattleTransition_Intro), 3);
    gTasks[taskId].data[1] = fadeToGrayDelay;
    gTasks[taskId].data[2] = fadeFromGrayDelay;
    gTasks[taskId].data[3] = numFades;
    gTasks[taskId].data[4] = fadeToGrayIncrement;
    gTasks[taskId].data[5] = fadeFromGrayIncrement;
    gTasks[taskId].data[6] = fadeToGrayDelay;
}
pub(crate) unsafe extern "C" fn IsIntroTaskDone() -> u8 {
    if FindTaskIdByFunc(Some(Task_BattleTransition_Intro)) == TASK_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_BattleTransition_Intro(taskId: u8) {
    while sTransitionIntroFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn TransitionIntro_FadeToGray(task: *mut Task) -> u8 {
    if (*task).data[6] == 0
        || ({
            (*task).data[6] -= 1;
            (*task).data[6]
        }) == 0
    {
        (*task).data[6] = (*task).data[1];
        (*task).data[7] += (*task).data[4];
        if (*task).data[7] > 16 {
            (*task).data[7] = 16;
        }
        BlendPalettes(PALETTES_ALL, (*task).data[7] as u8, 11627);
    }
    if (*task).data[7] >= 16 {
        (*task).data[0] += 1;
        (*task).data[6] = (*task).data[2];
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn TransitionIntro_FadeFromGray(task: *mut Task) -> u8 {
    if (*task).data[6] == 0
        || ({
            (*task).data[6] -= 1;
            (*task).data[6]
        }) == 0
    {
        (*task).data[6] = (*task).data[2];
        (*task).data[7] -= (*task).data[5];
        if (*task).data[7] < 0 {
            (*task).data[7] = 0;
        }
        BlendPalettes(PALETTES_ALL, (*task).data[7] as u8, 11627);
    }
    if (*task).data[7] == 0 {
        if ({
            (*task).data[3] -= 1;
            (*task).data[3]
        }) == 0
        {
            DestroyTask(FindTaskIdByFunc(Some(Task_BattleTransition_Intro)));
        } else {
            (*task).data[6] = (*task).data[1];
            (*task).data[0] = 0;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn InitTransitionData() {
    memset(sTransitionData as *mut u8, 0, 60);
    GetCameraOffsetWithPan(
        &raw mut (*sTransitionData).cameraX,
        &raw mut (*sTransitionData).cameraY,
    );
}
pub(crate) unsafe extern "C" fn VBlankCB_BattleTransition() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn GetBg0TilemapDst(tileset: *mut *mut u16) {
    let mut charBase: u16 = (67108872 as usize as *mut u16).read_volatile() >> 2;
    charBase <<= 14;
    *tileset = (BG_VRAM + charBase as i32) as usize as *mut u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBg0TilesDst(tilemap: *mut *mut u16, tileset: *mut *mut u16) {
    let mut screenBase: u16 = (67108872 as usize as *mut u16).read_volatile() >> 8;
    let mut charBase: u16 = (67108872 as usize as *mut u16).read_volatile() >> 2;
    screenBase <<= 11;
    charBase <<= 14;
    *tilemap = (BG_VRAM + screenBase as i32) as usize as *mut u16;
    *tileset = (BG_VRAM + charBase as i32) as usize as *mut u16;
}
pub(crate) unsafe extern "C" fn FadeScreenBlack() {
    BlendPalettes(PALETTES_ALL, 16, 0);
}
pub(crate) unsafe extern "C" fn SetSinWave(
    mut array: *mut i16,
    sinAdd: i16,
    mut index: i16,
    indexIncrementer: i16,
    amplitude: i16,
    mut arrSize: i16,
) {
    let mut i: u8 = 0;
    i = 0;
    while arrSize > 0 {
        *array.at(i) = sinAdd + Sin(index & 0xFF, amplitude);
        arrSize -= 1;
        i += 1;
        index += indexIncrementer;
    }
}
pub(crate) unsafe extern "C" fn SetCircularMask(
    mut buffer: *mut u16,
    centerX: i16,
    centerY: i16,
    radius: i16,
) {
    let mut i: i16 = 0;
    memset(buffer as *mut u8, 10, 320);
    i = 0;
    while i < 64 {
        let mut sinResult: i16 = 0;
        let mut cosResult: i16 = 0;
        let mut drawXLeft: i16 = 0;
        let mut drawYBottNext: i16 = 0;
        let mut drawYTopNext: i16 = 0;
        let mut drawX: i16 = 0;
        let mut drawYTop: i16 = 0;
        let mut drawYBott: i16 = 0;
        sinResult = Sin(i, radius);
        cosResult = Cos(i, radius);
        drawXLeft = centerX - sinResult;
        drawX = centerX + sinResult;
        drawYTop = centerY - cosResult;
        drawYBott = centerY + cosResult;
        if drawXLeft < 0 {
            drawXLeft = 0;
        }
        if drawX > DISPLAY_WIDTH as i16 {
            drawX = DISPLAY_WIDTH as i16;
        }
        if drawYTop < 0 {
            drawYTop = 0;
        }
        if drawYBott > 159 {
            drawYBott = 159;
        }
        drawX |= drawXLeft << 8;
        *buffer.at(drawYTop) = drawX as u16;
        *buffer.at(drawYBott) = drawX as u16;
        cosResult = Cos(i + 1, radius);
        drawYTopNext = centerY - cosResult;
        drawYBottNext = centerY + cosResult;
        if drawYTopNext < 0 {
            drawYTopNext = 0;
        }
        if drawYBottNext > 159 {
            drawYBottNext = 159;
        }
        while drawYTop > drawYTopNext {
            *buffer.at({
                drawYTop -= 1;
                drawYTop
            }) = drawX as u16;
        }
        while drawYTop < drawYTopNext {
            *buffer.at({
                drawYTop += 1;
                drawYTop
            }) = drawX as u16;
        }
        while drawYBott > drawYBottNext {
            *buffer.at({
                drawYBott -= 1;
                drawYBott
            }) = drawX as u16;
        }
        while drawYBott < drawYBottNext {
            *buffer.at({
                drawYBott += 1;
                drawYBott
            }) = drawX as u16;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitBlackWipe(
    mut data: *mut i16,
    startX: i16,
    startY: i16,
    endX: i16,
    endY: i16,
    xMove: i16,
    yMove: i16,
) {
    *data = startX;
    *data.at(1) = startY;
    *data.at(2) = startX;
    *data.at(3) = startY;
    *data.at(4) = endX;
    *data.at(5) = endY;
    *data.at(6) = xMove;
    *data.at(7) = yMove;
    *data.at(8) = endX - startX;
    if *data.at(8) < 0 {
        *data.at(8) = -*data.at(8);
        *data.at(6) = -xMove;
    }
    *data.at(9) = endY - startY;
    if *data.at(9) < 0 {
        *data.at(9) = -*data.at(9);
        *data.at(7) = -yMove;
    }
    *data.at(10) = 0;
}
pub(crate) unsafe extern "C" fn UpdateBlackWipe(mut data: *mut i16, xExact: u8, yExact: u8) -> u8 {
    let mut numFinished: u8 = 0;
    if *data.at(8) > *data.at(9) {
        *data.at(2) += *data.at(6);
        *data.at(10) += *data.at(9);
        if *data.at(10) > *data.at(8) {
            *data.at(3) += *data.at(7);
            *data.at(10) -= *data.at(8);
        }
    } else {
        *data.at(3) += *data.at(7);
        *data.at(10) += *data.at(8);
        if *data.at(10) > *data.at(9) {
            *data.at(2) += *data.at(6);
            *data.at(10) -= *data.at(9);
        }
    }
    numFinished = 0;
    if *data.at(6) > 0 && *data.at(2) >= *data.at(4)
        || *data.at(6) < 0 && *data.at(2) <= *data.at(4)
    {
        numFinished += 1;
        if xExact != 0 {
            *data.at(2) = *data.at(4);
        }
    }
    if *data.at(7) > 0 && *data.at(3) >= *data.at(5)
        || *data.at(7) < 0 && *data.at(3) <= *data.at(5)
    {
        numFinished += 1;
        if yExact != 0 {
            *data.at(3) = *data.at(5);
        }
    }
    if numFinished == 2 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn FrontierLogoWiggle_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    InitPatternWeaveTransition(task);
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    LZ77UnCompVram(
        sFrontierLogo_Tileset.as_ptr().cast_mut(),
        tileset as *mut c_void,
    );
    LoadPalette(
        sFrontierLogo_Palette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierLogoWiggle_SetGfx(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LZ77UnCompVram(
        sFrontierLogo_Tilemap.as_ptr().cast_mut(),
        tilemap as *mut c_void,
    );
    SetSinWave(
        gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut i16,
        0,
        (*task).data[4],
        132,
        (*task).data[5],
        DISPLAY_HEIGHT as i16,
    );
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn Task_FrontierLogoWiggle(taskId: u8) {
    while sFrontierLogoWiggle_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn Task_FrontierLogoWave(taskId: u8) {
    while sFrontierLogoWave_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn FrontierLogoWave_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    InitTransitionData();
    ScanlineEffect_Clear();
    ClearGpuRegBits(REG_OFFSET_DISPCNT, 24576);
    (*task).data[2] = 8192;
    (*task).data[1] = 0x7FFF;
    (*task).data[5] = 0;
    (*task).data[6] = 16;
    (*task).data[7] = 2560;
    (*sTransitionData).BLDCNT = 16193;
    (*sTransitionData).BLDALPHA = ((*task).data[6] as u16) << 8 | (*task).data[5] as u16;
    volatile_write(67108944 as usize as *mut u16, (*sTransitionData).BLDCNT);
    volatile_write(67108946 as usize as *mut u16, (*sTransitionData).BLDALPHA);
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                tilemap as *mut c_void,
                0x1000400,
            );
        }
    }
    LZ77UnCompVram(
        sFrontierLogo_Tileset.as_ptr().cast_mut(),
        tileset as *mut c_void,
    );
    LoadPalette(
        sFrontierLogo_Palette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*sTransitionData).cameraY = 0;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierLogoWave_SetGfx(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LZ77UnCompVram(
        sFrontierLogo_Tilemap.as_ptr().cast_mut(),
        tilemap as *mut c_void,
    );
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn FrontierLogoWave_InitScanline(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < DISPLAY_HEIGHT as u8 {
        gScanlineEffectRegBuffers[1][i] = (*sTransitionData).cameraY as u16;
        i += 1;
    }
    SetVBlankCallback(Some(VBlankCB_FrontierLogoWave));
    SetHBlankCallback(Some(HBlankCB_FrontierLogoWave));
    EnableInterrupts(INTR_FLAG_HBLANK);
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn FrontierLogoWave_Main(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    let mut sinVal: u16 = 0;
    let mut amplitude: u16 = 0;
    let mut sinSpread: u16 = 0;
    volatile_write(&raw mut (*sTransitionData).VBlank_DMA, FALSE);
    amplitude = ((*task).data[2] >> 8) as u16;
    sinVal = (*task).data[1] as u16;
    sinSpread = 384;
    (*task).data[1] -= (*task).data[7];
    if (*task).data[3] >= 70 {
        if (*task).data[2] as i32 - 384 >= 0 {
            (*task).data[2] -= 384;
        } else {
            (*task).data[2] = 0;
        }
    }
    if (*task).data[3] >= 0 && (*task).data[3] % 3 == 0 {
        if (*task).data[5] < 16 {
            (*task).data[5] += 1;
        } else if (*task).data[6] > 0 {
            (*task).data[6] -= 1;
        }
        (*sTransitionData).BLDALPHA = ((*task).data[6] as u16) << 8 | (*task).data[5] as u16;
    }
    i = 0;
    while i < DISPLAY_HEIGHT as u8 {
        let mut index: i16 = (sinVal as i32 / 256) as i16;
        gScanlineEffectRegBuffers[0][i] =
            (*sTransitionData).cameraY as u16 + Sin(index & 0xff, amplitude as i16) as u16;
        i += 1;
        sinVal += sinSpread;
    }
    if ({
        (*task).data[3] += 1;
        (*task).data[3]
    }) == 101
    {
        (*task).data[4] += 1;
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    }
    if (*task).data[4] != 0 && gPaletteFade.active() == 0 {
        DestroyTask(FindTaskIdByFunc(Some(Task_FrontierLogoWave)));
    }
    (*task).data[7] -= 17;
    volatile_write(
        &raw mut (*sTransitionData).VBlank_DMA,
        (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() + 1,
    );
    return FALSE;
}
pub(crate) unsafe extern "C" fn VBlankCB_FrontierLogoWave() {
    VBlankCB_BattleTransition();
    volatile_write(67108944 as usize as *mut u16, (*sTransitionData).BLDCNT);
    volatile_write(67108946 as usize as *mut u16, (*sTransitionData).BLDALPHA);
    if (&raw mut (*sTransitionData).VBlank_DMA).read_volatile() != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        gScanlineEffectRegBuffers[0].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        gScanlineEffectRegBuffers[1].as_mut_ptr() as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_FrontierLogoWave() {
    let mut var: u16 =
        gScanlineEffectRegBuffers[1][(67108870 as usize as *mut u16).read_volatile()];
    volatile_write(67108882 as usize as *mut u16, var);
}
pub(crate) unsafe extern "C" fn Task_FrontierSquares(taskId: u8) {
    while sFrontierSquares_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn Task_FrontierSquaresSpiral(taskId: u8) {
    while sFrontierSquaresSpiral_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn Task_FrontierSquaresScroll(taskId: u8) {
    while sFrontierSquaresScroll_Funcs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn FrontierSquares_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LZ77UnCompVram(
        sFrontierSquares_FilledBg_Tileset.as_ptr().cast_mut(),
        tileset as *mut c_void,
    );
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 32, 32);
    FillBgTilemapBufferRect(0, 1, 0, 0, 1, 32, 15);
    FillBgTilemapBufferRect(0, 1, 29, 0, 1, 32, 15);
    CopyBgTilemapBufferToVram(0);
    LoadPalette(
        sFrontierSquares_Palette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*task).data[2] = MARGIN_SIZE as i16;
    (*task).data[3] = 0;
    (*task).data[4] = 0;
    (*task).data[7] = 10;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquares_Draw(task: *mut Task) -> u8 {
    CopyRectToBgTilemapBufferRect(
        0,
        sFrontierSquares_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        SQUARE_SIZE,
        SQUARE_SIZE,
        (*task).data[2] as u8,
        (*task).data[3] as u8,
        SQUARE_SIZE,
        SQUARE_SIZE,
        15,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(0);
    (*task).data[2] += SQUARE_SIZE as i16;
    if ({
        (*task).data[4] += 1;
        (*task).data[4]
    }) == NUM_SQUARES_PER_ROW
    {
        (*task).data[2] = MARGIN_SIZE as i16;
        (*task).data[3] += SQUARE_SIZE as i16;
        (*task).data[4] = 0;
        if (*task).data[3] >= 20 {
            (*task).data[0] += 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquares_Shrink(task: *mut Task) -> u8 {
    let mut i: u8 = 0;
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    if ({
        let t1 = (*task).data[6];
        (*task).data[6] += 1;
        t1
    }) >= (*task).data[7]
    {
        match (*task).data[5] {
            0 => {
                i = 250;
                while i < 255 {
                    gPlttBufferUnfaded[i] = 0;
                    gPlttBufferFaded[i] = 0;
                    i += 1;
                }
            }
            1 => {
                BlendPalettes(0xffff7fff, 16, 0);
                LZ77UnCompVram(
                    sFrontierSquares_EmptyBg_Tileset.as_ptr().cast_mut(),
                    tileset as *mut c_void,
                );
            }
            2 => {
                LZ77UnCompVram(
                    sFrontierSquares_Shrink1_Tileset.as_ptr().cast_mut(),
                    tileset as *mut c_void,
                );
            }
            3 => {
                LZ77UnCompVram(
                    sFrontierSquares_Shrink2_Tileset.as_ptr().cast_mut(),
                    tileset as *mut c_void,
                );
            }
            _ => {
                FillBgTilemapBufferRect_Palette0(0, 1, 0, 0, 32, 32);
                CopyBgTilemapBufferToVram(0);
                (*task).data[0] += 1;
                return FALSE;
            }
        }
        (*task).data[6] = 0;
        (*task).data[5] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquaresSpiral_Init(task: *mut Task) -> u8 {
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LZ77UnCompVram(
        sFrontierSquares_FilledBg_Tileset.as_ptr().cast_mut(),
        tileset as *mut c_void,
    );
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 32, 32);
    FillBgTilemapBufferRect(0, 1, 0, 0, 1, 32, 15);
    FillBgTilemapBufferRect(0, 1, 29, 0, 1, 32, 15);
    CopyBgTilemapBufferToVram(0);
    LoadPalette(
        sFrontierSquares_Palette.as_ptr().cast_mut() as *mut c_void,
        224,
        32,
    );
    LoadPalette(
        sFrontierSquares_Palette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    BlendPalette(224, 16, 8, 0);
    (*task).data[2] = 34;
    (*task).data[3] = 0;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquaresSpiral_Outward(task: *mut Task) -> u8 {
    let mut pos: u8 = sFrontierSquaresSpiral_Positions[(*task).data[2]];
    let mut x: u8 = (pos as i32 % 7) as u8;
    let mut y: u8 = (pos as i32 / 7) as u8;
    CopyRectToBgTilemapBufferRect(
        0,
        sFrontierSquares_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        SQUARE_SIZE,
        SQUARE_SIZE,
        SQUARE_SIZE * x + MARGIN_SIZE,
        SQUARE_SIZE * y,
        SQUARE_SIZE,
        SQUARE_SIZE,
        15,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(0);
    if ({
        (*task).data[2] -= 1;
        (*task).data[2]
    }) < 0
    {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquaresSpiral_SetBlack(task: *mut Task) -> u8 {
    BlendPalette(224, 16, 3, 0);
    BlendPalettes(0xffff3fff, 16, 0);
    (*task).data[2] = 0;
    (*task).data[3] = 0;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquaresSpiral_Inward(task: *mut Task) -> u8 {
    if ({
        (*task).data[3] ^= 1;
        (*task).data[3]
    }) != 0
    {
        CopyRectToBgTilemapBufferRect(
            0,
            sFrontierSquares_Tilemap.as_ptr().cast_mut() as *mut c_void,
            0,
            0,
            SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE * (sFrontierSquaresSpiral_Positions[(*task).data[2]] as i32 % 7) as u8
                + MARGIN_SIZE,
            SQUARE_SIZE * (sFrontierSquaresSpiral_Positions[(*task).data[2]] as i32 / 7) as u8,
            SQUARE_SIZE,
            SQUARE_SIZE,
            14,
            0,
            0,
        );
    } else {
        if (*task).data[2] > 0 {
            FillBgTilemapBufferRect(
                0,
                1,
                SQUARE_SIZE
                    * (sFrontierSquaresSpiral_Positions[(*task).data[2] as i32 - 1] as i32 % 7)
                        as u8
                    + 1,
                SQUARE_SIZE
                    * (sFrontierSquaresSpiral_Positions[(*task).data[2] as i32 - 1] as i32 / 7)
                        as u8,
                SQUARE_SIZE,
                SQUARE_SIZE,
                15,
            );
        }
        (*task).data[2] += 1;
    }
    if (*task).data[2] >= NUM_SQUARES {
        (*task).data[0] += 1;
    }
    CopyBgTilemapBufferToVram(0);
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquares_End(task: *mut Task) -> u8 {
    FillBgTilemapBufferRect_Palette0(0, 1, 0, 0, 32, 32);
    CopyBgTilemapBufferToVram(0);
    BlendPalettes(PALETTES_ALL, 16, 0);
    DestroyTask(FindTaskIdByFunc((*task).func));
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_ScrollBg(taskId: u8) {
    if ({
        gTasks[taskId].data[2] ^= 1;
        gTasks[taskId].data[2]
    }) == 0
    {
        SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_X);
        SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_Y);
        gBattle_BG0_X += gTasks[taskId].data[0] as u16;
        gBattle_BG0_Y += gTasks[taskId].data[1] as u16;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_Init(task: *mut Task) -> u8 {
    let mut taskId: u8 = 0;
    let mut tilemap: *mut u16 = null_mut();
    let mut tileset: *mut u16 = null_mut();
    GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
    LZ77UnCompVram(
        sFrontierSquares_FilledBg_Tileset.as_ptr().cast_mut(),
        tileset as *mut c_void,
    );
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 32, 32);
    CopyBgTilemapBufferToVram(0);
    LoadPalette(
        sFrontierSquares_Palette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_X);
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_Y);
    (*task).data[2] = 0;
    taskId = CreateTask(Some(Task_ScrollBg), 1);
    match Random() as i32 % 4 {
        0 => {
            gTasks[taskId].data[0] = 1;
            gTasks[taskId].data[1] = 1;
        }
        1 => {
            gTasks[taskId].data[0] = -1;
            gTasks[taskId].data[1] = -1;
        }
        2 => {
            gTasks[taskId].data[0] = 1;
            gTasks[taskId].data[1] = -1;
        }
        _ => {
            gTasks[taskId].data[0] = -1;
            gTasks[taskId].data[1] = 1;
        }
    }
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_Draw(task: *mut Task) -> u8 {
    let mut pos: u8 = sFrontierSquaresScroll_Positions[(*task).data[2]];
    let mut x: u8 = (pos as i32 / 8) as u8;
    let mut y: u8 = (pos as i32 % 8) as u8;
    CopyRectToBgTilemapBufferRect(
        0,
        (&raw const *sFrontierSquares_Tilemap).cast_mut() as *mut c_void,
        0,
        0,
        SQUARE_SIZE,
        SQUARE_SIZE,
        SQUARE_SIZE * x + MARGIN_SIZE,
        SQUARE_SIZE * y,
        SQUARE_SIZE,
        SQUARE_SIZE,
        15,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(0);
    if ({
        (*task).data[2] += 1;
        (*task).data[2]
    }) >= 64
    {
        (*task).data[0] += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_SetBlack(task: *mut Task) -> u8 {
    BlendPalettes(0xffff7fff, 16, 0);
    (*task).data[2] = 0;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_Erase(task: *mut Task) -> u8 {
    let mut pos: u8 = sFrontierSquaresScroll_Positions[(*task).data[2]];
    let mut x: u8 = (pos as i32 / 8) as u8;
    let mut y: u8 = (pos as i32 % 8) as u8;
    FillBgTilemapBufferRect(
        0,
        1,
        SQUARE_SIZE * x + MARGIN_SIZE,
        SQUARE_SIZE * y,
        SQUARE_SIZE,
        SQUARE_SIZE,
        15,
    );
    CopyBgTilemapBufferToVram(0);
    if ({
        (*task).data[2] += 1;
        (*task).data[2]
    }) >= 64
    {
        DestroyTask(FindTaskIdByFunc(Some(Task_ScrollBg)));
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_End(task: *mut Task) -> u8 {
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_Y);
    FillBgTilemapBufferRect_Palette0(0, 1, 0, 0, 32, 32);
    CopyBgTilemapBufferToVram(0);
    BlendPalettes(PALETTES_ALL, 16, 0);
    DestroyTask(FindTaskIdByFunc((*task).func));
    (*task).data[0] += 1;
    return FALSE;
}
