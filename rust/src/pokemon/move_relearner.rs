//! Translated from `src/move_relearner.c` by tools/rustport/c2rs.py.
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
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    unused_assignments
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005};
use crate::field_screen_effect::FieldCB_ContinueScriptHandleMusic;
use crate::gpu_regs::SetGpuReg;
use crate::list_menu::{
    AddScrollIndicatorArrowPair, DestroyListMenuTask, ListMenu_ProcessInput,
    ListMenuGetScrollAndRow, ListMenuInit, RemoveScrollIndicatorArrowPair,
    gMultiuseListMenuTemplate, gTempScrollArrowTemplate,
};
use crate::menu::{
    ClearScheduledBgCopiesToVram, DoScheduledBgTilemapCopiesToVram,
    Menu_ProcessInputNoWrapClearOnChoose, ScheduleBgCopyTilemapToVram,
};
use crate::menu_helpers::{GetLRKeysPressed, ResetAllBgsCoordinates, ResetVramOamAndBgCntRegs};
use crate::menu_specialized::{
    InitMoveRelearnerWindows, LoadMoveRelearnerMovesList, MoveRelearnerCreateYesNoMenu,
    MoveRelearnerPrintMessage, MoveRelearnerRunTextPrinters,
};
use crate::overworld::{CB2_ReturnToField, gFieldCallback};
use crate::palette::{
    BeginNormalPaletteFade, FillPalette, TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::pokemon::{
    GetMonData2, GetMonData3, GetMoveRelearnerMoves, GiveMoveToMon, RemoveMonPPBonus,
    SetMonMoveSlot, gPlayerParty, gPlayerPartyCount,
};
use crate::pokemon_summary_screen::ShowSelectMovePokemonSummaryScreen;
use crate::script::LockPlayerFieldControls;
use crate::sound::{IsFanfareTaskInactive, PlayFanfare, PlaySE};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers, PutWindowTilemap,
};
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
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
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
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCopy_Nickname` with this module's view of its types.
#[inline]
unsafe fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy_Nickname(a0 as _, a1 as _) as *mut u8 }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
// Data tables (translate with cdata.py): sUI_Pal sUI_Tiles sHeartSpriteOamData sUnusedOam1 sUnusedOam2 sMoveRelearnerSpriteSheet sMoveRelearnerPalette sDisplayModeArrowsTemplate sMoveListScrollArrowsTemplate sHeartSprite_AppealEmptyFrame sHeartSprite_AppealFullFrame sHeartSprite_JamEmptyFrame sHeartSprite_JamFullFrame sHeartSpriteAnimationCommands sConstestMoveHeartSprite sMoveRelearnerMenuBackgroundTemplates

/// `__typeof__(*((__typeof__(sMoveRelearnerStruct))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sMoveRelearnerStruct_0_t {
    pub state: u8,
    pub heartSpriteIds: CArray<u8, 16>,
    pub movesToLearn: CArray<u16, 25>,
    pub partyMon: u8,
    pub moveSlot: u8,
    pub menuItems: CArray<ListMenuItem, 25>,
    pub numMenuChoices: u8,
    pub numToShowAtOnce: u8,
    pub moveListMenuTask: u8,
    pub moveListScrollArrowTask: u8,
    pub moveDisplayArrowTask: u8,
    pub scrollOffset: u16,
}

unsafe impl Sync for typeof___sMoveRelearnerStruct_0_t {}

/// `__typeof__(sMoveRelearnerMenuState)`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sMoveRelearnerMenuState_t {
    pub listOffset: u16,
    pub listRow: u16,
    pub showContestInfo: u8,
}

unsafe impl Sync for sMoveRelearnerMenuState_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<typeof___sMoveRelearnerStruct_0_t>() == 280);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, state) == 0);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, heartSpriteIds) == 1);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, movesToLearn) == 18);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, partyMon) == 68);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, moveSlot) == 69);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, menuItems) == 72);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, numMenuChoices) == 272);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, numToShowAtOnce) == 273);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, moveListMenuTask) == 274);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, moveListScrollArrowTask) == 275);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, moveDisplayArrowTask) == 276);
    assert!(offset_of!(typeof___sMoveRelearnerStruct_0_t, scrollOffset) == 278);
    assert!(size_of::<sMoveRelearnerMenuState_t>() == 8);
    assert!(offset_of!(sMoveRelearnerMenuState_t, listOffset) == 0);
    assert!(offset_of!(sMoveRelearnerMenuState_t, listRow) == 2);
    assert!(offset_of!(sMoveRelearnerMenuState_t, showContestInfo) == 4);
};

const MENU_STATE_CHOOSE_SETUP_STATE: u8 = 27;
const MENU_STATE_CONFIRM_DELETE_OLD_MOVE: u8 = 18;
const MENU_STATE_CONFIRM_STOP_TEACHING: u8 = 26;
const MENU_STATE_DOUBLE_FANFARE_FORGOT_MOVE: u8 = 30;
const MENU_STATE_FADE_AND_RETURN: u8 = 14;
const MENU_STATE_FADE_FROM_SUMMARY_SCREEN: u8 = 28;
const MENU_STATE_FADE_TO_BLACK: u8 = 0;
const MENU_STATE_GIVE_UP_CONFIRM: u8 = 13;
const MENU_STATE_IDLE_BATTLE_MODE: u8 = 4;
const MENU_STATE_IDLE_CONTEST_MODE: u8 = 6;
const MENU_STATE_PRINT_GIVE_UP_PROMPT: u8 = 12;
const MENU_STATE_PRINT_STOP_TEACHING: u8 = 24;
const MENU_STATE_PRINT_TEACH_MOVE_PROMPT: u8 = 8;
const MENU_STATE_PRINT_TEXT_THEN_FANFARE: u8 = 31;
const MENU_STATE_PRINT_TRYING_TO_LEARN_PROMPT: u8 = 16;
const MENU_STATE_PRINT_WHICH_MOVE_PROMPT: u8 = 19;
const MENU_STATE_RETURN_TO_FIELD: u8 = 15;
const MENU_STATE_SETUP_BATTLE_MODE: u8 = 3;
const MENU_STATE_SETUP_CONTEST_MODE: u8 = 5;
const MENU_STATE_SHOW_MOVE_SUMMARY_SCREEN: u8 = 20;
const MENU_STATE_TEACH_MOVE_CONFIRM: u8 = 9;
const MENU_STATE_TRY_OVERWRITE_MOVE: u8 = 29;
const MENU_STATE_UNREACHABLE: u8 = 2;
const MENU_STATE_WAIT_FOR_A_BUTTON: u8 = 33;
const MENU_STATE_WAIT_FOR_FADE: u8 = 1;
const MENU_STATE_WAIT_FOR_FANFARE: u8 = 32;
const MENU_STATE_WAIT_FOR_STOP_TEACHING: u8 = 25;
const MENU_STATE_WAIT_FOR_TRYING_TO_LEARN: u8 = 17;

static sConstestMoveHeartSprite: Table<SpriteTemplate> =
    Table((&raw const crate::data::move_relearner::sConstestMoveHeartSprite).cast());
static sDisplayModeArrowsTemplate: Table<ScrollArrowsTemplate> =
    Table((&raw const crate::data::move_relearner::sDisplayModeArrowsTemplate).cast());
static sMoveListScrollArrowsTemplate: Table<ScrollArrowsTemplate> =
    Table((&raw const crate::data::move_relearner::sMoveListScrollArrowsTemplate).cast());
static sMoveRelearnerMenuBackgroundTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::move_relearner::sMoveRelearnerMenuBackgroundTemplates).cast());
static sMoveRelearnerPalette: Table<SpritePalette> =
    Table((&raw const crate::data::move_relearner::sMoveRelearnerPalette).cast());
static sMoveRelearnerSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::move_relearner::sMoveRelearnerSpriteSheet).cast());

pub(crate) static mut sMoveRelearnerStruct: *mut typeof___sMoveRelearnerStruct_0_t = null_mut();
pub(crate) static mut sMoveRelearnerMenuState: sMoveRelearnerMenuState_t = unsafe { zeroed() };

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn VBlankCB_MoveRelearner() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe fn TeachMoveRelearnerMove() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_WaitForFadeOut), 10);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
}
pub(crate) unsafe fn Task_WaitForFadeOut(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetMainCallback2(Some(CB2_InitLearnMove));
        gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn CB2_InitLearnMove() {
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetTasks();
    ClearScheduledBgCopiesToVram();
    sMoveRelearnerStruct = AllocZeroed(280) as *mut typeof___sMoveRelearnerStruct_0_t;
    (*sMoveRelearnerStruct).partyMon = gSpecialVar_0x8004 as u8;
    SetVBlankCallback(Some(VBlankCB_MoveRelearner));
    InitMoveRelearnerBackgroundLayers();
    InitMoveRelearnerWindows(FALSE);
    sMoveRelearnerMenuState.listOffset = 0;
    sMoveRelearnerMenuState.listRow = 0;
    sMoveRelearnerMenuState.showContestInfo = FALSE;
    CreateLearnableMovesList();
    LoadSpriteSheet((&raw const *sMoveRelearnerSpriteSheet).cast_mut());
    LoadSpritePalette((&raw const *sMoveRelearnerPalette).cast_mut());
    CreateUISprites();
    (*sMoveRelearnerStruct).moveListMenuTask = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        sMoveRelearnerMenuState.listOffset,
        sMoveRelearnerMenuState.listRow,
    );
    SetBackdropFromColor(0);
    SetMainCallback2(Some(CB2_MoveRelearnerMain));
}
pub(crate) unsafe fn CB2_InitLearnMoveReturnFromSelectMove() {
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetTasks();
    ClearScheduledBgCopiesToVram();
    sMoveRelearnerStruct = AllocZeroed(280) as *mut typeof___sMoveRelearnerStruct_0_t;
    (*sMoveRelearnerStruct).state = MENU_STATE_FADE_FROM_SUMMARY_SCREEN;
    (*sMoveRelearnerStruct).partyMon = gSpecialVar_0x8004 as u8;
    (*sMoveRelearnerStruct).moveSlot = gSpecialVar_0x8005 as u8;
    SetVBlankCallback(Some(VBlankCB_MoveRelearner));
    InitMoveRelearnerBackgroundLayers();
    InitMoveRelearnerWindows(sMoveRelearnerMenuState.showContestInfo);
    CreateLearnableMovesList();
    LoadSpriteSheet((&raw const *sMoveRelearnerSpriteSheet).cast_mut());
    LoadSpritePalette((&raw const *sMoveRelearnerPalette).cast_mut());
    CreateUISprites();
    (*sMoveRelearnerStruct).moveListMenuTask = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        sMoveRelearnerMenuState.listOffset,
        sMoveRelearnerMenuState.listRow,
    );
    SetBackdropFromColor(0);
    SetMainCallback2(Some(CB2_MoveRelearnerMain));
}
unsafe fn InitMoveRelearnerBackgroundLayers() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(
        0,
        sMoveRelearnerMenuBackgroundTemplates.as_ptr().cast_mut(),
        2,
    );
    ResetAllBgsCoordinates();
    SetGpuReg(0x0, 4160);
    ShowBg(0);
    ShowBg(1);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
pub(crate) unsafe fn CB2_MoveRelearnerMain() {
    DoMoveRelearnerMain();
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
unsafe fn PrintMessageWithPlaceholders(src: *mut u8) {
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), src);
    MoveRelearnerPrintMessage(gStringVar4.as_mut_ptr());
}
unsafe fn DoMoveRelearnerMain() {
    match (*sMoveRelearnerStruct).state {
        MENU_STATE_FADE_TO_BLACK => {
            (*sMoveRelearnerStruct).state += 1;
            HideHeartSpritesAndShowTeachMoveText(FALSE);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
        }
        MENU_STATE_WAIT_FOR_FADE => {
            if gPaletteFade.active() == 0 {
                (*sMoveRelearnerStruct).state = MENU_STATE_IDLE_BATTLE_MODE;
            }
        }
        MENU_STATE_UNREACHABLE => {
            (*sMoveRelearnerStruct).state += 1;
        }
        MENU_STATE_SETUP_BATTLE_MODE => {
            HideHeartSpritesAndShowTeachMoveText(FALSE);
            (*sMoveRelearnerStruct).state += 1;
            AddScrollArrows();
        }
        MENU_STATE_IDLE_BATTLE_MODE => {
            HandleInput(FALSE);
        }
        MENU_STATE_SETUP_CONTEST_MODE => {
            ShowTeachMoveText(FALSE);
            (*sMoveRelearnerStruct).state += 1;
            AddScrollArrows();
        }
        MENU_STATE_IDLE_CONTEST_MODE => {
            HandleInput(TRUE);
        }
        MENU_STATE_PRINT_TEACH_MOVE_PROMPT => {
            if MoveRelearnerRunTextPrinters() == 0 {
                MoveRelearnerCreateYesNoMenu();
                (*sMoveRelearnerStruct).state += 1;
            }
        }
        MENU_STATE_TEACH_MOVE_CONFIRM => {
            let selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
            if selection == 0 {
                if GiveMoveToMon(
                    &raw mut gPlayerParty[(*sMoveRelearnerStruct).partyMon],
                    GetCurrentSelectedMove() as u16,
                ) != MON_HAS_MAX_MOVES
                {
                    PrintMessageWithPlaceholders(
                        (*(&raw const crate::data::strings::gText_MoveRelearnerPkmnLearnedMove)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    gSpecialVar_0x8004 = TRUE as u16;
                    (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_TEXT_THEN_FANFARE;
                } else {
                    (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_TRYING_TO_LEARN_PROMPT;
                }
            } else if selection == MENU_B_PRESSED || selection == 1 {
                if sMoveRelearnerMenuState.showContestInfo == FALSE {
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_BATTLE_MODE;
                } else if sMoveRelearnerMenuState.showContestInfo == TRUE {
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_CONTEST_MODE;
                }
            }
        }
        MENU_STATE_PRINT_GIVE_UP_PROMPT => {
            if MoveRelearnerRunTextPrinters() == 0 {
                MoveRelearnerCreateYesNoMenu();
                (*sMoveRelearnerStruct).state += 1;
            }
        }
        MENU_STATE_GIVE_UP_CONFIRM => {
            let selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
            if selection == 0 {
                gSpecialVar_0x8004 = FALSE as u16;
                (*sMoveRelearnerStruct).state = MENU_STATE_FADE_AND_RETURN;
            } else if selection == MENU_B_PRESSED || selection == 1 {
                if sMoveRelearnerMenuState.showContestInfo == FALSE {
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_BATTLE_MODE;
                } else if sMoveRelearnerMenuState.showContestInfo == TRUE {
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_CONTEST_MODE;
                }
            }
        }
        MENU_STATE_PRINT_TRYING_TO_LEARN_PROMPT => {
            PrintMessageWithPlaceholders(
                (*(&raw const crate::data::strings::gText_MoveRelearnerPkmnTryingToLearnMove)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            (*sMoveRelearnerStruct).state += 1;
        }
        MENU_STATE_WAIT_FOR_TRYING_TO_LEARN => {
            if MoveRelearnerRunTextPrinters() == 0 {
                MoveRelearnerCreateYesNoMenu();
                (*sMoveRelearnerStruct).state = MENU_STATE_CONFIRM_DELETE_OLD_MOVE;
            }
        }
        MENU_STATE_CONFIRM_DELETE_OLD_MOVE => {
            let selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
            if selection == 0 {
                PrintMessageWithPlaceholders(
                    (*(&raw const crate::data::strings::gText_MoveRelearnerWhichMoveToForget)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_WHICH_MOVE_PROMPT;
            } else if selection == MENU_B_PRESSED || selection == 1 {
                (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_STOP_TEACHING;
            }
        }
        MENU_STATE_PRINT_STOP_TEACHING => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                (*(&raw const crate::data::data_tables::gMoveNames)
                    .cast::<CArray<CArray<u8, 13>, 355>>())[GetCurrentSelectedMove()]
                .as_ptr()
                .cast_mut(),
            );
            PrintMessageWithPlaceholders(
                (*(&raw const crate::data::strings::gText_MoveRelearnerStopTryingToTeachMove)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            (*sMoveRelearnerStruct).state += 1;
        }
        MENU_STATE_WAIT_FOR_STOP_TEACHING => {
            if MoveRelearnerRunTextPrinters() == 0 {
                MoveRelearnerCreateYesNoMenu();
                (*sMoveRelearnerStruct).state += 1;
            }
        }
        MENU_STATE_CONFIRM_STOP_TEACHING => {
            let selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
            if selection == 0 {
                (*sMoveRelearnerStruct).state = MENU_STATE_CHOOSE_SETUP_STATE;
            } else if selection == MENU_B_PRESSED || selection == 1 {
                if sMoveRelearnerMenuState.showContestInfo == FALSE {
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_BATTLE_MODE;
                } else if sMoveRelearnerMenuState.showContestInfo == TRUE {
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_CONTEST_MODE;
                }
                (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_TRYING_TO_LEARN_PROMPT;
            }
        }
        MENU_STATE_CHOOSE_SETUP_STATE => {
            if MoveRelearnerRunTextPrinters() == 0 {
                FillWindowPixelBuffer(RELEARNERWIN_MSG, 0x11);
                if sMoveRelearnerMenuState.showContestInfo == FALSE {
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_BATTLE_MODE;
                } else if sMoveRelearnerMenuState.showContestInfo == TRUE {
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_CONTEST_MODE;
                }
            }
        }
        MENU_STATE_PRINT_WHICH_MOVE_PROMPT => {
            if MoveRelearnerRunTextPrinters() == 0 {
                (*sMoveRelearnerStruct).state = MENU_STATE_SHOW_MOVE_SUMMARY_SCREEN;
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            }
        }
        MENU_STATE_SHOW_MOVE_SUMMARY_SCREEN => {
            if gPaletteFade.active() == 0 {
                ShowSelectMovePokemonSummaryScreen(
                    gPlayerParty.as_mut_ptr(),
                    (*sMoveRelearnerStruct).partyMon,
                    gPlayerPartyCount - 1,
                    Some(CB2_InitLearnMoveReturnFromSelectMove),
                    GetCurrentSelectedMove() as u16,
                );
                FreeMoveRelearnerResources();
            }
        }
        21 => {
            if MoveRelearnerRunTextPrinters() == 0 {
                (*sMoveRelearnerStruct).state = MENU_STATE_FADE_AND_RETURN;
            }
        }
        22 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
        }
        MENU_STATE_FADE_AND_RETURN => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sMoveRelearnerStruct).state += 1;
        }
        MENU_STATE_RETURN_TO_FIELD => {
            if gPaletteFade.active() == 0 {
                FreeMoveRelearnerResources();
                SetMainCallback2(Some(CB2_ReturnToField));
            }
        }
        MENU_STATE_FADE_FROM_SUMMARY_SCREEN => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*sMoveRelearnerStruct).state += 1;
            if sMoveRelearnerMenuState.showContestInfo == FALSE {
                HideHeartSpritesAndShowTeachMoveText(TRUE);
            } else if sMoveRelearnerMenuState.showContestInfo == TRUE {
                ShowTeachMoveText(TRUE);
            }
            RemoveScrollArrows();
            CopyWindowToVram(RELEARNERWIN_MSG, COPYWIN_GFX);
        }
        MENU_STATE_TRY_OVERWRITE_MOVE => {
            if gPaletteFade.active() == 0 {
                if (*sMoveRelearnerStruct).moveSlot == MAX_MON_MOVES as u8 {
                    (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_STOP_TEACHING;
                } else {
                    let r#move: u16 = GetMonData2(
                        &raw mut gPlayerParty[(*sMoveRelearnerStruct).partyMon],
                        MON_DATA_MOVE1 + (*sMoveRelearnerStruct).moveSlot as i32,
                    ) as u16;
                    StringCopy(
                        gStringVar3.as_mut_ptr(),
                        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<
                            CArray<u8, 13>,
                            355,
                        >>(
                        ))[r#move]
                            .as_ptr()
                            .cast_mut(),
                    );
                    RemoveMonPPBonus(
                        &raw mut gPlayerParty[(*sMoveRelearnerStruct).partyMon],
                        (*sMoveRelearnerStruct).moveSlot,
                    );
                    SetMonMoveSlot(
                        &raw mut gPlayerParty[(*sMoveRelearnerStruct).partyMon],
                        GetCurrentSelectedMove() as u16,
                        (*sMoveRelearnerStruct).moveSlot,
                    );
                    StringCopy(
                        gStringVar2.as_mut_ptr(),
                        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<
                            CArray<u8, 13>,
                            355,
                        >>(
                        ))[GetCurrentSelectedMove()]
                        .as_ptr()
                        .cast_mut(),
                    );
                    PrintMessageWithPlaceholders(
                        (*(&raw const crate::data::strings::gText_MoveRelearnerAndPoof)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    (*sMoveRelearnerStruct).state = MENU_STATE_DOUBLE_FANFARE_FORGOT_MOVE;
                    gSpecialVar_0x8004 = TRUE as u16;
                }
            }
        }
        MENU_STATE_DOUBLE_FANFARE_FORGOT_MOVE => {
            if MoveRelearnerRunTextPrinters() == 0 {
                PrintMessageWithPlaceholders(
                    (*(&raw const crate::data::strings::gText_MoveRelearnerPkmnForgotMoveAndLearnedNew).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_TEXT_THEN_FANFARE;
                PlayFanfare(MUS_LEVEL_UP);
            }
        }
        MENU_STATE_PRINT_TEXT_THEN_FANFARE => {
            if MoveRelearnerRunTextPrinters() == 0 {
                PlayFanfare(MUS_LEVEL_UP);
                (*sMoveRelearnerStruct).state = MENU_STATE_WAIT_FOR_FANFARE;
            }
        }
        MENU_STATE_WAIT_FOR_FANFARE => {
            if IsFanfareTaskInactive() != 0 {
                (*sMoveRelearnerStruct).state = MENU_STATE_WAIT_FOR_A_BUTTON;
            }
        }
        MENU_STATE_WAIT_FOR_A_BUTTON if gMain.newKeys as i32 & A_BUTTON != 0 => {
            PlaySE(SE_SELECT);
            (*sMoveRelearnerStruct).state = MENU_STATE_FADE_AND_RETURN;
        }
        _ => {}
    }
}
unsafe fn FreeMoveRelearnerResources() {
    RemoveScrollArrows();
    DestroyListMenuTask(
        (*sMoveRelearnerStruct).moveListMenuTask,
        &raw mut sMoveRelearnerMenuState.listOffset,
        &raw mut sMoveRelearnerMenuState.listRow,
    );
    FreeAllWindowBuffers();
    Free(sMoveRelearnerStruct as *mut c_void);
    sMoveRelearnerStruct = null_mut();
    ResetSpriteData();
    FreeAllSpritePalettes();
}
unsafe fn HideHeartSpritesAndShowTeachMoveText(onlyHideSprites: u8) {
    for i in 0..16i32 {
        gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i]].set_invisible(TRUE as u16);
    }
    if onlyHideSprites == 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_TeachWhichMoveToPkmn)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
        FillWindowPixelBuffer(RELEARNERWIN_MSG, 0x11);
        AddTextPrinterParameterized(
            RELEARNERWIN_MSG,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            0,
            1,
            0,
            None,
        );
    }
}
pub(crate) unsafe fn HandleInput(showContest: u8) {
    let itemId: i32 = ListMenu_ProcessInput((*sMoveRelearnerStruct).moveListMenuTask);
    ListMenuGetScrollAndRow(
        (*sMoveRelearnerStruct).moveListMenuTask,
        &raw mut sMoveRelearnerMenuState.listOffset,
        &raw mut sMoveRelearnerMenuState.listRow,
    );
    'l1: {
        match itemId {
            LIST_NOTHING_CHOSEN => {
                if gMain.newKeys as i32 & 48 == 0 && GetLRKeysPressed() == 0 {
                    break 'l1;
                }
                PlaySE(SE_SELECT);
                if showContest == FALSE {
                    PutWindowTilemap(RELEARNERWIN_DESC_CONTEST);
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_CONTEST_MODE;
                    sMoveRelearnerMenuState.showContestInfo = TRUE;
                } else {
                    PutWindowTilemap(RELEARNERWIN_DESC_BATTLE);
                    (*sMoveRelearnerStruct).state = MENU_STATE_SETUP_BATTLE_MODE;
                    sMoveRelearnerMenuState.showContestInfo = FALSE;
                }
                ScheduleBgCopyTilemapToVram(1);
                MoveRelearnerShowHideHearts(GetCurrentSelectedMove());
            }
            LIST_CANCEL => {
                PlaySE(SE_SELECT);
                RemoveScrollArrows();
                (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_GIVE_UP_PROMPT;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_MoveRelearnerGiveUp)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                MoveRelearnerPrintMessage(gStringVar4.as_mut_ptr());
            }
            _ => {
                PlaySE(SE_SELECT);
                RemoveScrollArrows();
                (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_TEACH_MOVE_PROMPT;
                StringCopy(
                    gStringVar2.as_mut_ptr(),
                    (*(&raw const crate::data::data_tables::gMoveNames)
                        .cast::<CArray<CArray<u8, 13>, 355>>())[itemId]
                        .as_ptr()
                        .cast_mut(),
                );
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_MoveRelearnerTeachMoveConfirm)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                MoveRelearnerPrintMessage(gStringVar4.as_mut_ptr());
            }
        }
    }
}
unsafe fn GetCurrentSelectedMove() -> i32 {
    (*sMoveRelearnerStruct).menuItems
        [sMoveRelearnerMenuState.listRow as i32 + sMoveRelearnerMenuState.listOffset as i32]
        .id
}
unsafe fn ShowTeachMoveText(shouldDoNothingInstead: u8) {
    if shouldDoNothingInstead == FALSE {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_TeachWhichMoveToPkmn)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
        FillWindowPixelBuffer(RELEARNERWIN_MSG, 0x11);
        AddTextPrinterParameterized(
            RELEARNERWIN_MSG,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            0,
            1,
            0,
            None,
        );
    }
}
unsafe fn CreateUISprites() {
    (*sMoveRelearnerStruct).moveDisplayArrowTask = TASK_NONE;
    (*sMoveRelearnerStruct).moveListScrollArrowTask = TASK_NONE;
    AddScrollArrows();
    for i in 0..8i32 {
        (*sMoveRelearnerStruct).heartSpriteIds[i] = CreateSprite(
            (&raw const *sConstestMoveHeartSprite).cast_mut(),
            (i as i16 - (i / 4) as i16 * 4) * 8 + 104,
            (i / 4) as i16 * 8 + 36,
            0,
        );
    }
    let mut i: i32 = 0;
    while i < 8 {
        (*sMoveRelearnerStruct).heartSpriteIds[i + 8] = CreateSprite(
            (&raw const *sConstestMoveHeartSprite).cast_mut(),
            (i as i16 - (i / 4) as i16 * 4) * 8 + 104,
            (i / 4) as i16 * 8 + 52,
            0,
        );
        StartSpriteAnim(
            &raw mut gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i + 8]],
            2,
        );
        i += 1;
    }
    for i in 0..16i32 {
        gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i]].set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe fn AddScrollArrows() {
    if (*sMoveRelearnerStruct).moveDisplayArrowTask == TASK_NONE {
        (*sMoveRelearnerStruct).moveDisplayArrowTask = AddScrollIndicatorArrowPair(
            (&raw const *sDisplayModeArrowsTemplate).cast_mut(),
            &raw mut (*sMoveRelearnerStruct).scrollOffset,
        );
    }
    if (*sMoveRelearnerStruct).moveListScrollArrowTask == TASK_NONE {
        gTempScrollArrowTemplate = *sMoveListScrollArrowsTemplate;
        gTempScrollArrowTemplate.fullyDownThreshold = (*sMoveRelearnerStruct).numMenuChoices as u16
            - (*sMoveRelearnerStruct).numToShowAtOnce as u16;
        (*sMoveRelearnerStruct).moveListScrollArrowTask = AddScrollIndicatorArrowPair(
            &raw mut gTempScrollArrowTemplate,
            &raw mut sMoveRelearnerMenuState.listOffset,
        );
    }
}
unsafe fn RemoveScrollArrows() {
    if (*sMoveRelearnerStruct).moveDisplayArrowTask != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*sMoveRelearnerStruct).moveDisplayArrowTask);
        (*sMoveRelearnerStruct).moveDisplayArrowTask = TASK_NONE;
    }
    if (*sMoveRelearnerStruct).moveListScrollArrowTask != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*sMoveRelearnerStruct).moveListScrollArrowTask);
        (*sMoveRelearnerStruct).moveListScrollArrowTask = TASK_NONE;
    }
}
unsafe fn CreateLearnableMovesList() {
    let mut nickname: CArray<u8, 11> = zeroed();
    (*sMoveRelearnerStruct).numMenuChoices = GetMoveRelearnerMoves(
        &raw mut gPlayerParty[(*sMoveRelearnerStruct).partyMon],
        (*sMoveRelearnerStruct).movesToLearn.as_mut_ptr(),
    );
    let mut i: i32 = 0;
    while i < (*sMoveRelearnerStruct).numMenuChoices as i32 {
        (*sMoveRelearnerStruct).menuItems[i].name =
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[(*sMoveRelearnerStruct).movesToLearn[i]]
                .as_ptr()
                .cast_mut();
        (*sMoveRelearnerStruct).menuItems[i].id = (*sMoveRelearnerStruct).movesToLearn[i] as i32;
        i += 1;
    }
    GetMonData3(
        &raw mut gPlayerParty[(*sMoveRelearnerStruct).partyMon],
        MON_DATA_NICKNAME,
        nickname.as_mut_ptr(),
    );
    StringCopy_Nickname(gStringVar1.as_mut_ptr(), nickname.as_mut_ptr());
    (*sMoveRelearnerStruct).menuItems[(*sMoveRelearnerStruct).numMenuChoices].name =
        (*(&raw const crate::data::strings::gText_Cancel).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    (*sMoveRelearnerStruct).menuItems[(*sMoveRelearnerStruct).numMenuChoices].id = LIST_CANCEL;
    (*sMoveRelearnerStruct).numMenuChoices += 1;
    (*sMoveRelearnerStruct).numToShowAtOnce = LoadMoveRelearnerMovesList(
        (*sMoveRelearnerStruct).menuItems.as_mut_ptr(),
        (*sMoveRelearnerStruct).numMenuChoices as u16,
    );
}
pub unsafe fn MoveRelearnerShowHideHearts(r#move: i32) {
    let mut numHearts: u16 = 0;
    if sMoveRelearnerMenuState.showContestInfo == 0 || r#move == LIST_CANCEL {
        for i in 0..16u16 {
            gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i]].set_invisible(TRUE as u16);
        }
    } else {
        numHearts = ((*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .appeal as i32
            / 10) as u8 as u16;
        if numHearts == 0xFF {
            numHearts = 0;
        }
        for i in 0..8u16 {
            if i < numHearts {
                StartSpriteAnim(
                    &raw mut gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i]],
                    1,
                );
            } else {
                StartSpriteAnim(
                    &raw mut gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i]],
                    0,
                );
            }
            gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i]].set_invisible(FALSE as u16);
        }
        numHearts = ((*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .jam as i32
            / 10) as u8 as u16;
        if numHearts == 0xFF {
            numHearts = 0;
        }
        for i in 0..8u16 {
            if i < numHearts {
                StartSpriteAnim(
                    &raw mut gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i as i32 + 8]],
                    3,
                );
            } else {
                StartSpriteAnim(
                    &raw mut gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i as i32 + 8]],
                    2,
                );
            }
            gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i as i32 + 8]]
                .set_invisible(FALSE as u16);
        }
    }
}
pub(crate) unsafe fn SetBackdropFromColor(color: u16) {
    FillPalette(color, 0, 2);
}
