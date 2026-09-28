//! Translated from `src/roulette.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sWheel_Pal sGrid_Tilemap sWheel_Tilemap sBgTemplates sWindowTemplates sGridSelections sRouletteSlots sTableMinBets sRouletteTables sFlashData_Colors sFlashData_PokeIcons sYesNoTable_AcceptMinBet sYesNoTable_KeepPlaying sFiller sShadow_Pal sBall_Pal sBallCounter_Pal sCursor_Pal sCredit_Pal sShroomish_Pal sTaillow_Pal sGridIcons_Pal sWynaut_Pal sAzurill_Pal sSkitty_Pal sMakuhita_Pal sUnused1_Pal sUnused2_Pal sUnused3_Pal sUnused4_Pal sBall_Gfx sBallCounter_Gfx sShroomishTaillow_Gfx sGridIcons_Gfx sWheelIcons_Gfx sShadow_Gfx sCursor_Gfx sSpritePalettes sOam_GridHeader sOam_GridIcon sOam_WheelIcon sAffineAnim_Unused1 sAffineAnims_Unused1 sAffineAnim_Unused2 sAffineAnims_Unused2 sSpriteSheet_WheelIcons sAnim_WheelIcons sAnim_WheelIcon_OrangeWynaut sAnim_WheelIcon_GreenAzurill sAnim_WheelIcon_PurpleSkitty sAnim_WheelIcon_OrangeMakuhita sAnim_WheelIcon_GreenWynaut sAnim_WheelIcon_PurpleAzurill sAnim_WheelIcon_OrangeSkitty sAnim_WheelIcon_GreenMakuhita sAnim_WheelIcon_PurpleWynaut sAnim_WheelIcon_OrangeAzurill sAnim_WheelIcon_GreenSkitty sAnim_WheelIcon_PurpleMakuhita sSpriteSheet_Headers sSpriteSheet_GridIcons sAnim_Headers sAnim_GridIcons sAnim_WynautHeader sAnim_AzurillHeader sAnim_SkittyHeader sAnim_MakuhitaHeader sAnim_OrangeHeader sAnim_GreenHeader sAnim_PurpleHeader sAnim_GridIcon_Wynaut sAnim_GridIcon_Azurill sAnim_GridIcon_Skitty sAnim_GridIcon_Makuhita sSpriteTemplates_PokeHeaders sSpriteTemplates_ColorHeaders sSpriteTemplates_GridIcons sSpriteTemplates_WheelIcons sOam_Credit sOam_CreditDigit sOam_Multiplier sOam_BallCounter sSpriteSheets_Interface sAnim_CreditDigit sAnims_CreditDigit sAnim_Multiplier sAnims_Multiplier sAnim_BallCounter sAnims_BallCounter sSpriteTemplate_Credit sSpriteTemplate_CreditDigit sSpriteTemplate_Multiplier sSpriteTemplate_BallCounter sSpriteTemplate_Cursor sOam_Ball sSpriteSheet_Ball sAnim_Ball_RollFast sAnim_Ball_RollMedium sAnim_Ball_RollSlow sAnim_Ball_StopOnFrame1 sAnim_Ball_StopOnFrame3 sAnim_Ball_StopOnFrame4 sAnim_Ball_Still sAnim_Ball_StopOnFrame2 sAnims_Ball sSpriteTemplate_Ball sOam_WheelCenter sSpriteSheet_WheelCenter sSpriteTemplate_WheelCenter sOam_Shroomish sOam_Taillow sSpriteSheet_ShroomishTaillow sAnim_Shroomish sAnim_Taillow_WingDown_Left sAnim_Taillow_WingDown_Right sAnim_Taillow_FlapSlow_Left sAnim_Taillow_FlapSlow_Right sAnim_Taillow_FlapFast_Left sAnim_Taillow_FlapFast_Right sAnims_Shroomish sAnims_Taillow sSpriteTemplate_Shroomish sSpriteTemplate_Taillow sOam_ShroomishBallShadow sOam_ShroomishShadow sOam_TaillowShadow sSpriteSheet_Shadow sAffineAnim_Unused3 sAffineAnim_TaillowShadow sAffineAnims_Unused3 sAffineAnims_TaillowShadow sAffineAnim_Unused4 sAffineAnims_Unused4 sAnim_ShroomishBallShadow sAnim_UnstickMonShadow sAnims_ShroomishBallShadow sAnims_UnstickMonShadow sSpriteTemplate_ShroomishShadow sSpriteTemplate_TaillowShadow sShroomishShadowAlphas

/// `struct Roulette`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Roulette {
    pub unk0: u8,
    pub shroomishShadowTimer: u8,
    pub partySpeciesFlags: u8,
    bits_3: u8,
    bits_4: u8,
    pub hitFlags: u32,
    pub hitSquares: CArray<u8, 6>,
    pub pokeHits: CArray<u8, 4>,
    pub colorHits: CArray<u8, 3>,
    pub minBet: u8,
    bits_26: u8,
    pub betSelection: CArray<u8, 6>,
    pub wheelDelayTimer: u8,
    pub wheelSpeed: u8,
    pub wheelDelay: u8,
    pub wheelAngle: i16,
    pub gridX: i16,
    pub selectionRectDrawState: i16,
    pub updateGridHighlight: i16,
    pub wheelRotation: OamMatrix,
    pub shroomishShadowAlpha: u16,
    pub ball: *mut Sprite,
    pub spriteIds: CArray<u8, 64>,
    pub curBallSpriteId: u8,
    pub ballState: u8,
    pub hitSlot: u8,
    pub stuckHitSlot: u8,
    pub ballTravelDist: i16,
    pub ballTravelDistFast: i16,
    pub ballTravelDistMed: u16,
    pub ballTravelDistSlow: u16,
    pub ballAngle: f32,
    pub ballAngleSpeed: f32,
    pub ballAngleAccel: f32,
    pub ballDistToCenter: f32,
    pub ballFallSpeed: f32,
    pub ballFallAccel: f32,
    pub varA0: f32,
    pub playTaskId: u8,
    pub spinTaskId: u8,
    pub filler_1: CArray<u8, 2>,
    pub taskWaitDelay: u16,
    pub taskWaitKey: u16,
    pub nextTask: Option<unsafe extern "C" fn(u8)>,
    pub filler_2: CArray<u8, 4>,
    pub prevTask: Option<unsafe extern "C" fn(u8)>,
    pub flashUtil: RouletteFlashUtil,
    pub tilemapBuffers: CArray<CArray<u16, 1024>, 7>,
    pub gridTilemap: *mut u16,
}

impl Roulette {
    #[inline(always)]
    pub fn useTaillow(&self) -> u8 {
        ((self.bits_3 as u32 >> 0) & 0x1f) as u8
    }
    #[inline(always)]
    pub fn set_useTaillow(&mut self, v: u8) {
        self.bits_3 = (self.bits_3 & !(0x1f << 0)) | ((v as u8 & 0x1f) << 0);
    }
    #[inline(always)]
    pub fn ballStuck(&self) -> u8 {
        ((self.bits_3 as u32 >> 5) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_ballStuck(&mut self, v: u8) {
        self.bits_3 = (self.bits_3 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn ballUnstuck(&self) -> u8 {
        ((self.bits_3 as u32 >> 6) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_ballUnstuck(&mut self, v: u8) {
        self.bits_3 = (self.bits_3 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn ballRolling(&self) -> u8 {
        ((self.bits_3 as u32 >> 7) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_ballRolling(&mut self, v: u8) {
        self.bits_3 = (self.bits_3 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn tableId(&self) -> u8 {
        ((self.bits_4 as u32 >> 0) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_tableId(&mut self, v: u8) {
        self.bits_4 = (self.bits_4 & !(0x3 << 0)) | ((v as u8 & 0x3) << 0);
    }
    #[inline(always)]
    pub fn unused(&self) -> u8 {
        ((self.bits_4 as u32 >> 2) & 0x1f) as u8
    }
    #[inline(always)]
    pub fn set_unused(&mut self, v: u8) {
        self.bits_4 = (self.bits_4 & !(0x1f << 2)) | ((v as u8 & 0x1f) << 2);
    }
    #[inline(always)]
    pub fn isSpecialRate(&self) -> u8 {
        ((self.bits_4 as u32 >> 7) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_isSpecialRate(&mut self, v: u8) {
        self.bits_4 = (self.bits_4 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn curBallNum(&self) -> u8 {
        ((self.bits_26 as u32 >> 0) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_curBallNum(&mut self, v: u8) {
        self.bits_26 = (self.bits_26 & !(0xf << 0)) | ((v as u8 & 0xf) << 0);
    }
    #[inline(always)]
    pub fn unk1(&self) -> u8 {
        ((self.bits_26 as u32 >> 4) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_unk1(&mut self, v: u8) {
        self.bits_26 = (self.bits_26 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
}

unsafe impl Sync for Roulette {}

/// `struct GridSelection`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GridSelection {
    pub spriteIdOffset: u8,
    bits_1: u8,
    pub row: u8,
    pub x: u8,
    pub y: u8,
    pub var05: u8,
    pub tilemapOffset: u8,
    pub flag: u32,
    pub inSelectionFlags: u32,
    pub flashFlags: u16,
}

impl GridSelection {
    #[inline(always)]
    pub fn baseMultiplier(&self) -> u8 {
        ((self.bits_1 as u32 >> 0) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_baseMultiplier(&mut self, v: u8) {
        self.bits_1 = (self.bits_1 & !(0xf << 0)) | ((v as u8 & 0xf) << 0);
    }
    #[inline(always)]
    pub fn column(&self) -> u8 {
        ((self.bits_1 as u32 >> 4) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_column(&mut self, v: u8) {
        self.bits_1 = (self.bits_1 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
}

unsafe impl Sync for GridSelection {}

/// `struct RouletteSlot`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RouletteSlot {
    pub id1: u8,
    pub id2: u8,
    pub gridSquare: u8,
    pub flag: u32,
}

unsafe impl Sync for RouletteSlot {}

/// `struct RouletteTable`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RouletteTable {
    pub minBet: u8,
    pub randDistanceHigh: u8,
    pub randDistanceLow: u8,
    pub wheelSpeed: u8,
    pub wheelDelay: u8,
    pub shroomish: Shroomish,
    pub taillow: Taillow,
    pub ballSpeed: u16,
    pub baseTravelDist: u16,
    pub var1C: f32,
}

unsafe impl Sync for RouletteTable {}

/// `struct Shroomish`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct Shroomish {
    pub startAngle: u16,
    pub dropAngle: u16,
    pub fallSlowdown: u16,
}

unsafe impl Sync for Shroomish {}

/// `struct Taillow`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct Taillow {
    pub baseDropDelay: u16,
    pub rightStartAngle: u16,
    pub leftStartAngle: u16,
}

unsafe impl Sync for Taillow {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Roulette>() == 14720);
    assert!(offset_of!(Roulette, unk0) == 0);
    assert!(offset_of!(Roulette, shroomishShadowTimer) == 1);
    assert!(offset_of!(Roulette, partySpeciesFlags) == 2);
    assert!(offset_of!(Roulette, bits_3) == 3);
    assert!(offset_of!(Roulette, bits_4) == 4);
    assert!(offset_of!(Roulette, hitFlags) == 8);
    assert!(offset_of!(Roulette, hitSquares) == 12);
    assert!(offset_of!(Roulette, pokeHits) == 18);
    assert!(offset_of!(Roulette, colorHits) == 22);
    assert!(offset_of!(Roulette, minBet) == 25);
    assert!(offset_of!(Roulette, bits_26) == 26);
    assert!(offset_of!(Roulette, betSelection) == 27);
    assert!(offset_of!(Roulette, wheelDelayTimer) == 33);
    assert!(offset_of!(Roulette, wheelSpeed) == 34);
    assert!(offset_of!(Roulette, wheelDelay) == 35);
    assert!(offset_of!(Roulette, wheelAngle) == 36);
    assert!(offset_of!(Roulette, gridX) == 38);
    assert!(offset_of!(Roulette, selectionRectDrawState) == 40);
    assert!(offset_of!(Roulette, updateGridHighlight) == 42);
    assert!(offset_of!(Roulette, wheelRotation) == 44);
    assert!(offset_of!(Roulette, shroomishShadowAlpha) == 52);
    assert!(offset_of!(Roulette, ball) == 56);
    assert!(offset_of!(Roulette, spriteIds) == 60);
    assert!(offset_of!(Roulette, curBallSpriteId) == 124);
    assert!(offset_of!(Roulette, ballState) == 125);
    assert!(offset_of!(Roulette, hitSlot) == 126);
    assert!(offset_of!(Roulette, stuckHitSlot) == 127);
    assert!(offset_of!(Roulette, ballTravelDist) == 128);
    assert!(offset_of!(Roulette, ballTravelDistFast) == 130);
    assert!(offset_of!(Roulette, ballTravelDistMed) == 132);
    assert!(offset_of!(Roulette, ballTravelDistSlow) == 134);
    assert!(offset_of!(Roulette, ballAngle) == 136);
    assert!(offset_of!(Roulette, ballAngleSpeed) == 140);
    assert!(offset_of!(Roulette, ballAngleAccel) == 144);
    assert!(offset_of!(Roulette, ballDistToCenter) == 148);
    assert!(offset_of!(Roulette, ballFallSpeed) == 152);
    assert!(offset_of!(Roulette, ballFallAccel) == 156);
    assert!(offset_of!(Roulette, varA0) == 160);
    assert!(offset_of!(Roulette, playTaskId) == 164);
    assert!(offset_of!(Roulette, spinTaskId) == 165);
    assert!(offset_of!(Roulette, filler_1) == 166);
    assert!(offset_of!(Roulette, taskWaitDelay) == 168);
    assert!(offset_of!(Roulette, taskWaitKey) == 170);
    assert!(offset_of!(Roulette, nextTask) == 172);
    assert!(offset_of!(Roulette, filler_2) == 176);
    assert!(offset_of!(Roulette, prevTask) == 180);
    assert!(offset_of!(Roulette, flashUtil) == 184);
    assert!(offset_of!(Roulette, tilemapBuffers) == 380);
    assert!(offset_of!(Roulette, gridTilemap) == 14716);
    assert!(size_of::<GridSelection>() == 20);
    assert!(offset_of!(GridSelection, spriteIdOffset) == 0);
    assert!(offset_of!(GridSelection, bits_1) == 1);
    assert!(offset_of!(GridSelection, row) == 2);
    assert!(offset_of!(GridSelection, x) == 3);
    assert!(offset_of!(GridSelection, y) == 4);
    assert!(offset_of!(GridSelection, var05) == 5);
    assert!(offset_of!(GridSelection, tilemapOffset) == 6);
    assert!(offset_of!(GridSelection, flag) == 8);
    assert!(offset_of!(GridSelection, inSelectionFlags) == 12);
    assert!(offset_of!(GridSelection, flashFlags) == 16);
    assert!(size_of::<RouletteSlot>() == 8);
    assert!(offset_of!(RouletteSlot, id1) == 0);
    assert!(offset_of!(RouletteSlot, id2) == 1);
    assert!(offset_of!(RouletteSlot, gridSquare) == 2);
    assert!(offset_of!(RouletteSlot, flag) == 4);
    assert!(size_of::<RouletteTable>() == 32);
    assert!(offset_of!(RouletteTable, minBet) == 0);
    assert!(offset_of!(RouletteTable, randDistanceHigh) == 1);
    assert!(offset_of!(RouletteTable, randDistanceLow) == 2);
    assert!(offset_of!(RouletteTable, wheelSpeed) == 3);
    assert!(offset_of!(RouletteTable, wheelDelay) == 4);
    assert!(offset_of!(RouletteTable, shroomish) == 8);
    assert!(offset_of!(RouletteTable, taillow) == 16);
    assert!(offset_of!(RouletteTable, ballSpeed) == 24);
    assert!(offset_of!(RouletteTable, baseTravelDist) == 26);
    assert!(offset_of!(RouletteTable, var1C) == 28);
    assert!(size_of::<Shroomish>() == 8);
    assert!(offset_of!(Shroomish, startAngle) == 0);
    assert!(offset_of!(Shroomish, dropAngle) == 2);
    assert!(offset_of!(Shroomish, fallSlowdown) == 4);
    assert!(size_of::<Taillow>() == 8);
    assert!(offset_of!(Taillow, baseDropDelay) == 0);
    assert!(offset_of!(Taillow, rightStartAngle) == 2);
    assert!(offset_of!(Taillow, leftStartAngle) == 4);
};

const BALLS_PER_ROUND: u8 = 6;
const BALL_STATE_LANDED: u8 = 255;
const BALL_STATE_ROLLING: u8 = 0;
const BALL_STATE_STUCK: u8 = 1;
const COL_AZURILL: u8 = 2;
const COL_MAKUHITA: u8 = 4;
const COL_SKITTY: u8 = 3;
const COL_WYNAUT: u8 = 1;
const DEGREES_PER_SLOT: u16 = 30;
const FLASH_ICON: i32 = 13;
const FLASH_ICON_2: i32 = 14;
const FLASH_ICON_3: i32 = 15;
const F_FLASH_ICON: i32 = 8192;
const F_FLASH_OUTER_EDGES: u16 = 4096;
const F_ORANGE_ROW: u32 = 32;
const GFXTAG_BALL: u16 = 12;
const GFXTAG_SHADOW: u16 = 14;
const GFXTAG_SHROOMISH_TAILLOW: u16 = 13;
const HAS_SHROOMISH: u8 = 1;
const HAS_TAILLOW: u8 = 2;
const MAX_MULTIPLIER: i16 = 12;
const NO_DELAY: u16 = 65535;
const NUM_BOARD_COLORS: u8 = 3;
const NUM_BOARD_POKES: u8 = 4;
const NUM_GRID_SELECTIONS: u8 = 19;
const NUM_ROULETTE_SLOTS: u8 = 12;
const ROW_GREEN: u8 = 10;
const ROW_ORANGE: u8 = 5;
const ROW_PURPLE: u8 = 15;
const SELECTION_NONE: u8 = 0;
const SELECT_STATE_DRAW: i16 = 1;
const SELECT_STATE_ERASE: i16 = 255;
const SELECT_STATE_UPDATE: i16 = 2;
const SELECT_STATE_WAIT: i16 = 0;
const SLOT_MIDPOINT: i16 = 14;
const SPR_BALL_COUNTER_1: i32 = 26;
const SPR_BALL_COUNTER_2: i32 = 27;
const SPR_BALL_COUNTER_3: i32 = 28;
const SPR_CLEAR_MON: i32 = 55;
const SPR_CLEAR_MON_SHADOW_1: i32 = 56;
const SPR_CLEAR_MON_SHADOW_2: i32 = 57;
const SPR_COLOR_HEADER_1: i32 = 45;
const SPR_CREDIT: i32 = 20;
const SPR_CREDIT_DIG_1: i32 = 21;
const SPR_GRID_BALL_1: i32 = 49;
const SPR_GRID_ICON_ORANGE_WYNAUT: i32 = 29;
const SPR_MULTIPLIER: i32 = 25;
const SPR_POKE_HEADER_1: i32 = 41;
const SPR_WHEEL_BALL_1: i32 = 0;
const SPR_WHEEL_ICON_GREEN_AZURILL: i32 = 8;
const SPR_WHEEL_ICON_ORANGE_MAKUHITA: i32 = 10;
const SPR_WHEEL_ICON_ORANGE_WYNAUT: i32 = 7;
const SPR_WHEEL_ICON_PURPLE_SKITTY: i32 = 9;
const SPR_WIN_SLOT_CURSOR: i32 = 48;
const SQU_GREEN_MAKUHITA: i16 = 14;
const SQU_GREEN_WYNAUT: i16 = 11;
const SQU_ORANGE_MAKUHITA: i16 = 9;
const SQU_ORANGE_WYNAUT: i16 = 6;

static sBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::roulette::sBgTemplates).cast());
static sFlashData_Colors: Table<CArray<RouletteFlashSettings, 13>> =
    Table((&raw const crate::data::roulette::sFlashData_Colors).cast());
static sFlashData_PokeIcons: Table<CArray<RouletteFlashSettings, 3>> =
    Table((&raw const crate::data::roulette::sFlashData_PokeIcons).cast());
static sGridSelections: Table<CArray<GridSelection, 20>> =
    Table((&raw const crate::data::roulette::sGridSelections).cast());
static sGrid_Tilemap: Table<CArray<u32, 105>> =
    Table((&raw const crate::data::roulette::sGrid_Tilemap).cast());
static sRouletteSlots: Table<CArray<RouletteSlot, 12>> =
    Table((&raw const crate::data::roulette::sRouletteSlots).cast());
static sRouletteTables: Table<CArray<RouletteTable, 2>> =
    Table((&raw const crate::data::roulette::sRouletteTables).cast());
static sShroomishShadowAlphas: Table<CArray<u16, 10>> =
    Table((&raw const crate::data::roulette::sShroomishShadowAlphas).cast());
static sSpritePalettes: Table<CArray<SpritePalette, 13>> =
    Table((&raw const crate::data::roulette::sSpritePalettes).cast());
static sSpriteSheet_Ball: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::roulette::sSpriteSheet_Ball).cast());
static sSpriteSheet_GridIcons: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::roulette::sSpriteSheet_GridIcons).cast());
static sSpriteSheet_Headers: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::roulette::sSpriteSheet_Headers).cast());
static sSpriteSheet_Shadow: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::roulette::sSpriteSheet_Shadow).cast());
static sSpriteSheet_ShroomishTaillow: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::roulette::sSpriteSheet_ShroomishTaillow).cast());
static sSpriteSheet_WheelCenter: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::roulette::sSpriteSheet_WheelCenter).cast());
static sSpriteSheet_WheelIcons: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::roulette::sSpriteSheet_WheelIcons).cast());
static sSpriteSheets_Interface: Table<CArray<CompressedSpriteSheet, 6>> =
    Table((&raw const crate::data::roulette::sSpriteSheets_Interface).cast());
static sSpriteTemplate_Ball: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_Ball).cast());
static sSpriteTemplate_BallCounter: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_BallCounter).cast());
static sSpriteTemplate_Credit: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_Credit).cast());
static sSpriteTemplate_CreditDigit: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_CreditDigit).cast());
static sSpriteTemplate_Cursor: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_Cursor).cast());
static sSpriteTemplate_Multiplier: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_Multiplier).cast());
static sSpriteTemplate_Shroomish: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_Shroomish).cast());
static sSpriteTemplate_ShroomishShadow: Table<CArray<SpriteTemplate, 2>> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_ShroomishShadow).cast());
static sSpriteTemplate_Taillow: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_Taillow).cast());
static sSpriteTemplate_TaillowShadow: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_TaillowShadow).cast());
static sSpriteTemplate_WheelCenter: Table<SpriteTemplate> =
    Table((&raw const crate::data::roulette::sSpriteTemplate_WheelCenter).cast());
static sSpriteTemplates_ColorHeaders: Table<CArray<SpriteTemplate, 3>> =
    Table((&raw const crate::data::roulette::sSpriteTemplates_ColorHeaders).cast());
static sSpriteTemplates_GridIcons: Table<CArray<SpriteTemplate, 4>> =
    Table((&raw const crate::data::roulette::sSpriteTemplates_GridIcons).cast());
static sSpriteTemplates_PokeHeaders: Table<CArray<SpriteTemplate, 4>> =
    Table((&raw const crate::data::roulette::sSpriteTemplates_PokeHeaders).cast());
static sSpriteTemplates_WheelIcons: Table<CArray<SpriteTemplate, 12>> =
    Table((&raw const crate::data::roulette::sSpriteTemplates_WheelIcons).cast());
static sTableMinBets: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::roulette::sTableMinBets).cast());
static sWheel_Pal: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::roulette::sWheel_Pal).cast());
static sWheel_Tilemap: Table<CArray<u32, 104>> =
    Table((&raw const crate::data::roulette::sWheel_Tilemap).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::roulette::sWindowTemplates).cast());
static sYesNoTable_AcceptMinBet: Table<YesNoFuncTable> =
    Table((&raw const crate::data::roulette::sYesNoTable_AcceptMinBet).cast());
static sYesNoTable_KeepPlaying: Table<YesNoFuncTable> =
    Table((&raw const crate::data::roulette::sYesNoTable_KeepPlaying).cast());

pub(crate) static mut sRoulette: *mut Roulette = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTextWindowId: u8 = 0;

unsafe extern "C" {
    static Roulette_Text_BoardWillBeCleared: CArray<u8, 0>;
    static Roulette_Text_CoinCaseIsFull: CArray<u8, 0>;
    static Roulette_Text_ControlsInstruction: CArray<u8, 0>;
    static Roulette_Text_ItsAHit: CArray<u8, 0>;
    static Roulette_Text_Jackpot: CArray<u8, 0>;
    static Roulette_Text_KeepPlaying: CArray<u8, 0>;
    static Roulette_Text_NoCoinsLeft: CArray<u8, 0>;
    static Roulette_Text_NotEnoughCoins: CArray<u8, 0>;
    static Roulette_Text_NothingDoing: CArray<u8, 0>;
    static Roulette_Text_PlayMinimumWagerIsX: CArray<u8, 0>;
    static Roulette_Text_SpecialRateTable: CArray<u8, 0>;
    static Roulette_Text_YouveWonXCoins: CArray<u8, 0>;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gLocalTime: Time;
    static mut gMPlayInfo_SE1: MusicPlayerInfo;
    static mut gMPlayInfo_SE2: MusicPlayerInfo;
    static mut gMain: Main;
    static mut gOamMatrices: CArray<OamMatrix, 32>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static gRouletteMenu_Gfx: CArray<u32, 0>;
    static gRouletteWheel_Gfx: CArray<u32, 0>;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpriteCoordOffsetX: i16;
    static mut gSpriteCoordOffsetY: i16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AlertTVThatPlayerPlayedRoulette(a0: u16);
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginHardwarePaletteFade(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn Cos2(a0: u16) -> i16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DisplayYesNoMenuDefaultYes();
    fn DoYesNoFuncWithChoice(a0: u8, a1: *mut YesNoFuncTable);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn EnableInterrupts(a0: u16);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillTilemapRect(a0: *mut u16, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetCoins() -> u16;
    fn GetGameStat(a0: u8) -> u32;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn HideCoinsWindow();
    fn IncrementDailyRouletteUses();
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitTextBoxGfxAndPrinters();
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsFanfareTaskInactive() -> u8;
    fn IsSEPlaying() -> u8;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LockPlayerFieldControls();
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PrintCoinsString(a0: u32);
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RouletteFlash_Add(a0: *mut RouletteFlashUtil, a1: u8, a2: *mut RouletteFlashSettings) -> u8;
    fn RouletteFlash_Enable(a0: *mut RouletteFlashUtil, a1: u16);
    fn RouletteFlash_Reset(a0: *mut RouletteFlashUtil);
    fn RouletteFlash_Run(a0: *mut RouletteFlashUtil);
    fn RouletteFlash_Stop(a0: *mut RouletteFlashUtil, a1: u16);
    fn RtcCalcLocalTime();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCoins(a0: u16);
    fn SetGameStat(a0: u8, a1: u32);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetTilemapRect(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn ShowCoinsWindow(a0: u32, a1: u8, a2: u8);
    fn Sin2(a0: u16) -> i16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TryPutFindThatGamerOnAir(a0: u16);
    fn UnlockPlayerFieldControls();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayPanpotControl(a0: *mut MusicPlayerInfo, a1: u16, a2: i8);
    fn m4aSongNumStart(a0: u16);
    fn m4aSongNumStartOrChange(a0: u16);
    fn m4aSongNumStop(a0: u16);
    fn malloc_and_decompress(a0: *mut c_void, a1: *mut u32) -> *mut c_void;
}

pub(crate) unsafe extern "C" fn CB2_Roulette() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    if (*sRoulette).flashUtil.enabled != 0 {
        RouletteFlash_Run(&raw mut (*sRoulette).flashUtil);
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Roulette() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    UpdateWheelPosition();
    SetGpuReg(REG_OFFSET_BG1HOFS, 0x200 - (*sRoulette).gridX as u16);
    if (*sRoulette).shroomishShadowTimer != 0 {
        SetGpuReg(REG_OFFSET_BLDALPHA, (*sRoulette).shroomishShadowAlpha);
    }
    if (*sRoulette).updateGridHighlight != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        &raw mut (*sRoulette).tilemapBuffers[2][224] as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        (0x6002000 as usize as *mut c_void as *mut u8).at(448) as *mut c_void
                            as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800001a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
        (*sRoulette).updateGridHighlight = FALSE as i16;
    }
    'l5: {
        let sw1: i16 = (*sRoulette).selectionRectDrawState;
        let mut fall = false;
        if sw1 == SELECT_STATE_DRAW {
            fall = true;
            SetBgAttribute(0, BG_ATTR_CHARBASEINDEX, 0);
            ShowBg(0);
            {
                {
                    {
                        let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                        volatile_write(
                            dmaRegs,
                            &raw mut (*sRoulette).tilemapBuffers[0][224] as usize as u32,
                        );
                        volatile_write(
                            dmaRegs.at(1),
                            (0x600f800 as usize as *mut c_void as *mut u8).at(448) as *mut c_void
                                as usize as u32,
                        );
                        volatile_write(dmaRegs.at(2), 0x800001a0);
                        let _ = (dmaRegs.at(2)).read_volatile();
                    }
                }
            }
            (*sRoulette).selectionRectDrawState = SELECT_STATE_UPDATE;
            break 'l5;
        }
        if sw1 == SELECT_STATE_UPDATE {
            fall = true;
            {
                {
                    {
                        let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                        volatile_write(
                            dmaRegs,
                            &raw mut (*sRoulette).tilemapBuffers[0][224] as usize as u32,
                        );
                        volatile_write(
                            dmaRegs.at(1),
                            (0x600f800 as usize as *mut c_void as *mut u8).at(448) as *mut c_void
                                as usize as u32,
                        );
                        volatile_write(dmaRegs.at(2), 0x800001a0);
                        let _ = (dmaRegs.at(2)).read_volatile();
                    }
                }
            }
            break 'l5;
        }
        if sw1 == SELECT_STATE_ERASE {
            fall = true;
            SetBgAttribute(0, BG_ATTR_CHARBASEINDEX, 2);
            ShowBg(0);
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(
                                dmaRegs.at(1),
                                (0x600f800 as usize as *mut c_void as *mut u8).at(448)
                                    as *mut c_void as usize as u32,
                            );
                            volatile_write(dmaRegs.at(2), 0x810001a0);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
            (*sRoulette).selectionRectDrawState = SELECT_STATE_WAIT;
        }
        if fall || sw1 == SELECT_STATE_WAIT {
            fall = true;
            break 'l5;
        }
    }
}
pub(crate) unsafe extern "C" fn InitRouletteBgAndWindows() {
    let mut size: u32 = 0;
    sRoulette = AllocZeroed(14720) as *mut Roulette;
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(1, sBgTemplates.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        0,
        (*sRoulette).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*sRoulette).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sRoulette).tilemapBuffers[6].as_mut_ptr() as *mut c_void,
    );
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    InitTextBoxGfxAndPrinters();
    sTextWindowId = 0;
    (*sRoulette).gridTilemap = malloc_and_decompress(
        sGrid_Tilemap.as_ptr().cast_mut() as *mut c_void,
        &raw mut size,
    ) as *mut u16;
}
pub(crate) unsafe extern "C" fn FreeRoulette() {
    Free((*sRoulette).gridTilemap as *mut c_void);
    (*sRoulette).gridTilemap = null_mut();
    FreeAllWindowBuffers();
    UnsetBgTilemapBuffer(0);
    UnsetBgTilemapBuffer(1);
    UnsetBgTilemapBuffer(2);
    ResetBgsAndClearDma3BusyFlags(0);
    memset(sRoulette as *mut u8, 0, 14720);
    Free(sRoulette as *mut c_void);
    sRoulette = null_mut();
}
pub(crate) unsafe extern "C" fn InitRouletteTableData() {
    let mut i: u8 = 0;
    let mut bgColors: CArray<u16, 3> = CArray([10392, 6762, 10392]);
    (*sRoulette).set_tableId(gSpecialVar_0x8004 as u8 & 1);
    if gSpecialVar_0x8004 as i32 & ROULETTE_SPECIAL_RATE != 0 {
        (*sRoulette).set_isSpecialRate(TRUE);
    }
    (*sRoulette).wheelSpeed = sRouletteTables[(*sRoulette).tableId()].wheelSpeed;
    (*sRoulette).wheelDelay = sRouletteTables[(*sRoulette).tableId()].wheelDelay;
    (*sRoulette).minBet =
        sTableMinBets[(*sRoulette).tableId() as i32 + (*sRoulette).isSpecialRate() as i32 * 2];
    (*sRoulette).set_unk1(1);
    if (*sRoulette).minBet == 1 {
        gPlttBufferUnfaded[0] = {
            gPlttBufferUnfaded[81] = {
                gPlttBufferFaded[0] = {
                    gPlttBufferFaded[81] = bgColors[0];
                    gPlttBufferFaded[81]
                };
                gPlttBufferFaded[0]
            };
            gPlttBufferUnfaded[81]
        };
    } else {
        gPlttBufferUnfaded[0] = {
            gPlttBufferUnfaded[81] = {
                gPlttBufferFaded[0] = {
                    gPlttBufferFaded[81] = bgColors[1];
                    gPlttBufferFaded[81]
                };
                gPlttBufferFaded[0]
            };
            gPlttBufferUnfaded[81]
        };
    }
    RouletteFlash_Reset(&raw mut (*sRoulette).flashUtil);
    i = 0;
    while i < 13 {
        RouletteFlash_Add(
            &raw mut (*sRoulette).flashUtil,
            i,
            (&raw const sFlashData_Colors[i]).cast_mut(),
        );
        i += 1;
    }
    i = 0;
    while i < PARTY_SIZE as u8 {
        match GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) {
            SPECIES_SHROOMISH => {
                (*sRoulette).partySpeciesFlags |= HAS_SHROOMISH;
            }
            304 => {
                (*sRoulette).partySpeciesFlags |= HAS_TAILLOW;
            }
            _ => {}
        }
        i += 1;
    }
    RtcCalcLocalTime();
}
pub(crate) unsafe extern "C" fn CB2_LoadRoulette() {
    let mut taskId: u8 = 0;
    match gMain.state {
        0 => {
            SetVBlankCallback(None);
            ScanlineEffect_Stop();
            SetVBlankHBlankCallbacksToNull();
            ResetVramOamAndBgCntRegs();
            ResetAllBgsCoordinates();
        }
        1 => {
            InitRouletteBgAndWindows();
            DeactivateAllTextPrinters();
            SetGpuReg(REG_OFFSET_BLDCNT, 9216);
            SetGpuReg(REG_OFFSET_BLDALPHA, 1546);
        }
        2 => {
            ResetPaletteFade();
            ResetSpriteData();
            ResetTasks();
            ResetTempTileDataBuffers();
        }
        3 => {
            LoadPalette((&raw const *sWheel_Pal).cast_mut() as *mut c_void, 0, 448);
            DecompressAndCopyTileDataToVram(
                1,
                gRouletteMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                2,
                gRouletteWheel_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
        }
        4 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return;
            }
            InitRouletteTableData();
            CopyToBgTilemapBuffer(2, sWheel_Tilemap.as_ptr().cast_mut() as *mut c_void, 0, 0);
        }
        5 => {
            LoadOrFreeMiscSpritePalettesAndSheets(FALSE);
            CreateWheelBallSprites();
            CreateWheelCenterSprite();
            CreateInterfaceSprites();
            CreateGridSprites();
            CreateGridBallSprites();
            CreateWheelIconSprites();
        }
        6 => {
            AnimateSprites();
            BuildOamBuffer();
            SetCreditDigits(GetCoins());
            SetBallCounterNumLeft(BALLS_PER_ROUND);
            SetMultiplierSprite(SELECTION_NONE);
            DrawGridBackground(SELECTION_NONE);
            DrawStdWindowFrame(sTextWindowId, FALSE);
            AddTextPrinterParameterized(
                sTextWindowId,
                FONT_NORMAL,
                Roulette_Text_ControlsInstruction.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
            gSpriteCoordOffsetX = -60;
            gSpriteCoordOffsetY = 0;
        }
        7 => {
            SetGpuReg(0x0, 4160);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
        }
        8 => {
            EnableInterrupts(INTR_FLAG_VBLANK);
            SetVBlankCallback(Some(VBlankCB_Roulette));
            BeginHardwarePaletteFade(0xFF, 0, 16, 0, 1);
            taskId = {
                (*sRoulette).playTaskId = CreateTask(Some(Task_StartPlaying), 0);
                (*sRoulette).playTaskId
            };
            gTasks[taskId].data[6] = BALLS_PER_ROUND as i16;
            gTasks[taskId].data[13] = GetCoins() as i16;
            AlertTVThatPlayerPlayedRoulette(GetCoins());
            (*sRoulette).spinTaskId = CreateTask(Some(Task_SpinWheel), 1);
            SetMainCallback2(Some(CB2_Roulette));
            return;
        }
        _ => {}
    }
    gMain.state += 1;
}
pub(crate) unsafe extern "C" fn Task_SpinWheel(taskId: u8) {
    let mut sin: i16 = 0;
    let mut cos: i16 = 0;
    if ({
        let t1 = (*sRoulette).wheelDelayTimer;
        (*sRoulette).wheelDelayTimer += 1;
        t1
    }) == (*sRoulette).wheelDelay
    {
        (*sRoulette).wheelDelayTimer = 0;
        if ({
            (*sRoulette).wheelAngle -= (*sRoulette).wheelSpeed as i16;
            (*sRoulette).wheelAngle
        }) < 0
        {
            (*sRoulette).wheelAngle = 360 - (*sRoulette).wheelSpeed as i16;
        }
    }
    sin = Sin2((*sRoulette).wheelAngle as u16);
    cos = Cos2((*sRoulette).wheelAngle as u16);
    sin = sin / 16;
    (*sRoulette).wheelRotation.a = {
        (*sRoulette).wheelRotation.d = cos / 16;
        (*sRoulette).wheelRotation.d
    };
    (*sRoulette).wheelRotation.b = sin;
    (*sRoulette).wheelRotation.c = -sin;
}
pub(crate) unsafe extern "C" fn Task_StartPlaying(taskId: u8) {
    if UpdatePaletteFade() == 0 {
        SetGpuReg(REG_OFFSET_BLDCNT, 9216);
        SetGpuReg(REG_OFFSET_BLDALPHA, 2056);
        gTasks[taskId].data[6] = 0;
        ResetBallDataForNewSpin(taskId);
        ResetHits();
        HideWheelBalls();
        DrawGridBackground(SELECTION_NONE);
        SetBallCounterNumLeft(BALLS_PER_ROUND);
        StartTaskAfterDelayOrInput(taskId, Some(Task_ContinuePlaying), NO_DELAY, 3);
    }
}
pub(crate) unsafe extern "C" fn Task_AskKeepPlaying(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    DrawStdWindowFrame(sTextWindowId, FALSE);
    AddTextPrinterParameterized(
        sTextWindowId,
        FONT_NORMAL,
        Roulette_Text_KeepPlaying.as_ptr().cast_mut(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
    DoYesNoFuncWithChoice(taskId, (&raw const *sYesNoTable_KeepPlaying).cast_mut());
}
pub(crate) unsafe extern "C" fn Task_ContinuePlaying(taskId: u8) {
    ClearStdWindowAndFrame(0, TRUE);
    gTasks[taskId].func = Some(Task_SelectFirstEmptySquare);
}
pub(crate) unsafe extern "C" fn Task_StopPlaying(taskId: u8) {
    DestroyTask((*sRoulette).spinTaskId);
    ExitRoulette(taskId);
}
pub(crate) unsafe extern "C" fn UpdateGridSelectionRect(selectionId: u8) {
    let mut temp0: u8 = 0;
    let mut temp1: u8 = 0;
    match selectionId {
        SELECTION_NONE => {
            FillTilemapRect(&raw mut (*sRoulette).tilemapBuffers[0][0], 0, 14, 7, 16, 13);
        }
        COL_WYNAUT | COL_AZURILL | COL_SKITTY | COL_MAKUHITA => {
            temp0 = selectionId * 3 + 14;
            FillTilemapRect(&raw mut (*sRoulette).tilemapBuffers[0][0], 0, 14, 7, 16, 13);
            SetTilemapRect(
                &raw mut (*sRoulette).tilemapBuffers[0][0],
                (*sRoulette).gridTilemap.at(281),
                temp0,
                7,
                3,
                13,
            );
        }
        ROW_ORANGE | ROW_GREEN | ROW_PURPLE => {
            temp1 = ((selectionId as i32 - 1) / 5) as u8 * 3 + 10;
            FillTilemapRect(&raw mut (*sRoulette).tilemapBuffers[0][0], 0, 14, 7, 16, 13);
            SetTilemapRect(
                &raw mut (*sRoulette).tilemapBuffers[0][0],
                (*sRoulette).gridTilemap.at(320),
                14,
                temp1,
                16,
                3,
            );
        }
        _ => {
            temp0 = (selectionId as i32 % 5) as u8 * 3 + 14;
            temp1 = ((selectionId as i32 - 1) / 5) as u8 * 3 + 7;
            FillTilemapRect(&raw mut (*sRoulette).tilemapBuffers[0][0], 0, 14, 7, 16, 13);
            SetTilemapRect(
                &raw mut (*sRoulette).tilemapBuffers[0][0],
                (*sRoulette).gridTilemap.at(272),
                temp0,
                temp1,
                3,
                3,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateGridSelection(taskId: u8) {
    SetMultiplierSprite(gTasks[taskId].data[4] as u8);
    UpdateGridSelectionRect(gTasks[taskId].data[4] as u8);
}
pub(crate) unsafe extern "C" fn Task_StartHandleBetGridInput(taskId: u8) {
    (*sRoulette).selectionRectDrawState = SELECT_STATE_DRAW;
    UpdateGridSelectionRect(gTasks[taskId].data[4] as u8);
    (*sRoulette).wheelDelay = 2;
    (*sRoulette).wheelDelayTimer = 0;
    gTasks[taskId].func = Some(Task_HandleBetGridInput);
}
pub(crate) unsafe extern "C" fn Task_SelectFirstEmptySquare(taskId: u8) {
    let mut i: i16 = 0;
    if (*sRoulette).hitFlags & F_ORANGE_ROW != 0 {
        i = SQU_GREEN_WYNAUT;
        while i < SQU_GREEN_MAKUHITA {
            if (*sRoulette).hitFlags & sGridSelections[i].flag == 0 {
                break;
            }
            i += 1;
        }
    } else {
        i = SQU_ORANGE_WYNAUT;
        while i <= SQU_ORANGE_MAKUHITA {
            if (*sRoulette).hitFlags & sGridSelections[i].flag == 0 {
                break;
            }
            i += 1;
        }
    }
    gTasks[taskId].data[4] = i;
    ResetBallDataForNewSpin(taskId);
    DrawGridBackground(gTasks[taskId].data[4] as u8);
    SetMultiplierSprite(gTasks[taskId].data[4] as u8);
    FlashSelectionOnWheel(gTasks[taskId].data[4] as u8);
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].func = Some(Task_StartHandleBetGridInput);
}
pub(crate) unsafe extern "C" fn CanMoveSelectionInDir(selectionId: *mut i16, dir: u8) -> u8 {
    let mut temp1: i8 = 0;
    let mut temp: i8 = 0;
    let mut moveOffsets: CArray<i8, 4> = CArray([-5, 5, -1, 1]);
    let mut originalSelection: i8 = *selectionId as i8;
    match dir {
        0 | 1 => {
            temp1 = (*selectionId % 5) as i8;
            temp = temp1 + ROW_PURPLE as i8;
            if temp1 == SELECTION_NONE as i8 {
                temp1 = 5;
            }
        }
        2 | 3 => {
            temp1 = (*selectionId / 5) as i8 * 5;
            temp = temp1 + COL_MAKUHITA as i8;
            if temp1 == SELECTION_NONE as i8 {
                temp1 = 1;
            }
        }
        _ => {}
    }
    *selectionId += moveOffsets[dir] as i16;
    if *selectionId < temp1 as i16 {
        *selectionId = temp as i16;
    }
    if *selectionId > temp as i16 {
        *selectionId = temp1 as i16;
    }
    if *selectionId != originalSelection as i16 {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ProcessBetGridInput(taskId: u8) {
    let mut headerOffset: u8 = 0;
    let mut dirPressed: u8 = FALSE;
    if (gMain.newKeys as i32 & DPAD_UP == 0
        || ({
            dirPressed = TRUE;
            dirPressed
        }) != 0
            && CanMoveSelectionInDir(&raw mut gTasks[taskId].data[4], 0) != 0)
        && (gMain.newKeys as i32 & DPAD_DOWN == 0
            || ({
                dirPressed = 1;
                dirPressed
            }) != 0
                && CanMoveSelectionInDir(&raw mut gTasks[taskId].data[4], 1) != 0)
        && (gMain.newKeys as i32 & DPAD_LEFT == 0
            || ({
                dirPressed = TRUE;
                dirPressed
            }) != 0
                && CanMoveSelectionInDir(&raw mut gTasks[taskId].data[4], 2) != 0)
        && (gMain.newKeys as i32 & DPAD_RIGHT == 0
            || ({
                dirPressed = TRUE;
                dirPressed
            }) != 0
                && CanMoveSelectionInDir(&raw mut gTasks[taskId].data[4], 3) != 0)
        && dirPressed != 0
    {
        let mut i: u8 = 0;
        DrawGridBackground(gTasks[taskId].data[4] as u8);
        UpdateGridSelection(taskId);
        gTasks[taskId].data[1] = 0;
        PlaySE(SE_SELECT);
        RouletteFlash_Stop(&raw mut (*sRoulette).flashUtil, 0xFFFF);
        (*sRoulette).flashUtil.palettes[13].set_available({
            (*sRoulette).flashUtil.palettes[14].set_available({
                (*sRoulette).flashUtil.palettes[15].set_available(FALSE);
                (*sRoulette).flashUtil.palettes[15].available()
            });
            (*sRoulette).flashUtil.palettes[14].available()
        });
        FlashSelectionOnWheel(gTasks[taskId].data[4] as u8);
        i = 0;
        while i < NUM_BOARD_POKES {
            gSprites[(*sRoulette).spriteIds[i as i32 + SPR_POKE_HEADER_1]]
                .oam
                .set_tileNum(
                    gSprites[(*sRoulette).spriteIds[i as i32 + SPR_POKE_HEADER_1]].sheetTileStart
                        + (*(*gSprites[(*sRoulette).spriteIds[i as i32 + SPR_POKE_HEADER_1]].anims))
                            .r#type as u16,
                );
            i += 1;
        }
        if gTasks[taskId].data[4] as u16 as i32 - 1 < COL_MAKUHITA as i32
            && (*sRoulette).hitFlags & sGridSelections[gTasks[taskId].data[4]].flag == 0
        {
            headerOffset = gTasks[taskId].data[4] as u8 - 1;
            gSprites[(*sRoulette).spriteIds[headerOffset as i32 + SPR_POKE_HEADER_1]]
                .oam
                .set_tileNum(
                    gSprites[(*sRoulette).spriteIds[headerOffset as i32 + SPR_POKE_HEADER_1]]
                        .sheetTileStart
                        + (*(*gSprites
                            [(*sRoulette).spriteIds[headerOffset as i32 + SPR_POKE_HEADER_1]]
                            .anims)
                            .at(1))
                        .r#type as u16,
                );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartSpin(taskId: u8) {
    IncrementDailyRouletteUses();
    (*sRoulette).selectionRectDrawState = SELECT_STATE_ERASE;
    if (*sRoulette).minBet == 1 {
        (*sRoulette).wheelDelay = 1;
    } else {
        (*sRoulette).wheelDelay = 0;
    }
    (*sRoulette).wheelDelayTimer = 0;
    gTasks[taskId].data[1] = 32;
    gTasks[taskId].func = Some(Task_SlideGridOffscreen);
}
pub(crate) unsafe extern "C" fn Task_PlaceBet(taskId: u8) {
    (*sRoulette).betSelection[(*sRoulette).curBallNum()] = gTasks[taskId].data[4] as u8;
    gTasks[taskId].data[2] =
        GetMultiplier((*sRoulette).betSelection[(*sRoulette).curBallNum()]) as i16;
    SetMultiplierSprite((*sRoulette).betSelection[(*sRoulette).curBallNum()]);
    if ({
        gTasks[taskId].data[13] -= (*sRoulette).minBet as i16;
        gTasks[taskId].data[13]
    }) < 0
    {
        gTasks[taskId].data[13] = 0;
    }
    SetCreditDigits(gTasks[taskId].data[13] as u16);
    gTasks[taskId].func = Some(Task_StartSpin);
}
pub(crate) unsafe extern "C" fn Task_HandleBetGridInput(taskId: u8) {
    ProcessBetGridInput(taskId);
    match gTasks[taskId].data[1] {
        0 => {
            UpdateGridSelectionRect(gTasks[taskId].data[4] as u8);
            gTasks[taskId].data[1] += 1;
        }
        30 => {
            UpdateGridSelectionRect(SELECTION_NONE);
            gTasks[taskId].data[1] += 1;
        }
        59 => {
            gTasks[taskId].data[1] = 0;
        }
        _ => {
            gTasks[taskId].data[1] += 1;
        }
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if (*sRoulette).hitFlags & sGridSelections[gTasks[taskId].data[4]].flag != 0 {
            PlaySE(SE_BOO);
        } else {
            m4aSongNumStart(SE_SHOP);
            gTasks[taskId].func = Some(Task_PlaceBet);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideGridOffscreen(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[1];
        gTasks[taskId].data[1] -= 1;
        t1
    }) > 0
    {
        if gTasks[taskId].data[1] > 2 {
            gSpriteCoordOffsetX += 2;
        }
        if ({
            (*sRoulette).gridX += 4;
            (*sRoulette).gridX
        }) == 104
        {
            gSprites[(*sRoulette).spriteIds[25]].callback =
                Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite));
        }
    } else {
        ShowHideGridIcons(1, 255);
        ShowHideGridBalls(1, 255);
        gTasks[taskId].func = Some(Task_InitBallRoll);
        gTasks[taskId].data[1] = 0;
    }
}
pub(crate) unsafe extern "C" fn GetRandomForBallTravelDistance(ballNum: u16, rand: u16) -> u8 {
    match (*sRoulette).partySpeciesFlags {
        HAS_SHROOMISH | HAS_TAILLOW => {
            if gLocalTime.hours > 3 && gLocalTime.hours < 10 {
                if ballNum < 12 || rand as i32 & 1 != 0 {
                    return (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i32 / 2)
                        as u8;
                } else {
                    return 1;
                }
            } else if rand as i32 & 3 == 0 {
                return (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i32 / 2) as u8;
            } else {
                return sRouletteTables[(*sRoulette).tableId()].randDistanceLow;
            }
        }
        3 => {
            if gLocalTime.hours > 3 && gLocalTime.hours < 11 {
                if ballNum < BALLS_PER_ROUND as u16 || rand as i32 & 1 != 0 {
                    return (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i32 / 2)
                        as u8;
                } else {
                    return 1;
                }
            } else if rand as i32 & 1 != 0 && ballNum > BALLS_PER_ROUND as u16 {
                return (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i32 / 4) as u8;
            } else {
                return (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i32 / 2) as u8;
            }
        }
        _ => {
            if gLocalTime.hours > 3 && gLocalTime.hours < 10 {
                if rand as i32 & 3 == 0 {
                    return 1;
                } else {
                    return (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i32 / 2)
                        as u8;
                }
            } else if rand as i32 & 3 == 0 {
                if ballNum > 12 {
                    return (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i32 / 2)
                        as u8;
                } else {
                    return sRouletteTables[(*sRoulette).tableId()].randDistanceLow;
                }
            } else if rand as i32 & 32768 != 0 {
                if ballNum > 12 {
                    return sRouletteTables[(*sRoulette).tableId()].randDistanceLow;
                } else {
                    return sRouletteTables[(*sRoulette).tableId()].randDistanceHigh;
                }
            } else {
                return sRouletteTables[(*sRoulette).tableId()].randDistanceHigh * 2;
            }
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_InitBallRoll(taskId: u8) {
    let mut randTravelMod: u8 = 0;
    let mut randTravelDist: i8 = 0;
    let mut startAngleId: i8 = 0;
    let mut travelDist: u16 = 0;
    let mut rand: u16 = 0;
    let mut randmod: u16 = 0;
    let mut startAngles: CArray<u16, 4> = CArray([0, 180, 90, 270]);
    rand = Random();
    randmod = (rand as i32 % 100) as u16;
    (*sRoulette).curBallSpriteId = gTasks[taskId].data[6] as u8;
    (*sRoulette).ballState = {
        (*sRoulette).hitSlot = {
            (*sRoulette).stuckHitSlot = 0;
            (*sRoulette).stuckHitSlot
        };
        (*sRoulette).hitSlot
    };
    randTravelMod = GetRandomForBallTravelDistance(gTasks[taskId].data[8] as u16, rand);
    randTravelDist =
        rem_i32(rand as i32, randTravelMod as i32) as i8 - (randTravelMod as i32 / 2) as i8;
    if gLocalTime.hours < 13 {
        startAngleId = 0;
    } else {
        startAngleId = 1;
    }
    if randmod < 80 {
        startAngleId *= 2;
    } else {
        startAngleId = (1 - startAngleId) * 2;
    }
    (*sRoulette).ballTravelDist = ({
        travelDist = sRouletteTables[(*sRoulette).tableId()].baseTravelDist + randTravelDist as u16;
        travelDist
    }) as i16;
    travelDist = (({
        let v1: i16 = travelDist as i16;
        let mut f = v1 as f32;
        if v1 < 0 {
            f += 65536.0;
        }
        f
    }) as f32
        / 5.0f32 as f32) as u16;
    (*sRoulette).ballTravelDistFast = travelDist as i16 * 3;
    (*sRoulette).ballTravelDistSlow = {
        (*sRoulette).ballTravelDistMed = travelDist;
        (*sRoulette).ballTravelDistMed
    };
    (*sRoulette).ballAngle = ({
        let v2: i16 = startAngles[(rand as i32 & 1) + startAngleId as i32] as i16;
        let mut f = v2 as f32;
        if v2 < 0 {
            f += 65536.0;
        }
        f
    }) as f32;
    (*sRoulette).ballAngleSpeed = ({
        let v3: i16 = sRouletteTables[(*sRoulette).tableId()].ballSpeed as i16;
        let mut f = v3 as f32;
        if v3 < 0 {
            f += 65536.0;
        }
        f
    }) as f32;
    (*sRoulette).ballAngleAccel = ((((*sRoulette).ballAngleSpeed as f32 * 0.5f32 as f32) as f32
        - (*sRoulette).ballAngleSpeed as f32) as f32
        / ({
            let v4: i16 = (*sRoulette).ballTravelDistFast;
            let mut f = v4 as f32;
            if v4 < 0 {
                f += 65536.0;
            }
            f
        }) as f32) as f32;
    (*sRoulette).ballDistToCenter = 68.0f32 as f32;
    (*sRoulette).ballFallAccel = 0.0f32 as f32;
    (*sRoulette).ballFallSpeed = -(8.0f32 as f32
        / ({
            let v5: i16 = (*sRoulette).ballTravelDistFast;
            let mut f = v5 as f32;
            if v5 < 0 {
                f += 65536.0;
            }
            f
        }) as f32) as f32;
    (*sRoulette).varA0 = 36.0f32 as f32;
    gTasks[taskId].func = Some(Task_RollBall);
}
pub(crate) unsafe extern "C" fn Task_RollBall(taskId: u8) {
    (*sRoulette).set_ballRolling(TRUE);
    (*sRoulette).ball = &raw mut gSprites[(*sRoulette).spriteIds[(*sRoulette).curBallSpriteId]];
    (*(*sRoulette).ball).callback = Some(SpriteCB_RollBall_Start);
    gTasks[taskId].data[6] += 1;
    gTasks[taskId].data[8] += 1;
    SetBallCounterNumLeft(BALLS_PER_ROUND - gTasks[taskId].data[6] as u8);
    m4aSongNumStart(SE_ROULETTE_BALL);
    gTasks[taskId].func = Some(Task_RecordBallHit);
}
pub(crate) unsafe extern "C" fn Task_RecordBallHit(taskId: u8) {
    if (*sRoulette).ballState != BALL_STATE_ROLLING {
        if (*sRoulette).ballStuck() != 0 {
            if (*sRoulette).ballUnstuck() != 0 {
                (*sRoulette).set_ballUnstuck(FALSE);
                (*sRoulette).set_ballStuck(FALSE);
            }
        } else {
            if gTasks[taskId].data[1] == 0 {
                let mut won: u8 = IsHitInBetSelection(
                    RecordHit(taskId, (*sRoulette).hitSlot),
                    (*sRoulette).betSelection[(*sRoulette).curBallNum()],
                );
                gTasks[taskId].data[5] = won as i16;
                if won == TRUE {
                    RouletteFlash_Enable(&raw mut (*sRoulette).flashUtil, F_FLASH_OUTER_EDGES);
                }
            }
            if gTasks[taskId].data[1] <= 60 {
                if gMain.newKeys as i32 & A_BUTTON != 0 {
                    gTasks[taskId].data[1] = 60;
                }
                gTasks[taskId].data[1] += 1;
            } else {
                DrawGridBackground((*sRoulette).betSelection[(*sRoulette).curBallNum()]);
                ShowHideGridIcons(FALSE, gTasks[taskId].data[12] as u8);
                ShowHideGridBalls(FALSE, gTasks[taskId].data[6] as u8 - 1);
                gTasks[taskId].data[1] = 32;
                gTasks[taskId].func = Some(Task_SlideGridOnscreen);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideGridOnscreen(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[1];
        gTasks[taskId].data[1] -= 1;
        t1
    }) > 0
    {
        if gTasks[taskId].data[1] > 2 {
            gSpriteCoordOffsetX -= 2;
        }
        if ({
            (*sRoulette).gridX -= 4;
            (*sRoulette).gridX
        }) == 104
        {
            gSprites[(*sRoulette).spriteIds[25]].callback = Some(SpriteCB_GridSquare);
        }
    } else {
        ShowHideWinSlotCursor(gTasks[taskId].data[12] as u8);
        if gTasks[taskId].data[5] == TRUE as i16 {
            gTasks[taskId].data[1] = 121;
        } else {
            gTasks[taskId].data[1] = 61;
        }
        gTasks[taskId].func = Some(Task_FlashBallOnWinningSquare);
    }
}
pub(crate) unsafe extern "C" fn Task_FlashBallOnWinningSquare(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[1];
        gTasks[taskId].data[1] -= 1;
        t1
    }) > 1
    {
        match gTasks[taskId].data[1] % 16 {
            8 => {
                ShowHideGridIcons(FALSE, 255);
                ShowHideGridBalls(FALSE, 255);
            }
            0 => {
                ShowHideGridIcons(FALSE, gTasks[taskId].data[12] as u8);
                ShowHideGridBalls(FALSE, gTasks[taskId].data[6] as u8 - 1);
            }
            _ => {}
        }
    } else {
        StartTaskAfterDelayOrInput(taskId, Some(Task_PrintSpinResult), 30, 0);
    }
}
pub(crate) unsafe extern "C" fn Task_TryIncrementWins(taskId: u8) {
    match gTasks[taskId].data[5] {
        1 | 2 => {
            if IsFanfareTaskInactive() != 0 {
                let mut wins: u32 = GetGameStat(GAME_STAT_CONSECUTIVE_ROULETTE_WINS);
                if wins
                    < ({
                        gTasks[taskId].data[11] += 1;
                        gTasks[taskId].data[11]
                    }) as u32
                {
                    SetGameStat(
                        GAME_STAT_CONSECUTIVE_ROULETTE_WINS,
                        gTasks[taskId].data[11] as u32,
                    );
                }
                StartTaskAfterDelayOrInput(taskId, Some(Task_PrintPayout), NO_DELAY, 3);
            }
        }
        _ => {
            if IsSEPlaying() == 0 {
                gTasks[taskId].data[11] = 0;
                StartTaskAfterDelayOrInput(taskId, Some(Task_EndTurn), NO_DELAY, 3);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintSpinResult(taskId: u8) {
    match gTasks[taskId].data[5] {
        1 | 2 => {
            if gTasks[taskId].data[2] == MAX_MULTIPLIER {
                PlayFanfare(MUS_SLOTS_JACKPOT);
                DrawStdWindowFrame(sTextWindowId, FALSE);
                AddTextPrinterParameterized(
                    sTextWindowId,
                    FONT_NORMAL,
                    Roulette_Text_Jackpot.as_ptr().cast_mut(),
                    0,
                    1,
                    TEXT_SKIP_DRAW,
                    None,
                );
                CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
            } else {
                PlayFanfare(MUS_SLOTS_WIN);
                DrawStdWindowFrame(sTextWindowId, FALSE);
                AddTextPrinterParameterized(
                    sTextWindowId,
                    FONT_NORMAL,
                    Roulette_Text_ItsAHit.as_ptr().cast_mut(),
                    0,
                    1,
                    TEXT_SKIP_DRAW,
                    None,
                );
                CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
            }
        }
        _ => {
            m4aSongNumStart(SE_FAILURE);
            DrawStdWindowFrame(sTextWindowId, FALSE);
            AddTextPrinterParameterized(
                sTextWindowId,
                FONT_NORMAL,
                Roulette_Text_NothingDoing.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
        }
    }
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].func = Some(Task_TryIncrementWins);
}
pub(crate) unsafe extern "C" fn Task_GivePayout(taskId: u8) {
    match gTasks[taskId].data[7] {
        0 => {
            gTasks[taskId].data[13] += 1;
            m4aSongNumStart(SE_PIN);
            SetCreditDigits(gTasks[taskId].data[13] as u16);
            if gTasks[taskId].data[13] >= MAX_COINS {
                gTasks[taskId].data[1] = 0;
            } else {
                gTasks[taskId].data[1] -= 1;
                gTasks[taskId].data[7] += 1;
            }
        }
        3 => {
            m4aSongNumStop(SE_PIN);
            gTasks[taskId].data[7] = 0;
        }
        _ => {
            gTasks[taskId].data[7] += 1;
        }
    }
    if gTasks[taskId].data[1] == 0 {
        StartTaskAfterDelayOrInput(taskId, Some(Task_EndTurn), NO_DELAY, 3);
    }
}
pub(crate) unsafe extern "C" fn Task_PrintPayout(taskId: u8) {
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*sRoulette).minBet as i32 * gTasks[taskId].data[2] as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        2,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        Roulette_Text_YouveWonXCoins.as_ptr().cast_mut(),
    );
    DrawStdWindowFrame(sTextWindowId, FALSE);
    AddTextPrinterParameterized(
        sTextWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
    gTasks[taskId].data[1] = (*sRoulette).minBet as i16 * gTasks[taskId].data[2];
    gTasks[taskId].data[7] = 0;
    gTasks[taskId].func = Some(Task_GivePayout);
}
pub(crate) unsafe extern "C" fn Task_EndTurn(taskId: u8) {
    RouletteFlash_Stop(&raw mut (*sRoulette).flashUtil, 0xFFFF);
    (*sRoulette).flashUtil.palettes[13].set_available({
        (*sRoulette).flashUtil.palettes[14].set_available({
            (*sRoulette).flashUtil.palettes[15].set_available(FALSE);
            (*sRoulette).flashUtil.palettes[15].available()
        });
        (*sRoulette).flashUtil.palettes[14].available()
    });
    gSprites[(*sRoulette).spriteIds[SPR_WHEEL_ICON_ORANGE_WYNAUT
        + sGridSelections[gTasks[taskId].data[12]].spriteIdOffset as i32]]
        .set_invisible(TRUE as u16);
    gTasks[taskId].func = Some(Task_TryPrintEndTurnMsg);
}
pub(crate) unsafe extern "C" fn Task_TryPrintEndTurnMsg(taskId: u8) {
    let mut i: u8 = 0;
    gTasks[taskId].data[4] = i as i16;
    (*sRoulette).betSelection[(*sRoulette).curBallNum()] = SELECTION_NONE;
    DrawGridBackground(SELECTION_NONE);
    gSprites[(*sRoulette).spriteIds[48]].set_invisible(TRUE as u16);
    i = 0;
    while i < NUM_BOARD_POKES {
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_POKE_HEADER_1]]
            .oam
            .set_tileNum(
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_POKE_HEADER_1]].sheetTileStart
                    + (*(*gSprites[(*sRoulette).spriteIds[i as i32 + SPR_POKE_HEADER_1]].anims))
                        .r#type as u16,
            );
        i += 1;
    }
    if gTasks[taskId].data[13] >= (*sRoulette).minBet as i16 {
        if gTasks[taskId].data[6] == BALLS_PER_ROUND as i16 {
            DrawStdWindowFrame(sTextWindowId, FALSE);
            AddTextPrinterParameterized(
                sTextWindowId,
                FONT_NORMAL,
                Roulette_Text_BoardWillBeCleared.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
            StartTaskAfterDelayOrInput(taskId, Some(Task_ClearBoard), NO_DELAY, 3);
        } else if gTasks[taskId].data[13] == MAX_COINS {
            DrawStdWindowFrame(sTextWindowId, FALSE);
            AddTextPrinterParameterized(
                sTextWindowId,
                FONT_NORMAL,
                Roulette_Text_CoinCaseIsFull.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
            StartTaskAfterDelayOrInput(taskId, Some(Task_AskKeepPlaying), NO_DELAY, 3);
        } else {
            gTasks[taskId].func = Some(Task_AskKeepPlaying);
        }
    } else {
        DrawStdWindowFrame(sTextWindowId, FALSE);
        AddTextPrinterParameterized(
            sTextWindowId,
            FONT_NORMAL,
            Roulette_Text_NoCoinsLeft.as_ptr().cast_mut(),
            0,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
        CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
        StartTaskAfterDelayOrInput(taskId, Some(Task_StopPlaying), 60, 3);
    }
}
pub(crate) unsafe extern "C" fn Task_ClearBoard(taskId: u8) {
    let mut i: u8 = 0;
    gTasks[taskId].data[6] = 0;
    ResetBallDataForNewSpin(taskId);
    ResetHits();
    HideWheelBalls();
    DrawGridBackground(SELECTION_NONE);
    SetBallCounterNumLeft(BALLS_PER_ROUND);
    i = 0;
    while i < NUM_ROULETTE_SLOTS {
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_WHEEL_ICON_ORANGE_WYNAUT]]
            .set_invisible(FALSE as u16);
        i += 1;
    }
    if gTasks[taskId].data[13] == MAX_COINS {
        DrawStdWindowFrame(sTextWindowId, FALSE);
        AddTextPrinterParameterized(
            sTextWindowId,
            FONT_NORMAL,
            Roulette_Text_CoinCaseIsFull.as_ptr().cast_mut(),
            0,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
        CopyWindowToVram(sTextWindowId, COPYWIN_FULL);
        StartTaskAfterDelayOrInput(taskId, Some(Task_AskKeepPlaying), NO_DELAY, 3);
    } else {
        gTasks[taskId].func = Some(Task_AskKeepPlaying);
    }
}
pub(crate) unsafe extern "C" fn ExitRoulette(taskId: u8) {
    RouletteFlash_Stop(&raw mut (*sRoulette).flashUtil, 0xFFFF);
    RouletteFlash_Reset(&raw mut (*sRoulette).flashUtil);
    SetCoins(gTasks[taskId].data[13] as u16);
    if GetCoins() < (*sRoulette).minBet as u16 {
        gSpecialVar_0x8004 = TRUE as u16;
    } else {
        gSpecialVar_0x8004 = FALSE as u16;
    }
    TryPutFindThatGamerOnAir(GetCoins());
    BeginHardwarePaletteFade(0xFF, 0, 0, 16, 0);
    gTasks[taskId].func = Some(Task_ExitRoulette);
}
pub(crate) unsafe extern "C" fn Task_ExitRoulette(taskId: u8) {
    if UpdatePaletteFade() == 0 {
        SetVBlankCallback(None);
        gSpriteCoordOffsetX = {
            gSpriteCoordOffsetY = 0;
            gSpriteCoordOffsetY
        };
        ResetVramOamAndBgCntRegs();
        ResetAllBgsCoordinates();
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        SetGpuReg(REG_OFFSET_BLDY, 0);
        FreeAllSpritePalettes();
        ResetPaletteFade();
        ResetSpriteData();
        FreeRoulette();
        gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
        SetMainCallback2(Some(CB2_ReturnToField));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForNextTask(taskId: u8) {
    if (*sRoulette).taskWaitDelay == 0
        || gMain.newKeys as i32 & (*sRoulette).taskWaitKey as i32 != 0
    {
        gTasks[taskId].func = (*sRoulette).nextTask;
        if (*sRoulette).taskWaitKey > 0 {
            PlaySE(SE_SELECT);
        }
        (*sRoulette).nextTask = None;
        (*sRoulette).taskWaitKey = 0;
        (*sRoulette).taskWaitDelay = 0;
    }
    if (*sRoulette).taskWaitDelay != NO_DELAY {
        (*sRoulette).taskWaitDelay -= 1;
    }
}
pub(crate) unsafe extern "C" fn StartTaskAfterDelayOrInput(
    taskId: u8,
    mut task: Option<unsafe extern "C" fn(u8)>,
    delay: u16,
    key: u16,
) {
    (*sRoulette).prevTask = gTasks[taskId].func;
    if task.is_none() {
        task = (*sRoulette).prevTask;
    }
    (*sRoulette).nextTask = task;
    (*sRoulette).taskWaitDelay = delay;
    if delay == NO_DELAY && key == 0 {
        (*sRoulette).taskWaitKey = 0xFFFF;
    } else {
        (*sRoulette).taskWaitKey = key;
    }
    gTasks[taskId].func = Some(Task_WaitForNextTask);
}
pub(crate) unsafe extern "C" fn ResetBallDataForNewSpin(taskId: u8) {
    let mut i: u8 = 0;
    (*sRoulette).unk0 = FALSE;
    (*sRoulette).set_ballRolling(FALSE);
    (*sRoulette).set_ballStuck(FALSE);
    (*sRoulette).set_ballUnstuck(FALSE);
    (*sRoulette).set_useTaillow(FALSE);
    i = 0;
    while i < BALLS_PER_ROUND {
        (*sRoulette).betSelection[i] = SELECTION_NONE;
        i += 1;
    }
    (*sRoulette).set_curBallNum(0);
    gTasks[taskId].data[1] = 0;
}
pub(crate) unsafe extern "C" fn ResetHits() {
    let mut i: u8 = 0;
    (*sRoulette).hitFlags = 0;
    i = 0;
    while i < BALLS_PER_ROUND {
        (*sRoulette).hitSquares[i] = 0;
        i += 1;
    }
    i = 0;
    while i < NUM_BOARD_POKES {
        (*sRoulette).pokeHits[i] = 0;
        i += 1;
    }
    i = 0;
    while i < NUM_BOARD_COLORS {
        (*sRoulette).colorHits[i] = 0;
        i += 1;
    }
    ShowHideGridBalls(1, 255);
}
pub(crate) unsafe extern "C" fn RecordHit(taskId: u8, slotId: u8) -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut columnFlags: CArray<u32, 4> = CArray([0x10842, 0x21084, 0x42108, 0x84210]);
    let mut rowFlags: CArray<u32, 3> = CArray([992, 31744, 0xf8000]);
    if slotId >= NUM_ROULETTE_SLOTS {
        return 0;
    }
    (*sRoulette).hitSquares[gTasks[taskId].data[6] as i32 - 1] = sRouletteSlots[slotId].gridSquare;
    gTasks[taskId].data[12] = sRouletteSlots[slotId].gridSquare as i16;
    (*sRoulette).hitFlags |= sRouletteSlots[slotId].flag;
    i = 0;
    while i < NUM_BOARD_POKES {
        if sRouletteSlots[slotId].flag & columnFlags[i] != 0 {
            (*sRoulette).pokeHits[i] += 1;
        }
        if (*sRoulette).pokeHits[i] >= NUM_BOARD_COLORS {
            (*sRoulette).hitFlags |= columnFlags[i];
        }
        i += 1;
    }
    j = 0;
    while j < NUM_BOARD_COLORS {
        if sRouletteSlots[slotId].flag & rowFlags[j] != 0 {
            (*sRoulette).colorHits[j] += 1;
        }
        if (*sRoulette).colorHits[j] >= NUM_BOARD_POKES {
            (*sRoulette).hitFlags |= rowFlags[j];
        }
        j += 1;
    }
    return sRouletteSlots[slotId].gridSquare;
}
pub(crate) unsafe extern "C" fn IsHitInBetSelection(mut gridSquare: u8, betSelection: u8) -> u8 {
    let mut hit: u8 = gridSquare;
    if ({
        gridSquare -= 1;
        gridSquare
    }) < NUM_GRID_SELECTIONS
    {
        match betSelection {
            SELECTION_NONE => {
                return 3;
            }
            COL_WYNAUT | COL_AZURILL | COL_SKITTY | COL_MAKUHITA => {
                if hit as i32 == betSelection as i32 + ROW_ORANGE as i32
                    || hit as i32 == betSelection as i32 + ROW_GREEN as i32
                    || hit as i32 == betSelection as i32 + ROW_PURPLE as i32
                {
                    return TRUE;
                }
            }
            ROW_ORANGE | ROW_GREEN | ROW_PURPLE => {
                if hit as i32 >= betSelection as i32 + COL_WYNAUT as i32
                    && hit as i32 <= betSelection as i32 + COL_MAKUHITA as i32
                {
                    return TRUE;
                }
            }
            _ => {
                if hit == betSelection {
                    return TRUE;
                }
            }
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FlashSelectionOnWheel(selectionId: u8) {
    let mut flashFlags: u16 = 0;
    let mut numSelected: u8 = 0;
    let mut palOffset: u16 = 0;
    let mut i: u8 = 0;
    'l1: {
        match selectionId {
            ROW_ORANGE | ROW_GREEN | ROW_PURPLE => {
                i = selectionId + 1;
                while (i as i32) < selectionId as i32 + 5 {
                    if (*sRoulette).hitFlags & sGridSelections[i].flag == 0 {
                        flashFlags |= sGridSelections[i].flashFlags;
                    }
                    i += 1;
                }
                RouletteFlash_Enable(&raw mut (*sRoulette).flashUtil, {
                    flashFlags &= 57343;
                    flashFlags
                });
            }
            _ => {
                let mut iconFlash: CArray<RouletteFlashSettings, 3> = zeroed();
                memcpy(
                    iconFlash.as_mut_ptr() as *mut u8,
                    sFlashData_PokeIcons.as_ptr().cast_mut() as *mut u8,
                    24,
                );
                if selectionId >= COL_WYNAUT && selectionId <= COL_MAKUHITA {
                    numSelected = NUM_BOARD_COLORS;
                } else {
                    numSelected = 1;
                }
                palOffset = (selectionId as i32 / 5) as u16 - 1;
                match selectionId as i32 % 5 {
                    1 => {
                        palOffset = gSprites[(*sRoulette).spriteIds[7]].oam.paletteNum() * 16;
                    }
                    2 => {
                        palOffset = gSprites[(*sRoulette).spriteIds[8]].oam.paletteNum() * 16;
                    }
                    3 => {
                        palOffset = gSprites[(*sRoulette).spriteIds[9]].oam.paletteNum() * 16;
                    }
                    4 => {
                        palOffset = gSprites[(*sRoulette).spriteIds[10]].oam.paletteNum() * 16;
                    }
                    _ => {}
                }
                if numSelected == 1 {
                    if (*sRoulette).hitFlags & sGridSelections[selectionId].flag == 0 {
                        iconFlash[selectionId as i32 / 5 - 1].paletteOffset += palOffset;
                        RouletteFlash_Add(
                            &raw mut (*sRoulette).flashUtil,
                            13,
                            &raw mut iconFlash[selectionId as i32 / 5 - 1],
                        );
                    } else {
                        break 'l1;
                    }
                } else {
                    i = 0;
                    while i < NUM_BOARD_COLORS {
                        let mut columnSlotId: u8 = i * 5 + selectionId + 5;
                        if (*sRoulette).hitFlags & sGridSelections[columnSlotId].flag == 0 {
                            iconFlash[columnSlotId as i32 / 5 - 1].paletteOffset += palOffset;
                            RouletteFlash_Add(
                                &raw mut (*sRoulette).flashUtil,
                                i + NUM_ROULETTE_SLOTS + 1,
                                &raw mut iconFlash[columnSlotId as i32 / 5 - 1],
                            );
                            if numSelected == 3 {
                                flashFlags = sGridSelections[columnSlotId].flashFlags;
                            }
                            numSelected -= 1;
                        }
                        i += 1;
                    }
                    if numSelected != 2 {
                        flashFlags = 0;
                    }
                }
                RouletteFlash_Enable(&raw mut (*sRoulette).flashUtil, {
                    flashFlags |= sGridSelections[selectionId].flashFlags;
                    flashFlags
                });
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawGridBackground(selectionId: u8) {
    let mut i: u8 = 0;
    volatile_write(&raw mut i, 0);
    let mut j: u8 = 0;
    volatile_write(&raw mut j, 0);
    let mut x: u16 = 0;
    volatile_write(&raw mut x, 0);
    let mut y: u16 = 0;
    volatile_write(&raw mut y, 0);
    let mut tilemapOffset: u8 = 0;
    volatile_write(&raw mut tilemapOffset, 0);
    let mut selectionIds: CArray<u8, 5> = zeroed();
    let mut numSquares: u8 = 0;
    (*sRoulette).updateGridHighlight = TRUE as i16;
    ShowHideGridIcons(0, 0);
    SetTilemapRect(
        (*sRoulette).tilemapBuffers[2].as_mut_ptr(),
        (*sRoulette).gridTilemap,
        14,
        7,
        16,
        13,
    );
    match selectionId {
        SELECTION_NONE => {
            return;
        }
        COL_WYNAUT | COL_AZURILL | COL_SKITTY | COL_MAKUHITA => {
            numSquares = 4;
            volatile_write(&raw mut i, 0);
            while (&raw mut i).read_volatile() < numSquares {
                selectionIds[(&raw mut i).read_volatile()] =
                    (&raw mut i).read_volatile() * ROW_ORANGE + selectionId;
                volatile_write(&raw mut i, (&raw mut i).read_volatile() + 1);
            }
        }
        ROW_ORANGE | ROW_GREEN | ROW_PURPLE => {
            numSquares = 5;
            volatile_write(&raw mut i, 0);
            while (&raw mut i).read_volatile() < numSquares {
                selectionIds[(&raw mut i).read_volatile()] =
                    (&raw mut i).read_volatile() + selectionId;
                volatile_write(&raw mut i, (&raw mut i).read_volatile() + 1);
            }
        }
        _ => {
            numSquares = 1;
            selectionIds[0] = selectionId;
        }
    }
    volatile_write(&raw mut i, 0);
    while (&raw mut i).read_volatile() < numSquares {
        volatile_write(
            &raw mut tilemapOffset,
            sGridSelections[selectionIds[(&raw mut i).read_volatile()]].tilemapOffset,
        );
        volatile_write(
            &raw mut x,
            sGridSelections[selectionIds[(&raw mut i).read_volatile()]].x as u16,
        );
        volatile_write(&raw mut j, 0);
        while (&raw mut j).read_volatile() < 3 {
            volatile_write(
                &raw mut y,
                (sGridSelections[selectionIds[(&raw mut i).read_volatile()]].y as u16
                    + (&raw mut j).read_volatile() as u16)
                    * 32,
            );
            (*sRoulette).tilemapBuffers[2]
                [(&raw mut x).read_volatile() as i32 + (&raw mut y).read_volatile() as i32 + 0] =
                *(*sRoulette)
                    .gridTilemap
                    .at(((&raw mut tilemapOffset).read_volatile() as i32
                        + (&raw mut j).read_volatile() as i32)
                        * 3
                        + 208
                        + 0);
            (*sRoulette).tilemapBuffers[2]
                [(&raw mut x).read_volatile() as i32 + (&raw mut y).read_volatile() as i32 + 1] =
                *(*sRoulette)
                    .gridTilemap
                    .at(((&raw mut tilemapOffset).read_volatile() as i32
                        + (&raw mut j).read_volatile() as i32)
                        * 3
                        + 208
                        + 1);
            (*sRoulette).tilemapBuffers[2]
                [(&raw mut x).read_volatile() as i32 + (&raw mut y).read_volatile() as i32 + 2] =
                *(*sRoulette)
                    .gridTilemap
                    .at(((&raw mut tilemapOffset).read_volatile() as i32
                        + (&raw mut j).read_volatile() as i32)
                        * 3
                        + 208
                        + 2);
            volatile_write(&raw mut j, (&raw mut j).read_volatile() + 1);
        }
        volatile_write(&raw mut i, (&raw mut i).read_volatile() + 1);
    }
}
pub(crate) unsafe extern "C" fn GetMultiplier(mut selectionId: u8) -> u8 {
    let mut multipliers: CArray<u8, 5> = CArray([0, 3, 4, 6, 12]);
    if selectionId > NUM_GRID_SELECTIONS {
        selectionId = 0;
    }
    match sGridSelections[selectionId].baseMultiplier() {
        NUM_BOARD_COLORS => {
            selectionId = (selectionId as i32 / 5) as u8 - 1;
            if (*sRoulette).colorHits[selectionId] >= NUM_BOARD_POKES {
                return 0;
            }
            return multipliers[(*sRoulette).colorHits[selectionId] as i32 + 1];
        }
        NUM_BOARD_POKES => {
            selectionId = selectionId - 1;
            if (*sRoulette).pokeHits[selectionId] >= NUM_BOARD_COLORS {
                return 0;
            }
            return multipliers[(*sRoulette).pokeHits[selectionId] as i32 + 2];
        }
        NUM_ROULETTE_SLOTS => {
            if (*sRoulette).hitFlags & sGridSelections[selectionId].flag != 0 {
                return 0;
            }
            return multipliers[4];
        }
        _ => {}
    }
    return 0;
}
pub(crate) unsafe extern "C" fn UpdateWheelPosition() {
    let mut bg2x: i32 = 0;
    let mut bg2y: i32 = 0;
    SetGpuReg(REG_OFFSET_BG2PA, (*sRoulette).wheelRotation.a as u16);
    SetGpuReg(REG_OFFSET_BG2PB, (*sRoulette).wheelRotation.b as u16);
    SetGpuReg(REG_OFFSET_BG2PC, (*sRoulette).wheelRotation.c as u16);
    SetGpuReg(REG_OFFSET_BG2PD, (*sRoulette).wheelRotation.d as u16);
    bg2x = 0x7400
        - (*sRoulette).wheelRotation.a as i32 * (gSpriteCoordOffsetX as i32 + 116)
        - (*sRoulette).wheelRotation.b as i32 * (gSpriteCoordOffsetY as i32 + 80);
    bg2y = 0x5400
        - (*sRoulette).wheelRotation.c as i32 * (gSpriteCoordOffsetX as i32 + 116)
        - (*sRoulette).wheelRotation.d as i32 * (gSpriteCoordOffsetY as i32 + 80);
    SetGpuReg(REG_OFFSET_BG2X_L, bg2x as u16);
    SetGpuReg(REG_OFFSET_BG2X_H, ((bg2x & 0x0fff0000) >> 16) as u16);
    SetGpuReg(REG_OFFSET_BG2Y_L, bg2y as u16);
    SetGpuReg(REG_OFFSET_BG2Y_H, ((bg2y & 0x0fff0000) >> 16) as u16);
}
pub(crate) unsafe extern "C" fn Task_ShowMinBetYesNo(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    DoYesNoFuncWithChoice(taskId, (&raw const *sYesNoTable_AcceptMinBet).cast_mut());
}
pub(crate) unsafe extern "C" fn Task_FadeToRouletteGame(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetVBlankCallback(None);
        SetMainCallback2(Some(CB2_LoadRoulette));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_AcceptMinBet(taskId: u8) {
    ClearStdWindowAndFrame(0, TRUE);
    HideCoinsWindow();
    FreeAllWindowBuffers();
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    gPaletteFade.set_delayCounter(gPaletteFade.multipurpose2() as u8);
    UpdatePaletteFade();
    gTasks[taskId].func = Some(Task_FadeToRouletteGame);
}
pub(crate) unsafe extern "C" fn Task_DeclineMinBet(taskId: u8) {
    ClearStdWindowAndFrame(0, 0);
    HideCoinsWindow();
    UnlockPlayerFieldControls();
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn Task_NotEnoughForMinBet(taskId: u8) {
    gTasks[taskId].data[0] += 1;
    if gMain.newKeys as i32 & 3 != 0 {
        gSpecialVar_0x8004 = 1;
        HideCoinsWindow();
        ClearStdWindowAndFrame(0, TRUE);
        UnlockPlayerFieldControls();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_PrintMinBet(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        let mut minBet: u32 = sTableMinBets
            [(gSpecialVar_0x8004 as i32 & 1) + (gSpecialVar_0x8004 >> 7) as i32 * 2]
            as u32;
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            minBet as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            1,
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            Roulette_Text_PlayMinimumWagerIsX.as_ptr().cast_mut(),
        );
        DrawStdWindowFrame(0, 0);
        AddTextPrinterParameterized(
            0,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            0,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
        CopyWindowToVram(0, COPYWIN_FULL);
        gTasks[taskId].func = Some(Task_ShowMinBetYesNo);
    }
}
pub(crate) unsafe extern "C" fn Task_PrintRouletteEntryMsg(taskId: u8) {
    let mut minBet: i32 = 0;
    PrintCoinsString(gTasks[taskId].data[13] as u32);
    minBet = sTableMinBets[(gSpecialVar_0x8004 as i32 & 1) + (gSpecialVar_0x8004 >> 7) as i32 * 2]
        as i32;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        minBet,
        STR_CONV_MODE_LEADING_ZEROS,
        1,
    );
    if gTasks[taskId].data[13] as i32 >= minBet {
        if gSpecialVar_0x8004 as i32 & ROULETTE_SPECIAL_RATE != 0
            && gSpecialVar_0x8004 as i32 & 1 != 0
        {
            DrawStdWindowFrame(0, 0);
            AddTextPrinterParameterized(
                0,
                FONT_NORMAL,
                Roulette_Text_SpecialRateTable.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(0, COPYWIN_FULL);
            gTasks[taskId].func = Some(Task_PrintMinBet);
        } else {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                Roulette_Text_PlayMinimumWagerIsX.as_ptr().cast_mut(),
            );
            DrawStdWindowFrame(0, 0);
            AddTextPrinterParameterized(
                0,
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(0, COPYWIN_FULL);
            gTasks[taskId].func = Some(Task_ShowMinBetYesNo);
        }
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            Roulette_Text_NotEnoughCoins.as_ptr().cast_mut(),
        );
        DrawStdWindowFrame(0, 0);
        AddTextPrinterParameterized(
            0,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            0,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
        CopyWindowToVram(0, COPYWIN_FULL);
        gTasks[taskId].func = Some(Task_NotEnoughForMinBet);
        gTasks[taskId].data[13] = 0;
        gTasks[taskId].data[0] = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayRoulette() {
    let mut taskId: u8 = 0;
    LockPlayerFieldControls();
    ShowCoinsWindow(GetCoins() as u32, 1, 1);
    taskId = CreateTask(Some(Task_PrintRouletteEntryMsg), 0);
    gTasks[taskId].data[13] = GetCoins() as i16;
}
pub(crate) unsafe extern "C" fn LoadOrFreeMiscSpritePalettesAndSheets(free: u8) {
    if free == 0 {
        FreeAllSpritePalettes();
        LoadSpritePalettes(sSpritePalettes.as_ptr().cast_mut());
        LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Ball).cast_mut());
        LoadCompressedSpriteSheet((&raw const *sSpriteSheet_ShroomishTaillow).cast_mut());
        LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Shadow).cast_mut());
    } else {
        FreeSpriteTilesByTag(GFXTAG_SHADOW);
        FreeSpriteTilesByTag(GFXTAG_SHROOMISH_TAILLOW);
        FreeSpriteTilesByTag(GFXTAG_BALL);
        FreeAllSpritePalettes();
    }
}
pub(crate) unsafe extern "C" fn CreateWheelIconSprite(
    template: *mut SpriteTemplate,
    r1: u8,
    angle: *mut u16,
) -> u8 {
    let mut temp: u16 = 0;
    let mut spriteId: u8 = CreateSprite(template, 116, 80, (*(*template).oam).y() as u8);
    gSprites[spriteId].data[0] = *angle as i16;
    gSprites[spriteId].data[1] = r1 as i16;
    gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
    gSprites[spriteId].set_animPaused(TRUE);
    gSprites[spriteId].set_affineAnimPaused(TRUE);
    temp = *angle;
    *angle += DEGREES_PER_SLOT;
    if *angle >= 360 {
        *angle = temp - 330;
    }
    return spriteId;
}
pub(crate) unsafe extern "C" fn CreateGridSprites() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut spriteId: u8 = 0;
    let mut s: SpriteSheet = zeroed();
    LZ77UnCompWram(
        sSpriteSheet_Headers.data,
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
    );
    s.data = gDecompressionBuffer.as_mut_ptr() as *mut c_void;
    s.size = sSpriteSheet_Headers.size;
    s.tag = sSpriteSheet_Headers.tag;
    LoadSpriteSheet(&raw mut s);
    LZ77UnCompWram(
        sSpriteSheet_GridIcons.data,
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
    );
    s.data = gDecompressionBuffer.as_mut_ptr() as *mut c_void;
    s.size = sSpriteSheet_GridIcons.size;
    s.tag = sSpriteSheet_GridIcons.tag;
    LoadSpriteSheet(&raw mut s);
    i = 0;
    while i < NUM_BOARD_COLORS {
        let mut y: u8 = i * 24;
        j = 0;
        while j < NUM_BOARD_POKES {
            spriteId = {
                (*sRoulette).spriteIds
                    [i as i32 * NUM_BOARD_POKES as i32 + SPR_GRID_ICON_ORANGE_WYNAUT + j as i32] =
                    CreateSprite(
                        (&raw const sSpriteTemplates_GridIcons[j]).cast_mut(),
                        j as i16 * 24 + 148,
                        y as i16 + 92,
                        30,
                    );
                (*sRoulette).spriteIds
                    [i as i32 * NUM_BOARD_POKES as i32 + SPR_GRID_ICON_ORANGE_WYNAUT + j as i32]
            };
            gSprites[spriteId].set_animPaused(TRUE);
            y += 24;
            if y >= 72 {
                y = 0;
            }
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < 4 {
        spriteId = {
            (*sRoulette).spriteIds[i as i32 + SPR_POKE_HEADER_1] = CreateSprite(
                (&raw const sSpriteTemplates_PokeHeaders[i]).cast_mut(),
                i as i16 * 24 + 148,
                70,
                30,
            );
            (*sRoulette).spriteIds[i as i32 + SPR_POKE_HEADER_1]
        };
        gSprites[spriteId].set_animPaused(TRUE);
        i += 1;
    }
    i = 0;
    while i < 3 {
        spriteId = {
            (*sRoulette).spriteIds[i as i32 + SPR_COLOR_HEADER_1] = CreateSprite(
                (&raw const sSpriteTemplates_ColorHeaders[i]).cast_mut(),
                126,
                i as i16 * 24 + 92,
                30,
            );
            (*sRoulette).spriteIds[i as i32 + SPR_COLOR_HEADER_1]
        };
        gSprites[spriteId].set_animPaused(TRUE);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DestroyGridSprites() {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_ROULETTE_SLOTS {
        DestroySprite(
            &raw mut gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_ICON_ORANGE_WYNAUT]],
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ShowHideGridIcons(hideAll: u8, hideSquare: u8) {
    let mut i: u8 = 0;
    match hideAll {
        TRUE => {
            i = 0;
            while i < NUM_GRID_SELECTIONS {
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_ICON_ORANGE_WYNAUT]]
                    .set_invisible(TRUE as u16);
                i += 1;
            }
        }
        FALSE => {
            i = 0;
            while i < NUM_ROULETTE_SLOTS {
                if (*sRoulette).hitFlags & sRouletteSlots[i].flag == 0 {
                    gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_ICON_ORANGE_WYNAUT]]
                        .set_invisible(FALSE as u16);
                } else if sRouletteSlots[i].gridSquare != hideSquare {
                    gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_ICON_ORANGE_WYNAUT]]
                        .set_invisible(TRUE as u16);
                } else {
                    gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_ICON_ORANGE_WYNAUT]]
                        .set_invisible(FALSE as u16);
                }
                i += 1;
            }
            while i < NUM_GRID_SELECTIONS {
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_ICON_ORANGE_WYNAUT]]
                    .set_invisible(FALSE as u16);
                i += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateGridBallSprites() {
    let mut i: u8 = 0;
    i = 0;
    while i < BALLS_PER_ROUND {
        (*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1] =
            CreateSprite((&raw const *sSpriteTemplate_Ball).cast_mut(), 116, 20, 10);
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]].set_invisible(TRUE as u16);
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]].data[0] = 1;
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]].callback =
            Some(SpriteCB_GridSquare);
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]]
            .oam
            .set_priority(1);
        StartSpriteAnim(
            &raw mut gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]],
            8,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ShowHideGridBalls(hideAll: u8, hideBallId: u8) {
    let mut i: u8 = 0;
    if hideAll != 0 {
        while i < BALLS_PER_ROUND {
            gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]].set_invisible(TRUE as u16);
            i += 1;
        }
    } else {
        while i < BALLS_PER_ROUND {
            if (*sRoulette).hitSquares[i] == 0 || i == hideBallId {
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]]
                    .set_invisible(TRUE as u16);
            } else {
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]]
                    .set_invisible(FALSE as u16);
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]].x =
                    (sGridSelections[(*sRoulette).hitSquares[i]].x as i16 + 1) * 8 + 4;
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_GRID_BALL_1]].y =
                    (sGridSelections[(*sRoulette).hitSquares[i]].y as i16 + 1) * 8 + 3;
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn ShowHideWinSlotCursor(selectionId: u8) {
    if selectionId == 0 {
        gSprites[(*sRoulette).spriteIds[48]].set_invisible(TRUE as u16);
    } else {
        gSprites[(*sRoulette).spriteIds[48]].set_invisible(FALSE as u16);
        gSprites[(*sRoulette).spriteIds[48]].x = (sGridSelections[selectionId].x as i16 + 2) * 8;
        gSprites[(*sRoulette).spriteIds[48]].y = (sGridSelections[selectionId].y as i16 + 2) * 8;
    }
}
pub(crate) unsafe extern "C" fn CreateWheelIconSprites() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut angle: u16 = 0;
    let mut s: SpriteSheet = zeroed();
    LZ77UnCompWram(
        sSpriteSheet_WheelIcons.data,
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
    );
    s.data = gDecompressionBuffer.as_mut_ptr() as *mut c_void;
    s.size = sSpriteSheet_WheelIcons.size;
    s.tag = sSpriteSheet_WheelIcons.tag;
    LoadSpriteSheet(&raw mut s);
    angle = 15;
    i = 0;
    while i < NUM_BOARD_COLORS {
        j = 0;
        while j < NUM_BOARD_POKES {
            let mut spriteId: u8 = 0;
            spriteId = {
                (*sRoulette).spriteIds
                    [i as i32 * NUM_BOARD_POKES as i32 + SPR_WHEEL_ICON_ORANGE_WYNAUT + j as i32] =
                    CreateWheelIconSprite(
                        (&raw const sSpriteTemplates_WheelIcons
                            [i as i32 * NUM_BOARD_POKES as i32 + j as i32])
                            .cast_mut(),
                        40,
                        &raw mut angle,
                    );
                (*sRoulette).spriteIds
                    [i as i32 * NUM_BOARD_POKES as i32 + SPR_WHEEL_ICON_ORANGE_WYNAUT + j as i32]
            };
            gSprites[spriteId].set_animPaused(TRUE);
            gSprites[spriteId].set_affineAnimPaused(TRUE);
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WheelIcon(sprite: *mut Sprite) {
    let mut cos: i16 = 0;
    let mut sin: i16 = 0;
    let mut matrixNum: u32 = 0;
    let mut angle: i16 = (*sRoulette).wheelAngle + (*sprite).data[0];
    if angle >= 360 {
        angle -= 360;
    }
    sin = Sin2(angle as u16);
    cos = Cos2(angle as u16);
    (*sprite).x2 = (sin as i32 * (*sprite).data[1] as i32 >> 12) as i16;
    (*sprite).y2 = (-(cos as i32) * (*sprite).data[1] as i32 >> 12) as i16;
    matrixNum = (*sprite).oam.matrixNum();
    sin = sin / 16;
    gOamMatrices[matrixNum].d = {
        cos = cos / 16;
        cos
    };
    gOamMatrices[matrixNum].a = cos;
    gOamMatrices[matrixNum].b = sin;
    gOamMatrices[matrixNum].c = -sin;
}
pub(crate) unsafe extern "C" fn CreateInterfaceSprites() {
    let mut i: u8 = 0;
    i = 0;
    while i < 5 {
        let mut s: SpriteSheet = zeroed();
        LZ77UnCompWram(
            sSpriteSheets_Interface[i].data,
            gDecompressionBuffer.as_mut_ptr() as *mut c_void,
        );
        s.data = gDecompressionBuffer.as_mut_ptr() as *mut c_void;
        s.size = sSpriteSheets_Interface[i].size;
        s.tag = sSpriteSheets_Interface[i].tag;
        LoadSpriteSheet(&raw mut s);
        i += 1;
    }
    (*sRoulette).spriteIds[20] =
        CreateSprite((&raw const *sSpriteTemplate_Credit).cast_mut(), 208, 16, 4);
    gSprites[(*sRoulette).spriteIds[20]].set_animPaused(TRUE);
    i = 0;
    while i < MAX_COIN_DIGITS {
        (*sRoulette).spriteIds[i as i32 + SPR_CREDIT_DIG_1] = CreateSprite(
            (&raw const *sSpriteTemplate_CreditDigit).cast_mut(),
            i as i16 * 8 + 196,
            24,
            0,
        );
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CREDIT_DIG_1]].set_invisible(TRUE as u16);
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CREDIT_DIG_1]].set_animPaused(TRUE);
        i += 1;
    }
    (*sRoulette).spriteIds[25] = CreateSprite(
        (&raw const *sSpriteTemplate_Multiplier).cast_mut(),
        120,
        68,
        4,
    );
    gSprites[(*sRoulette).spriteIds[25]].set_animPaused(TRUE);
    i = 0;
    while i < 3 {
        (*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1] = CreateSprite(
            (&raw const *sSpriteTemplate_BallCounter).cast_mut(),
            i as i16 * 16 + 192,
            36,
            4,
        );
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1]].set_invisible(TRUE as u16);
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1]].set_animPaused(TRUE);
        i += 1;
    }
    (*sRoulette).spriteIds[48] =
        CreateSprite((&raw const *sSpriteTemplate_Cursor).cast_mut(), 152, 96, 9);
    gSprites[(*sRoulette).spriteIds[48]].oam.set_priority(1);
    gSprites[(*sRoulette).spriteIds[48]].set_animPaused(TRUE);
    gSprites[(*sRoulette).spriteIds[48]].set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn SetCreditDigits(mut num: u16) {
    let mut i: u8 = 0;
    let mut d: u16 = 1000;
    let mut printZero: u8 = FALSE;
    i = 0;
    while i < MAX_COIN_DIGITS {
        let mut digit: u8 = div_i32(num as i32, d as i32) as u8;
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CREDIT_DIG_1]].set_invisible(TRUE as u16);
        if digit > 0 || printZero != 0 || i == 3 {
            gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CREDIT_DIG_1]]
                .set_invisible(FALSE as u16);
            gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CREDIT_DIG_1]]
                .oam
                .set_tileNum(
                    gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CREDIT_DIG_1]].sheetTileStart
                        + (*(*gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CREDIT_DIG_1]].anims)
                            .at(digit))
                        .r#type as u16,
                );
            printZero = TRUE;
        }
        num = rem_i32(num as i32, d as i32) as u16;
        d = (d as i32 / 10) as u16;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetMultiplierAnimId(mut selectionId: u8) -> u8 {
    let mut animIds: CArray<u8, 5> = CArray([0, 1, 2, 3, 4]);
    if selectionId > NUM_GRID_SELECTIONS {
        selectionId = 0;
    }
    match sGridSelections[selectionId].baseMultiplier() {
        NUM_BOARD_COLORS => {
            selectionId = (selectionId as i32 / 5) as u8 - 1;
            if (*sRoulette).colorHits[selectionId] > 3 {
                return 0;
            }
            return animIds[(*sRoulette).colorHits[selectionId] as i32 + 1];
        }
        NUM_BOARD_POKES => {
            selectionId = selectionId - 1;
            if (*sRoulette).pokeHits[selectionId] > 2 {
                return 0;
            }
            return animIds[(*sRoulette).pokeHits[selectionId] as i32 + 2];
        }
        NUM_ROULETTE_SLOTS => {
            if (*sRoulette).hitFlags & sGridSelections[selectionId].flag != 0 {
                return 0;
            }
            return animIds[4];
        }
        _ => {}
    }
    return 0;
}
pub(crate) unsafe extern "C" fn SetMultiplierSprite(selectionId: u8) {
    let mut sprite: *mut Sprite = &raw mut gSprites[(*sRoulette).spriteIds[25]];
    (*sprite).animCmdIndex = GetMultiplierAnimId(selectionId);
    (*sprite).oam.set_tileNum(
        (*sprite).sheetTileStart + (*(*(*sprite).anims).at((*sprite).animCmdIndex)).r#type as u16,
    );
}
pub(crate) unsafe extern "C" fn SetBallCounterNumLeft(numBalls: u8) {
    let mut i: u8 = 0;
    let mut t: u8 = 0;
    if (*sRoulette).minBet == 1 {
        t = 2;
    }
    match numBalls {
        6 => {
            i = 0;
            while i < 3 {
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1]]
                    .set_invisible(FALSE as u16);
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1]]
                    .oam
                    .set_tileNum(
                        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1]]
                            .sheetTileStart
                            + (*(*gSprites[(*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1]]
                                .anims))
                                .r#type as u16,
                    );
                i += 1;
            }
        }
        5 => {
            gSprites[(*sRoulette).spriteIds[28]].oam.set_tileNum(
                gSprites[(*sRoulette).spriteIds[28]].sheetTileStart
                    + (*(*gSprites[(*sRoulette).spriteIds[28]].anims).at(t).at(1)).r#type as u16,
            );
        }
        4 => {
            gSprites[(*sRoulette).spriteIds[28]].oam.set_tileNum(
                gSprites[(*sRoulette).spriteIds[28]].sheetTileStart
                    + (*(*gSprites[(*sRoulette).spriteIds[28]].anims).at(t).at(2)).r#type as u16,
            );
        }
        3 => {
            gSprites[(*sRoulette).spriteIds[27]].oam.set_tileNum(
                gSprites[(*sRoulette).spriteIds[27]].sheetTileStart
                    + (*(*gSprites[(*sRoulette).spriteIds[27]].anims).at(t).at(1)).r#type as u16,
            );
        }
        2 => {
            gSprites[(*sRoulette).spriteIds[27]].oam.set_tileNum(
                gSprites[(*sRoulette).spriteIds[27]].sheetTileStart
                    + (*(*gSprites[(*sRoulette).spriteIds[27]].anims).at(t).at(2)).r#type as u16,
            );
        }
        1 => {
            gSprites[(*sRoulette).spriteIds[26]].oam.set_tileNum(
                gSprites[(*sRoulette).spriteIds[26]].sheetTileStart
                    + (*(*gSprites[(*sRoulette).spriteIds[26]].anims).at(t).at(1)).r#type as u16,
            );
        }
        _ => {
            i = 0;
            while i < 3 {
                gSprites[(*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1]]
                    .oam
                    .set_tileNum(
                        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1]]
                            .sheetTileStart
                            + (*(*gSprites[(*sRoulette).spriteIds[i as i32 + SPR_BALL_COUNTER_1]]
                                .anims)
                                .at(t)
                                .at(2))
                            .r#type as u16,
                    );
                i += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GridSquare(sprite: *mut Sprite) {
    (*sprite).x2 = (*sRoulette).gridX;
}
pub(crate) unsafe extern "C" fn CreateWheelCenterSprite() {
    let mut spriteId: u8 = 0;
    let mut s: SpriteSheet = zeroed();
    LZ77UnCompWram(
        sSpriteSheet_WheelCenter.data,
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
    );
    s.data = gDecompressionBuffer.as_mut_ptr() as *mut c_void;
    s.size = sSpriteSheet_WheelCenter.size;
    s.tag = sSpriteSheet_WheelCenter.tag;
    LoadSpriteSheet(&raw mut s);
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_WheelCenter).cast_mut(),
        116,
        80,
        81,
    );
    gSprites[spriteId].data[0] = (*sRoulette).wheelAngle;
    gSprites[spriteId].data[1] = 0;
    gSprites[spriteId].set_animPaused(TRUE);
    gSprites[spriteId].set_affineAnimPaused(TRUE);
    gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
}
pub(crate) unsafe extern "C" fn SpriteCB_WheelCenter(sprite: *mut Sprite) {
    let mut matrixNum: u32 = (*sprite).oam.matrixNum();
    let mut matrix: *mut OamMatrix = &raw mut gOamMatrices[0];
    (*matrix.at(matrixNum)).d = (*sRoulette).wheelRotation.a;
    (*matrix.at(matrixNum)).a = (*sRoulette).wheelRotation.a;
    (*matrix.at(matrixNum)).b = (*sRoulette).wheelRotation.b;
    (*matrix.at(matrixNum)).c = (*sRoulette).wheelRotation.c;
}
pub(crate) unsafe extern "C" fn CreateWheelBallSprites() {
    let mut i: u8 = 0;
    i = 0;
    while i < BALLS_PER_ROUND {
        (*sRoulette).spriteIds[i] = CreateSprite(
            (&raw const *sSpriteTemplate_Ball).cast_mut(),
            116,
            80,
            57 - i,
        );
        if (*sRoulette).spriteIds[i] != MAX_SPRITES {
            gSprites[(*sRoulette).spriteIds[i]].set_invisible(TRUE as u16);
            gSprites[(*sRoulette).spriteIds[i]].set_coordOffsetEnabled(TRUE as u16);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn HideWheelBalls() {
    let mut spriteId: u8 = (*sRoulette).spriteIds[0];
    let mut i: u8 = 0;
    i = 0;
    while i < BALLS_PER_ROUND {
        let mut j: u8 = 0;
        gSprites[spriteId].set_invisible(TRUE as u16);
        gSprites[spriteId].callback =
            Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite));
        StartSpriteAnim(&raw mut gSprites[spriteId], 0);
        j = 0;
        while j < 8 {
            gSprites[spriteId].data[j] = 0;
            j += 1;
        }
        spriteId += 1;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateBallRelativeWheelAngle(sprite: *mut Sprite) -> i16 {
    if (*sRoulette).wheelAngle > (*sprite).data[3] {
        (*sprite).data[6] = 360 - (*sRoulette).wheelAngle + (*sprite).data[3];
        if (*sprite).data[6] >= 360 {
            (*sprite).data[6] -= 360;
        }
    } else {
        (*sprite).data[6] = (*sprite).data[3] - (*sRoulette).wheelAngle;
    }
    return (*sprite).data[6];
}
pub(crate) unsafe extern "C" fn UpdateSlotBelowBall(sprite: *mut Sprite) -> u8 {
    (*sRoulette).hitSlot =
        (UpdateBallRelativeWheelAngle(sprite) as f32 / DEGREES_PER_SLOT as i32 as f32 as f32) as u8;
    return (*sRoulette).hitSlot;
}
pub(crate) unsafe extern "C" fn GetBallDistanceToSlotMidpoint(sprite: *mut Sprite) -> i16 {
    let mut angleIntoSlot: i16 = UpdateBallRelativeWheelAngle(sprite) % 30;
    let mut distanceToMidpoint: u16 = 0;
    if angleIntoSlot == SLOT_MIDPOINT {
        distanceToMidpoint = 0;
        return {
            (*sprite).data[2] = distanceToMidpoint as i16;
            (*sprite).data[2]
        };
    } else if angleIntoSlot >= SLOT_MIDPOINT {
        distanceToMidpoint = 43 - angleIntoSlot as u16;
        return {
            (*sprite).data[2] = distanceToMidpoint as i16;
            (*sprite).data[2]
        };
    } else {
        distanceToMidpoint = SLOT_MIDPOINT as u16 - angleIntoSlot as u16;
        return {
            (*sprite).data[2] = distanceToMidpoint as i16;
            (*sprite).data[2]
        };
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn UpdateBallPos(sprite: *mut Sprite) {
    let mut sin: i16 = 0;
    let mut cos: i16 = 0;
    (*sRoulette).ballAngleSpeed += (*sRoulette).ballAngleAccel as f32;
    (*sRoulette).ballAngle += (*sRoulette).ballAngleSpeed as f32;
    if (*sRoulette).ballAngle as f32 >= 360 as f32 {
        (*sRoulette).ballAngle -= 360.0f32 as f32;
    } else if ((*sRoulette).ballAngle as f32) < 0.0f32 as f32 {
        (*sRoulette).ballAngle += 360.0f32 as f32;
    }
    (*sprite).data[3] = (*sRoulette).ballAngle as i16;
    (*sRoulette).ballFallSpeed += (*sRoulette).ballFallAccel as f32;
    (*sRoulette).ballDistToCenter += (*sRoulette).ballFallSpeed as f32;
    (*sprite).data[4] = (*sRoulette).ballDistToCenter as i16;
    sin = Sin2((*sprite).data[3] as u16);
    cos = Cos2((*sprite).data[3] as u16);
    (*sprite).x2 = (sin as i32 * (*sprite).data[4] as i32 >> 12) as i16;
    (*sprite).y2 = (-(cos as i32) * (*sprite).data[4] as i32 >> 12) as i16;
    if IsSEPlaying() != 0 {
        m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE1, TRACKS_ALL, (*sprite).x2 as i8);
        m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, (*sprite).x2 as i8);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallLandInSlot(sprite: *mut Sprite) {
    let mut sin: i16 = 0;
    let mut cos: i16 = 0;
    (*sprite).data[3] = (*sRoulette).wheelAngle + (*sprite).data[6];
    if (*sprite).data[3] >= 360 {
        (*sprite).data[3] -= 360;
    }
    sin = Sin2((*sprite).data[3] as u16);
    cos = Cos2((*sprite).data[3] as u16);
    (*sprite).x2 = (sin as i32 * (*sprite).data[4] as i32 >> 12) as i16;
    (*sprite).y2 = (-(cos as i32) * (*sprite).data[4] as i32 >> 12) as i16;
    (*sprite).y2 += gSpriteCoordOffsetY;
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_ShroomishBallFall(sprite: *mut Sprite) {
    UpdateBallPos(sprite);
    (*sprite).data[2] += 1;
    if (*sprite).data[4] < -132 || (*sprite).data[4] > 80 {
        (*sprite).set_invisible(TRUE as u16);
    } else {
        (*sprite).set_invisible(FALSE as u16);
    }
    if (*sprite).data[2] >= DEGREES_PER_SLOT as i16 {
        if (*sprite).data[0] == 0 {
            if (*sRoulette).ballDistToCenter as f32
                <= ((*sRoulette).varA0 as f32 - 2.0f32 as f32) as f32
            {
                (*sRoulette).ballState = BALL_STATE_LANDED;
                (*sRoulette).set_ballRolling(0);
                StartSpriteAnim(sprite, (*sprite).animCmdIndex + 3);
                UpdateSlotBelowBall(sprite);
                (*sprite).data[4] = 30;
                UpdateBallRelativeWheelAngle(sprite);
                (*sprite).data[6] = (*sprite).data[6] / 30 * 30 + 15;
                (*sprite).callback = Some(SpriteCB_BallLandInSlot);
                m4aSongNumStartOrChange(71);
                (*sRoulette).ballFallAccel = ({
                    (*sRoulette).ballFallSpeed = 0.0f32 as f32;
                    (*sRoulette).ballFallSpeed
                }) as f32;
                (*sRoulette).ballAngleSpeed = -1.0f32 as f32;
            }
        } else {
            if (*sRoulette).ballDistToCenter as f32
                >= ((*sRoulette).varA0 as f32 - 2.0f32 as f32) as f32
            {
                (*sRoulette).ballState = BALL_STATE_LANDED;
                (*sRoulette).set_ballRolling(0);
                StartSpriteAnim(sprite, (*sprite).animCmdIndex + 3);
                UpdateSlotBelowBall(sprite);
                (*sprite).data[4] = 30;
                UpdateBallRelativeWheelAngle(sprite);
                (*sprite).data[6] = (*sprite).data[6] / 30 * 30 + 15;
                (*sprite).callback = Some(SpriteCB_BallLandInSlot);
                m4aSongNumStartOrChange(71);
                (*sRoulette).ballFallAccel = ({
                    (*sRoulette).ballFallSpeed = 0.0f32 as f32;
                    (*sRoulette).ballFallSpeed
                }) as f32;
                (*sRoulette).ballAngleSpeed = -1.0f32 as f32;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_Shroomish(sprite: *mut Sprite) {
    let mut slotOffset: f32 = 0.0;
    let mut ballFallDist: f32 = 0.0;
    let mut ballFallSpeed: f32 = 0.0;
    UpdateBallPos(sprite);
    match (*sprite).data[3] {
        0 => {
            if (*sprite).data[0] != TRUE as i16 {
                slotOffset = (*sprite).data[7] as f32;
                ballFallDist = ((slotOffset as f32
                    * sRouletteTables[(*sRoulette).tableId()].randDistanceHigh as f32)
                    as f32
                    + (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i32 - 1) as f32)
                    as f32;
                ballFallSpeed = (slotOffset as f32
                    / sRouletteTables[(*sRoulette).tableId()]
                        .shroomish
                        .fallSlowdown as f32) as f32;
            } else {
                return;
            }
        }
        180 => {
            if (*sprite).data[0] != 0 {
                slotOffset = (*sprite).data[7] as f32;
                ballFallDist = ((slotOffset as f32
                    * sRouletteTables[(*sRoulette).tableId()].randDistanceHigh as f32)
                    as f32
                    + (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i32 - 1) as f32)
                    as f32;
                ballFallSpeed = -(slotOffset as f32
                    / sRouletteTables[(*sRoulette).tableId()]
                        .shroomish
                        .fallSlowdown as f32) as f32;
            } else {
                return;
            }
        }
        _ => {
            return;
        }
    }
    (*sRoulette).varA0 = (*sRoulette).ballDistToCenter as f32;
    (*sRoulette).ballFallSpeed = ballFallSpeed as f32;
    (*sRoulette).ballFallAccel = -(((ballFallSpeed as f32 * 2.0f32 as f32) as f32
        / ballFallDist as f32) as f32
        + (2.0f32 as f32 / (ballFallDist as f32 * ballFallDist as f32) as f32) as f32)
        as f32;
    (*sRoulette).ballAngleSpeed = 0.0f32 as f32;
    (*sprite).set_animPaused(FALSE);
    (*sprite).animNum = 0;
    (*sprite).set_animBeginning(TRUE as u16);
    (*sprite).set_animEnded(FALSE as u16);
    (*sprite).callback = Some(SpriteCB_UnstickBall_ShroomishBallFall);
    (*sprite).data[2] = 0;
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_TaillowDrop(sprite: *mut Sprite) {
    (*sprite).y2 =
        (((*sprite).data[2] as f32 * 0.05f32 as f32) as f32 * (*sprite).data[2] as f32) as i16 - 45;
    (*sprite).data[2] += 1;
    if (*sprite).data[2] >= DEGREES_PER_SLOT as i16 && (*sprite).y2 >= 0 {
        (*sRoulette).ballState = BALL_STATE_LANDED;
        (*sRoulette).set_ballRolling(0);
        StartSpriteAnim(sprite, (*sprite).animCmdIndex + 3);
        UpdateSlotBelowBall(sprite);
        (*sprite).data[4] = 30;
        UpdateBallRelativeWheelAngle(sprite);
        (*sprite).data[6] = (*sprite).data[6] / 30 * 30 + 15;
        (*sprite).callback = Some(SpriteCB_BallLandInSlot);
        m4aSongNumStartOrChange(71);
        (*sRoulette).set_ballUnstuck(TRUE);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_TaillowPickUp(sprite: *mut Sprite) {
    if ({
        let t1 = (*sprite).data[2];
        (*sprite).data[2] += 1;
        t1
    }) < 45
    {
        (*sprite).y2 -= 1;
        if (*sprite).data[2] == 45 {
            if gSprites[(*sRoulette).spriteIds[55]].animCmdIndex == 1 {
                (*sprite).y2 += 1;
            }
        }
    } else {
        if (*sprite).data[2] < (*sprite).data[7] {
            if gSprites[(*sRoulette).spriteIds[55]].animDelayCounter() == 0 {
                if gSprites[(*sRoulette).spriteIds[55]].animCmdIndex == 1 {
                    (*sprite).y2 += 1;
                } else {
                    (*sprite).y2 -= 1;
                }
            }
        } else {
            (*sprite).set_animPaused(FALSE);
            (*sprite).animNum = 1;
            (*sprite).set_animBeginning(TRUE as u16);
            (*sprite).set_animEnded(FALSE as u16);
            (*sprite).data[2] = 0;
            (*sprite).callback = Some(SpriteCB_UnstickBall_TaillowDrop);
            m4aSongNumStart(SE_BALL_THROW);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall_Taillow(sprite: *mut Sprite) {
    UpdateBallPos(sprite);
    match (*sprite).data[3] {
        90 => {
            if (*sprite).data[0] != TRUE as i16 {
                (*sprite).callback =
                    Some(SpriteCB_UnstickBall_TaillowPickUp as unsafe extern "C" fn(*mut Sprite));
                (*sprite).data[2] = 0;
            }
        }
        270 => {
            if (*sprite).data[0] != 0 {
                (*sprite).callback =
                    Some(SpriteCB_UnstickBall_TaillowPickUp as unsafe extern "C" fn(*mut Sprite));
                (*sprite).data[2] = 0;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnstickBall(sprite: *mut Sprite) {
    UpdateBallPos(sprite);
    match (*sRoulette).useTaillow() {
        TRUE => {
            CreateTaillowSprite(sprite);
            (*sprite).callback = Some(SpriteCB_UnstickBall_Taillow);
        }
        _ => {
            CreateShroomishSprite(sprite);
            (*sprite).callback = Some(SpriteCB_UnstickBall_Shroomish);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_TryLandAdjacent(sprite: *mut Sprite) {
    UpdateBallPos(sprite);
    if ({
        let t1 = (*sprite).data[2];
        (*sprite).data[2] -= 1;
        t1
    }) == 16
    {
        (*sRoulette).ballFallSpeed *= -1.0f32 as f32;
    }
    if (*sprite).data[2] == 0 {
        if (*sprite).data[0] == 0 {
            (*sRoulette).ballState = BALL_STATE_LANDED;
            (*sRoulette).set_ballRolling(0);
            StartSpriteAnim(sprite, (*sprite).animCmdIndex + 3);
            UpdateSlotBelowBall(sprite);
            (*sprite).data[4] = 30;
            UpdateBallRelativeWheelAngle(sprite);
            (*sprite).data[6] = (*sprite).data[6] / 30 * 30 + 15;
            (*sprite).callback = Some(SpriteCB_BallLandInSlot);
            m4aSongNumStartOrChange(71);
        } else {
            (*sprite).set_animPaused(TRUE);
            m4aSongNumStart(SE_BALL_BOUNCE_1);
            SetBallStuck(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_TryLand(sprite: *mut Sprite) {
    UpdateBallPos(sprite);
    (*sprite).data[2] = 0;
    UpdateSlotBelowBall(sprite);
    if sRouletteSlots[(*sRoulette).hitSlot].flag & (*sRoulette).hitFlags == 0 {
        (*sRoulette).ballState = BALL_STATE_LANDED;
        (*sRoulette).set_ballRolling(0);
        StartSpriteAnim(sprite, (*sprite).animCmdIndex + 3);
        UpdateSlotBelowBall(sprite);
        (*sprite).data[4] = 30;
        UpdateBallRelativeWheelAngle(sprite);
        (*sprite).data[6] = (*sprite).data[6] / 30 * 30 + 15;
        (*sprite).callback = Some(SpriteCB_BallLandInSlot);
        m4aSongNumStartOrChange(71);
    } else {
        let mut slotId: u8 = 0;
        let mut fallRight: u32 = 0;
        m4aSongNumStart(SE_BALL_BOUNCE_1);
        fallRight = Random() as u32 & 1;
        if fallRight != 0 {
            (*sRoulette).ballAngleSpeed = 0.0f32 as f32;
            (*sRoulette).stuckHitSlot = {
                slotId = (((*sRoulette).hitSlot as i32 + 1) % 12) as u8;
                slotId
            };
        } else {
            let mut temp: f32 = 0.0;
            (*sRoulette).ballAngleSpeed = (({
                temp = sRouletteTables[(*sRoulette).tableId()].var1C as f32;
                temp
            }) as f32
                * 2.0f32 as f32) as f32;
            slotId = (((*sRoulette).hitSlot as i32 + NUM_ROULETTE_SLOTS as i32 - 1) % 12) as u8;
            (*sRoulette).stuckHitSlot = (*sRoulette).hitSlot;
        }
        if sRouletteSlots[slotId].flag & (*sRoulette).hitFlags != 0 {
            (*sprite).data[0] = TRUE as i16;
            (*sprite).data[2] = sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i16;
        } else {
            (*sprite).data[0] = FALSE as i16;
            if (*sRoulette).tableId() != 0 {
                (*sprite).data[2] = sRouletteTables[(*sRoulette).tableId()].randDistanceHigh as i16;
            } else {
                (*sprite).data[2] = sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i16;
                if fallRight != 0 {
                    (*sRoulette).ballAngleSpeed = 0.5f32 as f32;
                } else {
                    (*sRoulette).ballAngleSpeed = -1.5f32 as f32;
                }
            }
        }
        (*sRoulette).ballFallSpeed = 0.085f32 as f32;
        (*sprite).callback = Some(SpriteCB_RollBall_TryLandAdjacent);
        (*sprite).data[1] = 5;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_Slow(sprite: *mut Sprite) {
    UpdateBallPos(sprite);
    if (*sRoulette).ballAngleSpeed as f32 > 0.5f32 as f32 {
        return;
    }
    UpdateSlotBelowBall(sprite);
    if GetBallDistanceToSlotMidpoint(sprite) == 0 {
        (*sRoulette).ballAngleAccel = 0.0f32 as f32;
        (*sRoulette).ballAngleSpeed -= (sRouletteTables[(*sRoulette).tableId()].wheelSpeed as f32
            as f32
            / (sRouletteTables[(*sRoulette).tableId()].wheelDelay as i32 + 1) as f32)
            as f32;
        (*sprite).data[1] = 4;
        (*sprite).callback = Some(SpriteCB_RollBall_TryLand);
    } else {
        if (*sRoulette).ballAngleAccel as f32 != 0.0f32 as f32 {
            if ((*sRoulette).ballAngleSpeed as f32) < 0.0f32 as f32 {
                (*sRoulette).ballAngleAccel = 0.0f32 as f32;
                (*sRoulette).ballAngleSpeed = 0.0f32 as f32;
                (*sRoulette).ballFallSpeed /= 1.2f32 as f32;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_Medium(sprite: *mut Sprite) {
    UpdateBallPos(sprite);
    if (*sRoulette).ballDistToCenter as f32 > 40.0f32 as f32 {
        return;
    }
    (*sRoulette).ballFallSpeed =
        -(4.0f32 as f32 / (*sRoulette).ballTravelDistSlow as f32 as f32) as f32;
    (*sRoulette).ballAngleAccel = -((*sRoulette).ballAngleSpeed as f32
        / (*sRoulette).ballTravelDistSlow as f32 as f32) as f32;
    (*sprite).animNum = 2;
    (*sprite).set_animBeginning(TRUE as u16);
    (*sprite).set_animEnded(FALSE as u16);
    (*sprite).data[1] = 3;
    (*sprite).callback = Some(SpriteCB_RollBall_Slow);
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_Fast(sprite: *mut Sprite) {
    UpdateBallPos(sprite);
    if (*sRoulette).ballDistToCenter as f32 > 60.0f32 as f32 {
        return;
    }
    m4aSongNumStartOrChange(SE_ROULETTE_BALL2);
    (*sRoulette).ballFallSpeed =
        -(20.0f32 as f32 / (*sRoulette).ballTravelDistMed as f32 as f32) as f32;
    (*sRoulette).ballAngleAccel = ((1.0f32 as f32 - (*sRoulette).ballAngleSpeed as f32) as f32
        / (*sRoulette).ballTravelDistMed as f32 as f32) as f32;
    (*sprite).animNum = 1;
    (*sprite).set_animBeginning(TRUE as u16);
    (*sprite).set_animEnded(FALSE as u16);
    (*sprite).data[1] = 2;
    (*sprite).callback = Some(SpriteCB_RollBall_Medium);
}
pub(crate) unsafe extern "C" fn SpriteCB_RollBall_Start(sprite: *mut Sprite) {
    (*sprite).data[1] = 1;
    (*sprite).data[2] = 0;
    UpdateBallPos(sprite);
    (*sprite).set_invisible(FALSE as u16);
    (*sprite).callback = Some(SpriteCB_RollBall_Fast);
}
pub(crate) unsafe extern "C" fn CreateShroomishSprite(ball: *mut Sprite) {
    let mut t: u16 = 0;
    let mut i: u8 = 0;
    let mut coords: CArray<CArray<i16, 2>, 2> = zeroed();
    coords[0][0] = 116;
    coords[0][1] = 44;
    coords[1][0] = 116;
    coords[1][1] = 112;
    let mut roulette: *mut Roulette = null_mut();
    t = (*ball).data[7] as u16 - 2;
    roulette = sRoulette;
    (*sRoulette).spriteIds[55] = CreateSprite(
        (&raw const *sSpriteTemplate_Shroomish).cast_mut(),
        36,
        -12,
        50,
    );
    (*sRoulette).spriteIds[56] = CreateSprite(
        (&raw const sSpriteTemplate_ShroomishShadow[0]).cast_mut(),
        coords[(*ball).data[0]][0],
        coords[(*ball).data[0]][1],
        59,
    );
    (*sRoulette).spriteIds[57] = CreateSprite(
        (&raw const sSpriteTemplate_ShroomishShadow[1]).cast_mut(),
        36,
        140,
        51,
    );
    gSprites[(*sRoulette).spriteIds[57]]
        .oam
        .set_objMode(ST_OAM_OBJ_BLEND);
    i = 0;
    while i < 3 {
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CLEAR_MON]]
            .set_coordOffsetEnabled(FALSE as u16);
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CLEAR_MON]].set_invisible(TRUE as u16);
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CLEAR_MON]].set_animPaused(TRUE);
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CLEAR_MON]].set_affineAnimPaused(TRUE);
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CLEAR_MON]].data[4] =
            (*sRoulette).spriteIds[55] as i16;
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CLEAR_MON]].data[5] =
            (*sRoulette).spriteIds[56] as i16;
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CLEAR_MON]].data[6] =
            (*sRoulette).spriteIds[57] as i16;
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CLEAR_MON]].data[2] = t as i16;
        gSprites[(*sRoulette).spriteIds[i as i32 + SPR_CLEAR_MON]].data[3] = (*ball).data[7]
            * sRouletteTables[(*sRoulette).tableId()].randDistanceHigh as i16
            + (sRouletteTables[(*sRoulette).tableId()].randDistanceLow as i16 + -1);
        i += 1;
    }
    gSprites[(*sRoulette).spriteIds[56]].set_coordOffsetEnabled(TRUE as u16);
    (*sRoulette).ball = ball;
}
pub(crate) unsafe extern "C" fn CreateTaillowSprite(ball: *mut Sprite) {
    let mut i: u8 = 0;
    let mut t: i16 = 0;
    let mut coords: CArray<CArray<i16, 2>, 2> = zeroed();
    coords[0][0] = 256;
    coords[0][1] = 84;
    coords[1][0] = -16;
    coords[1][1] = 84;
    t = (*ball).data[7] - 2;
    (*sRoulette).spriteIds[55] = CreateSprite(
        (&raw const *sSpriteTemplate_Taillow).cast_mut(),
        coords[(*ball).data[0]][0],
        coords[(*ball).data[0]][1],
        50,
    );
    StartSpriteAnim(
        &raw mut gSprites[(*sRoulette).spriteIds[55]],
        (*ball).data[0] as u8,
    );
    (*sRoulette).spriteIds[56] = CreateSprite(
        (&raw const *sSpriteTemplate_TaillowShadow).cast_mut(),
        coords[(*ball).data[0]][0],
        coords[(*ball).data[0]][1],
        51,
    );
    gSprites[(*sRoulette).spriteIds[56]].set_affineAnimPaused(TRUE);
    gSprites[(*sRoulette).spriteIds[56]].set_animPaused(TRUE);
    (*ball).data[7] = t * sRouletteTables[(*sRoulette).tableId()].randDistanceHigh as i16
        + (sRouletteTables[(*sRoulette).tableId()]
            .taillow
            .baseDropDelay as i16
            + 45);
    while i < 2 {
        gSprites[(*sRoulette).spriteIds[SPR_CLEAR_MON + i as i32]].data[4] =
            (*sRoulette).spriteIds[55] as i16;
        gSprites[(*sRoulette).spriteIds[SPR_CLEAR_MON + i as i32]].data[5] =
            (*sRoulette).spriteIds[56] as i16;
        gSprites[(*sRoulette).spriteIds[SPR_CLEAR_MON + i as i32]].data[6] =
            (*sRoulette).spriteIds[56] as i16;
        gSprites[(*sRoulette).spriteIds[SPR_CLEAR_MON + i as i32]].data[2] = t;
        gSprites[(*sRoulette).spriteIds[SPR_CLEAR_MON + i as i32]].data[3] = (*ball).data[7] - 45;
        i += 1;
    }
    (*sRoulette).ball = ball;
}
pub(crate) unsafe extern "C" fn SetBallStuck(sprite: *mut Sprite) {
    let mut slotId: u8 = 0;
    let mut angle: u16 = 0;
    let mut numCandidates: u8 = 0;
    let mut maxSlotToCheck: u8 = 5;
    let mut betSlotId: u8 = 0;
    let mut i: u8 = 0;
    let mut slotsToSkip: u8 = 0;
    let mut slotCandidates: CArray<u8, 10> = CArray([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mut rand: u16 = Random();
    (*sRoulette).ballState = BALL_STATE_STUCK;
    (*sRoulette).set_ballStuck(TRUE);
    (*sRoulette).set_ballUnstuck(FALSE);
    (*sRoulette).hitSlot = 0xFF;
    (*sRoulette).ballAngle = (*sprite).data[3] as f32;
    (*sRoulette).ballFallSpeed = 0.0f32 as f32;
    (*sRoulette).ballAngleSpeed = sRouletteTables[(*sRoulette).tableId()].var1C as f32;
    angle = (*sRoulette).tableId() as u16 * DEGREES_PER_SLOT
        + 33
        + (1 - (*sRoulette).useTaillow() as u16) * 15;
    i = 0;
    while i < 4 {
        if (angle as i32) < (*sprite).data[3] as i32
            && (*sprite).data[3] as i32 <= angle as i32 + 90
        {
            (*sprite).data[0] = (i as i32 / 2) as i16;
            (*sRoulette).set_useTaillow((i as i32 % 2) as u8);
            break;
        }
        if i == 3 {
            (*sprite).data[0] = TRUE as i16;
            (*sRoulette).set_useTaillow(TRUE);
            break;
        }
        angle += 90;
        i += 1;
    }
    if (*sRoulette).useTaillow() != 0 {
        if (*sprite).data[0] != 0 {
            PlayCry_Normal(SPECIES_TAILLOW, -63);
        } else {
            PlayCry_Normal(SPECIES_TAILLOW, 63);
        }
    } else {
        PlayCry_Normal(SPECIES_SHROOMISH as u16, -63);
    }
    slotsToSkip = 2;
    slotId = (((*sRoulette).stuckHitSlot as i32 + 2) % 12) as u8;
    if (*sRoulette).useTaillow() == 1 && (*sRoulette).tableId() == 1 {
        maxSlotToCheck += 6;
    } else {
        maxSlotToCheck += slotsToSkip;
    }
    i = slotsToSkip;
    while i < maxSlotToCheck {
        if (*sRoulette).hitFlags & sRouletteSlots[slotId].flag == 0 {
            slotCandidates[{
                let t1 = numCandidates;
                numCandidates += 1;
                t1
            }] = i;
            if betSlotId == 0
                && sRouletteSlots[slotId].flag
                    & sGridSelections[(*sRoulette).betSelection[(*sRoulette).curBallNum()]]
                        .inSelectionFlags
                    != 0
            {
                betSlotId = i;
            }
        }
        slotId = ((slotId as i32 + 1) % 12) as u8;
        i += 1;
    }
    if (*sRoulette).useTaillow() as i32 + 1 & (*sRoulette).partySpeciesFlags as i32 != 0 {
        if betSlotId != 0 && rand as i32 % 256 < 192 {
            (*sprite).data[7] = betSlotId as i16;
        } else {
            (*sprite).data[7] = slotCandidates[rem_i32(rand as i32, numCandidates as i32)] as i16;
        }
    } else {
        (*sprite).data[7] = slotCandidates[rem_i32(rand as i32, numCandidates as i32)] as i16;
    }
    (*sprite).callback = Some(SpriteCB_UnstickBall);
}
pub(crate) unsafe extern "C" fn SpriteCB_ShroomishExit(sprite: *mut Sprite) {
    if ({
        let t1 = (*sprite).data[1];
        (*sprite).data[1] += 1;
        t1
    }) >= (*sprite).data[3]
    {
        (*sprite).x -= 2;
        if (*sprite).x < -16 {
            if (*sRoulette).ballUnstuck() == 0 {
                (*sRoulette).set_ballUnstuck(TRUE);
            }
            DestroySprite(sprite);
            (*sRoulette).shroomishShadowTimer = 0;
            (*sRoulette).shroomishShadowAlpha = sShroomishShadowAlphas[0];
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShroomishShakeScreen(sprite: *mut Sprite) {
    let mut screenShakeIdx: i32 = 0;
    let mut screenShakeOffsets: CArray<CArray<u16, 4>, 3> = zeroed();
    screenShakeOffsets[0][0] = 65535;
    screenShakeOffsets[0][1] = 0;
    screenShakeOffsets[0][2] = 1;
    screenShakeOffsets[0][3] = 0;
    screenShakeOffsets[1][0] = 65534;
    screenShakeOffsets[1][1] = 0;
    screenShakeOffsets[1][2] = 2;
    screenShakeOffsets[1][3] = 0;
    screenShakeOffsets[2][0] = 65533;
    screenShakeOffsets[2][1] = 0;
    screenShakeOffsets[2][2] = 3;
    screenShakeOffsets[2][3] = 0;
    if ({
        let t1 = (*sprite).data[1];
        (*sprite).data[1] += 1;
        t1
    }) < (*sprite).data[3]
    {
        if (*sprite).data[1] as i32 & 1 != 0 {
            gSpriteCoordOffsetY =
                screenShakeOffsets[(*sprite).data[2] / 2][(*sprite).data[7]] as i16;
            screenShakeIdx = (*sprite).data[7] as i32 + 1;
            (*sprite).data[7] = screenShakeIdx as i16 - (screenShakeIdx / 4) as i16 * 4;
        }
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
    } else {
        gSpriteCoordOffsetY = 0;
        gSprites[(*sRoulette).spriteIds[55]].set_animPaused(FALSE);
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShroomishFall(sprite: *mut Sprite) {
    let mut timer: f32 = 0.0;
    (*sprite).data[1] += 1;
    timer = (*sprite).data[1] as f32;
    (*sprite).y2 = ((timer as f32 * 0.039f32 as f32) as f32 * timer as f32) as i16;
    (*sRoulette).shroomishShadowAlpha =
        sShroomishShadowAlphas[((*sRoulette).shroomishShadowTimer as i32 - 1) / 2];
    if (*sRoulette).shroomishShadowTimer < 19 {
        (*sRoulette).shroomishShadowTimer += 1;
    }
    if (*sprite).data[1] > 60 {
        (*sprite).data[1] = 0;
        (*sprite).callback = Some(SpriteCB_ShroomishExit);
        gSprites[(*sprite).data[6]].callback = Some(SpriteCB_ShroomishExit);
        gSprites[(*sprite).data[6]].data[1] = -2;
        gSprites[(*sprite).data[5]].set_invisible(FALSE as u16);
        gSprites[(*sprite).data[5]].callback = Some(SpriteCB_ShroomishShakeScreen);
        m4aSongNumStart(SE_M_STRENGTH);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Shroomish(sprite: *mut Sprite) {
    if (*sprite).data[7] == 0 {
        if (*(*sRoulette).ball).data[0] == 0 {
            if (*(*sRoulette).ball).data[3] as i32
                != sRouletteTables[(*sRoulette).tableId()].shroomish.startAngle as i32
            {
                return;
            }
        } else {
            if (*(*sRoulette).ball).data[3] as i32
                != sRouletteTables[(*sRoulette).tableId()].shroomish.startAngle as i32 + 180
            {
                return;
            }
        }
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).data[7] += 1;
        m4aSongNumStart(SE_FALL);
        (*sRoulette).shroomishShadowTimer = 1;
        (*sRoulette).shroomishShadowAlpha = sShroomishShadowAlphas[0];
    } else {
        (*sRoulette).shroomishShadowAlpha =
            sShroomishShadowAlphas[((*sRoulette).shroomishShadowTimer as i32 - 1) / 2];
        if (*sRoulette).shroomishShadowTimer < 19 {
            (*sRoulette).shroomishShadowTimer += 1;
        }
        if (*(*sRoulette).ball).data[0] == 0 {
            if (*(*sRoulette).ball).data[3] as i32
                != sRouletteTables[(*sRoulette).tableId()].shroomish.dropAngle as i32
            {
                return;
            }
        } else {
            if (*(*sRoulette).ball).data[3] as i32
                != sRouletteTables[(*sRoulette).tableId()].shroomish.dropAngle as i32 + 180
            {
                return;
            }
        }
        gSprites[(*sprite).data[4]].callback = Some(SpriteCB_ShroomishFall);
        gSprites[(*sprite).data[4]].set_invisible(FALSE as u16);
        (*sprite).callback = Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite));
        (*sprite).data[7] = 0;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TaillowShadow_Flash(sprite: *mut Sprite) {
    (*sprite).set_invisible((*sprite).invisible() ^ 1);
}
pub(crate) unsafe extern "C" fn SpriteCB_Taillow_FlyAway(sprite: *mut Sprite) {
    if (*sprite).y > -16 {
        (*sprite).y -= 1;
    } else {
        (*sprite).callback = Some(SpriteCallbackDummy);
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).set_animPaused(TRUE);
        m4aSongNumStop(SE_TAILLOW_WING_FLAP);
        DestroySprite(sprite);
        FreeOamMatrix(gSprites[(*sRoulette).spriteIds[56]].oam.matrixNum() as u8);
        DestroySprite(&raw mut gSprites[(*sRoulette).spriteIds[56]]);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Taillow_PickUpBall(sprite: *mut Sprite) {
    if (*sprite).data[1] >= 0 {
        (*sprite).data[1] -= 1;
        (*sprite).y -= 1;
        if (*sprite).data[1] == 0 && (*sprite).animCmdIndex == 1 {
            (*sprite).y2 += 1;
        }
    } else {
        if (*sprite).data[3] >= 0 {
            (*sprite).data[3] -= 1;
            if (*sprite).animDelayCounter() == 0 {
                if (*sprite).animCmdIndex == 1 {
                    (*sprite).y2 += 1;
                } else {
                    (*sprite).y2 -= 1;
                }
            }
        } else {
            m4aSongNumStart(SE_FALL);
            StartSpriteAnim(sprite, (*(*sRoulette).ball).data[0] as u8 + 4);
            (*sprite).callback = Some(SpriteCB_Taillow_FlyAway);
            gSprites[(*sprite).data[6]].set_affineAnimPaused(FALSE);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Taillow_FlyIn(sprite: *mut Sprite) {
    let mut xMoveOffsets: CArray<i8, 2> = CArray([-1, 1]);
    let mut yMoveOffsets: CArray<CArray<i8, 2>, 8> = zeroed();
    yMoveOffsets[0][0] = 2;
    yMoveOffsets[0][1] = 0;
    yMoveOffsets[1][0] = 2;
    yMoveOffsets[1][1] = 0;
    yMoveOffsets[2][0] = 2;
    yMoveOffsets[2][1] = -1;
    yMoveOffsets[3][0] = 2;
    yMoveOffsets[3][1] = -1;
    yMoveOffsets[4][0] = 2;
    yMoveOffsets[4][1] = -1;
    yMoveOffsets[5][0] = 2;
    yMoveOffsets[5][1] = -1;
    yMoveOffsets[6][0] = 2;
    yMoveOffsets[6][1] = -2;
    yMoveOffsets[7][0] = 2;
    yMoveOffsets[7][1] = -2;
    if ({
        let t1 = (*sprite).data[1];
        (*sprite).data[1] -= 1;
        t1
    }) > 7
    {
        (*sprite).x += xMoveOffsets[(*(*sRoulette).ball).data[0]] as i16 * 2;
        if IsSEPlaying() != 0 {
            let mut pan: i8 = -(((116 - (*sprite).x as i32) / 2) as i8);
            m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE1, TRACKS_ALL, pan);
            m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, pan);
        }
    } else {
        if (*sprite).data[1] >= 0 {
            (*sprite).x += xMoveOffsets[(*(*sRoulette).ball).data[0]] as i16
                * yMoveOffsets[7 - (*sprite).data[1] as i32][0] as i16;
            (*sprite).y += yMoveOffsets[7 - (*sprite).data[1] as i32][1] as i16;
        } else {
            m4aSongNumStartOrChange(SE_TAILLOW_WING_FLAP);
            if (*(*sRoulette).ball).data[0] == 0 {
                PlayCry_Normal(SPECIES_TAILLOW, 63);
            } else {
                PlayCry_Normal(SPECIES_TAILLOW, -63);
            }
            StartSpriteAnim(sprite, (*(*sRoulette).ball).data[0] as u8 + 2);
            (*sprite).data[1] = 45;
            (*sprite).callback = Some(SpriteCB_Taillow_PickUpBall);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TaillowShadow_FlyIn(sprite: *mut Sprite) {
    let mut moveDir: CArray<i8, 2> = CArray([-1, 1]);
    if ({
        let t1 = (*sprite).data[1];
        (*sprite).data[1] -= 1;
        t1
    }) >= 0
    {
        (*sprite).x += moveDir[(*(*sRoulette).ball).data[0]] as i16 * 2;
        gSprites[(*sprite).data[6]].set_invisible(gSprites[(*sprite).data[6]].invisible() ^ 1);
    } else {
        (*sprite).callback = Some(SpriteCB_TaillowShadow_Flash);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Taillow(sprite: *mut Sprite) {
    if (*(*sRoulette).ball).data[0] == FALSE as i16 {
        if (*(*sRoulette).ball).data[3] as i32
            == sRouletteTables[(*sRoulette).tableId()]
                .taillow
                .rightStartAngle as i32
                + 90
        {
            gSprites[(*sprite).data[6]].data[1] = 52;
            gSprites[(*sprite).data[4]].data[1] = 52;
        } else {
            return;
        }
    } else {
        if (*(*sRoulette).ball).data[3] as i32
            == sRouletteTables[(*sRoulette).tableId()]
                .taillow
                .leftStartAngle as i32
                + 270
        {
            gSprites[(*sprite).data[6]].data[1] = 46;
            gSprites[(*sprite).data[4]].data[1] = 46;
        } else {
            return;
        }
    }
    gSprites[(*sprite).data[6]].callback = Some(SpriteCB_TaillowShadow_FlyIn);
    gSprites[(*sprite).data[4]].callback = Some(SpriteCB_Taillow_FlyIn);
    m4aSongNumStart(SE_FALL);
}
