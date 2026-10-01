//! Translated from `src/battle_arena.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    dead_code,
    unused_assignments
)]

use crate::battle_main::{
    gBattle_WIN0H, gBattle_WIN0V, gBattleMons, gBattleScripting, gBattleStruct, gCurrentMove,
    gHitMarker, gMoveResultFlags, gProtectStructs,
};
use crate::battle_main::{
    gBattleCommunication, gBattleTextBuff1, gBattleTextBuff2, gDisplayedStringBattle,
};
use crate::battle_message::{
    BattlePutTextOnWindow, BattleStringExpandPlaceholdersToDisplayedString,
};
use crate::battle_script_commands::HandleBattleWindow;
use crate::battle_setup::gTrainerBattleOpponent_A;
use crate::battle_tower::GetFrontierTrainerName;
use crate::bg::{CopyBgTilemapBufferToVram, FillBgTilemapBufferRect, IsDma3ManagerBusyWithBgCopy};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::VarSet;
use crate::ffi::{gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_Result};
use crate::frontier_util::SaveGameFrontier;
use crate::gpu_regs::SetGpuReg;
use crate::item::{AddBagItem, CopyItemName};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::m4a::{gMPlayInfo_BGM, m4aMPlayVolumeControl};
use crate::overworld::SetDynamicWarp;
use crate::palette::{BeginNormalPaletteFade, LoadCompressedPalette, gPaletteFade};
use crate::random::Random;
use crate::sound::PlaySE;
use crate::sprite::FreeSpriteTilesByTag;
use crate::sprite::gSprites;
use crate::string_util::gStringVar1;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
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
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// Data tables (translate with cdata.py): sMindRatings sOam_JudgmentIcon sAnim_JudgmentIcon_X sAnim_JudgmentIcon_Triangle sAnim_JudgmentIcon_Circle sAnim_JudgmentIcon_Line sAnims_JudgmentIcon sSpriteTemplate_JudgmentIcon sBattleArenaJudgmentSymbolsSpriteSheet sArenaFunctions sShortStreakPrizeItems sLongStreakPrizeItems

const ANIM_ICON_CIRCLE: i32 = 2;
const ANIM_ICON_LINE: u8 = 3;
const ANIM_ICON_TRIANGLE: i32 = 1;
const ANIM_ICON_X: i32 = 0;
const JUDGMENT_STATE_FINISHED: u8 = 8;
const TAG_JUDGMENT_ICON: u16 = 1000;

static sArenaFunctions: Table<CArray<Option<unsafe fn()>, 7>> =
    Table((&raw const crate::data::battle_arena::sArenaFunctions).cast());
static sBattleArenaJudgmentSymbolsSpriteSheet: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::battle_arena::sBattleArenaJudgmentSymbolsSpriteSheet).cast());
static sLongStreakPrizeItems: Table<CArray<u16, 9>> =
    Table((&raw const crate::data::battle_arena::sLongStreakPrizeItems).cast());
static sMindRatings: Table<CArray<i8, 355>> =
    Table((&raw const crate::data::battle_arena::sMindRatings).cast());
static sShortStreakPrizeItems: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::battle_arena::sShortStreakPrizeItems).cast());
static sSpriteTemplate_JudgmentIcon: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_arena::sSpriteTemplate_JudgmentIcon).cast());

#[unsafe(no_mangle)]
pub unsafe fn CallBattleArenaFunction() {
    sArenaFunctions[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub unsafe fn BattleArena_ShowJudgmentWindow(state: *mut u8) -> u8 {
    let mut result: u8 = ARENA_RESULT_RUNNING;
    match *state {
        0 => {
            BeginNormalPaletteFade(0x7FFFFF1C, 4, 0, 8, 0);
            SetGpuReg(REG_OFFSET_WININ, 16190);
            LoadCompressedSpriteSheet(sBattleArenaJudgmentSymbolsSpriteSheet.as_ptr().cast_mut());
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gBattleArenaJudgmentSymbolsPalette)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                496,
                32,
            );
            gBattle_WIN0H = 0xFF;
            gBattle_WIN0V = 0x70;
            *state += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                HandleBattleWindow(5, 0, 24, 13, 0);
                *state += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                gBattleTextBuff1[0] = CHAR_0;
                gBattleTextBuff1[1] = EOS;
                gBattleTextBuff2[0] = CHAR_0;
                gBattleTextBuff2[1] = EOS;
                BattleStringExpandPlaceholdersToDisplayedString(
                    (*(&raw const crate::data::battle_message::gText_PlayerMon1Name)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), ARENA_WIN_PLAYER_NAME);
                BattlePutTextOnWindow(
                    (*(&raw const crate::data::battle_message::gText_Vs).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    ARENA_WIN_VS,
                );
                BattleStringExpandPlaceholdersToDisplayedString(
                    (*(&raw const crate::data::battle_message::gText_OpponentMon1Name)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), ARENA_WIN_OPPONENT_NAME);
                BattlePutTextOnWindow(
                    (*(&raw const crate::data::battle_message::gText_Mind).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    ARENA_WIN_MIND,
                );
                BattlePutTextOnWindow(
                    (*(&raw const crate::data::battle_message::gText_Skill)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    ARENA_WIN_SKILL,
                );
                BattlePutTextOnWindow(
                    (*(&raw const crate::data::battle_message::gText_Body).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    ARENA_WIN_BODY,
                );
                BattleStringExpandPlaceholdersToDisplayedString(
                    (*(&raw const crate::data::battle_message::gText_Judgment)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                BattlePutTextOnWindow(
                    gDisplayedStringBattle.as_mut_ptr(),
                    ARENA_WIN_JUDGMENT_TITLE,
                );
                *state += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetGpuReg(REG_OFFSET_WININ, 16191);
                for i in 0..8i32 {
                    let spriteId: u8 = CreateSprite(
                        (&raw const *sSpriteTemplate_JudgmentIcon).cast_mut(),
                        64 + i as i16 * 16,
                        84,
                        0,
                    );
                    StartSpriteAnim(&raw mut gSprites[spriteId], ANIM_ICON_LINE);
                }
                result = ARENA_RESULT_STEP_DONE;
                *state += 1;
            }
        }
        4 => {
            PlaySE(SE_ARENA_TIMEUP1);
            ShowJudgmentSprite(80, 40, ARENA_CATEGORY_MIND, B_POSITION_PLAYER_LEFT);
            ShowJudgmentSprite(160, 40, ARENA_CATEGORY_MIND, B_POSITION_OPPONENT_LEFT);
            BattleStringExpandPlaceholdersToDisplayedString(
                (*(&raw const crate::data::battle_message::gText_Judgment).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            BattlePutTextOnWindow(
                gDisplayedStringBattle.as_mut_ptr(),
                ARENA_WIN_JUDGMENT_TITLE,
            );
            *state += 1;
            result = ARENA_RESULT_STEP_DONE;
        }
        5 => {
            PlaySE(SE_ARENA_TIMEUP1);
            ShowJudgmentSprite(80, 56, ARENA_CATEGORY_SKILL, B_POSITION_PLAYER_LEFT);
            ShowJudgmentSprite(160, 56, ARENA_CATEGORY_SKILL, B_POSITION_OPPONENT_LEFT);
            BattleStringExpandPlaceholdersToDisplayedString(
                (*(&raw const crate::data::battle_message::gText_Judgment).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            BattlePutTextOnWindow(
                gDisplayedStringBattle.as_mut_ptr(),
                ARENA_WIN_JUDGMENT_TITLE,
            );
            *state += 1;
            result = ARENA_RESULT_STEP_DONE;
        }
        6 => {
            PlaySE(SE_ARENA_TIMEUP1);
            ShowJudgmentSprite(80, 72, ARENA_CATEGORY_BODY, B_POSITION_PLAYER_LEFT);
            ShowJudgmentSprite(160, 72, ARENA_CATEGORY_BODY, B_POSITION_OPPONENT_LEFT);
            BattleStringExpandPlaceholdersToDisplayedString(
                (*(&raw const crate::data::battle_message::gText_Judgment).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            BattlePutTextOnWindow(
                gDisplayedStringBattle.as_mut_ptr(),
                ARENA_WIN_JUDGMENT_TITLE,
            );
            *state += 1;
            result = ARENA_RESULT_STEP_DONE;
        }
        7 => {
            PlaySE(SE_ARENA_TIMEUP2);
            if gBattleTextBuff1[0] > gBattleTextBuff2[0] {
                result = ARENA_RESULT_PLAYER_WON;
                gBattleScripting.battler = 0;
            } else if gBattleTextBuff1[0] < gBattleTextBuff2[0] {
                result = ARENA_RESULT_PLAYER_LOST;
                gBattleScripting.battler = 1;
            } else {
                result = ARENA_RESULT_TIE;
            }
            *state += 1;
        }
        JUDGMENT_STATE_FINISHED => {
            *state += 1;
        }
        9 => {
            SetGpuReg(REG_OFFSET_WININ, 16190);
            HandleBattleWindow(5, 0, 24, 13, WINDOW_CLEAR);
            CopyBgTilemapBufferToVram(0);
            m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 256);
            BeginNormalPaletteFade(0x7FFFFF1C, 4, 8, 0, 0);
            *state += 1;
        }
        10 if gPaletteFade.active() == 0 => {
            SetGpuReg(REG_OFFSET_WININ, 16191);
            FreeSpriteTilesByTag(TAG_JUDGMENT_ICON);
            result = ARENA_RESULT_STEP_DONE;
            *state += 1;
        }
        _ => {}
    }
    result
}
unsafe fn ShowJudgmentSprite(x: u8, y: u8, category: u8, battler: u8) {
    let mut animNum: i32 = 0;
    let mut pointsPlayer: i32 = 0;
    let mut pointsOpponent: i32 = 0;
    let mindPoints: *mut i8 = (*gBattleStruct).arenaMindPoints.as_mut_ptr();
    let skillPoints: *mut i8 = (*gBattleStruct).arenaSkillPoints.as_mut_ptr();
    let hpAtStart: *mut u16 = (*gBattleStruct).arenaStartHp.as_mut_ptr();
    match category {
        ARENA_CATEGORY_MIND => {
            pointsPlayer = *mindPoints.at(battler) as i32;
            pointsOpponent = *mindPoints.at(battler as i32 ^ 1) as i32;
        }
        ARENA_CATEGORY_SKILL => {
            pointsPlayer = *skillPoints.at(battler) as i32;
            pointsOpponent = *skillPoints.at(battler as i32 ^ 1) as i32;
        }
        ARENA_CATEGORY_BODY => {
            pointsPlayer = div_i32(
                gBattleMons[battler].hp as i32 * 100,
                *hpAtStart.at(battler) as i32,
            );
            pointsOpponent = div_i32(
                gBattleMons[battler as i32 ^ 1].hp as i32 * 100,
                *hpAtStart.at(battler as i32 ^ 1) as i32,
            );
        }
        _ => {}
    }
    if pointsPlayer > pointsOpponent {
        animNum = ANIM_ICON_CIRCLE;
        if battler != 0 {
            gBattleTextBuff2[0] += 2;
        } else {
            gBattleTextBuff1[0] += 2;
        }
    } else if pointsPlayer == pointsOpponent {
        animNum = ANIM_ICON_TRIANGLE;
        if battler != 0 {
            gBattleTextBuff2[0] += 1;
        } else {
            gBattleTextBuff1[0] += 1;
        }
    } else {
        animNum = ANIM_ICON_X;
    }
    pointsPlayer = CreateSprite(
        (&raw const *sSpriteTemplate_JudgmentIcon).cast_mut(),
        x as i16,
        y as i16,
        0,
    ) as i32;
    StartSpriteAnim(&raw mut gSprites[pointsPlayer], animNum as u8);
}
pub(crate) unsafe fn SpriteCB_JudgmentIcon(sprite: *mut Sprite) {
    if gBattleCommunication[0] > JUDGMENT_STATE_FINISHED {
        DestroySprite(sprite);
    }
}
pub unsafe fn BattleArena_InitPoints() {
    let mindPoints: *mut i8 = (*gBattleStruct).arenaMindPoints.as_mut_ptr();
    let skillPoints: *mut i8 = (*gBattleStruct).arenaSkillPoints.as_mut_ptr();
    let hpAtStart: *mut u16 = (*gBattleStruct).arenaStartHp.as_mut_ptr();
    *mindPoints = 0;
    *mindPoints.at(1) = 0;
    *skillPoints = 0;
    *skillPoints.at(1) = 0;
    *hpAtStart = gBattleMons[0].hp;
    *hpAtStart.at(1) = gBattleMons[1].hp;
}
pub unsafe fn BattleArena_AddMindPoints(battler: u8) {
    (*gBattleStruct).arenaMindPoints[battler] += sMindRatings[gCurrentMove];
}
pub unsafe fn BattleArena_AddSkillPoints(battler: u8) {
    let skillPoints: *mut i8 = (*gBattleStruct).arenaSkillPoints.as_mut_ptr();
    if gHitMarker & HITMARKER_OBEYS != 0 {
        let failedMoveBits: *mut u8 = &raw mut (*gBattleStruct).alreadyStatusedMoveAttempt;
        if *failedMoveBits as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[battler]
            != 0
        {
            *failedMoveBits &= !(gBitTable[battler] as u8);
            *skillPoints.at(battler) -= 2;
        } else if gMoveResultFlags as i32 & MOVE_RESULT_NO_EFFECT != 0 {
            if gMoveResultFlags as i32 & 1 == 0 || gBattleCommunication[6] != 1 {
                *skillPoints.at(battler) -= 2;
            }
        } else if gMoveResultFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0
            && gMoveResultFlags as i32 & MOVE_RESULT_NOT_VERY_EFFECTIVE != 0
        {
            *skillPoints.at(battler) += 1;
        } else if gMoveResultFlags as i32 & MOVE_RESULT_SUPER_EFFECTIVE != 0 {
            *skillPoints.at(battler) += 2;
        } else if gMoveResultFlags as i32 & MOVE_RESULT_NOT_VERY_EFFECTIVE != 0 {
            *skillPoints.at(battler) -= 1;
        } else if gProtectStructs[battler].protected() == 0 {
            *skillPoints.at(battler) += 1;
        }
    }
}
pub unsafe fn BattleArena_DeductSkillPoints(battler: u8, stringId: u16) {
    let skillPoints: *mut i8 = (*gBattleStruct).arenaSkillPoints.as_mut_ptr();
    match stringId {
        STRINGID_PKMNSXMADEYUSELESS
        | STRINGID_PKMNSXMADEITINEFFECTIVE
        | STRINGID_PKMNSXPREVENTSFLINCHING
        | STRINGID_PKMNSXBLOCKSY2
        | STRINGID_PKMNSXPREVENTSYLOSS
        | STRINGID_PKMNSXMADEYINEFFECTIVE
        | STRINGID_PKMNSXPREVENTSBURNS
        | STRINGID_PKMNSXBLOCKSY
        | STRINGID_PKMNPROTECTEDBY
        | STRINGID_PKMNPREVENTSUSAGE
        | STRINGID_PKMNRESTOREDHPUSING
        | STRINGID_PKMNPREVENTSPARALYSISWITH
        | STRINGID_PKMNPREVENTSROMANCEWITH
        | STRINGID_PKMNPREVENTSPOISONINGWITH
        | STRINGID_PKMNPREVENTSCONFUSIONWITH
        | STRINGID_PKMNRAISEDFIREPOWERWITH
        | STRINGID_PKMNANCHORSITSELFWITH
        | STRINGID_PKMNPREVENTSSTATLOSSWITH
        | STRINGID_PKMNSTAYEDAWAKEUSING => {
            *skillPoints.at(battler) -= 3;
        }
        _ => {}
    }
}
unsafe fn UpdateHPAtStart(battler: u8) {
    let hpAtStart: *mut u16 = (*gBattleStruct).arenaStartHp.as_mut_ptr();
    *hpAtStart.at(battler) = gBattleMons[battler].hp;
    if *hpAtStart.at(battler as i32 ^ 1) > gBattleMons[battler as i32 ^ 1].hp {
        *hpAtStart.at(battler as i32 ^ 1) = gBattleMons[battler as i32 ^ 1].hp;
    }
}
pub(crate) unsafe fn InitArenaChallenge() {
    let mut isCurrent: u32 = 0;
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    (*gSaveBlock2Ptr).frontier.challengeStatus = 0;
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = 0;
    (*gSaveBlock2Ptr).frontier.set_challengePaused(FALSE);
    (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(FALSE);
    if lvlMode != FRONTIER_LVL_50 as u32 {
        isCurrent = (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & STREAK_ARENA_OPEN;
    } else {
        isCurrent = (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & STREAK_ARENA_50;
    }
    if isCurrent == 0 {
        (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode] = 0;
    }
    SetDynamicWarp(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
    );
    gTrainerBattleOpponent_A = 0;
}
pub(crate) unsafe fn GetArenaData() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        ARENA_DATA_PRIZE => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.arenaPrize;
        }
        ARENA_DATA_WIN_STREAK => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode];
        }
        ARENA_DATA_WIN_STREAK_ACTIVE => {
            if lvlMode != FRONTIER_LVL_50 as u32 {
                gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.winStreakActiveFlags as u16
                    & STREAK_ARENA_OPEN as u16;
            } else {
                gSpecialVar_Result =
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags as u16 & STREAK_ARENA_50 as u16;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn SetArenaData() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        ARENA_DATA_PRIZE => {
            (*gSaveBlock2Ptr).frontier.arenaPrize = *(&raw const crate::ffi::gSpecialVar_0x8006)
                .cast::<u16>()
                .cast_mut();
        }
        ARENA_DATA_WIN_STREAK => {
            (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode] =
                *(&raw const crate::ffi::gSpecialVar_0x8006)
                    .cast::<u16>()
                    .cast_mut();
        }
        ARENA_DATA_WIN_STREAK_ACTIVE => {
            if lvlMode != FRONTIER_LVL_50 as u32 {
                if gSpecialVar_0x8006 != 0 {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags |= STREAK_ARENA_OPEN;
                } else {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags &= 0xffffff7f;
                }
            } else {
                if gSpecialVar_0x8006 != 0 {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags |= STREAK_ARENA_50;
                } else {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags &= 0xffffffbf;
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn SaveArenaChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) unsafe fn SetArenaPrize() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    if (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode] > 41 {
        (*gSaveBlock2Ptr).frontier.arenaPrize = sLongStreakPrizeItems[Random() % 9];
    } else {
        (*gSaveBlock2Ptr).frontier.arenaPrize = sShortStreakPrizeItems[Random() % 6];
    }
}
pub(crate) unsafe fn GiveArenaPrize() {
    if AddBagItem((*gSaveBlock2Ptr).frontier.arenaPrize, 1) == 1 {
        CopyItemName(
            (*gSaveBlock2Ptr).frontier.arenaPrize,
            gStringVar1.as_mut_ptr(),
        );
        (*gSaveBlock2Ptr).frontier.arenaPrize = ITEM_NONE;
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub(crate) unsafe fn BufferArenaOpponentName() {
    GetFrontierTrainerName(gStringVar1.as_mut_ptr(), gTrainerBattleOpponent_A);
}
pub unsafe fn DrawArenaRefereeTextBox() {
    let mut width: u8 = 27;
    let palNum: u8 = 7;
    FillBgTilemapBufferRect(0, 0, 254, 14, 1, 6, palNum);
    FillBgTilemapBufferRect(0, 0, 32, 14, 1, 6, palNum);
    FillBgTilemapBufferRect(0, 0x31, 0, 14, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0x33, 1, 14, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0x34, 2, 14, width, 1, palNum);
    width += 1;
    FillBgTilemapBufferRect(0, 0x35, 28, 14, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0x36, 29, 14, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0x37, 0, 15, 1, 5, palNum);
    FillBgTilemapBufferRect(0, 0x39, 1, 15, width, 5, palNum);
    FillBgTilemapBufferRect(0, 0x3A, 29, 15, 1, 5, palNum);
    FillBgTilemapBufferRect(0, 0x831, 0, 19, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0x833, 1, 19, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0x834, 2, 19, width - 2, 1, palNum);
    FillBgTilemapBufferRect(0, 0x835, 28, 19, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0x836, 29, 19, 1, 1, palNum);
}
pub unsafe fn EraseArenaRefereeTextBox() {
    let palNum: u8 = 0;
    FillBgTilemapBufferRect(0, 3, 0, 14, 1, 1, palNum);
    let height: u8 = 4;
    FillBgTilemapBufferRect(0, 4, 1, 14, 1, 1, palNum);
    let width: u8 = 27;
    FillBgTilemapBufferRect(0, 5, 2, 14, width, 1, palNum);
    FillBgTilemapBufferRect(0, 6, 28, 14, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 7, 29, 14, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 8, 0, 15, 1, height, palNum);
    FillBgTilemapBufferRect(0, 9, 1, 15, 1, height, palNum);
    FillBgTilemapBufferRect(0, 0xA, 2, 15, width, height, palNum);
    FillBgTilemapBufferRect(0, 0xB, 28, 15, 1, height, palNum);
    FillBgTilemapBufferRect(0, 0xC, 29, 15, 1, height, palNum);
    FillBgTilemapBufferRect(0, 0xD, 0, 19, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0xE, 1, 19, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0xF, 2, 19, width, 1, palNum);
    FillBgTilemapBufferRect(0, 0x10, 28, 19, 1, 1, palNum);
    FillBgTilemapBufferRect(0, 0x11, 29, 19, 1, 1, palNum);
}
