//! Translated from `src/battle_anim_psychic.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sAffineAnim_PsychUpSpiral sAffineAnims_PsychUpSpiral gPsychUpSpiralSpriteTemplate gLightScreenWallSpriteTemplate gReflectWallSpriteTemplate gMirrorCoatWallSpriteTemplate gBarrierWallSpriteTemplate gMagicCoatWallSpriteTemplate sAnim_ReflectSparkle sAnims_ReflectSparkle gReflectSparkleSpriteTemplate sAnim_SpecialScreenSparkle sAnims_SpecialScreenSparkle gSpecialScreenSparkleSpriteTemplate gGoldRingSpriteTemplate sAnim_BentSpoon_0 sAnim_BentSpoon_1 sAnims_BentSpoon gBentSpoonSpriteTemplate sAnim_QuestionMark sAnims_QuestionMark sAffineAnim_QuestionMark sAffineAnims_QuestionMark gQuestionMarkSpriteTemplate sAffineAnim_MeditateStretchAttacker sAffineAnim_Teleport gImprisonOrbSpriteTemplate gRedXSpriteTemplate sAffineAnim_SkillSwapOrb_0 sAffineAnim_SkillSwapOrb_1 sAffineAnim_SkillSwapOrb_2 sAffineAnim_SkillSwapOrb_3 sAffineAnims_SkillSwapOrb gSkillSwapOrbSpriteTemplate sAffineAnim_LusterPurgeCircle sAffineAnims_LusterPurgeCircle gLusterPurgeCircleSpriteTemplate sAffineAnim_PsychoBoostOrb_0 sAffineAnim_PsychoBoostOrb_1 sAffineAnims_PsychoBoostOrb gPsychoBoostOrbSpriteTemplate

static gImprisonOrbSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_psychic::gImprisonOrbSpriteTemplate).cast());
static gSkillSwapOrbSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_psychic::gSkillSwapOrbSpriteTemplate).cast());
static sAffineAnim_MeditateStretchAttacker: Table<CArray<AffineAnimCmd, 4>> = Table(
    (&raw const crate::data::battle_anim_psychic::sAffineAnim_MeditateStretchAttacker).cast(),
);
static sAffineAnim_Teleport: Table<CArray<AffineAnimCmd, 3>> =
    Table((&raw const crate::data::battle_anim_psychic::sAffineAnim_Teleport).cast());
static sAffineAnims_QuestionMark: Table<CArray<*mut AffineAnimCmd, 1>> =
    Table((&raw const crate::data::battle_anim_psychic::sAffineAnims_QuestionMark).cast());

unsafe extern "C" {
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gScanlineEffect: ScanlineEffect;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn AllocOamMatrix() -> u8;
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8);
    fn ChangeSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn CloneBattlerSpriteWithBlend(a0: u8) -> i16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn DestroySpriteWithActiveSheet(a0: *mut Sprite);
    fn FreeOamMatrix(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn GetBattlerYCoordWithElevation(a0: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAnimArcTranslation(a0: *mut Sprite);
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut Sprite, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn MoveBattlerSpriteToBG(a0: u8, a1: u8, a2: u8);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PrepareAffineAnimInTaskData(a0: *mut Task, a1: u8, a2: *mut AffineAnimCmd);
    fn ResetBattleAnimBg(a0: u8);
    fn ResetSpriteRotScale(a0: u8);
    fn RunAffineAnimFromTaskData(a0: *mut Task) -> u8;
    fn RunStoredCallbackWhenAnimEnds(a0: *mut Sprite);
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn SetBattlerSpriteYOffsetFromOtherYScale(a0: u8, a1: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetSpriteRotScale(a0: u8, a1: i16, a2: i16, a3: u16);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateAnimHorizontalArc(a0: *mut Sprite) -> u8;
}

pub(crate) unsafe extern "C" fn AnimDefensiveWall(sprite: *mut Sprite) {
    let mut isContest: u8 = IsContest();
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER || isContest != 0 {
        (*sprite).oam.set_priority(2);
        (*sprite).subpriority = 200;
    }
    if isContest == 0 {
        let mut battlerCopy: u8 = 0;
        let mut battler: u8 = {
            battlerCopy = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            battlerCopy
        };
        let mut rank: u8 = GetBattlerSpriteBGPriorityRank(battler);
        let mut var0: i32 = 1;
        let mut toBG_2: u8 = (rank as i32 ^ var0 != 0) as u8;
        if IsBattlerSpriteVisible(battler) != 0 {
            MoveBattlerSpriteToBG(battler, toBG_2, FALSE);
        }
        battler = battlerCopy ^ 2;
        if IsBattlerSpriteVisible(battler) != 0 {
            MoveBattlerSpriteToBG(battler, toBG_2 ^ var0 as u8, FALSE);
        }
    }
    if isContest == 0 && IsDoubleBattle() != 0 {
        if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
            (*sprite).x = 72;
            (*sprite).y = 80;
        } else {
            (*sprite).x = 176;
            (*sprite).y = 40;
        }
    } else {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            gBattleAnimArgs[0] = -gBattleAnimArgs[0];
        }
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16 + gBattleAnimArgs[0];
        (*sprite).y =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + gBattleAnimArgs[1];
    }
    (*sprite).data[0] = 0x100 + IndexOfSpritePaletteTag(gBattleAnimArgs[2] as u16) as i16 * 16;
    if isContest != 0 {
        (*sprite).y += 9;
        (*sprite).callback = Some(AnimDefensiveWall_Step2);
        (*sprite).callback.unwrap_unchecked()(sprite);
    } else {
        (*sprite).callback = Some(AnimDefensiveWall_Step1);
    }
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step1(sprite: *mut Sprite) {
    let mut battler: u8 = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
    if (*sprite).data[7] == 0 {
        (*sprite).data[7] = 1;
        return;
    }
    if IsBattlerSpriteVisible(battler) != 0 {
        gSprites[gBattlerSpriteIds[battler]].set_invisible(TRUE as u16);
    }
    battler = battler ^ 2;
    if IsBattlerSpriteVisible(battler) != 0 {
        gSprites[gBattlerSpriteIds[battler]].set_invisible(TRUE as u16);
    }
    (*sprite).callback = Some(AnimDefensiveWall_Step2);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step2(sprite: *mut Sprite) {
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (16 - (*sprite).data[3] as u16) << 8 | (*sprite).data[3] as u16,
    );
    if (*sprite).data[3] == 13 {
        (*sprite).callback = Some(AnimDefensiveWall_Step3);
    } else {
        (*sprite).data[3] += 1;
    }
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step3(sprite: *mut Sprite) {
    let mut color: u16 = 0;
    let mut startOffset: u16 = 0;
    let mut i: i32 = 0;
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) == 2
    {
        (*sprite).data[1] = 0;
        startOffset = (*sprite).data[0] as u16;
        color = gPlttBufferFaded[startOffset as i32 + 8];
        i = 8;
        while i > 0 {
            gPlttBufferFaded[startOffset as i32 + i] = gPlttBufferFaded[startOffset as i32 + i - 1];
            i -= 1;
        }
        gPlttBufferFaded[startOffset as i32 + 1] = color;
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) == 16
        {
            (*sprite).callback = Some(AnimDefensiveWall_Step4);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step4(sprite: *mut Sprite) {
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (16 - (*sprite).data[3] as u16) << 8 | (*sprite).data[3] as u16,
    );
    if ({
        (*sprite).data[3] -= 1;
        (*sprite).data[3]
    }) == -1
    {
        if IsContest() == 0 {
            let mut battlerCopy: u8 = 0;
            let mut battler: u8 = {
                battlerCopy = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
                battlerCopy
            };
            if IsBattlerSpriteVisible(battler) != 0 {
                gSprites[gBattlerSpriteIds[battler]].set_invisible(FALSE as u16);
            }
            battler = battlerCopy ^ 2;
            if IsBattlerSpriteVisible(battler) != 0 {
                gSprites[gBattlerSpriteIds[battler]].set_invisible(FALSE as u16);
            }
        }
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(AnimDefensiveWall_Step5);
    }
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step5(sprite: *mut Sprite) {
    if IsContest() == 0 {
        let mut battlerCopy: u8 = 0;
        let mut battler: u8 = {
            battlerCopy = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            battlerCopy
        };
        let mut rank: u8 = GetBattlerSpriteBGPriorityRank(battler);
        let mut var0: i32 = 1;
        let mut toBG2: u8 = (rank as i32 ^ var0 != 0) as u8;
        if IsBattlerSpriteVisible(battler) != 0 {
            ResetBattleAnimBg(toBG2);
        }
        battler = battlerCopy ^ 2;
        if IsBattlerSpriteVisible(battler) != 0 {
            ResetBattleAnimBg(toBG2 ^ var0 as u8);
        }
    }
    (*sprite).callback = Some(DestroyAnimSprite);
}
pub(crate) unsafe extern "C" fn AnimWallSparkle(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        let mut ignoreOffsets: u32 = gBattleAnimArgs[3] as u32;
        let mut respectMonPicOffsets: u8 = FALSE;
        if ignoreOffsets == 0 {
            respectMonPicOffsets = TRUE;
        }
        if IsContest() == 0 && IsDoubleBattle() != 0 {
            if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                (*sprite).x = 72 - gBattleAnimArgs[0];
                (*sprite).y = gBattleAnimArgs[1] + 80;
            } else {
                (*sprite).x = gBattleAnimArgs[0] + 176;
                (*sprite).y = gBattleAnimArgs[1] + 40;
            }
        } else {
            if gBattleAnimArgs[2] == ANIM_ATTACKER as i16 {
                InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
            } else {
                InitSpritePosToAnimTarget(sprite, respectMonPicOffsets);
            }
        }
        (*sprite).data[0] += 1;
    } else {
        if (*sprite).animEnded() != 0 || (*sprite).affineAnimEnded() != 0 {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBentSpoon(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        StartSpriteAnim(sprite, 1);
        (*sprite).x -= 40;
        (*sprite).y += 10;
        (*sprite).data[1] = -1;
    } else {
        (*sprite).x += 40;
        (*sprite).y -= 10;
        (*sprite).data[1] = 1;
    }
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
}
pub(crate) unsafe extern "C" fn AnimQuestionMark(sprite: *mut Sprite) {
    let mut x: i16 = GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_WIDTH) / 2;
    let mut y: i16 = GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_HEIGHT) / -2;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
        x = -x;
    }
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16 + x;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16 + y;
    if (*sprite).y < 16 {
        (*sprite).y = 16;
    }
    StoreSpriteCallbackInData6(sprite, Some(AnimQuestionMark_Step1));
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
}
pub(crate) unsafe extern "C" fn AnimQuestionMark_Step1(sprite: *mut Sprite) {
    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    (*sprite).affineAnims = sAffineAnims_QuestionMark.as_ptr().cast_mut();
    (*sprite).data[0] = 0;
    InitSpriteAffineAnim(sprite);
    (*sprite).callback = Some(AnimQuestionMark_Step2);
}
pub(crate) unsafe extern "C" fn AnimQuestionMark_Step2(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if (*sprite).affineAnimEnded() != 0 {
                FreeOamMatrix((*sprite).oam.matrixNum() as u8);
                (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
                (*sprite).data[1] = 18;
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if ({
                (*sprite).data[1] -= 1;
                (*sprite).data[1]
            }) == -1
            {
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MeditateStretchAttacker(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    (*task).data[0] = spriteId as i16;
    PrepareAffineAnimInTaskData(
        task,
        spriteId,
        sAffineAnim_MeditateStretchAttacker.as_ptr().cast_mut(),
    );
    (*task).func = Some(AnimTask_MeditateStretchAttacker_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_MeditateStretchAttacker_Step(taskId: u8) {
    if RunAffineAnimFromTaskData(&raw mut gTasks[taskId]) == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Teleport(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    (*task).data[0] = spriteId as i16;
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    (*task).data[3] = (if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        4
    } else {
        8
    }) as i16;
    PrepareAffineAnimInTaskData(
        task,
        (*task).data[0] as u8,
        sAffineAnim_Teleport.as_ptr().cast_mut(),
    );
    (*task).func = Some(AnimTask_Teleport_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_Teleport_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[1] {
        0 => {
            RunAffineAnimFromTaskData(task);
            if ({
                (*task).data[2] += 1;
                (*task).data[2]
            }) > 19
            {
                (*task).data[1] += 1;
            }
        }
        1 => {
            if (*task).data[3] != 0 {
                gSprites[(*task).data[0]].y2 -= 8;
                (*task).data[3] -= 1;
            } else {
                gSprites[(*task).data[0]].set_invisible(TRUE as u16);
                gSprites[(*task).data[0]].x = 272;
                ResetSpriteRotScale((*task).data[0] as u8);
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ImprisonOrbs(taskId: u8) {
    let mut var0: u16 = 0;
    let mut var1: u16 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[3] = 16;
    (*task).data[4] = 0;
    (*task).data[13] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*task).data[14] =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    var0 = (GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_WIDTH) / 3) as u16;
    var1 = (GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_HEIGHT) / 3) as u16;
    (*task).data[12] = (if var0 > var1 { var0 } else { var1 }) as i16;
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 16);
    (*task).func = Some(AnimTask_ImprisonOrbs_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_ImprisonOrbs_Step(taskId: u8) {
    let mut i: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 8
            {
                (*task).data[1] = 0;
                spriteId = CreateSprite(
                    (&raw const *gImprisonOrbSpriteTemplate).cast_mut(),
                    (*task).data[13],
                    (*task).data[14],
                    0,
                );
                (*task).data[(*task).data[2] as i32 + 8] = spriteId as i16;
                if spriteId != MAX_SPRITES {
                    match (*task).data[2] {
                        0 => {
                            gSprites[spriteId].x2 = (*task).data[12];
                            gSprites[spriteId].y2 = -(*task).data[12];
                        }
                        1 => {
                            gSprites[spriteId].x2 = -(*task).data[12];
                            gSprites[spriteId].y2 = (*task).data[12];
                        }
                        2 => {
                            gSprites[spriteId].x2 = (*task).data[12];
                            gSprites[spriteId].y2 = (*task).data[12];
                        }
                        3 => {
                            gSprites[spriteId].x2 = -(*task).data[12];
                            gSprites[spriteId].y2 = -(*task).data[12];
                        }
                        _ => {}
                    }
                }
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) == 5
                {
                    (*task).data[0] += 1;
                }
            }
        }
        1 => {
            if (*task).data[1] as i32 & 1 != 0 {
                (*task).data[3] -= 1;
            } else {
                (*task).data[4] += 1;
            }
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                ((*task).data[4] as u16) << 8 | (*task).data[3] as u16,
            );
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) == 32
            {
                i = 8;
                while i < 13 {
                    if (*task).data[i] != MAX_SPRITES as i16 {
                        DestroySprite(&raw mut gSprites[(*task).data[i]]);
                    }
                    i += 1;
                }
                (*task).data[0] += 1;
            }
        }
        2 => {
            (*task).data[0] += 1;
        }
        3 => {
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimRedX_Step(sprite: *mut Sprite) {
    if (*sprite).data[1] as i32 > (*sprite).data[0] as i32 - 10 {
        (*sprite).set_invisible((*sprite).data[1] as u16 & 1);
    }
    if (*sprite).data[1] == (*sprite).data[0] {
        DestroyAnimSprite(sprite);
    }
    (*sprite).data[1] += 1;
}
pub(crate) unsafe extern "C" fn AnimRedX(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    }
    (*sprite).data[0] = gBattleAnimArgs[1];
    (*sprite).callback = Some(AnimRedX_Step);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SkillSwap(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if IsContest() != 0 {
        if gBattleAnimArgs[0] == ANIM_TARGET as i16 {
            (*task).data[10] = -10;
            (*task).data[11] =
                GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_RIGHT) - 8;
            (*task).data[12] =
                GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_TOP) + 8;
            (*task).data[13] =
                GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_RIGHT) - 8;
            (*task).data[14] =
                GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_TOP) + 8;
        } else {
            (*task).data[10] = 10;
            (*task).data[11] =
                GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_LEFT) + 8;
            (*task).data[12] =
                GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_BOTTOM) - 8;
            (*task).data[13] =
                GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_LEFT) + 8;
            (*task).data[14] =
                GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_BOTTOM) - 8;
        }
    } else {
        if gBattleAnimArgs[0] == 1 {
            (*task).data[10] = -10;
            (*task).data[11] =
                GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_LEFT) + 8;
            (*task).data[12] =
                GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_TOP) + 8;
            (*task).data[13] =
                GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_LEFT) + 8;
            (*task).data[14] =
                GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_TOP) + 8;
        } else {
            (*task).data[10] = 10;
            (*task).data[11] =
                GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_RIGHT) - 8;
            (*task).data[12] =
                GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_BOTTOM) - 8;
            (*task).data[13] =
                GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_RIGHT) - 8;
            (*task).data[14] =
                GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_BOTTOM) - 8;
        }
    }
    (*task).data[1] = 6;
    (*task).func = Some(AnimTask_SkillSwap_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_SkillSwap_Step(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 6
            {
                (*task).data[1] = 0;
                spriteId = CreateSprite(
                    (&raw const *gSkillSwapOrbSpriteTemplate).cast_mut(),
                    (*task).data[11],
                    (*task).data[12],
                    0,
                );
                if spriteId != MAX_SPRITES {
                    gSprites[spriteId].data[0] = 16;
                    gSprites[spriteId].data[2] = (*task).data[13];
                    gSprites[spriteId].data[4] = (*task).data[14];
                    gSprites[spriteId].data[5] = (*task).data[10];
                    InitAnimArcTranslation(&raw mut gSprites[spriteId]);
                    StartSpriteAffineAnim(&raw mut gSprites[spriteId], (*task).data[2] as u8 & 3);
                }
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) == 12
                {
                    (*task).data[0] += 1;
                }
            }
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 17
            {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimSkillSwapOrb(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ExtrasensoryDistortion(taskId: u8) {
    let mut i: i16 = 0;
    let mut yOffset: u8 = 0;
    let mut scanlineParams: ScanlineEffectParams = zeroed();
    let mut task: *mut Task = &raw mut gTasks[taskId];
    yOffset = GetBattlerYCoordWithElevation(gBattleAnimTarget);
    (*task).data[14] = yOffset as i16 - 32;
    match gBattleAnimArgs[0] {
        0 => {
            (*task).data[11] = 2;
            (*task).data[12] = 5;
            (*task).data[13] = 64;
            (*task).data[15] = yOffset as i16 + 32;
        }
        1 => {
            (*task).data[11] = 2;
            (*task).data[12] = 5;
            (*task).data[13] = 192;
            (*task).data[15] = yOffset as i16 + 32;
        }
        2 => {
            (*task).data[11] = 4;
            (*task).data[12] = 4;
            (*task).data[13] = 0;
            (*task).data[15] = yOffset as i16 + 32;
        }
        _ => {}
    }
    if (*task).data[14] < 0 {
        (*task).data[14] = 0;
    }
    if GetBattlerSpriteBGPriorityRank(gBattleAnimTarget) == 1 {
        (*task).data[10] = gBattle_BG1_X as i16;
        scanlineParams.dmaDest = 67108884 as usize as *mut u16 as *mut c_void;
    } else {
        (*task).data[10] = gBattle_BG2_X as i16;
        scanlineParams.dmaDest = 67108888 as usize as *mut u16 as *mut c_void;
    }
    i = (*task).data[14];
    while i as i32 <= (*task).data[14] as i32 + 64 {
        gScanlineEffectRegBuffers[0][i] = (*task).data[10] as u16;
        gScanlineEffectRegBuffers[1][i] = (*task).data[10] as u16;
        i += 1;
    }
    scanlineParams.dmaControl = 0xa2600001;
    scanlineParams.initState = 1;
    scanlineParams.unused9 = 0;
    ScanlineEffect_SetParams(scanlineParams);
    (*task).func = Some(AnimTask_ExtrasensoryDistortion_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_ExtrasensoryDistortion_Step(taskId: u8) {
    let mut sineIndex: i16 = 0;
    let mut i: i16 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            sineIndex = (*task).data[13];
            i = (*task).data[14];
            while i <= (*task).data[15] {
                let mut var2: i16 =
                    shr_i32(gSineTable[sineIndex] as i32, (*task).data[12] as u32) as i16;
                if var2 > 0 {
                    var2 += (*task).data[1] & 3;
                } else if var2 < 0 {
                    var2 -= (*task).data[1] & 3;
                }
                gScanlineEffectRegBuffers[0][i] = (*task).data[10] as u16 + var2 as u16;
                gScanlineEffectRegBuffers[1][i] = (*task).data[10] as u16 + var2 as u16;
                sineIndex += (*task).data[11];
                i += 1;
            }
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 23
            {
                (*task).data[0] += 1;
            }
        }
        1 => {
            gScanlineEffect.state = 3;
            (*task).data[0] += 1;
        }
        2 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_TransparentCloneGrowAndShrink(taskId: u8) {
    let mut spriteId: i16 = 0;
    let mut matrixNum: i16 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
    matrixNum = AllocOamMatrix() as i16;
    if matrixNum == 0xFF {
        DestroyAnimVisualTask(taskId);
        return;
    }
    spriteId = CloneBattlerSpriteWithBlend(gBattleAnimArgs[0] as u8);
    if spriteId < 0 {
        FreeOamMatrix(matrixNum as u8);
        DestroyAnimVisualTask(taskId);
        return;
    }
    gSprites[spriteId].callback = Some(SpriteCallbackDummy);
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    gSprites[spriteId].oam.set_matrixNum(matrixNum as u32);
    gSprites[spriteId].set_affineAnimPaused(1);
    gSprites[spriteId].subpriority += 1;
    SetSpriteRotScale(spriteId as u8, 256, 256, 0);
    CalcCenterToCornerVec(
        &raw mut gSprites[spriteId],
        gSprites[spriteId].oam.shape() as u8,
        gSprites[spriteId].oam.size() as u8,
        gSprites[spriteId].oam.affineMode() as u8,
    );
    (*task).data[13] = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16;
    (*task).data[14] = matrixNum;
    (*task).data[15] = spriteId;
    (*task).func = Some(AnimTask_TransparentCloneGrowAndShrink_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_TransparentCloneGrowAndShrink_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[1] += 4;
            (*task).data[2] = 256 - (gSineTable[(*task).data[1]] >> 1);
            SetSpriteRotScale((*task).data[15] as u8, (*task).data[2], (*task).data[2], 0);
            SetBattlerSpriteYOffsetFromOtherYScale((*task).data[15] as u8, (*task).data[13] as u8);
            if (*task).data[1] == 48 {
                (*task).data[0] += 1;
            }
        }
        1 => {
            (*task).data[1] -= 4;
            (*task).data[2] = 256 - (gSineTable[(*task).data[1]] >> 1);
            SetSpriteRotScale((*task).data[15] as u8, (*task).data[2], (*task).data[2], 0);
            SetBattlerSpriteYOffsetFromOtherYScale((*task).data[15] as u8, (*task).data[13] as u8);
            if (*task).data[1] == 0 {
                (*task).data[0] += 1;
            }
        }
        2 => {
            DestroySpriteWithActiveSheet(&raw mut gSprites[(*task).data[15]]);
            (*task).data[0] += 1;
        }
        3 => {
            FreeOamMatrix((*task).data[14] as u8);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimPsychoBoost(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
            (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
            if IsContest() != 0 {
                (*sprite).y += 12;
            }
            (*sprite).data[1] = 8;
            SetGpuReg(REG_OFFSET_BLDCNT, 16192);
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                (16 - (*sprite).data[1] as u16) << 8 | (*sprite).data[1] as u16,
            );
            (*sprite).data[0] += 1;
        }
        1 => {
            if (*sprite).affineAnimEnded() != 0 {
                PlaySE12WithPanning(SE_M_TELEPORT, BattleAnimAdjustPanning(SOUND_PAN_ATTACKER));
                ChangeSpriteAffineAnim(sprite, 1);
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            if ({
                let t1 = (*sprite).data[2];
                (*sprite).data[2] += 1;
                t1
            }) > 1
            {
                (*sprite).data[2] = 0;
                (*sprite).data[1] -= 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - (*sprite).data[1] as u16) << 8 | (*sprite).data[1] as u16,
                );
                if (*sprite).data[1] == 0 {
                    (*sprite).data[0] += 1;
                    (*sprite).set_invisible(TRUE as u16);
                }
            }
            (*sprite).data[3] += 0x380;
            (*sprite).y2 -= (*sprite).data[3] >> 8;
            (*sprite).data[3] &= 0xFF;
        }
        3 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            DestroyAnimSprite(sprite);
        }
        _ => {}
    }
}
