//! Translated from `src/slot_machine.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sDigitalDisplayScenes sUnkPalette sSpecialDrawOdds sBiasSymbols sBiasesSpecial sBiasesRegular sDigitalDisplay_SpriteCoords sDigitalDisplay_SpriteCallbacks sSpriteTemplates_DigitalDisplay sSubspriteTables_DigitalDisplay sSpriteTemplate_PikaPowerBolt sSpriteTemplate_ReelTimeSmoke sSpriteTemplate_ReelTimeDuck sSpriteTemplate_ReelTimeExplosion sSpriteTemplate_ReelTimePikachuAura sReelTimeExplodeProbability sPokeballShiningPalTable sReelTimeSpeed_Probabilities sQuarterSpeed_ProbabilityBoost sSlotMatchFlags sSlotPayouts sReelBackground_Tilemap sReelTimeGfx sSlotMachineSpriteSheets sSlotMachineSpritePalettes sDigitalDisplay_Pal sInitialReelPositions sBiasProbabilities_Special sBiasProbabilities_Regular sReelTimeProbabilities_NormalGame sReelTimeProbabilities_LuckyGame sSymbolToMatch sReelTimeSymbols sReelSymbols sLitMatchLinePalTable sDarkMatchLinePalTable sMatchLinePalOffsets sBetToMatchLineIds sMatchLinesPerBet sFlashingLightsPalTable sSlotMachineMenu_Pal sReelTimeWindow_Tilemap sEmptyTilemap sDigitalDisplaySceneExitCallbacks sSpriteTemplate_ReelTimeBolt sSpriteTemplate_ReelTimeNumberGap sSpriteTemplate_ReelTimeShadow sSpriteTemplate_ReelTimeNumbers sSpriteTemplate_BrokenReelTimeMachine sSpriteTemplate_ReelTimeMachineAntennae sSpriteTemplate_ReelTimeMachine sSpriteTemplate_ReelBackground sSpriteTemplate_CoinNumber sSpriteTemplate_ReelSymbol sSpriteTemplate_ReelTimePikachu sSubspriteTable_ReelTimeNumberGap sSubspriteTable_ReelTimeShadow sSubspriteTable_BrokenReelTimeMachine sSubspriteTable_ReelTimeMachineAntennae sSubspriteTable_ReelTimeMachine sSubspriteTable_ReelBackground sBgTemplates sWindowTemplates sWindowTemplate_InfoBox sColors_ReeltimeHelp sSlotTasks sPayoutTasks sReelTasks sDecideStop_Bias sDecideStop_NoBias sReelStopShocks sDecideStop_Bias_Reel1_Bets sDecideStop_Bias_Reel2_Bets sDecideStop_Bias_Reel3_Bets sDecideStop_NoBias_Reel2_Bets sDecideStop_NoBias_Reel3_Bets sReelStopButtonTasks sReelButtonOffsets sPikaPowerBoltTasks sPikaPowerTileTable sReelTimeTasks sReelTimePikachuAnimIds sReelTimeBoltDelays sPikachuAuraFlashDelays sInfoBoxTasks sDigitalDisplayTasks sReelSymbols sReelTimeSymbols sInitialReelPositions sSpecialDrawOdds sBiasProbabilities_Special sBiasProbabilities_Regular sReelTimeProbabilities_NormalGame sReelTimeProbabilities_LuckyGame sReelTimeExplodeProbability sReelTimeSpeed_Probabilities sQuarterSpeed_ProbabilityBoost sBiasSymbols sBiasesSpecial sBiasesRegular sSymbolToMatch sSlotMatchFlags sSlotPayouts sDigitalDisplay_SpriteCoords sDigitalDisplay_SpriteCallbacks sDigitalDisplay_InsertBet sDigitalDisplay_StopReel sDigitalDisplay_Win sDigitalDisplay_Lose sDigitalDisplay_ReelTime sDigitalDisplay_BonusBig sDigitalDisplay_BonusRegular sDigitalDisplayScenes sDigitalDisplaySceneExitCallbacks sOam_8x8 sOam_8x16 sOam_16x16 sOam_16x32 sOam_32x32 sOam_32x64 sOam_64x32 sOam_64x64 sImageTable_ReelTimeNumbers sImageTable_ReelTimeShadow sImageTable_ReelTimeNumberGap sImageTable_ReelTimeBolt sImageTable_ReelTimePikachuAura sImageTable_ReelTimeExplosion sImageTable_ReelTimeDuck sImageTable_ReelTimeSmoke sImageTable_PikaPowerBolt sAnim_SingleFrame sAnim_ReelTimeDuck sAnim_ReelTimePikachu_Still sAnim_ReelTimePikachu_ChargingSlow sAnim_ReelTimePikachu_ChargingMedium sAnim_ReelTimePikachu_ChargingFast sAnim_ReelTimePikachu_Cheering sAnim_ReelTimePikachu_FellOver sAnim_ReelTimeNumber_0 sAnim_ReelTimeNumber_1 sAnim_ReelTimeNumber_2 sAnim_ReelTimeNumber_3 sAnim_ReelTimeNumber_4 sAnim_ReelTimeNumber_5 sAnim_ReelTimeBolt sAnim_ReelTimeExplosion sAnim_DigitalDisplay_AButton_Flashing sAnim_DigitalDisplay_AButton_Static sAnim_DigitalDisplay_DPad_Flashing sAnim_DigitalDisplay_Pokeball_Rocking sAnim_DigitalDisplay_Pokeball_Static sAnim_DigitalDisplay_Number_1 sAnim_DigitalDisplay_Number_2 sAnim_DigitalDisplay_Number_3 sAnim_DigitalDisplay_Number_4 sAnim_DigitalDisplay_Number_5 sAnims_SingleFrame sAnims_ReelTimeDuck sAnims_ReelTimePikachu sAnims_ReelTimeNumbers sAnims_ReelTimeBolt sAnims_ReelTimeExplosion sAnims_DigitalDisplay_AButton sAnims_DigitalDisplay_DPad sAnims_DigitalDisplay_Pokeball sAnims_DigitalDisplay_Number sAffineAnim_ReelTimeSmoke sAffineAnims_ReelTimeSmoke sAffineAnim_PikaPowerBolt sAffineAnims_PikaPowerBolt sSpriteTemplate_ReelSymbol sSpriteTemplate_CoinNumber sSpriteTemplate_ReelBackground sSpriteTemplate_ReelTimePikachu sSpriteTemplate_ReelTimeMachineAntennae sSpriteTemplate_ReelTimeMachine sSpriteTemplate_BrokenReelTimeMachine sSpriteTemplate_ReelTimeNumbers sSpriteTemplate_ReelTimeShadow sSpriteTemplate_ReelTimeNumberGap sSpriteTemplate_ReelTimeBolt sSpriteTemplate_ReelTimePikachuAura sSpriteTemplate_ReelTimeExplosion sSpriteTemplate_ReelTimeDuck sSpriteTemplate_ReelTimeSmoke sSpriteTemplate_DigitalDisplay_Reel sSpriteTemplate_DigitalDisplay_Time sSpriteTemplate_DigitalDisplay_Insert sSpriteTemplate_DigitalDisplay_Stop sSpriteTemplate_DigitalDisplay_Win sSpriteTemplate_DigitalDisplay_Lose sSpriteTemplate_DigitalDisplay_Bonus sSpriteTemplate_DigitalDisplay_Big sSpriteTemplate_DigitalDisplay_Reg sSpriteTemplate_DigitalDisplay_AButton sSpriteTemplate_DigitalDisplay_Smoke sSpriteTemplate_DigitalDisplay_Number sSpriteTemplate_DigitalDisplay_Pokeball sSpriteTemplate_DigitalDisplay_DPad sSpriteTemplate_PikaPowerBolt sSubsprites_ReelBackground sSubspriteTable_ReelBackground sSubsprites_ReelTimeMachineAntennae sSubspriteTable_ReelTimeMachineAntennae sSubsprites_ReelTimeMachine sSubspriteTable_ReelTimeMachine sSubsprites_BrokenReelTimeMachine sSubspriteTable_BrokenReelTimeMachine sSubsprites_ReelTimeShadow sSubspriteTable_ReelTimeShadow sSubsprites_ReelTimeNumberGap sSubspriteTable_ReelTimeNumberGap sSubsprites_DigitalDisplay_Reel sSubspriteTable_DigitalDisplay_Reel sSubsprites_DigitalDisplay_Time sSubspriteTable_DigitalDisplay_Time sSubsprites_DigitalDisplay_Insert sSubspriteTable_DigitalDisplay_Insert sSubsprites_DigitalDisplay_Unused1 sSubspriteTable_DigitalDisplay_Unused1 sSubsprites_DigitalDisplay_Win sSubspriteTable_DigitalDisplay_Win sSubsprites_DigitalDisplay_SmokeBig sSubsprites_DigitalDisplay_SmokeSmall sSubspriteTable_DigitalDisplay_Smoke sSubsprites_DigitalDisplay_Pokeball sSubspriteTable_DigitalDisplay_Pokeball sSubsprites_DigitalDisplay_DPad sSubspriteTable_DigitalDisplay_DPad sSubsprites_DigitalDisplay_StopS sSubspriteTable_DigitalDisplay_StopS sSubsprites_DigitalDisplay_StopT sSubspriteTable_DigitalDisplay_StopT sSubsprites_DigitalDisplay_StopO sSubspriteTable_DigitalDisplay_StopO sSubsprites_DigitalDisplay_StopP sSubspriteTable_DigitalDisplay_StopP sSubsprites_DigitalDisplay_BonusB sSubspriteTable_DigitalDisplay_BonusB sSubsprites_DigitalDisplay_BonusO sSubspriteTable_DigitalDisplay_BonusO sSubsprites_DigitalDisplay_BonusN sSubspriteTable_DigitalDisplay_BonusN sSubsprites_DigitalDisplay_BonusU sSubspriteTable_DigitalDisplay_BonusU sSubsprites_DigitalDisplay_BonusS sSubspriteTable_DigitalDisplay_BonusS sSubsprites_DigitalDisplay_BigB sSubspriteTable_DigitalDisplay_BigB sSubsprites_DigitalDisplay_BigI sSubspriteTable_DigitalDisplay_BigI sSubsprites_DigitalDisplay_BigG sSubspriteTable_DigitalDisplay_BigG sSubsprites_DigitalDisplay_RegR sSubspriteTable_DigitalDisplay_RegR sSubsprites_DigitalDisplay_RegE sSubspriteTable_DigitalDisplay_RegE sSubsprites_DigitalDisplay_RegG sSubspriteTable_DigitalDisplay_RegG sSpriteTemplates_DigitalDisplay sSubspriteTables_DigitalDisplay sSlotMachineSpriteSheets sReelBackground_Tilemap sUnusedColors sMiddleRowLit_Pal sTopRowLit_Pal sBottomRowt_Pal sNWSEDiagLit_Pal sNESWDiagLit_Pal sLitMatchLinePalTable sDarkMatchLinePalTable sMatchLinePalOffsets sBetToMatchLineIds sMatchLinesPerBet sFlashingLightsInside_Pal sFlashingLightsMiddle_Pal sFlashingLightsOutside_Pal sFlashingLightsPalTable sSlotMachineMenu_Pal sPokeballShining0_Pal sPokeballShining1_Pal sPokeballShining2_Pal sPokeballShiningPalTable sDigitalDisplay_Pal sUnkPalette sSlotMachineSpritePalettes sReelTimeGfx sReelTimeWindow_Tilemap sEmptyTilemap
#[allow(unused_imports)]
use crate::data::slot_machine::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenuGfx: *mut u16 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectedPikaPowerTile: *mut u16 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelOverlay_Tilemap: *mut u16 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDigitalDisplayGfxPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelTimeGfxPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelButtonPress_Tilemap: *mut u16 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelBackground_Gfx: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_ReelTimePikachu: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_ReelTimeMachineAntennae: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_ReelTimeMachine: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_BrokenReelTimeMachine: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Reel: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Time: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Insert: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Stop: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Win: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Lose: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Bonus: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Big: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Reg: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_AButton: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Smoke: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Number: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Pokeball: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_DPad: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelBackgroundSpriteSheet: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlotMachineSpritesheetsPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlotMachine: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sImageTables_DigitalDisplay: crate::ffi::Align4<[u8; 104]> =
    crate::ffi::Align4([0; 104]);

unsafe extern "C" {
    static mut gMain: u8;
    static mut gOamLimit: u8;
    static mut gPaletteFade: u8;
    static mut gSlotMachineDigitalDisplay_Gfx: u8;
    static mut gSlotMachineInfoBox_Tilemap: u8;
    static mut gSlotMachineMenu_Gfx: u8;
    static mut gSlotMachineMenu_Pal: u8;
    static mut gSlotMachineMenu_Tilemap: u8;
    static mut gSpriteCoordOffsetX: u8;
    static mut gSpriteCoordOffsetY: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_QuitTheGame: u8;
    static mut gText_ReelTimeHelp: u8;
    static mut gText_YouDontHaveThreeCoins: u8;
    static mut gText_YouveGot9999Coins: u8;
    static mut gText_YouveRunOutOfCoins: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn AlertTVThatPlayerPlayedSlotMachine(a0: u16);
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateInvisibleSprite(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenuParameterized(a0: u8, a1: u8, a2: u16, a3: u16, a4: u8, a5: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn EnableInterrupts(a0: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeOamMatrix(a0: u8);
    fn GetCoins() -> u16;
    fn GetCurrentMapMusic() -> u16;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn HideBg(a0: u8);
    fn IncrementDailySlotsUses();
    fn IncrementGameStat(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsFanfareTaskInactive() -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTilemap(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadSpriteSheets(a0: *mut u8);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadWordFromTwoHalfwords(a0: *mut u16, a1: *mut u32);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MultiplyInvertedPaletteRGBComponents(a0: u16, a1: u8, a2: u8, a3: u8);
    fn MultiplyPaletteRGBComponents(a0: u16, a1: u8, a2: u8, a3: u8);
    fn PlayFanfare(a0: u16);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn SetCoins(a0: u16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetSpriteSheetFrameTileNum(a0: *mut u8);
    fn SetSubspriteTables(a0: *mut u8, a1: *mut u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut u8, a1: u8);
    fn StopMapMusic();
    fn StoreWordInTwoHalfwords(a0: *mut u16, a1: u32);
    fn TransferPlttBuffer();
    fn TryPutFindThatGamerOnAir(a0: u16);
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn Task_FadeToSlotMachine(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
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
                    SetMainCallback2(Some(CB2_SlotMachineSetup));
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySlotMachine(
    machineId: u8,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut machineId = machineId;
        let mut exitCallback = exitCallback;
        let mut taskId: u8 = 0u8;
        ((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(104u32));
        PlaySlotMachine_Internal(machineId, exitCallback);
        taskId = CreateTask(Some(Task_FadeToSlotMachine), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn CB2_SlotMachineSetup() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                SlotMachineSetup_InitBgsWindows();
                InitSlotMachine();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                SlotMachineSetup_InitVRAM();
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                SlotMachineSetup_InitOAM();
                SlotMachineSetup_InitGpuRegs();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SlotMachineSetup_InitPalsSpritesTasks();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                SlotMachineSetup_InitTilemaps();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                SlotMachineSetup_LoadGfxAndTilemaps();
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                SlotMachineSetup_InitVBlank();
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                AllocDigitalDisplayGfx();
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                SetDigitalDisplayImagePtrs();
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                CreateSlotMachineSprites();
                CreateGameplayTasks();
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                SetMainCallback2(Some(CB2_SlotMachine));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_SlotMachine() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn SlotMachine_VBlankCB() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        SetGpuReg(
            64u8,
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(88)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(
            68u8,
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(90)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(
            72u8,
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(
            74u8,
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(94)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn PlaySlotMachine_Internal(
    machineId: u8,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut machineId = machineId;
        let mut exitCallback = exitCallback;
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((CreateTask(Some(SlotMachineDummyTask), 255u8)) as i32) as isize * 40,
        );
        (((task).wrapping_add(8)).cast::<i16>()).write(((machineId) as i16));
        StoreWordInTwoHalfwords(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).cast::<u16>(),
            ((core::mem::transmute::<_, usize>(exitCallback) as i32) as u32),
        );
    }
}
pub(crate) unsafe extern "C" fn SlotMachine_InitFromTask() {
    unsafe {
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((FindTaskIdByFunc(Some(SlotMachineDummyTask))) as i32) as isize * 40);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write((((((task).wrapping_add(8)).cast::<i16>()).read()) as u8));
        LoadWordFromTwoHalfwords(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).cast::<u16>(),
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(100)
                .cast::<Option<unsafe extern "C" fn()>>())
            .cast::<u32>(),
        );
    }
}
pub(crate) unsafe extern "C" fn SlotMachineDummyTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitBgsWindows() {
    unsafe {
        SetVBlankCallback(None);
        SetHBlankCallback(None);
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
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitVBlank() {
    unsafe {
        SetVBlankCallback(Some(SlotMachine_VBlankCB));
        EnableInterrupts(1u16);
        SetGpuReg(0u8, 12352u16);
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitVRAM() {
    unsafe {
        {
            let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u16).cast::<u8>();
            let mut _size: u32 = 65536u32;
            'l1: loop {
                if !((1i32) != 0) {
                    break 'l1;
                }
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l4: loop {
                                'l5: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(
                                            dmaRegs,
                                            ((&raw mut tmp) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2130706432i32)
                                                | crate::c::div_i32(
                                                    4096i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ))
                                                as u32),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
                if _size <= 4096u32 {
                    'l6: loop {
                        'l7: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l8: loop {
                                    'l9: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitOAM() {
    unsafe {
        'l1: loop {
            'l2: {
                {
                    let mut _dest: *mut u16 = ((117440512i32) as usize as *mut u16);
                    let mut _size: u32 = 1024u32;
                    'l3: loop {
                        'l4: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l5: loop {
                                    'l6: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l5;
                                    }
                                }
                            }
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
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitGpuRegs() {
    unsafe {
        SetGpuReg(8u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(14u8, 0u16);
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        SetGpuReg(28u8, 0u16);
        SetGpuReg(30u8, 0u16);
        SetGpuReg(72u8, 63u16);
        SetGpuReg(74u8, 63u16);
        SetGpuReg(80u8, 4168u16);
        SetGpuReg(82u8, 2057u16);
    }
}
pub(crate) unsafe extern "C" fn InitSlotMachine() {
    unsafe {
        let mut i: u8 = 0u8;
        SlotMachine_InitFromTask();
        (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .write(0u8);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
            .write(((((Random()) as i32) & 1i32) as u8));
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(0u8);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .write(0u8);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(11))
            .write(0u8);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<i16>())
        .write(((GetCoins()) as i16));
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26)
            .cast::<i16>())
        .write(8i16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(88)
            .cast::<u16>())
        .write(240u16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(90)
            .cast::<u16>())
        .write(160u16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(92)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(94)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(96)
            .cast::<u16>())
        .write(GetCurrentMapMusic());
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(34))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<i16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((crate::c::rem_i32(
                            ((((((((&raw const sInitialReelPositions).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<i16>())
                            .wrapping_offset(
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(3))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32),
                            21i32,
                        )) as i16),
                    );
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(28))
                    .cast::<i16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((504i32).wrapping_sub(
                            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(40))
                            .cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                .wrapping_mul(24i32),
                        )) as i16),
                    );
                    let __p1 = (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(28))
                    .cast::<i16>())
                    .wrapping_offset(((i) as i32) as isize);
                    (__p1).write(((crate::c::rem_i32((((__p1).read()) as i32), 504i32)) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
        AlertTVThatPlayerPlayedSlotMachine(GetCoins());
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitPalsSpritesTasks() {
    unsafe {
        ResetPaletteFade();
        ResetSpriteData();
        ((&raw mut gOamLimit).cast::<u8>()).write(128u8);
        FreeAllSpritePalettes();
        ResetTasks();
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitTilemaps() {
    unsafe {
        ((&raw mut sSelectedPikaPowerTile)
            .cast::<u8>()
            .cast::<*mut u16>())
        .write((Alloc(8u32)).cast::<u16>());
        ((&raw mut sReelOverlay_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .write((AllocZeroed(14u32)).cast::<u16>());
        ((&raw mut sReelButtonPress_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .write((AllocZeroed(8u32)).cast::<u16>());
        (((&raw mut sReelOverlay_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .write(8273u16);
        ((((&raw mut sReelOverlay_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(1))
        .write(10321u16);
        ((((&raw mut sReelOverlay_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(2))
        .write(8289u16);
        ((((&raw mut sReelOverlay_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(3))
        .write(10337u16);
        ((((&raw mut sReelOverlay_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(4))
        .write(8382u16);
        ((((&raw mut sReelOverlay_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(5))
        .write(10430u16);
        ((((&raw mut sReelOverlay_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(6))
        .write(8383u16);
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_LoadGfxAndTilemaps() {
    unsafe {
        LoadMenuGfx();
        LoadMenuAndReelOverlayTilemaps();
        LoadSlotMachineGfx();
        LoadMessageBoxGfx(0u8, 512u16, 240u8);
        LoadUserWindowBorderGfx(0u8, 532u16, 224u8);
        PutWindowTilemap(0u8);
    }
}
pub(crate) unsafe extern "C" fn CreateSlotMachineSprites() {
    unsafe {
        CreateReelSymbolSprites();
        CreateCreditPayoutNumberSprites();
        CreateInvisibleFlashMatchLineSprites();
        CreateReelBackgroundSprite();
    }
}
pub(crate) unsafe extern "C" fn CreateGameplayTasks() {
    unsafe {
        CreatePikaPowerBoltTask();
        CreateReelTasks();
        CreateDigitalDisplayTask();
        CreateSlotMachineTasks();
    }
}
pub(crate) unsafe extern "C" fn CreateSlotMachineTasks() {
    unsafe {
        Task_SlotMachine(CreateTask(Some(Task_SlotMachine), 0u8));
    }
}
pub(crate) unsafe extern "C" fn Task_SlotMachine(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sSlotTasks)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                    as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SlotTask_UnfadeScreen(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        LoadPikaPowerMeter(
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .read(),
        );
        let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read());
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_WaitUnfade(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read());
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_ReadyNewSpin(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<i16>())
        .write(0i16);
        let __p1 =
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
        (__p1).write((((((__p1).read()) as i32) & 192i32) as u8));
        (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(4u8);
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<i16>())
        .read()) as i32)
            <= 0i32
        {
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(25u8);
        } else {
            if (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10))
            .read())
                != 0
            {
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(3u8);
                CreateDigitalDisplayScene(4u8);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_ReadyNewReelTimeSpin(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (IsDigitalDisplayAnimFinished()) != 0 {
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(4u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_AskInsertBet(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        CreateDigitalDisplayScene(0u8);
        (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(5u8);
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<i16>())
        .read()) as i32)
            >= 9999i32
        {
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(23u8);
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_HandleBetInput(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: i16 = 0i16;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 4i32)
            != 0
        {
            OpenInfoBox(0u8);
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(8u8);
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 256i32)
                != 0
            {
                if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(
                        (3i32).wrapping_sub(
                            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(18)
                                .cast::<i16>())
                            .read()) as i32),
                        ),
                    )
                    >= 0i32
                {
                    {
                        i = ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<i16>())
                        .read();
                        'l1: loop {
                            if !(((i) as i32) < 3i32) {
                                break 'l1;
                            }
                            'l2: {
                                LightenBetTiles(((i) as u8));
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<i16>();
                    (__p1).write(
                        (((((__p1).read()) as i32).wrapping_sub(
                            (3i32).wrapping_sub(
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(18)
                                .cast::<i16>())
                                .read()) as i32),
                            ),
                        )) as i16),
                    );
                    ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<i16>())
                    .write(3i16);
                    (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(9u8);
                    PlaySE(95u16);
                } else {
                    (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(6u8);
                }
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0)
                    && (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<i16>())
                    .read()) as i32)
                        != 0i32)
                {
                    PlaySE(95u16);
                    LightenBetTiles(
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<i16>())
                        .read()) as u8),
                    );
                    let __p2 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                    let __p3 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<i16>())
                .read()) as i32)
                    >= 3i32)
                    || ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<i16>())
                    .read()) as i32)
                        != 0i32)
                        && (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 1i32)
                            != 0))
                {
                    (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(9u8);
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(21u8);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_PrintMsg_Need3Coins(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        DrawDialogueFrame(0u8, 0u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_YouDontHaveThreeCoins).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        CopyWindowToVram(0u8, 3u8);
        (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(7u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_WaitMsg_Need3Coins(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            ClearDialogWindowAndFrame(0u8, 1u8);
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(5u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_WaitInfoBox(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (IsInfoBoxClosed()) != 0 {
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(5u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_StartSpin(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        DrawMachineBias();
        DestroyDigitalDisplayScene();
        SpinSlotReel(0u8);
        SpinSlotReel(1u8);
        SpinSlotReel(2u8);
        IncrementDailySlotsUses();
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .read()) as i32)
            & 32i32)
            != 0
        {
            BeginReelTime();
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(10u8);
        } else {
            CreateDigitalDisplayScene(1u8);
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(11u8);
        }
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26)
            .cast::<i16>())
        .write(8i16);
        if (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .read())
            != 0
        {
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(26)
                .cast::<i16>())
            .write(((ReelTimeSpeed()) as i16));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_StartReelTimeSpin(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (IsReelTimeTaskDone()) != 0 {
            CreateDigitalDisplayScene(1u8);
            let __p1 =
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
            (__p1).write((((((__p1).read()) as i32) & (-33i32)) as u8));
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(11u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_ResetBiasFailure(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 30i32
        {
            ResetBiasFailure();
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(12u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_WaitReelStop(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(24u16);
            StopSlotReel(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<i16>())
                .read()) as u8),
            );
            PressStopReelButton(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<i16>())
                .read()) as u8),
            );
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(13u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_WaitAllReelsStop(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((IsSlotReelMoving(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<i16>())
            .read()) as u8),
        )) != 0)
        {
            let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(12u8);
            if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<i16>())
            .read()) as i32)
                >= 3i32
            {
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(14u8);
            }
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_CheckMatches(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let __p1 =
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
        (__p1).write((((((__p1).read()) as i32) & 192i32) as u8));
        CheckMatch();
        if (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .read())
            != 0
        {
            let __p2 =
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10);
            (__p2).write(((__p2).read()).wrapping_sub(1));
            let __p3 =
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(11);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .read())
            != 0
        {
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(15u8);
            AwardPayout();
            FlashSlotMachineLights();
            if (({
                let __p4 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<i16>();
                let __v5 = (((((__p4).read()) as i32).wrapping_sub(
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14)
                        .cast::<i16>())
                    .read()) as i32),
                )) as i16);
                (__p4).write(__v5);
                __v5
            }) as i32)
                < 0i32
            {
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<i16>())
                .write(0i16);
            }
            if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                & 384i32)
                != 0
            {
                PlayFanfare(389u16);
                CreateDigitalDisplayScene(6u8);
            } else {
                if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    PlayFanfare(389u16);
                    CreateDigitalDisplayScene(5u8);
                } else {
                    PlayFanfare(390u16);
                    CreateDigitalDisplayScene(2u8);
                }
            }
            if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                & 448i32)
                != 0
            {
                let __p6 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4);
                (__p6).write((((((__p6).read()) as i32) & (-193i32)) as u8));
                if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>())
                .read()) as i32)
                    & 384i32)
                    != 0
                {
                    ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .write(0u8);
                    ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11))
                    .write(0u8);
                    ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(0u8);
                    if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>())
                    .read()) as i32)
                        & 256i32)
                        != 0
                    {
                        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3))
                        .write(1u8);
                    }
                }
            }
            if ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                & 32i32)
                != 0)
                && (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32)
                    < 16i32)
            {
                let __p7 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2);
                (__p7).write(((__p7).read()).wrapping_add(1));
                AddPikaPowerBolt(
                    ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .read(),
                );
            }
        } else {
            CreateDigitalDisplayScene(3u8);
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(20u8);
            if (({
                let __p8 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<i16>();
                let __v9 = (((((__p8).read()) as i32).wrapping_add(
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<i16>())
                    .read()) as i32),
                )) as i16);
                (__p8).write(__v9);
                __v9
            }) as i32)
                > 9999i32
            {
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<i16>())
                .write(9999i16);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_WaitPayout(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (IsFinalTask_Task_Payout()) != 0 {
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(16u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_EndPayout(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (TryStopSlotMachineLights()) != 0 {
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(19u8);
            if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                & 384i32)
                != 0
            {
                IncrementGameStat(28u8);
            }
            if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                & 4i32)
                != 0
            {
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<i16>())
                .write(0i16);
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(9u8);
            }
            if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                & 32i32)
                != 0
            {
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(17u8);
            }
            if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10))
            .read())
                != 0)
                && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>())
                .read()) as i32)
                    & 4i32)
                    != 0)
            {
                CreateDigitalDisplayScene(4u8);
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(18u8);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_MatchedPower(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((IsPikaPowerBoltAnimating()) != 0) {
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(19u8);
            if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                & 4i32)
                != 0
            {
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(9u8);
                if (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10))
                .read())
                    != 0
                {
                    CreateDigitalDisplayScene(4u8);
                    (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(18u8);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_WaitReelTimeAnim(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (IsDigitalDisplayAnimFinished()) != 0 {
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(19u8);
            if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                & 4i32)
                != 0
            {
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(9u8);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_ResetBetTiles(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        DarkenBetTiles(0u8);
        DarkenBetTiles(1u8);
        DarkenBetTiles(2u8);
        (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_NoMatches(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 64i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(19u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_AskQuit(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        DrawDialogueFrame(0u8, 0u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_QuitTheGame).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        CopyWindowToVram(0u8, 3u8);
        CreateYesNoMenuParameterized(21u8, 7u8, 532u16, 384u16, 14u8, 15u8);
        (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(22u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_HandleQuitInput(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut input: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        if ((input) as i32) == 0i32 {
            ClearDialogWindowAndFrame(0u8, 1u8);
            DarkenBetTiles(0u8);
            DarkenBetTiles(1u8);
            DarkenBetTiles(2u8);
            let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<i16>())
                    .read()) as i32),
                )) as i16),
            );
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(27u8);
        } else {
            if (((input) as i32) == 1i32) || (((input) as i32) == (-1i32)) {
                ClearDialogWindowAndFrame(0u8, 1u8);
                (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(5u8);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_PrintMsg_MaxCoins(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        DrawDialogueFrame(0u8, 0u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_YouveGot9999Coins).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        CopyWindowToVram(0u8, 3u8);
        (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(24u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_WaitMsg_MaxCoins(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            ClearDialogWindowAndFrame(0u8, 1u8);
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(5u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_PrintMsg_NoMoreCoins(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        DrawDialogueFrame(0u8, 0u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_YouveRunOutOfCoins).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        CopyWindowToVram(0u8, 3u8);
        (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(26u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_WaitMsg_NoMoreCoins(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            ClearDialogWindowAndFrame(0u8, 1u8);
            (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).write(27u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_EndGame(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        SetCoins(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<i16>())
            .read()) as u16),
        );
        TryPutFindThatGamerOnAir(GetCoins());
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read());
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlotTask_FreeDataStructures(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetMainCallback2(
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Reel)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Reel)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Time)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Time)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Insert)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Insert)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Stop)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Stop)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Win)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Win)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Lose)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Lose)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Bonus)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Bonus)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Big)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Big)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Reg)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Reg)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_AButton)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_AButton)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Smoke)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Smoke)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Number)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Number)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_Pokeball)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_Pokeball)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sImageTable_DigitalDisplay_DPad)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_DigitalDisplay_DPad)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            if ((((&raw mut sImageTable_ReelTimePikachu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read()) as usize)
                != 0usize
            {
                Free(
                    ((&raw mut sImageTable_ReelTimePikachu)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_ReelTimePikachu)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            if ((((&raw mut sImageTable_ReelTimeMachineAntennae)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read()) as usize)
                != 0usize
            {
                Free(
                    ((&raw mut sImageTable_ReelTimeMachineAntennae)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_ReelTimeMachineAntennae)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            if ((((&raw mut sImageTable_ReelTimeMachine)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read()) as usize)
                != 0usize
            {
                Free(
                    ((&raw mut sImageTable_ReelTimeMachine)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_ReelTimeMachine)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            if ((((&raw mut sImageTable_BrokenReelTimeMachine)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read()) as usize)
                != 0usize
            {
                Free(
                    ((&raw mut sImageTable_BrokenReelTimeMachine)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sImageTable_BrokenReelTimeMachine)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free((((&raw mut sMenuGfx).cast::<u8>().cast::<*mut u16>()).read()).cast::<u8>());
                ((&raw mut sMenuGfx).cast::<u8>().cast::<*mut u16>()).write(core::ptr::null_mut());
            }
            {
                Free(
                    (((&raw mut sSelectedPikaPowerTile)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                );
                ((&raw mut sSelectedPikaPowerTile)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    (((&raw mut sReelOverlay_Tilemap)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                );
                ((&raw mut sReelOverlay_Tilemap)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sDigitalDisplayGfxPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sDigitalDisplayGfxPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>())
                    .write(core::ptr::null_mut());
            }
            {
                Free(
                    (((&raw mut sReelButtonPress_Tilemap)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                );
                ((&raw mut sReelButtonPress_Tilemap)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sReelBackground_Gfx)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sReelBackground_Gfx)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sReelBackgroundSpriteSheet)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sReelBackgroundSpriteSheet)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(
                    ((&raw mut sSlotMachineSpritesheetsPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                ((&raw mut sSlotMachineSpritesheetsPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
            }
            {
                Free(((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                    .write(core::ptr::null_mut());
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DrawMachineBias() {
    unsafe {
        let mut whichBias: u8 = 0u8;
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .read()) as i32)
            == 0i32
        {
            if !((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32)
                & 192i32)
                != 0)
            {
                if (ShouldTrySpecialBias()) != 0 {
                    whichBias = TrySelectBias_Special();
                    if ((whichBias) as u32) != crate::c::div_u32(6u32, 2u32) {
                        let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4);
                        (__p1).write(
                            (((((__p1).read()) as i32)
                                | ((((((&raw const sBiasesSpecial)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((whichBias) as i32) as isize))
                                .read()) as i32)) as u8),
                        );
                        if ((whichBias) as i32) != 1i32 {
                            return;
                        }
                    }
                }
                whichBias = TrySelectBias_Regular();
                if ((whichBias) as u32) != crate::c::div_u32(10u32, 2u32) {
                    let __p2 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4);
                    (__p2).write(
                        (((((__p2).read()) as i32)
                            | ((((((&raw const sBiasesRegular)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((whichBias) as i32) as isize))
                            .read()) as i32)) as u8),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetBiasFailure() {
    unsafe {
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
            .write(0u8);
        if (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .read())
            != 0
        {
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                .write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn GetBiasSymbol(machineBias: u8) -> u8 {
    unsafe {
        let mut machineBias = machineBias;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if (((machineBias) as i32) & 1i32) != 0 {
                        return ((((&raw const sBiasSymbols).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read();
                    }
                    machineBias = ((((machineBias) as i32) >> 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShouldTrySpecialBias() -> u8 {
    unsafe {
        let mut rval: u8 = ((Random()) as u8);
        if ((((((((&raw const sSpecialDrawOdds).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 3,
            ))
        .cast::<u8>())
        .wrapping_offset(
            (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_sub(1i32)) as isize,
        ))
        .read()) as i32)
            > ((rval) as i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TrySelectBias_Special() -> u8 {
    unsafe {
        let mut whichBias: i16 = 0i16;
        {
            whichBias = 0i16;
            'l1: loop {
                if !(((whichBias) as i32) < ((crate::c::div_u32(6u32, 2u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut rval: i16 = ((((Random()) as i32) & 255i32) as i16);
                    let mut value: i16 = ((((((((&raw const sBiasProbabilities_Special)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((whichBias) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i16);
                    if ((value) as i32) > ((rval) as i32) {
                        break 'l1;
                    }
                }
                whichBias = (whichBias).wrapping_add(1);
            }
        }
        return ((whichBias) as u8);
    }
}
pub(crate) unsafe extern "C" fn TrySelectBias_Regular() -> u8 {
    unsafe {
        let mut whichBias: i16 = 0i16;
        {
            whichBias = 0i16;
            'l1: loop {
                if !(((whichBias) as i32) < ((crate::c::div_u32(10u32, 2u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut rval: i16 = ((((Random()) as i32) & 255i32) as i16);
                    let mut value: i16 = ((((((((&raw const sBiasProbabilities_Regular)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((whichBias) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i16);
                    if (((whichBias) as i32) == 0i32)
                        && (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3))
                        .read()) as i32)
                            == 1i32)
                    {
                        value = ((((value) as i32).wrapping_add(10i32)) as i16);
                        if ((value) as i32) > 256i32 {
                            value = 256i16;
                        }
                    } else {
                        if (((whichBias) as i32) == 4i32)
                            && (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(3))
                            .read()) as i32)
                                == 1i32)
                        {
                            value = ((((value) as i32).wrapping_sub(10i32)) as i16);
                            if ((value) as i32) < 0i32 {
                                value = 0i16;
                            }
                        }
                    }
                    if ((value) as i32) > ((rval) as i32) {
                        break 'l1;
                    }
                }
                whichBias = (whichBias).wrapping_add(1);
            }
        }
        return ((whichBias) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetReelTimeSpinProbability(spins: u8) -> u8 {
    unsafe {
        let mut spins = spins;
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
            .read()) as i32)
            == 0i32
        {
            return ((((((&raw const sReelTimeProbabilities_NormalGame)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((spins) as i32) as isize * 17))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32) as isize,
            ))
            .read();
        } else {
            return ((((((&raw const sReelTimeProbabilities_LuckyGame)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((spins) as i32) as isize * 17))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32) as isize,
            ))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetReelTimeDraw() {
    unsafe {
        let mut rval: u8 = 0u8;
        let mut spins: i16 = 0i16;
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
            .write(0u8);
        rval = ((Random()) as u8);
        if ((rval) as i32) < ((GetReelTimeSpinProbability(0u8)) as i32) {
            return;
        }
        {
            spins = 5i16;
            'l1: loop {
                if !(((spins) as i32) > 0i32) {
                    break 'l1;
                }
                'l2: {
                    rval = ((Random()) as u8);
                    if ((rval) as i32) < ((GetReelTimeSpinProbability(((spins) as u8))) as i32) {
                        break 'l1;
                    }
                }
                spins = (spins).wrapping_sub(1);
            }
        }
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
            .write(((spins) as u8));
    }
}
pub(crate) unsafe extern "C" fn ShouldReelTimeMachineExplode(check: u16) -> u8 {
    unsafe {
        let mut check = check;
        let mut rval: u16 = ((((Random()) as i32) & 255i32) as u16);
        if ((rval) as i32)
            < ((((((&raw const sReelTimeExplodeProbability)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((check) as i32) as isize))
            .read()) as i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTimeSpeed() -> u16 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut rval: u8 = 0u8;
        let mut value: u8 = 0u8;
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<i16>())
        .read()) as i32)
            >= 300i32
        {
            i = 4u8;
        } else {
            if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<i16>())
            .read()) as i32)
                >= 250i32
            {
                i = 3u8;
            } else {
                if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<i16>())
                .read()) as i32)
                    >= 200i32
                {
                    i = 2u8;
                } else {
                    if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<i16>())
                    .read()) as i32)
                        >= 150i32
                    {
                        i = 1u8;
                    }
                }
            }
        }
        rval = ((crate::c::rem_i32(((Random()) as i32), 100i32)) as u8);
        value = (((((((&raw const sReelTimeSpeed_Probabilities)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 4))
        .cast::<u16>())
        .read()) as u8);
        if ((rval) as i32) < ((value) as i32) {
            return 4u16;
        }
        rval = ((crate::c::rem_i32(((Random()) as i32), 100i32)) as u8);
        value = ((((((((((&raw const sReelTimeSpeed_Probabilities)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(1))
        .read()) as i32)
            .wrapping_add(
                ((((((&raw const sQuarterSpeed_ProbabilityBoost)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11))
                    .read()) as i32) as isize,
                ))
                .read()) as i32),
            )) as u8);
        if ((rval) as i32) < ((value) as i32) {
            return 2u16;
        }
        return 8u16;
    }
}
pub(crate) unsafe extern "C" fn CheckMatch() {
    unsafe {
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(0u16);
        CheckMatch_CenterRow();
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<i16>())
        .read()) as i32)
            > 1i32
        {
            CheckMatch_TopAndBottom();
        }
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<i16>())
        .read()) as i32)
            > 2i32
        {
            CheckMatch_Diagonals();
        }
    }
}
pub(crate) unsafe extern "C" fn CheckMatch_CenterRow() {
    unsafe {
        let mut sym1: u8 = 0u8;
        let mut sym2: u8 = 0u8;
        let mut sym3: u8 = 0u8;
        let mut r#match: u8 = 0u8;
        sym1 = GetSymbolAtRest(0u8, 2i16);
        sym2 = GetSymbolAtRest(1u8, 2i16);
        sym3 = GetSymbolAtRest(2u8, 2i16);
        r#match = GetMatchFromSymbols(sym1, sym2, sym3);
        if ((r#match) as i32) != 9i32 {
            let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((&raw const sSlotPayouts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((r#match) as i32) as isize))
                    .read()) as i32),
                )) as i16),
            );
            let __p2 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    | ((((((&raw const sSlotMatchFlags)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((r#match) as i32) as isize))
                    .read()) as i32)) as u16),
            );
            FlashMatchLine(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CheckMatch_TopAndBottom() {
    unsafe {
        let mut sym1: u8 = 0u8;
        let mut sym2: u8 = 0u8;
        let mut sym3: u8 = 0u8;
        let mut r#match: u8 = 0u8;
        sym1 = GetSymbolAtRest(0u8, 1i16);
        sym2 = GetSymbolAtRest(1u8, 1i16);
        sym3 = GetSymbolAtRest(2u8, 1i16);
        r#match = GetMatchFromSymbols(sym1, sym2, sym3);
        if ((r#match) as i32) != 9i32 {
            if ((r#match) as i32) == 0i32 {
                r#match = 1u8;
            }
            let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((&raw const sSlotPayouts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((r#match) as i32) as isize))
                    .read()) as i32),
                )) as i16),
            );
            let __p2 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    | ((((((&raw const sSlotMatchFlags)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((r#match) as i32) as isize))
                    .read()) as i32)) as u16),
            );
            FlashMatchLine(1u8);
        }
        sym1 = GetSymbolAtRest(0u8, 3i16);
        sym2 = GetSymbolAtRest(1u8, 3i16);
        sym3 = GetSymbolAtRest(2u8, 3i16);
        r#match = GetMatchFromSymbols(sym1, sym2, sym3);
        if ((r#match) as i32) != 9i32 {
            if ((r#match) as i32) == 0i32 {
                r#match = 1u8;
            }
            let __p3 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((&raw const sSlotPayouts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((r#match) as i32) as isize))
                    .read()) as i32),
                )) as i16),
            );
            let __p4 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>();
            (__p4).write(
                (((((__p4).read()) as i32)
                    | ((((((&raw const sSlotMatchFlags)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((r#match) as i32) as isize))
                    .read()) as i32)) as u16),
            );
            FlashMatchLine(2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CheckMatch_Diagonals() {
    unsafe {
        let mut sym1: u8 = 0u8;
        let mut sym2: u8 = 0u8;
        let mut sym3: u8 = 0u8;
        let mut r#match: u8 = 0u8;
        sym1 = GetSymbolAtRest(0u8, 1i16);
        sym2 = GetSymbolAtRest(1u8, 2i16);
        sym3 = GetSymbolAtRest(2u8, 3i16);
        r#match = GetMatchFromSymbols(sym1, sym2, sym3);
        if ((r#match) as i32) != 9i32 {
            if ((r#match) as i32) != 0i32 {
                let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14)
                    .cast::<i16>();
                (__p1).write(
                    (((((__p1).read()) as i32).wrapping_add(
                        ((((((&raw const sSlotPayouts)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((r#match) as i32) as isize))
                        .read()) as i32),
                    )) as i16),
                );
                let __p2 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(
                    (((((__p2).read()) as i32)
                        | ((((((&raw const sSlotMatchFlags)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((r#match) as i32) as isize))
                        .read()) as i32)) as u16),
                );
            }
            FlashMatchLine(3u8);
        }
        sym1 = GetSymbolAtRest(0u8, 3i16);
        sym2 = GetSymbolAtRest(1u8, 2i16);
        sym3 = GetSymbolAtRest(2u8, 1i16);
        r#match = GetMatchFromSymbols(sym1, sym2, sym3);
        if ((r#match) as i32) != 9i32 {
            if ((r#match) as i32) != 0i32 {
                let __p3 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14)
                    .cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((&raw const sSlotPayouts)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((r#match) as i32) as isize))
                        .read()) as i32),
                    )) as i16),
                );
                let __p4 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p4).write(
                    (((((__p4).read()) as i32)
                        | ((((((&raw const sSlotMatchFlags)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((r#match) as i32) as isize))
                        .read()) as i32)) as u16),
                );
            }
            FlashMatchLine(4u8);
        }
    }
}
pub(crate) unsafe extern "C" fn GetMatchFromSymbols(sym1: u8, sym2: u8, sym3: u8) -> u8 {
    unsafe {
        let mut sym1 = sym1;
        let mut sym2 = sym2;
        let mut sym3 = sym3;
        if (((sym1) as i32) == ((sym2) as i32)) && (((sym1) as i32) == ((sym3) as i32)) {
            return ((((&raw const sSymbolToMatch).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((sym1) as i32) as isize))
            .read();
        }
        if ((((sym1) as i32) == 0i32) && (((sym2) as i32) == 0i32)) && (((sym3) as i32) == 1i32) {
            return 6u8;
        }
        if ((((sym1) as i32) == 1i32) && (((sym2) as i32) == 1i32)) && (((sym3) as i32) == 0i32) {
            return 6u8;
        }
        if ((sym1) as i32) == 4i32 {
            return 0u8;
        }
        return 9u8;
    }
}
pub(crate) unsafe extern "C" fn AwardPayout() {
    unsafe {
        Task_Payout(CreateTask(Some(Task_Payout), 4u8));
    }
}
pub(crate) unsafe extern "C" fn IsFinalTask_Task_Payout() -> u8 {
    unsafe {
        if ((FindTaskIdByFunc(Some(Task_Payout))) as i32) == 255i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Payout(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sPayoutTasks)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PayoutTask_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (IsMatchLineDoneFlashingBeforePayout()) != 0 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<i16>())
            .read()) as i32)
                == 0i32
            {
                (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PayoutTask_GivePayout(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !(({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) != 0)
        {
            if (IsFanfareTaskInactive()) != 0 {
                PlaySE(21u16);
            }
            let __p3 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
            if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<i16>())
            .read()) as i32)
                < 9999i32
            {
                let __p4 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(8i16);
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(44)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(4i16);
            }
        }
        if ((IsFanfareTaskInactive()) != 0)
            && (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 8i32)
                != 0)
        {
            PlaySE(21u16);
            let __p5 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<i16>();
            (__p5).write(
                (((((__p5).read()) as i32).wrapping_add(
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14)
                        .cast::<i16>())
                    .read()) as i32),
                )) as i16),
            );
            if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<i16>())
            .read()) as i32)
                > 9999i32
            {
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i16>())
                .write(9999i16);
            }
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<i16>())
            .write(0i16);
        }
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            let __p6 = ((task).wrapping_add(8)).cast::<i16>();
            (__p6).write(((__p6).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PayoutTask_Free(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (TryStopMatchLinesFlashing()) != 0 {
            DestroyTask(FindTaskIdByFunc(Some(Task_Payout)));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetSymbolAtRest(reel: u8, offset: i16) -> u8 {
    unsafe {
        let mut reel = reel;
        let mut offset = offset;
        let mut pos: i16 = ((crate::c::rem_i32(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(40))
            .cast::<i16>())
            .wrapping_offset(((reel) as i32) as isize))
            .read()) as i32)
                .wrapping_add(((offset) as i32)),
            21i32,
        )) as i16);
        if ((pos) as i32) < 0i32 {
            pos = ((((pos) as i32).wrapping_add(21i32)) as i16);
        }
        return ((((((&raw const sReelSymbols).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((reel) as i32) as isize * 21))
        .cast::<u8>())
        .wrapping_offset(((pos) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetSymbol(reel: u8, offset: i16) -> u8 {
    unsafe {
        let mut reel = reel;
        let mut offset = offset;
        let mut inc: i16 = 0i16;
        let mut pixelOffset: i16 = ((crate::c::rem_i32(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28))
            .cast::<i16>())
            .wrapping_offset(((reel) as i32) as isize))
            .read()) as i32),
            24i32,
        )) as i16);
        if ((pixelOffset) as i32) != 0i32 {
            inc = (-1i16);
        }
        return GetSymbolAtRest(
            reel,
            ((((offset) as i32).wrapping_add(((inc) as i32))) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn GetReelTimeSymbol(offset: i16) -> u8 {
    unsafe {
        let mut offset = offset;
        let mut newPosition: i16 = ((crate::c::rem_i32(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_add(((offset) as i32)),
            6i32,
        )) as i16);
        if ((newPosition) as i32) < 0i32 {
            newPosition = ((((newPosition) as i32).wrapping_add(6i32)) as i16);
        }
        return ((((&raw const sReelTimeSymbols).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((newPosition) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn AdvanceSlotReel(reelIndex: u8, value: i16) {
    unsafe {
        let mut reelIndex = reelIndex;
        let mut value = value;
        let __p1 = (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28))
        .cast::<i16>())
        .wrapping_offset(((reelIndex) as i32) as isize);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(((value) as i32))) as i16));
        let __p2 = (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28))
        .cast::<i16>())
        .wrapping_offset(((reelIndex) as i32) as isize);
        (__p2).write(((crate::c::rem_i32((((__p2).read()) as i32), 504i32)) as i16));
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(40))
            .cast::<i16>())
        .wrapping_offset(((reelIndex) as i32) as isize))
        .write(
            (((21i32).wrapping_sub(crate::c::div_i32(
                ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28))
                .cast::<i16>())
                .wrapping_offset(((reelIndex) as i32) as isize))
                .read()) as i32),
                24i32,
            ))) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdvanceSlotReelToNextSymbol(reelIndex: u8, value: i16) -> i16 {
    unsafe {
        let mut reelIndex = reelIndex;
        let mut value = value;
        let mut offset: i16 = ((crate::c::rem_i32(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28))
            .cast::<i16>())
            .wrapping_offset(((reelIndex) as i32) as isize))
            .read()) as i32),
            24i32,
        )) as i16);
        if ((offset) as i32) != 0i32 {
            if ((offset) as i32) < ((value) as i32) {
                value = offset;
            }
            AdvanceSlotReel(reelIndex, value);
            offset = ((crate::c::rem_i32(
                ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28))
                .cast::<i16>())
                .wrapping_offset(((reelIndex) as i32) as isize))
                .read()) as i32),
                24i32,
            )) as i16);
        }
        return offset;
    }
}
pub(crate) unsafe extern "C" fn AdvanceReeltimeReel(value: i16) {
    unsafe {
        let mut value = value;
        let __p1 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(((value) as i32))) as i16));
        let __p2 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<i16>();
        (__p2).write(((crate::c::rem_i32((((__p2).read()) as i32), 120i32)) as i16));
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(22)
            .cast::<i16>())
        .write(
            (((6i32).wrapping_sub(crate::c::div_i32(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<i16>())
                .read()) as i32),
                20i32,
            ))) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdvanceReeltimeReelToNextSymbol(value: i16) -> i16 {
    unsafe {
        let mut value = value;
        let mut offset: i16 = ((crate::c::rem_i32(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<i16>())
            .read()) as i32),
            20i32,
        )) as i16);
        if ((offset) as i32) != 0i32 {
            if ((offset) as i32) < ((value) as i32) {
                value = offset;
            }
            AdvanceReeltimeReel(value);
            offset = ((crate::c::rem_i32(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<i16>())
                .read()) as i32),
                20i32,
            )) as i16);
        }
        return offset;
    }
}
pub(crate) unsafe extern "C" fn CreateReelTasks() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut taskId: u8 = CreateTask(Some(Task_Reel), 2u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .write(((i) as i16));
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(58))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(taskId);
                    Task_Reel(taskId);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpinSlotReel(reelIndex: u8) {
    unsafe {
        let mut reelIndex = reelIndex;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(58))
            .cast::<u8>())
            .wrapping_offset(((reelIndex) as i32) as isize))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(58))
            .cast::<u8>())
            .wrapping_offset(((reelIndex) as i32) as isize))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn StopSlotReel(reelIndex: u8) {
    unsafe {
        let mut reelIndex = reelIndex;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(58))
            .cast::<u8>())
            .wrapping_offset(((reelIndex) as i32) as isize))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .write(2i16);
    }
}
pub(crate) unsafe extern "C" fn IsSlotReelMoving(reelIndex: u8) -> u8 {
    unsafe {
        let mut reelIndex = reelIndex;
        return ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(58))
            .cast::<u8>())
            .wrapping_offset(((reelIndex) as i32) as isize))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn Task_Reel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sReelTasks)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTask_StayStill(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ReelTask_Spin(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        AdvanceSlotReel(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(26)
                .cast::<i16>())
            .read(),
        );
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ReelTask_DecideStop(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
            .cast::<i16>())
        .wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                as isize,
        ))
        .write(0i16);
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(46))
            .cast::<i16>())
        .wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                as isize,
        ))
        .write(0i16);
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .read()) as i32)
            == 0i32
        {
            if ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32)
                == 0i32)
                || (!((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read())
                    != 0)))
                || (!(((((((&raw const sDecideStop_Bias)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize,
                ))
                .read())
                .unwrap_unchecked()())
                    != 0))
            {
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                    .write(0u8);
                (((((&raw const sDecideStop_NoBias)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize,
                ))
                .read())
                .unwrap_unchecked()();
            }
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .read(),
        );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ReelTask_MoveToStop(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut reelStopShocks = crate::ffi::Align4([0u8; 10]);
        let mut reelPixelPos: i16 = 0i16;
        crate::c::memcpy(
            ((&raw mut reelStopShocks).cast::<u16>()).cast::<u8>(),
            (((&raw const sReelStopShocks)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            10u32,
        );
        reelPixelPos = ((crate::c::rem_i32(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28))
            .cast::<i16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .read()) as i32),
            24i32,
        )) as i16);
        if ((reelPixelPos) as i32) != 0i32 {
            reelPixelPos = AdvanceSlotReelToNextSymbol(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(26)
                    .cast::<i16>())
                .read(),
            );
        } else {
            if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .read())
                != 0
            {
                let __p1 = (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize,
                );
                (__p1).write(((__p1).read()).wrapping_sub(1));
                AdvanceSlotReel(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(26)
                        .cast::<i16>())
                    .read(),
                );
                reelPixelPos = ((crate::c::rem_i32(
                    ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(28))
                    .cast::<i16>())
                    .wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32),
                    24i32,
                )) as i16);
            }
        }
        if (((reelPixelPos) as i32) == 0i32)
            && (((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .read()) as i32)
                == 0i32)
        {
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                (((((&raw mut reelStopShocks).cast::<u16>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        as isize,
                ))
                .read()) as i16),
            );
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ReelTask_ShakingStop(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(34))
            .cast::<u16>())
        .wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                as isize,
        ))
        .write(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_neg()) as i16),
        );
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) & 3i32)
            == 0i32
        {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p2).write((((((__p2).read()) as i32) >> 1) as i16));
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(0i16);
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(34))
            .cast::<u16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .write(0u16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel1() -> u8 {
    unsafe {
        let mut sym2: u8 = GetBiasSymbol(
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .read(),
        );
        let mut sym1: u8 = sym2;
        if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .read()) as i32)
            & 192i32)
            != 0
        {
            sym1 = 0u8;
            sym2 = 1u8;
        }
        return (((((&raw const sDecideStop_Bias_Reel1_Bets)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(u8, u8) -> u8>>())
        .cast::<Option<unsafe extern "C" fn(u8, u8) -> u8>>())
        .wrapping_offset(
            (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_sub(1i32)) as isize,
        ))
        .read())
        .unwrap_unchecked()(sym1, sym2);
    }
}
pub(crate) unsafe extern "C" fn EitherSymbolAtPos_Reel1(pos: i16, sym1: u8, sym2: u8) -> u8 {
    unsafe {
        let mut pos = pos;
        let mut sym1 = sym1;
        let mut sym2 = sym2;
        let mut sym: u8 = GetSymbol(0u8, pos);
        if (((sym) as i32) == ((sym1) as i32)) || (((sym) as i32) == ((sym2) as i32)) {
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
                .write(sym);
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AreCherriesOnScreen_Reel1(turns: i16) -> u8 {
    unsafe {
        let mut turns = turns;
        if ((((GetSymbol(0u8, (((1i32).wrapping_sub(((turns) as i32))) as i16))) as i32) == 4i32)
            || (((GetSymbol(0u8, (((2i32).wrapping_sub(((turns) as i32))) as i16))) as i32)
                == 4i32))
            || (((GetSymbol(0u8, (((3i32).wrapping_sub(((turns) as i32))) as i16))) as i32) == 4i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn BiasedTowardCherryOr7s() -> u8 {
    unsafe {
        if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .read()) as i32)
            & 194i32)
            != 0
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel1_Bet1(sym1: u8, sym2: u8) -> u8 {
    unsafe {
        let mut sym1 = sym1;
        let mut sym2 = sym2;
        let mut i: i16 = 0i16;
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) <= 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (EitherSymbolAtPos_Reel1(
                        (((2i32).wrapping_sub(((i) as i32))) as i16),
                        sym1,
                        sym2,
                    )) != 0
                    {
                        (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(52))
                        .cast::<i16>())
                        .write(2i16);
                        (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .write(i);
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel1_Bet2or3(sym1: u8, sym2: u8) -> u8 {
    unsafe {
        let mut sym1 = sym1;
        let mut sym2 = sym2;
        let mut i: i16 = 0i16;
        let mut cherry7Bias: u8 = BiasedTowardCherryOr7s();
        if ((cherry7Bias) != 0) || (!((AreCherriesOnScreen_Reel1(0i16)) != 0)) {
            {
                i = 1i16;
                'l1: loop {
                    if !(((i) as i32) <= 3i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (EitherSymbolAtPos_Reel1(i, sym1, sym2)) != 0 {
                            (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(52))
                            .cast::<i16>())
                            .write(i);
                            (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(46))
                            .cast::<i16>())
                            .write(0i16);
                            return 1u8;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        {
            i = 1i16;
            'l3: loop {
                if !(((i) as i32) <= 4i32) {
                    break 'l3;
                }
                'l4: {
                    let mut cherry7BiasCopy: u8 = cherry7Bias;
                    if ((cherry7BiasCopy) != 0) || (!((AreCherriesOnScreen_Reel1(i)) != 0)) {
                        if (EitherSymbolAtPos_Reel1(
                            (((1i32).wrapping_sub(((i) as i32))) as i16),
                            sym1,
                            sym2,
                        )) != 0
                        {
                            if (((i) as i32) == 1i32)
                                && (((cherry7BiasCopy) != 0)
                                    || (!((AreCherriesOnScreen_Reel1(3i16)) != 0)))
                            {
                                (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(52))
                                .cast::<i16>())
                                .write(3i16);
                                (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .write(3i16);
                                return 1u8;
                            }
                            if (((i) as i32) <= 3i32)
                                && (((cherry7BiasCopy) != 0)
                                    || (!((AreCherriesOnScreen_Reel1(
                                        ((((i) as i32).wrapping_add(1i32)) as i16),
                                    )) != 0)))
                            {
                                (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(52))
                                .cast::<i16>())
                                .write(2i16);
                                (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .write(((((i) as i32).wrapping_add(1i32)) as i16));
                                return 1u8;
                            }
                            (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(52))
                            .cast::<i16>())
                            .write(1i16);
                            (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(46))
                            .cast::<i16>())
                            .write(i);
                            return 1u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel2() -> u8 {
    unsafe {
        return (((((&raw const sDecideStop_Bias_Reel2_Bets)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .wrapping_offset(
            (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_sub(1i32)) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel2_Bet1or2() -> u8 {
    unsafe {
        let mut i: i16 = 0i16;
        let mut reel1BiasRow: i16 =
            (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
                .cast::<i16>())
            .read();
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) <= 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetSymbol(
                        1u8,
                        ((((reel1BiasRow) as i32).wrapping_sub(((i) as i32))) as i16),
                    )) as i32)
                        == ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(7))
                        .read()) as i32)
                    {
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(52))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(reel1BiasRow);
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(i);
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel2_Bet3() -> u8 {
    unsafe {
        let mut i: i16 = 0i16;
        if (DecideStop_Bias_Reel2_Bet1or2()) != 0 {
            if (((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52))
            .cast::<i16>())
            .read()) as i32)
                != 2i32)
                && (((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    > 1i32))
                && (((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 4i32)
            {
                {
                    i = 0i16;
                    'l1: loop {
                        if !(((i) as i32) <= 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            if ((GetSymbol(1u8, (((2i32).wrapping_sub(((i) as i32))) as i16)))
                                as i32)
                                == ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(7))
                                .read()) as i32)
                            {
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(52))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(2i16);
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(i);
                                break 'l1;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            return 1u8;
        }
        if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
            .cast::<i16>())
        .read()) as i32)
            != 2i32
        {
            {
                i = 0i16;
                'l3: loop {
                    if !(((i) as i32) <= 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((GetSymbol(1u8, (((2i32).wrapping_sub(((i) as i32))) as i16))) as i32)
                            == ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(7))
                            .read()) as i32)
                        {
                            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(52))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .write(2i16);
                            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .write(i);
                            return 1u8;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel3() -> u8 {
    unsafe {
        let mut biasSymbol: u8 =
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
                .read();
        if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .read()) as i32)
            & 64i32)
            != 0
        {
            biasSymbol = 0u8;
            if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7))
            .read()) as i32)
                == 0i32
            {
                biasSymbol = 1u8;
            }
        }
        return (((((&raw const sDecideStop_Bias_Reel3_Bets)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
        .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
        .wrapping_offset(
            (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_sub(1i32)) as isize,
        ))
        .read())
        .unwrap_unchecked()(biasSymbol);
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel3_Bet1or2(biasSymbol: u8) -> u8 {
    unsafe {
        let mut biasSymbol = biasSymbol;
        let mut i: i16 = 0i16;
        let mut reel2BiasRow: i16 =
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52))
            .cast::<i16>())
            .wrapping_offset(1))
            .read();
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) <= 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetSymbol(
                        2u8,
                        ((((reel2BiasRow) as i32).wrapping_sub(((i) as i32))) as i16),
                    )) as i32)
                        == ((biasSymbol) as i32)
                    {
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(52))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(reel2BiasRow);
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(i);
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel3_Bet3(biasSymbol: u8) -> u8 {
    unsafe {
        let mut biasSymbol = biasSymbol;
        let mut i: i16 = 0i16;
        let mut biasRow: i16 = 0i16;
        if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
            .cast::<i16>())
        .read()) as i32)
            == ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
        {
            return DecideStop_Bias_Reel3_Bet1or2(biasSymbol);
        }
        if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
            .cast::<i16>())
        .read()) as i32)
            == 1i32
        {
            biasRow = 3i16;
        } else {
            biasRow = 1i16;
        }
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) <= 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetSymbol(
                        2u8,
                        ((((biasRow) as i32).wrapping_sub(((i) as i32))) as i16),
                    )) as i32)
                        == ((biasSymbol) as i32)
                    {
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(i);
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(52))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(biasRow);
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel1() {
    unsafe {
        let mut i: i16 = 0i16;
        'l1: loop {
            if !(((AreCherriesOnScreen_Reel1(i)) as i32) != 0i32) {
                break 'l1;
            }
            i = (i).wrapping_add(1);
        }
        (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(46))
            .cast::<i16>())
        .write(i);
    }
}
pub(crate) unsafe extern "C" fn IfSymbol7_SwitchColor(symbol: *mut u8) -> u8 {
    unsafe {
        let mut symbol = symbol;
        if (((symbol).read()) as i32) == 0i32 {
            (symbol).write(1u8);
            return 1u8;
        }
        if (((symbol).read()) as i32) == 1i32 {
            (symbol).write(0u8);
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel2() {
    unsafe {
        (((((&raw const sDecideStop_NoBias_Reel2_Bets)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_sub(1i32)) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel2_Bet1() {
    unsafe {
        if ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52))
        .cast::<i16>())
        .read()) as i32)
            != 0i32)
            && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32)
                & 128i32)
                != 0)
        {
            let mut reel1MiddleSym: u8 = GetSymbol(
                0u8,
                (((2i32).wrapping_sub(
                    (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
            );
            if (IfSymbol7_SwitchColor(&raw mut reel1MiddleSym)) != 0 {
                let mut i: i16 = 0i16;
                {
                    i = 0i16;
                    'l1: loop {
                        if !(((i) as i32) <= 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            if ((reel1MiddleSym) as i32)
                                == ((GetSymbol(1u8, (((2i32).wrapping_sub(((i) as i32))) as i16)))
                                    as i32)
                            {
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(52))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(2i16);
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(i);
                                break 'l1;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel2_Bet2() {
    unsafe {
        if ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52))
        .cast::<i16>())
        .read()) as i32)
            != 0i32)
            && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32)
                & 128i32)
                != 0)
        {
            let mut reel1BiasSym: u8 = GetSymbol(
                0u8,
                (((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(
                        (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
            );
            if (IfSymbol7_SwitchColor(&raw mut reel1BiasSym)) != 0 {
                let mut i: i16 = 0i16;
                {
                    i = 0i16;
                    'l1: loop {
                        if !(((i) as i32) <= 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            if ((reel1BiasSym) as i32)
                                == ((GetSymbol(
                                    1u8,
                                    (((((((((&raw mut sSlotMachine)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(52))
                                    .cast::<i16>())
                                    .read()) as i32)
                                        .wrapping_sub(((i) as i32)))
                                        as i16),
                                )) as i32)
                            {
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(52))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(
                                    (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(52))
                                    .cast::<i16>())
                                    .read(),
                                );
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(i);
                                break 'l1;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel2_Bet3() {
    unsafe {
        let mut i: i16 = 0i16;
        let mut j: i16 = 0i16;
        let mut reel1BiasSym: u8 = 0u8;
        if ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52))
        .cast::<i16>())
        .read()) as i32)
            != 0i32)
            && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32)
                & 128i32)
                != 0)
        {
            if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52))
            .cast::<i16>())
            .read()) as i32)
                == 2i32
            {
                DecideStop_NoBias_Reel2_Bet2();
                return;
            }
            reel1BiasSym = GetSymbol(
                0u8,
                (((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(
                        (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
            );
            if (IfSymbol7_SwitchColor(&raw mut reel1BiasSym)) != 0 {
                j = 2i16;
                if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .read()) as i32)
                    == 3i32
                {
                    j = 3i16;
                }
                {
                    i = 0i16;
                    'l1: loop {
                        if !(((i) as i32) < 2i32) {
                            break 'l1;
                        }
                        'l2: {
                            if ((reel1BiasSym) as i32) == ((GetSymbol(1u8, j)) as i32) {
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(52))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(j);
                                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(0i16);
                                return;
                            }
                        }
                        i = (i).wrapping_add(1);
                        j = (j).wrapping_sub(1);
                    }
                }
                {
                    j = 1i16;
                    'l3: loop {
                        if !(((j) as i32) <= 4i32) {
                            break 'l3;
                        }
                        'l4: {
                            if ((reel1BiasSym) as i32)
                                == ((GetSymbol(
                                    1u8,
                                    (((((((((&raw mut sSlotMachine)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(52))
                                    .cast::<i16>())
                                    .read()) as i32)
                                        .wrapping_sub(((j) as i32)))
                                        as i16),
                                )) as i32)
                            {
                                if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(52))
                                .cast::<i16>())
                                .read()) as i32)
                                    == 1i32
                                {
                                    if ((j) as i32) <= 2i32 {
                                        ((((((&raw mut sSlotMachine)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(52))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .write(2i16);
                                        ((((((&raw mut sSlotMachine)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(46))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .write(((((j) as i32).wrapping_add(1i32)) as i16));
                                    } else {
                                        ((((((&raw mut sSlotMachine)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(52))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .write(1i16);
                                        ((((((&raw mut sSlotMachine)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(46))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .write(j);
                                    }
                                } else {
                                    if ((j) as i32) <= 2i32 {
                                        ((((((&raw mut sSlotMachine)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(52))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .write(3i16);
                                        ((((((&raw mut sSlotMachine)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(46))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .write(j);
                                    } else {
                                        ((((((&raw mut sSlotMachine)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(52))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .write(2i16);
                                        ((((((&raw mut sSlotMachine)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(46))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .write(((((j) as i32).wrapping_sub(1i32)) as i16));
                                    }
                                }
                                return;
                            }
                        }
                        j = (j).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MismatchedSyms_77(sym1: u8, sym2: u8) -> u8 {
    unsafe {
        let mut sym1 = sym1;
        let mut sym2 = sym2;
        if ((((sym1) as i32) == 0i32) && (((sym2) as i32) == 1i32))
            || ((((sym1) as i32) == 1i32) && (((sym2) as i32) == 0i32))
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn MismatchedSyms_777(sym1: u8, sym2: u8, sym3: u8) -> u8 {
    unsafe {
        let mut sym1 = sym1;
        let mut sym2 = sym2;
        let mut sym3 = sym3;
        if (((((sym1) as i32) == 0i32) && (((sym2) as i32) == 1i32)) && (((sym3) as i32) == 0i32))
            || (((((sym1) as i32) == 1i32) && (((sym2) as i32) == 0i32))
                && (((sym3) as i32) == 1i32))
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn NeitherMatchNor7Mismatch(sym1: u8, sym2: u8, sym3: u8) -> u8 {
    unsafe {
        let mut sym1 = sym1;
        let mut sym2 = sym2;
        let mut sym3 = sym3;
        if ((((((((sym1) as i32) == 0i32) && (((sym2) as i32) == 1i32))
            && (((sym3) as i32) == 0i32))
            || (((((sym1) as i32) == 1i32) && (((sym2) as i32) == 0i32))
                && (((sym3) as i32) == 1i32)))
            || (((((sym1) as i32) == 0i32) && (((sym2) as i32) == 0i32))
                && (((sym3) as i32) == 1i32)))
            || (((((sym1) as i32) == 1i32) && (((sym2) as i32) == 1i32))
                && (((sym3) as i32) == 0i32)))
            || ((((sym1) as i32) == ((sym2) as i32)) && (((sym1) as i32) == ((sym3) as i32)))
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel3() {
    unsafe {
        (((((&raw const sDecideStop_NoBias_Reel3_Bets)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_sub(1i32)) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel3_Bet1() {
    unsafe {
        let mut i: i16 = 0i16;
        let mut sym1: u8 = GetSymbol(
            0u8,
            (((2i32).wrapping_sub(
                (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(46))
                .cast::<i16>())
                .read()) as i32),
            )) as i16),
        );
        let mut sym2: u8 = GetSymbol(
            1u8,
            (((2i32).wrapping_sub(
                ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32),
            )) as i16),
        );
        if ((sym1) as i32) == ((sym2) as i32) {
            'l1: loop {
                if !((1i32) != 0) {
                    break 'l1;
                }
                let mut sym3: u8 = 0u8;
                if !(((((sym1) as i32)
                    == (({
                        let __v1 = GetSymbol(2u8, (((2i32).wrapping_sub(((i) as i32))) as i16));
                        sym3 = __v1;
                        __v1
                    }) as i32))
                    || ((((sym1) as i32) == 0i32) && (((sym3) as i32) == 1i32)))
                    || ((((sym1) as i32) == 1i32) && (((sym3) as i32) == 0i32)))
                {
                    break 'l1;
                }
                i = (i).wrapping_add(1);
            }
        } else {
            if (MismatchedSyms_77(sym1, sym2)) != 0 {
                if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read()) as i32)
                    & 128i32)
                    != 0
                {
                    {
                        i = 0i16;
                        'l2: loop {
                            if !(((i) as i32) <= 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                if ((sym1) as i32)
                                    == ((GetSymbol(
                                        2u8,
                                        (((2i32).wrapping_sub(((i) as i32))) as i16),
                                    )) as i32)
                                {
                                    ((((((&raw mut sSlotMachine)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .write(i);
                                    return;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                i = 0i16;
                'l4: loop {
                    if !((1i32) != 0) {
                        break 'l4;
                    }
                    if ((sym1) as i32)
                        != ((GetSymbol(2u8, (((2i32).wrapping_sub(((i) as i32))) as i16))) as i32)
                    {
                        break 'l4;
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(46))
            .cast::<i16>())
        .wrapping_offset(2))
        .write(i);
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel3_Bet2() {
    unsafe {
        let mut extraTurns: i16 = 0i16;
        let mut i: i16 = 0i16;
        let mut sym1: u8 = 0u8;
        let mut sym2: u8 = 0u8;
        let mut sym3: u8 = 0u8;
        if ((((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            != 0i32)
            && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52))
            .cast::<i16>())
            .read()) as i32)
                == ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)))
            && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32)
                & 128i32)
                != 0)
        {
            sym1 = GetSymbol(
                0u8,
                (((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(
                        (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
            );
            sym2 = GetSymbol(
                1u8,
                ((((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
            );
            if (MismatchedSyms_77(sym1, sym2)) != 0 {
                {
                    i = 0i16;
                    'l1: loop {
                        if !(((i) as i32) <= 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            sym3 = GetSymbol(
                                2u8,
                                ((((((((((&raw mut sSlotMachine)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(52))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_sub(((i) as i32)))
                                    as i16),
                            );
                            if ((sym1) as i32) == ((sym3) as i32) {
                                extraTurns = i;
                                break 'l1;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
        'l3: loop {
            if !((1i32) != 0) {
                break 'l3;
            }
            let mut numMatches: i16 = 0i16;
            {
                i = 1i16;
                numMatches = 0i16;
                'l4: loop {
                    if !(((i) as i32) <= 3i32) {
                        break 'l4;
                    }
                    'l5: {
                        sym1 = GetSymbol(
                            0u8,
                            ((((i) as i32).wrapping_sub(
                                (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .read()) as i32),
                            )) as i16),
                        );
                        sym2 = GetSymbol(
                            1u8,
                            ((((i) as i32).wrapping_sub(
                                ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as i32),
                            )) as i16),
                        );
                        sym3 = GetSymbol(
                            2u8,
                            ((((i) as i32).wrapping_sub(((extraTurns) as i32))) as i16),
                        );
                        if (!((NeitherMatchNor7Mismatch(sym1, sym2, sym3)) != 0))
                            && (!(((MismatchedSyms_777(sym1, sym2, sym3)) != 0)
                                && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4))
                                .read()) as i32)
                                    & 128i32)
                                    != 0)))
                        {
                            numMatches = (numMatches).wrapping_add(1);
                            break 'l4;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((numMatches) as i32) == 0i32 {
                break 'l3;
            }
            extraTurns = (extraTurns).wrapping_add(1);
        }
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(46))
            .cast::<i16>())
        .wrapping_offset(2))
        .write(extraTurns);
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel3_Bet3() {
    unsafe {
        let mut sym1: u8 = 0u8;
        let mut sym2: u8 = 0u8;
        let mut sym3: u8 = 0u8;
        let mut row: i16 = 0i16;
        let mut i: i16 = 0i16;
        DecideStop_NoBias_Reel3_Bet2();
        if ((((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            != 0i32)
            && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52))
            .cast::<i16>())
            .read()) as i32)
                != ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)))
            && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32)
                & 128i32)
                != 0)
        {
            sym1 = GetSymbol(
                0u8,
                (((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(
                        (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
            );
            sym2 = GetSymbol(
                1u8,
                ((((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
            );
            if (MismatchedSyms_77(sym1, sym2)) != 0 {
                row = 1i16;
                if (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .cast::<i16>())
                .read()) as i32)
                    == 1i32
                {
                    row = 3i16;
                }
                {
                    i = 0i16;
                    'l1: loop {
                        if !(((i) as i32) <= 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            sym3 = GetSymbol(
                                2u8,
                                ((((row) as i32).wrapping_sub(
                                    ((((((((&raw mut sSlotMachine)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        .wrapping_add(((i) as i32)),
                                )) as i16),
                            );
                            if ((sym1) as i32) == ((sym3) as i32) {
                                let __p1 =
                                    (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(2);
                                (__p1).write(
                                    (((((__p1).read()) as i32).wrapping_add(((i) as i32))) as i16),
                                );
                                break 'l1;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
        'l3: loop {
            if !((1i32) != 0) {
                break 'l3;
            }
            sym1 = GetSymbol(
                0u8,
                (((1i32).wrapping_sub(
                    (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
            );
            sym2 = GetSymbol(
                1u8,
                (((2i32).wrapping_sub(
                    ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
            sym3 = GetSymbol(
                2u8,
                (((3i32).wrapping_sub(
                    ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
            if ((NeitherMatchNor7Mismatch(sym1, sym2, sym3)) != 0)
                || (((MismatchedSyms_777(sym1, sym2, sym3)) != 0)
                    && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read()) as i32)
                        & 128i32)
                        != 0))
            {
                break 'l3;
            }
            let __p2 = (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        'l4: loop {
            if !((1i32) != 0) {
                break 'l4;
            }
            sym1 = GetSymbol(
                0u8,
                (((3i32).wrapping_sub(
                    (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
            );
            sym2 = GetSymbol(
                1u8,
                (((2i32).wrapping_sub(
                    ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
            sym3 = GetSymbol(
                2u8,
                (((1i32).wrapping_sub(
                    ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
            if ((NeitherMatchNor7Mismatch(sym1, sym2, sym3)) != 0)
                || (((MismatchedSyms_777(sym1, sym2, sym3)) != 0)
                    && ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read()) as i32)
                        & 128i32)
                        != 0))
            {
                break 'l4;
            }
            let __p3 = (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn PressStopReelButton(reelNum: u8) {
    unsafe {
        let mut reelNum = reelNum;
        let mut taskId: u8 = CreateTask(Some(Task_PressStopReelButton), 5u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(((reelNum) as i16));
        Task_PressStopReelButton(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_PressStopReelButton(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sReelStopButtonTasks)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8, u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8, u8)>>())
        .wrapping_offset(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            taskId,
        );
    }
}
pub(crate) unsafe extern "C" fn StopReelButton_Press(task: *mut u8, taskId: u8) {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        SetReelButtonTilemap(
            ((((&raw const sReelButtonOffsets)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .read(),
            98u16,
            99u16,
            114u16,
            115u16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn StopReelButton_Wait(task: *mut u8, taskId: u8) {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 11i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn StopReelButton_Unpress(task: *mut u8, taskId: u8) {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        SetReelButtonTilemap(
            ((((&raw const sReelButtonOffsets)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .read(),
            66u16,
            67u16,
            82u16,
            83u16,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn LightenMatchLine(matchLineId: u8) {
    unsafe {
        let mut matchLineId = matchLineId;
        LoadPalette(
            (((((&raw const sLitMatchLinePalTable)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((matchLineId) as i32) as isize))
            .read())
            .cast::<u8>(),
            ((((((&raw const sMatchLinePalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((matchLineId) as i32) as isize))
            .read()) as u16),
            2u16,
        );
    }
}
pub(crate) unsafe extern "C" fn DarkenMatchLine(matchLineId: u8) {
    unsafe {
        let mut matchLineId = matchLineId;
        LoadPalette(
            (((((&raw const sDarkMatchLinePalTable)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(((matchLineId) as i32) as isize))
            .read())
            .cast::<u8>(),
            ((((((&raw const sMatchLinePalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((matchLineId) as i32) as isize))
            .read()) as u16),
            2u16,
        );
    }
}
pub(crate) unsafe extern "C" fn LightenBetTiles(betVal: u8) {
    unsafe {
        let mut betVal = betVal;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw const sMatchLinesPerBet).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((betVal) as i32) as isize))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    LightenMatchLine(
                        ((((((&raw const sBetToMatchLineIds).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((betVal) as i32) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DarkenBetTiles(betVal: u8) {
    unsafe {
        let mut betVal = betVal;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw const sMatchLinesPerBet).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((betVal) as i32) as isize))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    DarkenMatchLine(
                        ((((((&raw const sBetToMatchLineIds).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((betVal) as i32) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateInvisibleFlashMatchLineSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(5u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateInvisibleSprite(Some(SpriteCB_FlashMatchingLines));
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(((i) as i16));
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(68))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(spriteId);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FlashMatchLine(matchLineId: u8) {
    unsafe {
        let mut matchLineId = matchLineId;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(68))
            .cast::<u8>())
            .wrapping_offset(((matchLineId) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(4i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(2i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn IsMatchLineDoneFlashingBeforePayout() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(5u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(68))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    );
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        != 0)
                        && ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .read())
                            != 0)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn TryStopMatchLinesFlashing() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(5u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if !((TryStopMatchLineFlashing(
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(68))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    )) != 0)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn TryStopMatchLineFlashing(spriteId: u8) -> u8 {
    unsafe {
        let mut spriteId = spriteId;
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
            return 1u8;
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        return ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlashMatchingLines(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut maxColorChange: i16 = 0i16;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0 {
            if !(({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_sub(1));
                __t2
            }) != 0)
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )) as i16),
                );
                maxColorChange = 4i16;
                if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) != 0 {
                    maxColorChange = 8i16;
                }
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    <= 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)).wrapping_neg()) as i16));
                    if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        != 0
                    {
                        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        (__p4).write(((__p4).read()).wrapping_sub(1));
                    }
                } else {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        >= ((maxColorChange) as i32)
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32)
                                .wrapping_neg()) as i16),
                        );
                    }
                }
                if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) != 0 {
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p5).write((((((__p5).read()) as i32) << 1) as i16));
                }
            }
            MultiplyPaletteRGBComponents(
                ((((((&raw const sMatchLinePalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                    ))
                .read()) as u16),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FlashSlotMachineLights() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_FlashSlotMachineLights), 6u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(1i16);
        Task_FlashSlotMachineLights(taskId);
    }
}
pub(crate) unsafe extern "C" fn TryStopSlotMachineLights() -> u8 {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_FlashSlotMachineLights));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            == 0i32
        {
            DestroyTask(taskId);
            LoadPalette(
                (((&raw const sSlotMachineMenu_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
                16u16,
                32u16,
            );
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_FlashSlotMachineLights(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !(({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) != 0)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(4i16);
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
                )) as i16),
            );
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32)
                || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    == 2i32)
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_neg()) as i16),
                );
            }
        }
        LoadPalette(
            (((((&raw const sFlashingLightsPalTable)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    as isize,
            ))
            .read())
            .cast::<u8>(),
            16u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn CreatePikaPowerBoltTask() {
    unsafe {
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(62))
            .write(CreateTask(Some(Task_CreatePikaPowerBolt), 8u8));
    }
}
pub(crate) unsafe extern "C" fn AddPikaPowerBolt(bolts: u8) {
    unsafe {
        let mut bolts = bolts;
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(62))
                .read()) as i32) as isize
                * 40,
        );
        ResetPikaPowerBoltTask(task);
        (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(1i16);
    }
}
pub(crate) unsafe extern "C" fn ResetPikaPowerBolts() {
    unsafe {
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(62))
                .read()) as i32) as isize
                * 40,
        );
        ResetPikaPowerBoltTask(task);
        (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(1i16);
    }
}
pub(crate) unsafe extern "C" fn IsPikaPowerBoltAnimating() -> u8 {
    unsafe {
        return ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(62))
                .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn Task_CreatePikaPowerBolt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sPikaPowerBoltTasks)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
        );
    }
}
pub(crate) unsafe extern "C" fn PikaPowerBolt_Idle(task: *mut u8) {
    unsafe {
        let mut task = task;
    }
}
pub(crate) unsafe extern "C" fn PikaPowerBolt_AddBolt(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
            ((CreatePikaPowerBoltSprite(
                (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    << 3)
                    .wrapping_add(20i32)) as i16),
                20i16,
            )) as i16),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn PikaPowerBolt_WaitAnim(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .read())
            != 0
        {
            let mut r5: i16 = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                .read()) as i32)
                .wrapping_add(2i32)) as i16);
            let mut r3: i16 = 0i16;
            let mut r2: i16 = 0i16;
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 1i32
            {
                r3 = 1i16;
                r2 = 1i16;
            } else {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 16i32
                {
                    r3 = 2i16;
                    r2 = 2i16;
                }
            }
            ((((&raw mut sSelectedPikaPowerTile)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(((r2) as i32) as isize))
            .write(
                (((((&raw const sPikaPowerTileTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((r3) as i32) as isize * 4))
                .cast::<u16>())
                .read(),
            );
            LoadBgTilemap(
                2u8,
                ((((&raw mut sSelectedPikaPowerTile)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .wrapping_offset(((r2) as i32) as isize))
                .cast::<u8>(),
                2u16,
                ((((r5) as i32).wrapping_add(64i32)) as u16),
            );
            DestroyPikaPowerBoltSprite(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            );
            (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn PikaPowerBolt_ClearAll(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut r5: i16 = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
            as i32)
            .wrapping_add(2i32)) as i16);
        let mut r3: i16 = 0i16;
        let mut r2: i16 = 3i16;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 1i32 {
            r3 = 1i16;
            r2 = 1i16;
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 16i32
            {
                r3 = 2i16;
                r2 = 2i16;
            }
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) == 0i32 {
            ((((&raw mut sSelectedPikaPowerTile)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(((r2) as i32) as isize))
            .write(
                ((((((&raw const sPikaPowerTileTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((r3) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(1))
                .read(),
            );
            LoadBgTilemap(
                2u8,
                ((((&raw mut sSelectedPikaPowerTile)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .wrapping_offset(((r2) as i32) as isize))
                .cast::<u8>(),
                2u16,
                ((((r5) as i32).wrapping_add(64i32)) as u16),
            );
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        if (({
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            >= 20i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn ResetPikaPowerBoltTask(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        {
            i = 2u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    ((((task).wrapping_add(8)).cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPikaPowerMeter(bolts: u8) {
    unsafe {
        let mut bolts = bolts;
        let mut i: i16 = 0i16;
        let mut r3: i16 = 0i16;
        let mut r1: i16 = 0i16;
        let mut r4: i16 = 3i16;
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < ((bolts) as i32)) {
                    break 'l1;
                }
                'l2: {
                    r3 = 0i16;
                    r1 = 0i16;
                    if ((i) as i32) == 0i32 {
                        r3 = 1i16;
                        r1 = 1i16;
                    } else {
                        if ((i) as i32) == 15i32 {
                            r3 = 2i16;
                            r1 = 2i16;
                        }
                    }
                    ((((&raw mut sSelectedPikaPowerTile)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(((r1) as i32) as isize))
                    .write(
                        (((((&raw const sPikaPowerTileTable).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((r3) as i32) as isize * 4))
                        .cast::<u16>())
                        .read(),
                    );
                    LoadBgTilemap(
                        2u8,
                        ((((&raw mut sSelectedPikaPowerTile)
                            .cast::<u8>()
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(((r1) as i32) as isize))
                        .cast::<u8>(),
                        2u16,
                        ((((r4) as i32).wrapping_add(64i32)) as u16),
                    );
                }
                i = (i).wrapping_add(1);
                r4 = (r4).wrapping_add(1);
            }
        }
        {
            'l3: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l3;
                }
                'l4: {
                    r3 = 0i16;
                    r1 = 3i16;
                    if ((i) as i32) == 0i32 {
                        r3 = 1i16;
                        r1 = 1i16;
                    } else {
                        if ((i) as i32) == 15i32 {
                            r3 = 2i16;
                            r1 = 2i16;
                        }
                    }
                    ((((&raw mut sSelectedPikaPowerTile)
                        .cast::<u8>()
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset(((r1) as i32) as isize))
                    .write(
                        ((((((&raw const sPikaPowerTileTable).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((r3) as i32) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read(),
                    );
                    LoadBgTilemap(
                        2u8,
                        ((((&raw mut sSelectedPikaPowerTile)
                            .cast::<u8>()
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(((r1) as i32) as isize))
                        .cast::<u8>(),
                        2u16,
                        ((((r4) as i32).wrapping_add(64i32)) as u16),
                    );
                }
                i = (i).wrapping_add(1);
                r4 = (r4).wrapping_add(1);
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(62))
                .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((bolts) as i16));
    }
}
pub(crate) unsafe extern "C" fn BeginReelTime() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_ReelTime), 7u8);
        Task_ReelTime(taskId);
    }
}
pub(crate) unsafe extern "C" fn IsReelTimeTaskDone() -> u8 {
    unsafe {
        if ((FindTaskIdByFunc(Some(Task_ReelTime))) as i32) == 255i32 {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_ReelTime(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sReelTimeTasks)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
        );
    }
}
pub(crate) unsafe extern "C" fn ReelTime_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .write(0u8);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(22)
            .cast::<i16>())
        .write(0i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(30i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(1280i16);
        ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write(0i16);
        ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write(0i16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        LoadReelTimeWindowTilemap(30i16, 0i16);
        CreateReelTimeMachineSprites();
        CreateReelTimePikachuSprite();
        CreateReelTimeNumberSprites();
        CreateReelTimeShadowSprites();
        CreateReelTimeNumberGapSprite();
        GetReelTimeDraw();
        StopMapMusic();
        PlayNewMapMusic(392u16);
    }
}
pub(crate) unsafe extern "C" fn ReelTime_WindowEnter(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut r3: i16 = 0i16;
        let __p1 = (&raw mut gSpriteCoordOffsetX).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        r3 = (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            .wrapping_add(240i32)
            & 255i32)
            >> 3) as i16);
        SetGpuReg(
            20u8,
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                & 511i32) as u16),
        );
        if (((r3) as i32)
            != ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32))
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                <= 18i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(r3);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    >> 3) as i16),
            );
            LoadReelTimeWindowTilemap(
                r3,
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read(),
            );
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) >= 200i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        AdvanceReeltimeReel(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >> 8)
                as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn ReelTime_WaitStartPikachu(task: *mut u8) {
    unsafe {
        let mut task = task;
        AdvanceReeltimeReel(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >> 8)
                as i16),
        );
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 60i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            CreateReelTimeBoltSprites();
            CreateReelTimePikachuAuraSprites();
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_PikachuSpeedUp1(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut i: i32 = 0i32;
        let mut pikachuAnimIds = crate::ffi::Align4([0u8; 4]);
        let mut reelTimeBoltDelays = crate::ffi::Align4([0u8; 8]);
        let mut pikachuAuraFlashDelays = crate::ffi::Align4([0u8; 8]);
        crate::c::memcpy(
            (&raw mut pikachuAnimIds).cast::<u8>(),
            ((&raw const sReelTimePikachuAnimIds).cast::<u8>().cast_mut()).cast::<u8>(),
            4u32,
        );
        crate::c::memcpy(
            ((&raw mut reelTimeBoltDelays).cast::<i16>()).cast::<u8>(),
            (((&raw const sReelTimeBoltDelays)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .cast::<u8>(),
            8u32,
        );
        crate::c::memcpy(
            ((&raw mut pikachuAuraFlashDelays).cast::<i16>()).cast::<u8>(),
            (((&raw const sPikachuAuraFlashDelays)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .cast::<u8>(),
            8u32,
        );
        AdvanceReeltimeReel(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >> 8)
                as i16),
        );
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(4i32)) as i16));
        i = (4i32).wrapping_sub(
            (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >> 8),
        );
        SetReelTimeBoltDelay(
            (((&raw mut reelTimeBoltDelays).cast::<i16>()).wrapping_offset((i) as isize)).read(),
        );
        SetReelTimePikachuAuraFlashDelay(
            (((&raw mut pikachuAuraFlashDelays).cast::<i16>()).wrapping_offset((i) as isize))
                .read(),
        );
        StartSpriteAnimIfDifferent(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(63))
                .read()) as i32) as isize
                    * 68,
            ),
            (((&raw mut pikachuAnimIds).cast::<u8>()).wrapping_offset((i) as isize)).read(),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) <= 256i32
        {
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(256i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_PikachuSpeedUp2(task: *mut u8) {
    unsafe {
        let mut task = task;
        AdvanceReeltimeReel(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >> 8)
                as i16),
        );
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 80i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            SetReelTimePikachuAuraFlashDelay(2i16);
            StartSpriteAnimIfDifferent(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(63))
                    .read()) as i32) as isize
                        * 68,
                ),
                3u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_WaitReel(task: *mut u8) {
    unsafe {
        let mut task = task;
        AdvanceReeltimeReel(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >> 8)
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8)
                as i32)
                .wrapping_add(128i32)) as i16),
        );
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 80i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_CheckExplode(task: *mut u8) {
    unsafe {
        let mut task = task;
        AdvanceReeltimeReel(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >> 8)
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8)
                as i32)
                .wrapping_add(64i32)) as i16),
        );
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 40i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            if (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .read())
                != 0
            {
                if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10))
                .read()) as i32)
                    <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
            } else {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    > 3i32
                {
                    let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                } else {
                    if (ShouldReelTimeMachineExplode(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as u16),
                    )) != 0
                    {
                        (((task).wrapping_add(8)).cast::<i16>()).write(14i16);
                    }
                }
            }
            let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
            (__p5).write(((__p5).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_LandOnOutcome(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut reeltimePixelOffset: i16 = ((crate::c::rem_i32(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<i16>())
            .read()) as i32),
            20i32,
        )) as i16);
        if (reeltimePixelOffset) != 0 {
            reeltimePixelOffset = AdvanceReeltimeReelToNextSymbol(
                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    >> 8) as i16),
            );
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8)
                    as i32)
                    .wrapping_add(64i32)) as i16),
            );
        } else {
            if ((GetReelTimeSymbol(1i16)) as i32)
                != ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5))
                .read()) as i32)
            {
                AdvanceReeltimeReel(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        >> 8) as i16),
                );
                reeltimePixelOffset = ((crate::c::rem_i32(
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<i16>())
                    .read()) as i32),
                    20i32,
                )) as i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as u8) as i32)
                        .wrapping_add(64i32)) as i16),
                );
            }
        }
        if (((reeltimePixelOffset) as i32) == 0i32)
            && (((GetReelTimeSymbol(1i16)) as i32)
                == ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5))
                .read()) as i32))
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_PikachuReact(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 60i32
        {
            StopMapMusic();
            DestroyReelTimeBoltSprites();
            DestroyReelTimePikachuAuraSprites();
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5))
            .read()) as i32)
                == 0i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(160i16);
                StartSpriteAnimIfDifferent(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(63))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    5u8,
                );
                PlayFanfare(391u16);
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(192i16);
                StartSpriteAnimIfDifferent(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(63))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    4u8,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(63))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(43))
                .write(0u8);
                if (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read())
                    != 0
                {
                    ResetPikaPowerBolts();
                    ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .write(0u8);
                }
                PlayFanfare(390u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_WaitClearPikaPower(task: *mut u8) {
    unsafe {
        let mut task = task;
        if ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32))
            && (!((IsPikaPowerBoltAnimating()) != 0))
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_CloseWindow(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut r4: i16 = 0i16;
        let __p1 = (&raw mut gSpriteCoordOffsetX).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        (__p3).write((((((__p3).read()) as i32).wrapping_add(8i32)) as i16));
        r4 = (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            .wrapping_sub(8i32)
            & 255i32)
            >> 3) as i16);
        SetGpuReg(
            20u8,
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                & 511i32) as u16),
        );
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) >> 3)
            <= 25i32
        {
            ClearReelTimeWindowTilemap(r4);
        } else {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_DestroySprites(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(11))
            .write(0u8);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10)).write(
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .read(),
        );
        ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write(0i16);
        SetGpuReg(20u8, 0u16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26)
            .cast::<i16>())
        .write(8i16);
        DestroyReelTimePikachuSprite();
        DestroyReelTimeMachineSprites();
        DestroyReelTimeShadowSprites();
        PlayNewMapMusic(
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96)
                .cast::<u16>())
            .read(),
        );
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .read()) as i32)
            == 0i32
        {
            DestroyTask(FindTaskIdByFunc(Some(Task_ReelTime)));
        } else {
            CreateDigitalDisplayScene(4u8);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                .write(((ReelTimeSpeed()) as i16));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_SetReelSpeed(task: *mut u8) {
    unsafe {
        let mut task = task;
        if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26)
            .cast::<i16>())
        .read()) as i32)
            == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if (crate::c::rem_i32(
                (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28))
                .cast::<i16>())
                .read()) as i32),
                24i32,
            ) == 0i32)
                && (((({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    & 7i32)
                    == 0i32)
            {
                let __p4 = (((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(26)
                    .cast::<i16>();
                (__p4).write((((((__p4).read()) as i32) >> 1) as i16));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_EndSuccess(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (IsDigitalDisplayAnimFinished()) != 0 {
            DestroyTask(FindTaskIdByFunc(Some(Task_ReelTime)));
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_ExplodeMachine(task: *mut u8) {
    unsafe {
        let mut task = task;
        DestroyReelTimeMachineSprites();
        DestroyReelTimeBoltSprites();
        DestroyReelTimePikachuAuraSprites();
        CreateReelTimeExplosionSprite();
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(78))
                .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        StartSpriteAnimIfDifferent(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(63))
                .read()) as i32) as isize
                    * 68,
            ),
            5u8,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(4i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        StopMapMusic();
        PlayFanfare(391u16);
        PlaySE(178u16);
    }
}
pub(crate) unsafe extern "C" fn ReelTime_WaitExplode(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((&raw mut gSpriteCoordOffsetY).cast::<i16>())
            .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read());
        SetGpuReg(
            22u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u16),
        );
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32) & 1i32)
            != 0
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        if ((({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            & 31i32)
            == 0i32
        {
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p3).write((((((__p3).read()) as i32) >> 1) as i16));
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) == 0i32 {
            DestroyReelTimeExplosionSprite();
            CreateReelTimeDuckSprites();
            CreateBrokenReelTimeMachineSprite();
            CreateReelTimeSmokeSprite();
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(78))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_WaitSmoke(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write(0i16);
        SetGpuReg(22u8, 0u16);
        if (IsReelTimeSmokeAnimFinished()) != 0 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            DestroyReelTimeSmokeSprite();
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_EndFailure(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write(0i16);
        SetGpuReg(20u8, 0u16);
        PlayNewMapMusic(
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96)
                .cast::<u16>())
            .read(),
        );
        DestroyReelTimePikachuSprite();
        DestroyBrokenReelTimeMachineSprite();
        DestroyReelTimeShadowSprites();
        DestroyReelTimeDuckSprites();
        DestroyTask(FindTaskIdByFunc(Some(Task_ReelTime)));
    }
}
pub(crate) unsafe extern "C" fn LoadReelTimeWindowTilemap(a0: i16, a1: i16) {
    unsafe {
        let mut a0 = a0;
        let mut a1 = a1;
        let mut i: i16 = 0i16;
        {
            i = 4i16;
            'l1: loop {
                if !(((i) as i32) < 15i32) {
                    break 'l1;
                }
                'l2: {
                    LoadBgTilemap(
                        1u8,
                        ((((&raw const sReelTimeWindow_Tilemap)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            (((a1) as i32).wrapping_add(
                                (((i) as i32).wrapping_sub(4i32)).wrapping_mul(20i32),
                            )) as isize,
                        ))
                        .cast::<u8>(),
                        2u16,
                        ((((32i32).wrapping_mul(((i) as i32))).wrapping_add(((a0) as i32))) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearReelTimeWindowTilemap(a0: i16) {
    unsafe {
        let mut a0 = a0;
        let mut i: u8 = 0u8;
        {
            i = 4u8;
            'l1: loop {
                if !(((i) as i32) < 15i32) {
                    break 'l1;
                }
                'l2: {
                    LoadBgTilemap(
                        1u8,
                        (((&raw const sEmptyTilemap)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>(),
                        2u16,
                        ((((32i32).wrapping_mul(((i) as i32))).wrapping_add(((a0) as i32))) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn OpenInfoBox(digDisplayId: u8) {
    unsafe {
        let mut digDisplayId = digDisplayId;
        let mut taskId: u8 = CreateTask(Some(Task_InfoBox), 1u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((digDisplayId) as i16));
        Task_InfoBox(taskId);
    }
}
pub(crate) unsafe extern "C" fn IsInfoBoxClosed() -> u8 {
    unsafe {
        if ((FindTaskIdByFunc(Some(Task_InfoBox))) as i32) == 255i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_InfoBox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sInfoBoxTasks)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
        );
    }
}
pub(crate) unsafe extern "C" fn InfoBox_FadeIn(task: *mut u8) {
    unsafe {
        let mut task = task;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn InfoBox_WaitFade(task: *mut u8) {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn InfoBox_DrawWindow(task: *mut u8) {
    unsafe {
        let mut task = task;
        DestroyDigitalDisplayScene();
        LoadInfoBoxTilemap();
        AddWindow((&raw const sWindowTemplate_InfoBox).cast::<u8>().cast_mut());
        PutWindowTilemap(1u8);
        FillWindowPixelBuffer(1u8, 0u8);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn InfoBox_AddText(task: *mut u8) {
    unsafe {
        let mut task = task;
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            2u8,
            5u8,
            ((&raw const sColors_ReeltimeHelp).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_ReelTimeHelp).cast::<u8>(),
        );
        CopyWindowToVram(1u8, 3u8);
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn InfoBox_WaitInput(task: *mut u8) {
    unsafe {
        let mut task = task;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 6i32)
            != 0
        {
            FillWindowPixelBuffer(1u8, 0u8);
            ClearWindowTilemap(1u8);
            CopyWindowToVram(1u8, 1u8);
            RemoveWindow(1u8);
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn InfoBox_LoadSlotMachineTilemap(task: *mut u8) {
    unsafe {
        let mut task = task;
        LoadSlotMachineMenuTilemap();
        ShowBg(3u8);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn InfoBox_CreateDigitalDisplay(task: *mut u8) {
    unsafe {
        let mut task = task;
        CreateDigitalDisplayScene(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn InfoBox_LoadPikaPowerMeter(task: *mut u8) {
    unsafe {
        let mut task = task;
        LoadPikaPowerMeter(
            ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .read(),
        );
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn InfoBox_FreeTask(task: *mut u8) {
    unsafe {
        let mut task = task;
        DestroyTask(FindTaskIdByFunc(Some(Task_InfoBox)));
    }
}
pub(crate) unsafe extern "C" fn CreateDigitalDisplayTask() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut task: *mut u8 = core::ptr::null_mut();
        i = CreateTask(Some(Task_DigitalDisplay), 3u8);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(61))
            .write(i);
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write((-1i16));
        {
            i = 4u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    ((((task).wrapping_add(8)).cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(64i16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateDigitalDisplayScene(id: u8) {
    unsafe {
        let mut id = id;
        let mut i: u8 = 0u8;
        let mut task: *mut u8 = core::ptr::null_mut();
        DestroyDigitalDisplayScene();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(61))
                .read()) as i32) as isize
                * 40,
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(((id) as i16));
        {
            i = 0u8;
            'l1: loop {
                if !(((((((((&raw const sDigitalDisplayScenes)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((id) as i32) as isize))
                .read())
                .wrapping_offset(((i) as i32) as isize * 4))
                .read()) as i32)
                    != 255i32)
                {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = 0u8;
                    spriteId = CreateStdDigitalDisplaySprite(
                        ((((((&raw const sDigitalDisplayScenes)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((id) as i32) as isize))
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .read(),
                        (((((((&raw const sDigitalDisplayScenes)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((id) as i32) as isize))
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(1))
                        .read(),
                        (((((((&raw const sDigitalDisplayScenes)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((id) as i32) as isize))
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<i16>())
                        .read(),
                    );
                    ((((task).wrapping_add(8)).cast::<i16>())
                        .wrapping_offset(((4i32).wrapping_add(((i) as i32))) as isize))
                    .write(((spriteId) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AddDigitalDisplaySprite(
    templateIdx: u8,
    callback: Option<unsafe extern "C" fn(*mut u8)>,
    x: i16,
    y: i16,
    spriteId: i16,
) {
    unsafe {
        let mut templateIdx = templateIdx;
        let mut callback = callback;
        let mut x = x;
        let mut y = y;
        let mut spriteId = spriteId;
        let mut i: u8 = 0u8;
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(61))
                .read()) as i32) as isize
                * 40,
        );
        {
            i = 4u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((task).wrapping_add(8)).cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 64i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((CreateDigitalDisplaySprite(templateIdx, callback, x, y, spriteId))
                                as i16),
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyDigitalDisplayScene() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(61))
                .read()) as i32) as isize
                * 40,
        );
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16) as i32)
            != 65535i32
        {
            (((((&raw const sDigitalDisplaySceneExitCallbacks)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            ))
            .read())
            .unwrap_unchecked()();
        }
        {
            i = 4u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((task).wrapping_add(8)).cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 64i32
                    {
                        DestroySprite(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                        );
                        ((((task).wrapping_add(8)).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(64i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsDigitalDisplayAnimFinished() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(61))
                .read()) as i32) as isize
                * 40,
        );
        {
            i = 4u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((task).wrapping_add(8)).cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 64i32
                    {
                        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(7))
                        .read())
                            != 0
                        {
                            return 0u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Task_DigitalDisplay(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sDigitalDisplayTasks)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
        );
    }
}
pub(crate) unsafe extern "C" fn DigitalDisplay_Idle(task: *mut u8) {
    unsafe {
        let mut task = task;
    }
}
pub(crate) unsafe extern "C" fn CreateReelSymbolSprites() {
    unsafe {
        let mut i: i16 = 0i16;
        let mut j: i16 = 0i16;
        let mut x: i16 = 0i16;
        {
            i = 0i16;
            x = 48i16;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i16;
                        'l3: loop {
                            if !(((j) as i32) < 120i32) {
                                break 'l3;
                            }
                            'l4: {
                                let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(
                                        ((CreateSprite(
                                            (&raw const sSpriteTemplate_ReelSymbol)
                                                .cast::<u8>()
                                                .cast_mut(),
                                            x,
                                            0i16,
                                            14u8,
                                        )) as i32) as isize
                                            * 68,
                                    );
                                crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (3u16) as i32);
                                (((sprite).wrapping_add(46)).cast::<i16>()).write(i);
                                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                    .write(j);
                                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                    .write((-1i16));
                            }
                            j = ((((j) as i32).wrapping_add(24i32)) as i16);
                        }
                    }
                }
                i = (i).wrapping_add(1);
                x = ((((x) as i32).wrapping_add(40i32)) as i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelSymbol(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28))
            .cast::<i16>())
            .wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((crate::c::rem_i32((((__p1).read()) as i32), 120i32)) as i16));
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            (((((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(34))
            .cast::<u16>())
            .wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                .wrapping_add(28i32))
            .wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(64).cast::<u16>()).write(GetSpriteTileStartByTag(
            ((GetSymbolAtRest(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                ((crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    24i32,
                )) as i16),
            )) as u16),
        ));
        SetSpriteSheetFrameTileNum(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateCreditPayoutNumberSprites() {
    unsafe {
        let mut i: i16 = 0i16;
        let mut x: i16 = 0i16;
        {
            x = 203i16;
            i = 1i16;
            'l1: loop {
                if !(((i) as i32) <= 9999i32) {
                    break 'l1;
                }
                'l2: {
                    CreateCoinNumberSprite(x, 23i16, 0u8, i);
                }
                i = ((((i) as i32).wrapping_mul(10i32)) as i16);
                x = ((((x) as i32).wrapping_sub(7i32)) as i16);
            }
        }
        {
            x = 235i16;
            i = 1i16;
            'l3: loop {
                if !(((i) as i32) <= 9999i32) {
                    break 'l3;
                }
                'l4: {
                    CreateCoinNumberSprite(x, 23i16, 1u8, i);
                }
                i = ((((i) as i32).wrapping_mul(10i32)) as i16);
                x = ((((x) as i32).wrapping_sub(7i32)) as i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCoinNumberSprite(
    x: i16,
    y: i16,
    isPayout: u8,
    digitMult: i16,
) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut isPayout = isPayout;
        let mut digitMult = digitMult;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((CreateSprite(
                (&raw const sSpriteTemplate_CoinNumber)
                    .cast::<u8>()
                    .cast_mut(),
                x,
                y,
                13u8,
            )) as i32) as isize
                * 68,
        );
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (2u16) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((isPayout) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(digitMult);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((((digitMult) as i32).wrapping_mul(10i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write((-1i16));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CoinNumber(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut tag: u16 = ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<i16>())
        .read()) as u16);
        if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
            tag = ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<i16>())
            .read()) as u16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            != ((tag) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(((tag) as i16));
            tag = ((crate::c::rem_i32(
                ((tag) as i32),
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16)
                    as i32),
            )) as u16);
            tag = ((crate::c::div_i32(
                ((tag) as i32),
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u16)
                    as i32),
            )) as u16);
            tag = ((((tag) as i32).wrapping_add(7i32)) as u16);
            ((sprite).wrapping_add(64).cast::<u16>()).write(GetSpriteTileStartByTag(tag));
            SetSpriteSheetFrameTileNum(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateReelBackgroundSprite() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_ReelBackground)
                .cast::<u8>()
                .cast_mut(),
            88i16,
            72i16,
            15u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
        SetSubspriteTables(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            ((&raw const sSubspriteTable_ReelBackground)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimePikachuSprite() {
    unsafe {
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        let mut spriteId: u8 = 0u8;
        if ((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            == 0usize
        {
            ((&raw mut sImageTable_ReelTimePikachu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(AllocZeroed(40u32));
        }
        ((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read());
        ((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(2048u16);
        (((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(2048),
        );
        (((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .wrapping_add(4)
        .cast::<u16>())
        .write(2048u16);
        (((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(16))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(4096),
        );
        (((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(16))
        .wrapping_add(4)
        .cast::<u16>())
        .write(2048u16);
        (((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(24))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(6144),
        );
        (((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(24))
        .wrapping_add(4)
        .cast::<u16>())
        .write(2048u16);
        (((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(32))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(8192),
        );
        (((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(32))
        .wrapping_add(4)
        .cast::<u16>())
        .write(2048u16);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_ReelTimePikachu)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(
            ((&raw mut sImageTable_ReelTimePikachu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        spriteId = CreateSprite((&raw mut spriteTemplate).cast::<u8>(), 280i16, 80i16, 1u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            1,
            1,
            (1u16) as i32,
        );
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(63))
            .write(spriteId);
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimePikachuSprite() {
    unsafe {
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(63))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        if ((((&raw mut sImageTable_ReelTimePikachu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            Free(
                ((&raw mut sImageTable_ReelTimePikachu)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut sImageTable_ReelTimePikachu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimePikachu(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(38).cast::<i16>()).write({
            let __v1 = 0i16;
            ((sprite).wrapping_add(36).cast::<i16>()).write(__v1);
            __v1
        });
        if ((((sprite).wrapping_add(42)).read()) as i32) == 4i32 {
            ((sprite).wrapping_add(38).cast::<i16>()).write({
                let __v2 = 8i16;
                ((sprite).wrapping_add(36).cast::<i16>()).write(__v2);
                __v2
            });
            if ((((((sprite).wrapping_add(43)).read()) as i32) != 0i32)
                && (((crate::c::bf_read((sprite).wrapping_add(44), 0, 6, false) as u8) as i32)
                    != 0i32))
                || ((((((sprite).wrapping_add(43)).read()) as i32) == 0i32)
                    && (((crate::c::bf_read((sprite).wrapping_add(44), 0, 6, false) as u8) as i32)
                        == 0i32))
            {
                ((sprite).wrapping_add(38).cast::<i16>()).write((-8i16));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeMachineSprites() {
    unsafe {
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if ((((&raw mut sImageTable_ReelTimeMachineAntennae)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            == 0usize
        {
            ((&raw mut sImageTable_ReelTimeMachineAntennae)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(AllocZeroed(8u32));
        }
        ((((&raw mut sImageTable_ReelTimeMachineAntennae)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(10240),
        );
        ((((&raw mut sImageTable_ReelTimeMachineAntennae)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(768u16);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_ReelTimeMachineAntennae)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(
            ((&raw mut sImageTable_ReelTimeMachineAntennae)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        spriteId = CreateSprite((&raw mut spriteTemplate).cast::<u8>(), 368i16, 52i16, 7u8);
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
        SetSubspriteTables(
            sprite,
            ((&raw const sSubspriteTable_ReelTimeMachineAntennae)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(73))
            .cast::<u8>())
        .write(spriteId);
        if ((((&raw mut sImageTable_ReelTimeMachine)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            == 0usize
        {
            ((&raw mut sImageTable_ReelTimeMachine)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(AllocZeroed(8u32));
        }
        ((((&raw mut sImageTable_ReelTimeMachine)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            ((((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(10240))
            .wrapping_offset(768),
        );
        ((((&raw mut sImageTable_ReelTimeMachine)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(1280u16);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_ReelTimeMachine)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(
            ((&raw mut sImageTable_ReelTimeMachine)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        spriteId = CreateSprite((&raw mut spriteTemplate).cast::<u8>(), 368i16, 84i16, 7u8);
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
        SetSubspriteTables(
            sprite,
            ((&raw const sSubspriteTable_ReelTimeMachine)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(73))
            .cast::<u8>())
        .wrapping_offset(1))
        .write(spriteId);
    }
}
pub(crate) unsafe extern "C" fn CreateBrokenReelTimeMachineSprite() {
    unsafe {
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if ((((&raw mut sImageTable_BrokenReelTimeMachine)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            == 0usize
        {
            ((&raw mut sImageTable_BrokenReelTimeMachine)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(AllocZeroed(8u32));
        }
        ((((&raw mut sImageTable_BrokenReelTimeMachine)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(12288),
        );
        ((((&raw mut sImageTable_BrokenReelTimeMachine)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(1536u16);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_BrokenReelTimeMachine)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(
            ((&raw mut sImageTable_BrokenReelTimeMachine)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        spriteId = CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            (((168i32)
                .wrapping_sub(((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32)))
                as i16),
            80i16,
            7u8,
        );
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
        SetSubspriteTables(
            sprite,
            ((&raw const sSubspriteTable_BrokenReelTimeMachine)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(66))
            .write(spriteId);
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeNumberSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut r5: i16 = 0i16;
        {
            i = 0u8;
            r5 = 0i16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(3u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw const sSpriteTemplate_ReelTimeNumbers)
                            .cast::<u8>()
                            .cast_mut(),
                        368i16,
                        0i16,
                        10u8,
                    );
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68);
                    crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
                    crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(r5);
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(75))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(spriteId);
                }
                i = (i).wrapping_add(1);
                r5 = ((((r5) as i32).wrapping_add(20i32)) as i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeNumbers(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut r0: i16 = (((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<i16>())
        .read()) as i32)
            .wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as u16) as i16);
        r0 = ((crate::c::rem_i32(((r0) as i32), 40i32)) as i16);
        ((sprite).wrapping_add(34).cast::<i16>())
            .write(((((r0) as i32).wrapping_add(59i32)) as i16));
        StartSpriteAnimIfDifferent(
            sprite,
            GetReelTimeSymbol(((crate::c::div_i32(((r0) as i32), 20i32)) as i16)),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeShadowSprites() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_ReelTimeShadow)
                .cast::<u8>()
                .cast_mut(),
            368i16,
            100i16,
            9u8,
        );
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        SetSubspriteTables(
            sprite,
            ((&raw const sSubspriteTable_ReelTimeShadow)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(78))
            .cast::<u8>())
        .write(spriteId);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_ReelTimeShadow)
                .cast::<u8>()
                .cast_mut(),
            288i16,
            104i16,
            4u8,
        );
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        SetSubspriteTables(
            sprite,
            ((&raw const sSubspriteTable_ReelTimeShadow)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(78))
            .cast::<u8>())
        .wrapping_offset(1))
        .write(spriteId);
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeNumberGapSprite() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_ReelTimeNumberGap)
                .cast::<u8>()
                .cast_mut(),
            368i16,
            76i16,
            11u8,
        );
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        SetSubspriteTables(
            sprite,
            ((&raw const sSubspriteTable_ReelTimeNumberGap)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(64))
            .write(spriteId);
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimeMachineSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(64))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(2u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(73))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut sImageTable_ReelTimeMachineAntennae)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            Free(
                ((&raw mut sImageTable_ReelTimeMachineAntennae)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut sImageTable_ReelTimeMachineAntennae)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        if ((((&raw mut sImageTable_ReelTimeMachine)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            Free(
                ((&raw mut sImageTable_ReelTimeMachine)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut sImageTable_ReelTimeMachine)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(3u32, 1u32)) {
                    break 'l3;
                }
                'l4: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(75))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimeShadowSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(2u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(78))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyBrokenReelTimeMachineSprite() {
    unsafe {
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(66))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        if ((((&raw mut sImageTable_BrokenReelTimeMachine)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            Free(
                ((&raw mut sImageTable_BrokenReelTimeMachine)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut sImageTable_BrokenReelTimeMachine)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeBoltSprites() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_ReelTimeBolt)
                .cast::<u8>()
                .cast_mut(),
            152i16,
            32i16,
            5u8,
        );
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
        (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
            .cast::<u8>())
        .write(spriteId);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write((-1i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write((-1i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(32i16);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_ReelTimeBolt)
                .cast::<u8>()
                .cast_mut(),
            184i16,
            32i16,
            5u8,
        );
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
            .cast::<u8>())
        .wrapping_offset(1))
        .write(spriteId);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write((-1i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(32i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeBolt(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            if (({
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                let __t5 = ((__p4).read()).wrapping_add(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                >= 8i32
            {
                (((sprite).wrapping_add(46)).cast::<i16>())
                    .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetReelTimeBoltDelay(delay: i16) {
    unsafe {
        let mut delay = delay;
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(delay);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(delay);
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimeBoltSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(2u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(80))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimePikachuAuraSprites() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_ReelTimePikachuAura)
                .cast::<u8>()
                .cast_mut(),
            72i16,
            80i16,
            3u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(1i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(16i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(8i16);
        (((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(82))
            .cast::<u8>())
        .write(spriteId);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_ReelTimePikachuAura)
                .cast::<u8>()
                .cast_mut(),
            104i16,
            80i16,
            3u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(63),
            0,
            1,
            (1u16) as i32,
        );
        ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(82))
            .cast::<u8>())
        .wrapping_offset(1))
        .write(spriteId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimePikachuAura(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut colors = crate::ffi::Align4([0u8; 2]);
        (&raw mut colors).cast::<u8>().wrapping_add(0).write(16u8);
        (&raw mut colors).cast::<u8>().wrapping_add(1).write(0u8);
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0)
            && ((({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                <= 0i32)
        {
            MultiplyInvertedPaletteRGBComponents(
                ((((256i32)
                    .wrapping_add(((IndexOfSpritePaletteTag(7u16)) as i32).wrapping_mul(16i32)))
                .wrapping_add(3i32)) as u16),
                (((&raw mut colors).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32) as isize,
                ))
                .read(),
                (((&raw mut colors).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32) as isize,
                ))
                .read(),
                (((&raw mut colors).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32) as isize,
                ))
                .read(),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p4).write((((((__p4).read()) as i32) & 1i32) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read());
        }
    }
}
pub(crate) unsafe extern "C" fn SetReelTimePikachuAuraFlashDelay(delay: i16) {
    unsafe {
        let mut delay = delay;
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(82))
            .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(delay);
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimePikachuAuraSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32).wrapping_add(((IndexOfSpritePaletteTag(7u16)) as i32).wrapping_mul(16i32)))
                .wrapping_add(3i32)) as u16),
            0u8,
            0u8,
            0u8,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(2u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(82))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeExplosionSprite() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_ReelTimeExplosion)
                .cast::<u8>()
                .cast_mut(),
            168i16,
            80i16,
            6u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(65))
            .write(spriteId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeExplosion(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read());
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimeExplosionSprite() {
    unsafe {
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(65))
                .read()) as i32) as isize
                    * 68,
            ),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeDuckSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut sp = crate::ffi::Align4([0u8; 8]);
        (&raw mut sp)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(0u16);
        (&raw mut sp)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(64u16);
        (&raw mut sp)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(128u16);
        (&raw mut sp)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(192u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(4u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw const sSpriteTemplate_ReelTimeDuck)
                            .cast::<u8>()
                            .cast_mut(),
                        (((80i32).wrapping_sub(
                            ((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32),
                        )) as i16),
                        68i16,
                        0u8,
                    );
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68);
                    crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
                    crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(
                        (((((&raw mut sp).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                            .read()) as i16),
                    );
                    ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(84))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(spriteId);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeDuck(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
        let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32) & 255i32) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            20i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            6i16,
        ));
        ((sprite).wrapping_add(67)).write(0u8);
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >= 128i32 {
            ((sprite).wrapping_add(67)).write(2u8);
        }
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            >= 16i32
        {
            crate::c::bf_write(
                (sprite).wrapping_add(63),
                0,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimeDuckSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(4u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(84))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeSmokeSprite() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_ReelTimeSmoke)
                .cast::<u8>()
                .cast_mut(),
            168i16,
            60i16,
            8u8,
        );
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (3u32) as i32);
        InitSpriteAffineAnim(sprite);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(67))
            .write(spriteId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeSmoke(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        } else {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 1i32 {
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                        ^ 1i32) as u16) as i32,
                );
                if (({
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    >= 24i32
                {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                }
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    >= 16i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
                }
            }
        }
        let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p7).write((((((__p7).read()) as i32) & 255i32) as i16));
        let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p8).write((((((__p8).read()) as i32).wrapping_add(16i32)) as i16));
        let __p9 = (sprite).wrapping_add(38).cast::<i16>();
        (__p9).write(
            (((((__p9).read()) as i32).wrapping_sub(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    >> 8),
            )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn IsReelTimeSmokeAnimFinished() -> u8 {
    unsafe {
        return ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(67))
                .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimeSmokeSprite() {
    unsafe {
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(67))
                .read()) as i32) as isize
                * 68,
        );
        FreeOamMatrix(((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8));
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreatePikaPowerBoltSprite(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_PikaPowerBolt)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            12u8,
        );
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (2u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (3u32) as i32);
        InitSpriteAffineAnim(sprite);
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PikaPowerBolt(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyPikaPowerBoltSprite(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        FreeOamMatrix(((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8));
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateStdDigitalDisplaySprite(
    templateIdx: u8,
    dispInfoId: u8,
    spriteId: i16,
) -> u8 {
    unsafe {
        let mut templateIdx = templateIdx;
        let mut dispInfoId = dispInfoId;
        let mut spriteId = spriteId;
        return CreateDigitalDisplaySprite(
            templateIdx,
            ((((&raw const sDigitalDisplay_SpriteCallbacks)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .wrapping_offset(((dispInfoId) as i32) as isize))
            .read(),
            (((((&raw const sDigitalDisplay_SpriteCoords)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((dispInfoId) as i32) as isize * 4))
            .cast::<i16>())
            .read(),
            ((((((&raw const sDigitalDisplay_SpriteCoords)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((dispInfoId) as i32) as isize * 4))
            .cast::<i16>())
            .wrapping_offset(1))
            .read(),
            spriteId,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateDigitalDisplaySprite(
    templateIdx: u8,
    callback: Option<unsafe extern "C" fn(*mut u8)>,
    x: i16,
    y: i16,
    internalSpriteId: i16,
) -> u8 {
    unsafe {
        let mut templateIdx = templateIdx;
        let mut callback = callback;
        let mut x = x;
        let mut y = y;
        let mut internalSpriteId = internalSpriteId;
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                ((((&raw const sSpriteTemplates_DigitalDisplay)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((templateIdx) as i32) as isize))
                .read()
                .cast::<crate::c::Rec4<24>>()
                .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(
            ((((&raw mut sImageTables_DigitalDisplay)
                .cast::<u8>()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((templateIdx) as i32) as isize))
            .read(),
        );
        spriteId = CreateSprite((&raw mut spriteTemplate).cast::<u8>(), x, y, 16u8);
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (3u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(callback);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(internalSpriteId);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
        if !(((((&raw const sSubspriteTables_DigitalDisplay)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((templateIdx) as i32) as isize))
        .read())
        .is_null()
        {
            SetSubspriteTables(
                sprite,
                ((((&raw const sSubspriteTables_DigitalDisplay)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((templateIdx) as i32) as isize))
                .read(),
            );
        }
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Static(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Smoke(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut targetX = crate::ffi::Align4([0u8; 8]);
        (&raw mut targetX)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(4i16);
        (&raw mut targetX)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write((-4i16));
        (&raw mut targetX)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(4i16);
        (&raw mut targetX)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write((-4i16));
        let mut targetY = crate::ffi::Align4([0u8; 8]);
        (&raw mut targetY)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(4i16);
        (&raw mut targetY)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(4i16);
        (&raw mut targetY)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write((-4i16));
        (&raw mut targetY)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write((-4i16));
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            >= 16i32
        {
            crate::c::bf_write(
                (sprite).wrapping_add(66),
                0,
                6,
                ((((crate::c::bf_read((sprite).wrapping_add(66), 0, 6, false) as u8) as i32) ^ 1i32)
                    as u8) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        if ((crate::c::bf_read((sprite).wrapping_add(66), 0, 6, false) as u8) as i32) != 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                (((&raw mut targetX).cast::<i16>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read(),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((&raw mut targetY).cast::<i16>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_SmokeNE(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
        SpriteCB_DigitalDisplay_Smoke(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_SmokeSW(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(63), 1, 1, (1u16) as i32);
        SpriteCB_DigitalDisplay_Smoke(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_SmokeSE(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(63), 1, 1, (1u16) as i32);
        SpriteCB_DigitalDisplay_Smoke(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Reel(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 208i32 {
                    ((sprite).wrapping_add(32).cast::<i16>()).write(208i16);
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 90i32
                {
                    let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p7 = (sprite).wrapping_add(32).cast::<i16>();
                (__p7).write((((((__p7).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 272i32 {
                    let __p8 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Time(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(4i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= 208i32 {
                    ((sprite).wrapping_add(32).cast::<i16>()).write(208i16);
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 90i32
                {
                    let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p7 = (sprite).wrapping_add(32).cast::<i16>();
                (__p7).write((((((__p7).read()) as i32).wrapping_sub(4i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= 144i32 {
                    let __p8 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_ReelTimeNumber(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                StartSpriteAnim(
                    sprite,
                    ((((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u8),
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if (({
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    >= 4i32
                {
                    let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                let __p6 = (sprite).wrapping_add(32).cast::<i16>();
                (__p6).write((((((__p6).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 208i32 {
                    ((sprite).wrapping_add(32).cast::<i16>()).write(208i16);
                    let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (({
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 90i32
                {
                    let __p10 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                let __p11 = (sprite).wrapping_add(32).cast::<i16>();
                (__p11).write((((((__p11).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 248i32 {
                    let __p12 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_PokeballRocking(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_add(8i32)) as i16));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) >= 112i32 {
                    ((sprite).wrapping_add(34).cast::<i16>()).write(112i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(16i16);
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    == 0i32
                {
                    let __p5 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p5).write(
                        (((((__p5).read()) as i32).wrapping_sub(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32),
                        )) as i16),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)).wrapping_neg()) as i16));
                    if (({
                        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                        let __t7 = ((__p6).read()).wrapping_add(1);
                        (__p6).write(__t7);
                        __t7
                    }) as i32)
                        >= 2i32
                    {
                        let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        (__p8).write((((((__p8).read()) as i32) >> 2) as i16));
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                            .write(0i16);
                        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .read()) as i32)
                            == 0i32
                        {
                            let __p9 = ((sprite).wrapping_add(46)).cast::<i16>();
                            (__p9).write(((__p9).read()).wrapping_add(1));
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .write(0i16);
                            crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (0u8) as i32);
                        }
                    }
                }
                let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p10).write(((__p10).read()).wrapping_add(1));
                let __p11 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p11).write((((((__p11).read()) as i32) & 7i32) as i16));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Stop(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 8i32
                {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p5 = (sprite).wrapping_add(34).cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_add(2i32)) as i16));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) >= 48i32 {
                    ((sprite).wrapping_add(34).cast::<i16>()).write(48i16);
                    let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_AButtonStop(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                if (({
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 32i32
                {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(5i16);
                    crate::c::bf_write((sprite).wrapping_add(1), 4, 1, (1u32) as i32);
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    StartSpriteAnim(sprite, 1u8);
                    SetGpuReg(
                        76u8,
                        ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .read()) as i32)
                            << 4)
                            | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32))
                            << 8) as u16),
                    );
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_sub(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            >> 8),
                    )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    < 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                }
                SetGpuReg(
                    76u8,
                    ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        << 4)
                        | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32))
                        << 8) as u16),
                );
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p6).write((((((__p6).read()) as i32) & 255i32) as i16));
                let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p7).write((((((__p7).read()) as i32).wrapping_add(128i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    == 0i32
                {
                    let __p8 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                    crate::c::bf_write((sprite).wrapping_add(1), 4, 1, (0u32) as i32);
                    StartSpriteAnim(sprite, 0u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_PokeballShining(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32) < 3i32
        {
            LoadPalette(
                (((((&raw const sPokeballShiningPalTable)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32) as isize,
                ))
                .read())
                .cast::<u8>(),
                (((256i32)
                    .wrapping_add(((IndexOfSpritePaletteTag(6u16)) as i32).wrapping_mul(16i32)))
                    as u16),
                32u16,
            );
            if (({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                >= 4i32
            {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
        } else {
            LoadPalette(
                (((((&raw const sPokeballShiningPalTable)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32) as isize,
                ))
                .read())
                .cast::<u8>(),
                (((256i32)
                    .wrapping_add(((IndexOfSpritePaletteTag(6u16)) as i32).wrapping_mul(16i32)))
                    as u16),
                32u16,
            );
            if (({
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t5 = ((__p4).read()).wrapping_add(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                >= 25i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
        }
        StartSpriteAnimIfDifferent(sprite, 1u8);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_RegBonus(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut letterXOffset = crate::ffi::Align4([0u8; 16]);
        (&raw mut letterXOffset)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(0i16);
        (&raw mut letterXOffset)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write((-40i16));
        (&raw mut letterXOffset)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(0i16);
        (&raw mut letterXOffset)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write(0i16);
        (&raw mut letterXOffset)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<i16>()
            .write(48i16);
        (&raw mut letterXOffset)
            .cast::<u8>()
            .wrapping_add(10)
            .cast::<i16>()
            .write(0i16);
        (&raw mut letterXOffset)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<i16>()
            .write(24i16);
        (&raw mut letterXOffset)
            .cast::<u8>()
            .wrapping_add(14)
            .cast::<i16>()
            .write(0i16);
        let mut letterYOffset = crate::ffi::Align4([0u8; 16]);
        (&raw mut letterYOffset)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write((-32i16));
        (&raw mut letterYOffset)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(0i16);
        (&raw mut letterYOffset)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write((-32i16));
        (&raw mut letterYOffset)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write((-48i16));
        (&raw mut letterYOffset)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<i16>()
            .write(0i16);
        (&raw mut letterYOffset)
            .cast::<u8>()
            .wrapping_add(10)
            .cast::<i16>()
            .write((-48i16));
        (&raw mut letterYOffset)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<i16>()
            .write(0i16);
        (&raw mut letterYOffset)
            .cast::<u8>()
            .wrapping_add(14)
            .cast::<i16>()
            .write((-48i16));
        let mut letterDelay = crate::ffi::Align4([0u8; 16]);
        (&raw mut letterDelay)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(16i16);
        (&raw mut letterDelay)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(12i16);
        (&raw mut letterDelay)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(16i16);
        (&raw mut letterDelay)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write(0i16);
        (&raw mut letterDelay)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<i16>()
            .write(0i16);
        (&raw mut letterDelay)
            .cast::<u8>()
            .wrapping_add(10)
            .cast::<i16>()
            .write(4i16);
        (&raw mut letterDelay)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<i16>()
            .write(8i16);
        (&raw mut letterDelay)
            .cast::<u8>()
            .wrapping_add(14)
            .cast::<i16>()
            .write(8i16);
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    (((&raw mut letterXOffset).cast::<i16>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize,
                    ))
                    .read(),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    (((&raw mut letterYOffset).cast::<i16>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize,
                    ))
                    .read(),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    (((&raw mut letterDelay).cast::<i16>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize,
                    ))
                    .read(),
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if (({
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = (__p3).read();
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                    __t4
                }) as i32)
                    == 0i32
                {
                    let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) > 0i32 {
                    let __p6 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p6).write((((((__p6).read()) as i32).wrapping_sub(4i32)) as i16));
                } else {
                    if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) < 0i32 {
                        let __p7 = (sprite).wrapping_add(36).cast::<i16>();
                        (__p7).write((((((__p7).read()) as i32).wrapping_add(4i32)) as i16));
                    }
                }
                if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) > 0i32 {
                    let __p8 = (sprite).wrapping_add(38).cast::<i16>();
                    (__p8).write((((((__p8).read()) as i32).wrapping_sub(4i32)) as i16));
                } else {
                    if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) < 0i32 {
                        let __p9 = (sprite).wrapping_add(38).cast::<i16>();
                        (__p9).write((((((__p9).read()) as i32).wrapping_add(4i32)) as i16));
                    }
                }
                if (((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == 0i32)
                    && (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) == 0i32)
                {
                    let __p10 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_BigBonus(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut sp0 = crate::ffi::Align4([0u8; 16]);
        (&raw mut sp0)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(160i16);
        (&raw mut sp0)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(192i16);
        (&raw mut sp0)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(224i16);
        (&raw mut sp0)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write(104i16);
        (&raw mut sp0)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<i16>()
            .write(80i16);
        (&raw mut sp0)
            .cast::<u8>()
            .wrapping_add(10)
            .cast::<i16>()
            .write(64i16);
        (&raw mut sp0)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<i16>()
            .write(48i16);
        (&raw mut sp0)
            .cast::<u8>()
            .wrapping_add(14)
            .cast::<i16>()
            .write(24i16);
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(12i16);
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            (((&raw mut sp0).cast::<i16>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize,
            ))
            .read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            (((&raw mut sp0).cast::<i16>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize,
            ))
            .read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            != 0i32
        {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_AButtonStart(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(92)
                    .cast::<u16>())
                .write(47u16);
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(94)
                    .cast::<u16>())
                .write(63u16);
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(90)
                    .cast::<u16>())
                .write(8328u16);
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_add(176i32)) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                    (((240i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    > 208i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(208i16);
                }
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    < 208i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(208i16);
                }
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(88)
                    .cast::<u16>())
                .write(
                    (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        << 8)
                        | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)) as u16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 51i32
                {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(92)
                        .cast::<u16>())
                    .write(63u16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if ((((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<i16>())
                .read()) as i32)
                    == 0i32
                {
                    break 'l1;
                }
                AddDigitalDisplaySprite(5u8, Some(SpriteCallbackDummy), 208i16, 116i16, 0i16);
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(88)
                    .cast::<u16>())
                .write(49376u16);
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(90)
                    .cast::<u16>())
                .write(26752u16);
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(92)
                    .cast::<u16>())
                .write(47u16);
                let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p6).write((((((__p6).read()) as i32).wrapping_add(2i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_add(192i32)) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                    (((224i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    > 208i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(208i16);
                }
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    < 208i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(208i16);
                }
                ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(88)
                    .cast::<u16>())
                .write(
                    (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        << 8)
                        | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)) as u16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 15i32
                {
                    let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(92)
                        .cast::<u16>())
                    .write(63u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EndDigitalDisplayScene_Dummy() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn EndDigitalDisplayScene_StopReel() {
    unsafe {
        SetGpuReg(76u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn EndDigitalDisplayScene_Win() {
    unsafe {
        LoadPalette(
            (((&raw const sDigitalDisplay_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
            (((256i32).wrapping_add(((IndexOfSpritePaletteTag(6u16)) as i32).wrapping_mul(16i32)))
                as u16),
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn EndDigitalDisplayScene_InsertBet() {
    unsafe {
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(88)
            .cast::<u16>())
        .write(240u16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(90)
            .cast::<u16>())
        .write(160u16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(92)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sSlotMachine).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(94)
            .cast::<u16>())
        .write(63u16);
    }
}
pub(crate) unsafe extern "C" fn LoadSlotMachineGfx() {
    unsafe {
        let mut i: u8 = 0u8;
        LoadReelBackground();
        ((&raw mut sDigitalDisplayGfxPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(Alloc(12800u32));
        LZDecompressWram(
            ((&raw mut gSlotMachineDigitalDisplay_Gfx).cast::<u32>()).cast::<u32>(),
            ((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).write(Alloc(13824u32));
        LZDecompressWram(
            ((&raw const sReelTimeGfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((&raw mut sReelTimeGfxPtr).cast::<u8>().cast::<*mut u8>()).read(),
        );
        ((&raw mut sSlotMachineSpritesheetsPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(
            (8u32).wrapping_mul(crate::c::div_u32(176u32, 8u32)),
        ));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(176u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut sSlotMachineSpritesheetsPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .write(
                        (((((&raw const sSlotMachineSpriteSheets)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .cast::<*mut u8>())
                        .read(),
                    );
                    (((((&raw mut sSlotMachineSpritesheetsPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        (((((&raw const sSlotMachineSpriteSheets)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read(),
                    );
                    (((((&raw mut sSlotMachineSpritesheetsPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .write(
                        (((((&raw const sSlotMachineSpriteSheets)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut sSlotMachineSpritesheetsPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(136))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(2560),
        );
        (((((&raw mut sSlotMachineSpritesheetsPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(144))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(5120),
        );
        (((((&raw mut sSlotMachineSpritesheetsPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(152))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(5632),
        );
        (((((&raw mut sSlotMachineSpritesheetsPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(160))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(6400),
        );
        LoadSpriteSheets(
            ((&raw mut sSlotMachineSpritesheetsPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        LoadSpritePalettes(
            ((&raw const sSlotMachineSpritePalettes)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn LoadReelBackground() {
    unsafe {
        let mut dest: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        ((&raw mut sReelBackgroundSpriteSheet)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((&raw mut sReelBackground_Gfx)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8192u32));
        dest = ((&raw mut sReelBackground_Gfx)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 64i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 32i32) {
                                break 'l3;
                            }
                            'l4: {
                                (dest).write(
                                    ((((&raw const sReelBackground_Tilemap)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                            dest = (dest).wrapping_offset(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sReelBackgroundSpriteSheet)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            ((&raw mut sReelBackground_Gfx)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sReelBackgroundSpriteSheet)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(2048u16);
        ((((&raw mut sReelBackgroundSpriteSheet)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(6)
        .cast::<u16>())
        .write(17u16);
        LoadSpriteSheet(
            ((&raw mut sReelBackgroundSpriteSheet)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn LoadMenuGfx() {
    unsafe {
        ((&raw mut sMenuGfx).cast::<u8>().cast::<*mut u16>()).write((Alloc(8704u32)).cast::<u16>());
        LZDecompressWram(
            ((&raw mut gSlotMachineMenu_Gfx).cast::<u32>()).cast::<u32>(),
            (((&raw mut sMenuGfx).cast::<u8>().cast::<*mut u16>()).read()).cast::<u8>(),
        );
        LoadBgTiles(
            2u8,
            (((&raw mut sMenuGfx).cast::<u8>().cast::<*mut u16>()).read()).cast::<u8>(),
            8704u16,
            0u16,
        );
        LoadPalette(
            (((&raw mut gSlotMachineMenu_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            0u16,
            160u16,
        );
        LoadPalette(
            (((&raw const sUnkPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            208u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadMenuAndReelOverlayTilemaps() {
    unsafe {
        LoadSlotMachineMenuTilemap();
        LoadSlotMachineReelOverlay();
    }
}
pub(crate) unsafe extern "C" fn LoadSlotMachineMenuTilemap() {
    unsafe {
        LoadBgTilemap(
            2u8,
            (((&raw mut gSlotMachineMenu_Tilemap).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            1280u16,
            0u16,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadSlotMachineReelOverlay() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut dx: i16 = 0i16;
        {
            x = 4i16;
            'l1: loop {
                if !(((x) as i32) < 18i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        dx = 0i16;
                        'l3: loop {
                            if !(((dx) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                LoadBgTilemap(
                                    3u8,
                                    (((&raw mut sReelOverlay_Tilemap)
                                        .cast::<u8>()
                                        .cast::<*mut u16>())
                                    .read())
                                    .cast::<u8>(),
                                    2u16,
                                    (((((x) as i32).wrapping_add(((dx) as i32)))
                                        .wrapping_add(160i32))
                                        as u16),
                                );
                                LoadBgTilemap(
                                    3u8,
                                    ((((&raw mut sReelOverlay_Tilemap)
                                        .cast::<u8>()
                                        .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(1))
                                    .cast::<u8>(),
                                    2u16,
                                    (((((x) as i32).wrapping_add(((dx) as i32)))
                                        .wrapping_add(416i32))
                                        as u16),
                                );
                                LoadBgTilemap(
                                    3u8,
                                    ((((&raw mut sReelOverlay_Tilemap)
                                        .cast::<u8>()
                                        .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(2))
                                    .cast::<u8>(),
                                    2u16,
                                    (((((x) as i32).wrapping_add(((dx) as i32)))
                                        .wrapping_add(192i32))
                                        as u16),
                                );
                                LoadBgTilemap(
                                    3u8,
                                    ((((&raw mut sReelOverlay_Tilemap)
                                        .cast::<u8>()
                                        .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(3))
                                    .cast::<u8>(),
                                    2u16,
                                    (((((x) as i32).wrapping_add(((dx) as i32)))
                                        .wrapping_add(384i32))
                                        as u16),
                                );
                            }
                            dx = (dx).wrapping_add(1);
                        }
                    }
                    LoadBgTilemap(
                        3u8,
                        ((((&raw mut sReelOverlay_Tilemap)
                            .cast::<u8>()
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(4))
                        .cast::<u8>(),
                        2u16,
                        ((((x) as i32).wrapping_add(192i32)) as u16),
                    );
                    LoadBgTilemap(
                        3u8,
                        ((((&raw mut sReelOverlay_Tilemap)
                            .cast::<u8>()
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(5))
                        .cast::<u8>(),
                        2u16,
                        ((((x) as i32).wrapping_add(384i32)) as u16),
                    );
                    {
                        y = 7i16;
                        'l5: loop {
                            if !(((y) as i32) <= 11i32) {
                                break 'l5;
                            }
                            'l6: {
                                LoadBgTilemap(
                                    3u8,
                                    ((((&raw mut sReelOverlay_Tilemap)
                                        .cast::<u8>()
                                        .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(6))
                                    .cast::<u8>(),
                                    2u16,
                                    ((((x) as i32).wrapping_add(((y) as i32).wrapping_mul(32i32)))
                                        as u16),
                                );
                            }
                            y = (y).wrapping_add(1);
                        }
                    }
                }
                x = ((((x) as i32).wrapping_add(5i32)) as i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetReelButtonTilemap(
    offset: i16,
    topLeft: u16,
    topRight: u16,
    bottomLeft: u16,
    bottomRight: u16,
) {
    unsafe {
        let mut offset = offset;
        let mut topLeft = topLeft;
        let mut topRight = topRight;
        let mut bottomLeft = bottomLeft;
        let mut bottomRight = bottomRight;
        (((&raw mut sReelButtonPress_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .write(topLeft);
        ((((&raw mut sReelButtonPress_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(1))
        .write(topRight);
        ((((&raw mut sReelButtonPress_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(2))
        .write(bottomLeft);
        ((((&raw mut sReelButtonPress_Tilemap)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(3))
        .write(bottomRight);
        LoadBgTilemap(
            2u8,
            (((&raw mut sReelButtonPress_Tilemap)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
            2u16,
            (((480i32).wrapping_add(((offset) as i32))) as u16),
        );
        LoadBgTilemap(
            2u8,
            ((((&raw mut sReelButtonPress_Tilemap)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(1))
            .cast::<u8>(),
            2u16,
            (((481i32).wrapping_add(((offset) as i32))) as u16),
        );
        LoadBgTilemap(
            2u8,
            ((((&raw mut sReelButtonPress_Tilemap)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(2))
            .cast::<u8>(),
            2u16,
            (((512i32).wrapping_add(((offset) as i32))) as u16),
        );
        LoadBgTilemap(
            2u8,
            ((((&raw mut sReelButtonPress_Tilemap)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(3))
            .cast::<u8>(),
            2u16,
            (((513i32).wrapping_add(((offset) as i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn LoadInfoBoxTilemap() {
    unsafe {
        LoadBgTilemap(
            2u8,
            (((&raw mut gSlotMachineInfoBox_Tilemap).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            1280u16,
            0u16,
        );
        HideBg(3u8);
    }
}
pub(crate) unsafe extern "C" fn SetDigitalDisplayImagePtrs() {
    unsafe {
        (((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Reel)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(1))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Time)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(2))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Insert)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(3))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Win)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(4))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Lose)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(5))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_AButton)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(6))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Smoke)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(7))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Number)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(8))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Pokeball)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(9))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_DPad)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(10))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Stop)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(11))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Stop)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(12))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Stop)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(13))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Stop)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(14))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Bonus)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(15))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Bonus)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(16))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Bonus)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(17))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Bonus)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(18))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Bonus)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(19))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Big)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(20))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Big)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(21))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Big)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(22))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Reg)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(23))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Reg)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(24))
        .write(
            ((&raw mut sImageTable_DigitalDisplay_Reg)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTables_DigitalDisplay)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(25))
        .write(core::ptr::null_mut());
    }
}
pub(crate) unsafe extern "C" fn AllocDigitalDisplayGfx() {
    unsafe {
        ((&raw mut sImageTable_DigitalDisplay_Reel)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Reel)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            ((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Reel)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(1536u16);
        ((&raw mut sImageTable_DigitalDisplay_Time)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Time)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(1536),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Time)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(512u16);
        ((&raw mut sImageTable_DigitalDisplay_Insert)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Insert)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(2048),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Insert)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(512u16);
        ((&raw mut sImageTable_DigitalDisplay_Stop)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Stop)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(2560),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Stop)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(512u16);
        ((&raw mut sImageTable_DigitalDisplay_Win)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Win)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(3072),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Win)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(768u16);
        ((&raw mut sImageTable_DigitalDisplay_Lose)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Lose)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(4096),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Lose)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(1024u16);
        ((&raw mut sImageTable_DigitalDisplay_Bonus)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Bonus)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(5120),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Bonus)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(512u16);
        ((&raw mut sImageTable_DigitalDisplay_Big)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Big)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(5632),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Big)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(768u16);
        ((&raw mut sImageTable_DigitalDisplay_Reg)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Reg)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(6400),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Reg)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(768u16);
        ((&raw mut sImageTable_DigitalDisplay_AButton)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(16u32));
        ((((&raw mut sImageTable_DigitalDisplay_AButton)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(7168),
        );
        ((((&raw mut sImageTable_DigitalDisplay_AButton)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(512u16);
        (((((&raw mut sImageTable_DigitalDisplay_AButton)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(7680),
        );
        (((((&raw mut sImageTable_DigitalDisplay_AButton)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .wrapping_add(4)
        .cast::<u16>())
        .write(512u16);
        ((&raw mut sImageTable_DigitalDisplay_Smoke)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        ((((&raw mut sImageTable_DigitalDisplay_Smoke)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(8192),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Smoke)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(640u16);
        ((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(40u32));
        ((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(8832),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(128u16);
        (((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(8960),
        );
        (((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .wrapping_add(4)
        .cast::<u16>())
        .write(128u16);
        (((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(16))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(9088),
        );
        (((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(16))
        .wrapping_add(4)
        .cast::<u16>())
        .write(128u16);
        (((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(24))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(9216),
        );
        (((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(24))
        .wrapping_add(4)
        .cast::<u16>())
        .write(128u16);
        (((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(32))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(9344),
        );
        (((((&raw mut sImageTable_DigitalDisplay_Number)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(32))
        .wrapping_add(4)
        .cast::<u16>())
        .write(128u16);
        ((&raw mut sImageTable_DigitalDisplay_Pokeball)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(16u32));
        ((((&raw mut sImageTable_DigitalDisplay_Pokeball)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(9728),
        );
        ((((&raw mut sImageTable_DigitalDisplay_Pokeball)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(1152u16);
        (((((&raw mut sImageTable_DigitalDisplay_Pokeball)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(10880),
        );
        (((((&raw mut sImageTable_DigitalDisplay_Pokeball)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .wrapping_add(4)
        .cast::<u16>())
        .write(1152u16);
        ((&raw mut sImageTable_DigitalDisplay_DPad)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(16u32));
        ((((&raw mut sImageTable_DigitalDisplay_DPad)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(12032),
        );
        ((((&raw mut sImageTable_DigitalDisplay_DPad)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<u16>())
        .write(384u16);
        (((((&raw mut sImageTable_DigitalDisplay_DPad)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .cast::<*mut u8>())
        .write(
            (((&raw mut sDigitalDisplayGfxPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(12416),
        );
        (((((&raw mut sImageTable_DigitalDisplay_DPad)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(8))
        .wrapping_add(4)
        .cast::<u16>())
        .write(384u16);
    }
}
