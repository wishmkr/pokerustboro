//! Translated from `src/battle_interface.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::unnecessary_cast,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::battle_anim_mons::{GetBattlerPosition, GetBattlerSide, IsDoubleBattle};
use crate::battle_gfx_sfx_util::LoadBattleBarGfx;
use crate::battle_main::{
    gBattleSpritesDataPtr, gBattleStruct, gBattleTypeFlags, gBattlersCount, gMonSpritesGfxPtr,
};
use crate::battle_main::{
    gBattlerPartyIndexes, gBattlerPositions, gDisplayedStringBattle, gHealthboxSpriteIds,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::menu::AddTextPrinterParameterized4;
use crate::palette::FillPalette;
use crate::pokedex::GetSetPokedexFlag;
use crate::pokemon::{
    GetMonData2, GetMonData3, GetMonGender, GetNature, SpeciesToNationalPokedexNum, gEnemyParty,
    gPlayerParty,
};
use crate::safari_zone::gNumSafariBalls;
use crate::sound::{PlaySE1WithPanning, PlaySE2WithPanning, PlaySE12WithPanning};
use crate::sprite::gSprites;
use crate::string_util::StringGet_Nickname;
use crate::task::{DestroyTask, TaskDummy};
use crate::task::{task_get, task_set, task_set_func};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{FillWindowPixelBuffer, GetWindowAttribute, RemoveWindow};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
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
/// `DestroySpriteAndFreeResources` with this module's view of its types.
#[inline]
unsafe fn DestroySpriteAndFreeResources(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySpriteAndFreeResources(a0 as _);
    }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `GetStringRightAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringRightAlignXOffset(a0, a1 as _, a2) }
}
/// `LoadCompressedSpriteSheetUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpriteSheetUsingHeap(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `RenderTextHandleBold` with this module's view of its types.
#[inline]
unsafe fn RenderTextHandleBold(a0: *mut u8, a1: u8, a2: *mut u8) -> u8 {
    unsafe { crate::text::RenderTextHandleBold(a0 as _, a1, a2 as _) }
}
/// `SetSubspriteTables` with this module's view of its types.
#[inline]
unsafe fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable) {
    unsafe {
        crate::sprite::SetSubspriteTables(a0 as _, a1 as _);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StringAppend` with this module's view of its types.
#[inline]
unsafe fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringAppend(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCompare` with this module's view of its types.
#[inline]
unsafe fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32 {
    unsafe { crate::string_util::StringCompare(a0 as _, a1 as _) }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tBattler: usize = 0;
const tSummaryBarSpriteId: usize = 1;
const hBar_HealthBoxSpriteId: usize = 5;
const hMain_HealthBarSpriteId: usize = 5;
const hOther_HealthBoxSpriteId: usize = 5;
const hBar_Data6: usize = 6;
const hMain_Battler: usize = 6;
const hMain_Data7: usize = 7;
const tIsBattleStart: usize = 10;
const tBlend: usize = 15;
// Data tables (translate with cdata.py): sOamData_64x32 sHealthboxPlayerSpriteTemplates sHealthboxOpponentSpriteTemplates sHealthboxSafariSpriteTemplate sOamData_Healthbar sHealthbarSpriteTemplates sUnused_Subsprites_0 sUnused_Subsprites_2 sUnused_Subsprites_1 sUnused_Subsprites_3 sHealthBar_Subsprites_Player sHealthBar_Subsprites_Opponent sUnused_SubspriteTable sHealthBar_SubspriteTables sStatusSummaryBar_Subsprites_Enter sStatusSummaryBar_Subsprites_Exit sStatusSummaryBar_SubspriteTable_Enter sStatusSummaryBar_SubspriteTable_Exit sUnusedStatusSummary sStatusSummaryBarSpriteSheet sStatusSummaryBarSpritePal sStatusSummaryBallsSpritePal sStatusSummaryBallsSpriteSheet sOamData_Unused64x32 sOamData_StatusSummaryBalls sStatusSummaryBarSpriteTemplates sStatusSummaryBallsSpriteTemplates sEmptyWhiteText_GrayHighlight sEmptyWhiteText_TransparentHighlight sStatusIconColors sHealthboxWindowTemplate

/// `struct TestingBar`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TestingBar {
    pub maxValue: i32,
    pub oldValue: i32,
    pub receivedValue: i32,
    bits_12: u8,
    pub unk10: u32,
}

impl TestingBar {
    #[inline(always)]
    pub fn unkC_0(&self) -> u32 {
        (self.bits_12 as u32) & 0x1f
    }
    #[inline(always)]
    pub fn set_unkC_0(&mut self, v: u32) {
        self.bits_12 = (self.bits_12 & !0x1f) | (v as u8 & 0x1f);
    }
}

unsafe impl Sync for TestingBar {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<TestingBar>() == 20);
    assert!(offset_of!(TestingBar, maxValue) == 0);
    assert!(offset_of!(TestingBar, oldValue) == 4);
    assert!(offset_of!(TestingBar, receivedValue) == 8);
    assert!(offset_of!(TestingBar, bits_12) == 12);
    assert!(offset_of!(TestingBar, unk10) == 16);
};

const B_EXPBAR_PIXELS: i32 = 64;
const B_HEALTHBAR_PIXELS: i32 = 48;
const HEALTHBOX_GFX_0: u8 = 0;
const HEALTHBOX_GFX_1: u8 = 1;
const HEALTHBOX_GFX_12: u8 = 12;
const HEALTHBOX_GFX_39: u8 = 39;
const HEALTHBOX_GFX_65: u8 = 65;
const HEALTHBOX_GFX_FRAME_END: u8 = 116;
const HEALTHBOX_GFX_FRAME_END_BAR: u8 = 117;
const HEALTHBOX_GFX_HP_BAR_GREEN: u8 = 3;
const HEALTHBOX_GFX_HP_BAR_RED: u8 = 56;
const HEALTHBOX_GFX_HP_BAR_YELLOW: u8 = 47;
const HEALTHBOX_GFX_STATUS_BALL_CAUGHT: u8 = 70;
const HEALTHBOX_GFX_STATUS_BRN_BATTLER0: u8 = 33;
const HEALTHBOX_GFX_STATUS_BRN_BATTLER1: u8 = 83;
const HEALTHBOX_GFX_STATUS_BRN_BATTLER2: u8 = 98;
const HEALTHBOX_GFX_STATUS_BRN_BATTLER3: u8 = 113;
const HEALTHBOX_GFX_STATUS_FRZ_BATTLER0: u8 = 30;
const HEALTHBOX_GFX_STATUS_FRZ_BATTLER1: u8 = 80;
const HEALTHBOX_GFX_STATUS_FRZ_BATTLER2: u8 = 95;
const HEALTHBOX_GFX_STATUS_FRZ_BATTLER3: u8 = 110;
const HEALTHBOX_GFX_STATUS_PRZ_BATTLER0: u8 = 24;
const HEALTHBOX_GFX_STATUS_PRZ_BATTLER1: u8 = 74;
const HEALTHBOX_GFX_STATUS_PRZ_BATTLER2: u8 = 89;
const HEALTHBOX_GFX_STATUS_PRZ_BATTLER3: u8 = 104;
const HEALTHBOX_GFX_STATUS_PSN_BATTLER0: u8 = 21;
const HEALTHBOX_GFX_STATUS_PSN_BATTLER1: u8 = 71;
const HEALTHBOX_GFX_STATUS_PSN_BATTLER2: u8 = 86;
const HEALTHBOX_GFX_STATUS_PSN_BATTLER3: u8 = 101;
const HEALTHBOX_GFX_STATUS_SLP_BATTLER0: u8 = 27;
const HEALTHBOX_GFX_STATUS_SLP_BATTLER1: u8 = 77;
const HEALTHBOX_GFX_STATUS_SLP_BATTLER2: u8 = 92;
const HEALTHBOX_GFX_STATUS_SLP_BATTLER3: u8 = 107;
const PAL_STATUS_BRN: u8 = 4;
const PAL_STATUS_FRZ: u8 = 3;
const PAL_STATUS_PAR: u8 = 1;
const PAL_STATUS_PSN: u8 = 0;
const PAL_STATUS_SLP: u8 = 2;

static sEmptyWhiteText_GrayHighlight: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::battle_interface::sEmptyWhiteText_GrayHighlight).cast());
static sEmptyWhiteText_TransparentHighlight: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::battle_interface::sEmptyWhiteText_TransparentHighlight).cast());
static sHealthBar_SubspriteTables: Table<CArray<SubspriteTable, 2>> =
    Table((&raw const crate::data::battle_interface::sHealthBar_SubspriteTables).cast());
static sHealthbarSpriteTemplates: Table<CArray<SpriteTemplate, 4>> =
    Table((&raw const crate::data::battle_interface::sHealthbarSpriteTemplates).cast());
static sHealthboxOpponentSpriteTemplates: Table<CArray<SpriteTemplate, 2>> =
    Table((&raw const crate::data::battle_interface::sHealthboxOpponentSpriteTemplates).cast());
static sHealthboxPlayerSpriteTemplates: Table<CArray<SpriteTemplate, 2>> =
    Table((&raw const crate::data::battle_interface::sHealthboxPlayerSpriteTemplates).cast());
static sHealthboxSafariSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_interface::sHealthboxSafariSpriteTemplate).cast());
static sHealthboxWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::battle_interface::sHealthboxWindowTemplate).cast());
static sStatusIconColors: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::battle_interface::sStatusIconColors).cast());
static sStatusSummaryBallsSpritePal: Table<SpritePalette> =
    Table((&raw const crate::data::battle_interface::sStatusSummaryBallsSpritePal).cast());
static sStatusSummaryBallsSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::battle_interface::sStatusSummaryBallsSpriteSheet).cast());
static sStatusSummaryBallsSpriteTemplates: Table<CArray<SpriteTemplate, 2>> =
    Table((&raw const crate::data::battle_interface::sStatusSummaryBallsSpriteTemplates).cast());
static sStatusSummaryBarSpritePal: Table<SpritePalette> =
    Table((&raw const crate::data::battle_interface::sStatusSummaryBarSpritePal).cast());
static sStatusSummaryBarSpriteSheet: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::battle_interface::sStatusSummaryBarSpriteSheet).cast());
static sStatusSummaryBarSpriteTemplates: Table<CArray<SpriteTemplate, 2>> =
    Table((&raw const crate::data::battle_interface::sStatusSummaryBarSpriteTemplates).cast());
static sStatusSummaryBar_SubspriteTable_Enter: Table<CArray<SubspriteTable, 1>> = Table(
    (&raw const crate::data::battle_interface::sStatusSummaryBar_SubspriteTable_Enter).cast(),
);
static sStatusSummaryBar_SubspriteTable_Exit: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::battle_interface::sStatusSummaryBar_SubspriteTable_Exit).cast());

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

unsafe fn DummiedOutFunction(unused1: i16, unused2: i16, unused3: i32) -> i32 {
    9
}
unsafe fn Debug_DrawNumber(mut number: i16, dest: *mut u16, unk: u8) {
    let mut j: i8 = 0;
    let mut buff: CArray<u8, 4> = zeroed();
    for i in 0..4i8 {
        buff[i] = 0;
    }
    let mut i: i8 = 3;
    loop {
        if number > 0 {
            buff[i] = (number % 10) as u8;
            number /= 10;
        } else {
            while i > -1 {
                buff[i] = 0xFF;
                i -= 1;
            }
            if buff[3] == 0xFF {
                buff[3] = 0;
            }
            break;
        }
        i -= 1;
    }
    if unk == 0 {
        j = 0;
        for i in 0..4i8 {
            if buff[j] == 0xFF {
                *dest.at(j as i32) &= 0xFC00;
                *dest.at(j as i32) |= 0x1E;
                *dest.at(i as i32 + 0x20) &= 0xFC00;
                *dest.at(i as i32 + 0x20) |= 0x1E;
            } else {
                *dest.at(j as i32) &= 0xFC00;
                *dest.at(j as i32) |= 0x14 + buff[j] as u16;
                *dest.at(i as i32 + 0x20) &= 0xFC00;
                *dest.at(i as i32 + 0x20) |= 0x34 + buff[i] as u16;
            }
            j += 1;
        }
    } else {
        for i in 0..4i8 {
            if buff[i] == 0xFF {
                *dest.at(i as i32) &= 0xFC00;
                *dest.at(i as i32) |= 0x1E;
                *dest.at(i as i32 + 0x20) &= 0xFC00;
                *dest.at(i as i32 + 0x20) |= 0x1E;
            } else {
                *dest.at(i as i32) &= 0xFC00;
                *dest.at(i as i32) |= 0x14 + buff[i] as u16;
                *dest.at(i as i32 + 0x20) &= 0xFC00;
                *dest.at(i as i32 + 0x20) |= 0x34 + buff[i] as u16;
            }
        }
    }
}
unsafe fn Debug_DrawNumberPair(number1: i16, number2: i16, dest: *mut u16) {
    *dest.at(4) = 0x1E;
    Debug_DrawNumber(number2, dest, FALSE);
    Debug_DrawNumber(number1, dest.at(5), TRUE);
}
#[unsafe(no_mangle)]
pub unsafe fn CreateBattlerHealthboxSprites(battler: u8) -> u8 {
    let mut data6: i16 = 0;
    let mut healthboxLeftSpriteId: u8 = 0;
    let mut healthboxRightSpriteId: u8 = 0;
    if IsDoubleBattle() == 0 {
        if GetBattlerSide(battler) == B_SIDE_PLAYER {
            healthboxLeftSpriteId = CreateSprite(
                (&raw const sHealthboxPlayerSpriteTemplates[0]).cast_mut(),
                DISPLAY_WIDTH as i16,
                DISPLAY_HEIGHT as i16,
                1,
            );
            healthboxRightSpriteId = CreateSpriteAtEnd(
                (&raw const sHealthboxPlayerSpriteTemplates[0]).cast_mut(),
                DISPLAY_WIDTH as i16,
                DISPLAY_HEIGHT as i16,
                1,
            );
            gSprites[healthboxLeftSpriteId].oam.set_shape(ST_OAM_SQUARE);
            gSprites[healthboxRightSpriteId]
                .oam
                .set_shape(ST_OAM_SQUARE);
            gSprites[healthboxRightSpriteId]
                .oam
                .set_tileNum(gSprites[healthboxRightSpriteId].oam.tileNum() + 64);
        } else {
            healthboxLeftSpriteId = CreateSprite(
                (&raw const sHealthboxOpponentSpriteTemplates[0]).cast_mut(),
                DISPLAY_WIDTH as i16,
                DISPLAY_HEIGHT as i16,
                1,
            );
            healthboxRightSpriteId = CreateSpriteAtEnd(
                (&raw const sHealthboxOpponentSpriteTemplates[0]).cast_mut(),
                DISPLAY_WIDTH as i16,
                DISPLAY_HEIGHT as i16,
                1,
            );
            gSprites[healthboxRightSpriteId]
                .oam
                .set_tileNum(gSprites[healthboxRightSpriteId].oam.tileNum() + 32);
            data6 = 2;
        }
        gSprites[healthboxLeftSpriteId].oam.affineParam = healthboxRightSpriteId as u16;
        gSprites[healthboxRightSpriteId].data[5] = healthboxLeftSpriteId as i16;
        gSprites[healthboxRightSpriteId].callback = Some(SpriteCB_HealthBoxOther);
    } else {
        if GetBattlerSide(battler) == B_SIDE_PLAYER {
            healthboxLeftSpriteId = CreateSprite(
                (&raw const sHealthboxPlayerSpriteTemplates
                    [GetBattlerPosition(battler) as i32 / 2])
                    .cast_mut(),
                DISPLAY_WIDTH as i16,
                DISPLAY_HEIGHT as i16,
                1,
            );
            healthboxRightSpriteId = CreateSpriteAtEnd(
                (&raw const sHealthboxPlayerSpriteTemplates
                    [GetBattlerPosition(battler) as i32 / 2])
                    .cast_mut(),
                DISPLAY_WIDTH as i16,
                DISPLAY_HEIGHT as i16,
                1,
            );
            gSprites[healthboxLeftSpriteId].oam.affineParam = healthboxRightSpriteId as u16;
            gSprites[healthboxRightSpriteId].data[5] = healthboxLeftSpriteId as i16;
            gSprites[healthboxRightSpriteId]
                .oam
                .set_tileNum(gSprites[healthboxRightSpriteId].oam.tileNum() + 32);
            gSprites[healthboxRightSpriteId].callback = Some(SpriteCB_HealthBoxOther);
            data6 = 1;
        } else {
            healthboxLeftSpriteId = CreateSprite(
                (&raw const sHealthboxOpponentSpriteTemplates
                    [GetBattlerPosition(battler) as i32 / 2])
                    .cast_mut(),
                DISPLAY_WIDTH as i16,
                DISPLAY_HEIGHT as i16,
                1,
            );
            healthboxRightSpriteId = CreateSpriteAtEnd(
                (&raw const sHealthboxOpponentSpriteTemplates
                    [GetBattlerPosition(battler) as i32 / 2])
                    .cast_mut(),
                DISPLAY_WIDTH as i16,
                DISPLAY_HEIGHT as i16,
                1,
            );
            gSprites[healthboxLeftSpriteId].oam.affineParam = healthboxRightSpriteId as u16;
            gSprites[healthboxRightSpriteId].data[5] = healthboxLeftSpriteId as i16;
            gSprites[healthboxRightSpriteId]
                .oam
                .set_tileNum(gSprites[healthboxRightSpriteId].oam.tileNum() + 32);
            gSprites[healthboxRightSpriteId].callback = Some(SpriteCB_HealthBoxOther);
            data6 = 2;
        }
    }
    let healthbarSpriteId: u8 = CreateSpriteAtEnd(
        (&raw const sHealthbarSpriteTemplates[gBattlerPositions[battler]]).cast_mut(),
        140,
        60,
        0,
    );
    let healthBarSpritePtr: *mut Sprite = &raw mut gSprites[healthbarSpriteId];
    SetSubspriteTables(
        healthBarSpritePtr,
        (&raw const sHealthBar_SubspriteTables[GetBattlerSide(battler)]).cast_mut(),
    );
    (*healthBarSpritePtr).set_subspriteMode(SUBSPRITES_IGNORE_PRIORITY);
    (*healthBarSpritePtr).oam.set_priority(1);
    CpuSet(
        GetHealthboxElementGfxPtr(HEALTHBOX_GFX_1) as *mut c_void,
        (OBJ_VRAM0 + (*healthBarSpritePtr).oam.tileNum() as i32 * 32) as usize as *mut c_void,
        0x4000010,
    );
    gSprites[healthboxLeftSpriteId].data[5] = healthbarSpriteId as i16;
    gSprites[healthboxLeftSpriteId].data[6] = battler as i16;
    gSprites[healthboxLeftSpriteId].set_invisible(TRUE as u16);
    gSprites[healthboxRightSpriteId].set_invisible(TRUE as u16);
    (*healthBarSpritePtr).data[5] = healthboxLeftSpriteId as i16;
    (*healthBarSpritePtr).data[6] = data6;
    (*healthBarSpritePtr).set_invisible(TRUE as u16);
    healthboxLeftSpriteId
}
#[unsafe(no_mangle)]
pub unsafe fn CreateSafariPlayerHealthboxSprites() -> u8 {
    let healthboxLeftSpriteId: u8 = CreateSprite(
        (&raw const *sHealthboxSafariSpriteTemplate).cast_mut(),
        DISPLAY_WIDTH as i16,
        DISPLAY_HEIGHT as i16,
        1,
    );
    let healthboxRightSpriteId: u8 = CreateSpriteAtEnd(
        (&raw const *sHealthboxSafariSpriteTemplate).cast_mut(),
        DISPLAY_WIDTH as i16,
        DISPLAY_HEIGHT as i16,
        1,
    );
    gSprites[healthboxLeftSpriteId].oam.set_shape(ST_OAM_SQUARE);
    gSprites[healthboxRightSpriteId]
        .oam
        .set_shape(ST_OAM_SQUARE);
    gSprites[healthboxRightSpriteId]
        .oam
        .set_tileNum(gSprites[healthboxRightSpriteId].oam.tileNum() + 64);
    gSprites[healthboxLeftSpriteId].oam.affineParam = healthboxRightSpriteId as u16;
    gSprites[healthboxRightSpriteId].data[hOther_HealthBoxSpriteId] = healthboxLeftSpriteId as i16;
    gSprites[healthboxRightSpriteId].callback = Some(SpriteCB_HealthBoxOther);
    healthboxLeftSpriteId
}
unsafe fn GetHealthboxElementGfxPtr(elementId: u8) -> *mut u8 {
    (*(&raw const crate::data::graphics::gHealthboxElementsGfxTable)
        .cast::<CArray<CArray<u8, 32>, 0>>())[elementId]
        .as_ptr()
        .cast_mut()
}
pub(crate) unsafe fn SpriteCB_HealthBar(sprite: *mut Sprite) {
    let healthboxSpriteId: u8 = (*sprite).data[hBar_HealthBoxSpriteId] as u8;
    match (*sprite).data[hBar_Data6] {
        0 => {
            (*sprite).x = gSprites[healthboxSpriteId].x + 16;
            (*sprite).y = gSprites[healthboxSpriteId].y;
        }
        1 => {
            (*sprite).x = gSprites[healthboxSpriteId].x + 16;
            (*sprite).y = gSprites[healthboxSpriteId].y;
        }
        _ => {
            (*sprite).x = gSprites[healthboxSpriteId].x + 8;
            (*sprite).y = gSprites[healthboxSpriteId].y;
        }
    }
    (*sprite).x2 = gSprites[healthboxSpriteId].x2;
    (*sprite).y2 = gSprites[healthboxSpriteId].y2;
}
pub(crate) unsafe fn SpriteCB_HealthBoxOther(sprite: *mut Sprite) {
    let healthboxMainSpriteId: u8 = (*sprite).data[hOther_HealthBoxSpriteId] as u8;
    (*sprite).x = gSprites[healthboxMainSpriteId].x + 64;
    (*sprite).y = gSprites[healthboxMainSpriteId].y;
    (*sprite).x2 = gSprites[healthboxMainSpriteId].x2;
    (*sprite).y2 = gSprites[healthboxMainSpriteId].y2;
}
pub unsafe fn SetBattleBarStruct(
    battler: u8,
    healthboxSpriteId: u8,
    maxVal: i32,
    oldVal: i32,
    receivedValue: i32,
) {
    (*(*gBattleSpritesDataPtr).battleBars.at(battler)).healthboxSpriteId = healthboxSpriteId;
    (*(*gBattleSpritesDataPtr).battleBars.at(battler)).maxValue = maxVal;
    (*(*gBattleSpritesDataPtr).battleBars.at(battler)).oldValue = oldVal;
    (*(*gBattleSpritesDataPtr).battleBars.at(battler)).receivedValue = receivedValue;
    (*(*gBattleSpritesDataPtr).battleBars.at(battler)).currValue = -32768;
}
#[unsafe(no_mangle)]
pub unsafe fn SetHealthboxSpriteInvisible(healthboxSpriteId: u8) {
    gSprites[healthboxSpriteId].set_invisible(TRUE as u16);
    gSprites[gSprites[healthboxSpriteId].data[hMain_HealthBarSpriteId]].set_invisible(TRUE as u16);
    gSprites[gSprites[healthboxSpriteId].oam.affineParam].set_invisible(TRUE as u16);
}
#[unsafe(no_mangle)]
pub unsafe fn SetHealthboxSpriteVisible(healthboxSpriteId: u8) {
    gSprites[healthboxSpriteId].set_invisible(FALSE as u16);
    gSprites[gSprites[healthboxSpriteId].data[hMain_HealthBarSpriteId]].set_invisible(FALSE as u16);
    gSprites[gSprites[healthboxSpriteId].oam.affineParam].set_invisible(FALSE as u16);
}
unsafe fn UpdateSpritePos(spriteId: u8, x: i16, y: i16) {
    gSprites[spriteId].x = x;
    gSprites[spriteId].y = y;
}
pub unsafe fn DestoryHealthboxSprite(healthboxSpriteId: u8) {
    DestroySprite(&raw mut gSprites[gSprites[healthboxSpriteId].oam.affineParam]);
    DestroySprite(&raw mut gSprites[gSprites[healthboxSpriteId].data[hMain_HealthBarSpriteId]]);
    DestroySprite(&raw mut gSprites[healthboxSpriteId]);
}
#[unsafe(no_mangle)]
pub unsafe fn DummyBattleInterfaceFunc(healthboxSpriteId: u8, isDoubleBattleBattlerOnly: u8) {}
pub unsafe fn UpdateOamPriorityInAllHealthboxes(priority: u8) {
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        let healthboxLeftSpriteId: u8 = gHealthboxSpriteIds[i];
        let healthboxRightSpriteId: u8 = gSprites[gHealthboxSpriteIds[i]].oam.affineParam as u8;
        let healthbarSpriteId: u8 =
            gSprites[gHealthboxSpriteIds[i]].data[hMain_HealthBarSpriteId] as u8;
        gSprites[healthboxLeftSpriteId]
            .oam
            .set_priority(priority as u16);
        gSprites[healthboxRightSpriteId]
            .oam
            .set_priority(priority as u16);
        gSprites[healthbarSpriteId]
            .oam
            .set_priority(priority as u16);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn InitBattlerHealthboxCoords(battler: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    if IsDoubleBattle() == 0 {
        if GetBattlerSide(battler) != B_SIDE_PLAYER {
            x = 44;
            y = 30;
        } else {
            x = 158;
            y = 88;
        }
    } else {
        match GetBattlerPosition(battler) {
            B_POSITION_PLAYER_LEFT => {
                x = 159;
                y = 76;
            }
            B_POSITION_PLAYER_RIGHT => {
                x = 171;
                y = 101;
            }
            B_POSITION_OPPONENT_LEFT => {
                x = 44;
                y = 19;
            }
            B_POSITION_OPPONENT_RIGHT => {
                x = 32;
                y = 44;
            }
            _ => {}
        }
    }
    UpdateSpritePos(gHealthboxSpriteIds[battler], x, y);
}
unsafe fn UpdateLvlInHealthbox(healthboxSpriteId: u8, lvl: u8) {
    let mut windowId: u32 = 0;
    let mut text: CArray<u8, 16> = zeroed();
    text[0] = CHAR_EXTRA_SYMBOL;
    text[1] = CHAR_LV_2;
    let mut objVram: *mut u8 = ConvertIntToDecimalStringN(
        text.as_mut_ptr().at(2),
        lvl as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    let xPos: u32 =
        5 * (3 - (objVram as usize).wrapping_sub(text.as_mut_ptr().at(2) as usize) as i32 as u32);
    let windowTileData: *mut u8 =
        AddTextPrinterAndCreateWindowOnHealthbox(text.as_mut_ptr(), xPos, 3, 2, &raw mut windowId);
    let spriteTileNum: u32 = gSprites[healthboxSpriteId].oam.tileNum() as u32 * 32;
    if GetBattlerSide(gSprites[healthboxSpriteId].data[hMain_Battler] as u8) == B_SIDE_PLAYER {
        objVram = OBJ_VRAM0 as usize as *mut c_void as *mut u8;
        if IsDoubleBattle() == 0 {
            objVram = objVram.at(spriteTileNum + 0x820);
        } else {
            objVram = objVram.at(spriteTileNum + 0x420);
        }
    } else {
        objVram = OBJ_VRAM0 as usize as *mut c_void as *mut u8;
        objVram = objVram.at(spriteTileNum + 0x400);
    }
    TextIntoHealthboxObject(objVram as *mut c_void, windowTileData, 3);
    RemoveWindowOnHealthbox(windowId);
}
pub unsafe fn UpdateHpTextInHealthbox(healthboxSpriteId: u8, value: i16, maxOrCurrent: u8) {
    let mut windowId: u32 = 0;
    let mut spriteTileNum: u32 = 0;
    let mut windowTileData: *mut u8 = null_mut();
    let mut text: CArray<u8, 32> = zeroed();
    let mut objVram: *mut c_void = null_mut();
    if GetBattlerSide(gSprites[healthboxSpriteId].data[6] as u8) == B_SIDE_PLAYER
        && IsDoubleBattle() == 0
    {
        spriteTileNum = gSprites[healthboxSpriteId].oam.tileNum() as u32 * 32;
        if maxOrCurrent != HP_CURRENT {
            ConvertIntToDecimalStringN(
                text.as_mut_ptr(),
                value as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                text.as_mut_ptr(),
                0,
                5,
                2,
                &raw mut windowId,
            );
            objVram = OBJ_VRAM0 as usize as *mut c_void;
            objVram = (objVram as *mut u8).at(spriteTileNum + 0xB40) as *mut c_void;
            HpTextIntoHealthboxObject(objVram, windowTileData, 2);
            RemoveWindowOnHealthbox(windowId);
        } else {
            ConvertIntToDecimalStringN(
                text.as_mut_ptr(),
                value as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            text[3] = CHAR_SLASH;
            text[4] = EOS;
            windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                text.as_mut_ptr(),
                4,
                5,
                2,
                &raw mut windowId,
            );
            objVram = OBJ_VRAM0 as usize as *mut c_void;
            objVram = (objVram as *mut u8).at(spriteTileNum + 0x3E0) as *mut c_void;
            HpTextIntoHealthboxObject(objVram, windowTileData, 1);
            objVram = OBJ_VRAM0 as usize as *mut c_void;
            objVram = (objVram as *mut u8).at(spriteTileNum + 0xB00) as *mut c_void;
            HpTextIntoHealthboxObject(objVram, windowTileData.at(32), 2);
            RemoveWindowOnHealthbox(windowId);
        }
    } else {
        memcpy(
            text.as_mut_ptr(),
            sEmptyWhiteText_GrayHighlight.as_ptr().cast_mut(),
            20,
        );
        let battler: u8 = gSprites[healthboxSpriteId].data[6] as u8;
        if IsDoubleBattle() == 1 || GetBattlerSide(battler) == 1 {
            UpdateHpTextInHealthboxInDoubles(healthboxSpriteId, value, maxOrCurrent);
        } else {
            let mut var: u32 = 0;
            if GetBattlerSide(gSprites[healthboxSpriteId].data[6] as u8) == B_SIDE_PLAYER {
                if maxOrCurrent == HP_CURRENT {
                    var = 29;
                } else {
                    var = 89;
                }
            } else {
                if maxOrCurrent == HP_CURRENT {
                    var = 20;
                } else {
                    var = 48;
                }
            }
            ConvertIntToDecimalStringN(
                text.as_mut_ptr().at(6),
                value as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            RenderTextHandleBold(
                (*gMonSpritesGfxPtr).barFontGfx,
                FONT_BOLD,
                text.as_mut_ptr(),
            );
            for i in 0..3u8 {
                CpuSet(
                    (*gMonSpritesGfxPtr).barFontGfx.at(i as i32 * 64 + 32) as *mut c_void,
                    (0x6010000
                        + 32 * (gSprites[healthboxSpriteId].oam.tileNum() as u32 + var + i as u32))
                        as usize as *mut c_void,
                    0x4000008,
                );
            }
        }
    }
}
unsafe fn UpdateHpTextInHealthboxInDoubles(healthboxSpriteId: u8, value: i16, maxOrCurrent: u8) {
    let mut windowId: u32 = 0;
    let mut spriteTileNum: u32 = 0;
    let mut windowTileData: *mut u8 = null_mut();
    let mut text: CArray<u8, 32> = zeroed();
    let mut objVram: *mut c_void = null_mut();
    if GetBattlerSide(gSprites[healthboxSpriteId].data[6] as u8) == B_SIDE_PLAYER {
        if (*(*gBattleSpritesDataPtr)
            .battlerData
            .at(gSprites[healthboxSpriteId].data[6]))
        .hpNumbersNoBars()
            != 0
        {
            spriteTileNum = gSprites[gSprites[healthboxSpriteId].data[5]].oam.tileNum() as u32 * 32;
            objVram =
                (OBJ_VRAM0 as usize as *mut c_void as *mut u8).at(spriteTileNum) as *mut c_void;
            if maxOrCurrent != HP_CURRENT {
                ConvertIntToDecimalStringN(
                    text.as_mut_ptr(),
                    value as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    3,
                );
                windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                    text.as_mut_ptr(),
                    0,
                    5,
                    0,
                    &raw mut windowId,
                );
                HpTextIntoHealthboxObject(
                    ((OBJ_VRAM0 as usize as *mut c_void as *mut u8).at(spriteTileNum) as *mut c_void
                        as *mut u8)
                        .at(192) as *mut c_void,
                    windowTileData,
                    2,
                );
                RemoveWindowOnHealthbox(windowId);
                CpuSet(
                    GetHealthboxElementGfxPtr(HEALTHBOX_GFX_FRAME_END) as *mut c_void,
                    (100730496_usize as *mut c_void as *mut u8).at(gSprites[healthboxSpriteId]
                        .oam
                        .tileNum()
                        as i32
                        * 32) as *mut c_void,
                    0x4000008,
                );
            } else {
                ConvertIntToDecimalStringN(
                    text.as_mut_ptr(),
                    value as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    3,
                );
                text[3] = CHAR_SLASH;
                text[4] = EOS;
                windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                    text.as_mut_ptr(),
                    4,
                    5,
                    0,
                    &raw mut windowId,
                );
                FillHealthboxObject(objVram, 0, 3);
                HpTextIntoHealthboxObject(
                    (100728928_usize as *mut c_void as *mut u8).at(spriteTileNum) as *mut c_void,
                    windowTileData,
                    3,
                );
                RemoveWindowOnHealthbox(windowId);
            }
        }
    } else {
        memcpy(
            text.as_mut_ptr(),
            sEmptyWhiteText_TransparentHighlight.as_ptr().cast_mut(),
            20,
        );
        let battler: u8 = gSprites[healthboxSpriteId].data[6] as u8;
        if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).hpNumbersNoBars() != 0 {
            let mut var: u8 = 4;
            if maxOrCurrent == HP_CURRENT {
                var = 0;
            }
            let r7: u8 = gSprites[healthboxSpriteId].data[5] as u8;
            let txtPtr: *mut u8 = ConvertIntToDecimalStringN(
                text.as_mut_ptr().at(6),
                value as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            if maxOrCurrent == 0 {
                StringCopy(
                    txtPtr,
                    (*(&raw const crate::data::strings::gText_Slash).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            }
            RenderTextHandleBold(
                (*gMonSpritesGfxPtr).barFontGfx,
                FONT_BOLD,
                text.as_mut_ptr(),
            );
            let mut i: u8 = var;
            while (i as i32) < var as i32 + 3 {
                if i < 3 {
                    CpuSet(
                        (*gMonSpritesGfxPtr)
                            .barFontGfx
                            .at((i as i32 - var as i32) * 64 + 32)
                            as *mut c_void,
                        (0x6010000 + 32 * (1 + gSprites[r7].oam.tileNum() as i32 + i as i32))
                            as usize as *mut c_void,
                        0x4000008,
                    );
                } else {
                    CpuSet(
                        (*gMonSpritesGfxPtr)
                            .barFontGfx
                            .at((i as i32 - var as i32) * 64 + 32)
                            as *mut c_void,
                        (100728864 + 32 * (i as i32 + gSprites[r7].oam.tileNum() as i32)) as usize
                            as *mut c_void,
                        0x4000008,
                    );
                }
                i += 1;
            }
            if maxOrCurrent == HP_CURRENT {
                CpuSet(
                    (*gMonSpritesGfxPtr).barFontGfx.at(224) as *mut c_void,
                    (0x6010000 + (gSprites[r7].oam.tileNum() as i32 + 4) * 32) as usize
                        as *mut c_void,
                    0x4000008,
                );
                {
                    {
                        let mut tmp: u32 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            (OBJ_VRAM0 + gSprites[r7].oam.tileNum() as i32 * 32) as usize
                                as *mut c_void,
                            0x5000008,
                        );
                    }
                }
            } else {
                if GetBattlerSide(battler) == B_SIDE_PLAYER {
                    CpuSet(
                        GetHealthboxElementGfxPtr(HEALTHBOX_GFX_FRAME_END) as *mut c_void,
                        (0x6010000_usize as *mut c_void as *mut u8).at((gSprites[healthboxSpriteId]
                            .oam
                            .tileNum()
                            as i32
                            + 52)
                            * 32) as *mut c_void,
                        0x4000008,
                    );
                }
            }
        }
    }
}
unsafe fn PrintSafariMonInfo(healthboxSpriteId: u8, mon: *mut Pokemon) {
    let mut text: CArray<u8, 20> = zeroed();
    let mut spriteTileNum: i32 = 0;
    let mut barFontGfx: *mut u8 = null_mut();
    memcpy(
        text.as_mut_ptr(),
        sEmptyWhiteText_GrayHighlight.as_ptr().cast_mut(),
        20,
    );
    barFontGfx = (*gMonSpritesGfxPtr).barFontGfx.at(0x520
        + GetBattlerPosition(gSprites[healthboxSpriteId].data[hMain_Battler] as u8) as i32 * 384);
    let var: u8 = 5;
    let nature: u8 = GetNature(mon);
    StringCopy(
        &raw mut text[6],
        (*(&raw const crate::data::pokemon_summary_screen::gNatureNamePointers)
            .cast::<CArray<*mut u8, 0>>())[nature],
    );
    RenderTextHandleBold(barFontGfx, FONT_BOLD, text.as_mut_ptr());
    let mut j: i32 = 6;
    let mut i: u8 = 0;
    while i < var {
        let mut elementId: u8 = 0;
        if text[j] >= 55 && text[j] <= 74 || text[j] >= 135 && text[j] <= 154 {
            elementId = 44;
        } else if text[j] >= 75 && text[j] <= 79 || text[j] >= 155 && text[j] <= 159 {
            elementId = 45;
        } else {
            elementId = 43;
        }
        CpuSet(
            GetHealthboxElementGfxPtr(elementId) as *mut c_void,
            barFontGfx.at(i as i32 * 64) as *mut c_void,
            0x4000008,
        );
        i += 1;
        j += 1;
    }
    for j in 1..(var as i32 + 1) {
        spriteTileNum =
            (gSprites[healthboxSpriteId].oam.tileNum() as i32 + j % 8 + j / 8 * 64) * 32;
        CpuSet(
            barFontGfx as *mut c_void,
            (OBJ_VRAM0 as usize as *mut c_void as *mut u8).at(spriteTileNum) as *mut c_void,
            0x4000008,
        );
        barFontGfx = barFontGfx.at(32);
        spriteTileNum =
            (8 + gSprites[healthboxSpriteId].oam.tileNum() as i32 + j % 8 + j / 8 * 64) * 32;
        CpuSet(
            barFontGfx as *mut c_void,
            (OBJ_VRAM0 as usize as *mut c_void as *mut u8).at(spriteTileNum) as *mut c_void,
            0x4000008,
        );
        barFontGfx = barFontGfx.at(32);
    }
    let healthBarSpriteId: u8 = gSprites[healthboxSpriteId].data[hMain_HealthBarSpriteId] as u8;
    ConvertIntToDecimalStringN(
        &raw mut text[6],
        (*gBattleStruct).safariCatchFactor as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    ConvertIntToDecimalStringN(
        &raw mut text[9],
        (*gBattleStruct).safariEscapeFactor as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    text[5] = CHAR_SPACE;
    text[8] = CHAR_SLASH;
    RenderTextHandleBold(
        (*gMonSpritesGfxPtr).barFontGfx,
        FONT_BOLD,
        text.as_mut_ptr(),
    );
    j = healthBarSpriteId as i32;
    for j in 0..5i32 {
        if j <= 1 {
            CpuSet(
                (*gMonSpritesGfxPtr).barFontGfx.at(0x40 * j + 0x20) as *mut c_void,
                (0x6010000_usize as *mut c_void as *mut u8).at((gSprites[healthBarSpriteId]
                    .oam
                    .tileNum()
                    as i32
                    + 2
                    + j)
                    * 32) as *mut c_void,
                0x4000008,
            );
        } else {
            CpuSet(
                (*gMonSpritesGfxPtr).barFontGfx.at(0x40 * j + 0x20) as *mut c_void,
                (100729024_usize as *mut c_void as *mut u8).at((j + gSprites[healthBarSpriteId]
                    .oam
                    .tileNum()
                    as i32)
                    * 32) as *mut c_void,
                0x4000008,
            );
        }
    }
}
pub unsafe fn SwapHpBarsWithHpText() {
    let mut healthBarSpriteId: u8 = 0;
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        'l1: {
            if gSprites[gHealthboxSpriteIds[i]].callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
                && GetBattlerSide(i as u8) != B_SIDE_OPPONENT
                && (IsDoubleBattle() != 0 || GetBattlerSide(i as u8) != B_SIDE_PLAYER)
            {
                (*(*gBattleSpritesDataPtr).battlerData.at(i)).set_hpNumbersNoBars(
                    (*(*gBattleSpritesDataPtr).battlerData.at(i)).hpNumbersNoBars() ^ 1,
                );
                let noBars: u8 =
                    (*(*gBattleSpritesDataPtr).battlerData.at(i)).hpNumbersNoBars() as u8;
                if GetBattlerSide(i as u8) == B_SIDE_PLAYER {
                    if IsDoubleBattle() == 0 {
                        break 'l1;
                    }
                    if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0 {
                        break 'l1;
                    }
                    if noBars == TRUE {
                        healthBarSpriteId =
                            gSprites[gHealthboxSpriteIds[i]].data[hMain_HealthBarSpriteId] as u8;
                        {
                            {
                                let mut tmp: u32 = 0;
                                volatile_write(&raw mut tmp, 0);
                                CpuSet(
                                    &raw mut tmp as *mut c_void,
                                    (OBJ_VRAM0
                                        + gSprites[healthBarSpriteId].oam.tileNum() as i32 * 32)
                                        as usize as *mut c_void,
                                    0x5000040,
                                );
                            }
                        }
                        UpdateHpTextInHealthboxInDoubles(
                            gHealthboxSpriteIds[i],
                            GetMonData2(&raw mut gPlayerParty[gBattlerPartyIndexes[i]], MON_DATA_HP)
                                as i16,
                            HP_CURRENT,
                        );
                        UpdateHpTextInHealthboxInDoubles(
                            gHealthboxSpriteIds[i],
                            GetMonData2(
                                &raw mut gPlayerParty[gBattlerPartyIndexes[i]],
                                MON_DATA_MAX_HP,
                            ) as i16,
                            HP_MAX,
                        );
                    } else {
                        UpdateStatusIconInHealthbox(gHealthboxSpriteIds[i]);
                        UpdateHealthboxAttribute(
                            gHealthboxSpriteIds[i],
                            &raw mut gPlayerParty[gBattlerPartyIndexes[i]],
                            HEALTHBOX_HEALTH_BAR,
                        );
                        CpuSet(
                            GetHealthboxElementGfxPtr(HEALTHBOX_GFX_FRAME_END_BAR) as *mut c_void,
                            (100730496 + gSprites[gHealthboxSpriteIds[i]].oam.tileNum() as i32 * 32)
                                as usize as *mut c_void,
                            0x4000008,
                        );
                    }
                } else {
                    if noBars == TRUE {
                        if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0 {
                            PrintSafariMonInfo(
                                gHealthboxSpriteIds[i],
                                &raw mut gEnemyParty[gBattlerPartyIndexes[i]],
                            );
                        } else {
                            healthBarSpriteId = gSprites[gHealthboxSpriteIds[i]].data
                                [hMain_HealthBarSpriteId]
                                as u8;
                            {
                                {
                                    let mut tmp: u32 = 0;
                                    volatile_write(&raw mut tmp, 0);
                                    CpuSet(
                                        &raw mut tmp as *mut c_void,
                                        (OBJ_VRAM0
                                            + gSprites[healthBarSpriteId].oam.tileNum() as i32 * 32)
                                            as usize
                                            as *mut c_void,
                                        0x5000040,
                                    );
                                }
                            }
                            UpdateHpTextInHealthboxInDoubles(
                                gHealthboxSpriteIds[i],
                                GetMonData2(
                                    &raw mut gEnemyParty[gBattlerPartyIndexes[i]],
                                    MON_DATA_HP,
                                ) as i16,
                                HP_CURRENT,
                            );
                            UpdateHpTextInHealthboxInDoubles(
                                gHealthboxSpriteIds[i],
                                GetMonData2(
                                    &raw mut gEnemyParty[gBattlerPartyIndexes[i]],
                                    MON_DATA_MAX_HP,
                                ) as i16,
                                HP_MAX,
                            );
                        }
                    } else {
                        UpdateStatusIconInHealthbox(gHealthboxSpriteIds[i]);
                        UpdateHealthboxAttribute(
                            gHealthboxSpriteIds[i],
                            &raw mut gEnemyParty[gBattlerPartyIndexes[i]],
                            HEALTHBOX_HEALTH_BAR,
                        );
                        if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0 {
                            UpdateHealthboxAttribute(
                                gHealthboxSpriteIds[i],
                                &raw mut gEnemyParty[gBattlerPartyIndexes[i]],
                                HEALTHBOX_NICK,
                            );
                        }
                    }
                }
                gSprites[gHealthboxSpriteIds[i]].data[hMain_Data7] ^= 1;
            }
        }
        i += 1;
    }
}
pub unsafe fn CreatePartyStatusSummarySprites(
    battler: u8,
    partyInfo: *mut HpAndStatus,
    skipPlayer: u8,
    isBattleStart: u8,
) -> u8 {
    let mut isOpponent: u8 = 0;
    let mut bar_X: i16 = 0;
    let mut bar_Y: i16 = 0;
    let mut bar_pos2_X: i16 = 0;
    let mut bar_data0: i16 = 0;
    let mut var: i32 = 0;
    let mut ballIconSpritesIds: CArray<u8, 6> = zeroed();
    if skipPlayer == 0 || GetBattlerPosition(battler) != B_POSITION_OPPONENT_RIGHT {
        if GetBattlerSide(battler) == B_SIDE_PLAYER {
            isOpponent = FALSE;
            bar_X = 136;
            bar_Y = 96;
            bar_pos2_X = 100;
            bar_data0 = -5;
        } else {
            isOpponent = TRUE;
            if skipPlayer == 0 || IsDoubleBattle() == 0 {
                bar_X = 104;
                bar_Y = 40;
            } else {
                bar_X = 104;
                bar_Y = 16;
            }
            bar_pos2_X = -100;
            bar_data0 = 5;
        }
    } else {
        isOpponent = TRUE;
        bar_X = 104;
        bar_Y = 40;
        bar_pos2_X = -100;
        bar_data0 = 5;
    }
    LoadCompressedSpriteSheetUsingHeap((&raw const *sStatusSummaryBarSpriteSheet).cast_mut());
    LoadSpriteSheet((&raw const *sStatusSummaryBallsSpriteSheet).cast_mut());
    LoadSpritePalette((&raw const *sStatusSummaryBarSpritePal).cast_mut());
    LoadSpritePalette((&raw const *sStatusSummaryBallsSpritePal).cast_mut());
    let summaryBarSpriteId: u8 = CreateSprite(
        (&raw const sStatusSummaryBarSpriteTemplates[isOpponent]).cast_mut(),
        bar_X,
        bar_Y,
        10,
    );
    SetSubspriteTables(
        &raw mut gSprites[summaryBarSpriteId],
        sStatusSummaryBar_SubspriteTable_Enter.as_ptr().cast_mut(),
    );
    gSprites[summaryBarSpriteId].x2 = bar_pos2_X;
    gSprites[summaryBarSpriteId].data[0] = bar_data0;
    if isOpponent != 0 {
        gSprites[summaryBarSpriteId].x -= 96;
        gSprites[summaryBarSpriteId].oam.set_matrixNum(ST_OAM_HFLIP);
    } else {
        gSprites[summaryBarSpriteId].x += 96;
    }
    let mut i: i32 = 0;
    while i < PARTY_SIZE {
        ballIconSpritesIds[i] = CreateSpriteAtEnd(
            (&raw const sStatusSummaryBallsSpriteTemplates[isOpponent]).cast_mut(),
            bar_X,
            bar_Y - 4,
            9,
        );
        if isBattleStart == 0 {
            gSprites[ballIconSpritesIds[i]].callback =
                Some(SpriteCB_StatusSummaryBalls_OnSwitchout);
        }
        if isOpponent == 0 {
            gSprites[ballIconSpritesIds[i]].x2 = 0;
            gSprites[ballIconSpritesIds[i]].y2 = 0;
        }
        gSprites[ballIconSpritesIds[i]].data[0] = summaryBarSpriteId as i16;
        if isOpponent == 0 {
            gSprites[ballIconSpritesIds[i]].x += 10 * i as i16 + 24;
            gSprites[ballIconSpritesIds[i]].data[1] = i as i16 * 7 + 10;
            gSprites[ballIconSpritesIds[i]].x2 = 120;
        } else {
            gSprites[ballIconSpritesIds[i]].x -= 10 * (5 - i as i16) + 24;
            gSprites[ballIconSpritesIds[i]].data[1] = (6 - i as i16) * 7 + 10;
            gSprites[ballIconSpritesIds[i]].x2 = -120;
        }
        gSprites[ballIconSpritesIds[i]].data[2] = isOpponent as i16;
        i += 1;
    }
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
            for i in 0..PARTY_SIZE {
                if (*partyInfo.at(i)).hp == HP_EMPTY_SLOT {
                    gSprites[ballIconSpritesIds[i]]
                        .oam
                        .set_tileNum(gSprites[ballIconSpritesIds[i]].oam.tileNum() + 1);
                    gSprites[ballIconSpritesIds[i]].data[7] = 1;
                } else if (*partyInfo.at(i)).hp == 0 {
                    gSprites[ballIconSpritesIds[i]]
                        .oam
                        .set_tileNum(gSprites[ballIconSpritesIds[i]].oam.tileNum() + 3);
                } else if (*partyInfo.at(i)).status != 0 {
                    gSprites[ballIconSpritesIds[i]]
                        .oam
                        .set_tileNum(gSprites[ballIconSpritesIds[i]].oam.tileNum() + 2);
                }
            }
        } else {
            i = 0;
            var = 5;
            for j in 0..PARTY_SIZE {
                'l3: {
                    if (*partyInfo.at(j)).hp == HP_EMPTY_SLOT {
                        gSprites[ballIconSpritesIds[var]]
                            .oam
                            .set_tileNum(gSprites[ballIconSpritesIds[var]].oam.tileNum() + 1);
                        gSprites[ballIconSpritesIds[var]].data[7] = 1;
                        var -= 1;
                        break 'l3;
                    } else if (*partyInfo.at(j)).hp == 0 {
                        gSprites[ballIconSpritesIds[i]]
                            .oam
                            .set_tileNum(gSprites[ballIconSpritesIds[i]].oam.tileNum() + 3);
                    } else if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0
                        && (*gBattleStruct).arenaLostPlayerMons as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[j]
                            != 0
                    {
                        gSprites[ballIconSpritesIds[i]]
                            .oam
                            .set_tileNum(gSprites[ballIconSpritesIds[i]].oam.tileNum() + 3);
                    } else if (*partyInfo.at(j)).status != 0 {
                        gSprites[ballIconSpritesIds[i]]
                            .oam
                            .set_tileNum(gSprites[ballIconSpritesIds[i]].oam.tileNum() + 2);
                    }
                    i += 1;
                }
            }
        }
    } else {
        if gBattleTypeFlags & 32832 != 0 {
            var = 5;
            for i in 0..PARTY_SIZE {
                if (*partyInfo.at(i)).hp == HP_EMPTY_SLOT {
                    gSprites[ballIconSpritesIds[var]]
                        .oam
                        .set_tileNum(gSprites[ballIconSpritesIds[var]].oam.tileNum() + 1);
                    gSprites[ballIconSpritesIds[var]].data[7] = 1;
                } else if (*partyInfo.at(i)).hp == 0 {
                    gSprites[ballIconSpritesIds[var]]
                        .oam
                        .set_tileNum(gSprites[ballIconSpritesIds[var]].oam.tileNum() + 3);
                } else if (*partyInfo.at(i)).status != 0 {
                    gSprites[ballIconSpritesIds[var]]
                        .oam
                        .set_tileNum(gSprites[ballIconSpritesIds[var]].oam.tileNum() + 2);
                }
                var -= 1;
            }
        } else {
            var = 0;
            i = 0;
            for j in 0..PARTY_SIZE {
                'l6: {
                    if (*partyInfo.at(j)).hp == HP_EMPTY_SLOT {
                        gSprites[ballIconSpritesIds[i]]
                            .oam
                            .set_tileNum(gSprites[ballIconSpritesIds[i]].oam.tileNum() + 1);
                        gSprites[ballIconSpritesIds[i]].data[7] = 1;
                        i += 1;
                        break 'l6;
                    } else if (*partyInfo.at(j)).hp == 0 {
                        gSprites[ballIconSpritesIds[5 - var]]
                            .oam
                            .set_tileNum(gSprites[ballIconSpritesIds[5 - var]].oam.tileNum() + 3);
                    } else if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0
                        && (*gBattleStruct).arenaLostOpponentMons as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[j]
                            != 0
                    {
                        gSprites[ballIconSpritesIds[5 - var]]
                            .oam
                            .set_tileNum(gSprites[ballIconSpritesIds[5 - var]].oam.tileNum() + 3);
                    } else if (*partyInfo.at(j)).status != 0 {
                        gSprites[ballIconSpritesIds[5 - var]]
                            .oam
                            .set_tileNum(gSprites[ballIconSpritesIds[5 - var]].oam.tileNum() + 2);
                    }
                    var += 1;
                }
            }
        }
    }
    let taskId: u8 = CreateTask(Some(TaskDummy), 5);
    task_set(taskId, 0, battler as i16);
    task_set(taskId, 1, summaryBarSpriteId as i16);
    for i in 0..PARTY_SIZE {
        task_set(taskId, 3 + i, ballIconSpritesIds[i] as i16);
    }
    task_set(taskId, tIsBattleStart, isBattleStart as i16);
    if isBattleStart != 0 {
        (*(*gBattleSpritesDataPtr).animationData)
            .set_field_9_x1C((*(*gBattleSpritesDataPtr).animationData).field_9_x1C() + 1);
    }
    PlaySE12WithPanning(SE_BALL_TRAY_ENTER, 0);
    taskId
}
pub unsafe fn Task_HidePartyStatusSummary(taskId: u8) {
    let mut ballIconSpriteIds: CArray<u8, 6> = zeroed();
    let isBattleStart: u8 = task_get(taskId, tIsBattleStart) as u8;
    let summaryBarSpriteId: u8 = task_get(taskId, 1) as u8;
    let battler: u8 = task_get(taskId, 0) as u8;
    for i in 0..PARTY_SIZE {
        ballIconSpriteIds[i] = task_get(taskId, 3 + i) as u8;
    }
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 16);
    task_set(taskId, tBlend, 16);
    let mut i: i32 = 0;
    while i < PARTY_SIZE {
        gSprites[ballIconSpriteIds[i]]
            .oam
            .set_objMode(ST_OAM_OBJ_BLEND);
        i += 1;
    }
    gSprites[summaryBarSpriteId]
        .oam
        .set_objMode(ST_OAM_OBJ_BLEND);
    if isBattleStart != 0 {
        for i in 0..PARTY_SIZE {
            if GetBattlerSide(battler) != B_SIDE_PLAYER {
                gSprites[ballIconSpriteIds[5 - i]].data[1] = 7 * i as i16;
                gSprites[ballIconSpriteIds[5 - i]].data[3] = 0;
                gSprites[ballIconSpriteIds[5 - i]].data[4] = 0;
                gSprites[ballIconSpriteIds[5 - i]].callback =
                    Some(SpriteCB_StatusSummaryBalls_Exit);
            } else {
                gSprites[ballIconSpriteIds[i]].data[1] = 7 * i as i16;
                gSprites[ballIconSpriteIds[i]].data[3] = 0;
                gSprites[ballIconSpriteIds[i]].data[4] = 0;
                gSprites[ballIconSpriteIds[i]].callback = Some(SpriteCB_StatusSummaryBalls_Exit);
            }
        }
        gSprites[summaryBarSpriteId].data[0] /= 2;
        gSprites[summaryBarSpriteId].data[1] = 0;
        gSprites[summaryBarSpriteId].callback = Some(SpriteCB_StatusSummaryBar_Exit);
        SetSubspriteTables(
            &raw mut gSprites[summaryBarSpriteId],
            sStatusSummaryBar_SubspriteTable_Exit.as_ptr().cast_mut(),
        );
        task_set_func(taskId, Some(Task_HidePartyStatusSummary_BattleStart_1));
    } else {
        task_set_func(taskId, Some(Task_HidePartyStatusSummary_DuringBattle));
    }
}
pub(crate) unsafe fn Task_HidePartyStatusSummary_BattleStart_1(taskId: u8) {
    if ({
        let t1 = task_get(taskId, 11);
        task_set(taskId, 11, task_get(taskId, 11) + 1);
        t1
    }) % 2
        == 0
    {
        if ({
            task_set(taskId, tBlend, task_get(taskId, tBlend) - 1);
            task_get(taskId, tBlend)
        }) < 0
        {
            return;
        }
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (16 - task_get(taskId, tBlend) as u16) << 8 | task_get(taskId, tBlend) as u16,
        );
    }
    if task_get(taskId, tBlend) == 0 {
        task_set_func(taskId, Some(Task_HidePartyStatusSummary_BattleStart_2));
    }
}
pub(crate) unsafe fn Task_HidePartyStatusSummary_BattleStart_2(taskId: u8) {
    let mut ballIconSpriteIds: CArray<u8, 6> = zeroed();
    let mut i: i32 = 0;
    let battler: u8 = task_get(taskId, tBattler) as u8;
    if ({
        task_set(taskId, tBlend, task_get(taskId, tBlend) - 1);
        task_get(taskId, tBlend)
    }) == -1
    {
        let summaryBarSpriteId: u8 = task_get(taskId, tSummaryBarSpriteId) as u8;
        i = 0;
        while i < PARTY_SIZE {
            ballIconSpriteIds[i] = task_get(taskId, 3 + i) as u8;
            i += 1;
        }
        (*(*gBattleSpritesDataPtr).animationData)
            .set_field_9_x1C((*(*gBattleSpritesDataPtr).animationData).field_9_x1C() - 1);
        if (*(*gBattleSpritesDataPtr).animationData).field_9_x1C() == 0 {
            DestroySpriteAndFreeResources(&raw mut gSprites[summaryBarSpriteId]);
            DestroySpriteAndFreeResources(&raw mut gSprites[ballIconSpriteIds[0]]);
        } else {
            FreeSpriteOamMatrix(&raw mut gSprites[summaryBarSpriteId]);
            DestroySprite(&raw mut gSprites[summaryBarSpriteId]);
            FreeSpriteOamMatrix(&raw mut gSprites[ballIconSpriteIds[0]]);
            DestroySprite(&raw mut gSprites[ballIconSpriteIds[0]]);
        }
        for i in 1..PARTY_SIZE {
            DestroySprite(&raw mut gSprites[ballIconSpriteIds[i]]);
        }
    } else if task_get(taskId, tBlend) == -3 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_partyStatusSummaryShown(0);
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_HidePartyStatusSummary_DuringBattle(taskId: u8) {
    let mut ballIconSpriteIds: CArray<u8, 6> = zeroed();
    let mut i: i32 = 0;
    let battler: u8 = task_get(taskId, tBattler) as u8;
    if ({
        task_set(taskId, tBlend, task_get(taskId, tBlend) - 1);
        task_get(taskId, tBlend)
    }) >= 0
    {
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (16 - task_get(taskId, tBlend) as u16) << 8 | task_get(taskId, tBlend) as u16,
        );
    } else if task_get(taskId, tBlend) == -1 {
        let summaryBarSpriteId: u8 = task_get(taskId, tSummaryBarSpriteId) as u8;
        i = 0;
        while i < PARTY_SIZE {
            ballIconSpriteIds[i] = task_get(taskId, 3 + i) as u8;
            i += 1;
        }
        DestroySpriteAndFreeResources(&raw mut gSprites[summaryBarSpriteId]);
        DestroySpriteAndFreeResources(&raw mut gSprites[ballIconSpriteIds[0]]);
        for i in 1..PARTY_SIZE {
            DestroySprite(&raw mut gSprites[ballIconSpriteIds[i]]);
        }
    } else if task_get(taskId, tBlend) == -3 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_partyStatusSummaryShown(0);
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn SpriteCB_StatusSummaryBar_Enter(sprite: *mut Sprite) {
    if (*sprite).x2 != 0 {
        (*sprite).x2 += (*sprite).data[0];
    }
}
pub(crate) unsafe fn SpriteCB_StatusSummaryBar_Exit(sprite: *mut Sprite) {
    (*sprite).data[1] += 32;
    if (*sprite).data[0] > 0 {
        (*sprite).x2 += (*sprite).data[1] >> 4;
    } else {
        (*sprite).x2 -= (*sprite).data[1] >> 4;
    }
    (*sprite).data[1] &= 0xF;
}
pub(crate) unsafe fn SpriteCB_StatusSummaryBalls_Enter(sprite: *mut Sprite) {
    let mut pan: i8 = 0;
    if (*sprite).data[1] > 0 {
        (*sprite).data[1] -= 1;
        return;
    }
    let var1: u8 = (*sprite).data[2] as u8;
    let mut var2: u16 = (*sprite).data[3] as u16;
    var2 += 56;
    (*sprite).data[3] = var2 as i16 & -16;
    if var1 != 0 {
        (*sprite).x2 += (var2 >> 4) as i16;
        if (*sprite).x2 > 0 {
            (*sprite).x2 = 0;
        }
    } else {
        (*sprite).x2 -= (var2 >> 4) as i16;
        if (*sprite).x2 < 0 {
            (*sprite).x2 = 0;
        }
    }
    if (*sprite).x2 == 0 {
        pan = SOUND_PAN_TARGET;
        if var1 != 0 {
            pan = SOUND_PAN_ATTACKER;
        }
        if (*sprite).data[7] != 0 {
            PlaySE2WithPanning(SE_BALL_TRAY_EXIT, pan);
        } else {
            PlaySE1WithPanning(SE_BALL_TRAY_BALL, pan);
        }
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe fn SpriteCB_StatusSummaryBalls_Exit(sprite: *mut Sprite) {
    if (*sprite).data[1] > 0 {
        (*sprite).data[1] -= 1;
        return;
    }
    let var1: u8 = (*sprite).data[2] as u8;
    let mut var2: u16 = (*sprite).data[3] as u16;
    var2 += 56;
    (*sprite).data[3] = var2 as i16 & -16;
    if var1 != 0 {
        (*sprite).x2 += (var2 >> 4) as i16;
    } else {
        (*sprite).x2 -= (var2 >> 4) as i16;
    }
    if (*sprite).x2 as i32 + (*sprite).x as i32 > 248
        || ((*sprite).x2 as i32 + (*sprite).x as i32) < -8
    {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe fn SpriteCB_StatusSummaryBalls_OnSwitchout(sprite: *mut Sprite) {
    let barSpriteId: u8 = (*sprite).data[0] as u8;
    (*sprite).x2 = gSprites[barSpriteId].x2;
    (*sprite).y2 = gSprites[barSpriteId].y2;
}
unsafe fn UpdateNickInHealthbox(healthboxSpriteId: u8, mon: *mut Pokemon) {
    let mut nickname: CArray<u8, 11> = zeroed();
    let mut windowId: u32 = 0;
    let mut windowTileData: *mut u8 = null_mut();
    StringCopy(
        gDisplayedStringBattle.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_HealthboxNickname).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    GetMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
    StringGet_Nickname(nickname.as_mut_ptr());
    let mut ptr: *mut c_void =
        StringAppend(gDisplayedStringBattle.as_mut_ptr(), nickname.as_mut_ptr()) as *mut c_void;
    let mut gender: u8 = GetMonGender(mon);
    let species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    if (species == SPECIES_NIDORAN_F || species == SPECIES_NIDORAN_M)
        && StringCompare(
            nickname.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[species]
                .as_ptr()
                .cast_mut(),
        ) == 0
    {
        gender = 100;
    }
    match gender {
        MON_MALE => {
            StringCopy(
                ptr as *mut u8,
                (*(&raw const crate::data::strings::gText_HealthboxGender_Male)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                gDisplayedStringBattle.as_mut_ptr(),
                0,
                3,
                2,
                &raw mut windowId,
            );
        }
        MON_FEMALE => {
            StringCopy(
                ptr as *mut u8,
                (*(&raw const crate::data::strings::gText_HealthboxGender_Female)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                gDisplayedStringBattle.as_mut_ptr(),
                0,
                3,
                2,
                &raw mut windowId,
            );
        }
        _ => {
            StringCopy(
                ptr as *mut u8,
                (*(&raw const crate::data::strings::gText_HealthboxGender_None)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                gDisplayedStringBattle.as_mut_ptr(),
                0,
                3,
                2,
                &raw mut windowId,
            );
        }
    }
    let spriteTileNum: u32 = gSprites[healthboxSpriteId].oam.tileNum() as u32 * 32;
    if GetBattlerSide(gSprites[healthboxSpriteId].data[6] as u8) == B_SIDE_PLAYER {
        TextIntoHealthboxObject(
            (0x6010040 + spriteTileNum) as usize as *mut c_void,
            windowTileData,
            6,
        );
        ptr = OBJ_VRAM0 as usize as *mut c_void;
        if IsDoubleBattle() == 0 {
            ptr = (ptr as *mut u8).at(spriteTileNum + 0x800) as *mut c_void;
        } else {
            ptr = (ptr as *mut u8).at(spriteTileNum + 0x400) as *mut c_void;
        }
        TextIntoHealthboxObject(ptr, windowTileData.at(192), 1);
    } else {
        TextIntoHealthboxObject(
            (0x6010020 + spriteTileNum) as usize as *mut c_void,
            windowTileData,
            7,
        );
    }
    RemoveWindowOnHealthbox(windowId);
}
unsafe fn TryAddPokeballIconToHealthbox(healthboxSpriteId: u8, noStatus: u8) {
    if gBattleTypeFlags & BATTLE_TYPE_WALLY_TUTORIAL != 0 {
        return;
    }
    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
        return;
    }
    let battler: u8 = gSprites[healthboxSpriteId].data[hMain_Battler] as u8;
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        return;
    }
    if GetSetPokedexFlag(
        SpeciesToNationalPokedexNum(GetMonData2(
            &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
            MON_DATA_SPECIES,
        ) as u16),
        FLAG_GET_CAUGHT,
    ) == 0
    {
        return;
    }
    let healthBarSpriteId: u8 = gSprites[healthboxSpriteId].data[hMain_HealthBarSpriteId] as u8;
    if noStatus != 0 {
        CpuSet(
            GetHealthboxElementGfxPtr(HEALTHBOX_GFX_STATUS_BALL_CAUGHT) as *mut c_void,
            (OBJ_VRAM0 + (gSprites[healthBarSpriteId].oam.tileNum() as i32 + 8) * 32) as usize
                as *mut c_void,
            0x4000008,
        );
    } else {
        {
            {
                let mut tmp: u32 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    (OBJ_VRAM0 + (gSprites[healthBarSpriteId].oam.tileNum() as i32 + 8) * 32)
                        as usize as *mut c_void,
                    0x5000008,
                );
            }
        }
    }
}
unsafe fn UpdateStatusIconInHealthbox(healthboxSpriteId: u8) {
    let mut status: u32 = 0;
    let mut statusGfxPtr: *mut u8 = null_mut();
    let mut tileNumAdder: i16 = 0;
    let mut statusPalId: u8 = 0;
    let battler: u8 = gSprites[healthboxSpriteId].data[hMain_Battler] as u8;
    let healthBarSpriteId: u8 = gSprites[healthboxSpriteId].data[hMain_HealthBarSpriteId] as u8;
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        status = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
            MON_DATA_STATUS,
        );
        if IsDoubleBattle() == 0 {
            tileNumAdder = 0x1A;
        } else {
            tileNumAdder = 0x12;
        }
    } else {
        status = GetMonData2(
            &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
            MON_DATA_STATUS,
        );
        tileNumAdder = 0x11;
    }
    if status & STATUS1_SLEEP != 0 {
        statusGfxPtr = GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(
            HEALTHBOX_GFX_STATUS_SLP_BATTLER0,
            battler,
        ));
        statusPalId = PAL_STATUS_SLP;
    } else if status & STATUS1_PSN_ANY != 0 {
        statusGfxPtr = GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(
            HEALTHBOX_GFX_STATUS_PSN_BATTLER0,
            battler,
        ));
        statusPalId = PAL_STATUS_PSN;
    } else if status & STATUS1_BURN != 0 {
        statusGfxPtr = GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(
            HEALTHBOX_GFX_STATUS_BRN_BATTLER0,
            battler,
        ));
        statusPalId = PAL_STATUS_BRN;
    } else if status & STATUS1_FREEZE != 0 {
        statusGfxPtr = GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(
            HEALTHBOX_GFX_STATUS_FRZ_BATTLER0,
            battler,
        ));
        statusPalId = PAL_STATUS_FRZ;
    } else if status & STATUS1_PARALYSIS != 0 {
        statusGfxPtr = GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(
            HEALTHBOX_GFX_STATUS_PRZ_BATTLER0,
            battler,
        ));
        statusPalId = PAL_STATUS_PAR;
    } else {
        statusGfxPtr = GetHealthboxElementGfxPtr(HEALTHBOX_GFX_39);
        for i in 0..3i32 {
            CpuSet(
                statusGfxPtr as *mut c_void,
                (OBJ_VRAM0
                    + (gSprites[healthboxSpriteId].oam.tileNum() as i32 + tileNumAdder as i32 + i)
                        * 32) as usize as *mut c_void,
                0x4000008,
            );
        }
        if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).hpNumbersNoBars() == 0 {
            CpuSet(
                GetHealthboxElementGfxPtr(HEALTHBOX_GFX_1) as *mut c_void,
                (OBJ_VRAM0 + gSprites[healthBarSpriteId].oam.tileNum() as i32 * 32) as usize
                    as *mut c_void,
                0x4000010,
            );
        }
        TryAddPokeballIconToHealthbox(healthboxSpriteId, TRUE);
        return;
    }
    let mut pltAdder: u32 = gSprites[healthboxSpriteId].oam.paletteNum() as u32 * 16;
    pltAdder += battler as u32 + 12;
    FillPalette(
        sStatusIconColors[statusPalId],
        OBJ_PLTT_OFFSET + pltAdder as u16,
        2,
    );
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[OBJ_PLTT_OFFSET as u32 + pltAdder] as *mut c_void,
        (OBJ_PLTT as usize as *mut u16).at(pltAdder) as *mut c_void,
        1,
    );
    CpuSet(
        statusGfxPtr as *mut c_void,
        (OBJ_VRAM0 + (gSprites[healthboxSpriteId].oam.tileNum() as i32 + tileNumAdder as i32) * 32)
            as usize as *mut c_void,
        0x4000018,
    );
    if (IsDoubleBattle() == 1 || GetBattlerSide(battler) == 1)
        && (*(*gBattleSpritesDataPtr).battlerData.at(battler)).hpNumbersNoBars() == 0
    {
        CpuSet(
            GetHealthboxElementGfxPtr(HEALTHBOX_GFX_0) as *mut c_void,
            (OBJ_VRAM0 + gSprites[healthBarSpriteId].oam.tileNum() as i32 * 32) as usize
                as *mut c_void,
            0x4000008,
        );
        CpuSet(
            GetHealthboxElementGfxPtr(HEALTHBOX_GFX_65) as *mut c_void,
            (OBJ_VRAM0 + (gSprites[healthBarSpriteId].oam.tileNum() as i32 + 1) * 32) as usize
                as *mut c_void,
            0x4000008,
        );
    }
    TryAddPokeballIconToHealthbox(healthboxSpriteId, FALSE);
}
fn GetStatusIconForBattlerId(statusElementId: u8, battler: u8) -> u8 {
    let mut ret: u8 = statusElementId;
    match statusElementId {
        HEALTHBOX_GFX_STATUS_PSN_BATTLER0 => {
            if battler == 0 {
                ret = HEALTHBOX_GFX_STATUS_PSN_BATTLER0;
            } else if battler == 1 {
                ret = HEALTHBOX_GFX_STATUS_PSN_BATTLER1;
            } else if battler == 2 {
                ret = HEALTHBOX_GFX_STATUS_PSN_BATTLER2;
            } else {
                ret = HEALTHBOX_GFX_STATUS_PSN_BATTLER3;
            }
        }
        HEALTHBOX_GFX_STATUS_PRZ_BATTLER0 => {
            if battler == 0 {
                ret = HEALTHBOX_GFX_STATUS_PRZ_BATTLER0;
            } else if battler == 1 {
                ret = HEALTHBOX_GFX_STATUS_PRZ_BATTLER1;
            } else if battler == 2 {
                ret = HEALTHBOX_GFX_STATUS_PRZ_BATTLER2;
            } else {
                ret = HEALTHBOX_GFX_STATUS_PRZ_BATTLER3;
            }
        }
        HEALTHBOX_GFX_STATUS_SLP_BATTLER0 => {
            if battler == 0 {
                ret = HEALTHBOX_GFX_STATUS_SLP_BATTLER0;
            } else if battler == 1 {
                ret = HEALTHBOX_GFX_STATUS_SLP_BATTLER1;
            } else if battler == 2 {
                ret = HEALTHBOX_GFX_STATUS_SLP_BATTLER2;
            } else {
                ret = HEALTHBOX_GFX_STATUS_SLP_BATTLER3;
            }
        }
        HEALTHBOX_GFX_STATUS_FRZ_BATTLER0 => {
            if battler == 0 {
                ret = HEALTHBOX_GFX_STATUS_FRZ_BATTLER0;
            } else if battler == 1 {
                ret = HEALTHBOX_GFX_STATUS_FRZ_BATTLER1;
            } else if battler == 2 {
                ret = HEALTHBOX_GFX_STATUS_FRZ_BATTLER2;
            } else {
                ret = HEALTHBOX_GFX_STATUS_FRZ_BATTLER3;
            }
        }
        HEALTHBOX_GFX_STATUS_BRN_BATTLER0 => {
            if battler == 0 {
                ret = HEALTHBOX_GFX_STATUS_BRN_BATTLER0;
            } else if battler == 1 {
                ret = HEALTHBOX_GFX_STATUS_BRN_BATTLER1;
            } else if battler == 2 {
                ret = HEALTHBOX_GFX_STATUS_BRN_BATTLER2;
            } else {
                ret = HEALTHBOX_GFX_STATUS_BRN_BATTLER3;
            }
        }
        _ => {}
    }
    ret
}
unsafe fn UpdateSafariBallsTextOnHealthbox(healthboxSpriteId: u8) {
    let mut windowId: u32 = 0;
    let windowTileData: *mut u8 = AddTextPrinterAndCreateWindowOnHealthbox(
        (*(&raw const crate::data::battle_message::gText_SafariBalls).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        3,
        2,
        &raw mut windowId,
    );
    let spriteTileNum: u32 = gSprites[healthboxSpriteId].oam.tileNum() as u32 * 32;
    TextIntoHealthboxObject(
        (100728896_usize as *mut c_void as *mut u8).at(spriteTileNum) as *mut c_void,
        windowTileData,
        6,
    );
    TextIntoHealthboxObject(
        (0x6010800_usize as *mut c_void as *mut u8).at(spriteTileNum) as *mut c_void,
        windowTileData.at(192),
        2,
    );
    RemoveWindowOnHealthbox(windowId);
}
unsafe fn UpdateLeftNoOfBallsTextOnHealthbox(healthboxSpriteId: u8) {
    let mut text: CArray<u8, 16> = zeroed();
    let mut windowId: u32 = 0;
    let txtPtr: *mut u8 = StringCopy(
        text.as_mut_ptr(),
        (*(&raw const crate::data::battle_message::gText_SafariBallLeft).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    ConvertIntToDecimalStringN(txtPtr, gNumSafariBalls as i32, STR_CONV_MODE_LEFT_ALIGN, 2);
    let windowTileData: *mut u8 = AddTextPrinterAndCreateWindowOnHealthbox(
        text.as_mut_ptr(),
        GetStringRightAlignXOffset(FONT_SMALL as i32, text.as_mut_ptr(), 0x2F) as u32,
        3,
        2,
        &raw mut windowId,
    );
    let spriteTileNum: u32 = gSprites[healthboxSpriteId].oam.tileNum() as u32 * 32;
    SafariTextIntoHealthboxObject(
        (100729536_usize as *mut c_void as *mut u8).at(spriteTileNum) as *mut c_void,
        windowTileData,
        2,
    );
    SafariTextIntoHealthboxObject(
        (0x6010a00_usize as *mut c_void as *mut u8).at(spriteTileNum) as *mut c_void,
        windowTileData.at(64),
        4,
    );
    RemoveWindowOnHealthbox(windowId);
}
#[unsafe(no_mangle)]
pub unsafe fn UpdateHealthboxAttribute(healthboxSpriteId: u8, mon: *mut Pokemon, elementId: u8) {
    let mut maxHp: i32 = 0;
    let mut currHp: i32 = 0;
    let battler: u8 = gSprites[healthboxSpriteId].data[hMain_Battler] as u8;
    if elementId == HEALTHBOX_ALL && IsDoubleBattle() == 0 {
        GetBattlerSide(battler);
    }
    if GetBattlerSide(gSprites[healthboxSpriteId].data[hMain_Battler] as u8) == B_SIDE_PLAYER {
        if elementId == HEALTHBOX_LEVEL || elementId == HEALTHBOX_ALL {
            UpdateLvlInHealthbox(healthboxSpriteId, GetMonData2(mon, MON_DATA_LEVEL) as u8);
        }
        if elementId == HEALTHBOX_CURRENT_HP || elementId == HEALTHBOX_ALL {
            UpdateHpTextInHealthbox(
                healthboxSpriteId,
                GetMonData2(mon, MON_DATA_HP) as i16,
                HP_CURRENT,
            );
        }
        if elementId == HEALTHBOX_MAX_HP || elementId == HEALTHBOX_ALL {
            UpdateHpTextInHealthbox(
                healthboxSpriteId,
                GetMonData2(mon, MON_DATA_MAX_HP) as i16,
                HP_MAX,
            );
        }
        if elementId == HEALTHBOX_HEALTH_BAR || elementId == HEALTHBOX_ALL {
            LoadBattleBarGfx(0);
            maxHp = GetMonData2(mon, MON_DATA_MAX_HP) as i32;
            currHp = GetMonData2(mon, MON_DATA_HP) as i32;
            SetBattleBarStruct(battler, healthboxSpriteId, maxHp, currHp, 0);
            MoveBattleBar(battler, healthboxSpriteId, HEALTH_BAR, 0);
        }
        let isDoubles: u8 = IsDoubleBattle();
        if isDoubles == 0 && (elementId == HEALTHBOX_EXP_BAR || elementId == HEALTHBOX_ALL) {
            LoadBattleBarGfx(3);
            let species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
            let level: u8 = GetMonData2(mon, MON_DATA_LEVEL) as u8;
            let exp: u32 = GetMonData2(mon, MON_DATA_EXP);
            let currLevelExp: u32 = (*(&raw const crate::data::pokemon::gExperienceTables)
                .cast::<CArray<CArray<u32, 101>, 0>>())
                [(*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .growthRate][level];
            let currExpBarValue: i32 = exp as i32 - currLevelExp as i32;
            let maxExpBarValue: i32 = (*(&raw const crate::data::pokemon::gExperienceTables)
                .cast::<CArray<CArray<u32, 101>, 0>>())
                [(*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .growthRate][level as i32 + 1] as i32
                - currLevelExp as i32;
            SetBattleBarStruct(
                battler,
                healthboxSpriteId,
                maxExpBarValue,
                currExpBarValue,
                isDoubles as i32,
            );
            MoveBattleBar(battler, healthboxSpriteId, EXP_BAR, 0);
        }
        if elementId == HEALTHBOX_NICK || elementId == HEALTHBOX_ALL {
            UpdateNickInHealthbox(healthboxSpriteId, mon);
        }
        if elementId == HEALTHBOX_STATUS_ICON || elementId == HEALTHBOX_ALL {
            UpdateStatusIconInHealthbox(healthboxSpriteId);
        }
        if elementId == HEALTHBOX_SAFARI_ALL_TEXT {
            UpdateSafariBallsTextOnHealthbox(healthboxSpriteId);
        }
        if elementId == HEALTHBOX_SAFARI_ALL_TEXT || elementId == HEALTHBOX_SAFARI_BALLS_TEXT {
            UpdateLeftNoOfBallsTextOnHealthbox(healthboxSpriteId);
        }
    } else {
        if elementId == HEALTHBOX_LEVEL || elementId == HEALTHBOX_ALL {
            UpdateLvlInHealthbox(healthboxSpriteId, GetMonData2(mon, MON_DATA_LEVEL) as u8);
        }
        if elementId == HEALTHBOX_HEALTH_BAR || elementId == HEALTHBOX_ALL {
            LoadBattleBarGfx(0);
            maxHp = GetMonData2(mon, MON_DATA_MAX_HP) as i32;
            currHp = GetMonData2(mon, MON_DATA_HP) as i32;
            SetBattleBarStruct(battler, healthboxSpriteId, maxHp, currHp, 0);
            MoveBattleBar(battler, healthboxSpriteId, HEALTH_BAR, 0);
        }
        if elementId == HEALTHBOX_NICK || elementId == HEALTHBOX_ALL {
            UpdateNickInHealthbox(healthboxSpriteId, mon);
        }
        if elementId == HEALTHBOX_STATUS_ICON || elementId == HEALTHBOX_ALL {
            UpdateStatusIconInHealthbox(healthboxSpriteId);
        }
    }
}
pub unsafe fn MoveBattleBar(battler: u8, healthboxSpriteId: u8, whichBar: u8, unused: u8) -> i32 {
    let mut currentBarValue: i32 = 0;
    if whichBar == HEALTH_BAR {
        currentBarValue = CalcNewBarValue(
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).maxValue,
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).oldValue,
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).receivedValue,
            &raw mut (*(*gBattleSpritesDataPtr).battleBars.at(battler)).currValue,
            6,
            1,
        );
    } else {
        let mut expFraction: u16 = GetScaledExpFraction(
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).oldValue,
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).receivedValue,
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).maxValue,
            8,
        ) as u16;
        if expFraction == 0 {
            expFraction = 1;
        }
        expFraction = (if div_i32(
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).receivedValue,
            expFraction as i32,
        ) < 0
        {
            -div_i32(
                (*(*gBattleSpritesDataPtr).battleBars.at(battler)).receivedValue,
                expFraction as i32,
            )
        } else {
            div_i32(
                (*(*gBattleSpritesDataPtr).battleBars.at(battler)).receivedValue,
                expFraction as i32,
            )
        }) as u16;
        currentBarValue = CalcNewBarValue(
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).maxValue,
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).oldValue,
            (*(*gBattleSpritesDataPtr).battleBars.at(battler)).receivedValue,
            &raw mut (*(*gBattleSpritesDataPtr).battleBars.at(battler)).currValue,
            8,
            expFraction,
        );
    }
    if whichBar == EXP_BAR
        || whichBar == HEALTH_BAR
            && (*(*gBattleSpritesDataPtr).battlerData.at(battler)).hpNumbersNoBars() == 0
    {
        MoveBattleBarGraphically(battler, whichBar);
    }
    if currentBarValue == -1 {
        (*(*gBattleSpritesDataPtr).battleBars.at(battler)).currValue = 0;
    }
    currentBarValue
}
unsafe fn MoveBattleBarGraphically(battler: u8, whichBar: u8) {
    let mut array: CArray<u8, 8> = zeroed();
    let mut filledPixelsCount: u8 = 0;
    let mut level: u8 = 0;
    let mut barElementId: u8 = 0;
    match whichBar {
        HEALTH_BAR => {
            filledPixelsCount = CalcBarFilledPixels(
                (*(*gBattleSpritesDataPtr).battleBars.at(battler)).maxValue,
                (*(*gBattleSpritesDataPtr).battleBars.at(battler)).oldValue,
                (*(*gBattleSpritesDataPtr).battleBars.at(battler)).receivedValue,
                &raw mut (*(*gBattleSpritesDataPtr).battleBars.at(battler)).currValue,
                array.as_mut_ptr(),
                6,
            );
            if filledPixelsCount > 24 {
                barElementId = HEALTHBOX_GFX_HP_BAR_GREEN;
            } else if filledPixelsCount > 9 {
                barElementId = HEALTHBOX_GFX_HP_BAR_YELLOW;
            } else {
                barElementId = HEALTHBOX_GFX_HP_BAR_RED;
            }
            for i in 0..6u8 {
                let healthbarSpriteId: u8 =
                    gSprites[(*(*gBattleSpritesDataPtr).battleBars.at(battler)).healthboxSpriteId]
                        .data[hMain_HealthBarSpriteId] as u8;
                if i < 2 {
                    CpuSet(
                        GetHealthboxElementGfxPtr(barElementId).at(array[i] as i32 * 32)
                            as *mut c_void,
                        (0x6010000
                            + (gSprites[healthbarSpriteId].oam.tileNum() as i32 + 2 + i as i32)
                                * 32) as usize as *mut c_void,
                        0x4000008,
                    );
                } else {
                    CpuSet(
                        GetHealthboxElementGfxPtr(barElementId).at(array[i] as i32 * 32)
                            as *mut c_void,
                        (100728896
                            + (i as i32 + gSprites[healthbarSpriteId].oam.tileNum() as i32) * 32)
                            as usize as *mut c_void,
                        0x4000008,
                    );
                }
            }
        }
        EXP_BAR => {
            CalcBarFilledPixels(
                (*(*gBattleSpritesDataPtr).battleBars.at(battler)).maxValue,
                (*(*gBattleSpritesDataPtr).battleBars.at(battler)).oldValue,
                (*(*gBattleSpritesDataPtr).battleBars.at(battler)).receivedValue,
                &raw mut (*(*gBattleSpritesDataPtr).battleBars.at(battler)).currValue,
                array.as_mut_ptr(),
                8,
            );
            level = GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                MON_DATA_LEVEL,
            ) as u8;
            if level == MAX_LEVEL as u8 {
                for i in 0..8u8 {
                    array[i] = 0;
                }
            }
            for i in 0..8u8 {
                if i < 4 {
                    CpuSet(
                        GetHealthboxElementGfxPtr(HEALTHBOX_GFX_12).at(array[i] as i32 * 32)
                            as *mut c_void,
                        (0x6010000
                            + (gSprites[(*(*gBattleSpritesDataPtr).battleBars.at(battler))
                                .healthboxSpriteId]
                                .oam
                                .tileNum() as i32
                                + 0x24
                                + i as i32)
                                * 32) as usize as *mut c_void,
                        0x4000008,
                    );
                } else {
                    CpuSet(
                        GetHealthboxElementGfxPtr(HEALTHBOX_GFX_12).at(array[i] as i32 * 32)
                            as *mut c_void,
                        (100731776
                            + (i as i32
                                + gSprites[(*(*gBattleSpritesDataPtr).battleBars.at(battler))
                                    .healthboxSpriteId]
                                    .oam
                                    .tileNum() as i32)
                                * 32) as usize as *mut c_void,
                        0x4000008,
                    );
                }
            }
        }
        _ => {}
    }
}
unsafe fn CalcNewBarValue(
    maxValue: i32,
    oldValue: i32,
    receivedValue: i32,
    currValue: *mut i32,
    mut scale: u8,
    toAdd: u16,
) -> i32 {
    let mut ret: i32 = 0;
    scale *= 8;
    if *currValue == -32768 {
        if maxValue < scale as i32 {
            *currValue = oldValue << 8;
        } else {
            *currValue = oldValue;
        }
    }
    let mut newValue: i32 = oldValue - receivedValue;
    if newValue < 0 {
        newValue = 0;
    } else if newValue > maxValue {
        newValue = maxValue;
    }
    if maxValue < scale as i32 {
        if newValue == *currValue >> 8 && *currValue & 0xFF == 0 {
            return -1;
        }
    } else {
        if newValue == *currValue {
            return -1;
        }
    }
    if maxValue < scale as i32 {
        let toAdd: i32 = div_i32(maxValue << 8, scale as i32);
        if receivedValue < 0 {
            *currValue += toAdd;
            ret = *currValue >> 8;
            if ret >= newValue {
                *currValue = newValue << 8;
                ret = newValue;
            }
        } else {
            *currValue -= toAdd;
            ret = *currValue >> 8;
            if *currValue & 0xFF > 0 {
                ret += 1;
            }
            if ret <= newValue {
                *currValue = newValue << 8;
                ret = newValue;
            }
        }
    } else {
        if receivedValue < 0 {
            *currValue += toAdd as i32;
            if *currValue > newValue {
                *currValue = newValue;
            }
            ret = *currValue;
        } else {
            *currValue -= toAdd as i32;
            if *currValue < newValue {
                *currValue = newValue;
            }
            ret = *currValue;
        }
    }
    ret
}
unsafe fn CalcBarFilledPixels(
    maxValue: i32,
    oldValue: i32,
    receivedValue: i32,
    currValue: *mut i32,
    pixelsArray: *mut u8,
    scale: u8,
) -> u8 {
    let mut pixels: u8 = 0;
    let mut newValue: i32 = oldValue - receivedValue;
    if newValue < 0 {
        newValue = 0;
    } else if newValue > maxValue {
        newValue = maxValue;
    }
    let totalPixels: u8 = scale * 8;
    let mut i: u8 = 0;
    while i < scale {
        *pixelsArray.at(i) = 0;
        i += 1;
    }
    if maxValue < totalPixels as i32 {
        pixels = (div_i32(*currValue * totalPixels as i32, maxValue) >> 8) as u8;
    } else {
        pixels = div_i32(*currValue * totalPixels as i32, maxValue) as u8;
    }
    let mut filledPixels: u8 = pixels;
    if filledPixels == 0 && newValue > 0 {
        *pixelsArray = 1;
        filledPixels = 1;
    } else {
        for i in 0..scale {
            if pixels >= 8 {
                *pixelsArray.at(i) = 8;
            } else {
                *pixelsArray.at(i) = pixels;
                break;
            }
            pixels -= 8;
        }
    }
    filledPixels
}
unsafe fn Debug_TestHealthBar(
    barInfo: *mut TestingBar,
    currValue: *mut i32,
    dest: *mut u16,
    unused: i32,
) -> i16 {
    let mut var: i16 = 0;
    let ret: i16 = CalcNewBarValue(
        (*barInfo).maxValue,
        (*barInfo).oldValue,
        (*barInfo).receivedValue,
        currValue,
        6,
        1,
    ) as i16;
    Debug_TestHealthBar_Helper(barInfo, currValue, dest);
    if (*barInfo).maxValue < B_HEALTHBAR_PIXELS {
        var = (*currValue >> 8) as i16;
    } else {
        var = *currValue as i16;
    }
    DummiedOutFunction((*barInfo).maxValue as i16, var, unused);
    ret
}
unsafe fn Debug_TestHealthBar_Helper(
    barInfo: *mut TestingBar,
    currValue: *mut i32,
    dest: *mut u16,
) {
    let mut pixels: CArray<u8, 6> = zeroed();
    let mut src: CArray<u16, 6> = zeroed();
    CalcBarFilledPixels(
        (*barInfo).maxValue,
        (*barInfo).oldValue,
        (*barInfo).receivedValue,
        currValue,
        pixels.as_mut_ptr(),
        6,
    );
    for i in 0..6u8 {
        src[i] =
            (((*barInfo).unkC_0() as u16) << 12) | ((*barInfo).unk10 as u16 + pixels[i] as u16);
    }
    CpuSet(src.as_mut_ptr() as *mut c_void, dest as *mut c_void, 6);
}
fn GetScaledExpFraction(oldValue: i32, receivedValue: i32, maxValue: i32, mut scale: u8) -> u8 {
    scale *= 8;
    let mut newVal: i32 = oldValue - receivedValue;
    if newVal < 0 {
        newVal = 0;
    } else if newVal > maxValue {
        newVal = maxValue;
    }
    let oldToMax: i8 = div_i32(oldValue * scale as i32, maxValue) as i8;
    let newToMax: i8 = div_i32(newVal * scale as i32, maxValue) as i8;
    let result: i32 = oldToMax as i32 - newToMax as i32;
    (if result < 0 { -result } else { result }) as u8
}
pub fn GetScaledHPFraction(hp: i16, maxhp: i16, scale: u8) -> u8 {
    let result: u8 = div_i32(hp as i32 * scale as i32, maxhp as i32) as u8;
    if result == 0 && hp > 0 {
        return 1;
    }
    result
}
pub unsafe fn GetHPBarLevel(hp: i16, maxhp: i16) -> u8 {
    let mut result: u8 = 0;
    if hp == maxhp {
        result = HP_BAR_FULL;
    } else {
        let fraction: u8 = GetScaledHPFraction(hp, maxhp, B_HEALTHBAR_PIXELS as u8);
        if fraction > 24 {
            result = HP_BAR_GREEN;
        } else if fraction > 9 {
            result = HP_BAR_YELLOW;
        } else if fraction > 0 {
            result = HP_BAR_RED;
        } else {
            result = HP_BAR_EMPTY;
        }
    }
    result
}
unsafe fn AddTextPrinterAndCreateWindowOnHealthbox(
    str: *mut u8,
    x: u32,
    y: u32,
    bgColor: u32,
    windowId: *mut u32,
) -> *mut u8 {
    let mut color: CArray<u8, 3> = zeroed();
    let mut winTemplate: WindowTemplate = *sHealthboxWindowTemplate;
    let winId: u16 = AddWindow(&raw mut winTemplate);
    FillWindowPixelBuffer(winId as u8, bgColor as u8 | (bgColor as u8) << 4);
    color[0] = bgColor as u8;
    color[1] = 1;
    color[2] = 3;
    AddTextPrinterParameterized4(
        winId as u8,
        FONT_SMALL,
        x as u8,
        y as u8,
        0,
        0,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        str,
    );
    *windowId = winId as u32;
    GetWindowAttribute(winId as u8, WINDOW_TILE_DATA) as usize as *mut u8
}
unsafe fn RemoveWindowOnHealthbox(windowId: u32) {
    RemoveWindow(windowId as u8);
}
unsafe fn FillHealthboxObject(dest: *mut c_void, valMult: u32, numTiles: u32) {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0x11111111 * valMult);
            CpuSet(
                &raw mut tmp as *mut c_void,
                dest,
                0x5000000 | (numTiles * 32 / 4) & 0x1FFFFF,
            );
        }
    }
}
unsafe fn HpTextIntoHealthboxObject(dest: *mut c_void, windowTileData: *mut u8, windowWidth: u32) {
    CpuSet(
        windowTileData.at(256) as *mut c_void,
        dest,
        0x04000000 | (windowWidth * 32 / 4) & 0x1FFFFF,
    );
}
unsafe fn TextIntoHealthboxObject(
    mut dest: *mut c_void,
    mut windowTileData: *mut u8,
    windowWidth: i32,
) {
    CpuSet(
        windowTileData.at(256) as *mut c_void,
        (dest as *mut u8).at(256) as *mut c_void,
        0x04000000 | (windowWidth * 32 / 4) as u32 & 0x1FFFFF,
    );
    for i in 0..windowWidth {
        CpuSet(
            windowTileData.at(20) as *mut c_void,
            (dest as *mut u8).at(20) as *mut c_void,
            0x4000003,
        );
        dest = (dest as *mut u8).at(32) as *mut c_void;
        windowTileData = windowTileData.at(32);
    }
}
unsafe fn SafariTextIntoHealthboxObject(
    dest: *mut c_void,
    windowTileData: *mut u8,
    windowWidth: u32,
) {
    CpuSet(
        windowTileData as *mut c_void,
        dest,
        0x04000000 | (windowWidth * 32 / 4) & 0x1FFFFF,
    );
    CpuSet(
        windowTileData.at(256) as *mut c_void,
        (dest as *mut u8).at(256) as *mut c_void,
        0x04000000 | (windowWidth * 32 / 4) & 0x1FFFFF,
    );
}
