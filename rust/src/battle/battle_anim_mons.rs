//! Translated from `src/battle_anim_mons.c` by tools/rustport/c2rs.py.
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
    clippy::too_many_arguments,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::battle_anim::{
    DestroyAnimSprite, DestroyAnimVisualTask, IsBattlerSpriteVisible, IsContest,
    RelocateBattleBgPal, gAnimFriendship, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim::{gAnimBattlerSpecies, gBattleAnimArgs};
use crate::battle_anim_utility_funcs::{GetAnimBgAttribute, SetAnimBgAttribute};
use crate::battle_main::{
    AnimSetCenterToCornerVecX, gBattleAnimBgTileBuffer, gBattleAnimBgTilemapBuffer,
    gBattleSpritesDataPtr, gBattleTypeFlags, gBattlersCount, gMonSpritesGfxPtr,
    gTransformedPersonalities,
};
use crate::battle_main::{
    gBattleMonForms, gBattlerPartyIndexes, gBattlerPositions, gBattlerSpriteIds,
};
use crate::bg::{CopyBgTilemapBufferToVram, FillBgTilemapBufferRect};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::gContestResources;
use crate::gpu_regs::SetGpuReg;
use crate::palette::{LoadCompressedPalette, PaletteStruct_ResetById};
use crate::pokemon::{
    GetMonData2, GetMonSpritePalFromSpeciesAndPersonality, ShouldIgnoreDeoxysForm, gEnemyParty,
    gPlayerParty,
};
use crate::pokemon_icon::UpdateMonIconFrame;
use crate::sprite::gSprites;
use crate::sprite::{AllocSpritePalette, FreeSpritePaletteByTag, IndexOfSpritePaletteTag};
use crate::task::{gTasks, task_get, task_set, task_set_func};
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
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
/// `CopyToBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16) {
    unsafe {
        crate::bg::CopyToBgTilemapBuffer(a0, a1 as _, a2, a3);
    }
}
/// `CreateInvisibleSpriteWithCallback` with this module's view of its types.
#[inline]
unsafe fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe fn(*mut Sprite)>) -> u8 {
    unsafe { crate::util::CreateInvisibleSpriteWithCallback(core::mem::transmute(a0)) }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadBgTiles` with this module's view of its types.
#[inline]
unsafe fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTiles(a0, a1 as _, a2, a3) }
}
/// `LoadSpecialPokePic_2` with this module's view of its types.
#[inline]
unsafe fn LoadSpecialPokePic_2(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
    a4: u8,
) {
    unsafe {
        crate::decompress::LoadSpecialPokePic_2(a0 as _, a1 as _, a2, a3, a4);
    }
}
/// `LoadSpecialPokePic_DontHandleDeoxys` with this module's view of its types.
#[inline]
unsafe fn LoadSpecialPokePic_DontHandleDeoxys(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
    a4: u8,
) {
    unsafe {
        crate::decompress::LoadSpecialPokePic_DontHandleDeoxys(a0 as _, a1 as _, a2, a3, a4);
    }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `RequestDma3Copy` with this module's view of its types.
#[inline]
unsafe fn RequestDma3Copy(a0: *mut c_void, a1: *mut c_void, a2: u16, a3: u8) -> i16 {
    unsafe { crate::dma3_manager::RequestDma3Copy(a0 as _, a1 as _, a2, a3) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
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
const sActiveTime: usize = 0;
const sCirclePos: usize = 0;
const sMoveSteps: usize = 0;
const tBattlerSpriteId: usize = 0;
const sAmplitude: usize = 1;
const sSpeedX: usize = 1;
const sStartX: usize = 1;
const sTaskId: usize = 1;
const tMoveSpeed: usize = 1;
const sCircleSpeed: usize = 2;
const sSpeedY: usize = 2;
const sSpriteId: usize = 2;
const sTargetX: usize = 2;
const tState: usize = 2;
const sDuration: usize = 3;
const sStartY: usize = 3;
const tCounter: usize = 3;
const sAmplitudeSpeed: usize = 4;
const sAmplitudeY: usize = 4;
const sCirclePosY: usize = 4;
const sTargetY: usize = 4;
const tPaletteNum: usize = 4;
const sAmplitudeChange: usize = 5;
const sCircleSpeedY: usize = 5;
const tNumTracesActive: usize = 5;
const tPriority: usize = 6;
// Data tables (translate with cdata.py): sBattlerCoords gCastformFrontSpriteCoords sCastformElevations sCastformBackSpriteYCoords sSpriteTemplates_MoveEffectMons sSpriteSheets_MoveEffectMons

const BG_ANIM_PAL_1: u8 = 8;
const BG_ANIM_PAL_2: u8 = 9;
const BG_ANIM_PAL_CONTEST: u8 = 14;

static gCastformFrontSpriteCoords: Table<CArray<MonCoords, 4>> =
    Table((&raw const crate::data::battle_anim_mons::gCastformFrontSpriteCoords).cast());
static sBattlerCoords: Table<CArray<CArray<UCoords8, 4>, 2>> =
    Table((&raw const crate::data::battle_anim_mons::sBattlerCoords).cast());
static sCastformBackSpriteYCoords: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_anim_mons::sCastformBackSpriteYCoords).cast());
static sCastformElevations: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_anim_mons::sCastformElevations).cast());
static sSpriteSheets_MoveEffectMons: Table<CArray<SpriteSheet, 2>> =
    Table((&raw const crate::data::battle_anim_mons::sSpriteSheets_MoveEffectMons).cast());
static sSpriteTemplates_MoveEffectMons: Table<CArray<SpriteTemplate, 2>> =
    Table((&raw const crate::data::battle_anim_mons::sSpriteTemplates_MoveEffectMons).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimTaskAffineAnim: *mut AffineAnimCmd = null_mut();

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `ArcTan2` with this module's view of its types.
#[inline]
unsafe fn ArcTan2(a0: i16, a1: i16) -> u16 {
    unsafe { crate::syscall::ArcTan2(a0, a1) }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `ObjAffineSet` with this module's view of its types.
#[inline]
unsafe fn ObjAffineSet(a0: *mut ObjAffineSrcData, a1: *mut c_void, a2: i32, a3: i32) {
    unsafe {
        crate::syscall::ObjAffineSet(a0 as _, a1 as _, a2, a3);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn GetBattlerSpriteCoord(battler: u8, mut coordType: u8) -> u8 {
    let mut retVal: u8 = 0;
    let mut species: u16 = 0;
    let mut spriteInfo: *mut BattleSpriteInfo = null_mut();
    if IsContest() != 0 && coordType == BATTLER_COORD_Y_PIC_OFFSET && battler == 3 {
        coordType = BATTLER_COORD_Y;
    }
    match coordType {
        BATTLER_COORD_X | BATTLER_COORD_X_2 => {
            retVal = sBattlerCoords[gBattleTypeFlags & 1][GetBattlerPosition(battler)].x;
        }
        BATTLER_COORD_Y => {
            retVal = sBattlerCoords[gBattleTypeFlags & 1][GetBattlerPosition(battler)].y;
        }
        _ => {
            if IsContest() != 0 {
                if (*(*gContestResources).moveAnim).hasTargetAnim() != 0 {
                    species = (*(*gContestResources).moveAnim).targetSpecies;
                } else {
                    species = (*(*gContestResources).moveAnim).species;
                }
            } else {
                if GetBattlerSide(battler) != B_SIDE_PLAYER {
                    spriteInfo = (*gBattleSpritesDataPtr).battlerData;
                    if (*spriteInfo.at(battler)).transformSpecies == 0 {
                        species = GetMonData2(
                            &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                            MON_DATA_SPECIES,
                        ) as u16;
                    } else {
                        species = (*spriteInfo.at(battler)).transformSpecies;
                    }
                } else {
                    spriteInfo = (*gBattleSpritesDataPtr).battlerData;
                    if (*spriteInfo.at(battler)).transformSpecies == 0 {
                        species = GetMonData2(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                            MON_DATA_SPECIES,
                        ) as u16;
                    } else {
                        species = (*spriteInfo.at(battler)).transformSpecies;
                    }
                }
            }
            if coordType == BATTLER_COORD_Y_PIC_OFFSET {
                retVal = GetBattlerSpriteFinal_Y(battler, species, TRUE);
            } else {
                retVal = GetBattlerSpriteFinal_Y(battler, species, FALSE);
            }
        }
    }
    retVal
}
pub unsafe fn GetBattlerYDelta(battler: u8, species: u16) -> u8 {
    let mut letter: u16 = 0;
    let mut personality: u32 = 0;
    let mut spriteInfo: *mut BattleSpriteInfo = null_mut();
    let mut ret: u8 = 0;
    let mut coordSpecies: u16 = 0;
    if GetBattlerSide(battler) == B_SIDE_PLAYER || IsContest() != 0 {
        if species == SPECIES_UNOWN {
            if IsContest() != 0 {
                if (*(*gContestResources).moveAnim).hasTargetAnim() != 0 {
                    personality = (*(*gContestResources).moveAnim).targetPersonality;
                } else {
                    personality = (*(*gContestResources).moveAnim).personality;
                }
            } else {
                spriteInfo = (*gBattleSpritesDataPtr).battlerData;
                if (*spriteInfo.at(battler)).transformSpecies == 0 {
                    personality = GetMonData2(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                        MON_DATA_PERSONALITY,
                    );
                } else {
                    personality = gTransformedPersonalities[battler];
                }
            }
            letter = (((personality & 0x03000000) >> 18
                | (personality & 0x00030000) >> 12
                | (personality & 0x00000300) >> 6
                | (personality & 0x00000003))
                % 28) as u16;
            if letter == 0 {
                coordSpecies = species;
            } else {
                coordSpecies = letter + SPECIES_UNOWN_B - 1;
            }
            ret = (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[coordSpecies]
                .y_offset;
        } else if species == SPECIES_CASTFORM {
            ret = sCastformBackSpriteYCoords[gBattleMonForms[battler]];
        } else if species > NUM_SPECIES {
            ret = (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[0]
                .y_offset;
        } else {
            ret = (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[species]
                .y_offset;
        }
    } else {
        if species == SPECIES_UNOWN {
            spriteInfo = (*gBattleSpritesDataPtr).battlerData;
            if (*spriteInfo.at(battler)).transformSpecies == 0 {
                personality = GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                    MON_DATA_PERSONALITY,
                );
            } else {
                personality = gTransformedPersonalities[battler];
            }
            letter = (((personality & 0x03000000) >> 18
                | (personality & 0x00030000) >> 12
                | (personality & 0x00000300) >> 6
                | (personality & 0x00000003))
                % 28) as u16;
            if letter == 0 {
                coordSpecies = species;
            } else {
                coordSpecies = letter + SPECIES_UNOWN_B - 1;
            }
            ret = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                .cast::<CArray<MonCoords, 0>>())[coordSpecies]
                .y_offset;
        } else if species == SPECIES_CASTFORM {
            ret = gCastformFrontSpriteCoords[gBattleMonForms[battler]].y_offset;
        } else if species > NUM_SPECIES {
            ret = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                .cast::<CArray<MonCoords, 0>>())[0]
                .y_offset;
        } else {
            ret = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                .cast::<CArray<MonCoords, 0>>())[species]
                .y_offset;
        }
    }
    ret
}
pub unsafe fn GetBattlerElevation(battler: u8, species: u16) -> u8 {
    let mut ret: u8 = 0;
    if GetBattlerSide(battler) == B_SIDE_OPPONENT && IsContest() == 0 {
        if species == SPECIES_CASTFORM {
            ret = sCastformElevations[gBattleMonForms[battler]];
        } else if species > NUM_SPECIES {
            ret = (*(&raw const crate::data::data_tables::gEnemyMonElevation)
                .cast::<CArray<u8, 412>>())[0];
        } else {
            ret = (*(&raw const crate::data::data_tables::gEnemyMonElevation)
                .cast::<CArray<u8, 412>>())[species];
        }
    }
    ret
}
pub unsafe fn GetBattlerSpriteFinal_Y(battler: u8, species: u16, a3: u8) -> u8 {
    let mut offset: u16 = 0;
    let mut y: u8 = 0;
    if GetBattlerSide(battler) == B_SIDE_PLAYER || IsContest() != 0 {
        offset = GetBattlerYDelta(battler, species) as u16;
    } else {
        offset = GetBattlerYDelta(battler, species) as u16;
        offset -= GetBattlerElevation(battler, species) as u16;
    }
    y = offset as u8 + sBattlerCoords[gBattleTypeFlags & 1][GetBattlerPosition(battler)].y;
    if a3 != 0 {
        if GetBattlerSide(battler) == B_SIDE_PLAYER {
            y += 8;
        }
        if y > 104 {
            y = 104;
        }
    }
    y
}
pub unsafe fn GetBattlerSpriteCoord2(battler: u8, coordType: u8) -> u8 {
    let mut species: u16 = 0;
    let mut spriteInfo: *mut BattleSpriteInfo = null_mut();
    if coordType == BATTLER_COORD_Y_PIC_OFFSET || coordType == BATTLER_COORD_Y_PIC_OFFSET_DEFAULT {
        if IsContest() != 0 {
            if (*(*gContestResources).moveAnim).hasTargetAnim() != 0 {
                species = (*(*gContestResources).moveAnim).targetSpecies;
            } else {
                species = (*(*gContestResources).moveAnim).species;
            }
        } else {
            spriteInfo = (*gBattleSpritesDataPtr).battlerData;
            if (*spriteInfo.at(battler)).transformSpecies == 0 {
                species = gAnimBattlerSpecies[battler];
            } else {
                species = (*spriteInfo.at(battler)).transformSpecies;
            }
        }
        if coordType == BATTLER_COORD_Y_PIC_OFFSET {
            return GetBattlerSpriteFinal_Y(battler, species, TRUE);
        } else {
            return GetBattlerSpriteFinal_Y(battler, species, FALSE);
        }
    } else {
        return GetBattlerSpriteCoord(battler, coordType);
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetBattlerSpriteDefault_Y(battler: u8) -> u8 {
    GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET_DEFAULT)
}
#[unsafe(no_mangle)]
pub unsafe fn GetSubstituteSpriteDefault_Y(battler: u8) -> u8 {
    let mut y: u16 = 0;
    if GetBattlerSide(battler) != B_SIDE_PLAYER {
        y = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as u16 + 16;
    } else {
        y = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as u16 + 17;
    }
    y as u8
}
pub unsafe fn GetBattlerYCoordWithElevation(battler: u8) -> u8 {
    let mut species: u16 = 0;
    let mut spriteInfo: *mut BattleSpriteInfo = null_mut();
    let mut y: u8 = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y);
    if IsContest() == 0 {
        if GetBattlerSide(battler) != B_SIDE_PLAYER {
            spriteInfo = (*gBattleSpritesDataPtr).battlerData;
            if (*spriteInfo.at(battler)).transformSpecies == 0 {
                species = GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                    MON_DATA_SPECIES,
                ) as u16;
            } else {
                species = (*spriteInfo.at(battler)).transformSpecies;
            }
        } else {
            spriteInfo = (*gBattleSpritesDataPtr).battlerData;
            if (*spriteInfo.at(battler)).transformSpecies == 0 {
                species = GetMonData2(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                    MON_DATA_SPECIES,
                ) as u16;
            } else {
                species = (*spriteInfo.at(battler)).transformSpecies;
            }
        }
        if GetBattlerSide(battler) != B_SIDE_PLAYER {
            y -= GetBattlerElevation(battler, species);
        }
    }
    y
}
pub unsafe fn GetAnimBattlerSpriteId(animBattler: u8) -> u8 {
    let mut sprites: *mut u8 = null_mut();
    if animBattler == ANIM_ATTACKER {
        if IsBattlerSpritePresent(gBattleAnimAttacker) != 0 {
            sprites = gBattlerSpriteIds.as_mut_ptr();
            return *sprites.at(gBattleAnimAttacker);
        } else {
            return SPRITE_NONE;
        }
    } else if animBattler == ANIM_TARGET {
        if IsBattlerSpritePresent(gBattleAnimTarget) != 0 {
            sprites = gBattlerSpriteIds.as_mut_ptr();
            return *sprites.at(gBattleAnimTarget);
        } else {
            return SPRITE_NONE;
        }
    } else if animBattler == ANIM_ATK_PARTNER {
        if IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) == 0 {
            return SPRITE_NONE;
        } else {
            return gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2];
        }
    } else {
        if IsBattlerSpriteVisible(gBattleAnimTarget ^ 2) != 0 {
            return gBattlerSpriteIds[gBattleAnimTarget as i32 ^ 2];
        } else {
            return SPRITE_NONE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn StoreSpriteCallbackInData6(
    sprite: *mut Sprite,
    callback: Option<unsafe fn(*mut Sprite)>,
) {
    (*sprite).data[6] = core::mem::transmute::<_, usize>(callback) as u32 as i16;
    (*sprite).data[7] = (core::mem::transmute::<_, usize>(callback) as u32 >> 16) as i16;
}
pub unsafe fn SetCallbackToStoredInData6(sprite: *mut Sprite) {
    let callback: u32 = (*sprite).data[6] as u16 as u32 | ((*sprite).data[7] as u32) << 16;
    (*sprite).callback =
        core::mem::transmute::<usize, Option<unsafe fn(*mut Sprite)>>(callback as usize);
}
pub unsafe fn TranslateSpriteInCircle(sprite: *mut Sprite) {
    if (*sprite).data[sDuration] != 0 {
        (*sprite).x2 = Sin((*sprite).data[sCirclePos], (*sprite).data[sAmplitude]);
        (*sprite).y2 = Cos((*sprite).data[sCirclePos], (*sprite).data[sAmplitude]);
        (*sprite).data[sCirclePos] += (*sprite).data[sCircleSpeed];
        if (*sprite).data[sCirclePos] >= 0x100 {
            (*sprite).data[sCirclePos] -= 0x100;
        } else if (*sprite).data[sCirclePos] < 0 {
            (*sprite).data[sCirclePos] += 0x100;
        }
        (*sprite).data[sDuration] -= 1;
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn TranslateSpriteInGrowingCircle(sprite: *mut Sprite) {
    if (*sprite).data[sDuration] != 0 {
        (*sprite).x2 = Sin(
            (*sprite).data[sCirclePos],
            ((*sprite).data[sAmplitudeChange] >> 8) + (*sprite).data[sAmplitude],
        );
        (*sprite).y2 = Cos(
            (*sprite).data[sCirclePos],
            ((*sprite).data[sAmplitudeChange] >> 8) + (*sprite).data[sAmplitude],
        );
        (*sprite).data[sCirclePos] += (*sprite).data[sCircleSpeed];
        (*sprite).data[sAmplitudeChange] += (*sprite).data[sAmplitudeSpeed];
        if (*sprite).data[sCirclePos] >= 0x100 {
            (*sprite).data[sCirclePos] -= 0x100;
        } else if (*sprite).data[sCirclePos] < 0 {
            (*sprite).data[sCirclePos] += 0x100;
        }
        (*sprite).data[sDuration] -= 1;
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
unsafe fn TranslateSpriteInLissajousCurve(sprite: *mut Sprite) {
    if (*sprite).data[sDuration] != 0 {
        (*sprite).x2 = Sin((*sprite).data[0], (*sprite).data[sAmplitude]);
        (*sprite).y2 = Cos((*sprite).data[sCirclePosY], (*sprite).data[sAmplitude]);
        (*sprite).data[0] += (*sprite).data[2];
        (*sprite).data[sCirclePosY] += (*sprite).data[sCircleSpeedY];
        if (*sprite).data[0] >= 0x100 {
            (*sprite).data[0] -= 0x100;
        } else if (*sprite).data[0] < 0 {
            (*sprite).data[0] += 0x100;
        }
        if (*sprite).data[sCirclePosY] >= 0x100 {
            (*sprite).data[sCirclePosY] -= 0x100;
        } else if (*sprite).data[sCirclePosY] < 0 {
            (*sprite).data[sCirclePosY] += 0x100;
        }
        (*sprite).data[sDuration] -= 1;
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn TranslateSpriteInEllipse(sprite: *mut Sprite) {
    if (*sprite).data[sDuration] != 0 {
        (*sprite).x2 = Sin((*sprite).data[sCirclePos], (*sprite).data[1]);
        (*sprite).y2 = Cos((*sprite).data[sCirclePos], (*sprite).data[sAmplitudeY]);
        (*sprite).data[sCirclePos] += (*sprite).data[sCircleSpeed];
        if (*sprite).data[sCirclePos] >= 0x100 {
            (*sprite).data[sCirclePos] -= 0x100;
        } else if (*sprite).data[sCirclePos] < 0 {
            (*sprite).data[sCirclePos] += 0x100;
        }
        (*sprite).data[sDuration] -= 1;
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn WaitAnimForDuration(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
pub(crate) unsafe fn AnimPosToTranslateLinear(sprite: *mut Sprite) {
    ConvertPosDataToTranslateLinearData(sprite);
    (*sprite).callback = Some(TranslateSpriteLinear);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub unsafe fn ConvertPosDataToTranslateLinearData(sprite: *mut Sprite) {
    if (*sprite).data[1] > (*sprite).data[2] {
        (*sprite).data[0] = -(*sprite).data[0];
    }
    let xDiff: i32 = (*sprite).data[2] as i32 - (*sprite).data[1] as i32;
    let old: i16 = (*sprite).data[0];
    (*sprite).data[0] = (if div_i32(xDiff, (*sprite).data[0] as i32) < 0 {
        -div_i32(xDiff, (*sprite).data[0] as i32)
    } else {
        div_i32(xDiff, (*sprite).data[0] as i32)
    }) as i16;
    (*sprite).data[2] = div_i32(
        (*sprite).data[sTargetY] as i32 - (*sprite).data[sStartY] as i32,
        (*sprite).data[0] as i32,
    ) as i16;
    (*sprite).data[1] = old;
}
pub unsafe fn TranslateSpriteLinear(sprite: *mut Sprite) {
    if (*sprite).data[sMoveSteps] > 0 {
        (*sprite).data[sMoveSteps] -= 1;
        (*sprite).x2 += (*sprite).data[sSpeedX];
        (*sprite).y2 += (*sprite).data[sSpeedY];
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn TranslateSpriteLinearFixedPoint(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
        (*sprite).data[3] += (*sprite).data[1];
        (*sprite).data[4] += (*sprite).data[2];
        (*sprite).x2 = (*sprite).data[3] >> 8;
        (*sprite).y2 = (*sprite).data[4] >> 8;
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
pub(crate) unsafe fn TranslateSpriteLinearFixedPointIconFrame(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
        (*sprite).data[3] += (*sprite).data[1];
        (*sprite).data[4] += (*sprite).data[2];
        (*sprite).x2 = (*sprite).data[3] >> 8;
        (*sprite).y2 = (*sprite).data[4] >> 8;
    } else {
        SetCallbackToStoredInData6(sprite);
    }
    UpdateMonIconFrame(sprite);
}
unsafe fn TranslateSpriteToBattleTargetPos(sprite: *mut Sprite) {
    (*sprite).data[sStartX] = (*sprite).x + (*sprite).x2;
    (*sprite).data[sStartY] = (*sprite).y + (*sprite).y2;
    (*sprite).data[sTargetX] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[sTargetY] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).callback = Some(AnimPosToTranslateLinear);
}
pub unsafe fn TranslateSpriteLinearById(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
        gSprites[(*sprite).data[3]].x2 += (*sprite).data[1];
        gSprites[(*sprite).data[3]].y2 += (*sprite).data[2];
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn TranslateSpriteLinearByIdFixedPoint(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
        (*sprite).data[3] += (*sprite).data[1];
        (*sprite).data[4] += (*sprite).data[2];
        gSprites[(*sprite).data[5]].x2 = (*sprite).data[3] >> 8;
        gSprites[(*sprite).data[5]].y2 = (*sprite).data[4] >> 8;
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn TranslateSpriteLinearAndFlicker(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
        (*sprite).x2 = (*sprite).data[2] >> 8;
        (*sprite).data[2] += (*sprite).data[1];
        (*sprite).y2 = (*sprite).data[4] >> 8;
        (*sprite).data[4] += (*sprite).data[3];
        if rem_i32((*sprite).data[0] as i32, (*sprite).data[5] as i32) == 0
            && (*sprite).data[5] != 0
        {
            (*sprite).set_invisible((*sprite).invisible() ^ 1);
        }
    } else {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn DestroySpriteAndMatrix(sprite: *mut Sprite) {
    FreeSpriteOamMatrix(sprite);
    DestroyAnimSprite(sprite);
}
unsafe fn TranslateSpriteToBattleAttackerPos(sprite: *mut Sprite) {
    (*sprite).data[sStartX] = (*sprite).x + (*sprite).x2;
    (*sprite).data[sStartY] = (*sprite).y + (*sprite).y2;
    (*sprite).data[sTargetX] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).data[sTargetY] =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).callback = Some(AnimPosToTranslateLinear);
}
unsafe fn EndUnkPaletteAnim(sprite: *mut Sprite) {
    PaletteStruct_ResetById((*sprite).data[5] as u16);
    DestroySpriteAndMatrix(sprite);
}
pub unsafe fn RunStoredCallbackWhenAffineAnimEnds(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn RunStoredCallbackWhenAnimEnds(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn DestroyAnimSpriteAndDisableBlend(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    DestroyAnimSprite(sprite);
}
pub unsafe fn DestroyAnimVisualTaskAndDisableBlend(taskId: u8) {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    DestroyAnimVisualTask(taskId);
}
pub unsafe fn SetSpriteCoordsToAnimAttackerCoords(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
}
pub unsafe fn SetAnimSpriteInitialXOffset(sprite: *mut Sprite, xOffset: i16) {
    let attackerX: u16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as u16;
    let targetX: u16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as u16;
    if attackerX > targetX {
        (*sprite).x -= xOffset;
    } else if attackerX < targetX {
        (*sprite).x += xOffset;
    } else {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            (*sprite).x -= xOffset;
        } else {
            (*sprite).x += xOffset;
        }
    }
}
pub unsafe fn InitAnimArcTranslation(sprite: *mut Sprite) {
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    InitAnimLinearTranslation(sprite);
    (*sprite).data[6] = div_i32(0x8000, (*sprite).data[0] as i32) as i16;
    (*sprite).data[7] = 0;
}
pub unsafe fn TranslateAnimHorizontalArc(sprite: *mut Sprite) -> u8 {
    if AnimTranslateLinear(sprite) != 0 {
        return TRUE;
    }
    (*sprite).data[7] += (*sprite).data[6];
    (*sprite).y2 += Sin(((*sprite).data[7] >> 8) as u8 as i16, (*sprite).data[5]);
    FALSE
}
pub unsafe fn TranslateAnimVerticalArc(sprite: *mut Sprite) -> u8 {
    if AnimTranslateLinear(sprite) != 0 {
        return TRUE;
    }
    (*sprite).data[7] += (*sprite).data[6];
    (*sprite).x2 += Sin(((*sprite).data[7] >> 8) as u8 as i16, (*sprite).data[5]);
    FALSE
}
pub unsafe fn SetSpritePrimaryCoordsFromSecondaryCoords(sprite: *mut Sprite) {
    (*sprite).x += (*sprite).x2;
    (*sprite).y += (*sprite).y2;
    (*sprite).x2 = 0;
    (*sprite).y2 = 0;
}
pub unsafe fn InitSpritePosToAnimTarget(sprite: *mut Sprite, respectMonPicOffsets: u8) {
    if respectMonPicOffsets == 0 {
        (*sprite).x = GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    }
    SetAnimSpriteInitialXOffset(sprite, gBattleAnimArgs[0]);
    (*sprite).y += gBattleAnimArgs[1];
}
pub unsafe fn InitSpritePosToAnimAttacker(sprite: *mut Sprite, respectMonPicOffsets: u8) {
    if respectMonPicOffsets == 0 {
        (*sprite).x = GetBattlerSpriteCoord2(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord2(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
    } else {
        (*sprite).x = GetBattlerSpriteCoord2(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).y =
            GetBattlerSpriteCoord2(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    }
    SetAnimSpriteInitialXOffset(sprite, gBattleAnimArgs[0]);
    (*sprite).y += gBattleAnimArgs[1];
}
#[unsafe(no_mangle)]
pub unsafe fn GetBattlerSide(battler: u8) -> u8 {
    gBattlerPositions[battler] & 1
}
#[unsafe(no_mangle)]
pub unsafe fn GetBattlerPosition(battler: u8) -> u8 {
    gBattlerPositions[battler]
}
#[unsafe(no_mangle)]
pub unsafe fn GetBattlerAtPosition(position: u8) -> u8 {
    let mut i: u8 = 0;
    while i < gBattlersCount {
        if gBattlerPositions[i] == position {
            break;
        }
        i += 1;
    }
    i
}
pub unsafe fn IsBattlerSpritePresent(battler: u8) -> u8 {
    if IsContest() != 0 {
        if gBattleAnimAttacker == battler {
            return TRUE;
        } else if gBattleAnimTarget == battler {
            return TRUE;
        } else {
            return FALSE;
        }
    } else {
        if gBattlerPositions[battler] == 0xff {
            return FALSE;
        } else if GetBattlerSide(battler) != B_SIDE_PLAYER {
            if GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                MON_DATA_HP,
            ) != 0
            {
                return TRUE;
            }
        } else {
            if GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                MON_DATA_HP,
            ) != 0
            {
                return TRUE;
            }
        }
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn IsDoubleBattle() -> u8 {
    gBattleTypeFlags as u8 & 1
}
pub unsafe fn GetBattleAnimBg1Data(out: *mut BattleAnimBgData) {
    if IsContest() != 0 {
        (*out).bgTiles = gBattleAnimBgTileBuffer;
        (*out).bgTilemap = gBattleAnimBgTilemapBuffer as *mut u16;
        (*out).paletteId = BG_ANIM_PAL_CONTEST;
        (*out).bgId = 1;
        (*out).tilesOffset = 0;
        (*out).unused = 0;
    } else {
        (*out).bgTiles = gBattleAnimBgTileBuffer;
        (*out).bgTilemap = gBattleAnimBgTilemapBuffer as *mut u16;
        (*out).paletteId = BG_ANIM_PAL_1;
        (*out).bgId = 1;
        (*out).tilesOffset = 0x200;
        (*out).unused = 0;
    }
}
pub unsafe fn GetBattleAnimBgData(out: *mut BattleAnimBgData, bgId: u32) {
    if IsContest() != 0 {
        (*out).bgTiles = gBattleAnimBgTileBuffer;
        (*out).bgTilemap = gBattleAnimBgTilemapBuffer as *mut u16;
        (*out).paletteId = BG_ANIM_PAL_CONTEST;
        (*out).bgId = 1;
        (*out).tilesOffset = 0;
        (*out).unused = 0;
    } else if bgId == 1 {
        GetBattleAnimBg1Data(out);
    } else {
        (*out).bgTiles = gBattleAnimBgTileBuffer;
        (*out).bgTilemap = gBattleAnimBgTilemapBuffer as *mut u16;
        (*out).paletteId = BG_ANIM_PAL_2;
        (*out).bgId = 2;
        (*out).tilesOffset = 0x300;
        (*out).unused = 0;
    }
}
pub unsafe fn GetBgDataForTransform(out: *mut BattleAnimBgData, battler: u8) {
    (*out).bgTiles = gBattleAnimBgTileBuffer;
    (*out).bgTilemap = gBattleAnimBgTilemapBuffer as *mut u16;
    if IsContest() != 0 {
        (*out).paletteId = BG_ANIM_PAL_CONTEST;
        (*out).bgId = 1;
        (*out).tilesOffset = 0;
        (*out).unused = 0;
    } else if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == 1 {
        (*out).paletteId = BG_ANIM_PAL_1;
        (*out).bgId = 1;
        (*out).tilesOffset = 0x200;
        (*out).unused = 0;
    } else {
        (*out).paletteId = BG_ANIM_PAL_2;
        (*out).bgId = 2;
        (*out).tilesOffset = 0x300;
        (*out).unused = 0;
    }
}
pub unsafe fn ClearBattleAnimBg(bgId: u32) {
    let mut bgAnimData: BattleAnimBgData = zeroed();
    GetBattleAnimBgData(&raw mut bgAnimData, bgId);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                bgAnimData.bgTiles as *mut c_void,
                0x5000800,
            );
        }
    }
    LoadBgTiles(
        bgAnimData.bgId,
        bgAnimData.bgTiles as *mut c_void,
        0x2000,
        bgAnimData.tilesOffset,
    );
    FillBgTilemapBufferRect(bgAnimData.bgId, 0, 0, 0, 32, 64, 17);
    CopyBgTilemapBufferToVram(bgAnimData.bgId);
}
pub unsafe fn AnimLoadCompressedBgGfx(bgId: u32, src: *mut u32, tilesOffset: u32) {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gBattleAnimBgTileBuffer as *mut c_void,
                0x5000800,
            );
        }
    }
    LZDecompressWram(src, gBattleAnimBgTileBuffer as *mut c_void);
    LoadBgTiles(
        bgId as u8,
        gBattleAnimBgTileBuffer as *mut c_void,
        0x2000,
        tilesOffset as u16,
    );
}
unsafe fn InitAnimBgTilemapBuffer(bgId: u32, src: *mut c_void) {
    FillBgTilemapBufferRect(bgId as u8, 0, 0, 0, 32, 64, 17);
    CopyToBgTilemapBuffer(bgId as u8, src, 0, 0);
}
pub unsafe fn AnimLoadCompressedBgTilemap(bgId: u32, src: *mut c_void) {
    InitAnimBgTilemapBuffer(bgId, src);
    CopyBgTilemapBufferToVram(bgId as u8);
}
pub unsafe fn AnimLoadCompressedBgTilemapHandleContest(
    data: *mut BattleAnimBgData,
    src: *mut c_void,
    largeScreen: u32,
) {
    InitAnimBgTilemapBuffer((*data).bgId as u32, src);
    if IsContest() == TRUE {
        RelocateBattleBgPal(
            (*data).paletteId as u16,
            (*data).bgTilemap,
            0,
            largeScreen as u8,
        );
    }
    CopyBgTilemapBufferToVram((*data).bgId);
}
pub unsafe fn GetBattleBgPaletteNum() -> u8 {
    if IsContest() != 0 {
        return 1;
    } else {
        return 2;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn UpdateAnimBg3ScreenSize(largeScreenSize: u8) {
    if largeScreenSize == 0 || IsContest() != 0 {
        SetAnimBgAttribute(3, BG_ANIM_SCREEN_SIZE, 0);
        SetAnimBgAttribute(3, BG_ANIM_AREA_OVERFLOW_MODE, 1);
    } else {
        SetAnimBgAttribute(3, BG_ANIM_SCREEN_SIZE, 1);
        SetAnimBgAttribute(3, BG_ANIM_AREA_OVERFLOW_MODE, 0);
    }
}
pub unsafe fn Trade_MoveSelectedMonToTarget(sprite: *mut Sprite) {
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    InitSpriteDataForLinearTranslation(sprite);
    (*sprite).callback = Some(TranslateSpriteLinearFixedPointIconFrame);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub unsafe fn InitSpriteDataForLinearTranslation(sprite: *mut Sprite) {
    let x: i16 = ((*sprite).data[2] - (*sprite).data[1]) << 8;
    let y: i16 = ((*sprite).data[4] - (*sprite).data[3]) << 8;
    (*sprite).data[1] = div_i32(x as i32, (*sprite).data[0] as i32) as i16;
    (*sprite).data[2] = div_i32(y as i32, (*sprite).data[0] as i32) as i16;
    (*sprite).data[4] = 0;
    (*sprite).data[3] = 0;
}
pub unsafe fn InitAnimLinearTranslation(sprite: *mut Sprite) {
    let x: i32 = (*sprite).data[2] as i32 - (*sprite).data[1] as i32;
    let y: i32 = (*sprite).data[4] as i32 - (*sprite).data[3] as i32;
    let movingLeft: u8 = (x < 0) as u8;
    let movingUp: u8 = (y < 0) as u8;
    let mut xDelta: u16 = ((if x < 0 { -x } else { x }) as u16) << 8;
    let mut yDelta: u16 = ((if y < 0 { -y } else { y }) as u16) << 8;
    xDelta = div_i32(xDelta as i32, (*sprite).data[0] as i32) as u16;
    yDelta = div_i32(yDelta as i32, (*sprite).data[0] as i32) as u16;
    if movingLeft != 0 {
        xDelta |= 1;
    } else {
        xDelta &= 65534;
    }
    if movingUp != 0 {
        yDelta |= 1;
    } else {
        yDelta &= 65534;
    }
    (*sprite).data[1] = xDelta as i16;
    (*sprite).data[2] = yDelta as i16;
    (*sprite).data[4] = 0;
    (*sprite).data[3] = 0;
}
pub unsafe fn StartAnimLinearTranslation(sprite: *mut Sprite) {
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    InitAnimLinearTranslation(sprite);
    (*sprite).callback = Some(AnimTranslateLinear_WithFollowup);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
unsafe fn StartAnimLinearTranslation_SetCornerVecX(sprite: *mut Sprite) {
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    InitAnimLinearTranslation(sprite);
    (*sprite).callback = Some(AnimTranslateLinear_WithFollowup_SetCornerVecX);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub unsafe fn AnimTranslateLinear(sprite: *mut Sprite) -> u8 {
    if (*sprite).data[0] == 0 {
        return TRUE;
    }
    let v1: u16 = (*sprite).data[1] as u16;
    let v2: u16 = (*sprite).data[2] as u16;
    let mut x: u16 = (*sprite).data[3] as u16;
    let mut y: u16 = (*sprite).data[4] as u16;
    x += v1;
    y += v2;
    if v1 as i32 & 1 != 0 {
        (*sprite).x2 = -((x >> 8) as i16);
    } else {
        (*sprite).x2 = (x >> 8) as i16;
    }
    if v2 as i32 & 1 != 0 {
        (*sprite).y2 = -((y >> 8) as i16);
    } else {
        (*sprite).y2 = (y >> 8) as i16;
    }
    (*sprite).data[3] = x as i16;
    (*sprite).data[4] = y as i16;
    (*sprite).data[0] -= 1;
    FALSE
}
pub unsafe fn AnimTranslateLinear_WithFollowup(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 {
        SetCallbackToStoredInData6(sprite);
    }
}
pub(crate) unsafe fn AnimTranslateLinear_WithFollowup_SetCornerVecX(sprite: *mut Sprite) {
    AnimSetCenterToCornerVecX(sprite);
    if AnimTranslateLinear(sprite) != 0 {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn InitAnimLinearTranslationWithSpeed(sprite: *mut Sprite) {
    let v1: i32 = (if ((*sprite).data[2] as i32 - (*sprite).data[1] as i32) < 0 {
        -((*sprite).data[2] as i32 - (*sprite).data[1] as i32)
    } else {
        (*sprite).data[2] as i32 - (*sprite).data[1] as i32
    }) << 8;
    (*sprite).data[0] = div_i32(v1, (*sprite).data[0] as i32) as i16;
    InitAnimLinearTranslation(sprite);
}
pub unsafe fn InitAnimLinearTranslationWithSpeedAndPos(sprite: *mut Sprite) {
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    InitAnimLinearTranslationWithSpeed(sprite);
    (*sprite).callback = Some(AnimTranslateLinear_WithFollowup);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
unsafe fn InitAnimFastLinearTranslation(sprite: *mut Sprite) {
    let xDiff: i32 = (*sprite).data[2] as i32 - (*sprite).data[1] as i32;
    let yDiff: i32 = (*sprite).data[4] as i32 - (*sprite).data[3] as i32;
    let x_sign: u8 = (xDiff < 0) as u8;
    let y_sign: u8 = (yDiff < 0) as u8;
    let mut x2: u16 = ((if xDiff < 0 { -xDiff } else { xDiff }) as u16) << 4;
    let mut y2: u16 = ((if yDiff < 0 { -yDiff } else { yDiff }) as u16) << 4;
    x2 = div_i32(x2 as i32, (*sprite).data[0] as i32) as u16;
    y2 = div_i32(y2 as i32, (*sprite).data[0] as i32) as u16;
    if x_sign != 0 {
        x2 |= 1;
    } else {
        x2 &= 65534;
    }
    if y_sign != 0 {
        y2 |= 1;
    } else {
        y2 &= 65534;
    }
    (*sprite).data[1] = x2 as i16;
    (*sprite).data[2] = y2 as i16;
    (*sprite).data[4] = 0;
    (*sprite).data[3] = 0;
}
pub unsafe fn InitAndRunAnimFastLinearTranslation(sprite: *mut Sprite) {
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    InitAnimFastLinearTranslation(sprite);
    (*sprite).callback = Some(AnimFastTranslateLinearWaitEnd);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub unsafe fn AnimFastTranslateLinear(sprite: *mut Sprite) -> u8 {
    if (*sprite).data[0] == 0 {
        return TRUE;
    }
    let v1: u16 = (*sprite).data[1] as u16;
    let v2: u16 = (*sprite).data[2] as u16;
    let mut x: u16 = (*sprite).data[3] as u16;
    let mut y: u16 = (*sprite).data[4] as u16;
    x += v1;
    y += v2;
    if v1 as i32 & 1 != 0 {
        (*sprite).x2 = -((x >> 4) as i16);
    } else {
        (*sprite).x2 = (x >> 4) as i16;
    }
    if v2 as i32 & 1 != 0 {
        (*sprite).y2 = -((y >> 4) as i16);
    } else {
        (*sprite).y2 = (y >> 4) as i16;
    }
    (*sprite).data[3] = x as i16;
    (*sprite).data[4] = y as i16;
    (*sprite).data[0] -= 1;
    FALSE
}
pub(crate) unsafe fn AnimFastTranslateLinearWaitEnd(sprite: *mut Sprite) {
    if AnimFastTranslateLinear(sprite) != 0 {
        SetCallbackToStoredInData6(sprite);
    }
}
pub unsafe fn InitAnimFastLinearTranslationWithSpeed(sprite: *mut Sprite) {
    let xDiff: i32 = (if ((*sprite).data[2] as i32 - (*sprite).data[1] as i32) < 0 {
        -((*sprite).data[2] as i32 - (*sprite).data[1] as i32)
    } else {
        (*sprite).data[2] as i32 - (*sprite).data[1] as i32
    }) << 4;
    (*sprite).data[0] = div_i32(xDiff, (*sprite).data[0] as i32) as i16;
    InitAnimFastLinearTranslation(sprite);
}
pub unsafe fn InitAnimFastLinearTranslationWithSpeedAndPos(sprite: *mut Sprite) {
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    InitAnimFastLinearTranslationWithSpeed(sprite);
    (*sprite).callback = Some(AnimFastTranslateLinearWaitEnd);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub unsafe fn SetSpriteRotScale(spriteId: u8, xScale: i16, yScale: i16, rotation: u16) {
    let mut src: ObjAffineSrcData = zeroed();
    let mut matrix: OamMatrix = zeroed();
    src.xScale = xScale;
    src.yScale = yScale;
    src.rotation = rotation;
    if ShouldRotScaleSpeciesBeFlipped() != 0 {
        src.xScale = -src.xScale;
    }
    let i: i32 = gSprites[spriteId].oam.matrixNum() as i32;
    ObjAffineSet(&raw mut src, &raw mut matrix as *mut c_void, 1, 2);
    (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[i]
        .a = matrix.a;
    (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[i]
        .b = matrix.b;
    (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[i]
        .c = matrix.c;
    (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[i]
        .d = matrix.d;
}
unsafe fn ShouldRotScaleSpeciesBeFlipped() -> u8 {
    if IsContest() != 0 {
        if gSprites[GetAnimBattlerSpriteId(ANIM_ATTACKER)].data[2] == SPECIES_UNOWN as i16 {
            return FALSE;
        } else {
            return TRUE;
        }
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn PrepareBattlerSpriteForRotScale(spriteId: u8, objMode: u8) {
    let battler: u8 = gSprites[spriteId].data[0] as u8;
    if IsContest() != 0 || IsBattlerSpriteVisible(battler) != 0 {
        gSprites[spriteId].set_invisible(FALSE as u16);
    }
    gSprites[spriteId].oam.set_objMode(objMode as u32);
    gSprites[spriteId].set_affineAnimPaused(TRUE);
    if IsContest() == 0 && gSprites[spriteId].oam.affineMode() == 0 {
        gSprites[spriteId].oam.set_matrixNum(
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).matrixNum as u32,
        );
    }
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    CalcCenterToCornerVec(
        &raw mut gSprites[spriteId],
        gSprites[spriteId].oam.shape() as u8,
        gSprites[spriteId].oam.size() as u8,
        gSprites[spriteId].oam.affineMode() as u8,
    );
}
pub unsafe fn ResetSpriteRotScale(spriteId: u8) {
    SetSpriteRotScale(spriteId, 0x100, 0x100, 0);
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_NORMAL as u32);
    gSprites[spriteId].set_affineAnimPaused(FALSE);
    CalcCenterToCornerVec(
        &raw mut gSprites[spriteId],
        gSprites[spriteId].oam.shape() as u8,
        gSprites[spriteId].oam.size() as u8,
        gSprites[spriteId].oam.affineMode() as u8,
    );
}
pub unsafe fn SetBattlerSpriteYOffsetFromRotation(spriteId: u8) {
    let matrixNum: u16 = gSprites[spriteId].oam.matrixNum() as u16;
    let mut c: i16 = (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[matrixNum]
        .c;
    if c < 0 {
        c = -c;
    }
    gSprites[spriteId].y2 = c >> 3;
}
pub unsafe fn TrySetSpriteRotScale(
    sprite: *mut Sprite,
    recalcCenterVector: u8,
    xScale: i16,
    yScale: i16,
    rotation: u16,
) {
    let mut i: i32 = 0;
    let mut src: ObjAffineSrcData = zeroed();
    let mut matrix: OamMatrix = zeroed();
    if (*sprite).oam.affineMode() & 1 != 0 {
        (*sprite).set_affineAnimPaused(TRUE);
        if recalcCenterVector != 0 {
            CalcCenterToCornerVec(
                sprite,
                (*sprite).oam.shape() as u8,
                (*sprite).oam.size() as u8,
                (*sprite).oam.affineMode() as u8,
            );
        }
        src.xScale = xScale;
        src.yScale = yScale;
        src.rotation = rotation;
        if ShouldRotScaleSpeciesBeFlipped() != 0 {
            src.xScale = -src.xScale;
        }
        i = (*sprite).oam.matrixNum() as i32;
        ObjAffineSet(&raw mut src, &raw mut matrix as *mut c_void, 1, 2);
        (*(&raw const crate::sprite::gOamMatrices)
            .cast::<CArray<OamMatrix, 32>>()
            .cast_mut())[i]
            .a = matrix.a;
        (*(&raw const crate::sprite::gOamMatrices)
            .cast::<CArray<OamMatrix, 32>>()
            .cast_mut())[i]
            .b = matrix.b;
        (*(&raw const crate::sprite::gOamMatrices)
            .cast::<CArray<OamMatrix, 32>>()
            .cast_mut())[i]
            .c = matrix.c;
        (*(&raw const crate::sprite::gOamMatrices)
            .cast::<CArray<OamMatrix, 32>>()
            .cast_mut())[i]
            .d = matrix.d;
    }
}
pub unsafe fn ResetSpriteRotScale_PreserveAffine(sprite: *mut Sprite) {
    TrySetSpriteRotScale(sprite, TRUE, 0x100, 0x100, 0);
    (*sprite).set_affineAnimPaused(FALSE);
    CalcCenterToCornerVec(
        sprite,
        (*sprite).oam.shape() as u8,
        (*sprite).oam.size() as u8,
        (*sprite).oam.affineMode() as u8,
    );
}
unsafe fn ArcTan2_(x: i16, y: i16) -> u16 {
    ArcTan2(x, y)
}
pub unsafe fn ArcTan2Neg(x: i16, y: i16) -> u16 {
    let var: u16 = ArcTan2_(x, y);
    var.wrapping_neg()
}
pub unsafe fn SetGrayscaleOrOriginalPalette(paletteNum: u16, restoreOriginalColor: u8) {
    let mut originalColor: *mut PlttData = null_mut();
    let mut destColor: *mut PlttData = null_mut();
    let mut average: u16 = 0;
    let paletteOffset: u16 = paletteNum * 16;
    if restoreOriginalColor == 0 {
        for i in 0..16i32 {
            originalColor = &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset as i32 + i] as *mut PlttData;
            average = (*originalColor).r() + (*originalColor).g() + (*originalColor).b();
            average = (average as i32 / 3) as u16;
            destColor = &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset as i32 + i] as *mut PlttData;
            (*destColor).set_r(average);
            (*destColor).set_g(average);
            (*destColor).set_b(average);
        }
    } else {
        CpuSet(
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset] as *mut c_void,
            &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset] as *mut c_void,
            0x4000008,
        );
    }
}
pub unsafe fn GetBattlePalettesMask(
    battleBackground: u8,
    attacker: u8,
    target: u8,
    attackerPartner: u8,
    targetPartner: u8,
    anim1: u8,
    anim2: u8,
) -> u32 {
    let mut selectedPalettes: u32 = 0;
    let mut shift: u32 = 0;
    if battleBackground != 0 {
        if IsContest() == 0 {
            selectedPalettes = 0xe;
        } else {
            selectedPalettes = shl_i32(1, GetBattleBgPaletteNum() as u32) as u32;
        }
    }
    if attacker != 0 {
        shift = gBattleAnimAttacker as u32 + 16;
        selectedPalettes |= shl_i32(1, shift) as u32;
    }
    if target != 0 {
        shift = gBattleAnimTarget as u32 + 16;
        selectedPalettes |= shl_i32(1, shift) as u32;
    }
    if attackerPartner != 0 && IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) != 0 {
        shift = (gBattleAnimAttacker as u32 ^ 2) + 16;
        selectedPalettes |= shl_i32(1, shift) as u32;
    }
    if targetPartner != 0 && IsBattlerSpriteVisible(gBattleAnimTarget ^ 2) != 0 {
        shift = (gBattleAnimTarget as u32 ^ 2) + 16;
        selectedPalettes |= shl_i32(1, shift) as u32;
    }
    if anim1 != 0 {
        if IsContest() == 0 {
            selectedPalettes |= 256;
        } else {
            selectedPalettes |= 16384;
        }
    }
    if anim2 != 0 && IsContest() == 0 {
        selectedPalettes |= 512;
    }
    selectedPalettes
}
pub unsafe fn GetBattleMonSpritePalettesMask(
    playerLeft: u8,
    playerRight: u8,
    opponentLeft: u8,
    opponentRight: u8,
) -> u32 {
    let mut selectedPalettes: u32 = 0;
    let mut shift: u32 = 0;
    if IsContest() != 0 {
        if playerLeft != 0 {
            selectedPalettes |= 0x40000;
            return selectedPalettes;
        }
    } else {
        if playerLeft != 0
            && IsBattlerSpriteVisible(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)) != 0
        {
            selectedPalettes |=
                shl_i32(1, GetBattlerAtPosition(B_POSITION_PLAYER_LEFT) as u32 + 16) as u32;
        }
        if playerRight != 0
            && IsBattlerSpriteVisible(GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT)) != 0
        {
            shift = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT) as u32 + 16;
            selectedPalettes |= shl_i32(1, shift) as u32;
        }
        if opponentLeft != 0
            && IsBattlerSpriteVisible(GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT)) != 0
        {
            shift = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT) as u32 + 16;
            selectedPalettes |= shl_i32(1, shift) as u32;
        }
        if opponentRight != 0
            && IsBattlerSpriteVisible(GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT)) != 0
        {
            shift = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT) as u32 + 16;
            selectedPalettes |= shl_i32(1, shift) as u32;
        }
    }
    selectedPalettes
}
pub fn GetSpritePalIdxByBattler(battler: u8) -> u8 {
    battler
}
unsafe fn GetSpritePalIdxByPosition(position: u8) -> u8 {
    GetBattlerAtPosition(position)
}
pub unsafe fn AnimSpriteOnMonPos(sprite: *mut Sprite) {
    let mut respectMonPicOffsets: u8 = 0;
    if (*sprite).data[0] == 0 {
        if gBattleAnimArgs[3] == 0 {
            respectMonPicOffsets = TRUE;
        } else {
            respectMonPicOffsets = FALSE;
        }
        if gBattleAnimArgs[2] == 0 {
            InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
        } else {
            InitSpritePosToAnimTarget(sprite, respectMonPicOffsets);
        }
        (*sprite).data[0] += 1;
    } else if (*sprite).animEnded() != 0 || (*sprite).affineAnimEnded() != 0 {
        DestroySpriteAndMatrix(sprite);
    }
}
pub unsafe fn TranslateAnimSpriteToTargetMonLocation(sprite: *mut Sprite) {
    let mut respectMonPicOffsets: u8 = 0;
    let mut coordType: u8 = 0;
    if gBattleAnimArgs[5] as i32 & 0xff00 == 0 {
        respectMonPicOffsets = TRUE;
    } else {
        respectMonPicOffsets = FALSE;
    }
    if gBattleAnimArgs[5] as i32 & 0xff == 0 {
        coordType = BATTLER_COORD_Y_PIC_OFFSET;
    } else {
        coordType = BATTLER_COORD_Y;
    }
    InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[2];
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimTarget, coordType) as i16 + gBattleAnimArgs[3];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub unsafe fn AnimThrowProjectile(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[2];
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + gBattleAnimArgs[3];
    (*sprite).data[5] = gBattleAnimArgs[5];
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimThrowProjectile_Step);
}
pub(crate) unsafe fn AnimThrowProjectile_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub unsafe fn AnimTravelDiagonally(sprite: *mut Sprite) {
    let mut respectMonPicOffsets: u8 = 0;
    let mut battler: u8 = 0;
    let mut coordType: u8 = 0;
    if gBattleAnimArgs[6] == 0 {
        respectMonPicOffsets = TRUE;
        coordType = BATTLER_COORD_Y_PIC_OFFSET;
    } else {
        respectMonPicOffsets = FALSE;
        coordType = BATTLER_COORD_Y;
    }
    if gBattleAnimArgs[5] == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
        battler = gBattleAnimAttacker;
    } else {
        InitSpritePosToAnimTarget(sprite, respectMonPicOffsets);
        battler = gBattleAnimTarget;
    }
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    InitSpritePosToAnimTarget(sprite, respectMonPicOffsets);
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] =
        GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[2];
    (*sprite).data[4] = GetBattlerSpriteCoord(battler, coordType) as i16 + gBattleAnimArgs[3];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub unsafe fn CloneBattlerSpriteWithBlend(animBattler: u8) -> i16 {
    let spriteId: u8 = GetAnimBattlerSpriteId(animBattler);
    if spriteId != SPRITE_NONE {
        for i in 0..(MAX_SPRITES as u16) {
            if gSprites[i].inUse() == 0 {
                gSprites[i] = gSprites[spriteId];
                gSprites[i].oam.set_objMode(ST_OAM_OBJ_BLEND);
                gSprites[i].set_invisible(FALSE as u16);
                return i as i16;
            }
        }
    }
    -1
}
pub unsafe fn DestroySpriteWithActiveSheet(sprite: *mut Sprite) {
    (*sprite).set_usingSheet(TRUE as u16);
    DestroySprite(sprite);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_AlphaFadeIn(taskId: u8) {
    let mut v1: i16 = 0;
    let mut v2: i16 = 0;
    if gBattleAnimArgs[2] > gBattleAnimArgs[0] {
        v2 = 1;
    }
    if gBattleAnimArgs[2] < gBattleAnimArgs[0] {
        v2 = -1;
    }
    if gBattleAnimArgs[3] > gBattleAnimArgs[1] {
        v1 = 1;
    }
    if gBattleAnimArgs[3] < gBattleAnimArgs[1] {
        v1 = -1;
    }
    task_set(taskId, 0, 0);
    task_set(taskId, 1, gBattleAnimArgs[4]);
    task_set(taskId, 2, 0);
    task_set(taskId, 3, gBattleAnimArgs[0]);
    task_set(taskId, 4, gBattleAnimArgs[1]);
    task_set(taskId, 5, v2);
    task_set(taskId, 6, v1);
    task_set(taskId, 7, gBattleAnimArgs[2]);
    task_set(taskId, 8, gBattleAnimArgs[3]);
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (gBattleAnimArgs[1] as u16) << 8 | gBattleAnimArgs[0] as u16,
    );
    task_set_func(taskId, Some(AnimTask_AlphaFadeIn_Step));
}
pub(crate) unsafe fn AnimTask_AlphaFadeIn_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if ({
        (*task).data[0] += 1;
        (*task).data[0]
    }) > (*task).data[1]
    {
        (*task).data[0] = 0;
        if ({
            (*task).data[2] += 1;
            (*task).data[2]
        }) as i32
            & 1
            != 0
        {
            if (*task).data[3] != (*task).data[7] {
                (*task).data[3] += (*task).data[5];
            }
        } else {
            if (*task).data[4] != (*task).data[8] {
                (*task).data[4] += (*task).data[6];
            }
        }
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            ((*task).data[4] as u16) << 8 | (*task).data[3] as u16,
        );
        if (*task).data[3] == (*task).data[7] && (*task).data[4] == (*task).data[8] {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendMonInAndOut(task: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
    if spriteId == SPRITE_NONE {
        DestroyAnimVisualTask(task);
        return;
    }
    task_set(
        task,
        0,
        0x100 + gSprites[spriteId].oam.paletteNum() as i16 * 16 + 1,
    );
    AnimTask_BlendPalInAndOutSetup(&raw mut (*gTasks.as_ptr())[task]);
}
unsafe fn AnimTask_BlendPalInAndOutSetup(task: *mut Task) {
    (*task).data[1] = gBattleAnimArgs[1];
    (*task).data[2] = 0;
    (*task).data[3] = gBattleAnimArgs[2];
    (*task).data[4] = 0;
    (*task).data[5] = gBattleAnimArgs[3];
    (*task).data[6] = 0;
    (*task).data[7] = gBattleAnimArgs[4];
    (*task).func = Some(AnimTask_BlendMonInAndOut_Step);
}
pub(crate) unsafe fn AnimTask_BlendMonInAndOut_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if ({
        (*task).data[4] += 1;
        (*task).data[4]
    }) >= (*task).data[5]
    {
        (*task).data[4] = 0;
        if (*task).data[6] == 0 {
            (*task).data[2] += 1;
            BlendPalette(
                (*task).data[0] as u16,
                15,
                (*task).data[2] as u8,
                (*task).data[1] as u16,
            );
            if (*task).data[2] == (*task).data[3] {
                (*task).data[6] = 1;
            }
        } else {
            (*task).data[2] -= 1;
            BlendPalette(
                (*task).data[0] as u16,
                15,
                (*task).data[2] as u8,
                (*task).data[1] as u16,
            );
            if (*task).data[2] == 0 {
                if ({
                    (*task).data[7] -= 1;
                    (*task).data[7]
                }) != 0
                {
                    (*task).data[4] = 0;
                    (*task).data[6] = 0;
                } else {
                    DestroyAnimVisualTask(taskId);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendPalInAndOutByTag(task: u8) {
    let palette: u8 = IndexOfSpritePaletteTag(gBattleAnimArgs[0] as u16);
    if palette == 0xff {
        DestroyAnimVisualTask(task);
        return;
    }
    task_set(task, 0, palette as i16 * 0x10 + 0x101);
    AnimTask_BlendPalInAndOutSetup(&raw mut (*gTasks.as_ptr())[task]);
}
pub unsafe fn PrepareAffineAnimInTaskData(
    task: *mut Task,
    spriteId: u8,
    affineAnimCmds: *mut AffineAnimCmd,
) {
    (*task).data[7] = 0;
    (*task).data[8] = 0;
    (*task).data[9] = 0;
    (*task).data[15] = spriteId as i16;
    (*task).data[10] = 0x100;
    (*task).data[11] = 0x100;
    (*task).data[12] = 0;
    StorePointerInVars(
        &raw mut (*task).data[13],
        &raw mut (*task).data[14],
        affineAnimCmds as *mut c_void,
    );
    PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_NORMAL);
}
pub unsafe fn RunAffineAnimFromTaskData(task: *mut Task) -> u8 {
    sAnimTaskAffineAnim = (LoadPointerFromVars((*task).data[13], (*task).data[14])
        as *mut AffineAnimCmd)
        .at((*task).data[7]);
    'l1: {
        match (*sAnimTaskAffineAnim).r#type {
            AFFINEANIMCMDTYPE_JUMP => {
                (*task).data[7] = (*sAnimTaskAffineAnim).jump.target as i16;
            }
            AFFINEANIMCMDTYPE_LOOP => {
                if (*sAnimTaskAffineAnim).r#loop.count != 0 {
                    if (*task).data[9] != 0 {
                        if ({
                            (*task).data[9] -= 1;
                            (*task).data[9]
                        }) == 0
                        {
                            (*task).data[7] += 1;
                            break 'l1;
                        }
                    } else {
                        (*task).data[9] = (*sAnimTaskAffineAnim).r#loop.count;
                    }
                    if (*task).data[7] == 0 {
                        break 'l1;
                    }
                    loop {
                        (*task).data[7] -= 1;
                        sAnimTaskAffineAnim = sAnimTaskAffineAnim.at(-1);
                        if (*sAnimTaskAffineAnim).r#type == AFFINEANIMCMDTYPE_LOOP {
                            (*task).data[7] += 1;
                            return TRUE;
                        }
                        if (*task).data[7] == 0 {
                            return TRUE;
                        }
                    }
                }
                (*task).data[7] += 1;
            }
            AFFINEANIMCMDTYPE_END => {
                gSprites[(*task).data[15]].y2 = 0;
                ResetSpriteRotScale((*task).data[15] as u8);
                return FALSE;
            }
            _ => {
                if (*sAnimTaskAffineAnim).frame.duration == 0 {
                    (*task).data[10] = (*sAnimTaskAffineAnim).frame.xScale;
                    (*task).data[11] = (*sAnimTaskAffineAnim).frame.yScale;
                    (*task).data[12] = (*sAnimTaskAffineAnim).frame.rotation as i16;
                    (*task).data[7] += 1;
                    sAnimTaskAffineAnim = sAnimTaskAffineAnim.at(1);
                }
                (*task).data[10] += (*sAnimTaskAffineAnim).frame.xScale;
                (*task).data[11] += (*sAnimTaskAffineAnim).frame.yScale;
                (*task).data[12] += (*sAnimTaskAffineAnim).frame.rotation as i16;
                SetSpriteRotScale(
                    (*task).data[15] as u8,
                    (*task).data[10],
                    (*task).data[11],
                    (*task).data[12] as u16,
                );
                SetBattlerSpriteYOffsetFromYScale((*task).data[15] as u8);
                if ({
                    (*task).data[8] += 1;
                    (*task).data[8]
                }) >= (*sAnimTaskAffineAnim).frame.duration as i16
                {
                    (*task).data[8] = 0;
                    (*task).data[7] += 1;
                }
            }
        }
    }
    TRUE
}
pub unsafe fn SetBattlerSpriteYOffsetFromYScale(spriteId: u8) {
    let var: i32 = MON_PIC_HEIGHT - GetBattlerYDeltaFromSpriteId(spriteId) as i32 * 2;
    let matrix: u16 = gSprites[spriteId].oam.matrixNum() as u16;
    let mut var2: i32 = if (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[matrix]
        .d
        != 0
    {
        div_i32(
            var << 8,
            (*(&raw const crate::sprite::gOamMatrices)
                .cast::<CArray<OamMatrix, 32>>()
                .cast_mut())[matrix]
                .d as i32,
        )
    } else {
        0
    };
    if var2 > 128 {
        var2 = 128;
    }
    gSprites[spriteId].y2 = ((var - var2) / 2) as i16;
}
pub unsafe fn SetBattlerSpriteYOffsetFromOtherYScale(spriteId: u8, otherSpriteId: u8) {
    let var: i32 = MON_PIC_HEIGHT - GetBattlerYDeltaFromSpriteId(otherSpriteId) as i32 * 2;
    let matrix: u16 = gSprites[spriteId].oam.matrixNum() as u16;
    let mut var2: i32 = if (*(&raw const crate::sprite::gOamMatrices)
        .cast::<CArray<OamMatrix, 32>>()
        .cast_mut())[matrix]
        .d
        != 0
    {
        div_i32(
            var << 8,
            (*(&raw const crate::sprite::gOamMatrices)
                .cast::<CArray<OamMatrix, 32>>()
                .cast_mut())[matrix]
                .d as i32,
        )
    } else {
        0
    };
    if var2 > 128 {
        var2 = 128;
    }
    gSprites[spriteId].y2 = ((var - var2) / 2) as i16;
}
unsafe fn GetBattlerYDeltaFromSpriteId(spriteId: u8) -> u16 {
    let mut spriteInfo: *mut BattleSpriteInfo = null_mut();
    let battler: u8 = gSprites[spriteId].data[0] as u8;
    let mut species: u16 = 0;
    for i in 0..(MAX_BATTLERS_COUNT as u16) {
        if gBattlerSpriteIds[i] == spriteId {
            if IsContest() != 0 {
                species = (*(*gContestResources).moveAnim).species;
                return (*(&raw const crate::data::data_tables::gMonBackPicCoords).cast::<CArray<
                    MonCoords,
                    0,
                >>(
                ))[species]
                    .y_offset as u16;
            } else {
                if GetBattlerSide(i as u8) == B_SIDE_PLAYER {
                    spriteInfo = (*gBattleSpritesDataPtr).battlerData;
                    if (*spriteInfo.at(battler)).transformSpecies == 0 {
                        species = GetMonData2(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[i]],
                            MON_DATA_SPECIES,
                        ) as u16;
                    } else {
                        species = (*spriteInfo.at(battler)).transformSpecies;
                    }
                    if species == SPECIES_CASTFORM {
                        return sCastformBackSpriteYCoords[gBattleMonForms[battler]] as u16;
                    } else {
                        return (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                            .cast::<CArray<MonCoords, 0>>())[species]
                            .y_offset as u16;
                    }
                } else {
                    spriteInfo = (*gBattleSpritesDataPtr).battlerData;
                    if (*spriteInfo.at(battler)).transformSpecies == 0 {
                        species = GetMonData2(
                            &raw mut gEnemyParty[gBattlerPartyIndexes[i]],
                            MON_DATA_SPECIES,
                        ) as u16;
                    } else {
                        species = (*spriteInfo.at(battler)).transformSpecies;
                    }
                    if species == SPECIES_CASTFORM {
                        return sCastformElevations[gBattleMonForms[battler]] as u16;
                    } else {
                        return (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                            .cast::<CArray<MonCoords, 0>>())[species]
                            .y_offset as u16;
                    }
                }
            }
        }
    }
    MON_PIC_HEIGHT as u16
}
pub unsafe fn StorePointerInVars(lo: *mut i16, hi: *mut i16, ptr: *mut c_void) {
    *lo = ptr as usize as i32 as i16;
    *hi = (ptr as usize as i32 >> 16) as i16;
}
pub unsafe fn LoadPointerFromVars(lo: i16, hi: i16) -> *mut c_void {
    (lo as u16 as i32 | (hi as u16 as i32) << 16) as usize as *mut c_void
}
pub unsafe fn PrepareEruptAnimTaskData(
    task: *mut Task,
    spriteId: u8,
    xScaleStart: i16,
    yScaleStart: i16,
    xScaleEnd: i16,
    yScaleEnd: i16,
    duration: u16,
) {
    (*task).data[8] = duration as i16;
    (*task).data[15] = spriteId as i16;
    (*task).data[9] = xScaleStart;
    (*task).data[10] = yScaleStart;
    (*task).data[13] = xScaleEnd;
    (*task).data[14] = yScaleEnd;
    (*task).data[11] = div_i32(xScaleEnd as i32 - xScaleStart as i32, duration as i32) as i16;
    (*task).data[12] = div_i32(yScaleEnd as i32 - yScaleStart as i32, duration as i32) as i16;
}
pub unsafe fn UpdateEruptAnimTask(task: *mut Task) -> u8 {
    if (*task).data[8] == 0 {
        return 0;
    }
    if ({
        (*task).data[8] -= 1;
        (*task).data[8]
    }) != 0
    {
        (*task).data[9] += (*task).data[11];
        (*task).data[10] += (*task).data[12];
    } else {
        (*task).data[9] = (*task).data[13];
        (*task).data[10] = (*task).data[14];
    }
    SetSpriteRotScale((*task).data[15] as u8, (*task).data[9], (*task).data[10], 0);
    if (*task).data[8] != 0 {
        SetBattlerSpriteYOffsetFromYScale((*task).data[15] as u8);
    } else {
        gSprites[(*task).data[15]].y2 = 0;
    }
    (*task).data[8] as u8
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GetFrustrationPowerLevel(taskId: u8) {
    let mut powerLevel: u16 = 0;
    if gAnimFriendship <= 30 {
        powerLevel = 0;
    } else if gAnimFriendship <= 100 {
        powerLevel = 1;
    } else if gAnimFriendship <= 200 {
        powerLevel = 2;
    } else {
        powerLevel = 3;
    }
    gBattleAnimArgs[7] = powerLevel as i16;
    DestroyAnimVisualTask(taskId);
}
unsafe fn SetPriorityForVisibleBattlers(priority: u8) {
    if IsBattlerSpriteVisible(gBattleAnimTarget) != 0 {
        gSprites[gBattlerSpriteIds[gBattleAnimTarget]]
            .oam
            .set_priority(priority as u16);
    }
    if IsBattlerSpriteVisible(gBattleAnimAttacker) != 0 {
        gSprites[gBattlerSpriteIds[gBattleAnimAttacker]]
            .oam
            .set_priority(priority as u16);
    }
    if IsBattlerSpriteVisible(gBattleAnimTarget ^ 2) != 0 {
        gSprites[gBattlerSpriteIds[gBattleAnimTarget as i32 ^ 2]]
            .oam
            .set_priority(priority as u16);
    }
    if IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) != 0 {
        gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]]
            .oam
            .set_priority(priority as u16);
    }
}
pub unsafe fn InitPrioritiesForVisibleBattlers() {
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        if IsBattlerSpriteVisible(i as u8) != 0 {
            gSprites[gBattlerSpriteIds[i]].subpriority = GetBattlerSpriteSubpriority(i as u8);
            gSprites[gBattlerSpriteIds[i]].oam.set_priority(2);
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetBattlerSpriteSubpriority(battler: u8) -> u8 {
    let mut position: u8 = 0;
    let mut subpriority: u8 = 0;
    if IsContest() != 0 {
        if battler == 2 {
            return 30;
        } else {
            return 40;
        }
    } else {
        position = GetBattlerPosition(battler);
        if position == B_POSITION_PLAYER_LEFT {
            subpriority = 30;
        } else if position == B_POSITION_PLAYER_RIGHT {
            subpriority = 20;
        } else if position == B_POSITION_OPPONENT_LEFT {
            subpriority = 40;
        } else {
            subpriority = 50;
        }
    }
    subpriority
}
pub unsafe fn GetBattlerSpriteBGPriority(battler: u8) -> u8 {
    let position: u8 = GetBattlerPosition(battler);
    if IsContest() != 0 {
        return 2;
    } else if position == B_POSITION_PLAYER_LEFT || position == B_POSITION_OPPONENT_RIGHT {
        return GetAnimBgAttribute(2, BG_ANIM_PRIORITY) as u8;
    } else {
        return GetAnimBgAttribute(1, BG_ANIM_PRIORITY) as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetBattlerSpriteBGPriorityRank(battler: u8) -> u8 {
    if IsContest() == 0 {
        let position: u8 = GetBattlerPosition(battler);
        if position == B_POSITION_PLAYER_LEFT || position == B_POSITION_OPPONENT_RIGHT {
            return 2;
        } else {
            return 1;
        }
    }
    1
}
pub unsafe fn CreateAdditionalMonSpriteForMoveAnim(
    species: u16,
    isBackpic: u8,
    id: u8,
    x: i16,
    y: i16,
    subpriority: u8,
    personality: u32,
    trainerId: u32,
    battler: u32,
    ignoreDeoxysForm: u32,
) -> u8 {
    let mut spriteId: u8 = 0;
    let sheet: u16 = LoadSpriteSheet((&raw const sSpriteSheets_MoveEffectMons[id]).cast_mut());
    let palette: u16 = AllocSpritePalette(sSpriteTemplates_MoveEffectMons[id].paletteTag) as u16;
    if !gMonSpritesGfxPtr.is_null() && (*gMonSpritesGfxPtr).buffer.is_null() {
        (*gMonSpritesGfxPtr).buffer = AllocZeroed(8192) as *mut u16;
    }
    if isBackpic == 0 {
        LoadCompressedPalette(
            GetMonSpritePalFromSpeciesAndPersonality(species, trainerId, personality),
            0x100 + palette * 16,
            32,
        );
        if ignoreDeoxysForm == TRUE as u32
            || ShouldIgnoreDeoxysForm(5, battler as u8) == TRUE
            || (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != 0
        {
            LoadSpecialPokePic_DontHandleDeoxys(
                (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                    .cast_mut(),
                (*gMonSpritesGfxPtr).buffer as *mut c_void,
                species as i32,
                personality,
                TRUE,
            );
        } else {
            LoadSpecialPokePic_2(
                (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                    .cast_mut(),
                (*gMonSpritesGfxPtr).buffer as *mut c_void,
                species as i32,
                personality,
                TRUE,
            );
        }
    } else {
        LoadCompressedPalette(
            GetMonSpritePalFromSpeciesAndPersonality(species, trainerId, personality),
            0x100 + palette * 16,
            32,
        );
        if ignoreDeoxysForm == TRUE as u32
            || ShouldIgnoreDeoxysForm(5, battler as u8) == TRUE
            || (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != 0
        {
            LoadSpecialPokePic_DontHandleDeoxys(
                (&raw const (*(&raw const crate::data::data_tables::gMonBackPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                    .cast_mut(),
                (*gMonSpritesGfxPtr).buffer as *mut c_void,
                species as i32,
                personality,
                FALSE,
            );
        } else {
            LoadSpecialPokePic_2(
                (&raw const (*(&raw const crate::data::data_tables::gMonBackPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                    .cast_mut(),
                (*gMonSpritesGfxPtr).buffer as *mut c_void,
                species as i32,
                personality,
                FALSE,
            );
        }
    }
    RequestDma3Copy(
        (*gMonSpritesGfxPtr).buffer as *mut c_void,
        (OBJ_VRAM0 + sheet as i32 * 0x20) as usize as *mut c_void,
        MON_PIC_SIZE,
        1,
    );
    Free((*gMonSpritesGfxPtr).buffer as *mut c_void);
    (*gMonSpritesGfxPtr).buffer = null_mut();
    if isBackpic == 0 {
        spriteId = CreateSprite(
            (&raw const sSpriteTemplates_MoveEffectMons[id]).cast_mut(),
            x,
            y + (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                .cast::<CArray<MonCoords, 0>>())[species]
                .y_offset as i16,
            subpriority,
        );
    } else {
        spriteId = CreateSprite(
            (&raw const sSpriteTemplates_MoveEffectMons[id]).cast_mut(),
            x,
            y + (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[species]
                .y_offset as i16,
            subpriority,
        );
    }
    if IsContest() != 0 {
        gSprites[spriteId].affineAnims =
            (*(&raw const crate::data::data_tables::gAffineAnims_BattleSpriteContest)
                .cast::<CArray<*mut AffineAnimCmd, 0>>())
            .as_ptr()
            .cast_mut();
        StartSpriteAffineAnim(&raw mut gSprites[spriteId], BATTLER_AFFINE_NORMAL);
    }
    spriteId
}
pub unsafe fn DestroySpriteAndFreeResources_(sprite: *mut Sprite) {
    DestroySpriteAndFreeResources(sprite);
}
pub unsafe fn GetBattlerSpriteCoordAttr(battler: u8, attr: u8) -> i16 {
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    let mut letter: u16 = 0;
    let mut unownSpecies: u16 = 0;
    let mut ret: i32 = 0;
    let mut coords: *mut MonCoords = null_mut();
    let mut spriteInfo: *mut BattleSpriteInfo = null_mut();
    if IsContest() != 0 {
        if (*(*gContestResources).moveAnim).hasTargetAnim() != 0 {
            species = (*(*gContestResources).moveAnim).targetSpecies;
            personality = (*(*gContestResources).moveAnim).targetPersonality;
        } else {
            species = (*(*gContestResources).moveAnim).species;
            personality = (*(*gContestResources).moveAnim).personality;
        }
        if species == SPECIES_UNOWN {
            letter = (((personality & 0x03000000) >> 18
                | (personality & 0x00030000) >> 12
                | (personality & 0x00000300) >> 6
                | (personality & 0x00000003))
                % 28) as u16;
            if letter == 0 {
                unownSpecies = SPECIES_UNOWN;
            } else {
                unownSpecies = letter + SPECIES_UNOWN_B - 1;
            }
            coords = (&raw const (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[unownSpecies])
                .cast_mut();
        } else if species == SPECIES_CASTFORM {
            coords = (&raw const gCastformFrontSpriteCoords[gBattleMonForms[battler]]).cast_mut();
        } else if species <= SPECIES_EGG as u16 {
            coords = (&raw const (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[species])
                .cast_mut();
        } else {
            coords = (&raw const (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[0])
                .cast_mut();
        }
    } else {
        if GetBattlerSide(battler) == B_SIDE_PLAYER {
            spriteInfo = (*gBattleSpritesDataPtr).battlerData;
            if (*spriteInfo.at(battler)).transformSpecies == 0 {
                species = GetMonData2(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                    MON_DATA_SPECIES,
                ) as u16;
                personality = GetMonData2(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                    MON_DATA_PERSONALITY,
                );
            } else {
                species = (*spriteInfo.at(battler)).transformSpecies;
                personality = gTransformedPersonalities[battler];
            }
            if species == SPECIES_UNOWN {
                letter = (((personality & 0x03000000) >> 18
                    | (personality & 0x00030000) >> 12
                    | (personality & 0x00000300) >> 6
                    | (personality & 0x00000003))
                    % 28) as u16;
                if letter == 0 {
                    unownSpecies = SPECIES_UNOWN;
                } else {
                    unownSpecies = letter + SPECIES_UNOWN_B - 1;
                }
                coords = (&raw const (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[unownSpecies])
                    .cast_mut();
            } else if species > NUM_SPECIES {
                coords = (&raw const (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[0])
                    .cast_mut();
            } else {
                coords = (&raw const (*(&raw const crate::data::data_tables::gMonBackPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[species])
                    .cast_mut();
            }
        } else {
            spriteInfo = (*gBattleSpritesDataPtr).battlerData;
            if (*spriteInfo.at(battler)).transformSpecies == 0 {
                species = GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                    MON_DATA_SPECIES,
                ) as u16;
                personality = GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                    MON_DATA_PERSONALITY,
                );
            } else {
                species = (*spriteInfo.at(battler)).transformSpecies;
                personality = gTransformedPersonalities[battler];
            }
            if species == SPECIES_UNOWN {
                letter = (((personality & 0x03000000) >> 18
                    | (personality & 0x00030000) >> 12
                    | (personality & 0x00000300) >> 6
                    | (personality & 0x00000003))
                    % 28) as u16;
                if letter == 0 {
                    unownSpecies = SPECIES_UNOWN;
                } else {
                    unownSpecies = letter + SPECIES_UNOWN_B - 1;
                }
                coords = (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[unownSpecies])
                    .cast_mut();
            } else if species == SPECIES_CASTFORM {
                coords =
                    (&raw const gCastformFrontSpriteCoords[gBattleMonForms[battler]]).cast_mut();
            } else if species > NUM_SPECIES {
                coords = (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[0])
                    .cast_mut();
            } else {
                coords = (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[species])
                    .cast_mut();
            }
        }
    }
    match attr {
        BATTLER_COORD_ATTR_HEIGHT => {
            return ((*coords).size as i16 & 0xF) * 8;
        }
        BATTLER_COORD_ATTR_WIDTH => {
            return ((*coords).size >> 4) as i16 * 8;
        }
        BATTLER_COORD_ATTR_LEFT => {
            return GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16
                - (((*coords).size >> 4) as i32 * 8 / 2) as i16;
        }
        BATTLER_COORD_ATTR_RIGHT => {
            return GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16
                + (((*coords).size >> 4) as i32 * 8 / 2) as i16;
        }
        BATTLER_COORD_ATTR_TOP => {
            return GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16
                - (((*coords).size as i32 & 0xF) * 8 / 2) as i16;
        }
        BATTLER_COORD_ATTR_BOTTOM => {
            return GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16
                + (((*coords).size as i32 & 0xF) * 8 / 2) as i16;
        }
        BATTLER_COORD_ATTR_RAW_BOTTOM => {
            ret = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as i32 + 31;
            return ret as i16 - (*coords).y_offset as i16;
        }
        _ => {
            return 0;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn SetAverageBattlerPositions(
    battler: u8,
    respectMonPicOffsets: u8,
    x: *mut i16,
    y: *mut i16,
) {
    let mut xCoordType: u8 = 0;
    let mut yCoordType: u8 = 0;
    let mut partnerX: i16 = 0;
    let mut partnerY: i16 = 0;
    if respectMonPicOffsets == 0 {
        xCoordType = BATTLER_COORD_X;
        yCoordType = BATTLER_COORD_Y;
    } else {
        xCoordType = BATTLER_COORD_X_2;
        yCoordType = BATTLER_COORD_Y_PIC_OFFSET;
    }
    let battlerX: i16 = GetBattlerSpriteCoord(battler, xCoordType) as i16;
    let battlerY: i16 = GetBattlerSpriteCoord(battler, yCoordType) as i16;
    if IsDoubleBattle() != 0 && IsContest() == 0 {
        partnerX = GetBattlerSpriteCoord(battler ^ 2, xCoordType) as i16;
        partnerY = GetBattlerSpriteCoord(battler ^ 2, yCoordType) as i16;
    } else {
        partnerX = battlerX;
        partnerY = battlerY;
    }
    *x = ((battlerX as i32 + partnerX as i32) / 2) as i16;
    *y = ((battlerY as i32 + partnerY as i32) / 2) as i16;
}
pub unsafe fn CreateInvisibleSpriteCopy(battler: i32, spriteId: u8, species: i32) -> u8 {
    let newSpriteId: u8 = CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy));
    gSprites[newSpriteId] = gSprites[spriteId];
    gSprites[newSpriteId].set_usingSheet(TRUE as u16);
    gSprites[newSpriteId].oam.set_priority(0);
    gSprites[newSpriteId].oam.set_objMode(ST_OAM_OBJ_WINDOW);
    gSprites[newSpriteId]
        .oam
        .set_tileNum(gSprites[spriteId].oam.tileNum());
    gSprites[newSpriteId].callback = Some(SpriteCallbackDummy);
    newSpriteId
}
pub unsafe fn AnimTranslateLinearAndFlicker_Flipped(sprite: *mut Sprite) {
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        (*sprite).x -= gBattleAnimArgs[0];
        gBattleAnimArgs[3] = -gBattleAnimArgs[3];
        (*sprite).set_hFlip(TRUE as u16);
    } else {
        (*sprite).x += gBattleAnimArgs[0];
    }
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[3] = gBattleAnimArgs[4];
    (*sprite).data[5] = gBattleAnimArgs[5];
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(TranslateSpriteLinearAndFlicker);
}
pub unsafe fn AnimTranslateLinearAndFlicker(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= gBattleAnimArgs[0];
        gBattleAnimArgs[3] *= -1;
    } else {
        (*sprite).x += gBattleAnimArgs[0];
    }
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[3] = gBattleAnimArgs[4];
    (*sprite).data[5] = gBattleAnimArgs[5];
    StartSpriteAnim(sprite, gBattleAnimArgs[6] as u8);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(TranslateSpriteLinearAndFlicker);
}
pub unsafe fn AnimSpinningSparkle(sprite: *mut Sprite) {
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        (*sprite).x -= gBattleAnimArgs[0];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
    }
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_AttackerPunchWithTrace(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[tBattlerSpriteId] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*task).data[tMoveSpeed] = (if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        -8
    } else {
        8
    }) as i16;
    (*task).data[tState] = 0;
    (*task).data[tCounter] = 0;
    gSprites[(*task).data[tBattlerSpriteId]].x2 -= (*task).data[tBattlerSpriteId];
    (*task).data[tPaletteNum] = AllocSpritePalette(ANIM_TAG_BENT_SPOON) as i16;
    (*task).data[tNumTracesActive] = 0;
    let dest: u16 = ((*task).data[tPaletteNum] as u16 + 16) * 16;
    let src: u16 = (gSprites[(*task).data[tBattlerSpriteId]].oam.paletteNum() + 16) * 16;
    (*task).data[tPriority] = GetBattlerSpriteSubpriority(gBattleAnimAttacker) as i16;
    if (*task).data[tPriority] == 20 || (*task).data[tPriority] == 40 {
        (*task).data[tPriority] = 2;
    } else {
        (*task).data[tPriority] = 3;
    }
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[src] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[dest] as *mut c_void,
        0x4000008,
    );
    BlendPalette(
        dest,
        16,
        gBattleAnimArgs[1] as u8,
        gBattleAnimArgs[0] as u16,
    );
    (*task).func = Some(AnimTask_AttackerPunchWithTrace_Step);
}
pub(crate) unsafe fn AnimTask_AttackerPunchWithTrace_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            CreateBattlerTrace(task, taskId);
            gSprites[(*task).data[tBattlerSpriteId]].x2 += (*task).data[tMoveSpeed];
            if ({
                (*task).data[tCounter] += 1;
                (*task).data[tCounter]
            }) == 5
            {
                (*task).data[tCounter] -= 1;
                (*task).data[tState] += 1;
            }
        }
        1 => {
            CreateBattlerTrace(task, taskId);
            gSprites[(*task).data[tBattlerSpriteId]].x2 -= (*task).data[tMoveSpeed];
            if ({
                (*task).data[tCounter] -= 1;
                (*task).data[tCounter]
            }) == 0
            {
                gSprites[(*task).data[tBattlerSpriteId]].x2 = 0;
                (*task).data[tState] += 1;
            }
        }
        2 if (*task).data[tNumTracesActive] == 0 => {
            FreeSpritePaletteByTag(ANIM_TAG_BENT_SPOON);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
unsafe fn CreateBattlerTrace(task: *mut Task, taskId: u8) {
    let spriteId: i16 = CloneBattlerSpriteWithBlend(0);
    if spriteId >= 0 {
        gSprites[spriteId]
            .oam
            .set_priority((*task).data[tPriority] as u16);
        gSprites[spriteId]
            .oam
            .set_paletteNum((*task).data[tPaletteNum] as u16);
        gSprites[spriteId].data[0] = 8;
        gSprites[spriteId].data[sTaskId] = taskId as i16;
        gSprites[spriteId].data[sSpriteId] = spriteId;
        gSprites[spriteId].x2 = gSprites[(*task).data[0]].x2;
        gSprites[spriteId].callback = Some(AnimBattlerTrace);
        (*task).data[tNumTracesActive] += 1;
    }
}
pub(crate) unsafe fn AnimBattlerTrace(sprite: *mut Sprite) {
    if ({
        (*sprite).data[sActiveTime] -= 1;
        (*sprite).data[sActiveTime]
    }) == 0
    {
        task_set(
            (*sprite).data[sTaskId],
            tNumTracesActive,
            task_get((*sprite).data[sTaskId], tNumTracesActive) - 1,
        );
        DestroySpriteWithActiveSheet(sprite);
    }
}
pub unsafe fn AnimWeatherBallUp(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*sprite).data[0] = 5;
    } else {
        (*sprite).data[0] = -10;
    }
    (*sprite).data[1] = -40;
    (*sprite).callback = Some(AnimWeatherBallUp_Step);
}
pub(crate) unsafe fn AnimWeatherBallUp_Step(sprite: *mut Sprite) {
    (*sprite).data[2] += (*sprite).data[0];
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).x2 = (*sprite).data[2] / 10;
    (*sprite).y2 = (*sprite).data[3] / 10;
    if (*sprite).data[1] < -20 {
        (*sprite).data[1] += 1;
    }
    if ((*sprite).y as i32 + (*sprite).y2 as i32) < -32 {
        DestroyAnimSprite(sprite);
    }
}
pub unsafe fn AnimWeatherBallDown(sprite: *mut Sprite) {
    let mut x: i32 = 0;
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[2] = (*sprite).x + gBattleAnimArgs[4];
    (*sprite).data[4] = (*sprite).y + gBattleAnimArgs[5];
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        x = gBattleAnimArgs[4] as u16 as i32 + 30;
        (*sprite).x += x as i16;
        (*sprite).y = gBattleAnimArgs[5] - 20;
    } else {
        x = gBattleAnimArgs[4] as u16 as i32 - 30;
        (*sprite).x += x as i16;
        (*sprite).y = gBattleAnimArgs[5] - 80;
    }
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
