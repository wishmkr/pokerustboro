//! Translated from `src/move_relearner.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static gContestEffects: CArray<ContestEffect, 0>;
    static gContestMoves: CArray<ContestMove, 0>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gMain: Main;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static mut gMultiuseListMenuTemplate: ListMenuTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTempScrollArrowTemplate: ScrollArrowsTemplate;
    static gText_Cancel: CArray<u8, 0>;
    static gText_MoveRelearnerAndPoof: CArray<u8, 0>;
    static gText_MoveRelearnerGiveUp: CArray<u8, 0>;
    static gText_MoveRelearnerPkmnForgotMoveAndLearnedNew: CArray<u8, 0>;
    static gText_MoveRelearnerPkmnLearnedMove: CArray<u8, 0>;
    static gText_MoveRelearnerPkmnTryingToLearnMove: CArray<u8, 0>;
    static gText_MoveRelearnerStopTryingToTeachMove: CArray<u8, 0>;
    static gText_MoveRelearnerTeachMoveConfirm: CArray<u8, 0>;
    static gText_MoveRelearnerWhichMoveToForget: CArray<u8, 0>;
    static gText_TeachWhichMoveToPkmn: CArray<u8, 0>;
    fn AddScrollIndicatorArrowPair(a0: *mut ScrollArrowsTemplate, a1: *mut u16) -> u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn ClearScheduledBgCopiesToVram();
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroyTask(a0: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetLRKeysPressed() -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMoveRelearnerMoves(a0: *mut Pokemon, a1: *mut u16) -> u8;
    fn GiveMoveToMon(a0: *mut Pokemon, a1: u16) -> u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitMoveRelearnerWindows(a0: u8);
    fn IsFanfareTaskInactive() -> u8;
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut ListMenuTemplate, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadMoveRelearnerMovesList(a0: *mut ListMenuItem, a1: u16) -> u8;
    fn LoadOam();
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LockPlayerFieldControls();
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MoveRelearnerCreateYesNoMenu();
    fn MoveRelearnerPrintMessage(a0: *mut u8);
    fn MoveRelearnerRunTextPrinters() -> u16;
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveMonPPBonus(a0: *mut Pokemon, a1: u8);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonMoveSlot(a0: *mut Pokemon, a1: u16, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowSelectMovePokemonSummaryScreen(
        a0: *mut Pokemon,
        a1: u8,
        a2: u8,
        a3: Option<unsafe extern "C" fn()>,
        a4: u16,
    );
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn VBlankCB_MoveRelearner() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TeachMoveRelearnerMove() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_WaitForFadeOut), 10);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
}
pub(crate) unsafe extern "C" fn Task_WaitForFadeOut(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetMainCallback2(Some(CB2_InitLearnMove));
        gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_InitLearnMove() {
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
pub(crate) unsafe extern "C" fn CB2_InitLearnMoveReturnFromSelectMove() {
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
pub(crate) unsafe extern "C" fn InitMoveRelearnerBackgroundLayers() {
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
pub(crate) unsafe extern "C" fn CB2_MoveRelearnerMain() {
    DoMoveRelearnerMain();
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn PrintMessageWithPlaceholders(src: *mut u8) {
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), src);
    MoveRelearnerPrintMessage(gStringVar4.as_mut_ptr());
}
pub(crate) unsafe extern "C" fn DoMoveRelearnerMain() {
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
            let mut selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
            if selection == 0 {
                if GiveMoveToMon(
                    &raw mut gPlayerParty[(*sMoveRelearnerStruct).partyMon],
                    GetCurrentSelectedMove() as u16,
                ) != MON_HAS_MAX_MOVES
                {
                    PrintMessageWithPlaceholders(
                        gText_MoveRelearnerPkmnLearnedMove.as_ptr().cast_mut(),
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
            let mut selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
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
                gText_MoveRelearnerPkmnTryingToLearnMove.as_ptr().cast_mut(),
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
            let mut selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
            if selection == 0 {
                PrintMessageWithPlaceholders(
                    gText_MoveRelearnerWhichMoveToForget.as_ptr().cast_mut(),
                );
                (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_WHICH_MOVE_PROMPT;
            } else if selection == MENU_B_PRESSED || selection == 1 {
                (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_STOP_TEACHING;
            }
        }
        MENU_STATE_PRINT_STOP_TEACHING => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[GetCurrentSelectedMove()].as_ptr().cast_mut(),
            );
            PrintMessageWithPlaceholders(
                gText_MoveRelearnerStopTryingToTeachMove.as_ptr().cast_mut(),
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
            let mut selection: i8 = Menu_ProcessInputNoWrapClearOnChoose();
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
                    let mut r#move: u16 = GetMonData2(
                        &raw mut gPlayerParty[(*sMoveRelearnerStruct).partyMon],
                        MON_DATA_MOVE1 + (*sMoveRelearnerStruct).moveSlot as i32,
                    ) as u16;
                    StringCopy(
                        gStringVar3.as_mut_ptr(),
                        gMoveNames[r#move].as_ptr().cast_mut(),
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
                        gMoveNames[GetCurrentSelectedMove()].as_ptr().cast_mut(),
                    );
                    PrintMessageWithPlaceholders(gText_MoveRelearnerAndPoof.as_ptr().cast_mut());
                    (*sMoveRelearnerStruct).state = MENU_STATE_DOUBLE_FANFARE_FORGOT_MOVE;
                    gSpecialVar_0x8004 = TRUE as u16;
                }
            }
        }
        MENU_STATE_DOUBLE_FANFARE_FORGOT_MOVE => {
            if MoveRelearnerRunTextPrinters() == 0 {
                PrintMessageWithPlaceholders(
                    gText_MoveRelearnerPkmnForgotMoveAndLearnedNew
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
        MENU_STATE_WAIT_FOR_A_BUTTON => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                (*sMoveRelearnerStruct).state = MENU_STATE_FADE_AND_RETURN;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn FreeMoveRelearnerResources() {
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
pub(crate) unsafe extern "C" fn HideHeartSpritesAndShowTeachMoveText(onlyHideSprites: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < 16 {
        gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i]].set_invisible(TRUE as u16);
        i += 1;
    }
    if onlyHideSprites == 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_TeachWhichMoveToPkmn.as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn HandleInput(showContest: u8) {
    let mut itemId: i32 = ListMenu_ProcessInput((*sMoveRelearnerStruct).moveListMenuTask);
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
                    gText_MoveRelearnerGiveUp.as_ptr().cast_mut(),
                );
                MoveRelearnerPrintMessage(gStringVar4.as_mut_ptr());
            }
            _ => {
                PlaySE(SE_SELECT);
                RemoveScrollArrows();
                (*sMoveRelearnerStruct).state = MENU_STATE_PRINT_TEACH_MOVE_PROMPT;
                StringCopy(
                    gStringVar2.as_mut_ptr(),
                    gMoveNames[itemId].as_ptr().cast_mut(),
                );
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_MoveRelearnerTeachMoveConfirm.as_ptr().cast_mut(),
                );
                MoveRelearnerPrintMessage(gStringVar4.as_mut_ptr());
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetCurrentSelectedMove() -> i32 {
    return (*sMoveRelearnerStruct).menuItems
        [sMoveRelearnerMenuState.listRow as i32 + sMoveRelearnerMenuState.listOffset as i32]
        .id;
}
pub(crate) unsafe extern "C" fn ShowTeachMoveText(shouldDoNothingInstead: u8) {
    if shouldDoNothingInstead == FALSE {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_TeachWhichMoveToPkmn.as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn CreateUISprites() {
    let mut i: i32 = 0;
    (*sMoveRelearnerStruct).moveDisplayArrowTask = TASK_NONE;
    (*sMoveRelearnerStruct).moveListScrollArrowTask = TASK_NONE;
    AddScrollArrows();
    i = 0;
    while i < 8 {
        (*sMoveRelearnerStruct).heartSpriteIds[i] = CreateSprite(
            (&raw const *sConstestMoveHeartSprite).cast_mut(),
            (i as i16 - (i / 4) as i16 * 4) * 8 + 104,
            (i / 4) as i16 * 8 + 36,
            0,
        );
        i += 1;
    }
    i = 0;
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
    i = 0;
    while i < 16 {
        gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i]].set_invisible(TRUE as u16);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn AddScrollArrows() {
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
pub(crate) unsafe extern "C" fn RemoveScrollArrows() {
    if (*sMoveRelearnerStruct).moveDisplayArrowTask != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*sMoveRelearnerStruct).moveDisplayArrowTask);
        (*sMoveRelearnerStruct).moveDisplayArrowTask = TASK_NONE;
    }
    if (*sMoveRelearnerStruct).moveListScrollArrowTask != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*sMoveRelearnerStruct).moveListScrollArrowTask);
        (*sMoveRelearnerStruct).moveListScrollArrowTask = TASK_NONE;
    }
}
pub(crate) unsafe extern "C" fn CreateLearnableMovesList() {
    let mut i: i32 = 0;
    let mut nickname: CArray<u8, 11> = zeroed();
    (*sMoveRelearnerStruct).numMenuChoices = GetMoveRelearnerMoves(
        &raw mut gPlayerParty[(*sMoveRelearnerStruct).partyMon],
        (*sMoveRelearnerStruct).movesToLearn.as_mut_ptr(),
    );
    i = 0;
    while i < (*sMoveRelearnerStruct).numMenuChoices as i32 {
        (*sMoveRelearnerStruct).menuItems[i].name = gMoveNames
            [(*sMoveRelearnerStruct).movesToLearn[i]]
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
        gText_Cancel.as_ptr().cast_mut();
    (*sMoveRelearnerStruct).menuItems[(*sMoveRelearnerStruct).numMenuChoices].id = LIST_CANCEL;
    (*sMoveRelearnerStruct).numMenuChoices += 1;
    (*sMoveRelearnerStruct).numToShowAtOnce = LoadMoveRelearnerMovesList(
        (*sMoveRelearnerStruct).menuItems.as_mut_ptr(),
        (*sMoveRelearnerStruct).numMenuChoices as u16,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveRelearnerShowHideHearts(r#move: i32) {
    let mut numHearts: u16 = 0;
    let mut i: u16 = 0;
    if sMoveRelearnerMenuState.showContestInfo == 0 || r#move == LIST_CANCEL {
        i = 0;
        while i < 16 {
            gSprites[(*sMoveRelearnerStruct).heartSpriteIds[i]].set_invisible(TRUE as u16);
            i += 1;
        }
    } else {
        numHearts = (gContestEffects[gContestMoves[r#move].effect].appeal as i32 / 10) as u8 as u16;
        if numHearts == 0xFF {
            numHearts = 0;
        }
        i = 0;
        while i < 8 {
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
            i += 1;
        }
        numHearts = (gContestEffects[gContestMoves[r#move].effect].jam as i32 / 10) as u8 as u16;
        if numHearts == 0xFF {
            numHearts = 0;
        }
        i = 0;
        while i < 8 {
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
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn SetBackdropFromColor(color: u16) {
    FillPalette(color, 0, 2);
}
