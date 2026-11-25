//! Translated from `src/battle_controller_wally.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

use crate::agb_main::gMain;
use crate::battle_anim::{
    DoMoveAnim, gAnimDisableStructPtr, gAnimFriendship, gAnimMoveDmg, gAnimMovePower,
    gAnimMoveTurn, gAnimScriptActive, gAnimScriptCallback, gWeatherMoveAnim,
};
use crate::battle_anim_mons::{
    GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide, GetBattlerSpriteCoord,
    GetBattlerSpriteDefault_Y, GetBattlerSpriteSubpriority, IsDoubleBattle,
    SetSpritePrimaryCoordsFromSecondaryCoords, StartAnimLinearTranslation,
    StoreSpriteCallbackInData6,
};
use crate::battle_anim_throw::TryShinyAnimation;
use crate::battle_controller_player::{
    ActionSelectionCreateCursorAt, ActionSelectionDestroyCursorAt, BattleControllerDummy,
    InitMoveSelectionsVarsAndStrings, PlayerHandleGetRawMonData, SetBattleEndCallbacks,
    SpriteCB_FreePlayerSpriteLoadMonSprite, Task_PlayerController_RestoreBgmAfterCry,
};
use crate::battle_controllers::{
    BtlController_EmitDataTransfer, BtlController_EmitOneReturnValue,
    BtlController_EmitTwoReturnValues, PrepareBufferDataTransferLink,
};
use crate::battle_gfx_sfx_util::{
    BattleStopLowHpSound, CopyAllBattleSpritesInvisibilities, DecompressTrainerBackPic,
    HandleLowHpMusicChange, InitAndLaunchSpecialAnimation, IsMoveWithoutAnimation,
    LoadBattleBarGfx, SetBattlerSpriteAffineMode, SpriteCB_TrainerSlideIn,
    SpriteCB_WaitForBattlerBallReleaseAnim, TryHandleLaunchBattleTableAnimation,
    TrySetBehindSubstituteSpriteBit,
};
use crate::battle_interface::{
    CreatePartyStatusSummarySprites, MoveBattleBar, SetBattleBarStruct,
    SetHealthboxSpriteInvisible, SetHealthboxSpriteVisible, Task_HidePartyStatusSummary,
    UpdateHealthboxAttribute, UpdateHpTextInHealthbox,
};
use crate::battle_intro::HandleIntroSlide;
use crate::battle_main::{
    BattleMainCB2, gActiveBattler, gBattle_BG0_X, gBattle_BG0_Y, gBattleControllerExecFlags,
    gBattleOutcome, gBattleSpritesDataPtr, gBattleStruct, gBattleTypeFlags,
    gBattlerControllerFuncs, gBattlerInMenuId, gDoingBattleAnim, gIntroSlideFlags,
    gTransformedPersonalities,
};
use crate::battle_main::{
    gActionSelectionCursor, gBattleBufferA, gBattleControllerData, gBattleMonForms,
    gBattlerPartyIndexes, gBattlerSpriteIds, gBattlerStatusSummaryTaskId, gDisplayedStringBattle,
    gHealthboxSpriteIds,
};
use crate::battle_message::{
    BattlePutTextOnWindow, BattleStringExpandPlaceholdersToDisplayedString, BufferStringBattle,
};
use crate::bg::IsDma3ManagerBusyWithBgCopy;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::item_menu::{DoWallyTutorialBagMenu, gSpecialVar_ItemId};
use crate::link::GetMultiplayerId;
use crate::palette::{
    BeginFastPaletteFade, BeginNormalPaletteFade, LoadCompressedPalette, gPaletteFade,
};
use crate::pokeball::{
    DoHitAnimHealthboxEffect, DoPokeballSendOutAnimation, StartHealthboxSlideIn,
};
use crate::pokemon::{
    GetMonData2, GetMonData3, SetMonData, SetMultiuseSpriteTemplateToPokemon,
    SetMultiuseSpriteTemplateToTrainerBack, gMultiuseSpriteTemplate, gPlayerParty,
};
use crate::reshow_battle_screen::ReshowBattleScreenDummy;
use crate::sound::{FadeOutMapMusic, PlayBGM, PlayCry_Normal, PlayFanfare, PlaySE};
use crate::sprite::gSprites;
use crate::sprite::{AllocSpritePalette, FreeSpritePaletteByTag, FreeSpriteTilesByTag};
use crate::task::DestroyTask;
use crate::task::{task_get, task_set, task_set_func};
use crate::text::IsTextPrinterActive;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
use crate::window::FreeAllWindowBuffers;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateInvisibleSpriteWithCallback` with this module's view of its types.
#[inline]
fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe fn(*mut Sprite)>) -> u8 {
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
const sSpeedX: usize = 0;
// Data tables (translate with cdata.py): sWallyBufferCommands

static sWallyBufferCommands: Table<CArray<Option<unsafe fn()>, 57>> =
    Table((&raw const crate::data::battle_controller_wally::sWallyBufferCommands).cast());

fn SpriteCB_Null7() {}
pub unsafe fn SetControllerToWally() {
    {
        let gl_i0 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i0] = Some(WallyBufferRunCommand))
    };
    (*crate::gl!(gBattleStruct)).wallyBattleState = 0;
    (*crate::gl!(gBattleStruct)).wallyMovesState = 0;
    (*crate::gl!(gBattleStruct)).wallyWaitFrames = 0;
    (*crate::gl!(gBattleStruct)).wallyMoveFrames = 0;
}
pub(crate) unsafe fn WallyBufferRunCommand() {
    if gBattleControllerExecFlags.get()
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
        != 0
    {
        if ({
            let gl_i1 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i1][0])
        }) < 57
        {
            sWallyBufferCommands[{
                let gl_i2 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i2][0])
            }]
            .unwrap_unchecked()();
        } else {
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe fn WallyHandleActions() {
    'l1: {
        let sw1: u8 = (*crate::gl!(gBattleStruct)).wallyBattleState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*crate::gl!(gBattleStruct)).wallyWaitFrames = B_WAIT_TIME_LONG;
            (*crate::gl!(gBattleStruct)).wallyBattleState += 1;
        }
        if fall || sw1 == 1 {
            if ({
                (*crate::gl!(gBattleStruct)).wallyWaitFrames -= 1;
                (*crate::gl!(gBattleStruct)).wallyWaitFrames
            }) == 0
            {
                PlaySE(SE_SELECT);
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, 0, 0);
                WallyBufferExecCompleted();
                (*crate::gl!(gBattleStruct)).wallyBattleState += 1;
                (*crate::gl!(gBattleStruct)).wallyMovesState = 0;
                (*crate::gl!(gBattleStruct)).wallyWaitFrames = B_WAIT_TIME_LONG;
            }
            break 'l1;
        }
        if sw1 == 2 {
            if ({
                (*crate::gl!(gBattleStruct)).wallyWaitFrames -= 1;
                (*crate::gl!(gBattleStruct)).wallyWaitFrames
            }) == 0
            {
                PlaySE(SE_SELECT);
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, 0, 0);
                WallyBufferExecCompleted();
                (*crate::gl!(gBattleStruct)).wallyBattleState += 1;
                (*crate::gl!(gBattleStruct)).wallyMovesState = 0;
                (*crate::gl!(gBattleStruct)).wallyWaitFrames = B_WAIT_TIME_LONG;
            }
            break 'l1;
        }
        if sw1 == 3 {
            if ({
                (*crate::gl!(gBattleStruct)).wallyWaitFrames -= 1;
                (*crate::gl!(gBattleStruct)).wallyWaitFrames
            }) == 0
            {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_WALLY_THROW, 0);
                WallyBufferExecCompleted();
                (*crate::gl!(gBattleStruct)).wallyBattleState += 1;
                (*crate::gl!(gBattleStruct)).wallyMovesState = 0;
                (*crate::gl!(gBattleStruct)).wallyWaitFrames = B_WAIT_TIME_LONG;
            }
            break 'l1;
        }
        if sw1 == 4 {
            if ({
                (*crate::gl!(gBattleStruct)).wallyWaitFrames -= 1;
                (*crate::gl!(gBattleStruct)).wallyWaitFrames
            }) == 0
            {
                PlaySE(SE_SELECT);
                ActionSelectionDestroyCursorAt(0);
                ActionSelectionCreateCursorAt(1, 0);
                (*crate::gl!(gBattleStruct)).wallyWaitFrames = B_WAIT_TIME_LONG;
                (*crate::gl!(gBattleStruct)).wallyBattleState += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            if ({
                (*crate::gl!(gBattleStruct)).wallyWaitFrames -= 1;
                (*crate::gl!(gBattleStruct)).wallyWaitFrames
            }) == 0
            {
                PlaySE(SE_SELECT);
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_USE_ITEM, 0);
                WallyBufferExecCompleted();
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn CompleteOnBattlerSpriteCallbackDummy() {
    if (*gSprites.as_ptr())[{
        let gl_i3 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i3])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnFinishedAnimation() {
    if gDoingBattleAnim.get() == 0 {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn OpenBagAfterPaletteFade() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        {
            let gl_i4 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i4] = Some(CompleteOnChosenItem))
        };
        ReshowBattleScreenDummy();
        FreeAllWindowBuffers();
        DoWallyTutorialBagMenu();
    }
}
pub(crate) unsafe fn CompleteOnChosenItem() {
    if crate::gl!(gMain.callback2) == Some(BattleMainCB2 as unsafe fn())
        && (*gPaletteFade.as_ptr()).active() == 0
    {
        BtlController_EmitOneReturnValue(B_COMM_TO_ENGINE, gSpecialVar_ItemId.get());
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn Intro_TryShinyAnimShowHealthbox() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .triedShinyMonAnim()
        == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .ballAnimActive()
            == 0
    {
        TryShinyAnimation(
            gActiveBattler.get(),
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i5 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i5])
            }],
        );
    }
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get() as i32 ^ 2))
    .triedShinyMonAnim()
        == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .ballAnimActive()
            == 0
    {
        TryShinyAnimation(
            gActiveBattler.get() ^ 2,
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i6 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gBattlerPartyIndexes[gl_i6])
            }],
        );
    }
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .ballAnimActive()
        == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .ballAnimActive()
            == 0
        && (*gSprites.as_ptr())[{
            let gl_i7 = gActiveBattler.get();
            crate::gl!(gBattleControllerData[gl_i7])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && (*gSprites.as_ptr())[{
            let gl_i8 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i8])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        if IsDoubleBattle() != 0 && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0 {
            DestroySprite(
                &raw mut (*gSprites.as_ptr())[{
                    let gl_i9 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gBattleControllerData[gl_i9])
                }],
            );
            UpdateHealthboxAttribute(
                {
                    let gl_i10 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gHealthboxSpriteIds[gl_i10])
                },
                &raw mut (*gPlayerParty.as_ptr())[{
                    let gl_i11 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gBattlerPartyIndexes[gl_i11])
                }],
                HEALTHBOX_ALL,
            );
            StartHealthboxSlideIn(gActiveBattler.get() ^ 2);
            SetHealthboxSpriteVisible({
                let gl_i12 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gHealthboxSpriteIds[gl_i12])
            });
        }
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i13 = gActiveBattler.get();
                crate::gl!(gBattleControllerData[gl_i13])
            }],
        );
        UpdateHealthboxAttribute(
            {
                let gl_i14 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i14])
            },
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i15 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i15])
            }],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler.get());
        SetHealthboxSpriteVisible({
            let gl_i16 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i16])
        });
        (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).set_introAnimActive(FALSE);
        {
            let gl_i17 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i17] = Some(Intro_WaitForShinyAnimAndHealthbox))
        };
    }
}
pub(crate) unsafe fn Intro_WaitForShinyAnimAndHealthbox() {
    let mut healthboxAnimDone: u32 = FALSE as u32;
    if (*gSprites.as_ptr())[{
        let gl_i18 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i18])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        healthboxAnimDone = TRUE as u32;
    }
    if healthboxAnimDone != 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .finishedShinyMonAnim()
            != 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .finishedShinyMonAnim()
            != 0
    {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_triedShinyMonAnim(FALSE);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_finishedShinyMonAnim(FALSE);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .set_triedShinyMonAnim(FALSE);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .set_finishedShinyMonAnim(FALSE);
        FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
        FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
        CreateTask(Some(Task_PlayerController_RestoreBgmAfterCry), 10);
        HandleLowHpMusicChange(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i19 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i19])
            }],
            gActiveBattler.get(),
        );
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnHealthbarDone() {
    let hpValue: i16 = MoveBattleBar(
        gActiveBattler.get(),
        {
            let gl_i20 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i20])
        },
        HEALTH_BAR,
        0,
    ) as i16;
    SetHealthboxSpriteVisible({
        let gl_i21 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i21])
    });
    if hpValue != -1 {
        UpdateHpTextInHealthbox(
            {
                let gl_i22 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i22])
            },
            hpValue,
            HP_CURRENT,
        );
    } else {
        HandleLowHpMusicChange(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i23 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i23])
            }],
            gActiveBattler.get(),
        );
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn DoHitAnimBlinkSpriteEffect() {
    let spriteId: u8 = {
        let gl_i24 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i24])
    };
    if crate::gl!(gSprites[spriteId].data[1]) == 32 {
        crate::gl!(gSprites[spriteId].data[1] = 0);
        (*gSprites.as_ptr())[spriteId].set_invisible(FALSE as u16);
        gDoingBattleAnim.set(FALSE);
        WallyBufferExecCompleted();
    } else {
        if crate::gl!(gSprites[spriteId].data[1]) % 4 == 0 {
            (*gSprites.as_ptr())[spriteId]
                .set_invisible((*gSprites.as_ptr())[spriteId].invisible() ^ 1);
        }
        crate::gl!(gSprites[spriteId].data[1] += 1);
    }
}
pub(crate) unsafe fn DoSwitchOutAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .specialAnimActive()
        == 0
    {
        FreeSpriteOamMatrix(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i25 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i25])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i26 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i26])
            }],
        );
        SetHealthboxSpriteInvisible({
            let gl_i27 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i27])
        });
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnBankSpriteCallbackDummy2() {
    if (*gSprites.as_ptr())[{
        let gl_i28 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i28])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnFinishedBattleAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .animFromTableActive()
        == 0
    {
        WallyBufferExecCompleted();
    }
}
unsafe fn WallyBufferExecCompleted() {
    {
        let gl_i29 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i29] = Some(WallyBufferRunCommand))
    };
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        let mut playerId: u8 = GetMultiplayerId();
        PrepareBufferDataTransferLink(B_COMM_CONTROLLER_IS_DONE, 4, &raw mut playerId);
        {
            let gl_i30 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i30][0] = CONTROLLER_TERMINATOR_NOP)
        };
    } else {
        gBattleControllerExecFlags
            .set(gBattleControllerExecFlags.get() & (!gBitTable[gActiveBattler.get()]));
    }
}
pub(crate) unsafe fn CompleteOnFinishedStatusAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .statusAnimActive()
        == 0
    {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn WallyHandleGetMonData() {
    let mut monData: CArray<u8, 256> = zeroed();
    let mut size: u32 = 0;
    let mut monToCheck: u8 = 0;
    if ({
        let gl_i31 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i31][2])
    }) == 0
    {
        size += CopyWallyMonData(
            ({
                let gl_i32 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i32])
            }) as u8,
            monData.as_mut_ptr(),
        );
    } else {
        monToCheck = {
            let gl_i33 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i33][2])
        };
        for i in 0..PARTY_SIZE {
            if monToCheck as i32 & 1 != 0 {
                size += CopyWallyMonData(i as u8, monData.as_mut_ptr().at(size));
            }
            monToCheck >>= 1;
        }
    }
    BtlController_EmitDataTransfer(
        B_COMM_TO_ENGINE,
        size as u16,
        monData.as_mut_ptr() as *mut c_void,
    );
    WallyBufferExecCompleted();
}
unsafe fn CopyWallyMonData(monId: u8, dst: *mut u8) -> u32 {
    let mut battleMon: BattlePokemon = zeroed();
    let mut moveData: MovePPInfo = zeroed();
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut src: *mut u8 = null_mut();
    let mut data16: i16 = 0;
    let mut data32: u32 = 0;
    let mut size: i32 = 0;
    match {
        let gl_i34 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i34][1])
    } {
        REQUEST_ALL_BATTLE => {
            battleMon.species =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPECIES) as u16;
            battleMon.item =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HELD_ITEM) as u16;
            for size in 0..MAX_MON_MOVES {
                battleMon.moves[size] =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MOVE1 + size) as u16;
                battleMon.pp[size] =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP1 + size) as u8;
            }
            battleMon.ppBonuses =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP_BONUSES) as u8;
            battleMon.friendship =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_FRIENDSHIP) as u8;
            battleMon.experience = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_EXP);
            battleMon.set_hpIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_IV,
            ));
            battleMon.set_attackIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_IV,
            ));
            battleMon.set_defenseIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_IV,
            ));
            battleMon.set_speedIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_IV,
            ));
            battleMon.set_spAttackIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_IV,
            ));
            battleMon.set_spDefenseIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_IV,
            ));
            battleMon.personality =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PERSONALITY);
            battleMon.status1 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_STATUS);
            battleMon.level =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_LEVEL) as u8;
            battleMon.hp = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP) as u16;
            battleMon.maxHP =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MAX_HP) as u16;
            battleMon.attack =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK) as u16;
            battleMon.defense =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF) as u16;
            battleMon.speed =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED) as u16;
            battleMon.spAttack =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK) as u16;
            battleMon.spDefense =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF) as u16;
            battleMon.set_isEgg(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_IS_EGG,
            ));
            battleMon.set_abilityNum(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ABILITY_NUM,
            ));
            battleMon.otId = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_OT_ID);
            GetMonData3(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
            StringCopy_Nickname(battleMon.nickname.as_mut_ptr(), nickname.as_mut_ptr());
            GetMonData3(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_OT_NAME,
                battleMon.otName.as_mut_ptr(),
            );
            src = &raw mut battleMon as *mut u8;
            for size in 0..88i32 {
                *dst.at(size) = *src.at(size);
            }
        }
        REQUEST_SPECIES_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPECIES) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_HELDITEM_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HELD_ITEM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MOVES_PP_BATTLE => {
            for size in 0..MAX_MON_MOVES {
                moveData.moves[size] =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MOVE1 + size) as u16;
                moveData.pp[size] =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP1 + size) as u8;
            }
            moveData.ppBonuses =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP_BONUSES) as u8;
            src = &raw mut moveData as *mut u8;
            for size in 0..16i32 {
                *dst.at(size) = *src.at(size);
            }
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            data16 = GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MOVE1
                    + ({
                        let gl_i35 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i35][1])
                    }) as i32
                    - REQUEST_MOVE1_BATTLE,
            ) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_PP_DATA_BATTLE => {
            size = 0;
            while size < MAX_MON_MOVES {
                *dst.at(size) =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            *dst.at(size) =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP_BONUSES) as u8;
            size += 1;
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            *dst = GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP1
                    + ({
                        let gl_i36 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i36][1])
                    }) as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
            ) as u8;
            size = 1;
        }
        REQUEST_OTID_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_OT_ID);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_EXP_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_EXP);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_HP_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP_EV) as u8;
            size = 1;
        }
        REQUEST_ATK_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK_EV) as u8;
            size = 1;
        }
        REQUEST_DEF_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF_EV) as u8;
            size = 1;
        }
        REQUEST_SPEED_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED_EV) as u8;
            size = 1;
        }
        REQUEST_SPATK_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK_EV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF_EV) as u8;
            size = 1;
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_FRIENDSHIP) as u8;
            size = 1;
        }
        REQUEST_POKERUS_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_POKERUS) as u8;
            size = 1;
        }
        REQUEST_MET_LOCATION_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MET_LOCATION) as u8;
            size = 1;
        }
        REQUEST_MET_LEVEL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MET_LEVEL) as u8;
            size = 1;
        }
        REQUEST_MET_GAME_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MET_GAME) as u8;
            size = 1;
        }
        REQUEST_POKEBALL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_POKEBALL) as u8;
            size = 1;
        }
        REQUEST_ALL_IVS_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP_IV) as u8;
            *dst.at(1) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK_IV) as u8;
            *dst.at(2) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF_IV) as u8;
            *dst.at(3) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED_IV) as u8;
            *dst.at(4) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK_IV) as u8;
            *dst.at(5) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF_IV) as u8;
            size = 6;
        }
        REQUEST_HP_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP_IV) as u8;
            size = 1;
        }
        REQUEST_ATK_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK_IV) as u8;
            size = 1;
        }
        REQUEST_DEF_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF_IV) as u8;
            size = 1;
        }
        REQUEST_SPEED_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED_IV) as u8;
            size = 1;
        }
        REQUEST_SPATK_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK_IV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF_IV) as u8;
            size = 1;
        }
        REQUEST_PERSONALITY_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PERSONALITY);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_CHECKSUM_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_CHECKSUM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_STATUS_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_STATUS);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_LEVEL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_LEVEL) as u8;
            size = 1;
        }
        REQUEST_HP_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MAX_HP_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MAX_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_ATK_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_DEF_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPEED_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPATK_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPDEF_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_COOL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_COOL) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_BEAUTY) as u8;
            size = 1;
        }
        REQUEST_CUTE_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_CUTE) as u8;
            size = 1;
        }
        REQUEST_SMART_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SMART) as u8;
            size = 1;
        }
        REQUEST_TOUGH_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_TOUGH) as u8;
            size = 1;
        }
        REQUEST_SHEEN_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SHEEN) as u8;
            size = 1;
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_COOL_RIBBON) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_BEAUTY_RIBBON) as u8;
            size = 1;
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_CUTE_RIBBON) as u8;
            size = 1;
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SMART_RIBBON) as u8;
            size = 1;
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_TOUGH_RIBBON) as u8;
            size = 1;
        }
        _ => {}
    }
    size as u32
}
pub(crate) unsafe fn WallyHandleGetRawMonData() {
    PlayerHandleGetRawMonData();
}
pub(crate) unsafe fn WallyHandleSetMonData() {
    let mut monToCheck: u8 = 0;
    if ({
        let gl_i37 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i37][2])
    }) == 0
    {
        SetWallyMonData(
            ({
                let gl_i38 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i38])
            }) as u8,
        );
    } else {
        monToCheck = {
            let gl_i39 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i39][2])
        };
        for i in 0..(PARTY_SIZE as u8) {
            if monToCheck as i32 & 1 != 0 {
                SetWallyMonData(i);
            }
            monToCheck >>= 1;
        }
    }
    WallyBufferExecCompleted();
}
unsafe fn SetWallyMonData(monId: u8) {
    let battlePokemon: *mut BattlePokemon =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][3] as *mut BattlePokemon;
    let moveData: *mut MovePPInfo =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][3] as *mut MovePPInfo;
    match {
        let gl_i40 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i40][1])
    } {
        REQUEST_ALL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPECIES,
                &raw mut (*battlePokemon).species as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HELD_ITEM,
                &raw mut (*battlePokemon).item as *mut c_void,
            );
            for i in 0..MAX_MON_MOVES {
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_MOVE1 + i,
                    &raw mut (*battlePokemon).moves[i] as *mut c_void,
                );
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_PP1 + i,
                    &raw mut (*battlePokemon).pp[i] as *mut c_void,
                );
            }
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP_BONUSES,
                &raw mut (*battlePokemon).ppBonuses as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_FRIENDSHIP,
                &raw mut (*battlePokemon).friendship as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_EXP,
                &raw mut (*battlePokemon).experience as *mut c_void,
            );
            let mut iv: u8 = (*battlePokemon).hpIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).attackIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).defenseIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).speedIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spAttackIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spDefenseIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_IV,
                &raw mut iv as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PERSONALITY,
                &raw mut (*battlePokemon).personality as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_STATUS,
                &raw mut (*battlePokemon).status1 as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_LEVEL,
                &raw mut (*battlePokemon).level as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP,
                &raw mut (*battlePokemon).hp as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MAX_HP,
                &raw mut (*battlePokemon).maxHP as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK,
                &raw mut (*battlePokemon).attack as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF,
                &raw mut (*battlePokemon).defense as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED,
                &raw mut (*battlePokemon).speed as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK,
                &raw mut (*battlePokemon).spAttack as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF,
                &raw mut (*battlePokemon).spDefense as *mut c_void,
            );
        }
        REQUEST_SPECIES_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPECIES,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_HELDITEM_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HELD_ITEM,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MOVES_PP_BATTLE => {
            for i in 0..MAX_MON_MOVES {
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_MOVE1 + i,
                    &raw mut (*moveData).moves[i] as *mut c_void,
                );
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_PP1 + i,
                    &raw mut (*moveData).pp[i] as *mut c_void,
                );
            }
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP_BONUSES,
                &raw mut (*moveData).ppBonuses as *mut c_void,
            );
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MOVE1
                    + ({
                        let gl_i41 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i41][1])
                    }) as i32
                    - REQUEST_MOVE1_BATTLE,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_PP_DATA_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP1,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP2,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP3,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][5] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP4,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][6] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP_BONUSES,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][7] as *mut c_void,
            );
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP1
                    + ({
                        let gl_i42 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i42][1])
                    }) as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_OTID_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_OT_ID,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_EXP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_EXP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_HP_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ATK_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_DEF_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_FRIENDSHIP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_POKERUS_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_POKERUS,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MET_LOCATION_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MET_LOCATION,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MET_LEVEL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MET_LEVEL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MET_GAME_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MET_GAME,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_POKEBALL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_POKEBALL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ALL_IVS_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][5] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][6] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][7] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][8] as *mut c_void,
            );
        }
        REQUEST_HP_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ATK_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_DEF_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_PERSONALITY_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PERSONALITY,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_CHECKSUM_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_CHECKSUM,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_STATUS_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_STATUS,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_LEVEL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_LEVEL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_HP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MAX_HP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MAX_HP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ATK_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_DEF_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_COOL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_COOL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_BEAUTY,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_CUTE,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SMART_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SMART,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_TOUGH,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SHEEN_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SHEEN,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_COOL_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_BEAUTY_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_CUTE_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SMART_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_TOUGH_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        _ => {}
    }
    HandleLowHpMusicChange(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i43 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i43])
        }],
        gActiveBattler.get(),
    );
}
pub(crate) unsafe fn WallyHandleSetRawMonData() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleLoadMonSprite() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleSwitchInAnim() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleReturnMonToBall() {
    if ({
        let gl_i44 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i44][1])
    }) == 0
    {
        InitAndLaunchSpecialAnimation(
            gActiveBattler.get(),
            gActiveBattler.get(),
            gActiveBattler.get(),
            B_ANIM_SWITCH_OUT_PLAYER_MON,
        );
        {
            let gl_i45 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i45] = Some(DoSwitchOutAnimation))
        };
    } else {
        FreeSpriteOamMatrix(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i46 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i46])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i47 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i47])
            }],
        );
        SetHealthboxSpriteInvisible({
            let gl_i48 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i48])
        });
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn WallyHandleDrawTrainerPic() {
    DecompressTrainerBackPic(TRAINER_BACK_PIC_WALLY, gActiveBattler.get());
    SetMultiuseSpriteTemplateToTrainerBack(
        TRAINER_BACK_PIC_WALLY,
        GetBattlerPosition(gActiveBattler.get()),
    );
    {
        let gl_v = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            80,
            80 + 4
                * (8 - (*(&raw const crate::data::data_tables::gTrainerBackPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[6]
                    .size as i16),
            30,
        );
        let gl_i49 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i49] = gl_v)
    };
    (*gSprites.as_ptr())[{
        let gl_i50 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i50])
    }]
    .oam
    .set_paletteNum(gActiveBattler.get() as u16);
    (*gSprites.as_ptr())[{
        let gl_i51 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i51])
    }]
    .x2 = DISPLAY_WIDTH as i16;
    (*gSprites.as_ptr())[{
        let gl_i52 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i52])
    }]
    .data[sSpeedX] = -2;
    (*gSprites.as_ptr())[{
        let gl_i53 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i53])
    }]
    .callback = Some(SpriteCB_TrainerSlideIn);
    {
        let gl_i54 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i54] = Some(CompleteOnBattlerSpriteCallbackDummy))
    };
}
pub(crate) unsafe fn WallyHandleTrainerSlide() {
    DecompressTrainerBackPic(TRAINER_BACK_PIC_WALLY, gActiveBattler.get());
    SetMultiuseSpriteTemplateToTrainerBack(
        TRAINER_BACK_PIC_WALLY,
        GetBattlerPosition(gActiveBattler.get()),
    );
    {
        let gl_v = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            80,
            80 + 4
                * (8 - (*(&raw const crate::data::data_tables::gTrainerBackPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[6]
                    .size as i16),
            30,
        );
        let gl_i55 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i55] = gl_v)
    };
    (*gSprites.as_ptr())[{
        let gl_i56 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i56])
    }]
    .oam
    .set_paletteNum(gActiveBattler.get() as u16);
    (*gSprites.as_ptr())[{
        let gl_i57 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i57])
    }]
    .x2 = -96;
    (*gSprites.as_ptr())[{
        let gl_i58 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i58])
    }]
    .data[sSpeedX] = 2;
    (*gSprites.as_ptr())[{
        let gl_i59 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i59])
    }]
    .callback = Some(SpriteCB_TrainerSlideIn);
    {
        let gl_i60 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i60] = Some(CompleteOnBankSpriteCallbackDummy2))
    };
}
pub(crate) unsafe fn WallyHandleTrainerSlideBack() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleFaintAnimation() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandlePaletteFade() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleSuccessBallThrowAnim() {
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).ballThrowCaseId = BALL_3_SHAKES_SUCCESS;
    gDoingBattleAnim.set(TRUE);
    InitAndLaunchSpecialAnimation(
        gActiveBattler.get(),
        gActiveBattler.get(),
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW_WITH_TRAINER,
    );
    {
        let gl_i61 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i61] = Some(CompleteOnFinishedAnimation))
    };
}
pub(crate) unsafe fn WallyHandleBallThrowAnim() {
    let ballThrowCaseId: u8 = {
        let gl_i62 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i62][1])
    };
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).ballThrowCaseId = ballThrowCaseId;
    gDoingBattleAnim.set(TRUE);
    InitAndLaunchSpecialAnimation(
        gActiveBattler.get(),
        gActiveBattler.get(),
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW_WITH_TRAINER,
    );
    {
        let gl_i63 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i63] = Some(CompleteOnFinishedAnimation))
    };
}
pub(crate) unsafe fn WallyHandlePause() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleMoveAnimation() {
    let r#move: u16 = ({
        let gl_i64 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i64][1])
    }) as u16
        | (({
            let gl_i65 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i65][2])
        }) as u16)
            << 8;
    gAnimMoveTurn.set({
        let gl_i66 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i66][3])
    });
    gAnimMovePower.set(
        ({
            let gl_i67 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i67][4])
        }) as u16
            | (({
                let gl_i68 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i68][5])
            }) as u16)
                << 8,
    );
    gAnimMoveDmg.set(
        ({
            let gl_i69 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i69][6])
        }) as i32
            | (({
                let gl_i70 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i70][7])
            }) as i32)
                << 8
            | (({
                let gl_i71 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i71][8])
            }) as i32)
                << 16
            | (({
                let gl_i72 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i72][9])
            }) as i32)
                << 24,
    );
    gAnimFriendship.set({
        let gl_i73 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i73][10])
    });
    gWeatherMoveAnim.set(
        ({
            let gl_i74 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i74][12])
        }) as u16
            | (({
                let gl_i75 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i75][13])
            }) as u16)
                << 8,
    );
    crate::gl!(
        gAnimDisableStructPtr =
            &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                .cast::<CArray<CArray<u8, 512>, 4>>()
                .cast_mut())[gActiveBattler.get()][16] as *mut DisableStruct
    );
    {
        let gl_i76 = gActiveBattler.get();
        crate::gl!(
            gTransformedPersonalities[gl_i76] =
                (*crate::gl!(gAnimDisableStructPtr)).transformedMonPersonality
        )
    };
    if IsMoveWithoutAnimation(r#move, gAnimMoveTurn.get()) != 0 {
        WallyBufferExecCompleted();
    } else {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .animationState = 0;
        {
            let gl_i77 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i77] = Some(WallyDoMoveAnimation))
        };
    }
}
pub(crate) unsafe fn WallyDoMoveAnimation() {
    let r#move: u16 = ({
        let gl_i78 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i78][1])
    }) as u16
        | (({
            let gl_i79 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i79][2])
        }) as u16)
            << 8;
    match (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .animationState
    {
        0 => {
            if (*(*crate::gl!(gBattleSpritesDataPtr))
                .battlerData
                .at(gActiveBattler.get()))
            .behindSubstitute()
                != 0
            {
                InitAndLaunchSpecialAnimation(
                    gActiveBattler.get(),
                    gActiveBattler.get(),
                    gActiveBattler.get(),
                    B_ANIM_SUBSTITUTE_TO_MON,
                );
            }
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 1;
        }
        1 => {
            if (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .specialAnimActive()
                == 0
            {
                SetBattlerSpriteAffineMode(ST_OAM_AFFINE_OFF as u8);
                DoMoveAnim(r#move);
                (*(*crate::gl!(gBattleSpritesDataPtr))
                    .healthBoxesData
                    .at(gActiveBattler.get()))
                .animationState = 2;
            }
        }
        2 => {
            (*gAnimScriptCallback.as_ptr()).unwrap_unchecked()();
            if gAnimScriptActive.get() == 0 {
                SetBattlerSpriteAffineMode(ST_OAM_AFFINE_NORMAL as u8);
                if (*(*crate::gl!(gBattleSpritesDataPtr))
                    .battlerData
                    .at(gActiveBattler.get()))
                .behindSubstitute()
                    != 0
                {
                    InitAndLaunchSpecialAnimation(
                        gActiveBattler.get(),
                        gActiveBattler.get(),
                        gActiveBattler.get(),
                        B_ANIM_MON_TO_SUBSTITUTE,
                    );
                }
                (*(*crate::gl!(gBattleSpritesDataPtr))
                    .healthBoxesData
                    .at(gActiveBattler.get()))
                .animationState = 3;
            }
        }
        3 if (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .specialAnimActive()
            == 0 =>
        {
            CopyAllBattleSpritesInvisibilities();
            TrySetBehindSubstituteSpriteBit(
                gActiveBattler.get(),
                ({
                    let gl_i80 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i80][1])
                }) as u16
                    | (({
                        let gl_i81 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i81][2])
                    }) as u16)
                        << 8,
            );
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 0;
            WallyBufferExecCompleted();
        }
        _ => {}
    }
}
pub(crate) unsafe fn WallyHandlePrintString() {
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    let stringId: *mut u16 = &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;)
        .cast_const())
    .cast::<CArray<CArray<u8, 512>, 4>>()
    .cast_mut())[gActiveBattler.get()][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow((*gDisplayedStringBattle.as_ptr()).as_mut_ptr(), B_WIN_MSG);
    {
        let gl_i82 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i82] = Some(CompleteOnInactiveTextPrinter))
    };
}
pub(crate) unsafe fn WallyHandlePrintSelectionString() {
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        WallyHandlePrintString();
    } else {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn HandleChooseActionAfterDma3() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        gBattle_BG0_X.set(0);
        gBattle_BG0_Y.set(DISPLAY_HEIGHT);
        {
            let gl_i83 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i83] = Some(WallyHandleActions))
        };
    }
}
pub(crate) unsafe fn WallyHandleChooseAction() {
    {
        let gl_i84 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i84] = Some(HandleChooseActionAfterDma3))
    };
    BattlePutTextOnWindow(
        (*(&raw const crate::data::battle_message::gText_BattleMenu).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        B_WIN_ACTION_MENU,
    );
    for i in 0..4i32 {
        ActionSelectionDestroyCursorAt(i as u8);
    }
    ActionSelectionCreateCursorAt(
        {
            let gl_i85 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i85])
        },
        0,
    );
    BattleStringExpandPlaceholdersToDisplayedString(
        (*(&raw const crate::data::battle_message::gText_WhatWillWallyDo).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    BattlePutTextOnWindow(
        (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
        B_WIN_ACTION_PROMPT,
    );
}
pub(crate) unsafe fn WallyHandleYesNoBox() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleChooseMove() {
    match (*crate::gl!(gBattleStruct)).wallyMovesState {
        0 => {
            InitMoveSelectionsVarsAndStrings();
            (*crate::gl!(gBattleStruct)).wallyMovesState += 1;
            (*crate::gl!(gBattleStruct)).wallyMoveFrames = 80;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                gBattle_BG0_X.set(0);
                gBattle_BG0_Y.set(320);
                (*crate::gl!(gBattleStruct)).wallyMovesState += 1;
            }
        }
        2 if ({
            (*crate::gl!(gBattleStruct)).wallyMoveFrames -= 1;
            (*crate::gl!(gBattleStruct)).wallyMoveFrames
        }) == 0 =>
        {
            PlaySE(SE_SELECT);
            BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_EXEC_SCRIPT, 0x100);
            WallyBufferExecCompleted();
        }
        _ => {}
    }
}
pub(crate) unsafe fn WallyHandleChooseItem() {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    {
        let gl_i86 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i86] = Some(OpenBagAfterPaletteFade))
    };
    gBattlerInMenuId.set(gActiveBattler.get());
}
pub(crate) unsafe fn WallyHandleChoosePokemon() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleCmd23() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleHealthBarUpdate() {
    LoadBattleBarGfx(0);
    let hpVal: i16 = ({
        let gl_i87 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i87][2])
    }) as i16
        | (({
            let gl_i88 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i88][3])
        }) as i16)
            << 8;
    if hpVal != INSTANT_HP_BAR_DROP {
        let maxHP: u32 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i89 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i89])
            }],
            MON_DATA_MAX_HP,
        );
        let curHP: u32 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i90 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i90])
            }],
            MON_DATA_HP,
        );
        SetBattleBarStruct(
            gActiveBattler.get(),
            {
                let gl_i91 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i91])
            },
            maxHP as i32,
            curHP as i32,
            hpVal as i32,
        );
    } else {
        let maxHP: u32 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i92 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i92])
            }],
            MON_DATA_MAX_HP,
        );
        SetBattleBarStruct(
            gActiveBattler.get(),
            {
                let gl_i93 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i93])
            },
            maxHP as i32,
            0,
            hpVal as i32,
        );
        UpdateHpTextInHealthbox(
            {
                let gl_i94 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i94])
            },
            0,
            HP_CURRENT,
        );
    }
    {
        let gl_i95 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i95] = Some(CompleteOnHealthbarDone))
    };
}
pub(crate) unsafe fn WallyHandleExpUpdate() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleStatusIconUpdate() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleStatusAnimation() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleStatusXor() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleDataTransfer() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleDMA3Transfer() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandlePlayBGM() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleCmd32() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleTwoReturnValues() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleChosenMonReturnValue() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleOneReturnValue() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleOneReturnValue_Duplicate() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleClearUnkVar() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleSetUnkVar() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleClearUnkFlag() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleToggleUnkFlag() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleHitAnimation() {
    if (*gSprites.as_ptr())[{
        let gl_i96 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i96])
    }]
    .invisible()
        == TRUE as u16
    {
        WallyBufferExecCompleted();
    } else {
        gDoingBattleAnim.set(TRUE);
        (*gSprites.as_ptr())[{
            let gl_i97 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i97])
        }]
        .data[1] = 0;
        DoHitAnimHealthboxEffect(gActiveBattler.get());
        {
            let gl_i98 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i98] = Some(DoHitAnimBlinkSpriteEffect))
        };
    }
}
pub(crate) unsafe fn WallyHandleCantSwitch() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandlePlaySE() {
    PlaySE(
        ({
            let gl_i99 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i99][1])
        }) as u16
            | (({
                let gl_i100 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i100][2])
            }) as u16)
                << 8,
    );
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandlePlayFanfareOrBGM() {
    if ({
        let gl_i101 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i101][3])
    }) != 0
    {
        BattleStopLowHpSound();
        PlayBGM(
            ({
                let gl_i102 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i102][1])
            }) as u16
                | (({
                    let gl_i103 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i103][2])
                }) as u16)
                    << 8,
        );
    } else {
        PlayFanfare(
            ({
                let gl_i104 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i104][1])
            }) as u16
                | (({
                    let gl_i105 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i105][2])
                }) as u16)
                    << 8,
        );
    }
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleFaintingCry() {
    let species: u16 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i106 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i106])
        }],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_Normal(species, 25);
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleIntroSlide() {
    HandleIntroSlide({
        let gl_i107 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i107][1])
    });
    gIntroSlideFlags.set(gIntroSlideFlags.get() | (1));
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleIntroTrainerBallThrow() {
    SetSpritePrimaryCoordsFromSecondaryCoords(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i108 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i108])
        }],
    );
    (*gSprites.as_ptr())[{
        let gl_i109 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i109])
    }]
    .data[0] = 50;
    (*gSprites.as_ptr())[{
        let gl_i110 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i110])
    }]
    .data[2] = -40;
    (*gSprites.as_ptr())[{
        let gl_i111 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i111])
    }]
    .data[4] = (*gSprites.as_ptr())[{
        let gl_i112 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i112])
    }]
    .y;
    (*gSprites.as_ptr())[{
        let gl_i113 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i113])
    }]
    .callback = Some(StartAnimLinearTranslation);
    (*gSprites.as_ptr())[{
        let gl_i114 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i114])
    }]
    .data[5] = gActiveBattler.get() as i16;
    StoreSpriteCallbackInData6(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i115 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i115])
        }],
        Some(SpriteCB_FreePlayerSpriteLoadMonSprite),
    );
    StartSpriteAnim(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i116 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i116])
        }],
        1,
    );
    let paletteNum: u8 = AllocSpritePalette(0xD6F8);
    LoadCompressedPalette(
        (*(&raw const crate::data::data_tables::gTrainerBackPicPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[6]
            .data,
        0x100 + paletteNum as u16 * 16,
        32,
    );
    (*gSprites.as_ptr())[{
        let gl_i117 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i117])
    }]
    .oam
    .set_paletteNum(paletteNum as u16);
    let taskId: u8 = CreateTask(Some(Task_StartSendOutAnim), 5);
    task_set(taskId, 0, gActiveBattler.get() as i16);
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .partyStatusSummaryShown()
        != 0
    {
        task_set_func(
            {
                let gl_i118 = gActiveBattler.get();
                crate::gl!(gBattlerStatusSummaryTaskId[gl_i118])
            },
            Some(Task_HidePartyStatusSummary),
        );
    }
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).set_introAnimActive(TRUE);
    {
        let gl_i119 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i119] = Some(BattleControllerDummy))
    };
}
pub(crate) unsafe fn StartSendOutAnim(battler: u8) {
    (*(*crate::gl!(gBattleSpritesDataPtr)).battlerData.at(battler)).transformSpecies = 0;
    crate::gl!(gBattlerPartyIndexes[battler] = crate::gl!(gBattleBufferA[battler][1]) as u16);
    let species: u16 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
        MON_DATA_SPECIES,
    ) as u16;
    crate::gl!(
        gBattleControllerData[battler] =
            CreateInvisibleSpriteWithCallback(Some(SpriteCB_WaitForBattlerBallReleaseAnim))
    );
    SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(battler));
    crate::gl!(
        gBattlerSpriteIds[battler] = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16,
            GetBattlerSpriteDefault_Y(battler) as i16,
            GetBattlerSpriteSubpriority(battler),
        )
    );
    (*gSprites.as_ptr())[crate::gl!(gBattleControllerData[battler])].data[1] =
        crate::gl!(gBattlerSpriteIds[battler]) as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattleControllerData[battler])].data[2] = battler as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].data[0] = battler as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].data[2] = species as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])]
        .oam
        .set_paletteNum(battler as u16);
    StartSpriteAnim(
        &raw mut (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])],
        crate::gl!(gBattleMonForms[battler]),
    );
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].set_invisible(TRUE as u16);
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].callback =
        Some(SpriteCallbackDummy);
    (*gSprites.as_ptr())[crate::gl!(gBattleControllerData[battler])].data[0] =
        DoPokeballSendOutAnimation(0, POKEBALL_PLAYER_SENDOUT) as i16;
}
pub(crate) unsafe fn Task_StartSendOutAnim(taskId: u8) {
    if task_get(taskId, 1) < 31 {
        task_set(taskId, 1, task_get(taskId, 1) + 1);
    } else {
        let savedActiveBank: u8 = gActiveBattler.get();
        gActiveBattler.set(task_get(taskId, 0) as u8);
        {
            let gl_i120 = gActiveBattler.get();
            crate::gl!(
                gBattleBufferA[gl_i120][1] = ({
                    let gl_i121 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i121])
                }) as u8
            )
        };
        StartSendOutAnim(gActiveBattler.get());
        {
            let gl_i122 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i122] = Some(Intro_TryShinyAnimShowHealthbox))
        };
        gActiveBattler.set(savedActiveBank);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn WallyHandleDrawPartyStatusSummary() {
    if ({
        let gl_i123 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i123][1])
    }) != 0
        && GetBattlerSide(gActiveBattler.get()) == 0
    {
        WallyBufferExecCompleted();
    } else {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_partyStatusSummaryShown(1);
        {
            let gl_v = CreatePartyStatusSummarySprites(
                gActiveBattler.get(),
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut HpAndStatus,
                {
                    let gl_i125 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i125][1])
                },
                {
                    let gl_i126 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i126][2])
                },
            );
            let gl_i124 = gActiveBattler.get();
            crate::gl!(gBattlerStatusSummaryTaskId[gl_i124] = gl_v)
        };
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe fn WallyHandleHidePartyStatusSummary() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleEndBounceEffect() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleSpriteInvisibility() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleBattleAnimation() {
    let animationId: u8 = {
        let gl_i127 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i127][1])
    };
    let argument: u16 = ({
        let gl_i128 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i128][2])
    }) as u16
        | (({
            let gl_i129 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i129][3])
        }) as u16)
            << 8;
    if TryHandleLaunchBattleTableAnimation(
        gActiveBattler.get(),
        gActiveBattler.get(),
        gActiveBattler.get(),
        animationId,
        argument,
    ) != 0
    {
        WallyBufferExecCompleted();
    } else {
        {
            let gl_i130 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i130] = Some(CompleteOnFinishedBattleAnimation))
        };
    }
}
pub(crate) unsafe fn WallyHandleLinkStandbyMsg() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleResetActionMoveSelection() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe fn WallyHandleEndLinkBattle() {
    gBattleOutcome.set({
        let gl_i131 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i131][1])
    });
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    WallyBufferExecCompleted();
    if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER == 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
    {
        {
            let gl_i132 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i132] = Some(SetBattleEndCallbacks))
        };
    }
}
pub(crate) fn WallyCmdEnd() {}
