//! Translated from `src/battle_controller_player_partner.c` by tools/rustport/c2rs.py.
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
    clippy::unnecessary_cast,
    unused_assignments
)]

use crate::battle_ai_script_commands::{BattleAI_ChooseMoveOrAction, BattleAI_SetupAIData};
use crate::battle_ai_switch_items::{AI_TrySwitchOrUseItem, GetMostSuitableMonToSwitchInto};
use crate::battle_anim::{
    DoMoveAnim, gAnimDisableStructPtr, gAnimFriendship, gAnimMoveDmg, gAnimMovePower,
    gAnimMoveTurn, gAnimScriptActive, gAnimScriptCallback, gWeatherMoveAnim,
};
use crate::battle_anim_mons::{
    GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide, GetBattlerSpriteCoord,
    GetBattlerSpriteDefault_Y, GetBattlerSpriteSubpriority, IsBattlerSpritePresent, IsDoubleBattle,
    SetSpritePrimaryCoordsFromSecondaryCoords, StartAnimLinearTranslation,
    StoreSpriteCallbackInData6,
};
use crate::battle_anim_throw::TryShinyAnimation;
use crate::battle_controller_player::{
    BattleControllerDummy, SetBattleEndCallbacks, SpriteCB_FreePlayerSpriteLoadMonSprite,
    Task_PlayerController_RestoreBgmAfterCry,
};
use crate::battle_controllers::{
    BtlController_EmitChosenMonReturnValue, BtlController_EmitDataTransfer,
    BtlController_EmitTwoReturnValues, PrepareBufferDataTransferLink, gUnusedControllerStruct,
};
use crate::battle_gfx_sfx_util::{
    BattleGfxSfxDummy2, BattleGfxSfxDummy3, BattleLoadPlayerMonSpriteGfx, BattleStopLowHpSound,
    ClearTemporarySpeciesSpriteData, CopyAllBattleSpritesInvisibilities,
    CopyBattleSpriteInvisibility, DecompressTrainerBackPic, DecompressTrainerFrontPic,
    HandleLowHpMusicChange, InitAndLaunchChosenStatusAnimation, InitAndLaunchSpecialAnimation,
    IsBattleSEPlaying, IsMoveWithoutAnimation, LoadBattleBarGfx, SetBattlerSpriteAffineMode,
    SpriteCB_TrainerSlideIn, SpriteCB_WaitForBattlerBallReleaseAnim,
    TryHandleLaunchBattleTableAnimation, TrySetBehindSubstituteSpriteBit,
};
use crate::battle_interface::{
    CreatePartyStatusSummarySprites, MoveBattleBar, SetBattleBarStruct,
    SetHealthboxSpriteInvisible, SetHealthboxSpriteVisible, Task_HidePartyStatusSummary,
    UpdateHealthboxAttribute, UpdateHpTextInHealthbox,
};
use crate::battle_intro::HandleIntroSlide;
use crate::battle_main::{
    SpriteCB_FaintSlideAnim, gAbsentBattlerFlags, gActiveBattler, gBattle_BG0_X, gBattle_BG0_Y,
    gBattleControllerExecFlags, gBattleOutcome, gBattleSpritesDataPtr, gBattleStruct,
    gBattleTypeFlags, gBattlerControllerFuncs, gBattlerTarget, gDoingBattleAnim, gIntroSlideFlags,
    gTransformedPersonalities,
};
use crate::battle_main::{
    gBattleBufferA, gBattleControllerData, gBattleMonForms, gBattlerPartyIndexes,
    gBattlerSpriteIds, gBattlerStatusSummaryTaskId, gDisplayedStringBattle, gHealthboxSpriteIds,
};
use crate::battle_message::{BattlePutTextOnWindow, BufferStringBattle};
use crate::battle_setup::gPartnerTrainerId;
use crate::battle_tower::GetFrontierTrainerFrontSpriteId;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::link::GetMultiplayerId;
use crate::m4a::m4aSongNumStop;
use crate::palette::{BeginFastPaletteFade, LoadCompressedPalette};
use crate::pokeball::{
    DoHitAnimHealthboxEffect, DoPokeballSendOutAnimation, StartHealthboxSlideIn,
};
use crate::pokemon::{
    CalculateMonStats, GetMonData2, GetMonData3, SetMonData, SetMultiuseSpriteTemplateToPokemon,
    SetMultiuseSpriteTemplateToTrainerBack, SetMultiuseSpriteTemplateToTrainerFront,
    gMultiuseSpriteTemplate, gPlayerParty,
};
use crate::sound::{
    FadeOutMapMusic, IsCryPlayingOrClearCrySongs, PlayBGM, PlayCry_ByMode, PlayFanfare, PlaySE,
    PlaySE12WithPanning,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AllocSpritePalette, FreeOamMatrix, FreeSpritePaletteByTag, FreeSpriteTilesByTag,
    IndexOfSpritePaletteTag,
};
use crate::task::DestroyTask;
use crate::task::{task_get, task_set, task_set_func};
use crate::text::IsTextPrinterActive;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
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
/// `StringCopy_Nickname` with this module's view of its types.
#[inline]
unsafe fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy_Nickname(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tExpTask_monId: usize = 0;
const tExpTask_gainedExp: usize = 1;
const sSpeedY: usize = 2;
const tExpTask_bank: usize = 2;
const tExpTask_frames: usize = 10;
// Data tables (translate with cdata.py): sPlayerPartnerBufferCommands sUnused

static sPlayerPartnerBufferCommands: Table<CArray<Option<unsafe fn()>, 57>> = Table(
    (&raw const crate::data::battle_controller_player_partner::sPlayerPartnerBufferCommands).cast(),
);

pub(crate) unsafe fn PlayerPartnerDummy() {}
pub unsafe fn SetControllerToPlayerPartner() {
    gBattlerControllerFuncs[gActiveBattler] = Some(PlayerPartnerBufferRunCommand);
}
pub(crate) unsafe fn PlayerPartnerBufferRunCommand() {
    if gBattleControllerExecFlags
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler]
        != 0
    {
        if gBattleBufferA[gActiveBattler][0] < 57 {
            sPlayerPartnerBufferCommands[gBattleBufferA[gActiveBattler][0]].unwrap_unchecked()();
        } else {
            PlayerPartnerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe fn CompleteOnBattlerSpriteCallbackDummy() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeTrainerSpriteAfterSlide() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        BattleGfxSfxDummy3(MALE);
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn Intro_DelayAndEnd() {
    if ({
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay -= 1;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay
    }) == 255
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay = 0;
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn Intro_WaitForHealthbox() {
    let mut finished: u32 = FALSE as u32;
    if IsDoubleBattle() == 0 || IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            finished = TRUE as u32;
        }
    } else {
        if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && gSprites[gHealthboxSpriteIds[gActiveBattler as i32 ^ 2]].callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            finished = TRUE as u32;
        }
    }
    if IsCryPlayingOrClearCrySongs() != 0 {
        finished = FALSE as u32;
    }
    if finished != 0 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay = 3;
        gBattlerControllerFuncs[gActiveBattler] = Some(Intro_DelayAndEnd);
    }
}
pub(crate) unsafe fn Intro_ShowHealthbox() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gActiveBattler as i32 ^ 2))
        .ballAnimActive()
            == 0
        && gSprites[gBattleControllerData[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && gSprites[gBattlerSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && ({
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay += 1;
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay
        }) != 1
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay = 0;
        if IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
            DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler as i32 ^ 2]]);
            UpdateHealthboxAttribute(
                gHealthboxSpriteIds[gActiveBattler as i32 ^ 2],
                &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler as i32 ^ 2]],
                HEALTHBOX_ALL,
            );
            StartHealthboxSlideIn(gActiveBattler ^ 2);
            SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler as i32 ^ 2]);
        }
        DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler]]);
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler);
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
        (*(*gBattleSpritesDataPtr).animationData).set_introAnimActive(FALSE);
        gBattlerControllerFuncs[gActiveBattler] = Some(Intro_WaitForHealthbox);
    }
}
pub(crate) unsafe fn WaitForMonAnimAfterLoad() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].animEnded() != 0
        && gSprites[gBattlerSpriteIds[gActiveBattler]].x2 == 0
    {
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnHealthbarDone() {
    let hpValue: i16 = MoveBattleBar(
        gActiveBattler,
        gHealthboxSpriteIds[gActiveBattler],
        HEALTH_BAR,
        0,
    ) as i16;
    SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
    if hpValue != -1 {
        UpdateHpTextInHealthbox(gHealthboxSpriteIds[gActiveBattler], hpValue, HP_CURRENT);
    } else {
        HandleLowHpMusicChange(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            gActiveBattler,
        );
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn Task_GiveExpToMon(taskId: u8) {
    let monId: u32 = task_get(taskId, tExpTask_monId) as u8 as u32;
    let battler: u8 = task_get(taskId, tExpTask_bank) as u8;
    let mut gainedExp: i16 = task_get(taskId, tExpTask_gainedExp);
    if IsDoubleBattle() == TRUE || monId != gBattlerPartyIndexes[battler] as u32 {
        let mon: *mut Pokemon = &raw mut gPlayerParty[monId];
        let species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
        let level: u8 = GetMonData2(mon, MON_DATA_LEVEL) as u8;
        let mut currExp: u32 = GetMonData2(mon, MON_DATA_EXP);
        let mut nextLvlExp: u32 = (*(&raw const crate::data::pokemon::gExperienceTables)
            .cast::<CArray<CArray<u32, 101>, 0>>())
            [(*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [species]
                .growthRate][level as i32 + 1];
        if currExp + gainedExp as u32 >= nextLvlExp {
            SetMonData(mon, MON_DATA_EXP, &raw mut nextLvlExp as *mut c_void);
            CalculateMonStats(mon);
            gainedExp -= nextLvlExp as i16 - currExp as i16;
            let savedActiveBank: u8 = gActiveBattler;
            gActiveBattler = battler;
            BtlController_EmitTwoReturnValues(
                B_COMM_TO_ENGINE,
                RET_VALUE_LEVELED_UP,
                gainedExp as u16,
            );
            gActiveBattler = savedActiveBank;
            if IsDoubleBattle() == TRUE
                && (monId as u16 == gBattlerPartyIndexes[battler]
                    || monId as u16 == gBattlerPartyIndexes[battler as i32 ^ 2])
            {
                task_set_func(taskId, Some(Task_LaunchLvlUpAnim));
            } else {
                task_set_func(taskId, Some(DestroyExpTaskAndCompleteOnInactiveTextPrinter));
            }
        } else {
            currExp += gainedExp as u32;
            SetMonData(mon, MON_DATA_EXP, &raw mut currExp as *mut c_void);
            gBattlerControllerFuncs[battler] = Some(CompleteOnInactiveTextPrinter);
            DestroyTask(taskId);
        }
    } else {
        task_set_func(taskId, Some(Task_PrepareToGiveExpWithExpBar));
    }
}
pub(crate) unsafe fn Task_PrepareToGiveExpWithExpBar(taskId: u8) {
    let monIndex: u8 = task_get(taskId, tExpTask_monId) as u8;
    let gainedExp: i32 = task_get(taskId, tExpTask_gainedExp) as i32;
    let battler: u8 = task_get(taskId, tExpTask_bank) as u8;
    let mon: *mut Pokemon = &raw mut gPlayerParty[monIndex];
    let level: u8 = GetMonData2(mon, MON_DATA_LEVEL) as u8;
    let species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    let mut exp: u32 = GetMonData2(mon, MON_DATA_EXP);
    let currLvlExp: u32 = (*(&raw const crate::data::pokemon::gExperienceTables)
        .cast::<CArray<CArray<u32, 101>, 0>>())[(*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[species]
        .growthRate][level];
    exp -= currLvlExp;
    let expToNextLvl: u32 = (*(&raw const crate::data::pokemon::gExperienceTables)
        .cast::<CArray<CArray<u32, 101>, 0>>())[(*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[species]
        .growthRate][level as i32 + 1]
        - currLvlExp;
    SetBattleBarStruct(
        battler,
        gHealthboxSpriteIds[battler],
        expToNextLvl as i32,
        exp as i32,
        -gainedExp,
    );
    PlaySE(SE_EXP);
    task_set_func(taskId, Some(Task_GiveExpWithExpBar));
}
pub(crate) unsafe fn Task_GiveExpWithExpBar(taskId: u8) {
    if task_get(taskId, tExpTask_frames) < 13 {
        task_set(
            taskId,
            tExpTask_frames,
            task_get(taskId, tExpTask_frames) + 1,
        );
    } else {
        let monId: u8 = task_get(taskId, tExpTask_monId) as u8;
        let mut gainedExp: i16 = task_get(taskId, tExpTask_gainedExp);
        let battler: u8 = task_get(taskId, tExpTask_bank) as u8;
        let r4: i16 = MoveBattleBar(battler, gHealthboxSpriteIds[battler], EXP_BAR, 0) as i16;
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[battler]);
        if r4 == -1 {
            m4aSongNumStop(SE_EXP);
            let level: u8 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u8;
            let mut currExp: i32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_EXP) as i32;
            let species: u16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES) as u16;
            let mut expOnNextLvl: i32 = (*(&raw const crate::data::pokemon::gExperienceTables)
                .cast::<CArray<CArray<u32, 101>, 0>>())
                [(*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .growthRate][level as i32 + 1] as i32;
            if currExp + gainedExp as i32 >= expOnNextLvl {
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_EXP,
                    &raw mut expOnNextLvl as *mut c_void,
                );
                CalculateMonStats(&raw mut gPlayerParty[monId]);
                gainedExp -= expOnNextLvl as i16 - currExp as i16;
                let savedActiveBank: u8 = gActiveBattler;
                gActiveBattler = battler;
                BtlController_EmitTwoReturnValues(
                    B_COMM_TO_ENGINE,
                    RET_VALUE_LEVELED_UP,
                    gainedExp as u16,
                );
                gActiveBattler = savedActiveBank;
                task_set_func(taskId, Some(Task_LaunchLvlUpAnim));
            } else {
                currExp += gainedExp as i32;
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_EXP,
                    &raw mut currExp as *mut c_void,
                );
                gBattlerControllerFuncs[battler] = Some(CompleteOnInactiveTextPrinter);
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe fn Task_LaunchLvlUpAnim(taskId: u8) {
    let mut battler: u8 = task_get(taskId, tExpTask_bank) as u8;
    let monIndex: u8 = task_get(taskId, tExpTask_monId) as u8;
    if IsDoubleBattle() == TRUE && monIndex as u16 == gBattlerPartyIndexes[battler as i32 ^ 2] {
        battler ^= BIT_FLANK;
    }
    InitAndLaunchSpecialAnimation(battler, battler, battler, B_ANIM_LVL_UP);
    task_set_func(taskId, Some(Task_UpdateLvlInHealthbox));
}
pub(crate) unsafe fn Task_UpdateLvlInHealthbox(taskId: u8) {
    let battler: u8 = task_get(taskId, tExpTask_bank) as u8;
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).specialAnimActive() == 0 {
        let monIndex: u8 = task_get(taskId, tExpTask_monId) as u8;
        GetMonData2(&raw mut gPlayerParty[monIndex], MON_DATA_LEVEL);
        if IsDoubleBattle() == TRUE && monIndex as u16 == gBattlerPartyIndexes[battler as i32 ^ 2] {
            UpdateHealthboxAttribute(
                gHealthboxSpriteIds[battler as i32 ^ 2],
                &raw mut gPlayerParty[monIndex],
                HEALTHBOX_ALL,
            );
        } else {
            UpdateHealthboxAttribute(
                gHealthboxSpriteIds[battler],
                &raw mut gPlayerParty[monIndex],
                HEALTHBOX_ALL,
            );
        }
        task_set_func(taskId, Some(DestroyExpTaskAndCompleteOnInactiveTextPrinter));
    }
}
pub(crate) unsafe fn DestroyExpTaskAndCompleteOnInactiveTextPrinter(taskId: u8) {
    let monIndex: u8 = task_get(taskId, tExpTask_monId) as u8;
    GetMonData2(&raw mut gPlayerParty[monIndex], MON_DATA_LEVEL);
    let battler: u8 = task_get(taskId, tExpTask_bank) as u8;
    gBattlerControllerFuncs[battler] = Some(CompleteOnInactiveTextPrinter);
    DestroyTask(taskId);
}
pub(crate) unsafe fn FreeMonSpriteAfterFaintAnim() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].y as i32
        + gSprites[gBattlerSpriteIds[gActiveBattler]].y2 as i32
        > DISPLAY_HEIGHT as i32
    {
        let species: u16 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_SPECIES,
        ) as u16;
        BattleGfxSfxDummy2(species);
        FreeOamMatrix(gSprites[gBattlerSpriteIds[gActiveBattler]].oam.matrixNum() as u8);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeMonSpriteAfterSwitchOutAnim() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0 {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnInactiveTextPrinter2() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn DoHitAnimBlinkSpriteEffect() {
    let spriteId: u8 = gBattlerSpriteIds[gActiveBattler];
    if gSprites[spriteId].data[1] == 32 {
        gSprites[spriteId].data[1] = 0;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gDoingBattleAnim = FALSE;
        PlayerPartnerBufferExecCompleted();
    } else {
        if gSprites[spriteId].data[1] % 4 == 0 {
            gSprites[spriteId].set_invisible(gSprites[spriteId].invisible() ^ 1);
        }
        gSprites[spriteId].data[1] += 1;
    }
}
pub(crate) unsafe fn SwitchIn_ShowSubstitute() {
    if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        CopyBattleSpriteInvisibility(gActiveBattler);
        if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute() != 0 {
            InitAndLaunchSpecialAnimation(
                gActiveBattler,
                gActiveBattler,
                gActiveBattler,
                B_ANIM_MON_TO_SUBSTITUTE,
            );
        }
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_WaitAndEnd);
    }
}
pub(crate) unsafe fn SwitchIn_WaitAndEnd() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0
        && gSprites[gBattlerSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn SwitchIn_ShowHealthbox() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).finishedShinyMonAnim() != 0 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_triedShinyMonAnim(FALSE);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_finishedShinyMonAnim(FALSE);
        FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
        FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
        CreateTask(Some(Task_PlayerController_RestoreBgmAfterCry), 10);
        HandleLowHpMusicChange(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            gActiveBattler,
        );
        StartSpriteAnim(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]], 0);
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler);
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_ShowSubstitute);
    }
}
pub(crate) unsafe fn SwitchIn_TryShinyAnim() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).triedShinyMonAnim() == 0
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
    {
        TryShinyAnimation(
            gActiveBattler,
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        );
    }
    if gSprites[gBattleControllerData[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
    {
        DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler]]);
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_ShowHealthbox);
    }
}
unsafe fn PlayerPartnerBufferExecCompleted() {
    gBattlerControllerFuncs[gActiveBattler] = Some(PlayerPartnerBufferRunCommand);
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        let mut playerId: u8 = GetMultiplayerId();
        PrepareBufferDataTransferLink(B_COMM_CONTROLLER_IS_DONE, 4, &raw mut playerId);
        gBattleBufferA[gActiveBattler][0] = CONTROLLER_TERMINATOR_NOP;
    } else {
        gBattleControllerExecFlags &= !gBitTable[gActiveBattler];
    }
}
pub(crate) unsafe fn CompleteOnFinishedStatusAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).statusAnimActive() == 0 {
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnFinishedBattleAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animFromTableActive() == 0 {
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn PlayerPartnerHandleGetMonData() {
    let mut monData: CArray<u8, 256> = zeroed();
    let mut size: u32 = 0;
    let mut monToCheck: u8 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        size += CopyPlayerPartnerMonData(
            gBattlerPartyIndexes[gActiveBattler] as u8,
            monData.as_mut_ptr(),
        );
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        for i in 0..PARTY_SIZE {
            if monToCheck as i32 & 1 != 0 {
                size += CopyPlayerPartnerMonData(i as u8, monData.as_mut_ptr().at(size));
            }
            monToCheck >>= 1;
        }
    }
    BtlController_EmitDataTransfer(
        B_COMM_TO_ENGINE,
        size as u16,
        monData.as_mut_ptr() as *mut c_void,
    );
    PlayerPartnerBufferExecCompleted();
}
unsafe fn CopyPlayerPartnerMonData(monId: u8, dst: *mut u8) -> u32 {
    let mut battleMon: BattlePokemon = zeroed();
    let mut moveData: MovePPInfo = zeroed();
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut src: *mut u8 = null_mut();
    let mut data16: i16 = 0;
    let mut data32: u32 = 0;
    let mut size: i32 = 0;
    match gBattleBufferA[gActiveBattler][1] {
        REQUEST_ALL_BATTLE => {
            battleMon.species = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES) as u16;
            battleMon.item = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HELD_ITEM) as u16;
            for size in 0..MAX_MON_MOVES {
                battleMon.moves[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MOVE1 + size) as u16;
                battleMon.pp[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP1 + size) as u8;
            }
            battleMon.ppBonuses =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP_BONUSES) as u8;
            battleMon.friendship =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_FRIENDSHIP) as u8;
            battleMon.experience = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_EXP);
            battleMon.set_hpIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP_IV));
            battleMon.set_attackIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK_IV));
            battleMon.set_defenseIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF_IV));
            battleMon.set_speedIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED_IV));
            battleMon.set_spAttackIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK_IV));
            battleMon.set_spDefenseIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF_IV));
            battleMon.personality = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PERSONALITY);
            battleMon.status1 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_STATUS);
            battleMon.level = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u8;
            battleMon.hp = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP) as u16;
            battleMon.maxHP = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MAX_HP) as u16;
            battleMon.attack = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK) as u16;
            battleMon.defense = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF) as u16;
            battleMon.speed = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED) as u16;
            battleMon.spAttack = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK) as u16;
            battleMon.spDefense = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF) as u16;
            battleMon.set_isEgg(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_IS_EGG));
            battleMon.set_abilityNum(GetMonData2(
                &raw mut gPlayerParty[monId],
                MON_DATA_ABILITY_NUM,
            ));
            battleMon.otId = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_OT_ID);
            GetMonData3(
                &raw mut gPlayerParty[monId],
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
            StringCopy_Nickname(battleMon.nickname.as_mut_ptr(), nickname.as_mut_ptr());
            GetMonData3(
                &raw mut gPlayerParty[monId],
                MON_DATA_OT_NAME,
                battleMon.otName.as_mut_ptr(),
            );
            src = &raw mut battleMon as *mut u8;
            for size in 0..88i32 {
                *dst.at(size) = *src.at(size);
            }
        }
        REQUEST_SPECIES_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_HELDITEM_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HELD_ITEM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MOVES_PP_BATTLE => {
            for size in 0..MAX_MON_MOVES {
                moveData.moves[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MOVE1 + size) as u16;
                moveData.pp[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP1 + size) as u8;
            }
            moveData.ppBonuses =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP_BONUSES) as u8;
            src = &raw mut moveData as *mut u8;
            for size in 0..16i32 {
                *dst.at(size) = *src.at(size);
            }
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            data16 = GetMonData2(
                &raw mut gPlayerParty[monId],
                MON_DATA_MOVE1 + gBattleBufferA[gActiveBattler][1] as i32 - REQUEST_MOVE1_BATTLE,
            ) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_PP_DATA_BATTLE => {
            size = 0;
            while size < MAX_MON_MOVES {
                *dst.at(size) =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            *dst.at(size) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP_BONUSES) as u8;
            size += 1;
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            *dst = GetMonData2(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP1 + gBattleBufferA[gActiveBattler][1] as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
            ) as u8;
            size = 1;
        }
        REQUEST_OTID_BATTLE => {
            data32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_OT_ID);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_EXP_BATTLE => {
            data32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_EXP);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_HP_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP_EV) as u8;
            size = 1;
        }
        REQUEST_ATK_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK_EV) as u8;
            size = 1;
        }
        REQUEST_DEF_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF_EV) as u8;
            size = 1;
        }
        REQUEST_SPEED_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED_EV) as u8;
            size = 1;
        }
        REQUEST_SPATK_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK_EV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF_EV) as u8;
            size = 1;
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_FRIENDSHIP) as u8;
            size = 1;
        }
        REQUEST_POKERUS_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_POKERUS) as u8;
            size = 1;
        }
        REQUEST_MET_LOCATION_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MET_LOCATION) as u8;
            size = 1;
        }
        REQUEST_MET_LEVEL_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MET_LEVEL) as u8;
            size = 1;
        }
        REQUEST_MET_GAME_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MET_GAME) as u8;
            size = 1;
        }
        REQUEST_POKEBALL_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_POKEBALL) as u8;
            size = 1;
        }
        REQUEST_ALL_IVS_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP_IV) as u8;
            *dst.at(1) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK_IV) as u8;
            *dst.at(2) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF_IV) as u8;
            *dst.at(3) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED_IV) as u8;
            *dst.at(4) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK_IV) as u8;
            *dst.at(5) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF_IV) as u8;
            size = 6;
        }
        REQUEST_HP_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP_IV) as u8;
            size = 1;
        }
        REQUEST_ATK_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK_IV) as u8;
            size = 1;
        }
        REQUEST_DEF_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF_IV) as u8;
            size = 1;
        }
        REQUEST_SPEED_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED_IV) as u8;
            size = 1;
        }
        REQUEST_SPATK_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK_IV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF_IV) as u8;
            size = 1;
        }
        REQUEST_PERSONALITY_BATTLE => {
            data32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PERSONALITY);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_CHECKSUM_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_CHECKSUM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_STATUS_BATTLE => {
            data32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_STATUS);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_LEVEL_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u8;
            size = 1;
        }
        REQUEST_HP_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MAX_HP_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MAX_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_ATK_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_DEF_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPEED_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPATK_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPDEF_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_COOL_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_COOL) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_BEAUTY) as u8;
            size = 1;
        }
        REQUEST_CUTE_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_CUTE) as u8;
            size = 1;
        }
        REQUEST_SMART_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SMART) as u8;
            size = 1;
        }
        REQUEST_TOUGH_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_TOUGH) as u8;
            size = 1;
        }
        REQUEST_SHEEN_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SHEEN) as u8;
            size = 1;
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_COOL_RIBBON) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_BEAUTY_RIBBON) as u8;
            size = 1;
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_CUTE_RIBBON) as u8;
            size = 1;
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SMART_RIBBON) as u8;
            size = 1;
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_TOUGH_RIBBON) as u8;
            size = 1;
        }
        _ => {}
    }
    size as u32
}
pub(crate) unsafe fn PlayerPartnerHandleGetRawMonData() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleSetMonData() {
    let mut monToCheck: u8 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        SetPlayerPartnerMonData(gBattlerPartyIndexes[gActiveBattler] as u8);
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        for i in 0..(PARTY_SIZE as u8) {
            if monToCheck as i32 & 1 != 0 {
                SetPlayerPartnerMonData(i);
            }
            monToCheck >>= 1;
        }
    }
    PlayerPartnerBufferExecCompleted();
}
unsafe fn SetPlayerPartnerMonData(monId: u8) {
    let battlePokemon: *mut BattlePokemon =
        &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler][3] as *mut BattlePokemon;
    let moveData: *mut MovePPInfo = &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
        .cast::<CArray<CArray<u8, 512>, 4>>()
        .cast_mut())[gActiveBattler][3] as *mut MovePPInfo;
    match gBattleBufferA[gActiveBattler][1] {
        REQUEST_ALL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPECIES,
                &raw mut (*battlePokemon).species as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HELD_ITEM,
                &raw mut (*battlePokemon).item as *mut c_void,
            );
            for i in 0..MAX_MON_MOVES {
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_MOVE1 + i,
                    &raw mut (*battlePokemon).moves[i] as *mut c_void,
                );
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_PP1 + i,
                    &raw mut (*battlePokemon).pp[i] as *mut c_void,
                );
            }
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut (*battlePokemon).ppBonuses as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_FRIENDSHIP,
                &raw mut (*battlePokemon).friendship as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_EXP,
                &raw mut (*battlePokemon).experience as *mut c_void,
            );
            let mut iv: u8 = (*battlePokemon).hpIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).attackIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).defenseIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).speedIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spAttackIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spDefenseIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut iv as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PERSONALITY,
                &raw mut (*battlePokemon).personality as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_STATUS,
                &raw mut (*battlePokemon).status1 as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_LEVEL,
                &raw mut (*battlePokemon).level as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP,
                &raw mut (*battlePokemon).hp as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MAX_HP,
                &raw mut (*battlePokemon).maxHP as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK,
                &raw mut (*battlePokemon).attack as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF,
                &raw mut (*battlePokemon).defense as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED,
                &raw mut (*battlePokemon).speed as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK,
                &raw mut (*battlePokemon).spAttack as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF,
                &raw mut (*battlePokemon).spDefense as *mut c_void,
            );
        }
        REQUEST_SPECIES_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPECIES,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HELDITEM_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HELD_ITEM,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MOVES_PP_BATTLE => {
            for i in 0..MAX_MON_MOVES {
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_MOVE1 + i,
                    &raw mut (*moveData).moves[i] as *mut c_void,
                );
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_PP1 + i,
                    &raw mut (*moveData).pp[i] as *mut c_void,
                );
            }
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut (*moveData).ppBonuses as *mut c_void,
            );
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MOVE1 + gBattleBufferA[gActiveBattler][1] as i32 - REQUEST_MOVE1_BATTLE,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_PP_DATA_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP1,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP2,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][4] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP3,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][5] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP4,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][6] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][7] as *mut c_void,
            );
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP1 + gBattleBufferA[gActiveBattler][1] as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_OTID_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_OT_ID,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_EXP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_EXP,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HP_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_FRIENDSHIP,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_POKERUS_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_POKERUS,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_LOCATION_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MET_LOCATION,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_LEVEL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MET_LEVEL,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_GAME_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MET_GAME,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_POKEBALL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_POKEBALL,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ALL_IVS_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][4] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][5] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][6] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][7] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][8] as *mut c_void,
            );
        }
        REQUEST_HP_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_PERSONALITY_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PERSONALITY,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CHECKSUM_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_CHECKSUM,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_STATUS_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_STATUS,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_LEVEL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_LEVEL,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MAX_HP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MAX_HP,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_COOL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_COOL,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_BEAUTY,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_CUTE,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SMART_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SMART,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_TOUGH,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SHEEN_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SHEEN,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_COOL_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_BEAUTY_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_CUTE_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SMART_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_TOUGH_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        _ => {}
    }
    HandleLowHpMusicChange(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
}
pub(crate) unsafe fn PlayerPartnerHandleSetRawMonData() {
    let dst: *mut u8 = (&raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]] as *mut u8)
        .at(gBattleBufferA[gActiveBattler][1]);
    for i in 0..gBattleBufferA[gActiveBattler][2] {
        *dst.at(i) = gBattleBufferA[gActiveBattler][3 + i as i32];
    }
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleLoadMonSprite() {
    BattleLoadPlayerMonSpriteGfx(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
    let species: u16 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(gActiveBattler));
    gBattlerSpriteIds[gActiveBattler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        GetBattlerSpriteCoord(gActiveBattler, BATTLER_COORD_X_2) as i16,
        GetBattlerSpriteDefault_Y(gActiveBattler) as i16,
        GetBattlerSpriteSubpriority(gActiveBattler),
    );
    gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = -240;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = gActiveBattler as i16;
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(gActiveBattler as u16);
    StartSpriteAnim(
        &raw mut gSprites[gBattlerSpriteIds[gActiveBattler]],
        gBattleMonForms[gActiveBattler],
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(WaitForMonAnimAfterLoad);
}
pub(crate) unsafe fn PlayerPartnerHandleSwitchInAnim() {
    ClearTemporarySpeciesSpriteData(gActiveBattler, gBattleBufferA[gActiveBattler][2]);
    gBattlerPartyIndexes[gActiveBattler] = gBattleBufferA[gActiveBattler][1] as u16;
    BattleLoadPlayerMonSpriteGfx(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
    StartSendOutAnim(gActiveBattler, gBattleBufferA[gActiveBattler][2]);
    gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_TryShinyAnim);
}
pub(crate) unsafe fn StartSendOutAnim(battler: u8, dontClearSubstituteBit: u8) {
    ClearTemporarySpeciesSpriteData(battler, dontClearSubstituteBit);
    gBattlerPartyIndexes[battler] = gBattleBufferA[battler][1] as u16;
    let species: u16 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
        MON_DATA_SPECIES,
    ) as u16;
    gBattleControllerData[battler] =
        CreateInvisibleSpriteWithCallback(Some(SpriteCB_WaitForBattlerBallReleaseAnim));
    SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(battler));
    gBattlerSpriteIds[battler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16,
        GetBattlerSpriteDefault_Y(battler) as i16,
        GetBattlerSpriteSubpriority(battler),
    );
    gSprites[gBattleControllerData[battler]].data[1] = gBattlerSpriteIds[battler] as i16;
    gSprites[gBattleControllerData[battler]].data[2] = battler as i16;
    gSprites[gBattlerSpriteIds[battler]].data[0] = battler as i16;
    gSprites[gBattlerSpriteIds[battler]].data[2] = species as i16;
    gSprites[gBattlerSpriteIds[battler]]
        .oam
        .set_paletteNum(battler as u16);
    StartSpriteAnim(
        &raw mut gSprites[gBattlerSpriteIds[battler]],
        gBattleMonForms[battler],
    );
    gSprites[gBattlerSpriteIds[battler]].set_invisible(TRUE as u16);
    gSprites[gBattlerSpriteIds[battler]].callback = Some(SpriteCallbackDummy);
    gSprites[gBattleControllerData[battler]].data[0] =
        DoPokeballSendOutAnimation(0, POKEBALL_PLAYER_SENDOUT) as i16;
}
pub(crate) unsafe fn PlayerPartnerHandleReturnMonToBall() {
    if gBattleBufferA[gActiveBattler][1] == 0 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
        gBattlerControllerFuncs[gActiveBattler] = Some(DoSwitchOutAnimation);
    } else {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn DoSwitchOutAnimation() {
    match (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState {
        0 => {
            if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute() != 0 {
                InitAndLaunchSpecialAnimation(
                    gActiveBattler,
                    gActiveBattler,
                    gActiveBattler,
                    B_ANIM_SUBSTITUTE_TO_MON,
                );
            }
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 1;
        }
        1 if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive()
            == 0 =>
        {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
            InitAndLaunchSpecialAnimation(
                gActiveBattler,
                gActiveBattler,
                gActiveBattler,
                B_ANIM_SWITCH_OUT_PLAYER_MON,
            );
            gBattlerControllerFuncs[gActiveBattler] = Some(FreeMonSpriteAfterSwitchOutAnim);
        }
        _ => {}
    }
}
pub(crate) unsafe fn PlayerPartnerHandleDrawTrainerPic() {
    let mut xPos: i16 = 0;
    let mut yPos: i16 = 0;
    let mut trainerPicId: u32 = 0;
    if gPartnerTrainerId == TRAINER_STEVEN_PARTNER {
        trainerPicId = TRAINER_BACK_PIC_STEVEN;
        xPos = 90;
        yPos = (8
            - (*(&raw const crate::data::data_tables::gTrainerBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[trainerPicId]
                .size as i16)
            * 4
            + 80;
    } else {
        trainerPicId = GetFrontierTrainerFrontSpriteId(gPartnerTrainerId) as u32;
        xPos = 32;
        yPos = (8
            - (*(&raw const crate::data::data_tables::gTrainerFrontPicCoords)
                .cast::<CArray<MonCoords, 0>>())[trainerPicId]
                .size as i16)
            * 4
            + 80;
    }
    if gPartnerTrainerId == TRAINER_STEVEN_PARTNER {
        DecompressTrainerBackPic(trainerPicId as u16, gActiveBattler);
        SetMultiuseSpriteTemplateToTrainerBack(
            trainerPicId as u16,
            GetBattlerPosition(gActiveBattler),
        );
        gBattlerSpriteIds[gActiveBattler] = CreateSprite(
            &raw mut gMultiuseSpriteTemplate,
            xPos,
            yPos,
            GetBattlerSpriteSubpriority(gActiveBattler),
        );
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .oam
            .set_paletteNum(gActiveBattler as u16);
        gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = DISPLAY_WIDTH as i16;
        gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = -2;
        gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_TrainerSlideIn);
    } else {
        DecompressTrainerFrontPic(trainerPicId as u16, gActiveBattler);
        SetMultiuseSpriteTemplateToTrainerFront(
            trainerPicId as u16,
            GetBattlerPosition(gActiveBattler),
        );
        gBattlerSpriteIds[gActiveBattler] = CreateSprite(
            &raw mut gMultiuseSpriteTemplate,
            xPos,
            yPos,
            GetBattlerSpriteSubpriority(gActiveBattler),
        );
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .oam
            .set_paletteNum(IndexOfSpritePaletteTag(
                (*(&raw const crate::data::data_tables::gTrainerFrontPicPaletteTable)
                    .cast::<CArray<CompressedSpritePalette, 0>>())[trainerPicId]
                    .tag,
            ) as u16);
        gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = DISPLAY_WIDTH as i16;
        gSprites[gBattlerSpriteIds[gActiveBattler]].y2 = 48;
        gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = -2;
        gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_TrainerSlideIn);
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .oam
            .set_affineMode(ST_OAM_AFFINE_OFF);
        gSprites[gBattlerSpriteIds[gActiveBattler]].set_hFlip(1);
    }
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnBattlerSpriteCallbackDummy);
}
pub(crate) unsafe fn PlayerPartnerHandleTrainerSlide() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleTrainerSlideBack() {
    SetSpritePrimaryCoordsFromSecondaryCoords(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = 35;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[2] = -40;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[4] =
        gSprites[gBattlerSpriteIds[gActiveBattler]].y;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(
        &raw mut gSprites[gBattlerSpriteIds[gActiveBattler]],
        Some(SpriteCallbackDummy),
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(FreeTrainerSpriteAfterSlide);
}
pub(crate) unsafe fn PlayerPartnerHandleFaintAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState == 0 {
        if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute() != 0 {
            InitAndLaunchSpecialAnimation(
                gActiveBattler,
                gActiveBattler,
                gActiveBattler,
                B_ANIM_SUBSTITUTE_TO_MON,
            );
        }
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState += 1;
    } else {
        if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0 {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
            HandleLowHpMusicChange(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                gActiveBattler,
            );
            PlaySE12WithPanning(SE_FAINT, SOUND_PAN_ATTACKER);
            gSprites[gBattlerSpriteIds[gActiveBattler]].data[1] = 0;
            gSprites[gBattlerSpriteIds[gActiveBattler]].data[sSpeedY] = 5;
            gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_FaintSlideAnim);
            gBattlerControllerFuncs[gActiveBattler] = Some(FreeMonSpriteAfterFaintAnim);
        }
    }
}
pub(crate) unsafe fn PlayerPartnerHandlePaletteFade() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleSuccessBallThrowAnim() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleBallThrowAnim() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandlePause() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleMoveAnimation() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        let r#move: u16 = gBattleBufferA[gActiveBattler][1] as u16
            | (gBattleBufferA[gActiveBattler][2] as u16) << 8;
        gAnimMoveTurn = gBattleBufferA[gActiveBattler][3];
        gAnimMovePower = gBattleBufferA[gActiveBattler][4] as u16
            | (gBattleBufferA[gActiveBattler][5] as u16) << 8;
        gAnimMoveDmg = gBattleBufferA[gActiveBattler][6] as i32
            | (gBattleBufferA[gActiveBattler][7] as i32) << 8
            | (gBattleBufferA[gActiveBattler][8] as i32) << 16
            | (gBattleBufferA[gActiveBattler][9] as i32) << 24;
        gAnimFriendship = gBattleBufferA[gActiveBattler][10];
        gWeatherMoveAnim = gBattleBufferA[gActiveBattler][12] as u16
            | (gBattleBufferA[gActiveBattler][13] as u16) << 8;
        gAnimDisableStructPtr = &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler][16] as *mut DisableStruct;
        gTransformedPersonalities[gActiveBattler] =
            (*gAnimDisableStructPtr).transformedMonPersonality;
        if IsMoveWithoutAnimation(r#move, gAnimMoveTurn) != 0 {
            PlayerPartnerBufferExecCompleted();
        } else {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
            gBattlerControllerFuncs[gActiveBattler] = Some(PlayerPartnerDoMoveAnimation);
        }
    }
}
pub(crate) unsafe fn PlayerPartnerDoMoveAnimation() {
    let r#move: u16 =
        gBattleBufferA[gActiveBattler][1] as u16 | (gBattleBufferA[gActiveBattler][2] as u16) << 8;
    let multihit: u8 = gBattleBufferA[gActiveBattler][11];
    match (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState {
        0 => {
            if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute() != 0
                && (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).flag_x8() == 0
            {
                (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).set_flag_x8(1);
                InitAndLaunchSpecialAnimation(
                    gActiveBattler,
                    gActiveBattler,
                    gActiveBattler,
                    B_ANIM_SUBSTITUTE_TO_MON,
                );
            }
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 1;
        }
        1 => {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive()
                == 0
            {
                SetBattlerSpriteAffineMode(ST_OAM_AFFINE_OFF as u8);
                DoMoveAnim(r#move);
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 2;
            }
        }
        2 => {
            gAnimScriptCallback.unwrap_unchecked()();
            if gAnimScriptActive == 0 {
                SetBattlerSpriteAffineMode(ST_OAM_AFFINE_NORMAL as u8);
                if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute()
                    != 0
                    && multihit < 2
                {
                    InitAndLaunchSpecialAnimation(
                        gActiveBattler,
                        gActiveBattler,
                        gActiveBattler,
                        B_ANIM_MON_TO_SUBSTITUTE,
                    );
                    (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).set_flag_x8(0);
                }
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 3;
            }
        }
        3 if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive()
            == 0 =>
        {
            CopyAllBattleSpritesInvisibilities();
            TrySetBehindSubstituteSpriteBit(
                gActiveBattler,
                gBattleBufferA[gActiveBattler][1] as u16
                    | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
            );
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
            PlayerPartnerBufferExecCompleted();
        }
        _ => {}
    }
}
pub(crate) unsafe fn PlayerPartnerHandlePrintString() {
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    let stringId: *mut u16 = &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
        .cast::<CArray<CArray<u8, 512>, 4>>()
        .cast_mut())[gActiveBattler][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnInactiveTextPrinter2);
}
pub(crate) unsafe fn PlayerPartnerHandlePrintSelectionString() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleChooseAction() {
    AI_TrySwitchOrUseItem();
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleYesNoBox() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleChooseMove() {
    let moveInfo: *mut ChooseMoveStruct =
        &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler][4] as *mut ChooseMoveStruct;
    BattleAI_SetupAIData(ALL_MOVES_MASK);
    let chosenMoveId: u8 = BattleAI_ChooseMoveOrAction();
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [(*moveInfo).moves[chosenMoveId]]
        .target as i32
        & 18
        != 0
    {
        gBattlerTarget = gActiveBattler;
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [(*moveInfo).moves[chosenMoveId]]
        .target as i32
        & MOVE_TARGET_BOTH as i32
        != 0
    {
        gBattlerTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
        if gAbsentBattlerFlags as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gBattlerTarget]
            != 0
        {
            gBattlerTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
        }
    }
    BtlController_EmitTwoReturnValues(
        B_COMM_TO_ENGINE,
        B_ACTION_EXEC_SCRIPT,
        chosenMoveId as u16 | (gBattlerTarget as u16) << 8,
    );
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleChooseItem() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleChoosePokemon() {
    let chosenMonId: i32 = GetMostSuitableMonToSwitchInto() as i32;
    if chosenMonId == PARTY_SIZE {
        let playerMonIdentity: u8 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        let selfIdentity: u8 = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
        for chosenMonId in 3..PARTY_SIZE {
            if GetMonData2(&raw mut gPlayerParty[chosenMonId], MON_DATA_HP) != 0
                && chosenMonId != gBattlerPartyIndexes[playerMonIdentity] as i32
                && chosenMonId != gBattlerPartyIndexes[selfIdentity] as i32
            {
                break;
            }
        }
    }
    *(*gBattleStruct)
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler) = chosenMonId as u8;
    BtlController_EmitChosenMonReturnValue(B_COMM_TO_ENGINE, chosenMonId as u8, null_mut());
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleCmd23() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleHealthBarUpdate() {
    LoadBattleBarGfx(0);
    let hpVal: i16 =
        gBattleBufferA[gActiveBattler][2] as i16 | (gBattleBufferA[gActiveBattler][3] as i16) << 8;
    if hpVal != INSTANT_HP_BAR_DROP {
        let maxHP: u32 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_MAX_HP,
        );
        let curHP: u32 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_HP,
        );
        SetBattleBarStruct(
            gActiveBattler,
            gHealthboxSpriteIds[gActiveBattler],
            maxHP as i32,
            curHP as i32,
            hpVal as i32,
        );
    } else {
        let maxHP: u32 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_MAX_HP,
        );
        SetBattleBarStruct(
            gActiveBattler,
            gHealthboxSpriteIds[gActiveBattler],
            maxHP as i32,
            0,
            hpVal as i32,
        );
    }
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnHealthbarDone);
}
pub(crate) unsafe fn PlayerPartnerHandleExpUpdate() {
    let monId: u8 = gBattleBufferA[gActiveBattler][1];
    if GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) >= MAX_LEVEL {
        PlayerPartnerBufferExecCompleted();
    } else {
        LoadBattleBarGfx(1);
        GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES);
        let expPointsToGive: i16 = gBattleBufferA[gActiveBattler][2] as i16
            | (gBattleBufferA[gActiveBattler][3] as i16) << 8;
        let taskId: u8 = CreateTask(Some(Task_GiveExpToMon), 10);
        task_set(taskId, tExpTask_monId, monId as i16);
        task_set(taskId, tExpTask_gainedExp, expPointsToGive);
        task_set(taskId, tExpTask_bank, gActiveBattler as i16);
        gBattlerControllerFuncs[gActiveBattler] = Some(BattleControllerDummy);
    }
}
pub(crate) unsafe fn PlayerPartnerHandleStatusIconUpdate() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_STATUS_ICON,
        );
        let battler: u8 = gActiveBattler;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_statusAnimActive(0);
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedStatusAnimation);
    }
}
pub(crate) unsafe fn PlayerPartnerHandleStatusAnimation() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        InitAndLaunchChosenStatusAnimation(
            gBattleBufferA[gActiveBattler][1],
            gBattleBufferA[gActiveBattler][2] as u32
                | (gBattleBufferA[gActiveBattler][3] as u32) << 8
                | (gBattleBufferA[gActiveBattler][4] as u32) << 16
                | (gBattleBufferA[gActiveBattler][5] as u32) << 24,
        );
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedStatusAnimation);
    }
}
pub(crate) unsafe fn PlayerPartnerHandleStatusXor() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleDataTransfer() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleDMA3Transfer() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandlePlayBGM() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleCmd32() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleTwoReturnValues() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleChosenMonReturnValue() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleOneReturnValue() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleOneReturnValue_Duplicate() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleClearUnkVar() {
    gUnusedControllerStruct.set_unk(0);
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleSetUnkVar() {
    gUnusedControllerStruct.set_unk(gBattleBufferA[gActiveBattler][1]);
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleClearUnkFlag() {
    gUnusedControllerStruct.set_flag(0);
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleToggleUnkFlag() {
    gUnusedControllerStruct.set_flag(gUnusedControllerStruct.flag() ^ 1);
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleHitAnimation() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].invisible() == TRUE as u16 {
        PlayerPartnerBufferExecCompleted();
    } else {
        gDoingBattleAnim = TRUE;
        gSprites[gBattlerSpriteIds[gActiveBattler]].data[1] = 0;
        DoHitAnimHealthboxEffect(gActiveBattler);
        gBattlerControllerFuncs[gActiveBattler] = Some(DoHitAnimBlinkSpriteEffect);
    }
}
pub(crate) unsafe fn PlayerPartnerHandleCantSwitch() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandlePlaySE() {
    let mut pan: i8 = 0;
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        pan = SOUND_PAN_ATTACKER;
    } else {
        pan = SOUND_PAN_TARGET;
    }
    PlaySE12WithPanning(
        gBattleBufferA[gActiveBattler][1] as u16 | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
        pan,
    );
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandlePlayFanfareOrBGM() {
    if gBattleBufferA[gActiveBattler][3] != 0 {
        BattleStopLowHpSound();
        PlayBGM(
            gBattleBufferA[gActiveBattler][1] as u16
                | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
        );
    } else {
        PlayFanfare(
            gBattleBufferA[gActiveBattler][1] as u16
                | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
        );
    }
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleFaintingCry() {
    let species: u16 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_ByMode(species, -25, CRY_MODE_FAINT);
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleIntroSlide() {
    HandleIntroSlide(gBattleBufferA[gActiveBattler][1]);
    gIntroSlideFlags |= 1;
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleIntroTrainerBallThrow() {
    SetSpritePrimaryCoordsFromSecondaryCoords(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = 50;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[2] = -40;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[4] =
        gSprites[gBattlerSpriteIds[gActiveBattler]].y;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(StartAnimLinearTranslation);
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[5] = gActiveBattler as i16;
    StoreSpriteCallbackInData6(
        &raw mut gSprites[gBattlerSpriteIds[gActiveBattler]],
        Some(SpriteCB_FreePlayerSpriteLoadMonSprite),
    );
    StartSpriteAnim(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]], 1);
    let paletteNum: u8 = AllocSpritePalette(0xD6F9);
    if gPartnerTrainerId == TRAINER_STEVEN_PARTNER {
        let spriteId: u8 = TRAINER_BACK_PIC_STEVEN as u8;
        LoadCompressedPalette(
            (*(&raw const crate::data::data_tables::gTrainerBackPicPaletteTable)
                .cast::<CArray<CompressedSpritePalette, 0>>())[spriteId]
                .data,
            0x100 + paletteNum as u16 * 16,
            32,
        );
    } else {
        let spriteId: u8 = GetFrontierTrainerFrontSpriteId(gPartnerTrainerId);
        LoadCompressedPalette(
            (*(&raw const crate::data::data_tables::gTrainerFrontPicPaletteTable)
                .cast::<CArray<CompressedSpritePalette, 0>>())[spriteId]
                .data,
            0x100 + paletteNum as u16 * 16,
            32,
        );
    }
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(paletteNum as u16);
    let taskId: u8 = CreateTask(Some(Task_StartSendOutAnim), 5);
    task_set(taskId, 0, gActiveBattler as i16);
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusSummaryShown() != 0
    {
        task_set_func(
            gBattlerStatusSummaryTaskId[gActiveBattler],
            Some(Task_HidePartyStatusSummary),
        );
    }
    (*(*gBattleSpritesDataPtr).animationData).set_introAnimActive(TRUE);
    gBattlerControllerFuncs[gActiveBattler] = Some(PlayerPartnerDummy);
}
pub(crate) unsafe fn Task_StartSendOutAnim(taskId: u8) {
    if task_get(taskId, 1) < 24 {
        task_set(taskId, 1, task_get(taskId, 1) + 1);
    } else {
        let savedActiveBank: u8 = gActiveBattler;
        gActiveBattler = task_get(taskId, 0) as u8;
        if IsDoubleBattle() == 0 || gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
            gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
            StartSendOutAnim(gActiveBattler, FALSE);
        } else {
            gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
            StartSendOutAnim(gActiveBattler, FALSE);
            gActiveBattler ^= BIT_FLANK;
            gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
            BattleLoadPlayerMonSpriteGfx(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                gActiveBattler,
            );
            StartSendOutAnim(gActiveBattler, FALSE);
            gActiveBattler ^= BIT_FLANK;
        }
        gBattlerControllerFuncs[gActiveBattler] = Some(Intro_ShowHealthbox);
        gActiveBattler = savedActiveBank;
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn PlayerPartnerHandleDrawPartyStatusSummary() {
    if gBattleBufferA[gActiveBattler][1] != 0 && GetBattlerSide(gActiveBattler) == 0 {
        PlayerPartnerBufferExecCompleted();
    } else {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_partyStatusSummaryShown(1);
        gBattlerStatusSummaryTaskId[gActiveBattler] = CreatePartyStatusSummarySprites(
            gActiveBattler,
            &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                .cast::<CArray<CArray<u8, 512>, 4>>()
                .cast_mut())[gActiveBattler][4] as *mut HpAndStatus,
            gBattleBufferA[gActiveBattler][1],
            gBattleBufferA[gActiveBattler][2],
        );
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer = 0;
        if gBattleBufferA[gActiveBattler][2] != 0 {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer =
                93;
        }
        gBattlerControllerFuncs[gActiveBattler] = Some(EndDrawPartyStatusSummary);
    }
}
pub(crate) unsafe fn EndDrawPartyStatusSummary() {
    if ({
        let t1 =
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer += 1;
        t1
    }) > 92
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer = 0;
        PlayerPartnerBufferExecCompleted();
    }
}
pub(crate) unsafe fn PlayerPartnerHandleHidePartyStatusSummary() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusSummaryShown() != 0
    {
        task_set_func(
            gBattlerStatusSummaryTaskId[gActiveBattler],
            Some(Task_HidePartyStatusSummary),
        );
    }
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleEndBounceEffect() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleSpriteInvisibility() {
    if IsBattlerSpritePresent(gActiveBattler) != 0 {
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .set_invisible(gBattleBufferA[gActiveBattler][1] as u16);
        CopyBattleSpriteInvisibility(gActiveBattler);
    }
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleBattleAnimation() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        let animationId: u8 = gBattleBufferA[gActiveBattler][1];
        let argument: u16 = gBattleBufferA[gActiveBattler][2] as u16
            | (gBattleBufferA[gActiveBattler][3] as u16) << 8;
        if TryHandleLaunchBattleTableAnimation(
            gActiveBattler,
            gActiveBattler,
            gActiveBattler,
            animationId,
            argument,
        ) != 0
        {
            PlayerPartnerBufferExecCompleted();
        } else {
            gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedBattleAnimation);
        }
    }
}
pub(crate) unsafe fn PlayerPartnerHandleLinkStandbyMsg() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleResetActionMoveSelection() {
    PlayerPartnerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerPartnerHandleEndLinkBattle() {
    gBattleOutcome = gBattleBufferA[gActiveBattler][1];
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    PlayerPartnerBufferExecCompleted();
    gBattlerControllerFuncs[gActiveBattler] = Some(SetBattleEndCallbacks);
}
pub(crate) fn PlayerPartnerCmdEnd() {}
