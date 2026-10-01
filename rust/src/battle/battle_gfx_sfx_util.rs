//! Translated from `src/battle_gfx_sfx_util.c` by tools/rustport/c2rs.py.
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
    clippy::bad_bit_mask,
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    dead_code,
    unused_assignments,
    unused_labels,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::battle_ai_script_commands::{BattleAI_ChooseMoveOrAction, BattleAI_SetupAIData};
use crate::battle_anim::{
    IsContest, LaunchBattleAnimation, gAnimScriptActive, gAnimScriptCallback, gBattleAnimAttacker,
    gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide, GetBattlerSpriteCoord,
    GetBattlerSpriteDefault_Y, GetSubstituteSpriteDefault_Y, IsBattlerSpritePresent,
    IsDoubleBattle,
};
use crate::battle_anim_status_effects::LaunchStatusAnimation;
use crate::battle_interface::{
    CreateBattlerHealthboxSprites, CreateSafariPlayerHealthboxSprites, DummyBattleInterfaceFunc,
    GetHPBarLevel, InitBattlerHealthboxCoords, SetHealthboxSpriteInvisible,
    UpdateHealthboxAttribute,
};
use crate::battle_main::{
    gActiveBattler, gBattleMons, gBattleSpritesDataPtr, gBattleStruct, gBattleTypeFlags,
    gBattlersCount, gIntroSlideFlags, gMonSpritesGfxPtr, gProtectStructs,
    gTransformedPersonalities,
};
use crate::battle_main::{
    gBattleMonForms, gBattlerPartyIndexes, gBattlerPositions, gBattlerSpriteIds,
    gHealthboxSpriteIds,
};
use crate::battle_util::CheckMoveLimitations;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::gContestResources;
use crate::m4a::{gMPlayInfo_SE1, gMPlayInfo_SE2, m4aMPlayStop, m4aSongNumStop};
use crate::palette::{LoadCompressedPalette, LoadPalette};
use crate::party_menu::{BufferBattlePartyCurrentOrder, GetPartyIdFromBattlePartyId};
use crate::pokemon::{
    GetMonData2, GetMonFrontSpritePal, GetMonSpritePalFromSpeciesAndPersonality,
    GetNatureFromPersonality, ShouldIgnoreDeoxysForm, gEnemyParty, gPlayerParty,
};
use crate::random::Random;
use crate::sound::{IsSEPlaying, PlaySE};
use crate::sprite::FreeSpritePaletteByTag;
use crate::sprite::gSprites;
use crate::task::DestroyTask;
use crate::task::{task_get, task_set};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
use crate::util::gBitTable;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AnimateSprite` with this module's view of its types.
#[inline]
unsafe fn AnimateSprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::AnimateSprite(a0 as _);
    }
}
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
/// `DecompressPicFromTable_2` with this module's view of its types.
#[inline]
unsafe fn DecompressPicFromTable_2(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32) {
    unsafe {
        crate::decompress::DecompressPicFromTable_2(a0 as _, a1 as _, a2);
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `HandleLoadSpecialPokePic` with this module's view of its types.
#[inline]
unsafe fn HandleLoadSpecialPokePic(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic(a0 as _, a1 as _, a2, a3);
    }
}
/// `HandleLoadSpecialPokePic_DontHandleDeoxys` with this module's view of its types.
#[inline]
unsafe fn HandleLoadSpecialPokePic_DontHandleDeoxys(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_DontHandleDeoxys(a0 as _, a1 as _, a2, a3);
    }
}
/// `LZDecompressVram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressVram(a0 as _, a1 as _);
    }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
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
const sSpeedX: usize = 0;
const tBattlerId: usize = 0;
// Data tables (translate with cdata.py): sSpriteSheet_SinglesPlayerHealthbox sSpriteSheet_SinglesOpponentHealthbox sSpriteSheets_DoublesPlayerHealthbox sSpriteSheets_DoublesOpponentHealthbox sSpriteSheet_SafariHealthbox sSpriteSheets_HealthBar sSpritePalettes_HealthBoxHealthBar

static sSpritePalettes_HealthBoxHealthBar: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::battle_gfx_sfx_util::sSpritePalettes_HealthBoxHealthBar).cast());
static sSpriteSheet_SafariHealthbox: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::battle_gfx_sfx_util::sSpriteSheet_SafariHealthbox).cast());
static sSpriteSheet_SinglesOpponentHealthbox: Table<CompressedSpriteSheet> = Table(
    (&raw const crate::data::battle_gfx_sfx_util::sSpriteSheet_SinglesOpponentHealthbox).cast(),
);
static sSpriteSheet_SinglesPlayerHealthbox: Table<CompressedSpriteSheet> = Table(
    (&raw const crate::data::battle_gfx_sfx_util::sSpriteSheet_SinglesPlayerHealthbox).cast(),
);
static sSpriteSheets_DoublesOpponentHealthbox: Table<CArray<CompressedSpriteSheet, 2>> = Table(
    (&raw const crate::data::battle_gfx_sfx_util::sSpriteSheets_DoublesOpponentHealthbox).cast(),
);
static sSpriteSheets_DoublesPlayerHealthbox: Table<CArray<CompressedSpriteSheet, 2>> = Table(
    (&raw const crate::data::battle_gfx_sfx_util::sSpriteSheets_DoublesPlayerHealthbox).cast(),
);
static sSpriteSheets_HealthBar: Table<CArray<CompressedSpriteSheet, 4>> =
    Table((&raw const crate::data::battle_gfx_sfx_util::sSpriteSheets_HealthBar).cast());

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

pub unsafe fn AllocateBattleSpritesData() {
    gBattleSpritesDataPtr = AllocZeroed(16) as *mut BattleSpriteData;
    (*gBattleSpritesDataPtr).battlerData = AllocZeroed(16) as *mut BattleSpriteInfo;
    (*gBattleSpritesDataPtr).healthBoxesData = AllocZeroed(48) as *mut BattleHealthboxInfo;
    (*gBattleSpritesDataPtr).animationData = AllocZeroed(16) as *mut BattleAnimationInfo;
    (*gBattleSpritesDataPtr).battleBars = AllocZeroed(80) as *mut BattleBarInfo;
}
pub unsafe fn FreeBattleSpritesData() {
    if gBattleSpritesDataPtr.is_null() {
        return;
    }
    Free((*gBattleSpritesDataPtr).battleBars as *mut c_void);
    (*gBattleSpritesDataPtr).battleBars = null_mut();
    Free((*gBattleSpritesDataPtr).animationData as *mut c_void);
    (*gBattleSpritesDataPtr).animationData = null_mut();
    Free((*gBattleSpritesDataPtr).healthBoxesData as *mut c_void);
    (*gBattleSpritesDataPtr).healthBoxesData = null_mut();
    Free((*gBattleSpritesDataPtr).battlerData as *mut c_void);
    (*gBattleSpritesDataPtr).battlerData = null_mut();
    Free(gBattleSpritesDataPtr as *mut c_void);
    gBattleSpritesDataPtr = null_mut();
}
pub unsafe fn ChooseMoveAndTargetInBattlePalace() -> u16 {
    let mut chosenMoveId: i32 = -1;
    let moveInfo: *mut ChooseMoveStruct =
        &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler][4] as *mut ChooseMoveStruct;
    let unusableMovesBits: u8 = CheckMoveLimitations(gActiveBattler, 0, MOVE_LIMITATIONS_ALL);
    let mut percent: i32 = Random() as i32 % 100;
    let mut i: i32 = if (*gBattleStruct).palaceFlags as u32
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler]
        != 0
    {
        2
    } else {
        0
    };
    let mut var2: i32 = i;
    let mut var1: i32 = i + 2;
    while i < var1 {
        if (*(&raw const crate::data::battle_script_commands::gBattlePalaceNatureToMoveGroupLikelihood).cast::<CArray<CArray<u8, 4>, 25>>())
            [GetNatureFromPersonality(gBattleMons[gActiveBattler].personality)][i] as i32
            > percent
        {
            break;
        }
        i += 1;
    }
    percent = i - var2;
    if i == var1 {
        percent = PALACE_MOVE_GROUP_SUPPORT as i32;
    }
    var2 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if (*moveInfo).moves[i] == MOVE_NONE {
            break;
        }
        if percent == GetBattlePalaceMoveGroup((*moveInfo).moves[i]) as i32
            && (*moveInfo).currentPP[i] != 0
        {
            var2 |= gBitTable[i] as i32;
        }
        i += 1;
    }
    if var2 != 0 {
        (*gBattleStruct).palaceFlags &= 15;
        (*gBattleStruct).palaceFlags |= (var2 as u8) << 4;
        BattleAI_SetupAIData(var2 as u8);
        chosenMoveId = BattleAI_ChooseMoveOrAction() as i32;
    }
    if chosenMoveId == -1 {
        if unusableMovesBits != ALL_MOVES_MASK {
            var1 = 0;
            var2 = 0;
            i = 0;
            while i < MAX_MON_MOVES {
                if GetBattlePalaceMoveGroup((*moveInfo).moves[i]) == PALACE_MOVE_GROUP_ATTACK
                    && (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        & unusableMovesBits as u32
                        == 0
                {
                    var1 += 1;
                }
                if GetBattlePalaceMoveGroup((*moveInfo).moves[i]) == PALACE_MOVE_GROUP_DEFENSE
                    && (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        & unusableMovesBits as u32
                        == 0
                {
                    var1 += 16;
                }
                if GetBattlePalaceMoveGroup((*moveInfo).moves[i]) == PALACE_MOVE_GROUP_SUPPORT
                    && (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        & unusableMovesBits as u32
                        == 0
                {
                    var1 += 256;
                }
                i += 1;
            }
            if var1 & 0xF >= 2 {
                var2 += 1;
            }
            if var1 & 240 >= 32 {
                var2 += 1;
            }
            if var1 & 240 >= 512 {
                var2 += 1;
            }
            if var2 > 1 || var2 == 0 {
                loop {
                    i = Random() as i32 % 4;
                    if gBitTable[i] & unusableMovesBits as u32 == 0 {
                        chosenMoveId = i;
                    }
                    if chosenMoveId != -1 {
                        break;
                    }
                }
            } else {
                if var1 & 0xF >= 2 {
                    var2 = PALACE_MOVE_GROUP_ATTACK as i32;
                }
                if var1 & 240 >= 32 {
                    var2 = PALACE_MOVE_GROUP_DEFENSE as i32;
                }
                if var1 & 240 >= 512 {
                    var2 = PALACE_MOVE_GROUP_SUPPORT as i32;
                }
                loop {
                    i = Random() as i32 % 4;
                    if gBitTable[i] & unusableMovesBits as u32 == 0
                        && var2 == GetBattlePalaceMoveGroup((*moveInfo).moves[i]) as i32
                    {
                        chosenMoveId = i;
                    }
                    if chosenMoveId != -1 {
                        break;
                    }
                }
            }
            if Random() as i32 % 100 >= 50 {
                gProtectStructs[gActiveBattler].set_palaceUnableToUseMove(TRUE as u32);
                return 0;
            }
        } else {
            gProtectStructs[gActiveBattler].set_palaceUnableToUseMove(TRUE as u32);
            return 0;
        }
    }
    if (*moveInfo).moves[chosenMoveId] == MOVE_CURSE {
        if (*moveInfo).monTypes[0] != TYPE_GHOST && (*moveInfo).monTypes[1] != TYPE_GHOST {
            var1 = MOVE_TARGET_USER as i32;
        } else {
            var1 = MOVE_TARGET_SELECTED as i32;
        }
    } else {
        var1 = (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [(*moveInfo).moves[chosenMoveId]]
            .target as i32;
    }
    if var1 & MOVE_TARGET_USER as i32 != 0 {
        chosenMoveId |= (gActiveBattler as i32) << 8;
    } else if var1 == MOVE_TARGET_SELECTED as i32 {
        chosenMoveId |= GetBattlePalaceTarget() as i32;
    } else {
        chosenMoveId |=
            (GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) & 1 ^ 1) as i32) << 8;
    }
    chosenMoveId as u16
}
unsafe fn GetBattlePalaceMoveGroup(r#move: u16) -> u8 {
    match (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())[r#move]
        .target
    {
        MOVE_TARGET_SELECTED
        | MOVE_TARGET_USER_OR_SELECTED
        | 4
        | MOVE_TARGET_BOTH
        | MOVE_TARGET_FOES_AND_ALLY => {
            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [r#move]
                .power
                == 0
            {
                return PALACE_MOVE_GROUP_SUPPORT;
            } else {
                return PALACE_MOVE_GROUP_ATTACK;
            }
        }
        MOVE_TARGET_DEPENDS | MOVE_TARGET_OPPONENTS_FIELD => {
            return PALACE_MOVE_GROUP_SUPPORT;
        }
        MOVE_TARGET_USER => {
            return PALACE_MOVE_GROUP_DEFENSE;
        }
        _ => {
            return PALACE_MOVE_GROUP_ATTACK;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetBattlePalaceTarget() -> u16 {
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        let mut opposing1: u8 = 0;
        let mut opposing2: u8 = 0;
        if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
            opposing1 = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            opposing2 = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
        } else {
            opposing1 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            opposing2 = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
        }
        if gBattleMons[opposing1].hp == gBattleMons[opposing2].hp {
            return ((gActiveBattler as u16 & BIT_SIDE as u16 ^ BIT_SIDE as u16) + (Random() & 2))
                << 8;
        }
        'l1: {
            let sw1: u8 =
                (*(&raw const crate::battle_anim_smokescreen::gBattlePalaceNatureToMoveTarget)
                    .cast::<CArray<u8, 0>>())
                    [GetNatureFromPersonality(gBattleMons[gActiveBattler].personality)];
            let fall = false;
            if sw1 == PALACE_TARGET_STRONGER {
                if gBattleMons[opposing1].hp > gBattleMons[opposing2].hp {
                    return (opposing1 as u16) << 8;
                } else {
                    return (opposing2 as u16) << 8;
                }
            }
            if fall || sw1 == PALACE_TARGET_WEAKER {
                if gBattleMons[opposing1].hp < gBattleMons[opposing2].hp {
                    return (opposing1 as u16) << 8;
                } else {
                    return (opposing2 as u16) << 8;
                }
            }
            if fall || sw1 == PALACE_TARGET_RANDOM {
                return ((gActiveBattler as u16 & BIT_SIDE as u16 ^ BIT_SIDE as u16)
                    + (Random() & 2))
                    << 8;
            }
        }
    }
    (gActiveBattler as u16 ^ 1) << 8
}
pub unsafe fn SpriteCB_WaitForBattlerBallReleaseAnim(sprite: *mut Sprite) {
    let spriteId: u8 = (*sprite).data[1] as u8;
    if gSprites[spriteId].affineAnimEnded() == 0 {
        return;
    }
    if gSprites[spriteId].invisible() != 0 {
        return;
    }
    if gSprites[spriteId].animPaused() != 0 {
        gSprites[spriteId].set_animPaused(0);
    } else {
        if gSprites[spriteId].animEnded() != 0 {
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    }
}
unsafe fn UnusedDoBattleSpriteAffineAnim(sprite: *mut Sprite, pointless: u8) {
    (*sprite).set_animPaused(TRUE);
    (*sprite).callback = Some(SpriteCallbackDummy);
    if pointless == 0 {
        StartSpriteAffineAnim(sprite, 1);
    } else {
        StartSpriteAffineAnim(sprite, 1);
    }
    AnimateSprite(sprite);
}
pub unsafe fn SpriteCB_TrainerSlideIn(sprite: *mut Sprite) {
    if gIntroSlideFlags as i32 & 1 == 0 {
        (*sprite).x2 += (*sprite).data[sSpeedX];
        if (*sprite).x2 == 0 {
            if (*sprite).y2 != 0 {
                (*sprite).callback = Some(SpriteCB_TrainerSlideVertical);
            } else {
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
        }
    }
}
pub(crate) unsafe fn SpriteCB_TrainerSlideVertical(sprite: *mut Sprite) {
    (*sprite).y2 -= 2;
    if (*sprite).y2 == 0 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub unsafe fn InitAndLaunchChosenStatusAnimation(isStatus2: u8, status: u32) {
    (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_statusAnimActive(1);
    if isStatus2 == 0 {
        if status == STATUS1_FREEZE {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_FRZ);
        } else if status == STATUS1_POISON || status & STATUS1_TOXIC_POISON != 0 {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_PSN);
        } else if status == STATUS1_BURN {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_BRN);
        } else if status & STATUS1_SLEEP != 0 {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_SLP);
        } else if status == STATUS1_PARALYSIS {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_PRZ);
        } else {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_statusAnimActive(0);
        }
    } else {
        if status & STATUS2_INFATUATION != 0 {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_INFATUATION);
        } else if status & STATUS2_CONFUSION != 0 {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_CONFUSION);
        } else if status & STATUS2_CURSED != 0 {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_CURSED);
        } else if status & STATUS2_NIGHTMARE != 0 {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_NIGHTMARE);
        } else if status & STATUS2_WRAPPED != 0 {
            LaunchStatusAnimation(gActiveBattler, B_ANIM_STATUS_WRAPPED);
        } else {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_statusAnimActive(0);
        }
    }
}
pub unsafe fn TryHandleLaunchBattleTableAnimation(
    activeBattler: u8,
    atkBattler: u8,
    defBattler: u8,
    tableId: u8,
    argument: u16,
) -> u8 {
    if tableId == B_ANIM_CASTFORM_CHANGE && argument as i32 & CASTFORM_SUBSTITUTE != 0 {
        gBattleMonForms[activeBattler] = argument as u8 & 127;
        return TRUE;
    }
    if (*(*gBattleSpritesDataPtr).battlerData.at(activeBattler)).behindSubstitute() != 0
        && ShouldAnimBeDoneRegardlessOfSubstitute(tableId) == 0
    {
        return TRUE;
    }
    if (*(*gBattleSpritesDataPtr).battlerData.at(activeBattler)).behindSubstitute() != 0
        && tableId == B_ANIM_SUBSTITUTE_FADE
        && gSprites[gBattlerSpriteIds[activeBattler]].invisible() != 0
    {
        LoadBattleMonGfxAndAnimate(activeBattler, TRUE, gBattlerSpriteIds[activeBattler]);
        ClearBehindSubstituteBit(activeBattler);
        return TRUE;
    }
    gBattleAnimAttacker = atkBattler;
    gBattleAnimTarget = defBattler;
    (*(*gBattleSpritesDataPtr).animationData).animArg = argument;
    LaunchBattleAnimation(
        (*crate::asmdata::gBattleAnims_General.cast::<CArray<*mut u8, 0>>())
            .as_ptr()
            .cast_mut(),
        tableId as u16,
        FALSE,
    );
    let taskId: u8 = CreateTask(Some(Task_ClearBitWhenBattleTableAnimDone), 10);
    task_set(taskId, tBattlerId, activeBattler as i16);
    (*(*gBattleSpritesDataPtr)
        .healthBoxesData
        .at(task_get(taskId, tBattlerId)))
    .set_animFromTableActive(1);
    FALSE
}
pub(crate) unsafe fn Task_ClearBitWhenBattleTableAnimDone(taskId: u8) {
    gAnimScriptCallback.unwrap_unchecked()();
    if gAnimScriptActive == 0 {
        (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(task_get(taskId, tBattlerId)))
        .set_animFromTableActive(0);
        DestroyTask(taskId);
    }
}
fn ShouldAnimBeDoneRegardlessOfSubstitute(animId: u8) -> u8 {
    match animId {
        B_ANIM_SUBSTITUTE_FADE
        | B_ANIM_RAIN_CONTINUES
        | B_ANIM_SUN_CONTINUES
        | B_ANIM_SANDSTORM_CONTINUES
        | B_ANIM_HAIL_CONTINUES
        | B_ANIM_SNATCH_MOVE => {
            return TRUE;
        }
        _ => {
            return FALSE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn InitAndLaunchSpecialAnimation(
    activeBattler: u8,
    atkBattler: u8,
    defBattler: u8,
    tableId: u8,
) {
    gBattleAnimAttacker = atkBattler;
    gBattleAnimTarget = defBattler;
    LaunchBattleAnimation(
        (*crate::asmdata::gBattleAnims_Special.cast::<CArray<*mut u8, 0>>())
            .as_ptr()
            .cast_mut(),
        tableId as u16,
        FALSE,
    );
    let taskId: u8 = CreateTask(Some(Task_ClearBitWhenSpecialAnimDone), 10);
    task_set(taskId, tBattlerId, activeBattler as i16);
    (*(*gBattleSpritesDataPtr)
        .healthBoxesData
        .at(task_get(taskId, tBattlerId)))
    .set_specialAnimActive(1);
}
pub(crate) unsafe fn Task_ClearBitWhenSpecialAnimDone(taskId: u8) {
    gAnimScriptCallback.unwrap_unchecked()();
    if gAnimScriptActive == 0 {
        (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(task_get(taskId, tBattlerId)))
        .set_specialAnimActive(0);
        DestroyTask(taskId);
    }
}
pub unsafe fn IsMoveWithoutAnimation(r#move: u16, animationTurn: u8) -> u8 {
    FALSE
}
pub unsafe fn IsBattleSEPlaying(battler: u8) -> u8 {
    let zero: u8 = 0;
    if IsSEPlaying() != 0 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).soundTimer += 1;
        if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).soundTimer < 30 {
            return TRUE;
        }
        m4aMPlayStop(&raw mut gMPlayInfo_SE1);
        m4aMPlayStop(&raw mut gMPlayInfo_SE2);
    }
    if zero == 0 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).soundTimer = 0;
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn BattleLoadOpponentMonSpriteGfx(mon: *mut Pokemon, battler: u8) {
    let mut currentPersonality: u32 = 0;
    let mut species: u16 = 0;
    let mut lzPaletteData: *mut c_void = null_mut();
    let monsPersonality: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies == SPECIES_NONE {
        species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
        currentPersonality = monsPersonality;
    } else {
        species = (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies;
        currentPersonality = gTransformedPersonalities[battler];
    }
    let otId: u32 = GetMonData2(mon, MON_DATA_OT_ID);
    let position: u8 = GetBattlerPosition(battler);
    HandleLoadSpecialPokePic_DontHandleDeoxys(
        (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
            .cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[position],
        species as i32,
        currentPersonality,
    );
    let mut paletteOffset: u16 = 0x100 + battler as u16 * 16;
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies == SPECIES_NONE {
        lzPaletteData = GetMonFrontSpritePal(mon) as *mut c_void;
    } else {
        lzPaletteData =
            GetMonSpritePalFromSpeciesAndPersonality(species, otId, monsPersonality) as *mut c_void;
    }
    LZDecompressWram(
        lzPaletteData as *mut u32,
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
    );
    LoadPalette(
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        paletteOffset,
        32,
    );
    LoadPalette(
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        128 + (battler as u16 * 16),
        32,
    );
    if species == SPECIES_CASTFORM {
        paletteOffset = 0x100 + battler as u16 * 16;
        LZDecompressWram(
            lzPaletteData as *mut u32,
            (*gBattleStruct).castformPalette.as_mut_ptr() as *mut c_void,
        );
        LoadPalette(
            (*gBattleStruct).castformPalette[gBattleMonForms[battler]].as_mut_ptr() as *mut c_void,
            paletteOffset,
            32,
        );
    }
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != SPECIES_NONE {
        BlendPalette(paletteOffset, 16, 6, 32767);
        CpuSet(
            &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset] as *mut c_void,
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset] as *mut c_void,
            0x4000008,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BattleLoadPlayerMonSpriteGfx(mon: *mut Pokemon, battler: u8) {
    let mut currentPersonality: u32 = 0;
    let mut species: u16 = 0;
    let mut lzPaletteData: *mut c_void = null_mut();
    let monsPersonality: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies == SPECIES_NONE {
        species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
        currentPersonality = monsPersonality;
    } else {
        species = (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies;
        currentPersonality = gTransformedPersonalities[battler];
    }
    let otId: u32 = GetMonData2(mon, MON_DATA_OT_ID);
    let position: u8 = GetBattlerPosition(battler);
    if ShouldIgnoreDeoxysForm(1, battler) == 1
        || (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != SPECIES_NONE
    {
        HandleLoadSpecialPokePic_DontHandleDeoxys(
            (&raw const (*(&raw const crate::data::data_tables::gMonBackPicTable)
                .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                .cast_mut(),
            (*gMonSpritesGfxPtr).sprites.ptr[position],
            species as i32,
            currentPersonality,
        );
    } else {
        HandleLoadSpecialPokePic(
            (&raw const (*(&raw const crate::data::data_tables::gMonBackPicTable)
                .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                .cast_mut(),
            (*gMonSpritesGfxPtr).sprites.ptr[position],
            species as i32,
            currentPersonality,
        );
    }
    let mut paletteOffset: u16 = 0x100 + battler as u16 * 16;
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies == SPECIES_NONE {
        lzPaletteData = GetMonFrontSpritePal(mon) as *mut c_void;
    } else {
        lzPaletteData =
            GetMonSpritePalFromSpeciesAndPersonality(species, otId, monsPersonality) as *mut c_void;
    }
    LZDecompressWram(
        lzPaletteData as *mut u32,
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
    );
    LoadPalette(
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        paletteOffset,
        32,
    );
    LoadPalette(
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        128 + (battler as u16 * 16),
        32,
    );
    if species == SPECIES_CASTFORM {
        paletteOffset = 0x100 + battler as u16 * 16;
        LZDecompressWram(
            lzPaletteData as *mut u32,
            (*gBattleStruct).castformPalette.as_mut_ptr() as *mut c_void,
        );
        LoadPalette(
            (*gBattleStruct).castformPalette[gBattleMonForms[battler]].as_mut_ptr() as *mut c_void,
            paletteOffset,
            32,
        );
    }
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != SPECIES_NONE {
        BlendPalette(paletteOffset, 16, 6, 32767);
        CpuSet(
            &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset] as *mut c_void,
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset] as *mut c_void,
            0x4000008,
        );
    }
}
fn BattleGfxSfxDummy1() {}
pub fn BattleGfxSfxDummy2(species: u16) {}
pub unsafe fn DecompressTrainerFrontPic(frontPicId: u16, battler: u8) {
    let position: u8 = GetBattlerPosition(battler);
    DecompressPicFromTable_2(
        (&raw const (*(&raw const crate::data::data_tables::gTrainerFrontPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[frontPicId])
            .cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[position],
        SPECIES_NONE as i32,
    );
    LoadCompressedSpritePalette(
        (&raw const (*(&raw const crate::data::data_tables::gTrainerFrontPicPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[frontPicId])
            .cast_mut(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn DecompressTrainerBackPic(backPicId: u16, battler: u8) {
    let position: u8 = GetBattlerPosition(battler);
    DecompressPicFromTable_2(
        (&raw const (*(&raw const crate::data::data_tables::gTrainerBackPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[backPicId])
            .cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[position],
        SPECIES_NONE as i32,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::data_tables::gTrainerBackPicPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[backPicId]
            .data,
        0x100 + battler as u16 * 16,
        32,
    );
}
pub unsafe fn BattleGfxSfxDummy3(gender: u8) {}
pub unsafe fn FreeTrainerFrontPicPalette(frontPicId: u16) {
    FreeSpritePaletteByTag(
        (*(&raw const crate::data::data_tables::gTrainerFrontPicPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[frontPicId]
            .tag,
    );
}
pub unsafe fn BattleLoadAllHealthBoxesGfxAtOnce() {
    let mut numberOfBattlers: u8 = 0;
    LoadSpritePalette((&raw const sSpritePalettes_HealthBoxHealthBar[0]).cast_mut());
    LoadSpritePalette((&raw const sSpritePalettes_HealthBoxHealthBar[1]).cast_mut());
    if IsDoubleBattle() == 0 {
        LoadCompressedSpriteSheet((&raw const *sSpriteSheet_SinglesPlayerHealthbox).cast_mut());
        LoadCompressedSpriteSheet((&raw const *sSpriteSheet_SinglesOpponentHealthbox).cast_mut());
        numberOfBattlers = 2;
    } else {
        LoadCompressedSpriteSheet((&raw const sSpriteSheets_DoublesPlayerHealthbox[0]).cast_mut());
        LoadCompressedSpriteSheet((&raw const sSpriteSheets_DoublesPlayerHealthbox[1]).cast_mut());
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheets_DoublesOpponentHealthbox[0]).cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheets_DoublesOpponentHealthbox[1]).cast_mut(),
        );
        numberOfBattlers = MAX_BATTLERS_COUNT;
    }
    for i in 0..numberOfBattlers {
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheets_HealthBar[gBattlerPositions[i]]).cast_mut(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BattleLoadAllHealthBoxesGfx(state: u8) -> u8 {
    let mut retVal: u8 = FALSE;
    if state != 0 {
        if state == 1 {
            LoadSpritePalette((&raw const sSpritePalettes_HealthBoxHealthBar[0]).cast_mut());
            LoadSpritePalette((&raw const sSpritePalettes_HealthBoxHealthBar[1]).cast_mut());
        } else if IsDoubleBattle() == 0 {
            if state == 2 {
                if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0 {
                    LoadCompressedSpriteSheet(
                        (&raw const *sSpriteSheet_SafariHealthbox).cast_mut(),
                    );
                } else {
                    LoadCompressedSpriteSheet(
                        (&raw const *sSpriteSheet_SinglesPlayerHealthbox).cast_mut(),
                    );
                }
            } else if state == 3 {
                LoadCompressedSpriteSheet(
                    (&raw const *sSpriteSheet_SinglesOpponentHealthbox).cast_mut(),
                );
            } else if state == 4 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_HealthBar[gBattlerPositions[0]]).cast_mut(),
                );
            } else if state == 5 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_HealthBar[gBattlerPositions[1]]).cast_mut(),
                );
            } else {
                retVal = TRUE;
            }
        } else {
            if state == 2 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_DoublesPlayerHealthbox[0]).cast_mut(),
                );
            } else if state == 3 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_DoublesPlayerHealthbox[1]).cast_mut(),
                );
            } else if state == 4 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_DoublesOpponentHealthbox[0]).cast_mut(),
                );
            } else if state == 5 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_DoublesOpponentHealthbox[1]).cast_mut(),
                );
            } else if state == 6 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_HealthBar[gBattlerPositions[0]]).cast_mut(),
                );
            } else if state == 7 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_HealthBar[gBattlerPositions[1]]).cast_mut(),
                );
            } else if state == 8 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_HealthBar[gBattlerPositions[2]]).cast_mut(),
                );
            } else if state == 9 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheets_HealthBar[gBattlerPositions[3]]).cast_mut(),
                );
            } else {
                retVal = TRUE;
            }
        }
    }
    retVal
}
pub unsafe fn LoadBattleBarGfx(unused: u8) {
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gBattleInterfaceGfx_BattleBar)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*gMonSpritesGfxPtr).barFontGfx as *mut c_void,
    );
}
pub unsafe fn BattleInitAllSprites(state1: *mut u8, battler: *mut u8) -> u8 {
    let mut retVal: u8 = FALSE;
    match *state1 {
        0 => {
            ClearSpritesBattlerHealthboxAnimData();
            *state1 += 1;
        }
        1 => {
            if BattleLoadAllHealthBoxesGfx(*battler) == 0 {
                *battler += 1;
            } else {
                *battler = 0;
                *state1 += 1;
            }
        }
        2 => {
            *state1 += 1;
        }
        3 => {
            if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0 && *battler == 0 {
                gHealthboxSpriteIds[*battler] = CreateSafariPlayerHealthboxSprites();
            } else {
                gHealthboxSpriteIds[*battler] = CreateBattlerHealthboxSprites(*battler);
            }
            *battler += 1;
            if *battler == gBattlersCount {
                *battler = 0;
                *state1 += 1;
            }
        }
        4 => {
            InitBattlerHealthboxCoords(*battler);
            if gBattlerPositions[*battler] <= B_POSITION_OPPONENT_LEFT {
                DummyBattleInterfaceFunc(gHealthboxSpriteIds[*battler], FALSE);
            } else {
                DummyBattleInterfaceFunc(gHealthboxSpriteIds[*battler], TRUE);
            }
            *battler += 1;
            if *battler == gBattlersCount {
                *battler = 0;
                *state1 += 1;
            }
        }
        5 => {
            if GetBattlerSide(*battler) == B_SIDE_PLAYER {
                if gBattleTypeFlags & BATTLE_TYPE_SAFARI == 0 {
                    UpdateHealthboxAttribute(
                        gHealthboxSpriteIds[*battler],
                        &raw mut gPlayerParty[gBattlerPartyIndexes[*battler]],
                        HEALTHBOX_ALL,
                    );
                }
            } else {
                UpdateHealthboxAttribute(
                    gHealthboxSpriteIds[*battler],
                    &raw mut gEnemyParty[gBattlerPartyIndexes[*battler]],
                    HEALTHBOX_ALL,
                );
            }
            SetHealthboxSpriteInvisible(gHealthboxSpriteIds[*battler]);
            *battler += 1;
            if *battler == gBattlersCount {
                *battler = 0;
                *state1 += 1;
            }
        }
        6 => {
            LoadAndCreateEnemyShadowSprites();
            BufferBattlePartyCurrentOrder();
            retVal = TRUE;
        }
        _ => {}
    }
    retVal
}
#[unsafe(no_mangle)]
pub unsafe fn ClearSpritesHealthboxAnimData() {
    memset((*gBattleSpritesDataPtr).healthBoxesData as *mut u8, 0, 48);
    memset((*gBattleSpritesDataPtr).animationData as *mut u8, 0, 16);
}
unsafe fn ClearSpritesBattlerHealthboxAnimData() {
    ClearSpritesHealthboxAnimData();
    memset((*gBattleSpritesDataPtr).battlerData as *mut u8, 0, 16);
}
pub unsafe fn CopyAllBattleSpritesInvisibilities() {
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        (*(*gBattleSpritesDataPtr).battlerData.at(i))
            .set_invisible(gSprites[gBattlerSpriteIds[i]].invisible());
        i += 1;
    }
}
pub unsafe fn CopyBattleSpriteInvisibility(battler: u8) {
    (*(*gBattleSpritesDataPtr).battlerData.at(battler))
        .set_invisible(gSprites[gBattlerSpriteIds[battler]].invisible());
}
pub unsafe fn HandleSpeciesGfxDataChange(battlerAtk: u8, battlerDef: u8, castform: u8) {
    let mut paletteOffset: u16 = 0;
    let mut personalityValue: u32 = 0;
    let mut otId: u32 = 0;
    let mut position: u8 = 0;
    let mut lzPaletteData: *mut u32 = null_mut();
    if castform != 0 {
        StartSpriteAnim(
            &raw mut gSprites[gBattlerSpriteIds[battlerAtk]],
            (*(*gBattleSpritesDataPtr).animationData).animArg as u8,
        );
        paletteOffset = 0x100 + battlerAtk as u16 * 16;
        LoadPalette(
            (*gBattleStruct).castformPalette[(*(*gBattleSpritesDataPtr).animationData).animArg]
                .as_mut_ptr() as *mut c_void,
            paletteOffset,
            32,
        );
        gBattleMonForms[battlerAtk] = (*(*gBattleSpritesDataPtr).animationData).animArg as u8;
        if (*(*gBattleSpritesDataPtr).battlerData.at(battlerAtk)).transformSpecies != SPECIES_NONE {
            BlendPalette(paletteOffset, 16, 6, 32767);
            CpuSet(
                &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                    .cast::<CArray<u16, 512>>()
                    .cast_mut())[paletteOffset] as *mut c_void,
                &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                    .cast::<CArray<u16, 512>>()
                    .cast_mut())[paletteOffset] as *mut c_void,
                0x4000008,
            );
        }
        gSprites[gBattlerSpriteIds[battlerAtk]].y = GetBattlerSpriteDefault_Y(battlerAtk) as i16;
    } else {
        let mut targetSpecies: u16 = 0;
        if IsContest() != 0 {
            position = B_POSITION_PLAYER_LEFT;
            targetSpecies = (*(*gContestResources).moveAnim).targetSpecies;
            personalityValue = (*(*gContestResources).moveAnim).personality;
            otId = (*(*gContestResources).moveAnim).otId;
            HandleLoadSpecialPokePic_DontHandleDeoxys(
                (&raw const (*(&raw const crate::data::data_tables::gMonBackPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[targetSpecies])
                    .cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[position],
                targetSpecies as i32,
                (*(*gContestResources).moveAnim).targetPersonality,
            );
        } else {
            position = GetBattlerPosition(battlerAtk);
            if GetBattlerSide(battlerDef) == B_SIDE_OPPONENT {
                targetSpecies = GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[battlerDef]],
                    MON_DATA_SPECIES,
                ) as u16;
            } else {
                targetSpecies = GetMonData2(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[battlerDef]],
                    MON_DATA_SPECIES,
                ) as u16;
            }
            if GetBattlerSide(battlerAtk) == B_SIDE_PLAYER {
                personalityValue = GetMonData2(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[battlerAtk]],
                    MON_DATA_PERSONALITY,
                );
                otId = GetMonData2(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[battlerAtk]],
                    MON_DATA_OT_ID,
                );
                HandleLoadSpecialPokePic_DontHandleDeoxys(
                    (&raw const (*(&raw const crate::data::data_tables::gMonBackPicTable)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())[targetSpecies])
                        .cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr[position],
                    targetSpecies as i32,
                    gTransformedPersonalities[battlerAtk],
                );
            } else {
                personalityValue = GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[battlerAtk]],
                    MON_DATA_PERSONALITY,
                );
                otId = GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[battlerAtk]],
                    MON_DATA_OT_ID,
                );
                HandleLoadSpecialPokePic_DontHandleDeoxys(
                    (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())[targetSpecies])
                        .cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr[position],
                    targetSpecies as i32,
                    gTransformedPersonalities[battlerAtk],
                );
            }
        }
        {
            let mut _src: *mut c_void = (*gMonSpritesGfxPtr).sprites.ptr[position];
            let mut _dest: *mut c_void = (OBJ_VRAM0
                + gSprites[gBattlerSpriteIds[battlerAtk]].oam.tileNum() as i32 * 32)
                as usize as *mut c_void;
            let mut _size: u32 = MON_PIC_SIZE as u32;
            {
                {
                    {
                        let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                        volatile_write(dmaRegs, _src as usize as u32);
                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                        volatile_write(dmaRegs.at(2), 0x84000000 | (_size / 4));
                        let _ = (dmaRegs.at(2)).read_volatile();
                    }
                }
            }
        }
        paletteOffset = 0x100 + battlerAtk as u16 * 16;
        lzPaletteData =
            GetMonSpritePalFromSpeciesAndPersonality(targetSpecies, otId, personalityValue);
        LZDecompressWram(
            lzPaletteData,
            (*(&raw const crate::decompress::gDecompressionBuffer)
                .cast::<CArray<u8, 16384>>()
                .cast_mut())
            .as_mut_ptr() as *mut c_void,
        );
        LoadPalette(
            (*(&raw const crate::decompress::gDecompressionBuffer)
                .cast::<CArray<u8, 16384>>()
                .cast_mut())
            .as_mut_ptr() as *mut c_void,
            paletteOffset,
            32,
        );
        if targetSpecies == SPECIES_CASTFORM {
            gSprites[gBattlerSpriteIds[battlerAtk]].anims =
                (*(&raw const crate::data::data_tables::gMonFrontAnimsPtrTable).cast::<CArray<
                    *mut *mut AnimCmd,
                    0,
                >>(
                ))[targetSpecies];
            LZDecompressWram(
                lzPaletteData,
                (*gBattleStruct).castformPalette.as_mut_ptr() as *mut c_void,
            );
            LoadPalette(
                (*gBattleStruct).castformPalette[gBattleMonForms[battlerDef]].as_mut_ptr()
                    as *mut c_void,
                paletteOffset,
                32,
            );
        }
        BlendPalette(paletteOffset, 16, 6, 32767);
        CpuSet(
            &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset] as *mut c_void,
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[paletteOffset] as *mut c_void,
            0x4000008,
        );
        if IsContest() == 0 {
            (*(*gBattleSpritesDataPtr).battlerData.at(battlerAtk)).transformSpecies = targetSpecies;
            gBattleMonForms[battlerAtk] = gBattleMonForms[battlerDef];
        }
        gSprites[gBattlerSpriteIds[battlerAtk]].y = GetBattlerSpriteDefault_Y(battlerAtk) as i16;
        StartSpriteAnim(
            &raw mut gSprites[gBattlerSpriteIds[battlerAtk]],
            gBattleMonForms[battlerAtk],
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BattleLoadSubstituteOrMonSpriteGfx(battler: u8, loadMonSprite: u8) {
    let mut position: i32 = 0;
    let mut palOffset: i32 = 0;
    if loadMonSprite == 0 {
        if IsContest() != 0 {
            position = B_POSITION_PLAYER_LEFT as i32;
        } else {
            position = GetBattlerPosition(battler) as i32;
        }
        if IsContest() != 0 {
            LZDecompressVram(
                (*(&raw const crate::data::graphics::gSubstituteDollBackGfx)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[position],
            );
        } else if GetBattlerSide(battler) != B_SIDE_PLAYER {
            LZDecompressVram(
                (*(&raw const crate::data::graphics::gSubstituteDollFrontGfx)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[position],
            );
        } else {
            LZDecompressVram(
                (*(&raw const crate::data::graphics::gSubstituteDollBackGfx)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[position],
            );
        }
        for i in 1..4i32 {
            {
                let mut _src: *mut c_void = (*gMonSpritesGfxPtr).sprites.ptr[position];
                let mut _dest: *mut c_void = (*gMonSpritesGfxPtr).sprites.byte[position]
                    .at(MON_PIC_SIZE as i32 * i)
                    as *mut c_void;
                let mut _size: u32 = MON_PIC_SIZE as u32;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x84000000 | (_size / 4));
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x84000400);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                    _src = (_src as *mut u8).at(4096) as *mut c_void;
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                }
            }
        }
        palOffset = 0x100 + battler as i32 * 16;
        LoadCompressedPalette(
            (*(&raw const crate::data::graphics::gSubstituteDollPal).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
            palOffset as u16,
            32,
        );
    } else {
        if IsContest() == 0 {
            if GetBattlerSide(battler) != B_SIDE_PLAYER {
                BattleLoadOpponentMonSpriteGfx(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
                    battler,
                );
            } else {
                BattleLoadPlayerMonSpriteGfx(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
                    battler,
                );
            }
        }
    }
}
pub unsafe fn LoadBattleMonGfxAndAnimate(battler: u8, loadMonSprite: u8, spriteId: u8) {
    BattleLoadSubstituteOrMonSpriteGfx(battler, loadMonSprite);
    StartSpriteAnim(&raw mut gSprites[spriteId], gBattleMonForms[battler]);
    if loadMonSprite == 0 {
        gSprites[spriteId].y = GetSubstituteSpriteDefault_Y(battler) as i16;
    } else {
        gSprites[spriteId].y = GetBattlerSpriteDefault_Y(battler) as i16;
    }
}
pub unsafe fn TrySetBehindSubstituteSpriteBit(battler: u8, r#move: u16) {
    if r#move == MOVE_SUBSTITUTE {
        (*(*gBattleSpritesDataPtr).battlerData.at(battler)).set_behindSubstitute(1);
    }
}
pub unsafe fn ClearBehindSubstituteBit(battler: u8) {
    (*(*gBattleSpritesDataPtr).battlerData.at(battler)).set_behindSubstitute(0);
}
pub unsafe fn HandleLowHpMusicChange(mon: *mut Pokemon, battler: u8) {
    let hp: u16 = GetMonData2(mon, MON_DATA_HP) as u16;
    let maxHP: u16 = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
    if GetHPBarLevel(hp as i16, maxHP as i16) == HP_BAR_RED {
        if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).lowHpSong() == 0 {
            if (*(*gBattleSpritesDataPtr).battlerData.at(battler as i32 ^ 2)).lowHpSong() == 0 {
                PlaySE(SE_LOW_HEALTH);
            }
            (*(*gBattleSpritesDataPtr).battlerData.at(battler)).set_lowHpSong(1);
        }
    } else {
        (*(*gBattleSpritesDataPtr).battlerData.at(battler)).set_lowHpSong(0);
        if IsDoubleBattle() == 0 {
            m4aSongNumStop(SE_LOW_HEALTH);
            return;
        }
        if IsDoubleBattle() != 0
            && (*(*gBattleSpritesDataPtr).battlerData.at(battler as i32 ^ 2)).lowHpSong() == 0
        {
            m4aSongNumStop(SE_LOW_HEALTH);
        }
    }
}
pub unsafe fn BattleStopLowHpSound() {
    let playerBattler: u8 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
    (*(*gBattleSpritesDataPtr).battlerData.at(playerBattler)).set_lowHpSong(0);
    if IsDoubleBattle() != 0 {
        (*(*gBattleSpritesDataPtr)
            .battlerData
            .at(playerBattler as i32 ^ 2))
        .set_lowHpSong(0);
    }
    m4aSongNumStop(SE_LOW_HEALTH);
}
pub unsafe fn GetMonHPBarLevel(mon: *mut Pokemon) -> u8 {
    let hp: u16 = GetMonData2(mon, MON_DATA_HP) as u16;
    let maxHP: u16 = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
    GetHPBarLevel(hp as i16, maxHP as i16)
}
pub unsafe fn HandleBattleLowHpMusicChange() {
    if gMain.inBattle() != 0 {
        let playerBattler1: u8 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        let playerBattler2: u8 = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
        let battler1PartyId: u8 =
            GetPartyIdFromBattlePartyId(gBattlerPartyIndexes[playerBattler1] as u8);
        let battler2PartyId: u8 =
            GetPartyIdFromBattlePartyId(gBattlerPartyIndexes[playerBattler2] as u8);
        if GetMonData2(&raw mut gPlayerParty[battler1PartyId], MON_DATA_HP) != 0 {
            HandleLowHpMusicChange(&raw mut gPlayerParty[battler1PartyId], playerBattler1);
        }
        if IsDoubleBattle() != 0
            && GetMonData2(&raw mut gPlayerParty[battler2PartyId], MON_DATA_HP) != 0
        {
            HandleLowHpMusicChange(&raw mut gPlayerParty[battler2PartyId], playerBattler2);
        }
    }
}
pub unsafe fn SetBattlerSpriteAffineMode(affineMode: u8) {
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        if IsBattlerSpritePresent(i as u8) != 0 {
            gSprites[gBattlerSpriteIds[i]]
                .oam
                .set_affineMode(affineMode as u32);
            if affineMode == ST_OAM_AFFINE_OFF as u8 {
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(i)).matrixNum =
                    gSprites[gBattlerSpriteIds[i]].oam.matrixNum() as u8;
                gSprites[gBattlerSpriteIds[i]].oam.set_matrixNum(0);
            } else {
                gSprites[gBattlerSpriteIds[i]].oam.set_matrixNum(
                    (*(*gBattleSpritesDataPtr).healthBoxesData.at(i)).matrixNum as u32,
                );
            }
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LoadAndCreateEnemyShadowSprites() {
    LoadCompressedSpriteSheet(
        (&raw const (*(&raw const crate::battle_anim_smokescreen::gSpriteSheet_EnemyShadow)
            .cast::<CompressedSpriteSheet>()))
            .cast_mut(),
    );
    let mut battler: u8 = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
    (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId = CreateSprite(
        (&raw const (*(&raw const crate::battle_anim_smokescreen::gSpriteTemplate_EnemyShadow)
            .cast::<SpriteTemplate>()))
            .cast_mut(),
        GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16,
        GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as i16 + 29,
        0xC8,
    );
    gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].data[0] =
        battler as i16;
    if IsDoubleBattle() != 0 {
        battler = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId = CreateSprite(
            (&raw const (*(&raw const crate::battle_anim_smokescreen::gSpriteTemplate_EnemyShadow).cast::<SpriteTemplate>())).cast_mut(),
            GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16,
            GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as i16 + 29,
            0xC8,
        );
        gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].data[0] =
            battler as i16;
    }
}
pub unsafe fn SpriteCB_EnemyShadow(shadowSprite: *mut Sprite) {
    let mut invisible: u8 = FALSE;
    let battler: u8 = (*shadowSprite).data[tBattlerId] as u8;
    let battlerSprite: *mut Sprite = &raw mut gSprites[gBattlerSpriteIds[battler]];
    if (*battlerSprite).inUse() == 0 || IsBattlerSpritePresent(battler) == 0 {
        (*shadowSprite).callback = Some(SpriteCB_SetInvisible);
        return;
    }
    if gAnimScriptActive != 0 || (*battlerSprite).invisible() != 0 {
        invisible = TRUE;
    } else if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != SPECIES_NONE
        && (*(&raw const crate::data::data_tables::gEnemyMonElevation).cast::<CArray<u8, 412>>())
            [(*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies]
            == 0
    {
        invisible = TRUE;
    }
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).behindSubstitute() != 0 {
        invisible = TRUE;
    }
    (*shadowSprite).x = (*battlerSprite).x;
    (*shadowSprite).x2 = (*battlerSprite).x2;
    (*shadowSprite).set_invisible(invisible as u16);
}
#[unsafe(no_mangle)]
pub unsafe fn SpriteCB_SetInvisible(sprite: *mut Sprite) {
    (*sprite).set_invisible(TRUE as u16);
}
#[unsafe(no_mangle)]
pub unsafe fn SetBattlerShadowSpriteCallback(battler: u8, mut species: u16) {
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        return;
    }
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != SPECIES_NONE {
        species = (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies;
    }
    if (*(&raw const crate::data::data_tables::gEnemyMonElevation).cast::<CArray<u8, 412>>())
        [species]
        != 0
    {
        gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].callback =
            Some(SpriteCB_EnemyShadow);
    } else {
        gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].callback =
            Some(SpriteCB_SetInvisible);
    }
}
pub unsafe fn HideBattlerShadowSprite(battler: u8) {
    gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].callback =
        Some(SpriteCB_SetInvisible);
}
#[unsafe(no_mangle)]
pub unsafe fn FillAroundBattleWindows() {
    let mut vramPtr: *mut u16 = 100663872_usize as *mut u16;
    for i in 0..9i32 {
        for j in 0..16i32 {
            if *vramPtr as i32 & 0xF000 == 0 {
                *vramPtr |= 0xF000;
            }
            if *vramPtr as i32 & 0x0F00 == 0 {
                *vramPtr |= 0x0F00;
            }
            if *vramPtr as i32 & 0x00F0 == 0 {
                *vramPtr |= 0x00F0;
            }
            if *vramPtr as i32 & 0x000F == 0 {
                *vramPtr |= 0x000F;
            }
            vramPtr = vramPtr.at(1);
        }
    }
}
pub unsafe fn ClearTemporarySpeciesSpriteData(battler: u8, dontClearSubstitute: u8) {
    (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies = SPECIES_NONE;
    gBattleMonForms[battler] = 0;
    if dontClearSubstitute == 0 {
        ClearBehindSubstituteBit(battler);
    }
}
pub unsafe fn AllocateMonSpritesGfx() {
    gMonSpritesGfxPtr = null_mut();
    gMonSpritesGfxPtr = AllocZeroed(384) as *mut MonSpritesGfx;
    (*gMonSpritesGfxPtr).firstDecompressed = AllocZeroed(32768);
    for i in 0..MAX_BATTLERS_COUNT {
        (*gMonSpritesGfxPtr).sprites.ptr[i] = ((*gMonSpritesGfxPtr).firstDecompressed as *mut u8)
            .at(i as i32 * MON_PIC_SIZE as i32 * 4)
            as *mut c_void;
        *(*gMonSpritesGfxPtr).templates.as_mut_ptr().at(i) =
            (*(&raw const crate::data::pokemon::gBattlerSpriteTemplates)
                .cast::<CArray<SpriteTemplate, 0>>())[i];
        for j in 0..4u8 {
            (*gMonSpritesGfxPtr).frameImages[i][j].data =
                ((*gMonSpritesGfxPtr).sprites.ptr[i] as *mut u8).at(j as i32 * MON_PIC_SIZE as i32)
                    as *mut c_void;
            (*gMonSpritesGfxPtr).frameImages[i][j].size = MON_PIC_SIZE;
        }
        (*gMonSpritesGfxPtr).templates[i].images = (*gMonSpritesGfxPtr).frameImages[i].as_mut_ptr();
    }
    (*gMonSpritesGfxPtr).barFontGfx = AllocZeroed(0x1000) as *mut u8;
}
pub unsafe fn FreeMonSpritesGfx() {
    if gMonSpritesGfxPtr.is_null() {
        return;
    }
    if !(*gMonSpritesGfxPtr).buffer.is_null() {
        Free((*gMonSpritesGfxPtr).buffer as *mut c_void);
        (*gMonSpritesGfxPtr).buffer = null_mut();
    }
    if !(*gMonSpritesGfxPtr).unusedPtr.is_null() {
        Free((*gMonSpritesGfxPtr).unusedPtr);
        (*gMonSpritesGfxPtr).unusedPtr = null_mut();
    }
    Free((*gMonSpritesGfxPtr).barFontGfx as *mut c_void);
    (*gMonSpritesGfxPtr).barFontGfx = null_mut();
    Free((*gMonSpritesGfxPtr).firstDecompressed);
    (*gMonSpritesGfxPtr).firstDecompressed = null_mut();
    (*gMonSpritesGfxPtr).sprites.ptr[0] = null_mut();
    (*gMonSpritesGfxPtr).sprites.ptr[1] = null_mut();
    (*gMonSpritesGfxPtr).sprites.ptr[2] = null_mut();
    (*gMonSpritesGfxPtr).sprites.ptr[3] = null_mut();
    Free(gMonSpritesGfxPtr as *mut c_void);
    gMonSpritesGfxPtr = null_mut();
}
pub unsafe fn ShouldPlayNormalMonCry(mon: *mut Pokemon) -> u32 {
    if GetMonData2(mon, MON_DATA_STATUS) & 4095 != 0 {
        return FALSE as u32;
    }
    let hp: i16 = GetMonData2(mon, MON_DATA_HP) as i16;
    let maxHP: i16 = GetMonData2(mon, MON_DATA_MAX_HP) as i16;
    let barLevel: i32 = GetHPBarLevel(hp, maxHP) as i32;
    if barLevel <= HP_BAR_YELLOW as i32 {
        return FALSE as u32;
    }
    TRUE as u32
}
