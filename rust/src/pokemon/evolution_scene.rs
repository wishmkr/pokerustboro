//! Translated from `src/evolution_scene.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sUnusedPal1 sBgAnim_Gfx sBgAnim_Inner_Tilemap sBgAnim_Outer_Tilemap sBgAnim_Intro_Pal sUnusedPal2 sUnusedPal3 sUnusedPal4 sBgAnim_Pal sText_ShedinjaJapaneseName sBgAnim_PaletteControl sBgAnim_PalIndexes
#[allow(unused_imports)]
use crate::data::evolution_scene::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sEvoStructPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBgAnimPal: *mut u16 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCB2_AfterEvolution: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" {
    static mut gAffineAnimsDisabled: u8;
    static mut gBattleCommunication: u8;
    static mut gBattleEnvironment: u8;
    static mut gBattleStringsTable: u8;
    static mut gBattleTextBuff1: u8;
    static mut gBattleTextBuff2: u8;
    static mut gBattle_BG0_X: u8;
    static mut gBattle_BG0_Y: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_BG2_Y: u8;
    static mut gBattle_BG3_X: u8;
    static mut gBattle_BG3_Y: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gDummySpriteAffineAnimTable: u8;
    static mut gEvolutionTable: u8;
    static mut gMain: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gMoveToLearn: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyCount: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gText_BattleYesNoChoice: u8;
    static mut gText_CommunicationStandby5: u8;
    static mut gText_CongratsPkmnEvolved: u8;
    static mut gText_EllipsisQuestionMark: u8;
    static mut gText_PkmnIsEvolving: u8;
    static mut gText_PkmnStoppedEvolving: u8;
    static mut gTradeEvolutionSceneYesNoWindowTemplate: u8;
    static mut gWirelessCommType: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
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
    fn CalculateMonStats(a0: *mut u8);
    fn CalculatePlayerPartyCount() -> u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyMon(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn CycleEvolutionMonSprite(a0: u8, a1: u8) -> u8;
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn DecompressPicFromTable_2(a0: *mut u8, a1: *mut u8, a2: i32);
    fn DestroyTask(a0: u8);
    fn DestroyWirelessStatusIndicatorSprite();
    fn DoMonFrontSpriteAnimation(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DrawTextOnTradeWindow(a0: u8, a1: *mut u8, a2: u8);
    fn EvolutionRenameMon(a0: *mut u8, a1: u16, a2: u16);
    fn EvolutionSparkles_ArcDown() -> u8;
    fn EvolutionSparkles_CircleInward() -> u8;
    fn EvolutionSparkles_SpiralUpward(a0: u16) -> u8;
    fn EvolutionSparkles_SprayAndFlash(a0: u16) -> u8;
    fn EvolutionSparkles_SprayAndFlash_Trade(a0: u16) -> u8;
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonSpritesGfx();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBattleBgTemplateData(a0: u8, a1: u8) -> u32;
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonSpritePalStructFromOtIdPersonality(a0: u16, a1: u32, a2: u32) -> *mut u8;
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
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadTradeAnimGfx();
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MonTryLearningNewMove(a0: *mut u8, a1: u8) -> u16;
    fn Overworld_PlaySpecialMapMusic();
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayFanfare(a0: u16);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn RemoveMonPPBonus(a0: *mut u8, a1: u8);
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
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowSelectMovePokemonSummaryScreen(
        a0: *mut u8,
        a1: u8,
        a2: u8,
        a3: Option<unsafe extern "C" fn()>,
        a4: u16,
    );
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn SpriteCallbackDummy_2(a0: *mut u8);
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
    unsafe {
        UpdatePaletteFade();
        RunTasks();
    }
}
pub(crate) unsafe extern "C" fn Task_BeginEvolutionScene(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let mut postEvoSpecies: u16 = 0u16;
                    let mut canStopEvo: u8 = 0u8;
                    let mut partyId: u8 = 0u8;
                    mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .read()) as i32) as isize
                            * 100,
                    );
                    postEvoSpecies = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as u16);
                    canStopEvo = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as u8);
                    partyId = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read()) as u8);
                    DestroyTask(taskId);
                    EvolutionScene(mon, postEvoSpecies, canStopEvo, partyId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BeginEvolutionScene(
    mon: *mut u8,
    postEvoSpecies: u16,
    canStopEvo: u8,
    partyId: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut postEvoSpecies = postEvoSpecies;
        let mut canStopEvo = canStopEvo;
        let mut partyId = partyId;
        let mut taskId: u8 = CreateTask(Some(Task_BeginEvolutionScene), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((postEvoSpecies) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((canStopEvo) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(((partyId) as i16));
        SetMainCallback2(Some(CB2_BeginEvolutionScene));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionScene(
    mon: *mut u8,
    postEvoSpecies: u16,
    canStopEvo: u8,
    partyId: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut postEvoSpecies = postEvoSpecies;
        let mut canStopEvo = canStopEvo;
        let mut partyId = partyId;
        let mut name = crate::ffi::Align4([0u8; 20]);
        let mut currSpecies: u16 = 0u16;
        let mut trainerId: u32 = 0u32;
        let mut personality: u32 = 0u32;
        let mut pokePal: *mut u8 = core::ptr::null_mut();
        let mut id: u8 = 0u8;
        SetHBlankCallback(None);
        SetVBlankCallback(None);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((100663296i32) as usize as *mut u8),
                                ((83886080i32
                                    | (crate::c::div_i32(98304i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        SetGpuReg(76u8, 0u16);
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        SetGpuReg(66u8, 0u16);
        SetGpuReg(70u8, 0u16);
        SetGpuReg(72u8, 0u16);
        SetGpuReg(74u8, 0u16);
        ResetPaletteFade();
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(256u16);
        ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattleEnvironment).cast::<u8>()).write(9u8);
        InitBattleBgsVideo();
        LoadBattleTextboxAndBackground();
        ResetSpriteData();
        ScanlineEffect_Stop();
        ResetTasks();
        FreeAllSpritePalettes();
        ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(4u8);
        ((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(100u32));
        AllocateMonSpritesGfx();
        GetMonData3(mon, 2i32, (&raw mut name).cast::<u8>());
        StringCopy_Nickname(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut name).cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gSpeciesNames).cast::<u8>())
                .wrapping_offset(((postEvoSpecies) as i32) as isize * 11))
            .cast::<u8>(),
        );
        currSpecies = ((GetMonData2(mon, 11i32)) as u16);
        trainerId = GetMonData2(mon, 1i32);
        personality = GetMonData2(mon, 0i32);
        DecompressPicFromTable_2(
            ((&raw mut gMonFrontPicTable).cast::<u8>())
                .wrapping_offset(((currSpecies) as i32) as isize * 8),
            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<*mut u8>())
            .wrapping_offset(1))
            .read(),
            ((currSpecies) as i32),
        );
        pokePal = GetMonSpritePalStructFromOtIdPersonality(currSpecies, trainerId, personality);
        LoadCompressedPalette(((pokePal).cast::<*mut u32>()).read(), 272u16, 32u16);
        SetMultiuseSpriteTemplateToPokemon(currSpecies, 1u8);
        (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
        .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).write({
            let __v1 = CreateSprite(
                (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                120i16,
                64i16,
                30u8,
            );
            id = __v1;
            __v1
        });
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy_2));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(5),
            4,
            4,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        DecompressPicFromTable_2(
            ((&raw mut gMonFrontPicTable).cast::<u8>())
                .wrapping_offset(((postEvoSpecies) as i32) as isize * 8),
            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<*mut u8>())
            .wrapping_offset(3))
            .read(),
            ((postEvoSpecies) as i32),
        );
        pokePal = GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
        LoadCompressedPalette(((pokePal).cast::<*mut u32>()).read(), 288u16, 32u16);
        SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, 3u8);
        (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
        .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).write(
            {
                let __v2 = CreateSprite(
                    (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                    120i16,
                    64i16,
                    30u8,
                );
                id = __v2;
                __v2
            },
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy_2));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(5),
            4,
            4,
            (2u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        LoadEvoSparkleSpriteAndPal();
        ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2)).write(
            {
                let __v3 = CreateTask(Some(Task_EvolutionScene), 0u8);
                id = __v3;
                __v3
            },
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((currSpecies) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((postEvoSpecies) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((canStopEvo) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(((partyId) as i16));
        crate::c::memcpy(
            (((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u16>())
            .cast::<u8>(),
            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(32))
                .cast::<u8>(),
            96u32,
        );
        SetGpuReg(0u8, 8000u16);
        SetHBlankCallback(Some(EvoDummyFunc));
        SetVBlankCallback(Some(VBlankCB_EvolutionScene));
        m4aMPlayAllStop();
        SetMainCallback2(Some(CB2_EvolutionSceneUpdate));
    }
}
pub(crate) unsafe extern "C" fn CB2_EvolutionSceneLoadGraphics() {
    unsafe {
        let mut id: u8 = 0u8;
        let mut pokePal: *mut u8 = core::ptr::null_mut();
        let mut postEvoSpecies: u16 = 0u16;
        let mut trainerId: u32 = 0u32;
        let mut personality: u32 = 0u32;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32) as isize
                * 100,
        );
        postEvoSpecies = ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u16);
        trainerId = GetMonData2(mon, 1i32);
        personality = GetMonData2(mon, 0i32);
        SetHBlankCallback(None);
        SetVBlankCallback(None);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((100663296i32) as usize as *mut u8),
                                ((83886080i32
                                    | (crate::c::div_i32(98304i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        SetGpuReg(76u8, 0u16);
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        SetGpuReg(66u8, 0u16);
        SetGpuReg(70u8, 0u16);
        SetGpuReg(72u8, 0u16);
        SetGpuReg(74u8, 0u16);
        ResetPaletteFade();
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(256u16);
        ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattleEnvironment).cast::<u8>()).write(9u8);
        InitBattleBgsVideo();
        LoadBattleTextboxAndBackground();
        ResetSpriteData();
        FreeAllSpritePalettes();
        ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(4u8);
        DecompressPicFromTable_2(
            ((&raw mut gMonFrontPicTable).cast::<u8>())
                .wrapping_offset(((postEvoSpecies) as i32) as isize * 8),
            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<*mut u8>())
            .wrapping_offset(3))
            .read(),
            ((postEvoSpecies) as i32),
        );
        pokePal = GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
        LoadCompressedPalette(((pokePal).cast::<*mut u32>()).read(), 288u16, 32u16);
        SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, 3u8);
        (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
        .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).write(
            {
                let __v1 = CreateSprite(
                    (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                    120i16,
                    64i16,
                    30u8,
                );
                id = __v1;
                __v1
            },
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy_2));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(5),
            4,
            4,
            (2u16) as i32,
        );
        SetGpuReg(0u8, 8000u16);
        SetHBlankCallback(Some(EvoDummyFunc));
        SetVBlankCallback(Some(VBlankCB_EvolutionScene));
        SetMainCallback2(Some(CB2_EvolutionSceneUpdate));
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
    }
}
pub(crate) unsafe extern "C" fn CB2_TradeEvolutionSceneLoadGraphics() {
    unsafe {
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32) as isize
                * 100,
        );
        let mut postEvoSpecies: u16 = ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u16);
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                SetGpuReg(0u8, 0u16);
                SetHBlankCallback(None);
                SetVBlankCallback(None);
                ResetSpriteData();
                FreeAllSpritePalettes();
                ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(4u8);
                ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG3_X).cast::<u16>()).write(256u16);
                ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetPaletteFade();
                SetHBlankCallback(Some(EvoDummyFunc));
                SetVBlankCallback(Some(VBlankCB_TradeEvolutionScene));
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadTradeAnimGfx();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                FillBgTilemapBufferRect(1u8, 0u16, 0u8, 0u8, 32u8, 32u8, 17u8);
                CopyBgTilemapBufferToVram(1u8);
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                {
                    let mut pokePal: *mut u8 = core::ptr::null_mut();
                    let mut trainerId: u32 = GetMonData2(mon, 1i32);
                    let mut personality: u32 = GetMonData2(mon, 0i32);
                    DecompressPicFromTable_2(
                        ((&raw mut gMonFrontPicTable).cast::<u8>())
                            .wrapping_offset(((postEvoSpecies) as i32) as isize * 8),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(3))
                        .read(),
                        ((postEvoSpecies) as i32),
                    );
                    pokePal = GetMonSpritePalStructFromOtIdPersonality(
                        postEvoSpecies,
                        trainerId,
                        personality,
                    );
                    LoadCompressedPalette(((pokePal).cast::<*mut u32>()).read(), 288u16, 32u16);
                    let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                {
                    let mut id: u8 = 0u8;
                    SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, 1u8);
                    (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
                        .wrapping_add(16)
                        .cast::<*mut *mut u8>())
                    .write(
                        ((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>())
                            .cast::<*mut u8>(),
                    );
                    ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .write({
                        let __v7 = CreateSprite(
                            (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                            120i16,
                            64i16,
                            30u8,
                        );
                        id = __v7;
                        __v7
                    });
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy_2));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 68))
                        .wrapping_add(5),
                        4,
                        4,
                        (2u16) as i32,
                    );
                    let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    LinkTradeDrawWindow();
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                }
                BlendPalettes(4294967295u32, 16u8, 0u16);
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                InitTradeSequenceBgGpuRegs();
                ShowBg(0u8);
                ShowBg(1u8);
                SetMainCallback2(Some(CB2_TradeEvolutionSceneUpdate));
                SetGpuReg(0u8, 4928u16);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TradeEvolutionScene(
    mon: *mut u8,
    postEvoSpecies: u16,
    preEvoSpriteId: u8,
    partyId: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut postEvoSpecies = postEvoSpecies;
        let mut preEvoSpriteId = preEvoSpriteId;
        let mut partyId = partyId;
        let mut name = crate::ffi::Align4([0u8; 20]);
        let mut currSpecies: u16 = 0u16;
        let mut trainerId: u32 = 0u32;
        let mut personality: u32 = 0u32;
        let mut pokePal: *mut u8 = core::ptr::null_mut();
        let mut id: u8 = 0u8;
        GetMonData3(mon, 2i32, (&raw mut name).cast::<u8>());
        StringCopy_Nickname(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut name).cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gSpeciesNames).cast::<u8>())
                .wrapping_offset(((postEvoSpecies) as i32) as isize * 11))
            .cast::<u8>(),
        );
        ((&raw mut gAffineAnimsDisabled).cast::<u8>()).write(1u8);
        currSpecies = ((GetMonData2(mon, 11i32)) as u16);
        personality = GetMonData2(mon, 0i32);
        trainerId = GetMonData2(mon, 1i32);
        ((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(100u32));
        (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).write(preEvoSpriteId);
        DecompressPicFromTable_2(
            ((&raw mut gMonFrontPicTable).cast::<u8>())
                .wrapping_offset(((postEvoSpecies) as i32) as isize * 8),
            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<*mut u8>())
            .wrapping_offset(1))
            .read(),
            ((postEvoSpecies) as i32),
        );
        pokePal = GetMonSpritePalStructFromOtIdPersonality(postEvoSpecies, trainerId, personality);
        LoadCompressedPalette(((pokePal).cast::<*mut u32>()).read(), 288u16, 32u16);
        SetMultiuseSpriteTemplateToPokemon(postEvoSpecies, 1u8);
        (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
        .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).write(
            {
                let __v1 = CreateSprite(
                    (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                    120i16,
                    64i16,
                    30u8,
                );
                id = __v1;
                __v1
            },
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy_2));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(5),
            4,
            4,
            (2u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        LoadEvoSparkleSpriteAndPal();
        ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2)).write(
            {
                let __v2 = CreateTask(Some(Task_TradeEvolutionScene), 0u8);
                id = __v2;
                __v2
            },
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((currSpecies) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((postEvoSpecies) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(((partyId) as i16));
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(256u16);
        ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            1,
            1,
            (1u8) as i32,
        );
        SetVBlankCallback(Some(VBlankCB_TradeEvolutionScene));
        SetMainCallback2(Some(CB2_TradeEvolutionSceneUpdate));
    }
}
pub(crate) unsafe extern "C" fn CB2_EvolutionSceneUpdate() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
        RunTasks();
    }
}
pub(crate) unsafe extern "C" fn CB2_TradeEvolutionSceneUpdate() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
        RunTasks();
    }
}
pub(crate) unsafe extern "C" fn CreateShedinja(preEvoSpecies: u16, mon: *mut u8) {
    unsafe {
        let mut preEvoSpecies = preEvoSpecies;
        let mut mon = mon;
        let mut data: u32 = 0u32;
        if ((((((((&raw mut gEvolutionTable).cast::<u8>())
            .wrapping_offset(((preEvoSpecies) as i32) as isize * 40))
        .cast::<u8>())
        .cast::<u16>())
        .read()) as i32)
            == 13i32)
            && (((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) < 6i32)
        {
            let mut i: i32 = 0i32;
            let mut shedinja: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
            );
            CopyMon(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
                ),
                mon,
                100u32,
            );
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
                ),
                11i32,
                ((((((&raw mut gEvolutionTable).cast::<u8>())
                    .wrapping_offset(((preEvoSpecies) as i32) as isize * 40))
                .cast::<u8>())
                .wrapping_offset(8))
                .wrapping_add(4)
                .cast::<u16>())
                .cast::<u8>(),
            );
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
                ),
                2i32,
                (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gEvolutionTable).cast::<u8>())
                        .wrapping_offset(((preEvoSpecies) as i32) as isize * 40))
                    .cast::<u8>())
                    .wrapping_offset(8))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 11,
                ))
                .cast::<u8>(),
            );
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
                ),
                12i32,
                (&raw mut data).cast::<u8>(),
            );
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
                ),
                8i32,
                (&raw mut data).cast::<u8>(),
            );
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
                ),
                10i32,
                (&raw mut data).cast::<u8>(),
            );
            {
                i = 50i32;
                'l1: loop {
                    if !(i < 55i32) {
                        break 'l1;
                    }
                    'l2: {
                        SetMonData(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 100,
                            ),
                            i,
                            (&raw mut data).cast::<u8>(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 67i32;
                'l3: loop {
                    if !(i <= 79i32) {
                        break 'l3;
                    }
                    'l4: {
                        SetMonData(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 100,
                            ),
                            i,
                            (&raw mut data).cast::<u8>(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
                ),
                55i32,
                (&raw mut data).cast::<u8>(),
            );
            data = 255u32;
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
                ),
                64i32,
                (&raw mut data).cast::<u8>(),
            );
            CalculateMonStats(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) as isize * 100,
            ));
            CalculatePlayerPartyCount();
            GetSetPokedexFlag(
                SpeciesToNationalPokedexNum(
                    ((((((&raw mut gEvolutionTable).cast::<u8>())
                        .wrapping_offset(((preEvoSpecies) as i32) as isize * 40))
                    .cast::<u8>())
                    .wrapping_offset(8))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read(),
                ),
                2u8,
            );
            GetSetPokedexFlag(
                SpeciesToNationalPokedexNum(
                    ((((((&raw mut gEvolutionTable).cast::<u8>())
                        .wrapping_offset(((preEvoSpecies) as i32) as isize * 40))
                    .cast::<u8>())
                    .wrapping_offset(8))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read(),
                ),
                3u8,
            );
            if ((GetMonData2(shedinja, 11i32) == 303u32) && (GetMonData2(shedinja, 3i32) == 1u32))
                && (GetMonData2(mon, 11i32) == 302u32)
            {
                SetMonData(
                    shedinja,
                    2i32,
                    ((&raw const sText_ShedinjaJapaneseName)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_EvolutionScene(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var: u32 = 0u32;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32) as isize
                * 100,
        );
        if ((((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            == 2i32)
            && ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 8i32))
            && ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                    as i32) as isize
                    * 40,
            ))
            .wrapping_add(4))
            .read())
                != 0))
            && ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                & 1i32)
                != 0)
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(17i16);
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                    as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write(1i16);
            StopBgAnimation();
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_PkmnIsEvolving).cast::<u8>(),
                    );
                    BattlePutTextOnWindow((&raw mut gStringVar4).cast::<u8>(), 0u8);
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsTextPrinterActive(0u8)) != 0) {
                    EvoScene_DoMonAnimAndCry(
                        (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).read(),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u16),
                    );
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (EvoScene_IsMonAnimFinished(
                    (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).read(),
                )) != 0
                {
                    PlaySE(376u16);
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsSEPlaying()) != 0) {
                    PlayNewMapMusic(377u16);
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    BeginNormalPaletteFade(28u32, 4i8, 0u8, 16u8, 0u16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    StartBgAnimation(0u8);
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2))
                        .write(EvolutionSparkles_SpiralUpward(17u16));
                    let __p7 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(1u8);
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2))
                        .write(EvolutionSparkles_ArcDown());
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).write(
                        CycleEvolutionMonSprite(
                            (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .read(),
                            ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1))
                            .read(),
                        ),
                    );
                    let __p9 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if (({
                    let __p10 = (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3);
                    let __t11 = ((__p10).read()).wrapping_sub(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 0i32
                {
                    ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(3u8);
                    if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2))
                            .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(4))
                    .read())
                        != 0)
                    {
                        let __p12 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p12).write(((__p12).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2))
                    .write(EvolutionSparkles_CircleInward());
                let __p13 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).write(
                        EvolutionSparkles_SprayAndFlash(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as u16),
                        ),
                    );
                    let __p14 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    PlaySE(33u16);
                    let __p15 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (IsSEPlaying()) != 0 {
                    m4aMPlayAllStop();
                    crate::c::memcpy(
                        ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(32))
                        .cast::<u8>(),
                        (((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<u16>())
                        .cast::<u8>(),
                        96u32,
                    );
                    RestoreBgAfterAnim();
                    BeginNormalPaletteFade(28u32, 0i8, 16u8, 0u8, 0u16);
                    let __p16 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    EvoScene_DoMonAnimAndCry(
                        ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read(),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as u16),
                    );
                    let __p17 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p17).write(((__p17).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                if (IsCryFinished()) != 0 {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_CongratsPkmnEvolved).cast::<u8>(),
                    );
                    BattlePutTextOnWindow((&raw mut gStringVar4).cast::<u8>(), 0u8);
                    PlayBGM(371u16);
                    let __p18 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p18).write(((__p18).read()).wrapping_add(1));
                    SetMonData(
                        mon,
                        11i32,
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .cast::<u8>(),
                    );
                    CalculateMonStats(mon);
                    EvolutionRenameMon(
                        mon,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u16),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as u16),
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as u16),
                        ),
                        2u8,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as u16),
                        ),
                        3u8,
                    );
                    IncrementGameStat(14u8);
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                if !((IsTextPrinterActive(0u8)) != 0) {
                    var = ((MonTryLearningNewMove(
                        mon,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as u8),
                    )) as u32);
                    if (var != 0u32)
                        && (!((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(9))
                        .read())
                            != 0))
                    {
                        let mut nickname = crate::ffi::Align4([0u8; 20]);
                        if !((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as i32)
                            & 128i32)
                            != 0)
                        {
                            StopMapMusic();
                            Overworld_PlaySpecialMapMusic();
                        }
                        let __p19 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3);
                        (__p19).write((((((__p19).read()) as i32) | 128i32) as i16));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(0i16);
                        GetMonData3(mon, 2i32, (&raw mut nickname).cast::<u8>());
                        StringCopy_Nickname(
                            (&raw mut gBattleTextBuff1).cast::<u8>(),
                            (&raw mut nickname).cast::<u8>(),
                        );
                        if var == 65535u32 {
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(22i16);
                        } else {
                            if var == 65534u32 {
                                break 'l1;
                            } else {
                                (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(20i16);
                            }
                        }
                    } else {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        let __p20 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p20).write(((__p20).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if !((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        & 128i32)
                        != 0)
                    {
                        StopMapMusic();
                        Overworld_PlaySpecialMapMusic();
                    }
                    if !((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read())
                        != 0)
                    {
                        CreateShedinja(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as u16),
                            mon,
                        );
                    }
                    DestroyTask(taskId);
                    FreeMonSpritesGfx();
                    {
                        Free(((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    FreeAllWindowBuffers();
                    SetMainCallback2(
                        ((&raw mut gCB2_AfterEvolution)
                            .cast::<u8>()
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 17i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    m4aMPlayAllStop();
                    BeginNormalPaletteFade(393244u32, 0i8, 16u8, 0u8, 32767u16);
                    let __p21 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p21).write(((__p21).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    EvoScene_DoMonAnimAndCry(
                        (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).read(),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u16),
                    );
                    let __p22 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p22).write(((__p22).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 19i32 {
                if (EvoScene_IsMonAnimFinished(
                    (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).read(),
                )) != 0
                {
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read())
                        != 0
                    {
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            (&raw mut gText_EllipsisQuestionMark).cast::<u8>(),
                        );
                    } else {
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            (&raw mut gText_PkmnStoppedEvolving).cast::<u8>(),
                        );
                    }
                    BattlePutTextOnWindow((&raw mut gStringVar4).cast::<u8>(), 0u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .write(1i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(15i16);
                }
                break 'l1;
            }
            if __sw1 == 20i32 {
                if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                    BufferMoveToLearnIntoBattleTextBuff2();
                    PlayFanfare(367u16);
                    BattleStringExpandPlaceholdersToDisplayedString(
                        ((((&raw mut gBattleStringsTable).cast::<*mut u8>()).cast::<*mut u8>())
                            .wrapping_offset(3))
                        .read(),
                    );
                    BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 0u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(64i16);
                    let __p23 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p23).write(((__p23).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 21i32 {
                if ((!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)))
                    && ((({
                        let __p24 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4);
                        let __t25 = ((__p24).read()).wrapping_sub(1);
                        (__p24).write(__t25);
                        __t25
                    }) as i32)
                        == 0i32)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(15i16);
                }
                break 'l1;
            }
            if __sw1 == 22i32 {
                'l2: {
                    let __sw26 = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32);
                    let mut __fall = false;
                    if __sw26 == 0i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BufferMoveToLearnIntoBattleTextBuff2();
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(4))
                                .read(),
                            );
                            BattlePutTextOnWindow(
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                0u8,
                            );
                            let __p27 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p27).write(((__p27).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw26 == 1i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(5))
                                .read(),
                            );
                            BattlePutTextOnWindow(
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                0u8,
                            );
                            let __p28 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p28).write(((__p28).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw26 == 2i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(6))
                                .read(),
                            );
                            BattlePutTextOnWindow(
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                0u8,
                            );
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(7))
                            .write(5i16);
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(8))
                            .write(10i16);
                            let __p29 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p29).write(((__p29).read()).wrapping_add(1));
                        }
                    }
                    if __fall || __sw26 == 3i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            HandleBattleWindow(24u8, 8u8, 29u8, 13u8, 0u8);
                            BattlePutTextOnWindow(
                                (&raw mut gText_BattleYesNoChoice).cast::<u8>(),
                                12u8,
                            );
                            let __p30 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p30).write(((__p30).read()).wrapping_add(1));
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(1))
                                .write(0u8);
                            BattleCreateYesNoCursorAt(0u8);
                        }
                        break 'l2;
                    }
                    if __sw26 == 4i32 {
                        __fall = true;
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0)
                            && ((((((&raw mut gBattleCommunication).cast::<u8>())
                                .wrapping_offset(1))
                            .read()) as i32)
                                != 0i32)
                        {
                            PlaySE(5u16);
                            BattleDestroyYesNoCursorAt(
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(1))
                                    .read(),
                            );
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(1))
                                .write(0u8);
                            BattleCreateYesNoCursorAt(0u8);
                        }
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 128i32)
                            != 0)
                            && ((((((&raw mut gBattleCommunication).cast::<u8>())
                                .wrapping_offset(1))
                            .read()) as i32)
                                == 0i32)
                        {
                            PlaySE(5u16);
                            BattleDestroyYesNoCursorAt(
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(1))
                                    .read(),
                            );
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(1))
                                .write(1u8);
                            BattleCreateYesNoCursorAt(1u8);
                        }
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 1i32)
                            != 0
                        {
                            HandleBattleWindow(24u8, 8u8, 29u8, 13u8, 1u8);
                            PlaySE(5u16);
                            if (((((&raw mut gBattleCommunication).cast::<u8>())
                                .wrapping_offset(1))
                            .read()) as i32)
                                != 0i32
                            {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .write(
                                    ((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(8))
                                    .read(),
                                );
                            } else {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .write(
                                    ((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(7))
                                    .read(),
                                );
                                if ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .read()) as i32)
                                    == 5i32
                                {
                                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                                }
                            }
                        }
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            HandleBattleWindow(24u8, 8u8, 29u8, 13u8, 1u8);
                            PlaySE(5u16);
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .write(
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(8))
                                .read(),
                            );
                        }
                        break 'l2;
                    }
                    if __sw26 == 5i32 {
                        __fall = true;
                        if !((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                            7,
                            1,
                            false,
                        ) as u16)
                            != 0)
                        {
                            FreeAllWindowBuffers();
                            ShowSelectMovePokemonSummaryScreen(
                                (&raw mut gPlayerParty).cast::<u8>(),
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(10))
                                .read()) as u8),
                                ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                                    .wrapping_sub(1i32)) as u8),
                                Some(CB2_EvolutionSceneLoadGraphics),
                                ((&raw mut gMoveToLearn).cast::<u16>()).read(),
                            );
                            let __p31 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p31).write(((__p31).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw26 == 6i32 {
                        __fall = true;
                        if (!((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                            7,
                            1,
                            false,
                        ) as u16)
                            != 0))
                            && (core::mem::transmute::<_, usize>(
                                (((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(4)
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                .read(),
                            ) == (CB2_EvolutionSceneUpdate as *const () as usize))
                        {
                            var = ((GetMoveSlotToReplace()) as u32);
                            if var == 4u32 {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .write(10i16);
                            } else {
                                let mut r#move: u16 =
                                    ((GetMonData2(mon, (((var).wrapping_add(13u32)) as i32)))
                                        as u16);
                                if (IsHMMove2(r#move)) != 0 {
                                    BattleStringExpandPlaceholdersToDisplayedString(
                                        ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                            .cast::<*mut u8>())
                                        .wrapping_offset(307))
                                        .read(),
                                    );
                                    BattlePutTextOnWindow(
                                        (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                        0u8,
                                    );
                                    ((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(6))
                                    .write(12i16);
                                } else {
                                    {
                                        ((&raw mut gBattleTextBuff2).cast::<u8>()).write(253u8);
                                        (((&raw mut gBattleTextBuff2).cast::<u8>())
                                            .wrapping_offset(1))
                                        .write(2u8);
                                        (((&raw mut gBattleTextBuff2).cast::<u8>())
                                            .wrapping_offset(2))
                                        .write(((((r#move) as i32) & 255i32) as u8));
                                        (((&raw mut gBattleTextBuff2).cast::<u8>())
                                            .wrapping_offset(3))
                                        .write((((((r#move) as i32) & 65280i32) >> 8) as u8));
                                        (((&raw mut gBattleTextBuff2).cast::<u8>())
                                            .wrapping_offset(4))
                                        .write(255u8);
                                    }
                                    RemoveMonPPBonus(mon, ((var) as u8));
                                    SetMonMoveSlot(
                                        mon,
                                        ((&raw mut gMoveToLearn).cast::<u16>()).read(),
                                        ((var) as u8),
                                    );
                                    let __p32 = (((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(6);
                                    (__p32).write(((__p32).read()).wrapping_add(1));
                                }
                            }
                        }
                        break 'l2;
                    }
                    if __sw26 == 7i32 {
                        __fall = true;
                        BattleStringExpandPlaceholdersToDisplayedString(
                            ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(207))
                            .read(),
                        );
                        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 0u8);
                        let __p33 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6);
                        (__p33).write(((__p33).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw26 == 8i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(7))
                                .read(),
                            );
                            BattlePutTextOnWindow(
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                0u8,
                            );
                            let __p34 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p34).write(((__p34).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw26 == 9i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(208))
                                .read(),
                            );
                            BattlePutTextOnWindow(
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                0u8,
                            );
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(20i16);
                        }
                        break 'l2;
                    }
                    if __sw26 == 10i32 {
                        __fall = true;
                        BattleStringExpandPlaceholdersToDisplayedString(
                            ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(8))
                            .read(),
                        );
                        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 0u8);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(7))
                        .write(11i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(8))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(3i16);
                        break 'l2;
                    }
                    if __sw26 == 11i32 {
                        __fall = true;
                        BattleStringExpandPlaceholdersToDisplayedString(
                            ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(9))
                            .read(),
                        );
                        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 0u8);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(15i16);
                        break 'l2;
                    }
                    if __sw26 == 12i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .write(5i16);
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TradeEvolutionScene(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var: u32 = 0u32;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32) as isize
                * 100,
        );
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnIsEvolving).cast::<u8>(),
                );
                DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 1u8);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsTextPrinterActive(0u8)) != 0) {
                    PlayCry_Normal(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u16),
                        0i8,
                    );
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (IsCryFinished()) != 0 {
                    m4aSongNumStop(377u16);
                    PlaySE(376u16);
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsSEPlaying()) != 0) {
                    PlayBGM(377u16);
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    BeginNormalPaletteFade(28u32, 4i8, 0u8, 16u8, 0u16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    StartBgAnimation(1u8);
                    var = ((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        4,
                        4,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(16i32)) as u32);
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2))
                        .write(EvolutionSparkles_SpiralUpward(((var) as u16)));
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    SetGpuReg(14u8, 1539u16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    let __p7 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(1u8);
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2))
                        .write(EvolutionSparkles_ArcDown());
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).write(
                        CycleEvolutionMonSprite(
                            (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .read(),
                            ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1))
                            .read(),
                        ),
                    );
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (({
                    let __p9 = (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3);
                    let __t10 = ((__p9).read()).wrapping_sub(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    == 0i32
                {
                    ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(3u8);
                    if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2))
                            .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(4))
                    .read())
                        != 0)
                    {
                        let __p11 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2))
                    .write(EvolutionSparkles_CircleInward());
                let __p12 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).write(
                        EvolutionSparkles_SprayAndFlash_Trade(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as u16),
                        ),
                    );
                    let __p13 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    PlaySE(33u16);
                    let __p14 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if (IsSEPlaying()) != 0 {
                    Free(
                        (((&raw mut sBgAnimPal).cast::<u8>().cast::<*mut u16>()).read())
                            .cast::<u8>(),
                    );
                    EvoScene_DoMonAnimAndCry(
                        ((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read(),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as u16),
                    );
                    crate::c::memcpy(
                        ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(32))
                        .cast::<u8>(),
                        (((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<u16>())
                        .cast::<u8>(),
                        96u32,
                    );
                    let __p15 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (IsCryFinished()) != 0 {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_CongratsPkmnEvolved).cast::<u8>(),
                    );
                    DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 1u8);
                    PlayFanfare(371u16);
                    let __p16 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                    SetMonData(
                        mon,
                        11i32,
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .cast::<u8>(),
                    );
                    CalculateMonStats(mon);
                    EvolutionRenameMon(
                        mon,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u16),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as u16),
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as u16),
                        ),
                        2u8,
                    );
                    GetSetPokedexFlag(
                        SpeciesToNationalPokedexNum(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as u16),
                        ),
                        3u8,
                    );
                    IncrementGameStat(14u8);
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                if (!((IsTextPrinterActive(0u8)) != 0))
                    && (((IsFanfareTaskInactive()) as i32) == 1i32)
                {
                    var = ((MonTryLearningNewMove(
                        mon,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as u8),
                    )) as u32);
                    if (var != 0u32)
                        && (!((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(9))
                        .read())
                            != 0))
                    {
                        let mut nickname = crate::ffi::Align4([0u8; 20]);
                        let __p17 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3);
                        (__p17).write((((((__p17).read()) as i32) | 128i32) as i16));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(0i16);
                        GetMonData3(mon, 2i32, (&raw mut nickname).cast::<u8>());
                        StringCopy_Nickname(
                            (&raw mut gBattleTextBuff1).cast::<u8>(),
                            (&raw mut nickname).cast::<u8>(),
                        );
                        if var == 65535u32 {
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(20i16);
                        } else {
                            if var == 65534u32 {
                                break 'l1;
                            } else {
                                (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(18i16);
                            }
                        }
                    } else {
                        PlayBGM(377u16);
                        DrawTextOnTradeWindow(
                            0u8,
                            (&raw mut gText_CommunicationStandby5).cast::<u8>(),
                            1u8,
                        );
                        let __p18 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p18).write(((__p18).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                if !((IsTextPrinterActive(0u8)) != 0) {
                    DestroyTask(taskId);
                    {
                        Free(((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    crate::c::bf_write(
                        ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                        1,
                        1,
                        (0u8) as i32,
                    );
                    SetMainCallback2(
                        ((&raw mut gCB2_AfterEvolution)
                            .cast::<u8>()
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                if !((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(2)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0)
                {
                    m4aMPlayAllStop();
                    BeginNormalPaletteFade(
                        ((crate::c::shl_i32(
                            1i32,
                            ((((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                .wrapping_add(16i32)) as u32),
                        ) | 262172i32) as u32),
                        0i8,
                        16u8,
                        0u8,
                        32767u16,
                    );
                    let __p19 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p19).write(((__p19).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    EvoScene_DoMonAnimAndCry(
                        (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).read(),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u16),
                    );
                    let __p20 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p20).write(((__p20).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 17i32 {
                if (EvoScene_IsMonAnimFinished(
                    (((&raw mut sEvoStructPtr).cast::<u8>().cast::<*mut u8>()).read()).read(),
                )) != 0
                {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_EllipsisQuestionMark).cast::<u8>(),
                    );
                    DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 1u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .write(1i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(13i16);
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                    BufferMoveToLearnIntoBattleTextBuff2();
                    PlayFanfare(367u16);
                    BattleStringExpandPlaceholdersToDisplayedString(
                        ((((&raw mut gBattleStringsTable).cast::<*mut u8>()).cast::<*mut u8>())
                            .wrapping_offset(3))
                        .read(),
                    );
                    DrawTextOnTradeWindow(0u8, (&raw mut gDisplayedStringBattle).cast::<u8>(), 1u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(64i16);
                    let __p21 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p21).write(((__p21).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 19i32 {
                if ((!((IsTextPrinterActive(0u8)) != 0))
                    && (((IsFanfareTaskInactive()) as i32) == 1i32))
                    && ((({
                        let __p22 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4);
                        let __t23 = ((__p22).read()).wrapping_sub(1);
                        (__p22).write(__t23);
                        __t23
                    }) as i32)
                        == 0i32)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(13i16);
                }
                break 'l1;
            }
            if __sw1 == 20i32 {
                'l2: {
                    let __sw24 = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32);
                    let mut __fall = false;
                    if __sw24 == 0i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BufferMoveToLearnIntoBattleTextBuff2();
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(4))
                                .read(),
                            );
                            DrawTextOnTradeWindow(
                                0u8,
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                1u8,
                            );
                            let __p25 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p25).write(((__p25).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw24 == 1i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(5))
                                .read(),
                            );
                            DrawTextOnTradeWindow(
                                0u8,
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                1u8,
                            );
                            let __p26 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p26).write(((__p26).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw24 == 2i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(6))
                                .read(),
                            );
                            DrawTextOnTradeWindow(
                                0u8,
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                1u8,
                            );
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(7))
                            .write(5i16);
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(8))
                            .write(9i16);
                            let __p27 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p27).write(((__p27).read()).wrapping_add(1));
                        }
                    }
                    if __fall || __sw24 == 3i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            LoadUserWindowBorderGfx(0u8, 168u16, 224u8);
                            CreateYesNoMenu(
                                (&raw mut gTradeEvolutionSceneYesNoWindowTemplate).cast::<u8>(),
                                168u16,
                                14u8,
                                0u8,
                            );
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(1))
                                .write(0u8);
                            let __p28 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p28).write(((__p28).read()).wrapping_add(1));
                            (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(1))
                                .write(0u8);
                        }
                        break 'l2;
                    }
                    if __sw24 == 4i32 {
                        __fall = true;
                        'l3: {
                            let __sw29 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
                            if __sw29 == 0i32 {
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(1))
                                    .write(0u8);
                                BattleStringExpandPlaceholdersToDisplayedString(
                                    ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                        .cast::<*mut u8>())
                                    .wrapping_offset(292))
                                    .read(),
                                );
                                DrawTextOnTradeWindow(
                                    0u8,
                                    (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                    1u8,
                                );
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .write(
                                    ((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(7))
                                    .read(),
                                );
                                if ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .read()) as i32)
                                    == 5i32
                                {
                                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                                }
                                break 'l3;
                            }
                            if __sw29 == 1i32 || __sw29 == (-1i32) {
                                (((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(1))
                                    .write(1u8);
                                BattleStringExpandPlaceholdersToDisplayedString(
                                    ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                        .cast::<*mut u8>())
                                    .wrapping_offset(292))
                                    .read(),
                                );
                                DrawTextOnTradeWindow(
                                    0u8,
                                    (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                    1u8,
                                );
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .write(
                                    ((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(8))
                                    .read(),
                                );
                                break 'l3;
                            }
                        }
                        break 'l2;
                    }
                    if __sw24 == 5i32 {
                        __fall = true;
                        if !((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                            7,
                            1,
                            false,
                        ) as u16)
                            != 0)
                        {
                            if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                                DestroyWirelessStatusIndicatorSprite();
                            }
                            Free(GetBgTilemapBuffer(3u8));
                            Free(GetBgTilemapBuffer(1u8));
                            Free(GetBgTilemapBuffer(0u8));
                            FreeAllWindowBuffers();
                            ShowSelectMovePokemonSummaryScreen(
                                (&raw mut gPlayerParty).cast::<u8>(),
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(10))
                                .read()) as u8),
                                ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                                    .wrapping_sub(1i32)) as u8),
                                Some(CB2_TradeEvolutionSceneLoadGraphics),
                                ((&raw mut gMoveToLearn).cast::<u16>()).read(),
                            );
                            let __p30 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p30).write(((__p30).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw24 == 6i32 {
                        __fall = true;
                        if (!((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                            7,
                            1,
                            false,
                        ) as u16)
                            != 0))
                            && (core::mem::transmute::<_, usize>(
                                (((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(4)
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                .read(),
                            ) == (CB2_TradeEvolutionSceneUpdate as *const () as usize))
                        {
                            var = ((GetMoveSlotToReplace()) as u32);
                            if var == 4u32 {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .write(9i16);
                            } else {
                                let mut r#move: u16 =
                                    ((GetMonData2(mon, (((var).wrapping_add(13u32)) as i32)))
                                        as u16);
                                if (IsHMMove2(r#move)) != 0 {
                                    BattleStringExpandPlaceholdersToDisplayedString(
                                        ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                            .cast::<*mut u8>())
                                        .wrapping_offset(307))
                                        .read(),
                                    );
                                    DrawTextOnTradeWindow(
                                        0u8,
                                        (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                        1u8,
                                    );
                                    ((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(6))
                                    .write(11i16);
                                } else {
                                    {
                                        ((&raw mut gBattleTextBuff2).cast::<u8>()).write(253u8);
                                        (((&raw mut gBattleTextBuff2).cast::<u8>())
                                            .wrapping_offset(1))
                                        .write(2u8);
                                        (((&raw mut gBattleTextBuff2).cast::<u8>())
                                            .wrapping_offset(2))
                                        .write(((((r#move) as i32) & 255i32) as u8));
                                        (((&raw mut gBattleTextBuff2).cast::<u8>())
                                            .wrapping_offset(3))
                                        .write((((((r#move) as i32) & 65280i32) >> 8) as u8));
                                        (((&raw mut gBattleTextBuff2).cast::<u8>())
                                            .wrapping_offset(4))
                                        .write(255u8);
                                    }
                                    RemoveMonPPBonus(mon, ((var) as u8));
                                    SetMonMoveSlot(
                                        mon,
                                        ((&raw mut gMoveToLearn).cast::<u16>()).read(),
                                        ((var) as u8),
                                    );
                                    BattleStringExpandPlaceholdersToDisplayedString(
                                        ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                            .cast::<*mut u8>())
                                        .wrapping_offset(207))
                                        .read(),
                                    );
                                    DrawTextOnTradeWindow(
                                        0u8,
                                        (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                        1u8,
                                    );
                                    let __p31 = (((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(6);
                                    (__p31).write(((__p31).read()).wrapping_add(1));
                                }
                            }
                        }
                        break 'l2;
                    }
                    if __sw24 == 7i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(7))
                                .read(),
                            );
                            DrawTextOnTradeWindow(
                                0u8,
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                1u8,
                            );
                            let __p32 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6);
                            (__p32).write(((__p32).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                    if __sw24 == 8i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            BattleStringExpandPlaceholdersToDisplayedString(
                                ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(208))
                                .read(),
                            );
                            DrawTextOnTradeWindow(
                                0u8,
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                1u8,
                            );
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(18i16);
                        }
                        break 'l2;
                    }
                    if __sw24 == 9i32 {
                        __fall = true;
                        BattleStringExpandPlaceholdersToDisplayedString(
                            ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(8))
                            .read(),
                        );
                        DrawTextOnTradeWindow(
                            0u8,
                            (&raw mut gDisplayedStringBattle).cast::<u8>(),
                            1u8,
                        );
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(7))
                        .write(10i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(8))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(3i16);
                        break 'l2;
                    }
                    if __sw24 == 10i32 {
                        __fall = true;
                        BattleStringExpandPlaceholdersToDisplayedString(
                            ((((&raw mut gBattleStringsTable).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(9))
                            .read(),
                        );
                        DrawTextOnTradeWindow(
                            0u8,
                            (&raw mut gDisplayedStringBattle).cast::<u8>(),
                            1u8,
                        );
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(13i16);
                        break 'l2;
                    }
                    if __sw24 == 11i32 {
                        __fall = true;
                        if (!((IsTextPrinterActive(0u8)) != 0)) && (!((IsSEPlaying()) != 0)) {
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .write(5i16);
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EvoDummyFunc() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn VBlankCB_EvolutionScene() {
    unsafe {
        SetGpuReg(16u8, ((&raw mut gBattle_BG0_X).cast::<u16>()).read());
        SetGpuReg(18u8, ((&raw mut gBattle_BG0_Y).cast::<u16>()).read());
        SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
        SetGpuReg(24u8, ((&raw mut gBattle_BG2_X).cast::<u16>()).read());
        SetGpuReg(26u8, ((&raw mut gBattle_BG2_Y).cast::<u16>()).read());
        SetGpuReg(28u8, ((&raw mut gBattle_BG3_X).cast::<u16>()).read());
        SetGpuReg(30u8, ((&raw mut gBattle_BG3_Y).cast::<u16>()).read());
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_TradeEvolutionScene() {
    unsafe {
        SetGpuReg(16u8, ((&raw mut gBattle_BG0_X).cast::<u16>()).read());
        SetGpuReg(18u8, ((&raw mut gBattle_BG0_Y).cast::<u16>()).read());
        SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
        SetGpuReg(24u8, ((&raw mut gBattle_BG2_X).cast::<u16>()).read());
        SetGpuReg(26u8, ((&raw mut gBattle_BG2_Y).cast::<u16>()).read());
        SetGpuReg(28u8, ((&raw mut gBattle_BG3_X).cast::<u16>()).read());
        SetGpuReg(30u8, ((&raw mut gBattle_BG3_Y).cast::<u16>()).read());
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateBgPalette(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).wrapping_offset(6)).read()) != 0 {
            return;
        }
        if (({
            let __p1 = (data).wrapping_offset(5);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            < 20i32
        {
            return;
        }
        if (({
            let __t3 = (data).read();
            (data).write(((data).read()).wrapping_add(1));
            __t3
        }) as i32)
            > ((((((((&raw const sBgAnim_PaletteControl).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 4))
            .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
        {
            if ((((((((&raw const sBgAnim_PaletteControl).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 4))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                == ((((data).wrapping_offset(1)).read()) as i32)
            {
                let __p4 = (data).wrapping_offset(3);
                (__p4).write(((__p4).read()).wrapping_add(1));
                if ((((data).wrapping_offset(3)).read()) as i32)
                    == ((((((((&raw const sBgAnim_PaletteControl).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 4))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32)
                {
                    ((data).wrapping_offset(3)).write(0i16);
                    let __p5 = (data).wrapping_offset(2);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                ((data).wrapping_offset(1)).write(
                    (((((((&raw const sBgAnim_PaletteControl).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 4))
                    .cast::<u8>())
                    .read()) as i16),
                );
            } else {
                LoadPalette(
                    ((((&raw mut sBgAnimPal).cast::<u8>().cast::<*mut u16>()).read())
                        .wrapping_offset(
                            (((((data).wrapping_offset(1)).read()) as i32).wrapping_mul(16i32))
                                as isize,
                        ))
                    .cast::<u8>(),
                    160u16,
                    32u16,
                );
                (data).write(0i16);
                let __p6 = (data).wrapping_offset(1);
                (__p6).write(((__p6).read()).wrapping_add(1));
            }
        }
        if ((((data).wrapping_offset(2)).read()) as i32) == ((crate::c::div_u32(4u32, 1u32)) as i32)
        {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateBgAnimTask(isLink: u8) {
    unsafe {
        let mut isLink = isLink;
        let mut taskId: u8 = CreateTask(Some(Task_AnimateBg), 7u8);
        if !((isLink) != 0) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(1i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut outer_X: *mut u16 = core::ptr::null_mut();
        let mut outer_Y: *mut u16 = core::ptr::null_mut();
        let mut inner_X: *mut u16 = (&raw mut gBattle_BG1_X).cast::<u16>();
        let mut inner_Y: *mut u16 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        if !((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read())
            != 0)
        {
            outer_X = (&raw mut gBattle_BG2_X).cast::<u16>();
            outer_Y = (&raw mut gBattle_BG2_Y).cast::<u16>();
        } else {
            outer_X = (&raw mut gBattle_BG3_X).cast::<u16>();
            outer_Y = (&raw mut gBattle_BG3_Y).cast::<u16>();
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(
            (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(5i32)
                & 255i32) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(128i32)
                & 255i32) as i16),
        );
        (inner_X).write(
            ((((Cos(
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read(),
                4i16,
            )) as i32)
                .wrapping_add(8i32)) as u16),
        );
        (inner_Y).write(
            ((((Sin(
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read(),
                4i16,
            )) as i32)
                .wrapping_add(16i32)) as u16),
        );
        (outer_X).write(
            ((((Cos(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read(),
                4i16,
            )) as i32)
                .wrapping_add(8i32)) as u16),
        );
        (outer_Y).write(
            ((((Sin(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read(),
                4i16,
            )) as i32)
                .wrapping_add(16i32)) as u16),
        );
        if !((FuncIsActiveTask(Some(Task_UpdateBgPalette))) != 0) {
            DestroyTask(taskId);
            (inner_X).write(0u16);
            (inner_Y).write(0u16);
            (outer_X).write(256u16);
            (outer_Y).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn InitMovingBgPalette(palette: *mut u16) {
    unsafe {
        let mut palette = palette;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(800u32, 16u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 16i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((palette).wrapping_offset(
                                    (((i).wrapping_mul(16i32)).wrapping_add(j)) as isize,
                                ))
                                .write(
                                    ((((&raw const sBgAnim_Pal)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(
                                        ((((((((&raw const sBgAnim_PalIndexes)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartBgAnimation(isLink: u8) {
    unsafe {
        let mut isLink = isLink;
        let mut innerBgId: u8 = 0u8;
        let mut outerBgId: u8 = 0u8;
        ((&raw mut sBgAnimPal).cast::<u8>().cast::<*mut u16>())
            .write((AllocZeroed(1600u32)).cast::<u16>());
        InitMovingBgPalette(((&raw mut sBgAnimPal).cast::<u8>().cast::<*mut u16>()).read());
        if !((isLink) != 0) {
            innerBgId = 1u8;
            outerBgId = 2u8;
        } else {
            innerBgId = 1u8;
            outerBgId = 3u8;
        }
        LoadPalette(
            (((&raw const sBgAnim_Intro_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            160u16,
            32u16,
        );
        DecompressAndLoadBgGfxUsingHeap(
            1u8,
            (((&raw const sBgAnim_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        CopyToBgTilemapBuffer(
            innerBgId,
            (((&raw const sBgAnim_Inner_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u16,
            0u16,
        );
        CopyToBgTilemapBuffer(
            outerBgId,
            (((&raw const sBgAnim_Outer_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u16,
            0u16,
        );
        CopyBgTilemapBufferToVram(innerBgId);
        CopyBgTilemapBufferToVram(outerBgId);
        if !((isLink) != 0) {
            SetGpuReg(80u8, 1090u16);
            SetGpuReg(82u8, 2056u16);
            SetGpuReg(0u8, 5952u16);
            SetBgAttribute(innerBgId, 7u8, 2u8);
            SetBgAttribute(outerBgId, 7u8, 2u8);
            ShowBg(1u8);
            ShowBg(2u8);
        } else {
            SetGpuReg(80u8, 2114u16);
            SetGpuReg(82u8, 2056u16);
            SetGpuReg(0u8, 6976u16);
        }
        CreateTask(Some(Task_UpdateBgPalette), 5u8);
        CreateBgAnimTask(isLink);
    }
}
pub(crate) unsafe extern "C" fn PauseBgPaletteAnim() {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_UpdateBgPalette));
        if ((taskId) as i32) != 255i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(1i16);
        }
        FillPalette(0u16, 160u16, 32u16);
    }
}
pub(crate) unsafe extern "C" fn StopBgAnimation() {
    unsafe {
        let mut taskId: u8 = 0u8;
        if (({
            let __v1 = FindTaskIdByFunc(Some(Task_UpdateBgPalette));
            taskId = __v1;
            __v1
        }) as i32)
            != 255i32
        {
            DestroyTask(taskId);
        }
        if (({
            let __v2 = FindTaskIdByFunc(Some(Task_AnimateBg));
            taskId = __v2;
            __v2
        }) as i32)
            != 255i32
        {
            DestroyTask(taskId);
        }
        FillPalette(0u16, 160u16, 32u16);
        RestoreBgAfterAnim();
    }
}
pub(crate) unsafe extern "C" fn RestoreBgAfterAnim() {
    unsafe {
        SetGpuReg(80u8, 0u16);
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
        SetBgAttribute(1u8, 7u8, ((GetBattleBgTemplateData(1u8, 5u8)) as u8));
        SetBgAttribute(2u8, 7u8, ((GetBattleBgTemplateData(2u8, 5u8)) as u8));
        SetGpuReg(0u8, 6464u16);
        Free((((&raw mut sBgAnimPal).cast::<u8>().cast::<*mut u16>()).read()).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn EvoScene_DoMonAnimAndCry(monSpriteId: u8, speciesId: u16) {
    unsafe {
        let mut monSpriteId = monSpriteId;
        let mut speciesId = speciesId;
        DoMonFrontSpriteAnimation(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68),
            speciesId,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn EvoScene_IsMonAnimFinished(monSpriteId: u8) -> u32 {
    unsafe {
        let mut monSpriteId = monSpriteId;
        if core::mem::transmute::<_, usize>(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize)
        {
            return 1u32;
        }
        return 0u32;
    }
}
