//! Translated from `src/battle_gfx_sfx_util.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gActiveBattler: u8;
    static mut gAnimScriptActive: u8;
    static mut gAnimScriptCallback: Option<unsafe extern "C" fn()>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static gBattleAnims_General: CArray<*mut u8, 0>;
    static gBattleAnims_Special: CArray<*mut u8, 0>;
    static mut gBattleBufferA: CArray<CArray<u8, 512>, 4>;
    static gBattleInterfaceGfx_BattleBar: CArray<u32, 0>;
    static mut gBattleMonForms: CArray<u8, 4>;
    static mut gBattleMons: CArray<BattlePokemon, 4>;
    static gBattleMoves: CArray<BattleMove, 0>;
    static gBattlePalaceNatureToMoveGroupLikelihood: CArray<CArray<u8, 4>, 25>;
    static gBattlePalaceNatureToMoveTarget: CArray<u8, 0>;
    static mut gBattleSpritesDataPtr: *mut BattleSpriteData;
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTypeFlags: u32;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlerPositions: CArray<u8, 4>;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static gBattlerSpriteTemplates: CArray<SpriteTemplate, 0>;
    static mut gBattlersCount: u8;
    static gBitTable: CArray<u32, 0>;
    static mut gContestResources: *mut ContestResources;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static gEnemyMonElevation: CArray<u8, 412>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gHealthboxSpriteIds: CArray<u8, 4>;
    static mut gIntroSlideFlags: u16;
    static mut gMPlayInfo_SE1: MusicPlayerInfo;
    static mut gMPlayInfo_SE2: MusicPlayerInfo;
    static mut gMain: Main;
    static gMonBackPicTable: CArray<CompressedSpriteSheet, 0>;
    static gMonFrontAnimsPtrTable: CArray<*mut *mut AnimCmd, 0>;
    static gMonFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gProtectStructs: CArray<ProtectStruct, 4>;
    static gSpriteSheet_EnemyShadow: CompressedSpriteSheet;
    static gSpriteTemplate_EnemyShadow: SpriteTemplate;
    static mut gSprites: CArray<Sprite, 65>;
    static gSubstituteDollBackGfx: CArray<u32, 0>;
    static gSubstituteDollFrontGfx: CArray<u32, 0>;
    static gSubstituteDollPal: CArray<u32, 0>;
    static mut gTasks: CArray<Task, 0>;
    static gTrainerBackPicPaletteTable: CArray<CompressedSpritePalette, 0>;
    static gTrainerBackPicTable: CArray<CompressedSpriteSheet, 0>;
    static gTrainerFrontPicPaletteTable: CArray<CompressedSpritePalette, 0>;
    static gTrainerFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gTransformedPersonalities: CArray<u32, 4>;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprite(a0: *mut Sprite);
    fn BattleAI_ChooseMoveOrAction() -> u8;
    fn BattleAI_SetupAIData(a0: u8);
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BufferBattlePartyCurrentOrder();
    fn CheckMoveLimitations(a0: u8, a1: u8, a2: u8) -> u8;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateBattlerHealthboxSprites(a0: u8) -> u8;
    fn CreateSafariPlayerHealthboxSprites() -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressPicFromTable_2(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32);
    fn DestroyTask(a0: u8);
    fn DummyBattleInterfaceFunc(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteDefault_Y(a0: u8) -> u8;
    fn GetHPBarLevel(a0: i16, a1: i16) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonFrontSpritePal(a0: *mut Pokemon) -> *mut u32;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetPartyIdFromBattlePartyId(a0: u8) -> u8;
    fn GetSubstituteSpriteDefault_Y(a0: u8) -> u8;
    fn HandleLoadSpecialPokePic(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32, a3: u32);
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
    );
    fn InitBattlerHealthboxCoords(a0: u8);
    fn IsBattlerSpritePresent(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsSEPlaying() -> u8;
    fn LZDecompressVram(a0: *mut u32, a1: *mut c_void);
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LaunchBattleAnimation(a0: *mut *mut u8, a1: u16, a2: u8);
    fn LaunchStatusAnimation(a0: u8, a1: u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn SetHealthboxSpriteInvisible(a0: u8);
    fn ShouldIgnoreDeoxysForm(a0: u8, a1: u8) -> u8;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn UpdateHealthboxAttribute(a0: u8, a1: *mut Pokemon, a2: u8);
    fn m4aMPlayStop(a0: *mut MusicPlayerInfo);
    fn m4aSongNumStop(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocateBattleSpritesData() {
    gBattleSpritesDataPtr = AllocZeroed(16) as *mut BattleSpriteData;
    (*gBattleSpritesDataPtr).battlerData = AllocZeroed(16) as *mut BattleSpriteInfo;
    (*gBattleSpritesDataPtr).healthBoxesData = AllocZeroed(48) as *mut BattleHealthboxInfo;
    (*gBattleSpritesDataPtr).animationData = AllocZeroed(16) as *mut BattleAnimationInfo;
    (*gBattleSpritesDataPtr).battleBars = AllocZeroed(80) as *mut BattleBarInfo;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeBattleSpritesData() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMoveAndTargetInBattlePalace() -> u16 {
    let mut i: i32 = 0;
    let mut var1: i32 = 0;
    let mut var2: i32 = 0;
    let mut chosenMoveId: i32 = -1;
    let mut moveInfo: *mut ChooseMoveStruct =
        &raw mut gBattleBufferA[gActiveBattler][4] as *mut ChooseMoveStruct;
    let mut unusableMovesBits: u8 = CheckMoveLimitations(gActiveBattler, 0, MOVE_LIMITATIONS_ALL);
    let mut percent: i32 = Random() as i32 % 100;
    i = if (*gBattleStruct).palaceFlags as u32 & gBitTable[gActiveBattler] != 0 {
        2
    } else {
        0
    };
    var2 = i;
    var1 = i + 2;
    while i < var1 {
        if gBattlePalaceNatureToMoveGroupLikelihood
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
                    && gBitTable[i] & unusableMovesBits as u32 == 0
                {
                    var1 += 1;
                }
                if GetBattlePalaceMoveGroup((*moveInfo).moves[i]) == PALACE_MOVE_GROUP_DEFENSE
                    && gBitTable[i] & unusableMovesBits as u32 == 0
                {
                    var1 += 16;
                }
                if GetBattlePalaceMoveGroup((*moveInfo).moves[i]) == PALACE_MOVE_GROUP_SUPPORT
                    && gBitTable[i] & unusableMovesBits as u32 == 0
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
        var1 = gBattleMoves[(*moveInfo).moves[chosenMoveId]].target as i32;
    }
    if var1 & MOVE_TARGET_USER as i32 != 0 {
        chosenMoveId |= (gActiveBattler as i32) << 8;
    } else if var1 == MOVE_TARGET_SELECTED as i32 {
        chosenMoveId |= GetBattlePalaceTarget() as i32;
    } else {
        chosenMoveId |=
            (GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) & 1 ^ 1) as i32) << 8;
    }
    return chosenMoveId as u16;
}
pub(crate) unsafe extern "C" fn GetBattlePalaceMoveGroup(r#move: u16) -> u8 {
    match gBattleMoves[r#move].target {
        MOVE_TARGET_SELECTED
        | MOVE_TARGET_USER_OR_SELECTED
        | 4
        | MOVE_TARGET_BOTH
        | MOVE_TARGET_FOES_AND_ALLY => {
            if gBattleMoves[r#move].power == 0 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetBattlePalaceTarget() -> u16 {
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
            return (gActiveBattler as u16 & BIT_SIDE as u16 ^ BIT_SIDE as u16) + (Random() & 2)
                << 8;
        }
        'l1: {
            let sw1: u8 = gBattlePalaceNatureToMoveTarget
                [GetNatureFromPersonality(gBattleMons[gActiveBattler].personality)];
            let mut fall = false;
            if sw1 == PALACE_TARGET_STRONGER {
                fall = true;
                if gBattleMons[opposing1].hp > gBattleMons[opposing2].hp {
                    return (opposing1 as u16) << 8;
                } else {
                    return (opposing2 as u16) << 8;
                }
            }
            if fall || sw1 == PALACE_TARGET_WEAKER {
                fall = true;
                if gBattleMons[opposing1].hp < gBattleMons[opposing2].hp {
                    return (opposing1 as u16) << 8;
                } else {
                    return (opposing2 as u16) << 8;
                }
            }
            if fall || sw1 == PALACE_TARGET_RANDOM {
                fall = true;
                return (gActiveBattler as u16 & BIT_SIDE as u16 ^ BIT_SIDE as u16)
                    + (Random() & 2)
                    << 8;
            }
        }
    }
    return (gActiveBattler as u16 ^ 1) << 8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_WaitForBattlerBallReleaseAnim(sprite: *mut Sprite) {
    let mut spriteId: u8 = (*sprite).data[1] as u8;
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
pub(crate) unsafe extern "C" fn UnusedDoBattleSpriteAffineAnim(sprite: *mut Sprite, pointless: u8) {
    (*sprite).set_animPaused(TRUE);
    (*sprite).callback = Some(SpriteCallbackDummy);
    if pointless == 0 {
        StartSpriteAffineAnim(sprite, 1);
    } else {
        StartSpriteAffineAnim(sprite, 1);
    }
    AnimateSprite(sprite);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_TrainerSlideIn(sprite: *mut Sprite) {
    if gIntroSlideFlags as i32 & 1 == 0 {
        (*sprite).x2 += (*sprite).data[0];
        if (*sprite).x2 == 0 {
            if (*sprite).y2 != 0 {
                (*sprite).callback = Some(SpriteCB_TrainerSlideVertical);
            } else {
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerSlideVertical(sprite: *mut Sprite) {
    (*sprite).y2 -= 2;
    if (*sprite).y2 == 0 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAndLaunchChosenStatusAnimation(isStatus2: u8, status: u32) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryHandleLaunchBattleTableAnimation(
    activeBattler: u8,
    atkBattler: u8,
    defBattler: u8,
    tableId: u8,
    argument: u16,
) -> u8 {
    let mut taskId: u8 = 0;
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
        gBattleAnims_General.as_ptr().cast_mut(),
        tableId as u16,
        FALSE,
    );
    taskId = CreateTask(Some(Task_ClearBitWhenBattleTableAnimDone), 10);
    gTasks[taskId].data[0] = activeBattler as i16;
    (*(*gBattleSpritesDataPtr)
        .healthBoxesData
        .at(gTasks[taskId].data[0]))
    .set_animFromTableActive(1);
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_ClearBitWhenBattleTableAnimDone(taskId: u8) {
    gAnimScriptCallback.unwrap_unchecked()();
    if gAnimScriptActive == 0 {
        (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gTasks[taskId].data[0]))
        .set_animFromTableActive(0);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn ShouldAnimBeDoneRegardlessOfSubstitute(animId: u8) -> u8 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAndLaunchSpecialAnimation(
    activeBattler: u8,
    atkBattler: u8,
    defBattler: u8,
    tableId: u8,
) {
    let mut taskId: u8 = 0;
    gBattleAnimAttacker = atkBattler;
    gBattleAnimTarget = defBattler;
    LaunchBattleAnimation(
        gBattleAnims_Special.as_ptr().cast_mut(),
        tableId as u16,
        FALSE,
    );
    taskId = CreateTask(Some(Task_ClearBitWhenSpecialAnimDone), 10);
    gTasks[taskId].data[0] = activeBattler as i16;
    (*(*gBattleSpritesDataPtr)
        .healthBoxesData
        .at(gTasks[taskId].data[0]))
    .set_specialAnimActive(1);
}
pub(crate) unsafe extern "C" fn Task_ClearBitWhenSpecialAnimDone(taskId: u8) {
    gAnimScriptCallback.unwrap_unchecked()();
    if gAnimScriptActive == 0 {
        (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gTasks[taskId].data[0]))
        .set_specialAnimActive(0);
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMoveWithoutAnimation(r#move: u16, animationTurn: u8) -> u8 {
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBattleSEPlaying(battler: u8) -> u8 {
    let mut zero: u8 = 0;
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
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleLoadOpponentMonSpriteGfx(mon: *mut Pokemon, battler: u8) {
    let mut monsPersonality: u32 = 0;
    let mut currentPersonality: u32 = 0;
    let mut otId: u32 = 0;
    let mut species: u16 = 0;
    let mut position: u8 = 0;
    let mut paletteOffset: u16 = 0;
    let mut lzPaletteData: *mut c_void = null_mut();
    monsPersonality = GetMonData2(mon, MON_DATA_PERSONALITY);
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies == SPECIES_NONE {
        species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
        currentPersonality = monsPersonality;
    } else {
        species = (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies;
        currentPersonality = gTransformedPersonalities[battler];
    }
    otId = GetMonData2(mon, MON_DATA_OT_ID);
    position = GetBattlerPosition(battler);
    HandleLoadSpecialPokePic_DontHandleDeoxys(
        (&raw const gMonFrontPicTable[species]).cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[position],
        species as i32,
        currentPersonality,
    );
    paletteOffset = 0x100 + battler as u16 * 16;
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies == SPECIES_NONE {
        lzPaletteData = GetMonFrontSpritePal(mon) as *mut c_void;
    } else {
        lzPaletteData =
            GetMonSpritePalFromSpeciesAndPersonality(species, otId, monsPersonality) as *mut c_void;
    }
    LZDecompressWram(
        lzPaletteData as *mut u32,
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
    );
    LoadPalette(
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
        paletteOffset,
        32,
    );
    LoadPalette(
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
        128 + (0x000 + battler as u16 * 16),
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
            &raw mut gPlttBufferFaded[paletteOffset] as *mut c_void,
            &raw mut gPlttBufferUnfaded[paletteOffset] as *mut c_void,
            0x4000008,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleLoadPlayerMonSpriteGfx(mon: *mut Pokemon, battler: u8) {
    let mut monsPersonality: u32 = 0;
    let mut currentPersonality: u32 = 0;
    let mut otId: u32 = 0;
    let mut species: u16 = 0;
    let mut position: u8 = 0;
    let mut paletteOffset: u16 = 0;
    let mut lzPaletteData: *mut c_void = null_mut();
    monsPersonality = GetMonData2(mon, MON_DATA_PERSONALITY);
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies == SPECIES_NONE {
        species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
        currentPersonality = monsPersonality;
    } else {
        species = (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies;
        currentPersonality = gTransformedPersonalities[battler];
    }
    otId = GetMonData2(mon, MON_DATA_OT_ID);
    position = GetBattlerPosition(battler);
    if ShouldIgnoreDeoxysForm(1, battler) == 1
        || (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != SPECIES_NONE
    {
        HandleLoadSpecialPokePic_DontHandleDeoxys(
            (&raw const gMonBackPicTable[species]).cast_mut(),
            (*gMonSpritesGfxPtr).sprites.ptr[position],
            species as i32,
            currentPersonality,
        );
    } else {
        HandleLoadSpecialPokePic(
            (&raw const gMonBackPicTable[species]).cast_mut(),
            (*gMonSpritesGfxPtr).sprites.ptr[position],
            species as i32,
            currentPersonality,
        );
    }
    paletteOffset = 0x100 + battler as u16 * 16;
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies == SPECIES_NONE {
        lzPaletteData = GetMonFrontSpritePal(mon) as *mut c_void;
    } else {
        lzPaletteData =
            GetMonSpritePalFromSpeciesAndPersonality(species, otId, monsPersonality) as *mut c_void;
    }
    LZDecompressWram(
        lzPaletteData as *mut u32,
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
    );
    LoadPalette(
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
        paletteOffset,
        32,
    );
    LoadPalette(
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
        128 + (0x000 + battler as u16 * 16),
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
            &raw mut gPlttBufferFaded[paletteOffset] as *mut c_void,
            &raw mut gPlttBufferUnfaded[paletteOffset] as *mut c_void,
            0x4000008,
        );
    }
}
pub(crate) unsafe extern "C" fn BattleGfxSfxDummy1() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleGfxSfxDummy2(species: u16) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecompressTrainerFrontPic(frontPicId: u16, battler: u8) {
    let mut position: u8 = GetBattlerPosition(battler);
    DecompressPicFromTable_2(
        (&raw const gTrainerFrontPicTable[frontPicId]).cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[position],
        SPECIES_NONE as i32,
    );
    LoadCompressedSpritePalette((&raw const gTrainerFrontPicPaletteTable[frontPicId]).cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecompressTrainerBackPic(backPicId: u16, battler: u8) {
    let mut position: u8 = GetBattlerPosition(battler);
    DecompressPicFromTable_2(
        (&raw const gTrainerBackPicTable[backPicId]).cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[position],
        SPECIES_NONE as i32,
    );
    LoadCompressedPalette(
        gTrainerBackPicPaletteTable[backPicId].data,
        0x100 + battler as u16 * 16,
        32,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleGfxSfxDummy3(gender: u8) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeTrainerFrontPicPalette(frontPicId: u16) {
    FreeSpritePaletteByTag(gTrainerFrontPicPaletteTable[frontPicId].tag);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleLoadAllHealthBoxesGfxAtOnce() {
    let mut numberOfBattlers: u8 = 0;
    let mut i: u8 = 0;
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
    i = 0;
    while i < numberOfBattlers {
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheets_HealthBar[gBattlerPositions[i]]).cast_mut(),
        );
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleLoadAllHealthBoxesGfx(state: u8) -> u8 {
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
    return retVal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattleBarGfx(unused: u8) {
    LZDecompressWram(
        gBattleInterfaceGfx_BattleBar.as_ptr().cast_mut(),
        (*gMonSpritesGfxPtr).barFontGfx as *mut c_void,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleInitAllSprites(state1: *mut u8, battler: *mut u8) -> u8 {
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
    return retVal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSpritesHealthboxAnimData() {
    memset((*gBattleSpritesDataPtr).healthBoxesData as *mut u8, 0, 48);
    memset((*gBattleSpritesDataPtr).animationData as *mut u8, 0, 16);
}
pub(crate) unsafe extern "C" fn ClearSpritesBattlerHealthboxAnimData() {
    ClearSpritesHealthboxAnimData();
    memset((*gBattleSpritesDataPtr).battlerData as *mut u8, 0, 16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyAllBattleSpritesInvisibilities() {
    let mut i: i32 = 0;
    i = 0;
    while i < gBattlersCount as i32 {
        (*(*gBattleSpritesDataPtr).battlerData.at(i))
            .set_invisible(gSprites[gBattlerSpriteIds[i]].invisible());
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyBattleSpriteInvisibility(battler: u8) {
    (*(*gBattleSpritesDataPtr).battlerData.at(battler))
        .set_invisible(gSprites[gBattlerSpriteIds[battler]].invisible());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleSpeciesGfxDataChange(battlerAtk: u8, battlerDef: u8, castform: u8) {
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
                &raw mut gPlttBufferFaded[paletteOffset] as *mut c_void,
                &raw mut gPlttBufferUnfaded[paletteOffset] as *mut c_void,
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
                (&raw const gMonBackPicTable[targetSpecies]).cast_mut(),
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
                    (&raw const gMonBackPicTable[targetSpecies]).cast_mut(),
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
                    (&raw const gMonFrontPicTable[targetSpecies]).cast_mut(),
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
                        let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                        volatile_write(dmaRegs, _src as usize as u32);
                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                        volatile_write(dmaRegs.at(2), 0x84000000 | _size / 4);
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
            gDecompressionBuffer.as_mut_ptr() as *mut c_void,
        );
        LoadPalette(
            gDecompressionBuffer.as_mut_ptr() as *mut c_void,
            paletteOffset,
            32,
        );
        if targetSpecies == SPECIES_CASTFORM {
            gSprites[gBattlerSpriteIds[battlerAtk]].anims = gMonFrontAnimsPtrTable[targetSpecies];
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
            &raw mut gPlttBufferFaded[paletteOffset] as *mut c_void,
            &raw mut gPlttBufferUnfaded[paletteOffset] as *mut c_void,
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
pub unsafe extern "C" fn BattleLoadSubstituteOrMonSpriteGfx(battler: u8, loadMonSprite: u8) {
    let mut i: i32 = 0;
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
                gSubstituteDollBackGfx.as_ptr().cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[position],
            );
        } else if GetBattlerSide(battler) != B_SIDE_PLAYER {
            LZDecompressVram(
                gSubstituteDollFrontGfx.as_ptr().cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[position],
            );
        } else {
            LZDecompressVram(
                gSubstituteDollBackGfx.as_ptr().cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[position],
            );
        }
        i = 1;
        while i < 4 {
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
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x84000000 | _size / 4);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
            i += 1;
        }
        palOffset = 0x100 + battler as i32 * 16;
        LoadCompressedPalette(gSubstituteDollPal.as_ptr().cast_mut(), palOffset as u16, 32);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattleMonGfxAndAnimate(battler: u8, loadMonSprite: u8, spriteId: u8) {
    BattleLoadSubstituteOrMonSpriteGfx(battler, loadMonSprite);
    StartSpriteAnim(&raw mut gSprites[spriteId], gBattleMonForms[battler]);
    if loadMonSprite == 0 {
        gSprites[spriteId].y = GetSubstituteSpriteDefault_Y(battler) as i16;
    } else {
        gSprites[spriteId].y = GetBattlerSpriteDefault_Y(battler) as i16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetBehindSubstituteSpriteBit(battler: u8, r#move: u16) {
    if r#move == MOVE_SUBSTITUTE {
        (*(*gBattleSpritesDataPtr).battlerData.at(battler)).set_behindSubstitute(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBehindSubstituteBit(battler: u8) {
    (*(*gBattleSpritesDataPtr).battlerData.at(battler)).set_behindSubstitute(0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleLowHpMusicChange(mon: *mut Pokemon, battler: u8) {
    let mut hp: u16 = GetMonData2(mon, MON_DATA_HP) as u16;
    let mut maxHP: u16 = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
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
            return;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleStopLowHpSound() {
    let mut playerBattler: u8 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
    (*(*gBattleSpritesDataPtr).battlerData.at(playerBattler)).set_lowHpSong(0);
    if IsDoubleBattle() != 0 {
        (*(*gBattleSpritesDataPtr)
            .battlerData
            .at(playerBattler as i32 ^ 2))
        .set_lowHpSong(0);
    }
    m4aSongNumStop(SE_LOW_HEALTH);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonHPBarLevel(mon: *mut Pokemon) -> u8 {
    let mut hp: u16 = GetMonData2(mon, MON_DATA_HP) as u16;
    let mut maxHP: u16 = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
    return GetHPBarLevel(hp as i16, maxHP as i16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleBattleLowHpMusicChange() {
    if gMain.inBattle() != 0 {
        let mut playerBattler1: u8 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        let mut playerBattler2: u8 = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
        let mut battler1PartyId: u8 =
            GetPartyIdFromBattlePartyId(gBattlerPartyIndexes[playerBattler1] as u8);
        let mut battler2PartyId: u8 =
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattlerSpriteAffineMode(affineMode: u8) {
    let mut i: i32 = 0;
    i = 0;
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
pub unsafe extern "C" fn LoadAndCreateEnemyShadowSprites() {
    let mut battler: u8 = 0;
    LoadCompressedSpriteSheet((&raw const gSpriteSheet_EnemyShadow).cast_mut());
    battler = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
    (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId = CreateSprite(
        (&raw const gSpriteTemplate_EnemyShadow).cast_mut(),
        GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16,
        GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as i16 + 29,
        0xC8,
    );
    gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].data[0] =
        battler as i16;
    if IsDoubleBattle() != 0 {
        battler = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId = CreateSprite(
            (&raw const gSpriteTemplate_EnemyShadow).cast_mut(),
            GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16,
            GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as i16 + 29,
            0xC8,
        );
        gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].data[0] =
            battler as i16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_EnemyShadow(shadowSprite: *mut Sprite) {
    let mut invisible: u8 = FALSE;
    let mut battler: u8 = (*shadowSprite).data[0] as u8;
    let mut battlerSprite: *mut Sprite = &raw mut gSprites[gBattlerSpriteIds[battler]];
    if (*battlerSprite).inUse() == 0 || IsBattlerSpritePresent(battler) == 0 {
        (*shadowSprite).callback = Some(SpriteCB_SetInvisible);
        return;
    }
    if gAnimScriptActive != 0 || (*battlerSprite).invisible() != 0 {
        invisible = TRUE;
    } else if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != SPECIES_NONE
        && gEnemyMonElevation[(*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies]
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
pub unsafe extern "C" fn SpriteCB_SetInvisible(sprite: *mut Sprite) {
    (*sprite).set_invisible(TRUE as u16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattlerShadowSpriteCallback(battler: u8, mut species: u16) {
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        return;
    }
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != SPECIES_NONE {
        species = (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies;
    }
    if gEnemyMonElevation[species] != 0 {
        gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].callback =
            Some(SpriteCB_EnemyShadow);
    } else {
        gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].callback =
            Some(SpriteCB_SetInvisible);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideBattlerShadowSprite(battler: u8) {
    gSprites[(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).shadowSpriteId].callback =
        Some(SpriteCB_SetInvisible);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillAroundBattleWindows() {
    let mut vramPtr: *mut u16 = 100663872 as usize as *mut u16;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < 9 {
        j = 0;
        while j < 16 {
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
            j += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearTemporarySpeciesSpriteData(battler: u8, dontClearSubstitute: u8) {
    (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies = SPECIES_NONE;
    gBattleMonForms[battler] = 0;
    if dontClearSubstitute == 0 {
        ClearBehindSubstituteBit(battler);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocateMonSpritesGfx() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    gMonSpritesGfxPtr = null_mut();
    gMonSpritesGfxPtr = AllocZeroed(384) as *mut MonSpritesGfx;
    (*gMonSpritesGfxPtr).firstDecompressed = AllocZeroed(32768);
    i = 0;
    while i < MAX_BATTLERS_COUNT {
        (*gMonSpritesGfxPtr).sprites.ptr[i] = ((*gMonSpritesGfxPtr).firstDecompressed as *mut u8)
            .at(i as i32 * MON_PIC_SIZE as i32 * 4)
            as *mut c_void;
        *(*gMonSpritesGfxPtr).templates.as_mut_ptr().at(i) = gBattlerSpriteTemplates[i];
        j = 0;
        while j < 4 {
            (*gMonSpritesGfxPtr).frameImages[i][j].data =
                ((*gMonSpritesGfxPtr).sprites.ptr[i] as *mut u8).at(j as i32 * MON_PIC_SIZE as i32)
                    as *mut c_void;
            (*gMonSpritesGfxPtr).frameImages[i][j].size = MON_PIC_SIZE;
            j += 1;
        }
        (*gMonSpritesGfxPtr).templates[i].images = (*gMonSpritesGfxPtr).frameImages[i].as_mut_ptr();
        i += 1;
    }
    (*gMonSpritesGfxPtr).barFontGfx = AllocZeroed(0x1000) as *mut u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMonSpritesGfx() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldPlayNormalMonCry(mon: *mut Pokemon) -> u32 {
    let mut hp: i16 = 0;
    let mut maxHP: i16 = 0;
    let mut barLevel: i32 = 0;
    if GetMonData2(mon, MON_DATA_STATUS) & 4095 != 0 {
        return FALSE as u32;
    }
    hp = GetMonData2(mon, MON_DATA_HP) as i16;
    maxHP = GetMonData2(mon, MON_DATA_MAX_HP) as i16;
    barLevel = GetHPBarLevel(hp, maxHP) as i32;
    if barLevel <= HP_BAR_YELLOW as i32 {
        return FALSE as u32;
    }
    return TRUE as u32;
}
