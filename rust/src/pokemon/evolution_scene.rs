//! Translated from `src/evolution_scene.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sUnusedPal1 sBgAnim_Gfx sBgAnim_Inner_Tilemap sBgAnim_Outer_Tilemap sBgAnim_Intro_Pal sUnusedPal2 sUnusedPal3 sUnusedPal4 sBgAnim_Pal sText_ShedinjaJapaneseName sBgAnim_PaletteControl sBgAnim_PalIndexes

/// `struct EvoInfo`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct EvoInfo {
    pub preEvoSpriteId: u8,
    pub postEvoSpriteId: u8,
    pub evoTaskId: u8,
    pub delayTimer: u8,
    pub savedPalette: CArray<u16, 48>,
}

unsafe impl Sync for EvoInfo {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<EvoInfo>() == 100);
    assert!(offset_of!(EvoInfo, preEvoSpriteId) == 0);
    assert!(offset_of!(EvoInfo, postEvoSpriteId) == 1);
    assert!(offset_of!(EvoInfo, evoTaskId) == 2);
    assert!(offset_of!(EvoInfo, delayTimer) == 3);
    assert!(offset_of!(EvoInfo, savedPalette) == 4);
};

const EVOSTATE_CANCEL: i16 = 17;
const EVOSTATE_CANCEL_MON_ANIM: i16 = 18;
const EVOSTATE_CANCEL_MSG: i16 = 19;
const EVOSTATE_CYCLE_MON_SPRITE: i16 = 7;
const EVOSTATE_END: i16 = 16;
const EVOSTATE_EVO_MON_ANIM: i16 = 13;
const EVOSTATE_EVO_SOUND: i16 = 11;
const EVOSTATE_FADE_IN: i16 = 0;
const EVOSTATE_INTRO_MON_ANIM: i16 = 2;
const EVOSTATE_INTRO_MSG: i16 = 1;
const EVOSTATE_INTRO_SOUND: i16 = 3;
const EVOSTATE_LEARNED_MOVE: i16 = 20;
const EVOSTATE_REPLACE_MOVE: i16 = 22;
const EVOSTATE_RESTORE_SCREEN: i16 = 12;
const EVOSTATE_SET_MON_EVOLVED: i16 = 14;
const EVOSTATE_SPARKLE_ARC: i16 = 6;
const EVOSTATE_SPARKLE_CIRCLE: i16 = 9;
const EVOSTATE_SPARKLE_SPRAY: i16 = 10;
const EVOSTATE_START_BG_AND_SPARKLE_SPIRAL: i16 = 5;
const EVOSTATE_START_MUSIC: i16 = 4;
const EVOSTATE_TRY_LEARN_ANOTHER_MOVE: i16 = 21;
const EVOSTATE_TRY_LEARN_MOVE: i16 = 15;
const EVOSTATE_WAIT_CYCLE_MON_SPRITE: i16 = 8;
const MVSTATE_ASK_CANCEL: i16 = 10;
const MVSTATE_CANCEL: i16 = 11;
const MVSTATE_FORGET_MSG_1: i16 = 7;
const MVSTATE_FORGET_MSG_2: i16 = 8;
const MVSTATE_HANDLE_MOVE_SELECT: i16 = 6;
const MVSTATE_HANDLE_YES_NO: i16 = 4;
const MVSTATE_INTRO_MSG_1: i16 = 0;
const MVSTATE_INTRO_MSG_2: i16 = 1;
const MVSTATE_INTRO_MSG_3: i16 = 2;
const MVSTATE_LEARNED_MOVE: i16 = 9;
const MVSTATE_PRINT_YES_NO: i16 = 3;
const MVSTATE_RETRY_AFTER_HM: i16 = 12;
const MVSTATE_SHOW_MOVE_SELECT: i16 = 5;
const TASK_BIT_CAN_STOP: i32 = 1;
const TASK_BIT_LEARN_MOVE: i32 = 128;
const T_EVOSTATE_CANCEL: i16 = 15;
const T_EVOSTATE_CANCEL_MON_ANIM: i16 = 16;
const T_EVOSTATE_CANCEL_MSG: i16 = 17;
const T_EVOSTATE_CYCLE_MON_SPRITE: i16 = 6;
const T_EVOSTATE_END: i16 = 14;
const T_EVOSTATE_EVO_MON_ANIM: i16 = 11;
const T_EVOSTATE_EVO_SOUND: i16 = 10;
const T_EVOSTATE_INTRO_CRY: i16 = 1;
const T_EVOSTATE_INTRO_MSG: i16 = 0;
const T_EVOSTATE_INTRO_SOUND: i16 = 2;
const T_EVOSTATE_LEARNED_MOVE: i16 = 18;
const T_EVOSTATE_REPLACE_MOVE: i16 = 20;
const T_EVOSTATE_SET_MON_EVOLVED: i16 = 12;
const T_EVOSTATE_SPARKLE_ARC: i16 = 5;
const T_EVOSTATE_SPARKLE_CIRCLE: i16 = 8;
const T_EVOSTATE_SPARKLE_SPRAY: i16 = 9;
const T_EVOSTATE_START_BG_AND_SPARKLE_SPIRAL: i16 = 4;
const T_EVOSTATE_START_MUSIC: i16 = 3;
const T_EVOSTATE_TRY_LEARN_ANOTHER_MOVE: i16 = 19;
const T_EVOSTATE_TRY_LEARN_MOVE: i16 = 13;
const T_EVOSTATE_WAIT_CYCLE_MON_SPRITE: i16 = 7;
const T_MVSTATE_ASK_CANCEL: i16 = 9;
const T_MVSTATE_CANCEL: i16 = 10;
const T_MVSTATE_FORGET_MSG: i16 = 7;
const T_MVSTATE_HANDLE_MOVE_SELECT: i16 = 6;
const T_MVSTATE_HANDLE_YES_NO: i16 = 4;
const T_MVSTATE_INTRO_MSG_1: i16 = 0;
const T_MVSTATE_INTRO_MSG_2: i16 = 1;
const T_MVSTATE_INTRO_MSG_3: i16 = 2;
const T_MVSTATE_LEARNED_MOVE: i16 = 8;
const T_MVSTATE_PRINT_YES_NO: i16 = 3;
const T_MVSTATE_RETRY_AFTER_HM: i16 = 11;
const T_MVSTATE_SHOW_MOVE_SELECT: i16 = 5;

static sBgAnim_Gfx: Table<CArray<u32, 446>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Gfx).cast());
static sBgAnim_Inner_Tilemap: Table<CArray<u32, 313>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Inner_Tilemap).cast());
static sBgAnim_Intro_Pal: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Intro_Pal).cast());
static sBgAnim_Outer_Tilemap: Table<CArray<u32, 309>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Outer_Tilemap).cast());
static sBgAnim_Pal: Table<CArray<u16, 32>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_Pal).cast());
static sBgAnim_PalIndexes: Table<CArray<CArray<u8, 16>, 50>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_PalIndexes).cast());
static sBgAnim_PaletteControl: Table<CArray<CArray<u8, 4>, 4>> =
    Table((&raw const crate::data::evolution_scene::sBgAnim_PaletteControl).cast());
static sText_ShedinjaJapaneseName: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::evolution_scene::sText_ShedinjaJapaneseName).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sEvoStructPtr: *mut EvoInfo = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBgAnimPal: *mut u16 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCB2_AfterEvolution: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" {
    static mut gAffineAnimsDisabled: u8;
    static mut gBattleCommunication: CArray<u8, 8>;
    static mut gBattleEnvironment: u8;
    static gBattleStringsTable: CArray<*mut u8, 0>;
    static mut gBattleTextBuff1: CArray<u8, 16>;
    static mut gBattleTextBuff2: CArray<u8, 16>;
    static mut gBattle_BG0_X: u16;
    static mut gBattle_BG0_Y: u16;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattle_BG2_Y: u16;
    static mut gBattle_BG3_X: u16;
    static mut gBattle_BG3_Y: u16;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static gDummySpriteAffineAnimTable: CArray<*mut AffineAnimCmd, 0>;
    static mut gEvolutionTable: CArray<CArray<Evolution, 5>, 0>;
    static mut gMain: Main;
    static gMonFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static mut gMoveToLearn: u16;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gReservedSpritePaletteCount: u8;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    static gText_BattleYesNoChoice: CArray<u8, 0>;
    static gText_CommunicationStandby5: CArray<u8, 0>;
    static gText_CongratsPkmnEvolved: CArray<u8, 0>;
    static gText_EllipsisQuestionMark: CArray<u8, 0>;
    static gText_PkmnIsEvolving: CArray<u8, 0>;
    static gText_PkmnStoppedEvolving: CArray<u8, 0>;
    static gTradeEvolutionSceneYesNoWindowTemplate: WindowTemplate;
    static mut gWirelessCommType: u8;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AllocateMonSpritesGfx();
    fn AnimateSprites();
    fn BattleCreateYesNoCursorAt(a0: u8);
    fn BattleDestroyYesNoCursorAt(a0: u8);
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BufferMoveToLearnIntoBattleTextBuff2();
    fn BuildOamBuffer();
    fn CalculateMonStats(a0: *mut Pokemon);
    fn CalculatePlayerPartyCount() -> u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyMon(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn CycleEvolutionMonSprite(a0: u8, a1: u8) -> u8;
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DecompressPicFromTable_2(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32);
    fn DestroyTask(a0: u8);
    fn DestroyWirelessStatusIndicatorSprite();
    fn DoMonFrontSpriteAnimation(a0: *mut Sprite, a1: u16, a2: u8, a3: u8);
    fn DrawTextOnTradeWindow(a0: u8, a1: *mut u8, a2: u8);
    fn EvolutionRenameMon(a0: *mut Pokemon, a1: u16, a2: u16);
    fn EvolutionSparkles_ArcDown() -> u8;
    fn EvolutionSparkles_CircleInward() -> u8;
    fn EvolutionSparkles_SpiralUpward(a0: u16) -> u8;
    fn EvolutionSparkles_SprayAndFlash(a0: u16) -> u8;
    fn EvolutionSparkles_SprayAndFlash_Trade(a0: u16) -> u8;
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonSpritesGfx();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBattleBgTemplateData(a0: u8, a1: u8) -> u32;
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonSpritePalStructFromOtIdPersonality(
        a0: u16,
        a1: u32,
        a2: u32,
    ) -> *mut CompressedSpritePalette;
    fn GetMoveSlotToReplace() -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn HandleBattleWindow(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn IncrementGameStat(a0: u8);
    fn InitBattleBgsVideo();
    fn InitTradeSequenceBgGpuRegs();
    fn IsCryFinished() -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsHMMove2(a0: u16) -> u32;
    fn IsSEPlaying() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LinkTradeDrawWindow();
    fn LoadBattleTextboxAndBackground();
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadEvoSparkleSpriteAndPal();
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadTradeAnimGfx();
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MonTryLearningNewMove(a0: *mut Pokemon, a1: u8) -> u16;
    fn Overworld_PlaySpecialMapMusic();
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayFanfare(a0: u16);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn RemoveMonPPBonus(a0: *mut Pokemon, a1: u8);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_Stop();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMonMoveSlot(a0: *mut Pokemon, a1: u16, a2: u8);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowSelectMovePokemonSummaryScreen(
        a0: *mut Pokemon,
        a1: u8,
        a2: u8,
        a3: Option<unsafe extern "C" fn()>,
        a4: u16,
    );
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn SpriteCallbackDummy_2(a0: *mut Sprite);
    fn StopMapMusic();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayAllStop();
    fn m4aSongNumStop(a0: u16);
}

pub(crate) unsafe extern "C" fn CB2_BeginEvolutionScene() {
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe extern "C" fn Task_BeginEvolutionScene(taskId: u8) {
    let mut mon: *mut Pokemon = null_mut();
    match gTasks[taskId].data[0] {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                let mut postEvoSpecies: u16 = 0;
                let mut canStopEvo: u8 = 0;
                let mut partyId: u8 = 0;
                mon = &raw mut gPlayerParty[gTasks[taskId].data[10]];
                postEvoSpecies = gTasks[taskId].data[2] as u16;
                canStopEvo = gTasks[taskId].data[3] as u8;
                partyId = gTasks[taskId].data[10] as u8;
                DestroyTask(taskId);
                EvolutionScene(mon, postEvoSpecies, canStopEvo, partyId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BeginEvolutionScene(
    mon: *mut Pokemon,
    postEvoSpecies: u16,
    canStopEvo: u8,
    partyId: u8,
) {
    let mut taskId: u8 = CreateTask(Some(Task_BeginEvolutionScene), 0);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[2] = postEvoSpecies as i16;
    gTasks[taskId].data[3] = canStopEvo as i16;
    gTasks[taskId].data[10] = partyId as i16;
    SetMainCallback2(Some(CB2_BeginEvolutionScene));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionScene(
    mon: *mut Pokemon,
    postEvoSpecies: u16,
    canStopEvo: u8,
    partyId: u8,
) {
    let mut name: CArray<u8, 20> = zeroed();
    let mut currSpecies: u16 = 0;
    let mut trainerId: u32 = 0;
    let mut personality: u32 = 0;
    let mut pokePal: *mut CompressedSpritePalette = null_mut();
    let mut id: u8 = 0;
    SetHBlankCallback(None);
    SetVBlankCallback(None);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                VRAM as usize as *mut c_void,
                0x5006000,
            );
        }
    }
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WIN1H, 0);
    SetGpuReg(REG_OFFSET_WIN1V, 0);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    ResetPaletteFade();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 256;
    gBattle_BG3_Y = 0;
    gBattleEnvironment = BATTLE_ENVIRONMENT_PLAIN;
    InitBattleBgsVideo();
    LoadBattleTextboxAndBackground();
    ResetSpriteData();
    ScanlineEffect_Stop();
    ResetTasks();
    FreeAllSpritePalettes();
    gReservedSpritePaletteCount = 4;
    sEvoStructPtr = AllocZeroed(100) as *mut EvoInfo;
    AllocateMonSpritesGfx();
    GetMonData3(mon, MON_DATA_NICKNAME, name.as_mut_ptr());
    StringCopy_Nickname(gStringVar1.as_mut_ptr(), name.as_mut_ptr());
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gSpeciesNames[postEvoSpecies].as_ptr().cast_mut(),
    );
    currSpecies = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    trainerId = GetMonData2(mon, MON_DATA_OT_ID);
    personality = GetMonData2(mon, MON_DATA_PERSONALITY);
    DecompressPicFromTable_2(
        (&raw const gMonFrontPicTable[currSpecies]).cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[1],
        currSpecies as i32,
    );
    pokePal = GetMonSpritePalStructFromOtIdPersonality(currSpecies, trainerId, personality);
    LoadCompressedPalette((*pokePal).data, 272, 32);
    SetMultiuseSpriteTemplateToPokemon(currSpecies, B_POSITION_OPPONENT_LEFT);
    gMultiuseSpriteTemplate.affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
    (*sEvoStructPtr).preEvoSpriteId = {
        id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
        id
    };
    gSprites[id].callback = Some(SpriteCallbackDummy_2);
    gSprites[id].oam.set_paletteNum(1);
    gSprites[id].set_invisible(TRUE as u16);
    DecompressPicFromTable_2(
        (&raw const gMonFrontPicTable[postEvoSpecies]).cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[3],
        postEvoSpecies as i32,
    );
    pokePal = GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
    LoadCompressedPalette((*pokePal).data, 288, 32);
    SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, B_POSITION_OPPONENT_RIGHT);
    gMultiuseSpriteTemplate.affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
    (*sEvoStructPtr).postEvoSpriteId = {
        id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
        id
    };
    gSprites[id].callback = Some(SpriteCallbackDummy_2);
    gSprites[id].oam.set_paletteNum(2);
    gSprites[id].set_invisible(TRUE as u16);
    LoadEvoSparkleSpriteAndPal();
    (*sEvoStructPtr).evoTaskId = {
        id = CreateTask(Some(Task_EvolutionScene), 0);
        id
    };
    gTasks[id].data[0] = 0;
    gTasks[id].data[1] = currSpecies as i16;
    gTasks[id].data[2] = postEvoSpecies as i16;
    gTasks[id].data[3] = canStopEvo as i16;
    gTasks[id].data[4] = TRUE as i16;
    gTasks[id].data[9] = FALSE as i16;
    gTasks[id].data[10] = partyId as i16;
    memcpy(
        &raw mut (*sEvoStructPtr).savedPalette as *mut u8,
        &raw mut gPlttBufferUnfaded[32] as *mut u8,
        96,
    );
    SetGpuReg(REG_OFFSET_DISPCNT, 8000);
    SetHBlankCallback(Some(EvoDummyFunc));
    SetVBlankCallback(Some(VBlankCB_EvolutionScene));
    m4aMPlayAllStop();
    SetMainCallback2(Some(CB2_EvolutionSceneUpdate));
}
pub(crate) unsafe extern "C" fn CB2_EvolutionSceneLoadGraphics() {
    let mut id: u8 = 0;
    let mut pokePal: *mut CompressedSpritePalette = null_mut();
    let mut postEvoSpecies: u16 = 0;
    let mut trainerId: u32 = 0;
    let mut personality: u32 = 0;
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gTasks[(*sEvoStructPtr).evoTaskId].data[10]];
    postEvoSpecies = gTasks[(*sEvoStructPtr).evoTaskId].data[2] as u16;
    trainerId = GetMonData2(mon, MON_DATA_OT_ID);
    personality = GetMonData2(mon, MON_DATA_PERSONALITY);
    SetHBlankCallback(None);
    SetVBlankCallback(None);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                VRAM as usize as *mut c_void,
                0x5006000,
            );
        }
    }
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WIN1H, 0);
    SetGpuReg(REG_OFFSET_WIN1V, 0);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    ResetPaletteFade();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 256;
    gBattle_BG3_Y = 0;
    gBattleEnvironment = BATTLE_ENVIRONMENT_PLAIN;
    InitBattleBgsVideo();
    LoadBattleTextboxAndBackground();
    ResetSpriteData();
    FreeAllSpritePalettes();
    gReservedSpritePaletteCount = 4;
    DecompressPicFromTable_2(
        (&raw const gMonFrontPicTable[postEvoSpecies]).cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[3],
        postEvoSpecies as i32,
    );
    pokePal = GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
    LoadCompressedPalette((*pokePal).data, 288, 32);
    SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, B_POSITION_OPPONENT_RIGHT);
    gMultiuseSpriteTemplate.affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
    (*sEvoStructPtr).postEvoSpriteId = {
        id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
        id
    };
    gSprites[id].callback = Some(SpriteCallbackDummy_2);
    gSprites[id].oam.set_paletteNum(2);
    SetGpuReg(REG_OFFSET_DISPCNT, 8000);
    SetHBlankCallback(Some(EvoDummyFunc));
    SetVBlankCallback(Some(VBlankCB_EvolutionScene));
    SetMainCallback2(Some(CB2_EvolutionSceneUpdate));
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
}
pub(crate) unsafe extern "C" fn CB2_TradeEvolutionSceneLoadGraphics() {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gTasks[(*sEvoStructPtr).evoTaskId].data[10]];
    let mut postEvoSpecies: u16 = gTasks[(*sEvoStructPtr).evoTaskId].data[2] as u16;
    match gMain.state {
        0 => {
            SetGpuReg(0x0, 0);
            SetHBlankCallback(None);
            SetVBlankCallback(None);
            ResetSpriteData();
            FreeAllSpritePalettes();
            gReservedSpritePaletteCount = 4;
            gBattle_BG0_X = 0;
            gBattle_BG0_Y = 0;
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            gBattle_BG2_X = 0;
            gBattle_BG2_Y = 0;
            gBattle_BG3_X = 256;
            gBattle_BG3_Y = 0;
            gMain.state += 1;
        }
        1 => {
            ResetPaletteFade();
            SetHBlankCallback(Some(EvoDummyFunc));
            SetVBlankCallback(Some(VBlankCB_TradeEvolutionScene));
            gMain.state += 1;
        }
        2 => {
            LoadTradeAnimGfx();
            gMain.state += 1;
        }
        3 => {
            FillBgTilemapBufferRect(1, 0, 0, 0, 0x20, 0x20, 0x11);
            CopyBgTilemapBufferToVram(1);
            gMain.state += 1;
        }
        4 => {
            let mut pokePal: *mut CompressedSpritePalette = null_mut();
            let mut trainerId: u32 = GetMonData2(mon, MON_DATA_OT_ID);
            let mut personality: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
            DecompressPicFromTable_2(
                (&raw const gMonFrontPicTable[postEvoSpecies]).cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[3],
                postEvoSpecies as i32,
            );
            pokePal =
                GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
            LoadCompressedPalette((*pokePal).data, 288, 32);
            gMain.state += 1;
        }
        5 => {
            let mut id: u8 = 0;
            SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, B_POSITION_OPPONENT_LEFT);
            gMultiuseSpriteTemplate.affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
            (*sEvoStructPtr).postEvoSpriteId = {
                id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
                id
            };
            gSprites[id].callback = Some(SpriteCallbackDummy_2);
            gSprites[id].oam.set_paletteNum(2);
            gMain.state += 1;
            LinkTradeDrawWindow();
        }
        6 => {
            if gWirelessCommType != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
            }
            BlendPalettes(PALETTES_ALL, 0x10, 0);
            gMain.state += 1;
        }
        7 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            InitTradeSequenceBgGpuRegs();
            ShowBg(0);
            ShowBg(1);
            SetMainCallback2(Some(CB2_TradeEvolutionSceneUpdate));
            SetGpuReg(REG_OFFSET_DISPCNT, 4928);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TradeEvolutionScene(
    mon: *mut Pokemon,
    postEvoSpecies: u16,
    preEvoSpriteId: u8,
    partyId: u8,
) {
    let mut name: CArray<u8, 20> = zeroed();
    let mut currSpecies: u16 = 0;
    let mut trainerId: u32 = 0;
    let mut personality: u32 = 0;
    let mut pokePal: *mut CompressedSpritePalette = null_mut();
    let mut id: u8 = 0;
    GetMonData3(mon, MON_DATA_NICKNAME, name.as_mut_ptr());
    StringCopy_Nickname(gStringVar1.as_mut_ptr(), name.as_mut_ptr());
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gSpeciesNames[postEvoSpecies].as_ptr().cast_mut(),
    );
    gAffineAnimsDisabled = TRUE;
    currSpecies = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    personality = GetMonData2(mon, MON_DATA_PERSONALITY);
    trainerId = GetMonData2(mon, MON_DATA_OT_ID);
    sEvoStructPtr = AllocZeroed(100) as *mut EvoInfo;
    (*sEvoStructPtr).preEvoSpriteId = preEvoSpriteId;
    DecompressPicFromTable_2(
        (&raw const gMonFrontPicTable[postEvoSpecies]).cast_mut(),
        (*gMonSpritesGfxPtr).sprites.ptr[1],
        postEvoSpecies as i32,
    );
    pokePal = GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
    LoadCompressedPalette((*pokePal).data, 288, 32);
    SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, B_POSITION_OPPONENT_LEFT);
    gMultiuseSpriteTemplate.affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
    (*sEvoStructPtr).postEvoSpriteId = {
        id = CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 64, 30);
        id
    };
    gSprites[id].callback = Some(SpriteCallbackDummy_2);
    gSprites[id].oam.set_paletteNum(2);
    gSprites[id].set_invisible(TRUE as u16);
    LoadEvoSparkleSpriteAndPal();
    (*sEvoStructPtr).evoTaskId = {
        id = CreateTask(Some(Task_TradeEvolutionScene), 0);
        id
    };
    gTasks[id].data[0] = 0;
    gTasks[id].data[1] = currSpecies as i16;
    gTasks[id].data[2] = postEvoSpecies as i16;
    gTasks[id].data[4] = TRUE as i16;
    gTasks[id].data[9] = FALSE as i16;
    gTasks[id].data[10] = partyId as i16;
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 256;
    gBattle_BG3_Y = 0;
    gTextFlags.set_useAlternateDownArrow(TRUE);
    SetVBlankCallback(Some(VBlankCB_TradeEvolutionScene));
    SetMainCallback2(Some(CB2_TradeEvolutionSceneUpdate));
}
pub(crate) unsafe extern "C" fn CB2_EvolutionSceneUpdate() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe extern "C" fn CB2_TradeEvolutionSceneUpdate() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe extern "C" fn CreateShedinja(preEvoSpecies: u16, mon: *mut Pokemon) {
    let mut data: u32 = 0;
    if gEvolutionTable[preEvoSpecies][0].method == EVO_LEVEL_NINJASK
        && gPlayerPartyCount < PARTY_SIZE as u8
    {
        let mut i: i32 = 0;
        let mut shedinja: *mut Pokemon = &raw mut gPlayerParty[gPlayerPartyCount];
        CopyMon(
            &raw mut gPlayerParty[gPlayerPartyCount] as *mut c_void,
            mon as *mut c_void,
            100,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_SPECIES,
            &raw mut gEvolutionTable[preEvoSpecies][1].targetSpecies as *mut c_void,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_NICKNAME,
            gSpeciesNames[gEvolutionTable[preEvoSpecies][1].targetSpecies]
                .as_ptr()
                .cast_mut() as *mut c_void,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_HELD_ITEM,
            &raw mut data as *mut c_void,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_MARKINGS,
            &raw mut data as *mut c_void,
        );
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_ENCRYPT_SEPARATOR,
            &raw mut data as *mut c_void,
        );
        i = MON_DATA_COOL_RIBBON;
        while i < 55 {
            SetMonData(
                &raw mut gPlayerParty[gPlayerPartyCount],
                i,
                &raw mut data as *mut c_void,
            );
            i += 1;
        }
        i = MON_DATA_CHAMPION_RIBBON;
        while i <= MON_DATA_UNUSED_RIBBONS {
            SetMonData(
                &raw mut gPlayerParty[gPlayerPartyCount],
                i,
                &raw mut data as *mut c_void,
            );
            i += 1;
        }
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_STATUS,
            &raw mut data as *mut c_void,
        );
        data = MAIL_NONE;
        SetMonData(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_MAIL,
            &raw mut data as *mut c_void,
        );
        CalculateMonStats(&raw mut gPlayerParty[gPlayerPartyCount]);
        CalculatePlayerPartyCount();
        GetSetPokedexFlag(
            SpeciesToNationalPokedexNum(gEvolutionTable[preEvoSpecies][1].targetSpecies),
            FLAG_SET_SEEN,
        );
        GetSetPokedexFlag(
            SpeciesToNationalPokedexNum(gEvolutionTable[preEvoSpecies][1].targetSpecies),
            FLAG_SET_CAUGHT,
        );
        if GetMonData2(shedinja, MON_DATA_SPECIES) == SPECIES_SHEDINJA as u32
            && GetMonData2(shedinja, MON_DATA_LANGUAGE) == LANGUAGE_JAPANESE as u32
            && GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_NINJASK
        {
            SetMonData(
                shedinja,
                MON_DATA_NICKNAME,
                sText_ShedinjaJapaneseName.as_ptr().cast_mut() as *mut c_void,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_EvolutionScene(taskId: u8) {
    let mut var: u32 = 0;
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gTasks[taskId].data[10]];
    if gMain.heldKeys == B_BUTTON as u16
        && gTasks[taskId].data[0] == EVOSTATE_WAIT_CYCLE_MON_SPRITE
        && gTasks[gBattleCommunication[2]].isActive != 0
        && gTasks[taskId].data[3] as i32 & TASK_BIT_CAN_STOP != 0
    {
        gTasks[taskId].data[0] = EVOSTATE_CANCEL;
        gTasks[gBattleCommunication[2]].data[8] = TRUE as i16;
        StopBgAnimation();
        return;
    }
    'l1: {
        match gTasks[taskId].data[0] {
            EVOSTATE_FADE_IN => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
                gSprites[(*sEvoStructPtr).preEvoSpriteId].set_invisible(FALSE as u16);
                gTasks[taskId].data[0] += 1;
                ShowBg(0);
                ShowBg(1);
                ShowBg(2);
                ShowBg(3);
            }
            EVOSTATE_INTRO_MSG => {
                if gPaletteFade.active() == 0 {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        gText_PkmnIsEvolving.as_ptr().cast_mut(),
                    );
                    BattlePutTextOnWindow(gStringVar4.as_mut_ptr(), B_WIN_MSG);
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_INTRO_MON_ANIM => {
                if IsTextPrinterActive(0) == 0 {
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).preEvoSpriteId,
                        gTasks[taskId].data[1] as u16,
                    );
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_INTRO_SOUND => {
                if EvoScene_IsMonAnimFinished((*sEvoStructPtr).preEvoSpriteId) != 0 {
                    PlaySE(MUS_EVOLUTION_INTRO);
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_START_MUSIC => {
                if IsSEPlaying() == 0 {
                    PlayNewMapMusic(MUS_EVOLUTION);
                    gTasks[taskId].data[0] += 1;
                    BeginNormalPaletteFade(0x1C, 4, 0, 0x10, 0);
                }
            }
            EVOSTATE_START_BG_AND_SPARKLE_SPIRAL => {
                if gPaletteFade.active() == 0 {
                    StartBgAnimation(FALSE);
                    gBattleCommunication[2] = EvolutionSparkles_SpiralUpward(17);
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_SPARKLE_ARC => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    gTasks[taskId].data[0] += 1;
                    (*sEvoStructPtr).delayTimer = 1;
                    gBattleCommunication[2] = EvolutionSparkles_ArcDown();
                }
            }
            EVOSTATE_CYCLE_MON_SPRITE => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    gBattleCommunication[2] = CycleEvolutionMonSprite(
                        (*sEvoStructPtr).preEvoSpriteId,
                        (*sEvoStructPtr).postEvoSpriteId,
                    );
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_WAIT_CYCLE_MON_SPRITE => {
                if ({
                    (*sEvoStructPtr).delayTimer -= 1;
                    (*sEvoStructPtr).delayTimer
                }) == 0
                {
                    (*sEvoStructPtr).delayTimer = 3;
                    if gTasks[gBattleCommunication[2]].isActive == 0 {
                        gTasks[taskId].data[0] += 1;
                    }
                }
            }
            EVOSTATE_SPARKLE_CIRCLE => {
                gBattleCommunication[2] = EvolutionSparkles_CircleInward();
                gTasks[taskId].data[0] += 1;
            }
            EVOSTATE_SPARKLE_SPRAY => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    gBattleCommunication[2] =
                        EvolutionSparkles_SprayAndFlash(gTasks[taskId].data[2] as u16);
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_EVO_SOUND => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    PlaySE(SE_EXP);
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_RESTORE_SCREEN => {
                if IsSEPlaying() != 0 {
                    m4aMPlayAllStop();
                    memcpy(
                        &raw mut gPlttBufferUnfaded[32] as *mut u8,
                        (*sEvoStructPtr).savedPalette.as_mut_ptr() as *mut u8,
                        96,
                    );
                    RestoreBgAfterAnim();
                    BeginNormalPaletteFade(0x1C, 0, 0x10, 0, 0);
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_EVO_MON_ANIM => {
                if gPaletteFade.active() == 0 {
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).postEvoSpriteId,
                        gTasks[taskId].data[2] as u16,
                    );
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_SET_MON_EVOLVED => {
                if IsCryFinished() != 0 {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        gText_CongratsPkmnEvolved.as_ptr().cast_mut(),
                    );
                    BattlePutTextOnWindow(gStringVar4.as_mut_ptr(), B_WIN_MSG);
                    PlayBGM(MUS_EVOLVED);
                    gTasks[taskId].data[0] += 1;
                    SetMonData(
                        mon,
                        MON_DATA_SPECIES,
                        &raw mut gTasks[taskId].data[2] as *mut c_void,
                    );
                    CalculateMonStats(mon);
                    EvolutionRenameMon(
                        mon,
                        gTasks[taskId].data[1] as u16,
                        gTasks[taskId].data[2] as u16,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(gTasks[taskId].data[2] as u16),
                        FLAG_SET_SEEN,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(gTasks[taskId].data[2] as u16),
                        FLAG_SET_CAUGHT,
                    );
                    IncrementGameStat(GAME_STAT_EVOLVED_POKEMON);
                }
            }
            EVOSTATE_TRY_LEARN_MOVE => {
                if IsTextPrinterActive(0) == 0 {
                    var = MonTryLearningNewMove(mon, gTasks[taskId].data[4] as u8) as u32;
                    if var != MOVE_NONE as u32 && gTasks[taskId].data[9] == 0 {
                        let mut nickname: CArray<u8, 20> = zeroed();
                        if gTasks[taskId].data[3] as i32 & TASK_BIT_LEARN_MOVE == 0 {
                            StopMapMusic();
                            Overworld_PlaySpecialMapMusic();
                        }
                        gTasks[taskId].data[3] |= TASK_BIT_LEARN_MOVE as i16;
                        gTasks[taskId].data[4] = FALSE as i16;
                        gTasks[taskId].data[6] = MVSTATE_INTRO_MSG_1;
                        GetMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
                        StringCopy_Nickname(gBattleTextBuff1.as_mut_ptr(), nickname.as_mut_ptr());
                        if var == MON_HAS_MAX_MOVES as u32 {
                            gTasks[taskId].data[0] = EVOSTATE_REPLACE_MOVE;
                        } else if var == MON_ALREADY_KNOWS_MOVE as u32 {
                            break 'l1;
                        } else {
                            gTasks[taskId].data[0] = EVOSTATE_LEARNED_MOVE;
                        }
                    } else {
                        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                        gTasks[taskId].data[0] += 1;
                    }
                }
            }
            EVOSTATE_END => {
                if gPaletteFade.active() == 0 {
                    if gTasks[taskId].data[3] as i32 & TASK_BIT_LEARN_MOVE == 0 {
                        StopMapMusic();
                        Overworld_PlaySpecialMapMusic();
                    }
                    if gTasks[taskId].data[9] == 0 {
                        CreateShedinja(gTasks[taskId].data[1] as u16, mon);
                    }
                    DestroyTask(taskId);
                    FreeMonSpritesGfx();
                    Free(sEvoStructPtr as *mut c_void);
                    sEvoStructPtr = null_mut();
                    FreeAllWindowBuffers();
                    SetMainCallback2(gCB2_AfterEvolution);
                }
            }
            EVOSTATE_CANCEL => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    m4aMPlayAllStop();
                    BeginNormalPaletteFade(0x6001C, 0, 0x10, 0, 32767);
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_CANCEL_MON_ANIM => {
                if gPaletteFade.active() == 0 {
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).preEvoSpriteId,
                        gTasks[taskId].data[1] as u16,
                    );
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_CANCEL_MSG => {
                if EvoScene_IsMonAnimFinished((*sEvoStructPtr).preEvoSpriteId) != 0 {
                    if gTasks[taskId].data[9] != 0 {
                        StringExpandPlaceholders(
                            gStringVar4.as_mut_ptr(),
                            gText_EllipsisQuestionMark.as_ptr().cast_mut(),
                        );
                    } else {
                        StringExpandPlaceholders(
                            gStringVar4.as_mut_ptr(),
                            gText_PkmnStoppedEvolving.as_ptr().cast_mut(),
                        );
                    }
                    BattlePutTextOnWindow(gStringVar4.as_mut_ptr(), B_WIN_MSG);
                    gTasks[taskId].data[9] = TRUE as i16;
                    gTasks[taskId].data[0] = EVOSTATE_TRY_LEARN_MOVE;
                }
            }
            EVOSTATE_LEARNED_MOVE => {
                if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                    BufferMoveToLearnIntoBattleTextBuff2();
                    PlayFanfare(MUS_LEVEL_UP);
                    BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[3]);
                    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                    gTasks[taskId].data[4] = 0x40;
                    gTasks[taskId].data[0] += 1;
                }
            }
            EVOSTATE_TRY_LEARN_ANOTHER_MOVE => {
                if IsTextPrinterActive(0) == 0
                    && IsSEPlaying() == 0
                    && ({
                        gTasks[taskId].data[4] -= 1;
                        gTasks[taskId].data[4]
                    }) == 0
                {
                    gTasks[taskId].data[0] = EVOSTATE_TRY_LEARN_MOVE;
                }
            }
            EVOSTATE_REPLACE_MOVE => 'l2: {
                let sw3: i16 = gTasks[taskId].data[6];
                let mut fall = false;
                if sw3 == MVSTATE_INTRO_MSG_1 {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BufferMoveToLearnIntoBattleTextBuff2();
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[4]);
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        gTasks[taskId].data[6] += 1;
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_INTRO_MSG_2 {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[5]);
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        gTasks[taskId].data[6] += 1;
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_INTRO_MSG_3 {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[6]);
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        gTasks[taskId].data[7] = MVSTATE_SHOW_MOVE_SELECT;
                        gTasks[taskId].data[8] = MVSTATE_ASK_CANCEL;
                        gTasks[taskId].data[6] += 1;
                    }
                }
                if fall || sw3 == MVSTATE_PRINT_YES_NO {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        HandleBattleWindow(24, 8, 29, 13, 0);
                        BattlePutTextOnWindow(
                            gText_BattleYesNoChoice.as_ptr().cast_mut(),
                            B_WIN_YESNO,
                        );
                        gTasks[taskId].data[6] += 1;
                        gBattleCommunication[1] = 0;
                        BattleCreateYesNoCursorAt(0);
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_HANDLE_YES_NO {
                    fall = true;
                    if gMain.newKeys as i32 & DPAD_UP != 0 && gBattleCommunication[1] != 0 {
                        PlaySE(SE_SELECT);
                        BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                        gBattleCommunication[1] = 0;
                        BattleCreateYesNoCursorAt(0);
                    }
                    if gMain.newKeys as i32 & DPAD_DOWN != 0 && gBattleCommunication[1] == 0 {
                        PlaySE(SE_SELECT);
                        BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                        gBattleCommunication[1] = 1;
                        BattleCreateYesNoCursorAt(1);
                    }
                    if gMain.newKeys as i32 & A_BUTTON != 0 {
                        HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                        PlaySE(SE_SELECT);
                        if gBattleCommunication[1] != 0 {
                            gTasks[taskId].data[6] = gTasks[taskId].data[8];
                        } else {
                            gTasks[taskId].data[6] = gTasks[taskId].data[7];
                            if gTasks[taskId].data[6] == MVSTATE_SHOW_MOVE_SELECT {
                                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                            }
                        }
                    }
                    if gMain.newKeys as i32 & B_BUTTON != 0 {
                        HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                        PlaySE(SE_SELECT);
                        gTasks[taskId].data[6] = gTasks[taskId].data[8];
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_SHOW_MOVE_SELECT {
                    fall = true;
                    if gPaletteFade.active() == 0 {
                        FreeAllWindowBuffers();
                        ShowSelectMovePokemonSummaryScreen(
                            gPlayerParty.as_mut_ptr(),
                            gTasks[taskId].data[10] as u8,
                            gPlayerPartyCount - 1,
                            Some(CB2_EvolutionSceneLoadGraphics),
                            gMoveToLearn,
                        );
                        gTasks[taskId].data[6] += 1;
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_HANDLE_MOVE_SELECT {
                    fall = true;
                    if gPaletteFade.active() == 0
                        && gMain.callback2
                            == Some(CB2_EvolutionSceneUpdate as unsafe extern "C" fn())
                    {
                        var = GetMoveSlotToReplace() as u32;
                        if var == MAX_MON_MOVES as u32 {
                            gTasks[taskId].data[6] = MVSTATE_ASK_CANCEL;
                        } else {
                            let mut r#move: u16 =
                                GetMonData2(mon, var as i32 + MON_DATA_MOVE1) as u16;
                            if IsHMMove2(r#move) != 0 {
                                BattleStringExpandPlaceholdersToDisplayedString(
                                    gBattleStringsTable[307],
                                );
                                BattlePutTextOnWindow(
                                    gDisplayedStringBattle.as_mut_ptr(),
                                    B_WIN_MSG,
                                );
                                gTasks[taskId].data[6] = MVSTATE_RETRY_AFTER_HM;
                            } else {
                                gBattleTextBuff2[0] = 0xFD;
                                gBattleTextBuff2[1] = 2;
                                gBattleTextBuff2[2] = r#move as u8 & 0xFF;
                                gBattleTextBuff2[3] = ((r#move as i32 & 0xFF00) >> 8) as u8;
                                gBattleTextBuff2[4] = 0xFF;
                                RemoveMonPPBonus(mon, var as u8);
                                SetMonMoveSlot(mon, gMoveToLearn, var as u8);
                                gTasks[taskId].data[6] += 1;
                            }
                        }
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_FORGET_MSG_1 {
                    fall = true;
                    BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[207]);
                    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                    gTasks[taskId].data[6] += 1;
                    break 'l2;
                }
                if sw3 == MVSTATE_FORGET_MSG_2 {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[7]);
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        gTasks[taskId].data[6] += 1;
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_LEARNED_MOVE {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[208]);
                        BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                        gTasks[taskId].data[0] = EVOSTATE_LEARNED_MOVE;
                    }
                    break 'l2;
                }
                if sw3 == MVSTATE_ASK_CANCEL {
                    fall = true;
                    BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[8]);
                    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                    gTasks[taskId].data[7] = MVSTATE_CANCEL;
                    gTasks[taskId].data[8] = MVSTATE_INTRO_MSG_1;
                    gTasks[taskId].data[6] = MVSTATE_PRINT_YES_NO;
                    break 'l2;
                }
                if sw3 == MVSTATE_CANCEL {
                    fall = true;
                    BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[9]);
                    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                    gTasks[taskId].data[0] = EVOSTATE_TRY_LEARN_MOVE;
                    break 'l2;
                }
                if sw3 == MVSTATE_RETRY_AFTER_HM {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        gTasks[taskId].data[6] = MVSTATE_SHOW_MOVE_SELECT;
                    }
                    break 'l2;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TradeEvolutionScene(taskId: u8) {
    let mut var: u32 = 0;
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gTasks[taskId].data[10]];
    'l1: {
        match gTasks[taskId].data[0] {
            T_EVOSTATE_INTRO_MSG => {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_PkmnIsEvolving.as_ptr().cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 1);
                gTasks[taskId].data[0] += 1;
            }
            T_EVOSTATE_INTRO_CRY => {
                if IsTextPrinterActive(0) == 0 {
                    PlayCry_Normal(gTasks[taskId].data[1] as u16, 0);
                    gTasks[taskId].data[0] += 1;
                }
            }
            T_EVOSTATE_INTRO_SOUND => {
                if IsCryFinished() != 0 {
                    m4aSongNumStop(MUS_EVOLUTION);
                    PlaySE(MUS_EVOLUTION_INTRO);
                    gTasks[taskId].data[0] += 1;
                }
            }
            T_EVOSTATE_START_MUSIC => {
                if IsSEPlaying() == 0 {
                    PlayBGM(MUS_EVOLUTION);
                    gTasks[taskId].data[0] += 1;
                    BeginNormalPaletteFade(0x1C, 4, 0, 0x10, 0);
                }
            }
            T_EVOSTATE_START_BG_AND_SPARKLE_SPIRAL => {
                if gPaletteFade.active() == 0 {
                    StartBgAnimation(TRUE);
                    var = gSprites[(*sEvoStructPtr).preEvoSpriteId].oam.paletteNum() as u32 + 16;
                    gBattleCommunication[2] = EvolutionSparkles_SpiralUpward(var as u16);
                    gTasks[taskId].data[0] += 1;
                    SetGpuReg(REG_OFFSET_BG3CNT, 1539);
                }
            }
            T_EVOSTATE_SPARKLE_ARC => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    gTasks[taskId].data[0] += 1;
                    (*sEvoStructPtr).delayTimer = 1;
                    gBattleCommunication[2] = EvolutionSparkles_ArcDown();
                }
            }
            T_EVOSTATE_CYCLE_MON_SPRITE => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    gBattleCommunication[2] = CycleEvolutionMonSprite(
                        (*sEvoStructPtr).preEvoSpriteId,
                        (*sEvoStructPtr).postEvoSpriteId,
                    );
                    gTasks[taskId].data[0] += 1;
                }
            }
            T_EVOSTATE_WAIT_CYCLE_MON_SPRITE => {
                if ({
                    (*sEvoStructPtr).delayTimer -= 1;
                    (*sEvoStructPtr).delayTimer
                }) == 0
                {
                    (*sEvoStructPtr).delayTimer = 3;
                    if gTasks[gBattleCommunication[2]].isActive == 0 {
                        gTasks[taskId].data[0] += 1;
                    }
                }
            }
            T_EVOSTATE_SPARKLE_CIRCLE => {
                gBattleCommunication[2] = EvolutionSparkles_CircleInward();
                gTasks[taskId].data[0] += 1;
            }
            T_EVOSTATE_SPARKLE_SPRAY => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    gBattleCommunication[2] =
                        EvolutionSparkles_SprayAndFlash_Trade(gTasks[taskId].data[2] as u16);
                    gTasks[taskId].data[0] += 1;
                }
            }
            T_EVOSTATE_EVO_SOUND => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    PlaySE(SE_EXP);
                    gTasks[taskId].data[0] += 1;
                }
            }
            T_EVOSTATE_EVO_MON_ANIM => {
                if IsSEPlaying() != 0 {
                    Free(sBgAnimPal as *mut c_void);
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).postEvoSpriteId,
                        gTasks[taskId].data[2] as u16,
                    );
                    memcpy(
                        &raw mut gPlttBufferUnfaded[32] as *mut u8,
                        (*sEvoStructPtr).savedPalette.as_mut_ptr() as *mut u8,
                        96,
                    );
                    gTasks[taskId].data[0] += 1;
                }
            }
            T_EVOSTATE_SET_MON_EVOLVED => {
                if IsCryFinished() != 0 {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        gText_CongratsPkmnEvolved.as_ptr().cast_mut(),
                    );
                    DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 1);
                    PlayFanfare(MUS_EVOLVED);
                    gTasks[taskId].data[0] += 1;
                    SetMonData(
                        mon,
                        MON_DATA_SPECIES,
                        &raw mut gTasks[taskId].data[2] as *mut c_void,
                    );
                    CalculateMonStats(mon);
                    EvolutionRenameMon(
                        mon,
                        gTasks[taskId].data[1] as u16,
                        gTasks[taskId].data[2] as u16,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(gTasks[taskId].data[2] as u16),
                        FLAG_SET_SEEN,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(gTasks[taskId].data[2] as u16),
                        FLAG_SET_CAUGHT,
                    );
                    IncrementGameStat(GAME_STAT_EVOLVED_POKEMON);
                }
            }
            T_EVOSTATE_TRY_LEARN_MOVE => {
                if IsTextPrinterActive(0) == 0 && IsFanfareTaskInactive() == TRUE {
                    var = MonTryLearningNewMove(mon, gTasks[taskId].data[4] as u8) as u32;
                    if var != MOVE_NONE as u32 && gTasks[taskId].data[9] == 0 {
                        let mut nickname: CArray<u8, 20> = zeroed();
                        gTasks[taskId].data[3] |= TASK_BIT_LEARN_MOVE as i16;
                        gTasks[taskId].data[4] = FALSE as i16;
                        gTasks[taskId].data[6] = 0;
                        GetMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
                        StringCopy_Nickname(gBattleTextBuff1.as_mut_ptr(), nickname.as_mut_ptr());
                        if var == MON_HAS_MAX_MOVES as u32 {
                            gTasks[taskId].data[0] = T_EVOSTATE_REPLACE_MOVE;
                        } else if var == MON_ALREADY_KNOWS_MOVE as u32 {
                            break 'l1;
                        } else {
                            gTasks[taskId].data[0] = T_EVOSTATE_LEARNED_MOVE;
                        }
                    } else {
                        PlayBGM(MUS_EVOLUTION);
                        DrawTextOnTradeWindow(
                            0,
                            gText_CommunicationStandby5.as_ptr().cast_mut(),
                            1,
                        );
                        gTasks[taskId].data[0] += 1;
                    }
                }
            }
            T_EVOSTATE_END => {
                if IsTextPrinterActive(0) == 0 {
                    DestroyTask(taskId);
                    Free(sEvoStructPtr as *mut c_void);
                    sEvoStructPtr = null_mut();
                    gTextFlags.set_useAlternateDownArrow(FALSE);
                    SetMainCallback2(gCB2_AfterEvolution);
                }
            }
            T_EVOSTATE_CANCEL => {
                if gTasks[gBattleCommunication[2]].isActive == 0 {
                    m4aMPlayAllStop();
                    BeginNormalPaletteFade(
                        shl_i32(
                            1,
                            gSprites[(*sEvoStructPtr).preEvoSpriteId].oam.paletteNum() as u32 + 16,
                        ) as u32
                            | 0x4001C,
                        0,
                        0x10,
                        0,
                        32767,
                    );
                    gTasks[taskId].data[0] += 1;
                }
            }
            T_EVOSTATE_CANCEL_MON_ANIM => {
                if gPaletteFade.active() == 0 {
                    EvoScene_DoMonAnimAndCry(
                        (*sEvoStructPtr).preEvoSpriteId,
                        gTasks[taskId].data[1] as u16,
                    );
                    gTasks[taskId].data[0] += 1;
                }
            }
            T_EVOSTATE_CANCEL_MSG => {
                if EvoScene_IsMonAnimFinished((*sEvoStructPtr).preEvoSpriteId) != 0 {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        gText_EllipsisQuestionMark.as_ptr().cast_mut(),
                    );
                    DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 1);
                    gTasks[taskId].data[9] = TRUE as i16;
                    gTasks[taskId].data[0] = T_EVOSTATE_TRY_LEARN_MOVE;
                }
            }
            T_EVOSTATE_LEARNED_MOVE => {
                if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                    BufferMoveToLearnIntoBattleTextBuff2();
                    PlayFanfare(MUS_LEVEL_UP);
                    BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[3]);
                    DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                    gTasks[taskId].data[4] = 0x40;
                    gTasks[taskId].data[0] += 1;
                }
            }
            T_EVOSTATE_TRY_LEARN_ANOTHER_MOVE => {
                if IsTextPrinterActive(0) == 0
                    && IsFanfareTaskInactive() == TRUE
                    && ({
                        gTasks[taskId].data[4] -= 1;
                        gTasks[taskId].data[4]
                    }) == 0
                {
                    gTasks[taskId].data[0] = T_EVOSTATE_TRY_LEARN_MOVE;
                }
            }
            T_EVOSTATE_REPLACE_MOVE => 'l2: {
                let sw3: i16 = gTasks[taskId].data[6];
                let mut fall = false;
                if sw3 == T_MVSTATE_INTRO_MSG_1 {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BufferMoveToLearnIntoBattleTextBuff2();
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[4]);
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        gTasks[taskId].data[6] += 1;
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_INTRO_MSG_2 {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[5]);
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        gTasks[taskId].data[6] += 1;
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_INTRO_MSG_3 {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[6]);
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        gTasks[taskId].data[7] = T_MVSTATE_SHOW_MOVE_SELECT;
                        gTasks[taskId].data[8] = T_MVSTATE_ASK_CANCEL;
                        gTasks[taskId].data[6] += 1;
                    }
                }
                if fall || sw3 == T_MVSTATE_PRINT_YES_NO {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        LoadUserWindowBorderGfx(0, 0xA8, 224);
                        CreateYesNoMenu(
                            (&raw const gTradeEvolutionSceneYesNoWindowTemplate).cast_mut(),
                            0xA8,
                            0xE,
                            0,
                        );
                        gBattleCommunication[1] = 0;
                        gTasks[taskId].data[6] += 1;
                        gBattleCommunication[1] = 0;
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_HANDLE_YES_NO {
                    fall = true;
                    match Menu_ProcessInputNoWrapClearOnChoose() {
                        0 => {
                            gBattleCommunication[1] = 0;
                            BattleStringExpandPlaceholdersToDisplayedString(
                                gBattleStringsTable[292],
                            );
                            DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                            gTasks[taskId].data[6] = gTasks[taskId].data[7];
                            if gTasks[taskId].data[6] == T_MVSTATE_SHOW_MOVE_SELECT {
                                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                            }
                        }
                        1 | MENU_B_PRESSED => {
                            gBattleCommunication[1] = 1;
                            BattleStringExpandPlaceholdersToDisplayedString(
                                gBattleStringsTable[292],
                            );
                            DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                            gTasks[taskId].data[6] = gTasks[taskId].data[8];
                        }
                        _ => {}
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_SHOW_MOVE_SELECT {
                    fall = true;
                    if gPaletteFade.active() == 0 {
                        if gWirelessCommType != 0 {
                            DestroyWirelessStatusIndicatorSprite();
                        }
                        Free(GetBgTilemapBuffer(3));
                        Free(GetBgTilemapBuffer(1));
                        Free(GetBgTilemapBuffer(0));
                        FreeAllWindowBuffers();
                        ShowSelectMovePokemonSummaryScreen(
                            gPlayerParty.as_mut_ptr(),
                            gTasks[taskId].data[10] as u8,
                            gPlayerPartyCount - 1,
                            Some(CB2_TradeEvolutionSceneLoadGraphics),
                            gMoveToLearn,
                        );
                        gTasks[taskId].data[6] += 1;
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_HANDLE_MOVE_SELECT {
                    fall = true;
                    if gPaletteFade.active() == 0
                        && gMain.callback2
                            == Some(CB2_TradeEvolutionSceneUpdate as unsafe extern "C" fn())
                    {
                        var = GetMoveSlotToReplace() as u32;
                        if var == MAX_MON_MOVES as u32 {
                            gTasks[taskId].data[6] = T_MVSTATE_ASK_CANCEL;
                        } else {
                            let mut r#move: u16 =
                                GetMonData2(mon, var as i32 + MON_DATA_MOVE1) as u16;
                            if IsHMMove2(r#move) != 0 {
                                BattleStringExpandPlaceholdersToDisplayedString(
                                    gBattleStringsTable[307],
                                );
                                DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                                gTasks[taskId].data[6] = T_MVSTATE_RETRY_AFTER_HM;
                            } else {
                                gBattleTextBuff2[0] = 0xFD;
                                gBattleTextBuff2[1] = 2;
                                gBattleTextBuff2[2] = r#move as u8 & 0xFF;
                                gBattleTextBuff2[3] = ((r#move as i32 & 0xFF00) >> 8) as u8;
                                gBattleTextBuff2[4] = 0xFF;
                                RemoveMonPPBonus(mon, var as u8);
                                SetMonMoveSlot(mon, gMoveToLearn, var as u8);
                                BattleStringExpandPlaceholdersToDisplayedString(
                                    gBattleStringsTable[207],
                                );
                                DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                                gTasks[taskId].data[6] += 1;
                            }
                        }
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_FORGET_MSG {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[7]);
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        gTasks[taskId].data[6] += 1;
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_LEARNED_MOVE {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[208]);
                        DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                        gTasks[taskId].data[0] = T_EVOSTATE_LEARNED_MOVE;
                    }
                    break 'l2;
                }
                if sw3 == T_MVSTATE_ASK_CANCEL {
                    fall = true;
                    BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[8]);
                    DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                    gTasks[taskId].data[7] = T_MVSTATE_CANCEL;
                    gTasks[taskId].data[8] = T_MVSTATE_INTRO_MSG_1;
                    gTasks[taskId].data[6] = T_MVSTATE_PRINT_YES_NO;
                    break 'l2;
                }
                if sw3 == T_MVSTATE_CANCEL {
                    fall = true;
                    BattleStringExpandPlaceholdersToDisplayedString(gBattleStringsTable[9]);
                    DrawTextOnTradeWindow(0, gDisplayedStringBattle.as_mut_ptr(), 1);
                    gTasks[taskId].data[0] = T_EVOSTATE_TRY_LEARN_MOVE;
                    break 'l2;
                }
                if sw3 == T_MVSTATE_RETRY_AFTER_HM {
                    fall = true;
                    if IsTextPrinterActive(0) == 0 && IsSEPlaying() == 0 {
                        gTasks[taskId].data[6] = T_MVSTATE_SHOW_MOVE_SELECT;
                    }
                    break 'l2;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn EvoDummyFunc() {}
pub(crate) unsafe extern "C" fn VBlankCB_EvolutionScene() {
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_X);
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_Y);
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X);
    SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y);
    SetGpuReg(REG_OFFSET_BG3HOFS, gBattle_BG3_X);
    SetGpuReg(REG_OFFSET_BG3VOFS, gBattle_BG3_Y);
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn VBlankCB_TradeEvolutionScene() {
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_X);
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_Y);
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X);
    SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y);
    SetGpuReg(REG_OFFSET_BG3HOFS, gBattle_BG3_X);
    SetGpuReg(REG_OFFSET_BG3VOFS, gBattle_BG3_Y);
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn Task_UpdateBgPalette(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(6) != 0 {
        return;
    }
    if ({
        let t1 = *data.at(5);
        *data.at(5) += 1;
        t1
    }) < 20
    {
        return;
    }
    if ({
        let t2 = *data;
        *data += 1;
        t2
    }) > sBgAnim_PaletteControl[*data.at(2)][3] as i16
    {
        if sBgAnim_PaletteControl[*data.at(2)][1] as i16 == *data.at(1) {
            *data.at(3) += 1;
            if *data.at(3) == sBgAnim_PaletteControl[*data.at(2)][2] as i16 {
                *data.at(3) = 0;
                *data.at(2) += 1;
            }
            *data.at(1) = sBgAnim_PaletteControl[*data.at(2)][0] as i16;
        } else {
            LoadPalette(
                sBgAnimPal.at(*data.at(1) as i32 * 16) as *mut c_void,
                160,
                32,
            );
            *data = 0;
            *data.at(1) += 1;
        }
    }
    if *data.at(2) == 4 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CreateBgAnimTask(isLink: u8) {
    let mut taskId: u8 = CreateTask(Some(Task_AnimateBg), 7);
    if isLink == 0 {
        gTasks[taskId].data[2] = FALSE as i16;
    } else {
        gTasks[taskId].data[2] = TRUE as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateBg(taskId: u8) {
    let mut outer_X: *mut u16 = null_mut();
    let mut outer_Y: *mut u16 = null_mut();
    let mut inner_X: *mut u16 = &raw mut gBattle_BG1_X;
    let mut inner_Y: *mut u16 = &raw mut gBattle_BG1_Y;
    if gTasks[taskId].data[2] == 0 {
        outer_X = &raw mut gBattle_BG2_X;
        outer_Y = &raw mut gBattle_BG2_Y;
    } else {
        outer_X = &raw mut gBattle_BG3_X;
        outer_Y = &raw mut gBattle_BG3_Y;
    }
    gTasks[taskId].data[0] = gTasks[taskId].data[0] + 5 & 0xFF;
    gTasks[taskId].data[1] = gTasks[taskId].data[0] + 0x80 & 0xFF;
    *inner_X = Cos(gTasks[taskId].data[0], 4) as u16 + 8;
    *inner_Y = Sin(gTasks[taskId].data[0], 4) as u16 + 16;
    *outer_X = Cos(gTasks[taskId].data[1], 4) as u16 + 8;
    *outer_Y = Sin(gTasks[taskId].data[1], 4) as u16 + 16;
    if FuncIsActiveTask(Some(Task_UpdateBgPalette)) == 0 {
        DestroyTask(taskId);
        *inner_X = 0;
        *inner_Y = 0;
        *outer_X = 256;
        *outer_Y = 0;
    }
}
pub(crate) unsafe extern "C" fn InitMovingBgPalette(mut palette: *mut u16) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < 50 {
        j = 0;
        while j < 16 {
            *palette.at(i * 16 + j) = sBgAnim_Pal[sBgAnim_PalIndexes[i][j]];
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn StartBgAnimation(isLink: u8) {
    let mut innerBgId: u8 = 0;
    let mut outerBgId: u8 = 0;
    sBgAnimPal = AllocZeroed(0x640) as *mut u16;
    InitMovingBgPalette(sBgAnimPal);
    if isLink == 0 {
        innerBgId = 1;
        outerBgId = 2;
    } else {
        innerBgId = 1;
        outerBgId = 3;
    }
    LoadPalette(
        sBgAnim_Intro_Pal.as_ptr().cast_mut() as *mut c_void,
        160,
        32,
    );
    DecompressAndLoadBgGfxUsingHeap(1, sBgAnim_Gfx.as_ptr().cast_mut() as *mut c_void, 0, 0, 0);
    CopyToBgTilemapBuffer(
        innerBgId,
        sBgAnim_Inner_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyToBgTilemapBuffer(
        outerBgId,
        sBgAnim_Outer_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(innerBgId);
    CopyBgTilemapBufferToVram(outerBgId);
    if isLink == 0 {
        SetGpuReg(REG_OFFSET_BLDCNT, 1090);
        SetGpuReg(REG_OFFSET_BLDALPHA, 2056);
        SetGpuReg(REG_OFFSET_DISPCNT, 5952);
        SetBgAttribute(innerBgId, BG_ATTR_PRIORITY, 2);
        SetBgAttribute(outerBgId, BG_ATTR_PRIORITY, 2);
        ShowBg(1);
        ShowBg(2);
    } else {
        SetGpuReg(REG_OFFSET_BLDCNT, 2114);
        SetGpuReg(REG_OFFSET_BLDALPHA, 2056);
        SetGpuReg(REG_OFFSET_DISPCNT, 6976);
    }
    CreateTask(Some(Task_UpdateBgPalette), 5);
    CreateBgAnimTask(isLink);
}
pub(crate) unsafe extern "C" fn PauseBgPaletteAnim() {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_UpdateBgPalette));
    if taskId != TASK_NONE {
        gTasks[taskId].data[6] = TRUE as i16;
    }
    FillPalette(0, 160, 32);
}
pub(crate) unsafe extern "C" fn StopBgAnimation() {
    let mut taskId: u8 = 0;
    if ({
        taskId = FindTaskIdByFunc(Some(Task_UpdateBgPalette));
        taskId
    }) != TASK_NONE
    {
        DestroyTask(taskId);
    }
    if ({
        taskId = FindTaskIdByFunc(Some(Task_AnimateBg));
        taskId
    }) != TASK_NONE
    {
        DestroyTask(taskId);
    }
    FillPalette(0, 160, 32);
    RestoreBgAfterAnim();
}
pub(crate) unsafe extern "C" fn RestoreBgAfterAnim() {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    SetBgAttribute(1, BG_ATTR_PRIORITY, GetBattleBgTemplateData(1, 5) as u8);
    SetBgAttribute(2, BG_ATTR_PRIORITY, GetBattleBgTemplateData(2, 5) as u8);
    SetGpuReg(REG_OFFSET_DISPCNT, 6464);
    Free(sBgAnimPal as *mut c_void);
}
pub(crate) unsafe extern "C" fn EvoScene_DoMonAnimAndCry(monSpriteId: u8, speciesId: u16) {
    DoMonFrontSpriteAnimation(&raw mut gSprites[monSpriteId], speciesId, 0, 0);
}
pub(crate) unsafe extern "C" fn EvoScene_IsMonAnimFinished(monSpriteId: u8) -> u32 {
    if gSprites[monSpriteId].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        return TRUE as u32;
    }
    return FALSE as u32;
}
