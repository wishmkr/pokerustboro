//! Translated from `src/decoration.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): DecorGfx_SMALL_DESK DecorGfx_POKEMON_DESK DecorGfx_HEAVY_DESK DecorGfx_RAGGED_DESK DecorGfx_COMFORT_DESK DecorGfx_PRETTY_DESK DecorGfx_BRICK_DESK DecorGfx_CAMP_DESK DecorGfx_HARD_DESK DecorGfx_SMALL_CHAIR DecorGfx_POKEMON_CHAIR DecorGfx_HEAVY_CHAIR DecorGfx_PRETTY_CHAIR DecorGfx_COMFORT_CHAIR DecorGfx_RAGGED_CHAIR DecorGfx_BRICK_CHAIR DecorGfx_CAMP_CHAIR DecorGfx_HARD_CHAIR DecorGfx_RED_PLANT DecorGfx_TROPICAL_PLANT DecorGfx_PRETTY_FLOWERS DecorGfx_COLORFUL_PLANT DecorGfx_BIG_PLANT DecorGfx_GORGEOUS_PLANT DecorGfx_RED_BRICK DecorGfx_YELLOW_BRICK DecorGfx_BLUE_BRICK DecorGfx_RED_BALLOON DecorGfx_BLUE_BALLOON DecorGfx_YELLOW_BALLOON DecorGfx_RED_TENT DecorGfx_BLUE_TENT DecorGfx_SOLID_BOARD DecorGfx_SLIDE DecorGfx_FENCE_LENGTH DecorGfx_FENCE_WIDTH DecorGfx_TIRE DecorGfx_STAND DecorGfx_MUD_BALL DecorGfx_BREAKABLE_DOOR DecorGfx_SAND_ORNAMENT DecorGfx_SILVER_SHIELD DecorGfx_GOLD_SHIELD DecorGfx_GLASS_ORNAMENT DecorGfx_TV DecorGfx_ROUND_TV DecorGfx_CUTE_TV DecorGfx_GLITTER_MAT DecorGfx_JUMP_MAT DecorGfx_SPIN_MAT DecorGfx_C_LOW_NOTE_MAT DecorGfx_D_NOTE_MAT DecorGfx_E_NOTE_MAT DecorGfx_F_NOTE_MAT DecorGfx_G_NOTE_MAT DecorGfx_A_NOTE_MAT DecorGfx_B_NOTE_MAT DecorGfx_C_HIGH_NOTE_MAT DecorGfx_SURF_MAT DecorGfx_THUNDER_MAT DecorGfx_FIRE_BLAST_MAT DecorGfx_POWDER_SNOW_MAT DecorGfx_ATTRACT_MAT DecorGfx_FISSURE_MAT DecorGfx_SPIKES_MAT DecorGfx_BALL_POSTER DecorGfx_GREEN_POSTER DecorGfx_RED_POSTER DecorGfx_BLUE_POSTER DecorGfx_CUTE_POSTER DecorGfx_PIKA_POSTER DecorGfx_LONG_POSTER DecorGfx_SEA_POSTER DecorGfx_SKY_POSTER DecorGfx_KISS_POSTER DecorGfx_PICHU_DOLL DecorGfx_PIKACHU_DOLL DecorGfx_MARILL_DOLL DecorGfx_TOGEPI_DOLL DecorGfx_CYNDAQUIL_DOLL DecorGfx_CHIKORITA_DOLL DecorGfx_TOTODILE_DOLL DecorGfx_JIGGLYPUFF_DOLL DecorGfx_MEOWTH_DOLL DecorGfx_CLEFAIRY_DOLL DecorGfx_DITTO_DOLL DecorGfx_SMOOCHUM_DOLL DecorGfx_TREECKO_DOLL DecorGfx_TORCHIC_DOLL DecorGfx_MUDKIP_DOLL DecorGfx_DUSKULL_DOLL DecorGfx_WYNAUT_DOLL DecorGfx_BALTOY_DOLL DecorGfx_KECLEON_DOLL DecorGfx_AZURILL_DOLL DecorGfx_SKITTY_DOLL DecorGfx_SWABLU_DOLL DecorGfx_GULPIN_DOLL DecorGfx_LOTAD_DOLL DecorGfx_SEEDOT_DOLL DecorGfx_PIKA_CUSHION DecorGfx_ROUND_CUSHION DecorGfx_KISS_CUSHION DecorGfx_ZIGZAG_CUSHION DecorGfx_SPIN_CUSHION DecorGfx_DIAMOND_CUSHION DecorGfx_BALL_CUSHION DecorGfx_GRASS_CUSHION DecorGfx_FIRE_CUSHION DecorGfx_WATER_CUSHION DecorGfx_SNORLAX_DOLL DecorGfx_RHYDON_DOLL DecorGfx_LAPRAS_DOLL DecorGfx_VENUSAUR_DOLL DecorGfx_CHARIZARD_DOLL DecorGfx_BLASTOISE_DOLL DecorGfx_WAILMER_DOLL DecorGfx_REGIROCK_DOLL DecorGfx_REGICE_DOLL DecorGfx_REGISTEEL_DOLL DecorDesc_SMALL_DESK DecorDesc_POKEMON_DESK DecorDesc_HEAVY_DESK DecorDesc_RAGGED_DESK DecorDesc_COMFORT_DESK DecorDesc_PRETTY_DESK DecorDesc_BRICK_DESK DecorDesc_CAMP_DESK DecorDesc_HARD_DESK DecorDesc_SMALL_CHAIR DecorDesc_POKEMON_CHAIR DecorDesc_HEAVY_CHAIR DecorDesc_PRETTY_CHAIR DecorDesc_COMFORT_CHAIR DecorDesc_RAGGED_CHAIR DecorDesc_BRICK_CHAIR DecorDesc_CAMP_CHAIR DecorDesc_HARD_CHAIR DecorDesc_RED_PLANT DecorDesc_TROPICAL_PLANT DecorDesc_PRETTY_FLOWERS DecorDesc_COLORFUL_PLANT DecorDesc_BIG_PLANT DecorDesc_GORGEOUS_PLANT DecorDesc_RED_BRICK DecorDesc_YELLOW_BRICK DecorDesc_BLUE_BRICK DecorDesc_RED_BALLOON DecorDesc_BLUE_BALLOON DecorDesc_YELLOW_BALLOON DecorDesc_RED_TENT DecorDesc_BLUE_TENT DecorDesc_SOLID_BOARD DecorDesc_SLIDE DecorDesc_FENCE_LENGTH DecorDesc_FENCE_WIDTH DecorDesc_TIRE DecorDesc_STAND DecorDesc_MUD_BALL DecorDesc_BREAKABLE_DOOR DecorDesc_SAND_ORNAMENT DecorDesc_SILVER_SHIELD DecorDesc_GOLD_SHIELD DecorDesc_GLASS_ORNAMENT DecorDesc_TV DecorDesc_ROUND_TV DecorDesc_CUTE_TV DecorDesc_GLITTER_MAT DecorDesc_JUMP_MAT DecorDesc_SPIN_MAT DecorDesc_C_LOW_NOTE_MAT DecorDesc_D_NOTE_MAT DecorDesc_E_NOTE_MAT DecorDesc_F_NOTE_MAT DecorDesc_G_NOTE_MAT DecorDesc_A_NOTE_MAT DecorDesc_B_NOTE_MAT DecorDesc_C_HIGH_NOTE_MAT DecorDesc_SURF_MAT DecorDesc_THUNDER_MAT DecorDesc_FIRE_BLAST_MAT DecorDesc_POWDER_SNOW_MAT DecorDesc_ATTRACT_MAT DecorDesc_FISSURE_MAT DecorDesc_SPIKES_MAT DecorDesc_BALL_POSTER DecorDesc_GREEN_POSTER DecorDesc_RED_POSTER DecorDesc_BLUE_POSTER DecorDesc_CUTE_POSTER DecorDesc_PIKA_POSTER DecorDesc_LONG_POSTER DecorDesc_SEA_POSTER DecorDesc_SKY_POSTER DecorDesc_KISS_POSTER DecorDesc_PICHU_DOLL DecorDesc_PIKACHU_DOLL DecorDesc_MARILL_DOLL DecorDesc_TOGEPI_DOLL DecorDesc_CYNDAQUIL_DOLL DecorDesc_CHIKORITA_DOLL DecorDesc_TOTODILE_DOLL DecorDesc_JIGGLYPUFF_DOLL DecorDesc_MEOWTH_DOLL DecorDesc_CLEFAIRY_DOLL DecorDesc_DITTO_DOLL DecorDesc_SMOOCHUM_DOLL DecorDesc_TREECKO_DOLL DecorDesc_TORCHIC_DOLL DecorDesc_MUDKIP_DOLL DecorDesc_DUSKULL_DOLL DecorDesc_WYNAUT_DOLL DecorDesc_BALTOY_DOLL DecorDesc_KECLEON_DOLL DecorDesc_AZURILL_DOLL DecorDesc_SKITTY_DOLL DecorDesc_SWABLU_DOLL DecorDesc_GULPIN_DOLL DecorDesc_LOTAD_DOLL DecorDesc_SEEDOT_DOLL DecorDesc_PIKA_CUSHION DecorDesc_ROUND_CUSHION DecorDesc_KISS_CUSHION DecorDesc_ZIGZAG_CUSHION DecorDesc_SPIN_CUSHION DecorDesc_DIAMOND_CUSHION DecorDesc_BALL_CUSHION DecorDesc_GRASS_CUSHION DecorDesc_FIRE_CUSHION DecorDesc_WATER_CUSHION DecorDesc_SNORLAX_DOLL DecorDesc_RHYDON_DOLL DecorDesc_LAPRAS_DOLL DecorDesc_VENUSAUR_DOLL DecorDesc_CHARIZARD_DOLL DecorDesc_BLASTOISE_DOLL DecorDesc_WAILMER_DOLL DecorDesc_REGIROCK_DOLL DecorDesc_REGICE_DOLL DecorDesc_REGISTEEL_DOLL gDecorations sDecorationCategoryNames sDecorationMainMenuActions sSecretBasePCMenuItemDescriptions sSecretBasePC_SelectedDecorationActions sDecorationWindowTemplates sDecorationMenuPalette sDecorationItemsListMenuTemplate gDecorIconTable sDecorTilemap_1x1_Tiles sDecorTilemap_3x1_Tiles sDecorTilemap_2x2_Tiles sDecorTilemap_1x3_Tiles sDecorTilemap_2x1_Tiles sDecorTilemap_4x2_Tiles sDecorTilemap_3x3_Tiles sDecorTilemap_3x2_Tiles sDecorTilemap_1x1_Y sDecorTilemap_2x1_Y sDecorTilemap_3x1_Y sDecorTilemap_4x2_Y sDecorTilemap_2x2_Y sDecorTilemap_1x2_Y sDecorTilemap_1x3_Y sDecorTilemap_2x4_Y sDecorTilemap_3x3_Y sDecorTilemap_3x2_Y sDecorTilemap_1x1_X sDecorTilemap_2x1_X sDecorTilemap_3x1_X sDecorTilemap_4x2_X sDecorTilemap_2x2_X sDecorTilemap_1x2_X sDecorTilemap_1x3_X sDecorTilemap_2x4_X sDecorTilemap_3x3_X sDecorTilemap_3x2_X sDecorTilemaps sDecorationMovementInfo sDecorSelectorAnimCmd0 sDecorSelectorAnimCmds sDecorSelectorSpriteFrameImages sDecorationSelectorSpriteTemplate sDecorWhilePlacingSpriteTemplate sSpritePal_PlaceDecoration sPlaceDecorationYesNoFunctions sCancelDecoratingYesNoFunctions sPlacePutAwayYesNoFunctions sDecorationStandElevations sDecorationSlideElevation sDecorShapeSizes sBrendanPalette sMayPalette sReturnDecorationYesNoFunctions sStopPuttingAwayDecorationsYesNoFunctions sDecorationPuttingAwayCursor sSpritePal_PuttingAwayCursorBrendan sSpritePal_PuttingAwayCursorMay sPuttingAwayCursorOamData sPuttingAwayCursorAnimCmd0 sPuttingAwayCursorAnimCmds sPuttingAwayCursorPicTable sPuttingAwayCursorSpriteTemplate sTossDecorationYesNoFunctions
#[allow(unused_imports)]
use crate::data::decoration::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurDecorationItems: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationActionsCursorPos: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNumOwnedDecorationsInCurCategory: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSecretBaseItemsIndicesBuffer: crate::ffi::Align4<[u8; 16]> =
    crate::ffi::Align4([0; 16]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayerRoomItemsIndicesBuffer: crate::ffi::Align4<[u8; 12]> =
    crate::ffi::Align4([0; 12]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationsCursorPos: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationsScrollOffset: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurDecorationIndex: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurDecorationCategory: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFiller: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationContext: crate::ffi::Align4<[u8; 12]> =
    crate::ffi::Align4([0; 12]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorMenuWindowIds: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationItemsMenu: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlaceDecorationGraphicsDataBuffer: crate::ffi::Align4<[u8; 2212]> =
    crate::ffi::Align4([0; 2212]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurDecorMapX: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurDecorMapY: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecor_CameraSpriteObjectIdx1: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecor_CameraSpriteObjectIdx2: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationLastDirectionMoved: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorSelectorOam: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorRearrangementDataBuffer: crate::ffi::Align4<[u8; 128]> =
    crate::ffi::Align4([0; 128]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurDecorSelectedInRearrangement: u8 = 0u8;

unsafe extern "C" {
    static mut SecretBase_EventScript_InitDecorations: u8;
    static mut SecretBase_EventScript_PCCancel: u8;
    static mut SecretBase_EventScript_PutAwayDecoration: u8;
    static mut SecretBase_EventScript_SetDecoration: u8;
    static mut gDecorationInventories: u8;
    static mut gFieldCallback: u8;
    static mut gFieldCamera: u8;
    static mut gItemIcon4x4Buffer: u8;
    static mut gItemIconDecompressionBuffer: u8;
    static mut gItemIconSpriteTemplate: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_0x8007: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_Cancel: u8;
    static mut gText_CancelDecorating: u8;
    static mut gText_CantBePlacedHere: u8;
    static mut gText_CantPlaceInRoom: u8;
    static mut gText_CantThrowAwayInUse: u8;
    static mut gText_Color161Shadow161: u8;
    static mut gText_DecorationReturnedToPC: u8;
    static mut gText_DecorationThrownAway: u8;
    static mut gText_DecorationWillBeDiscarded: u8;
    static mut gText_Exit: u8;
    static mut gText_GoBackPrevMenu: u8;
    static mut gText_InUseAlready: u8;
    static mut gText_NoDecorationHere: u8;
    static mut gText_NoDecorations: u8;
    static mut gText_NoDecorationsInUse: u8;
    static mut gText_NoMoreDecorations: u8;
    static mut gText_NoMoreDecorations2: u8;
    static mut gText_PlaceItHere: u8;
    static mut gText_ReturnDecorationToPC: u8;
    static mut gText_StopPuttingAwayDecorations: u8;
    static mut gTilesetPointer_SecretBase: u8;
    static mut gTilesetPointer_SecretBaseRedCave: u8;
    fn AddScrollIndicatorArrowPairParameterized(
        a0: u32,
        a1: i32,
        a2: i32,
        a3: i32,
        a4: i32,
        a5: i32,
        a6: i32,
        a7: *mut u16,
    ) -> u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut u8, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocItemIconTemporaryBuffers() -> u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn BlitMenuInfoIcon(a0: u8, a1: u8, a2: u16, a3: u16);
    fn CB2_ReturnToField();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn CondenseDecorationsInCategory(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyItemIconPicTo4x4Buffer(a0: *mut u8, a1: *mut u8);
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DisplayItemMessageOnField(a0: u8, a1: *mut u8, a2: Option<unsafe extern "C" fn(u8)>);
    fn DisplayYesNoMenuDefaultYes();
    fn DoYesNoFuncWithChoice(a0: u8, a1: *mut u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawWholeMapView();
    fn ExitTraderMenu(a0: u8);
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeItemIconTemporaryBuffers();
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetMaxWidthInMenuTable(a0: *mut u8, a1: i32) -> i32;
    fn GetMetatileAttributesById(a0: u16) -> u16;
    fn GetNumOwnedDecorations() -> u8;
    fn GetNumOwnedDecorationsInCategory(a0: u8) -> u8;
    fn GetObjectEventIdByPosition(a0: u16, a1: u16, a2: u8) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn HideSecretBaseDecorationSprites();
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileEntryAt(a0: i32, a1: i32, a2: u16);
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessInput() -> i8;
    fn MetatileBehavior_HoldsLargeDecoration(a0: u8) -> u8;
    fn MetatileBehavior_HoldsSmallDecoration(a0: u8) -> u8;
    fn MetatileBehavior_IsNormal(a0: u8) -> u8;
    fn MetatileBehavior_IsPlayerRoomPCOn(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseHole(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseImpassable(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseNorthWall(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBasePC(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseTrainerSpot(a0: u8) -> u8;
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut u8);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ReshowPlayerPC(a0: u8);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetCursorScrollWithinListBounds(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8, a4: u8);
    fn SetCursorWithinListBounds(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetWarpDestination(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn TryMoveObjectEventToMapCoords(a0: u8, a1: u8, a2: u8, a3: i16, a4: i16);
    fn TryOverrideObjectEventTemplateCoords(a0: u8, a1: u8, a2: u8);
    fn TryPutSecretBaseVisitOnAir();
    fn TrySpawnObjectEvent(a0: u8, a1: u8, a2: u8) -> u8;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WarpIntoMap();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitDecorationContextItems() {
    unsafe {
        if ((((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read()) as i32) < 8i32 {
            ((&raw mut gCurDecorationItems)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(
                ((((&raw mut gDecorationInventories).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 8,
                ))
                .cast::<*mut u8>())
                .read(),
            );
        }
        if (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).read()) as i32) == 0i32
        {
            (((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>()).write(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                    .cast::<u8>())
                .wrapping_add(18))
                .cast::<u8>(),
            );
            (((&raw mut sDecorationContext).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                    .cast::<u8>())
                .wrapping_add(34))
                .cast::<u8>(),
            );
        }
        if (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).read()) as i32) == 1i32
        {
            (((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>()).write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10012))
                    .cast::<u8>(),
            );
            (((&raw mut sDecorationContext).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10024))
                    .cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AddDecorationWindow(windowIndex: u8) -> u8 {
    unsafe {
        let mut windowIndex = windowIndex;
        let mut windowId: *mut u8 = core::ptr::null_mut();
        let mut template = crate::ffi::Align4([0u8; 8]);
        windowId = (((&raw mut sDecorMenuWindowIds).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((windowIndex) as i32) as isize);
        if ((windowIndex) as i32) == 0i32 {
            (&raw mut template)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .write_unaligned(
                    ((&raw const sDecorationWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
                );
            (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(
                ((GetMaxWidthInMenuTable(
                    ((&raw const sDecorationMainMenuActions)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((crate::c::div_u32(32u32, 8u32)) as i32),
                )) as u8),
            );
            if (((((&raw mut template).cast::<u8>()).wrapping_add(3)).read()) as i32) > 18i32 {
                (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(18u8);
            }
            (windowId).write(((AddWindow((&raw mut template).cast::<u8>())) as u8));
        } else {
            (windowId).write(
                ((AddWindow(
                    (((&raw const sDecorationWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((windowIndex) as i32) as isize * 8),
                )) as u8),
            );
        }
        DrawStdFrameWithCustomTileAndPalette((windowId).read(), 0u8, 532u16, 14u8);
        ScheduleBgCopyTilemapToVram(0u8);
        return (windowId).read();
    }
}
pub(crate) unsafe extern "C" fn RemoveDecorationWindow(windowIndex: u8) {
    unsafe {
        let mut windowIndex = windowIndex;
        ClearStdWindowAndFrameToTransparent(
            ((((&raw mut sDecorMenuWindowIds).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((windowIndex) as i32) as isize))
            .read(),
            0u8,
        );
        ClearWindowTilemap(
            ((((&raw mut sDecorMenuWindowIds).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((windowIndex) as i32) as isize))
            .read(),
        );
        RemoveWindow(
            ((((&raw mut sDecorMenuWindowIds).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((windowIndex) as i32) as isize))
            .read(),
        );
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn AddDecorationActionsWindow() {
    unsafe {
        let mut windowId: u8 = AddDecorationWindow(0u8);
        PrintMenuTable(
            windowId,
            ((crate::c::div_u32(32u32, 8u32)) as u8),
            ((&raw const sDecorationMainMenuActions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        InitMenuInUpperLeftCornerNormal(
            windowId,
            ((crate::c::div_u32(32u32, 8u32)) as u8),
            ((&raw mut sDecorationActionsCursorPos)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn InitDecorationActionsWindow() {
    unsafe {
        ((&raw mut sDecorationActionsCursorPos)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        LockPlayerFieldControls();
        AddDecorationActionsWindow();
        PrintCurMainMenuDescription();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSecretBaseDecorationMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        InitDecorationActionsWindow();
        (((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>()).write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_add(18))
            .cast::<u8>(),
        );
        (((&raw mut sDecorationContext).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_add(34))
            .cast::<u8>(),
        );
        (((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(8)).write(16u8);
        (((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).write(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleDecorationActionsMenuInput));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPlayerRoomDecorationMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        InitDecorationActionsWindow();
        (((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>()).write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10012))
                .cast::<u8>(),
        );
        (((&raw mut sDecorationContext).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10024))
                .cast::<u8>(),
        );
        (((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(8)).write(12u8);
        (((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).write(1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleDecorationActionsMenuInput));
    }
}
pub(crate) unsafe extern "C" fn HandleDecorationActionsMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let mut menuPos: i8 = ((Menu_GetCursorPos()) as i8);
            'l1: {
                let __sw1 = ((Menu_ProcessInput()) as i32);
                let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
                if !__matched {
                    PlaySE(5u16);
                    (((((((&raw const sDecorationMainMenuActions)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut sDecorationActionsCursorPos)
                            .cast::<u8>()
                            .cast::<u8>())
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .wrapping_add(4))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read())
                    .unwrap_unchecked()(taskId);
                    break 'l1;
                }
                if __sw1 == (-2i32) {
                    ((&raw mut sDecorationActionsCursorPos)
                        .cast::<u8>()
                        .cast::<u8>())
                    .write(Menu_GetCursorPos());
                    if ((menuPos) as i32)
                        != ((((&raw mut sDecorationActionsCursorPos)
                            .cast::<u8>()
                            .cast::<u8>())
                        .read()) as i32)
                    {
                        PrintCurMainMenuDescription();
                    }
                    break 'l1;
                }
                if __sw1 == (-1i32) {
                    PlaySE(5u16);
                    DecorationMenuAction_Cancel(taskId);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintCurMainMenuDescription() {
    unsafe {
        FillWindowPixelBuffer(0u8, 17u8);
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            ((((&raw const sSecretBasePCMenuItemDescriptions)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((&raw mut sDecorationActionsCursorPos)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize,
            ))
            .read(),
            0u8,
            None,
            2u8,
            1u8,
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn DecorationMenuAction_Decorate(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((GetNumOwnedDecorations()) as i32) == 0i32 {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_NoDecorations).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(ReturnToDecorationActionsAfterInvalidSelection),
            );
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(0i16);
            ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).write(0u8);
            SecretBasePC_PrepMenuForSelectingStoredDecors(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn DecorationMenuAction_PutAway(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((HasDecorationsInUse(taskId)) != 0) {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_NoDecorationsInUse).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(ReturnToDecorationActionsAfterInvalidSelection),
            );
        } else {
            RemoveDecorationWindow(0u8);
            ClearDialogWindowAndFrame(0u8, 0u8);
            FadeScreen(1u8, 0i8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ContinuePuttingAwayDecorations));
        }
    }
}
pub(crate) unsafe extern "C" fn DecorationMenuAction_Toss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((GetNumOwnedDecorations()) as i32) == 0i32 {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_NoDecorations).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(ReturnToDecorationActionsAfterInvalidSelection),
            );
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(1i16);
            ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).write(0u8);
            SecretBasePC_PrepMenuForSelectingStoredDecors(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn DecorationMenuAction_Cancel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RemoveDecorationWindow(0u8);
        if !(((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).read()) != 0) {
            ScriptContext_SetupScript((&raw mut SecretBase_EventScript_PCCancel).cast::<u8>());
            DestroyTask(taskId);
        } else {
            ReshowPlayerPC(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ReturnToDecorationActionsAfterInvalidSelection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PrintCurMainMenuDescription();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleDecorationActionsMenuInput));
    }
}
pub(crate) unsafe extern "C" fn SecretBasePC_PrepMenuForSelectingStoredDecors(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        LoadPalette(
            (((&raw const sDecorationMenuPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            208u16,
            32u16,
        );
        ClearDialogWindowAndFrame(0u8, 0u8);
        RemoveDecorationWindow(0u8);
        InitDecorationCategoriesWindow(taskId);
    }
}
pub(crate) unsafe extern "C" fn InitDecorationCategoriesWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut windowId: u8 = AddDecorationWindow(1u8);
        PrintDecorationCategoryMenuItems(taskId);
        InitMenuInUpperLeftCornerNormal(
            windowId,
            9u8,
            ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleDecorationCategoriesMenuInput));
    }
}
pub(crate) unsafe extern "C" fn ReinitDecorationCategoriesWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FillWindowPixelBuffer(
            ((((&raw mut sDecorMenuWindowIds).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .read(),
            17u8,
        );
        PrintDecorationCategoryMenuItems(taskId);
        InitMenuInUpperLeftCornerNormal(
            ((((&raw mut sDecorMenuWindowIds).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .read(),
            9u8,
            ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleDecorationCategoriesMenuInput));
    }
}
pub(crate) unsafe extern "C" fn PrintDecorationCategoryMenuItems(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut windowId: u8 = ((((&raw mut sDecorMenuWindowIds).cast::<u8>()).cast::<u8>())
            .wrapping_offset(1))
        .read();
        let mut isPlayerRoom: u8 =
            (((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).read();
        let mut shouldDisable: u8 = 0u8;
        if (((isPlayerRoom) as i32) == 1i32)
            && (((((data).wrapping_offset(11)).read()) as i32) == 0i32)
        {
            shouldDisable = 1u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((shouldDisable) as i32) == 1i32) && (((i) as i32) != 6i32))
                        && (((i) as i32) != 7i32)
                    {
                        PrintDecorationCategoryMenuItem(
                            windowId,
                            i,
                            8u8,
                            ((((i) as i32).wrapping_mul(16i32)) as u8),
                            1u8,
                            255u8,
                        );
                    } else {
                        PrintDecorationCategoryMenuItem(
                            windowId,
                            i,
                            8u8,
                            ((((i) as i32).wrapping_mul(16i32)) as u8),
                            0u8,
                            255u8,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as i32)
                == 2i32
            {
                (&raw mut gText_Exit).cast::<u8>()
            } else {
                (&raw mut gText_Cancel).cast::<u8>()
            }),
            8u8,
            (((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
            0u8,
            None,
        );
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintDecorationCategoryMenuItem(
    winid: u8,
    category: u8,
    x: u8,
    y: u8,
    disabled: u8,
    speed: u8,
) {
    unsafe {
        let mut winid = winid;
        let mut category = category;
        let mut x = x;
        let mut y = y;
        let mut disabled = disabled;
        let mut speed = speed;
        let mut width: u8 = 0u8;
        let mut str: *mut u8 = core::ptr::null_mut();
        width = ((if ((x) as i32) == 8i32 { 104i32 } else { 96i32 }) as u8);
        y = (y).wrapping_add(1);
        ColorMenuItemString((&raw mut gStringVar4).cast::<u8>(), disabled);
        str = ((&raw mut gStringVar4).cast::<u8>())
            .wrapping_offset(((StringLength((&raw mut gStringVar4).cast::<u8>())) as i32) as isize);
        StringCopy(
            str,
            ((((&raw const sDecorationCategoryNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((category) as i32) as isize))
            .read(),
        );
        AddTextPrinterParameterized(
            winid,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x,
            y,
            speed,
            None,
        );
        str = ConvertIntToDecimalStringN(
            str,
            ((GetNumOwnedDecorationsInCategory(category)) as i32),
            1i32,
            2u8,
        );
        ({
            let __t1 = str;
            str = (str).wrapping_offset(1);
            __t1
        })
        .write(186u8);
        ConvertIntToDecimalStringN(
            str,
            ((((((&raw mut gDecorationInventories).cast::<u8>())
                .wrapping_offset(((category) as i32) as isize * 8))
            .wrapping_add(4))
            .read()) as i32),
            1i32,
            2u8,
        );
        x = ((GetStringRightAlignXOffset(
            1i32,
            (&raw mut gStringVar4).cast::<u8>(),
            ((width) as i32),
        )) as u8);
        AddTextPrinterParameterized(
            winid,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x,
            y,
            speed,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn ColorMenuItemString(str: *mut u8, disabled: u8) {
    unsafe {
        let mut str = str;
        let mut disabled = disabled;
        StringCopy(str, (&raw mut gText_Color161Shadow161).cast::<u8>());
        if ((disabled) as i32) == 1i32 {
            ((str).wrapping_offset(2)).write(4u8);
            ((str).wrapping_offset(5)).write(5u8);
        } else {
            ((str).wrapping_offset(2)).write(2u8);
            ((str).wrapping_offset(5)).write(3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn HandleDecorationCategoriesMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let mut input: i8 = Menu_ProcessInput();
            'l1: {
                let __sw1 = ((input) as i32);
                let __matched = __sw1 == (-1i32) || __sw1 == 8i32 || __sw1 == (-2i32);
                if __sw1 == (-1i32) || __sw1 == 8i32 {
                    PlaySE(5u16);
                    ExitDecorationCategoriesMenu(taskId);
                    break 'l1;
                }
                if __sw1 == (-2i32) {
                    break 'l1;
                }
                if !__matched {
                    PlaySE(5u16);
                    ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>())
                        .write(((input) as u8));
                    SelectDecorationCategory(taskId);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SelectDecorationCategory(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut sNumOwnedDecorationsInCurCategory)
            .cast::<u8>()
            .cast::<u8>())
        .write(GetNumOwnedDecorationsInCategory(
            ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read(),
        ));
        if ((((&raw mut sNumOwnedDecorationsInCurCategory)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            != 0i32
        {
            CondenseDecorationsInCategory(
                ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read(),
            );
            ((&raw mut gCurDecorationItems)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(
                ((((&raw mut gDecorationInventories).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 8,
                ))
                .cast::<*mut u8>())
                .read(),
            );
            IdentifyOwnedDecorationsCurrentlyInUse(taskId);
            ((&raw mut sDecorationsScrollOffset)
                .cast::<u8>()
                .cast::<u16>())
            .write(0u16);
            ((&raw mut sDecorationsCursorPos).cast::<u8>().cast::<u16>()).write(0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ShowDecorationItemsWindow));
        } else {
            RemoveDecorationWindow(1u8);
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_NoDecorations).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(ReturnToDecorationCategoriesAfterInvalidSelection),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ReturnToDecorationCategoriesAfterInvalidSelection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrame(0u8, 0u8);
        InitDecorationCategoriesWindow(taskId);
    }
}
pub(crate) unsafe extern "C" fn ExitDecorationCategoriesMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .read()) as i32)
            != 2i32
        {
            ReturnToActionsMenuFromCategories(taskId);
        } else {
            ExitTraderDecorationMenu(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ReturnToActionsMenuFromCategories(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RemoveDecorationWindow(1u8);
        AddDecorationActionsWindow();
        DrawDialogueFrame(0u8, 0u8);
        PrintCurMainMenuDescription();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleDecorationActionsMenuInput));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowDecorationCategoriesWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        LoadPalette(
            (((&raw const sDecorationMenuPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            208u16,
            32u16,
        );
        ClearDialogWindowAndFrame(0u8, 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .write(2i16);
        ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).write(0u8);
        InitDecorationCategoriesWindow(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyDecorationCategoryName(dest: *mut u8, category: u8) {
    unsafe {
        let mut dest = dest;
        let mut category = category;
        StringCopy(
            dest,
            ((((&raw const sDecorationCategoryNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((category) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn ExitTraderDecorationMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RemoveDecorationWindow(1u8);
        ExitTraderMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn InitDecorationItemsMenuLimits() {
    unsafe {
        ((((&raw mut sDecorationItemsMenu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1312))
        .write(
            ((((((&raw mut sNumOwnedDecorationsInCurCategory)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32)
                .wrapping_add(1i32)) as u8),
        );
        if ((((((&raw mut sDecorationItemsMenu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1312))
        .read()) as i32)
            > 8i32
        {
            ((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1313))
            .write(8u8);
        } else {
            ((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1313))
            .write(
                ((((&raw mut sDecorationItemsMenu)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1312))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InitDecorationItemsMenuScrollAndCursor() {
    unsafe {
        SetCursorWithinListBounds(
            (&raw mut sDecorationsScrollOffset)
                .cast::<u8>()
                .cast::<u16>(),
            (&raw mut sDecorationsCursorPos).cast::<u8>().cast::<u16>(),
            ((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1313))
            .read(),
            ((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1312))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn InitDecorationItemsMenuScrollAndCursor2() {
    unsafe {
        SetCursorScrollWithinListBounds(
            (&raw mut sDecorationsScrollOffset)
                .cast::<u8>()
                .cast::<u16>(),
            (&raw mut sDecorationsCursorPos).cast::<u8>().cast::<u16>(),
            ((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1313))
            .read(),
            ((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1312))
            .read(),
            8u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintDecorationItemMenuItems(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        let mut i: u16 = 0u16;
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((((((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read()) as i32)
            < 6i32)
            || (((((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read()) as i32)
                > 7i32))
            && ((((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).read()) as i32)
                == 1i32))
            && (((((data).wrapping_offset(11)).read()) as i32) == 0i32)
        {
            ColorMenuItemString((&raw mut gStringVar1).cast::<u8>(), 1u8);
        } else {
            ColorMenuItemString((&raw mut gStringVar1).cast::<u8>(), 0u8);
        }
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sDecorationItemsMenu)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1312))
                    .read()) as i32)
                        .wrapping_sub(1i32))
                {
                    break 'l1;
                }
                'l2: {
                    CopyDecorationMenuItemName(
                        ((((((&raw mut sDecorationItemsMenu)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(328))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u8>(),
                        ((((((&raw mut gCurDecorationItems)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u16),
                    );
                    ((((((&raw mut sDecorationItemsMenu)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .write(
                        ((((((&raw mut sDecorationItemsMenu)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(328))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u8>(),
                    );
                    ((((((&raw mut sDecorationItemsMenu)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .write(((i) as i32));
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy(
            ((((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(328))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 24))
            .cast::<u8>(),
            (&raw mut gText_Cancel).cast::<u8>(),
        );
        ((((((&raw mut sDecorationItemsMenu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 8))
        .cast::<*mut u8>())
        .write(
            ((((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(328))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 24))
            .cast::<u8>(),
        );
        ((((((&raw mut sDecorationItemsMenu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i32>())
        .write((-2i32));
        (&raw mut gMultiuseListMenuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sDecorationItemsListMenuTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16)).write(
            ((((&raw mut sDecorMenuWindowIds).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .read(),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(
            ((((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1312))
            .read()) as u16),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).cast::<*mut u8>()).write(
            (((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .cast::<u8>(),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write(
            ((((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1313))
            .read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn CopyDecorationMenuItemName(dest: *mut u8, decoration: u16) {
    unsafe {
        let mut dest = dest;
        let mut decoration = decoration;
        StringCopy(dest, (&raw mut gStringVar1).cast::<u8>());
        StringAppend(
            dest,
            (((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((decoration) as i32) as isize * 32))
            .wrapping_add(1))
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn DecorationItemsMenu_OnCursorMove(
    itemIndex: i32,
    flag: u8,
    menu: *mut u8,
) {
    unsafe {
        let mut itemIndex = itemIndex;
        let mut flag = flag;
        let mut menu = menu;
        if ((flag) as i32) != 1i32 {
            PlaySE(5u16);
        }
        PrintDecorationItemDescription(itemIndex);
    }
}
pub(crate) unsafe extern "C" fn DecorationItemsMenu_PrintDecorationInUse(
    windowId: u8,
    itemIndex: u32,
    y: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut itemIndex = itemIndex;
        let mut y = y;
        if itemIndex != 4294967294u32 {
            if ((IsDecorationIndexInSecretBase((((itemIndex).wrapping_add(1u32)) as u8))) as i32)
                == 1i32
            {
                BlitMenuInfoIcon(
                    windowId,
                    24u8,
                    92u16,
                    ((((y) as i32).wrapping_add(2i32)) as u16),
                );
            } else {
                if ((IsDecorationIndexInPlayersRoom((((itemIndex).wrapping_add(1u32)) as u8)))
                    as i32)
                    == 1i32
                {
                    BlitMenuInfoIcon(
                        windowId,
                        25u8,
                        92u16,
                        ((((y) as i32).wrapping_add(2i32)) as u16),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AddDecorationItemsScrollIndicators() {
    unsafe {
        if ((((((&raw mut sDecorationItemsMenu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1314))
        .read()) as i32)
            == 255i32
        {
            ((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1314))
            .write(AddScrollIndicatorArrowPairParameterized(
                2u32,
                60i32,
                12i32,
                148i32,
                ((((((&raw mut sDecorationItemsMenu)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1312))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sDecorationItemsMenu)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1313))
                        .read()) as i32),
                    ),
                110i32,
                110i32,
                (&raw mut sDecorationsScrollOffset)
                    .cast::<u8>()
                    .cast::<u16>(),
            ));
        }
    }
}
pub(crate) unsafe extern "C" fn RemoveDecorationItemsScrollIndicators() {
    unsafe {
        if ((((((&raw mut sDecorationItemsMenu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1314))
        .read()) as i32)
            != 255i32
        {
            RemoveScrollIndicatorArrowPair(
                ((((&raw mut sDecorationItemsMenu)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1314))
                .read(),
            );
            ((((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1314))
            .write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn AddDecorationItemsWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        AddDecorationWindow(1u8);
        InitDecorationItemsWindow(taskId);
    }
}
pub(crate) unsafe extern "C" fn InitDecorationItemsWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        AddDecorationWindow(3u8);
        ShowDecorationCategorySummaryWindow(
            ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read(),
        );
        ((&raw mut sDecorationItemsMenu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(1316u32));
        ((((&raw mut sDecorationItemsMenu)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1314))
        .write(255u8);
        InitDecorationItemsMenuLimits();
        InitDecorationItemsMenuScrollAndCursor();
        InitDecorationItemsMenuScrollAndCursor2();
        PrintDecorationItemMenuItems(taskId);
        ((data).wrapping_offset(13)).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                ((&raw mut sDecorationsScrollOffset)
                    .cast::<u8>()
                    .cast::<u16>())
                .read(),
                ((&raw mut sDecorationsCursorPos).cast::<u8>().cast::<u16>()).read(),
            )) as i16),
        );
        AddDecorationItemsScrollIndicators();
    }
}
pub(crate) unsafe extern "C" fn ShowDecorationItemsWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        InitDecorationItemsWindow(taskId);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleDecorationItemsMenuInput));
    }
}
pub(crate) unsafe extern "C" fn HandleDecorationItemsMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        let mut input: i32 = 0i32;
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            input = ListMenu_ProcessInput(((((data).wrapping_offset(13)).read()) as u8));
            ListMenuGetScrollAndRow(
                ((((data).wrapping_offset(13)).read()) as u8),
                (&raw mut sDecorationsScrollOffset)
                    .cast::<u8>()
                    .cast::<u16>(),
                (&raw mut sDecorationsCursorPos).cast::<u8>().cast::<u16>(),
            );
            'l1: {
                let __sw1 = input;
                let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
                if __sw1 == (-1i32) {
                    break 'l1;
                }
                if __sw1 == (-2i32) {
                    PlaySE(5u16);
                    (((((((&raw const sSecretBasePC_SelectedDecorationActions)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((data).wrapping_offset(11)).read()) as i32) as isize * 8,
                    ))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .wrapping_offset(1))
                    .read())
                    .unwrap_unchecked()(taskId);
                    break 'l1;
                }
                if !__matched {
                    PlaySE(5u16);
                    ((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>())
                        .write(((input) as u8));
                    RemoveDecorationItemsScrollIndicators();
                    DestroyListMenuTask(
                        ((((data).wrapping_offset(13)).read()) as u8),
                        (&raw mut sDecorationsScrollOffset)
                            .cast::<u8>()
                            .cast::<u16>(),
                        (&raw mut sDecorationsCursorPos).cast::<u8>().cast::<u16>(),
                    );
                    RemoveDecorationWindow(1u8);
                    RemoveDecorationItemsOtherWindows();
                    Free(
                        ((&raw mut sDecorationItemsMenu)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((((((&raw const sSecretBasePC_SelectedDecorationActions)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((data).wrapping_offset(11)).read()) as i32) as isize * 8,
                    ))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read())
                    .unwrap_unchecked()(taskId);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowDecorationCategorySummaryWindow(category: u8) {
    unsafe {
        let mut category = category;
        PrintDecorationCategoryMenuItem(AddDecorationWindow(2u8), category, 0u8, 0u8, 0u8, 0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintDecorationItemDescription(itemIndex: i32) {
    unsafe {
        let mut itemIndex = itemIndex;
        let mut windowId: u8 = 0u8;
        let mut str: *mut u8 = core::ptr::null_mut();
        windowId = ((((&raw mut sDecorMenuWindowIds).cast::<u8>()).cast::<u8>())
            .wrapping_offset(3))
        .read();
        FillWindowPixelBuffer(windowId, 17u8);
        if ((itemIndex) as u32)
            >= ((((&raw mut sNumOwnedDecorationsInCurCategory)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as u32)
        {
            str = (&raw mut gText_GoBackPrevMenu).cast::<u8>();
        } else {
            str = (((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gCurDecorationItems)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((itemIndex) as isize))
                    .read()) as i32) as isize
                        * 32,
                ))
            .wrapping_add(24)
            .cast::<*mut u8>())
            .read();
        }
        AddTextPrinterParameterized(windowId, 1u8, str, 0u8, 1u8, 0u8, None);
    }
}
pub(crate) unsafe extern "C" fn RemoveDecorationItemsOtherWindows() {
    unsafe {
        RemoveDecorationWindow(3u8);
        RemoveDecorationWindow(2u8);
    }
}
pub(crate) unsafe extern "C" fn IsDecorationIndexInSecretBase(idx: u8) -> u8 {
    unsafe {
        let mut idx = idx;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sSecretBaseItemsIndicesBuffer).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((idx) as i32)
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsDecorationIndexInPlayersRoom(idx: u8) -> u8 {
    unsafe {
        let mut idx = idx;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(12u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sPlayerRoomItemsIndicesBuffer).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((idx) as i32)
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IdentifyOwnedDecorationsCurrentlyInUseInternal(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut k: u16 = 0u16;
        let mut count: u16 = 0u16;
        count = 0u16;
        crate::c::memset(
            ((&raw mut sSecretBaseItemsIndicesBuffer).cast::<u8>()).cast::<u8>(),
            0i32,
            16u32,
        );
        crate::c::memset(
            ((&raw mut sPlayerRoomItemsIndicesBuffer).cast::<u8>()).cast::<u8>(),
            0i32,
            12u32,
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(6812))
                    .cast::<u8>())
                    .wrapping_add(18))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        {
                            j = 0u16;
                            'l3: loop {
                                if !(((j) as i32)
                                    < ((((((&raw mut gDecorationInventories).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut sCurDecorationCategory)
                                                .cast::<u8>()
                                                .cast::<u8>())
                                            .read())
                                                as i32)
                                                as isize
                                                * 8,
                                        ))
                                    .wrapping_add(4))
                                    .read()) as i32))
                                {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((&raw mut gCurDecorationItems)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read()) as i32)
                                        == ((((((((((&raw mut gSaveBlock1Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(6812))
                                        .cast::<u8>())
                                        .wrapping_add(18))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32)
                                    {
                                        {
                                            k = 0u16;
                                            'l5: loop {
                                                if !(((((k) as i32)) < (((count) as i32))) && ((((((((&raw mut sSecretBaseItemsIndicesBuffer).cast::<u8>()).cast::<u8>()).wrapping_offset((((k) as i32)) as isize)).read()) as i32)) != ((((j) as i32))).wrapping_add(1i32))) { break 'l5; }
                                                'l6: {}
                                                k = (k).wrapping_add(1);
                                            }
                                        }
                                        if ((k) as i32) == ((count) as i32) {
                                            ((((&raw mut sSecretBaseItemsIndicesBuffer)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset(((count) as i32) as isize))
                                            .write(((((j) as i32).wrapping_add(1i32)) as u8));
                                            count = (count).wrapping_add(1);
                                            break 'l3;
                                        }
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        count = 0u16;
        {
            i = 0u16;
            'l7: loop {
                if !(((i) as u32) < crate::c::div_u32(12u32, 1u32)) {
                    break 'l7;
                }
                'l8: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10012))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        {
                            j = 0u16;
                            'l9: loop {
                                if !(((j) as i32)
                                    < ((((((&raw mut gDecorationInventories).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut sCurDecorationCategory)
                                                .cast::<u8>()
                                                .cast::<u8>())
                                            .read())
                                                as i32)
                                                as isize
                                                * 8,
                                        ))
                                    .wrapping_add(4))
                                    .read()) as i32))
                                {
                                    break 'l9;
                                }
                                'l10: {
                                    if (((((((&raw mut gCurDecorationItems)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read()) as i32)
                                        == ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(10012))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read())
                                            as i32))
                                        && (((IsDecorationIndexInSecretBase(
                                            ((((j) as i32).wrapping_add(1i32)) as u8),
                                        )) as i32)
                                            != 1i32)
                                    {
                                        {
                                            k = 0u16;
                                            'l11: loop {
                                                if !(((((k) as i32)) < (((count) as i32))) && ((((((((&raw mut sPlayerRoomItemsIndicesBuffer).cast::<u8>()).cast::<u8>()).wrapping_offset((((k) as i32)) as isize)).read()) as i32)) != ((((j) as i32))).wrapping_add(1i32))) { break 'l11; }
                                                'l12: {}
                                                k = (k).wrapping_add(1);
                                            }
                                        }
                                        if ((k) as i32) == ((count) as i32) {
                                            ((((&raw mut sPlayerRoomItemsIndicesBuffer)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset(((count) as i32) as isize))
                                            .write(((((j) as i32).wrapping_add(1i32)) as u8));
                                            count = (count).wrapping_add(1);
                                            break 'l9;
                                        }
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IdentifyOwnedDecorationsCurrentlyInUse(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        IdentifyOwnedDecorationsCurrentlyInUseInternal(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSelectedDecorInThePC() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sSecretBaseItemsIndicesBuffer).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == (((((&raw mut sDecorationsScrollOffset)
                            .cast::<u8>()
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(
                                ((((&raw mut sDecorationsCursorPos).cast::<u8>().cast::<u16>())
                                    .read()) as i32),
                            ))
                        .wrapping_add(1i32)
                    {
                        return 0u8;
                    }
                    if (((i) as u32) < crate::c::div_u32(12u32, 1u32))
                        && (((((((&raw mut sPlayerRoomItemsIndicesBuffer).cast::<u8>())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == (((((&raw mut sDecorationsScrollOffset)
                                .cast::<u8>()
                                .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(
                                    ((((&raw mut sDecorationsCursorPos).cast::<u8>().cast::<u16>())
                                        .read()) as i32),
                                ))
                            .wrapping_add(1i32))
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
pub(crate) unsafe extern "C" fn Task_ShowDecorationItemsWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        AddDecorationWindow(1u8);
        ShowDecorationItemsWindow(taskId);
    }
}
pub(crate) unsafe extern "C" fn DontTossDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrame(0u8, 0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ShowDecorationItemsWindow));
    }
}
pub(crate) unsafe extern "C" fn ReturnToDecorationItemsAfterInvalidSelection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            ClearDialogWindowAndFrame(0u8, 0u8);
            AddDecorationWindow(1u8);
            ShowDecorationItemsWindow(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn DecorationItemsMenuAction_Cancel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        RemoveDecorationItemsScrollIndicators();
        RemoveDecorationItemsOtherWindows();
        DestroyListMenuTask(
            ((((data).wrapping_offset(13)).read()) as u8),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        Free(
            ((&raw mut sDecorationItemsMenu)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ReinitDecorationCategoriesWindow(taskId);
    }
}
pub(crate) unsafe extern "C" fn SetInitialPositions(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read(),
        );
        PlayerGetDestCoords(
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>(),
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1),
        );
    }
}
pub(crate) unsafe extern "C" fn WarpToInitialPosition(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DrawWholeMapView();
        SetWarpDestination(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<i8>())
                .read(),
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read(),
            (-1i8),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i8),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as i8),
        );
        WarpIntoMap();
    }
}
pub(crate) unsafe extern "C" fn GetDecorationElevation(decoration: u8, tileIndex: u8) -> u16 {
    unsafe {
        let mut decoration = decoration;
        let mut tileIndex = tileIndex;
        let mut elevation: u16 = 65535u16;
        'l1: {
            let __sw1 = ((decoration) as i32);
            let __matched = __sw1 == 38i32 || __sw1 == 34i32;
            if __sw1 == 38i32 {
                elevation = ((((((((&raw const sDecorationStandElevations)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((tileIndex) as i32) as isize))
                .read()) as i32)
                    << 12) as u16);
                return elevation;
            }
            if __sw1 == 34i32 {
                elevation = ((((((((&raw const sDecorationSlideElevation)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((tileIndex) as i32) as isize))
                .read()) as i32)
                    << 12) as u16);
                return elevation;
            }
            if !__matched {
                return elevation;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn ShowDecorationOnMap_(
    mapX: u16,
    mapY: u16,
    decWidth: u8,
    decHeight: u8,
    decoration: u16,
) {
    unsafe {
        let mut mapX = mapX;
        let mut mapY = mapY;
        let mut decWidth = decWidth;
        let mut decHeight = decHeight;
        let mut decoration = decoration;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut attributes: u16 = 0u16;
        let mut impassableFlag: u16 = 0u16;
        let mut overlapsWall: u16 = 0u16;
        let mut elevation: u16 = 0u16;
        {
            j = 0u16;
            'l1: loop {
                if !(((j) as i32) < ((decHeight) as i32)) {
                    break 'l1;
                }
                'l2: {
                    y = ((((((mapY) as i32).wrapping_sub(((decHeight) as i32))).wrapping_add(1i32))
                        .wrapping_add(((j) as i32))) as i16);
                    {
                        i = 0u16;
                        'l3: loop {
                            if !(((i) as i32) < ((decWidth) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                x = ((((mapX) as i32).wrapping_add(((i) as i32))) as i16);
                                attributes = GetMetatileAttributesById(
                                    (((512i32).wrapping_add(
                                        (((((((((&raw const gDecorations)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((decoration) as i32) as isize * 32))
                                        .wrapping_add(28)
                                        .cast::<*mut u16>())
                                        .read())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_mul(((decWidth) as i32)))
                                                .wrapping_add(((i) as i32)))
                                                as isize,
                                        ))
                                        .read()) as i32),
                                    )) as u16),
                                );
                                if (((MetatileBehavior_IsSecretBaseImpassable(
                                    (((((attributes) as i32) & 255i32) >> 0) as u8),
                                )) as i32)
                                    == 1i32)
                                    || (((((((((&raw const gDecorations)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((decoration) as i32) as isize * 32))
                                    .wrapping_add(17))
                                    .read()) as i32)
                                        != 1i32)
                                        && ((((attributes) as i32) >> 12) != 0i32))
                                {
                                    impassableFlag = 3072u16;
                                } else {
                                    impassableFlag = 0u16;
                                }
                                if ((((((((&raw const gDecorations).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((decoration) as i32) as isize * 32))
                                .wrapping_add(17))
                                .read()) as i32)
                                    != 3i32)
                                    && (((MetatileBehavior_IsSecretBaseNorthWall(
                                        ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32)))
                                            as u8),
                                    )) as i32)
                                        == 1i32)
                                {
                                    overlapsWall = 1u16;
                                } else {
                                    overlapsWall = 0u16;
                                }
                                elevation = GetDecorationElevation(
                                    ((((&raw const gDecorations).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(((decoration) as i32) as isize * 32))
                                    .read(),
                                    (((((j) as i32).wrapping_mul(((decWidth) as i32)))
                                        .wrapping_add(((i) as i32)))
                                        as u8),
                                );
                                if ((elevation) as i32) != 65535i32 {
                                    MapGridSetMetatileEntryAt(
                                        ((x) as i32),
                                        ((y) as i32),
                                        ((((((((((((&raw const gDecorations)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((decoration) as i32) as isize * 32))
                                        .wrapping_add(28)
                                        .cast::<*mut u16>())
                                        .read())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_mul(((decWidth) as i32)))
                                                .wrapping_add(((i) as i32)))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            .wrapping_add((512i32 | ((overlapsWall) as i32)))
                                            | ((impassableFlag) as i32))
                                            | ((elevation) as i32))
                                            as u16),
                                    );
                                } else {
                                    MapGridSetMetatileIdAt(
                                        ((x) as i32),
                                        ((y) as i32),
                                        (((((((((((&raw const gDecorations)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((decoration) as i32) as isize * 32))
                                        .wrapping_add(28)
                                        .cast::<*mut u16>())
                                        .read())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_mul(((decWidth) as i32)))
                                                .wrapping_add(((i) as i32)))
                                                as isize,
                                        ))
                                        .read()) as i32)
                                            .wrapping_add((512i32 | ((overlapsWall) as i32)))
                                            | ((impassableFlag) as i32))
                                            as u16),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowDecorationOnMap(mapX: u16, mapY: u16, decoration: u16) {
    unsafe {
        let mut mapX = mapX;
        let mut mapY = mapY;
        let mut decoration = decoration;
        'l1: {
            let __sw1 = (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((decoration) as i32) as isize * 32))
            .wrapping_add(18))
            .read()) as i32);
            if __sw1 == 0i32 {
                ShowDecorationOnMap_(mapX, mapY, 1u8, 1u8, decoration);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ShowDecorationOnMap_(mapX, mapY, 2u8, 1u8, decoration);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ShowDecorationOnMap_(mapX, mapY, 3u8, 1u8, decoration);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ShowDecorationOnMap_(mapX, mapY, 4u8, 2u8, decoration);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ShowDecorationOnMap_(mapX, mapY, 2u8, 2u8, decoration);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ShowDecorationOnMap_(mapX, mapY, 1u8, 2u8, decoration);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ShowDecorationOnMap_(mapX, mapY, 1u8, 3u8, decoration);
                break 'l1;
            }
            if __sw1 == 7i32 {
                ShowDecorationOnMap_(mapX, mapY, 2u8, 4u8, decoration);
                break 'l1;
            }
            if __sw1 == 8i32 {
                ShowDecorationOnMap_(mapX, mapY, 3u8, 3u8, decoration);
                break 'l1;
            }
            if __sw1 == 9i32 {
                ShowDecorationOnMap_(mapX, mapY, 3u8, 2u8, decoration);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDecoration() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 14i32) {
                    break 'l1;
                }
                'l2: {
                    if ((FlagGet((((174i32).wrapping_add(((i) as i32))) as u16))) as i32) == 1i32 {
                        FlagClear((((174i32).wrapping_add(((i) as i32))) as u16));
                        {
                            j = 0u8;
                            'l3: loop {
                                if !(((j) as i32)
                                    < ((((((&raw mut gMapHeader).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                    .read())
                                    .read()) as i32))
                                {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((((((&raw mut gMapHeader).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((j) as i32) as isize * 24))
                                    .wrapping_add(20)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        == (174i32).wrapping_add(((i) as i32))
                                    {
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        VarSet(
                            (((16400i32).wrapping_add(
                                ((((((((((&raw mut gMapHeader).cast::<u8>())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((j) as i32) as isize * 24))
                                .wrapping_add(1))
                                .read()) as i32)
                                    .wrapping_sub(240i32),
                            )) as u16),
                            ((((((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>())
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(28)
                            .cast::<*mut u16>())
                            .read())
                            .read(),
                        );
                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
                            (((((((((&raw mut gMapHeader).cast::<u8>())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((j) as i32) as isize * 24))
                            .read()) as u16),
                        );
                        ((&raw mut gSpecialVar_0x8006).cast::<u16>())
                            .write(((&raw mut sCurDecorMapX).cast::<u8>().cast::<u16>()).read());
                        ((&raw mut gSpecialVar_0x8007).cast::<u16>())
                            .write(((&raw mut sCurDecorMapY).cast::<u8>().cast::<u16>()).read());
                        TrySpawnObjectEvent(
                            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .wrapping_add(1)
                            .cast::<i8>())
                            .read()) as u8),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<i8>())
                            .read()) as u8),
                        );
                        TryMoveObjectEventToMapCoords(
                            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .wrapping_add(1)
                            .cast::<i8>())
                            .read()) as u8),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<i8>())
                            .read()) as u8),
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i16),
                            ((((&raw mut gSpecialVar_0x8007).cast::<u16>()).read()) as i16),
                        );
                        TryOverrideObjectEventTemplateCoords(
                            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .wrapping_add(1)
                            .cast::<i8>())
                            .read()) as u8),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<i8>())
                            .read()) as u8),
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HasDecorationSpace() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(8)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>())
                        .read())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DecorationItemsMenuAction_AttemptPlace(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).read()) as i32)
            == 1i32)
            && (((((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read()) as i32)
                != 6i32))
            && (((((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read()) as i32)
                != 7i32)
        {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_CantPlaceInRoom).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(ReturnToDecorationItemsAfterInvalidSelection),
            );
        } else {
            if ((IsSelectedDecorInThePC()) as i32) == 1i32 {
                if ((HasDecorationSpace()) as i32) == 1i32 {
                    FadeScreen(1u8, 0i8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_PlaceDecoration));
                } else {
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(8)).read())
                            as i32),
                        1i32,
                        2u8,
                    );
                    if (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).read())
                        as i32)
                        == 0i32
                    {
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            (&raw mut gText_NoMoreDecorations).cast::<u8>(),
                        );
                    } else {
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            (&raw mut gText_NoMoreDecorations2).cast::<u8>(),
                        );
                    }
                    DisplayItemMessageOnField(
                        taskId,
                        (&raw mut gStringVar4).cast::<u8>(),
                        Some(ReturnToDecorationItemsAfterInvalidSelection),
                    );
                }
            } else {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_InUseAlready).cast::<u8>(),
                );
                DisplayItemMessageOnField(
                    taskId,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(ReturnToDecorationItemsAfterInvalidSelection),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PlaceDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetInitialPositions(taskId);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                ConfigureCameraObjectForPlacingDecoration(
                    (&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>(),
                    ((((&raw mut gCurDecorationItems)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    ))
                    .read(),
                );
                SetUpDecorationShape(taskId);
                SetUpPlacingDecorationPlayerAvatar(
                    taskId,
                    (&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>(),
                );
                FadeInFromBlack();
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(2i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12))
                    .write(0i16);
                    ContinueDecorating(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ConfigureCameraObjectForPlacingDecoration(
    data: *mut u8,
    decor: u8,
) {
    unsafe {
        let mut data = data;
        let mut decor = decor;
        ((&raw mut sDecor_CameraSpriteObjectIdx1)
            .cast::<u8>()
            .cast::<u8>())
        .write(
            (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gFieldCamera).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u32>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .read()) as u8),
        );
        (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(((gpu_pal_decompress_alloc_tag_and_upload(data, decor)) as u32));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gFieldCamera).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u32>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gFieldCamera).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(InitializePuttingAwayCursorSprite));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gFieldCamera).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(
            (((((((&raw const sDecorationMovementInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((data).cast::<*mut u8>()).read()).wrapping_add(18)).read()) as i32)
                        as isize
                        * 4,
                ))
            .wrapping_add(2))
            .read()) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gFieldCamera).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            (((((((&raw const sDecorationMovementInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((data).cast::<*mut u8>()).read()).wrapping_add(18)).read()) as i32)
                        as isize
                        * 4,
                ))
            .wrapping_add(3))
            .read()) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SetUpPlacingDecorationPlayerAvatar(taskId: u8, data: *mut u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data = data;
        let mut x: u8 = 0u8;
        x = (((((16i32).wrapping_mul(
            (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u8) as i32),
        ))
        .wrapping_add(
            (((((((&raw const sDecorationMovementInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((data).cast::<*mut u8>()).read()).wrapping_add(18)).read()) as i32)
                        as isize
                        * 4,
                ))
            .wrapping_add(2))
            .read()) as i32),
        ))
        .wrapping_sub(
            (8i32).wrapping_mul(
                (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u8) as i32)
                    .wrapping_sub(1i32),
            ),
        )) as u8);
        if ((((((((data).cast::<*mut u8>()).read()).wrapping_add(18)).read()) as i32) == 2i32)
            || (((((((data).cast::<*mut u8>()).read()).wrapping_add(18)).read()) as i32) == 8i32))
            || (((((((data).cast::<*mut u8>()).read()).wrapping_add(18)).read()) as i32) == 9i32)
        {
            x = ((((x) as i32).wrapping_sub(8i32)) as u8);
        }
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 0i32
        {
            ((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .write(CreateObjectGraphicsSprite(
                193u16,
                Some(SpriteCallbackDummy),
                ((x) as i16),
                72i16,
                0u8,
            ));
        } else {
            ((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .write(CreateObjectGraphicsSprite(
                194u16,
                Some(SpriteCallbackDummy),
                ((x) as i16),
                72i16,
                0u8,
            ));
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx2)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ),
        );
        ((&raw mut sDecor_CameraSpriteObjectIdx1)
            .cast::<u8>()
            .cast::<u8>())
        .write(
            (((((&raw mut gFieldCamera).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn SetUpDecorationShape(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gCurDecorationItems)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    ))
                    .read()) as i32) as isize
                        * 32,
                ))
            .wrapping_add(18))
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(1i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(2i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(3i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(4i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(2i16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(2i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(2i16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(1i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(2i16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(1i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(3i16);
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(2i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(4i16);
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(3i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(3i16);
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(3i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(2i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AttemptPlaceDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(1i16);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(1i16);
        ResetCursorMovement();
        AttemptPlaceDecoration_(taskId);
    }
}
pub(crate) unsafe extern "C" fn AttemptCancelPlaceDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(1i16);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(1i16);
        ResetCursorMovement();
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_CancelDecorating).cast::<u8>(),
        );
        DisplayItemMessageOnField(
            taskId,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(CancelDecoratingPrompt),
        );
    }
}
pub(crate) unsafe extern "C" fn IsSecretBaseTrainerSpot(behaviorAt: u8, layerType: u16) -> u8 {
    unsafe {
        let mut behaviorAt = behaviorAt;
        let mut layerType = layerType;
        if !((((MetatileBehavior_IsSecretBaseTrainerSpot(behaviorAt)) as i32) == 1i32)
            && (((layerType) as i32) == 0i32))
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn IsntInitialPosition(
    taskId: u8,
    x: i16,
    y: i16,
    layerType: u16,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut x = x;
        let mut y = y;
        let mut layerType = layerType;
        if ((((x) as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                .wrapping_add(7i32))
            && (((y) as i32)
                == ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
                    .wrapping_add(7i32)))
            && (((layerType) as i32) != 0i32)
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn IsFloorOrBoardAndHole(behaviorAt: u16, decoration: *mut u8) -> u8 {
    unsafe {
        let mut behaviorAt = behaviorAt;
        let mut decoration = decoration;
        if ((MetatileBehavior_IsSecretBaseTrainerSpot(((behaviorAt) as u8))) as i32) != 1i32 {
            if ((((decoration).read()) as i32) == 33i32)
                && (((MetatileBehavior_IsSecretBaseHole(((behaviorAt) as u8))) as i32) == 1i32)
            {
                return 1u8;
            }
            if (MetatileBehavior_IsNormal(((behaviorAt) as u8))) != 0 {
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CanPlaceDecoration(taskId: u8, decoration: *mut u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut decoration = decoration;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut behaviorAt: u8 = 0u8;
        let mut layerType: u16 = 0u16;
        let mut mapY: u8 = 0u8;
        let mut mapX: u8 = 0u8;
        let mut curY: i16 = 0i16;
        let mut curX: i16 = 0i16;
        mapY = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .read()) as u8);
        mapX = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as u8);
        'l1: {
            let __sw1 = ((((decoration).wrapping_add(17)).read()) as i32);
            if __sw1 == 0i32 || __sw1 == 1i32 {
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < ((mapY) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            curY = ((((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_sub(((i) as i32)))
                                as i16);
                            {
                                j = 0u8;
                                'l4: loop {
                                    if !(((j) as i32) < ((mapX) as i32)) {
                                        break 'l4;
                                    }
                                    'l5: {
                                        curX = (((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .read())
                                            as i32)
                                            .wrapping_add(((j) as i32)))
                                            as i16);
                                        behaviorAt = ((MapGridGetMetatileBehaviorAt(
                                            ((curX) as i32),
                                            ((curY) as i32),
                                        ))
                                            as u8);
                                        layerType = ((((GetMetatileAttributesById(
                                            (((512i32).wrapping_add(
                                                ((((((decoration)
                                                    .wrapping_add(28)
                                                    .cast::<*mut u16>())
                                                .read())
                                                .wrapping_offset(
                                                    ((((((mapY) as i32).wrapping_sub(1i32))
                                                        .wrapping_sub(((i) as i32)))
                                                    .wrapping_mul(((mapX) as i32)))
                                                    .wrapping_add(((j) as i32)))
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32),
                                            )) as u16),
                                        ))
                                            as i32)
                                            & 61440i32)
                                            as u16);
                                        if !((IsFloorOrBoardAndHole(
                                            ((behaviorAt) as u16),
                                            decoration,
                                        )) != 0)
                                        {
                                            return 0u8;
                                        }
                                        if !((IsntInitialPosition(taskId, curX, curY, layerType))
                                            != 0)
                                        {
                                            return 0u8;
                                        }
                                        behaviorAt = GetObjectEventIdByPosition(
                                            ((curX) as u16),
                                            ((curY) as u16),
                                            0u8,
                                        );
                                        if (((behaviorAt) as i32) != 0i32)
                                            && (((behaviorAt) as i32) != 16i32)
                                        {
                                            return 0u8;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0u8;
                    'l6: loop {
                        if !(((i) as i32) < ((mapY) as i32).wrapping_sub(1i32)) {
                            break 'l6;
                        }
                        'l7: {
                            curY = ((((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_sub(((i) as i32)))
                                as i16);
                            {
                                j = 0u8;
                                'l8: loop {
                                    if !(((j) as i32) < ((mapX) as i32)) {
                                        break 'l8;
                                    }
                                    'l9: {
                                        curX = (((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .read())
                                            as i32)
                                            .wrapping_add(((j) as i32)))
                                            as i16);
                                        behaviorAt = ((MapGridGetMetatileBehaviorAt(
                                            ((curX) as i32),
                                            ((curY) as i32),
                                        ))
                                            as u8);
                                        layerType = ((((GetMetatileAttributesById(
                                            (((512i32).wrapping_add(
                                                ((((((decoration)
                                                    .wrapping_add(28)
                                                    .cast::<*mut u16>())
                                                .read())
                                                .wrapping_offset(
                                                    ((((((mapY) as i32).wrapping_sub(1i32))
                                                        .wrapping_sub(((i) as i32)))
                                                    .wrapping_mul(((mapX) as i32)))
                                                    .wrapping_add(((j) as i32)))
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32),
                                            )) as u16),
                                        ))
                                            as i32)
                                            & 61440i32)
                                            as u16);
                                        if (!((MetatileBehavior_IsNormal(behaviorAt)) != 0))
                                            && (!((IsSecretBaseTrainerSpot(behaviorAt, layerType))
                                                != 0))
                                        {
                                            return 0u8;
                                        }
                                        if !((IsntInitialPosition(taskId, curX, curY, layerType))
                                            != 0)
                                        {
                                            return 0u8;
                                        }
                                        if ((GetObjectEventIdByPosition(
                                            ((curX) as u16),
                                            ((curY) as u16),
                                            0u8,
                                        )) as i32)
                                            != 16i32
                                        {
                                            return 0u8;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                curY = (((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_sub(((mapY) as i32)))
                .wrapping_add(1i32)) as i16);
                {
                    j = 0u8;
                    'l10: loop {
                        if !(((j) as i32) < ((mapX) as i32)) {
                            break 'l10;
                        }
                        'l11: {
                            curX = (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_add(((j) as i32)))
                                as i16);
                            behaviorAt =
                                ((MapGridGetMetatileBehaviorAt(((curX) as i32), ((curY) as i32)))
                                    as u8);
                            layerType = ((((GetMetatileAttributesById(
                                (((512i32).wrapping_add(
                                    ((((((decoration).wrapping_add(28).cast::<*mut u16>()).read())
                                        .wrapping_offset(((j) as i32) as isize))
                                    .read()) as i32),
                                )) as u16),
                            )) as i32)
                                & 61440i32) as u16);
                            if (!((MetatileBehavior_IsNormal(behaviorAt)) != 0))
                                && (!((MetatileBehavior_IsSecretBaseNorthWall(behaviorAt)) != 0))
                            {
                                return 0u8;
                            }
                            if !((IsntInitialPosition(taskId, curX, curY, layerType)) != 0) {
                                return 0u8;
                            }
                            behaviorAt =
                                GetObjectEventIdByPosition(((curX) as u16), ((curY) as u16), 0u8);
                            if (((behaviorAt) as i32) != 0i32) && (((behaviorAt) as i32) != 16i32) {
                                return 0u8;
                            }
                        }
                        j = (j).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    i = 0u8;
                    'l12: loop {
                        if !(((i) as i32) < ((mapY) as i32)) {
                            break 'l12;
                        }
                        'l13: {
                            curY = ((((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_sub(((i) as i32)))
                                as i16);
                            {
                                j = 0u8;
                                'l14: loop {
                                    if !(((j) as i32) < ((mapX) as i32)) {
                                        break 'l14;
                                    }
                                    'l15: {
                                        curX = (((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .read())
                                            as i32)
                                            .wrapping_add(((j) as i32)))
                                            as i16);
                                        if !((MetatileBehavior_IsSecretBaseNorthWall(
                                            ((MapGridGetMetatileBehaviorAt(
                                                ((curX) as i32),
                                                ((curY) as i32),
                                            )) as u8),
                                        )) != 0)
                                        {
                                            return 0u8;
                                        }
                                        if MapGridGetMetatileIdAt(
                                            ((curX) as i32),
                                            ((curY) as i32).wrapping_add(1i32),
                                        ) == 652i32
                                        {
                                            return 0u8;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                curY = ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read();
                {
                    j = 0u8;
                    'l16: loop {
                        if !(((j) as i32) < ((mapX) as i32)) {
                            break 'l16;
                        }
                        'l17: {
                            curX = (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_add(((j) as i32)))
                                as i16);
                            behaviorAt =
                                ((MapGridGetMetatileBehaviorAt(((curX) as i32), ((curY) as i32)))
                                    as u8);
                            if ((((decoration).wrapping_add(18)).read()) as i32) == 5i32 {
                                if !((MetatileBehavior_HoldsLargeDecoration(behaviorAt)) != 0) {
                                    return 0u8;
                                }
                            } else {
                                if !((MetatileBehavior_HoldsSmallDecoration(behaviorAt)) != 0) {
                                    if !((MetatileBehavior_HoldsLargeDecoration(behaviorAt)) != 0) {
                                        return 0u8;
                                    }
                                }
                            }
                            if ((GetObjectEventIdByPosition(((curX) as u16), ((curY) as u16), 0u8))
                                as i32)
                                != 16i32
                            {
                                return 0u8;
                            }
                        }
                        j = (j).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn AttemptPlaceDecoration_(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((CanPlaceDecoration(
            taskId,
            (((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gCurDecorationItems)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read()) as i32) as isize
                    * 32,
            ),
        )) as i32)
            == 1i32
        {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PlaceItHere).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(PlaceDecorationPrompt),
            );
        } else {
            PlaySE(32u16);
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_CantBePlacedHere).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(CantPlaceDecorationPrompt),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PlaceDecorationPrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayYesNoMenuDefaultYes();
        DoYesNoFuncWithChoice(
            taskId,
            (&raw const sPlaceDecorationYesNoFunctions)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn PlaceDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrame(0u8, 0u8);
        PlaceDecoration_(taskId);
        if (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gCurDecorationItems)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read()) as i32) as isize
                    * 32,
            ))
        .wrapping_add(17))
        .read()) as i32)
            != 4i32
        {
            ShowDecorationOnMap(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u16),
                ((((((&raw mut gCurDecorationItems)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read()) as u16),
            );
        } else {
            ((&raw mut sCurDecorMapX).cast::<u8>().cast::<u16>()).write(
                (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(7i32)) as u16),
            );
            ((&raw mut sCurDecorMapY).cast::<u8>().cast::<u16>()).write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_sub(7i32)) as u16),
            );
            ScriptContext_SetupScript((&raw mut SecretBase_EventScript_SetDecoration).cast::<u8>());
        }
        let __p1 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        if (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32) == 86i32 {
            TryPutSecretBaseVisitOnAir();
        }
        CancelDecorating_(taskId);
    }
}
pub(crate) unsafe extern "C" fn PlaceDecoration_(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(8)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>())
                        .read())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        (((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((&raw mut gCurDecorationItems)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize,
                            ))
                            .read(),
                        );
                        (((((&raw mut sDecorationContext).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_sub(7i32)
                                << 4)
                                .wrapping_add(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_sub(7i32),
                                )) as u8),
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !(((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(9)).read()) != 0) {
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 16i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((((((&raw mut sSecretBaseItemsIndicesBuffer).cast::<u8>())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 0i32
                        {
                            ((((&raw mut sSecretBaseItemsIndicesBuffer).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    .wrapping_add(1i32)) as u8),
                            );
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0u16;
                'l5: loop {
                    if !(((i) as i32) < 12i32) {
                        break 'l5;
                    }
                    'l6: {
                        if ((((((&raw mut sPlayerRoomItemsIndicesBuffer).cast::<u8>())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 0i32
                        {
                            ((((&raw mut sPlayerRoomItemsIndicesBuffer).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    .wrapping_add(1i32)) as u8),
                            );
                            break 'l5;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CancelDecoratingPrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayYesNoMenuDefaultYes();
        DoYesNoFuncWithChoice(
            taskId,
            (&raw const sCancelDecoratingYesNoFunctions)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn CancelDecorating(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrame(0u8, 0u8);
        CancelDecorating_(taskId);
    }
}
pub(crate) unsafe extern "C" fn CancelDecorating_(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FadeScreen(1u8, 0i8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(c1_overworld_prev_quest));
    }
}
pub(crate) unsafe extern "C" fn c1_overworld_prev_quest(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32);
            if __sw1 == 0i32 {
                LockPlayerFieldControls();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    WarpToInitialPosition(taskId);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                FreePlayerSpritePalette();
                FreeSpritePaletteByTag(3045u16);
                ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(FieldCB_InitDecorationItemsWindow));
                SetMainCallback2(Some(CB2_ReturnToField));
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_InitDecorationItemsWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = ((((data).wrapping_offset(2)).read()) as i32);
            if __sw1 == 0i32 {
                HideSecretBaseDecorationSprites();
                let __p2 = (data).wrapping_offset(2);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ScriptContext_SetupScript(
                    (&raw mut SecretBase_EventScript_InitDecorations).cast::<u8>(),
                );
                let __p3 = (data).wrapping_offset(2);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LockPlayerFieldControls();
                let __p4 = (data).wrapping_offset(2);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(HandleDecorationItemsMenuInput));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCB_InitDecorationItemsWindow() {
    unsafe {
        let mut taskId: u8 = 0u8;
        LockPlayerFieldControls();
        FadeInFromBlack();
        taskId = CreateTask(Some(Task_InitDecorationItemsWindow), 8u8);
        AddDecorationItemsWindow(taskId);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn ApplyCursorMovement_IsInvalid(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((((&raw mut sDecorationLastDirectionMoved)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 1i32)
            && ((((((data).wrapping_offset(1)).read()) as i32)
                .wrapping_sub(((((data).wrapping_offset(6)).read()) as i32)))
            .wrapping_sub(6i32)
                < 0i32)
        {
            let __p1 = (data).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
            return 0u8;
        }
        if (((((&raw mut sDecorationLastDirectionMoved)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 2i32)
            && (((((data).wrapping_offset(1)).read()) as i32).wrapping_sub(7i32)
                >= (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
        {
            let __p2 = (data).wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_sub(1));
            return 0u8;
        }
        if (((((&raw mut sDecorationLastDirectionMoved)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 3i32)
            && ((((data).read()) as i32).wrapping_sub(7i32) < 0i32)
        {
            (data).write(((data).read()).wrapping_add(1));
            return 0u8;
        }
        if (((((&raw mut sDecorationLastDirectionMoved)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 4i32)
            && (((((data).read()) as i32)
                .wrapping_add(((((data).wrapping_offset(5)).read()) as i32)))
            .wrapping_sub(8i32)
                >= (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                    .cast::<i32>())
                .read())
        {
            (data).write(((data).read()).wrapping_sub(1));
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn IsHoldingDirection() -> u8 {
    unsafe {
        let mut heldKeys: u16 = (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 240i32) as u16);
        if (((((heldKeys) as i32) != 64i32) && (((heldKeys) as i32) != 128i32))
            && (((heldKeys) as i32) != 32i32))
            && (((heldKeys) as i32) != 16i32)
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ResetCursorMovement() {
    unsafe {
        ((&raw mut sDecorationLastDirectionMoved)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn Task_SelectLocation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .read())
            != 0)
        {
            if ((((data).wrapping_offset(10)).read()) as i32) == 1i32 {
                ((((((&raw const sPlacePutAwayYesNoFunctions)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(12)).read()) as i32) as isize * 8))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read())
                .unwrap_unchecked()(taskId);
                return;
            }
            if ((((data).wrapping_offset(10)).read()) as i32) == 2i32 {
                ((((((&raw const sPlacePutAwayYesNoFunctions)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(12)).read()) as i32) as isize * 8))
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read())
                .unwrap_unchecked()(taskId);
                return;
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(44)
                .cast::<u16>())
            .read()) as i32)
                & 240i32)
                == 64i32
            {
                ((&raw mut sDecorationLastDirectionMoved)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(1u8);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(0i16);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .write((-2i16));
                let __p1 = (data).wrapping_offset(1);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(44)
                .cast::<u16>())
            .read()) as i32)
                & 240i32)
                == 128i32
            {
                ((&raw mut sDecorationLastDirectionMoved)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(2u8);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(0i16);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(2i16);
                let __p2 = (data).wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(44)
                .cast::<u16>())
            .read()) as i32)
                & 240i32)
                == 32i32
            {
                ((&raw mut sDecorationLastDirectionMoved)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(3u8);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write((-2i16));
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(0i16);
                (data).write(((data).read()).wrapping_sub(1));
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(44)
                .cast::<u16>())
            .read()) as i32)
                & 240i32)
                == 16i32
            {
                ((&raw mut sDecorationLastDirectionMoved)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(4u8);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(2i16);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(0i16);
                (data).write(((data).read()).wrapping_add(1));
            }
            if (!((IsHoldingDirection()) != 0)) || (!((ApplyCursorMovement_IsInvalid(taskId)) != 0))
            {
                ResetCursorMovement();
            }
        }
        if (((&raw mut sDecorationLastDirectionMoved)
            .cast::<u8>()
            .cast::<u8>())
        .read())
            != 0
        {
            let __p3 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(4);
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(4);
            (__p4).write((((((__p4).read()) as i32) & 7i32) as i16));
        }
        if !((((data).wrapping_offset(10)).read()) != 0) {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                ((data).wrapping_offset(10)).write(1i16);
            }
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                ((data).wrapping_offset(10)).write(2i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ContinueDecorating(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrame(0u8, 1u8);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_SelectLocation));
    }
}
pub(crate) unsafe extern "C" fn CantPlaceDecorationPrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0)
            || (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0)
        {
            ContinueDecorating(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ClearPlaceDecorationGraphicsDataBuffer(data: *mut u8) {
    unsafe {
        let mut data = data;
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                data,
                                (16777216u32
                                    | (crate::c::div_u32(
                                        2212u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
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
    }
}
pub(crate) unsafe extern "C" fn CopyPalette(dest: *mut u16, pal: u16) {
    unsafe {
        let mut dest = dest;
        let mut pal = pal;
        'l1: loop {
            'l2: {
                CpuFastSet(
                    (((((((&raw mut gTilesetPointer_SecretBase).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((pal) as i32) as isize * 32))
                    .cast::<u16>())
                    .cast::<u8>(),
                    (dest).cast::<u8>(),
                    (crate::c::div_u32(32u32, ((crate::c::div_i32(32i32, 8i32)) as u32))
                        & 2097151u32),
                );
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CopyTile(dest: *mut u8, tile: u16) {
    unsafe {
        let mut dest = dest;
        let mut tile = tile;
        let mut buffer = crate::ffi::Align4([0u8; 32]);
        let mut mode: u16 = 0u16;
        let mut i: u16 = 0u16;
        mode = ((((tile) as i32) >> 10) as u16);
        if ((tile) as i32) != 0i32 {
            tile = ((((tile) as i32) & 1023i32) as u16);
        }
        'l1: loop {
            'l2: {
                CpuFastSet(
                    ((((((&raw mut gTilesetPointer_SecretBase).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u32>())
                    .read())
                    .wrapping_offset(
                        ((crate::c::div_u32(
                            ((((tile) as i32).wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                                as u32),
                            4u32,
                        )) as i32) as isize,
                    ))
                    .cast::<u8>(),
                    (&raw mut buffer).cast::<u8>(),
                    ((crate::c::div_i32(
                        crate::c::div_i32(256i32, 8i32),
                        crate::c::div_i32(32i32, 8i32),
                    ) & 2097151i32) as u32),
                );
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l3: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 {
                'l4: loop {
                    'l5: {
                        CpuFastSet(
                            (&raw mut buffer).cast::<u8>(),
                            dest,
                            ((crate::c::div_i32(
                                crate::c::div_i32(256i32, 8i32),
                                crate::c::div_i32(32i32, 8i32),
                            ) & 2097151i32) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l4;
                    }
                }
                break 'l3;
            }
            if __sw1 == 1i32 {
                {
                    i = 0u16;
                    'l6: loop {
                        if !(((i) as i32) < 8i32) {
                            break 'l6;
                        }
                        'l7: {
                            ((dest).wrapping_offset(
                                (((4i32).wrapping_mul(((i) as i32))).wrapping_add(0i32)) as isize,
                            ))
                            .write(
                                ((((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                    (((4i32).wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                        .wrapping_sub(1i32))
                                        as isize,
                                ))
                                .read()) as i32)
                                    >> 4)
                                    .wrapping_add(
                                        (((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                            (((4i32).wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                                .wrapping_sub(1i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            & 15i32)
                                            << 4),
                                    )) as u8),
                            );
                            ((dest).wrapping_offset(
                                (((4i32).wrapping_mul(((i) as i32))).wrapping_add(1i32)) as isize,
                            ))
                            .write(
                                ((((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                    (((4i32).wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                        .wrapping_sub(2i32))
                                        as isize,
                                ))
                                .read()) as i32)
                                    >> 4)
                                    .wrapping_add(
                                        (((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                            (((4i32).wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                                .wrapping_sub(2i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            & 15i32)
                                            << 4),
                                    )) as u8),
                            );
                            ((dest).wrapping_offset(
                                (((4i32).wrapping_mul(((i) as i32))).wrapping_add(2i32)) as isize,
                            ))
                            .write(
                                ((((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                    (((4i32).wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                        .wrapping_sub(3i32))
                                        as isize,
                                ))
                                .read()) as i32)
                                    >> 4)
                                    .wrapping_add(
                                        (((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                            (((4i32).wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                                .wrapping_sub(3i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            & 15i32)
                                            << 4),
                                    )) as u8),
                            );
                            ((dest).wrapping_offset(
                                (((4i32).wrapping_mul(((i) as i32))).wrapping_add(3i32)) as isize,
                            ))
                            .write(
                                ((((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                    (((4i32).wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                        .wrapping_sub(4i32))
                                        as isize,
                                ))
                                .read()) as i32)
                                    >> 4)
                                    .wrapping_add(
                                        (((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                            (((4i32).wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                                .wrapping_sub(4i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            & 15i32)
                                            << 4),
                                    )) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l3;
            }
            if __sw1 == 2i32 {
                {
                    i = 0u16;
                    'l8: loop {
                        if !(((i) as i32) < 8i32) {
                            break 'l8;
                        }
                        'l9: {
                            ((dest).wrapping_offset(
                                (((4i32).wrapping_mul(((i) as i32))).wrapping_add(0i32)) as isize,
                            ))
                            .write(
                                (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                    (((4i32).wrapping_mul((7i32).wrapping_sub(((i) as i32))))
                                        .wrapping_add(0i32))
                                        as isize,
                                ))
                                .read(),
                            );
                            ((dest).wrapping_offset(
                                (((4i32).wrapping_mul(((i) as i32))).wrapping_add(1i32)) as isize,
                            ))
                            .write(
                                (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                    (((4i32).wrapping_mul((7i32).wrapping_sub(((i) as i32))))
                                        .wrapping_add(1i32))
                                        as isize,
                                ))
                                .read(),
                            );
                            ((dest).wrapping_offset(
                                (((4i32).wrapping_mul(((i) as i32))).wrapping_add(2i32)) as isize,
                            ))
                            .write(
                                (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                    (((4i32).wrapping_mul((7i32).wrapping_sub(((i) as i32))))
                                        .wrapping_add(2i32))
                                        as isize,
                                ))
                                .read(),
                            );
                            ((dest).wrapping_offset(
                                (((4i32).wrapping_mul(((i) as i32))).wrapping_add(3i32)) as isize,
                            ))
                            .write(
                                (((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                    (((4i32).wrapping_mul((7i32).wrapping_sub(((i) as i32))))
                                        .wrapping_add(3i32))
                                        as isize,
                                ))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l3;
            }
            if __sw1 == 3i32 {
                {
                    i = 0u16;
                    'l10: loop {
                        if !(((i) as i32) < 32i32) {
                            break 'l10;
                        }
                        'l11: {
                            ((dest).wrapping_offset(((i) as i32) as isize)).write(
                                ((((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                    ((31i32).wrapping_sub(((i) as i32))) as isize,
                                ))
                                .read()) as i32)
                                    >> 4)
                                    .wrapping_add(
                                        (((((((&raw mut buffer).cast::<u8>()).wrapping_offset(
                                            ((31i32).wrapping_sub(((i) as i32))) as isize,
                                        ))
                                        .read())
                                            as i32)
                                            & 15i32)
                                            << 4),
                                    )) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l3;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetDecorSelectionBoxTiles(data: *mut u8) {
    unsafe {
        let mut data = data;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 64i32) {
                    break 'l1;
                }
                'l2: {
                    CopyTile(
                        (((data).wrapping_add(132)).cast::<u8>()).wrapping_offset(
                            (((i) as i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))) as isize,
                        ),
                        ((((data).wrapping_add(4)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetMetatile(tile: u16) -> u16 {
    unsafe {
        let mut tile = tile;
        return ((((((((((&raw mut gTilesetPointer_SecretBaseRedCave).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(((tile) as i32) as isize))
        .read()) as i32)
            & 4095i32) as u16);
    }
}
pub(crate) unsafe extern "C" fn SetDecorSelectionMetatiles(data: *mut u8) {
    unsafe {
        let mut data = data;
        let mut i: u8 = 0u8;
        let mut shape: u8 = 0u8;
        shape = ((((data).cast::<*mut u8>()).read()).wrapping_add(18)).read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw const sDecorTilemaps).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((shape) as i32) as isize * 16))
                    .wrapping_add(12))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((((data).wrapping_add(4)).cast::<u16>()).wrapping_offset(
                        (((((((((&raw const sDecorTilemaps).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((shape) as i32) as isize * 16))
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize,
                    ))
                    .write(GetMetatile(
                        (((((((((((data).cast::<*mut u8>()).read())
                            .wrapping_add(28)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(
                            (((((((((&raw const sDecorTilemaps).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((shape) as i32) as isize * 16))
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_mul(8i32))
                        .wrapping_add(
                            (((((((((&raw const sDecorTilemaps).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((shape) as i32) as isize * 16))
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                        )) as u16),
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetDecorSelectionBoxOamAttributes(decorShape: u8) {
    unsafe {
        let mut decorShape = decorShape;
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(0),
            0,
            8,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(1),
            0,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(1),
            2,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(1),
            4,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(1),
            5,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(1),
            6,
            2,
            ((((((&raw const sDecorationMovementInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((decorShape) as i32) as isize * 4))
            .read()) as u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(2),
            0,
            9,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(3),
            1,
            5,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(3),
            6,
            2,
            (((((((&raw const sDecorationMovementInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((decorShape) as i32) as isize * 4))
            .wrapping_add(1))
            .read()) as u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(4),
            0,
            10,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sDecorSelectorOam).cast::<u8>()).wrapping_add(5),
            4,
            4,
            (0u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn InitializePuttingAwayCursorSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(InitializePuttingAwayCursorSprite2));
    }
}
pub(crate) unsafe extern "C" fn InitializePuttingAwayCursorSprite2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            == 0i32
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                < 15i32
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            }
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p2).write((((((__p2).read()) as i32) & 31i32) as i16));
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn gpu_pal_decompress_alloc_tag_and_upload(
    data: *mut u8,
    decor: u8,
) -> u8 {
    unsafe {
        let mut data = data;
        let mut decor = decor;
        ClearPlaceDecorationGraphicsDataBuffer(data);
        ((data).cast::<*mut u8>()).write(
            (((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((decor) as i32) as isize * 32),
        );
        if ((((((data).cast::<*mut u8>()).read()).wrapping_add(17)).read()) as i32) == 4i32 {
            return CreateObjectGraphicsSprite(
                (((((data).cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u16>())
                .read())
                .read(),
                Some(SpriteCallbackDummy),
                0i16,
                0i16,
                1u8,
            );
        }
        FreeSpritePaletteByTag(3045u16);
        SetDecorSelectionMetatiles(data);
        SetDecorSelectionBoxOamAttributes(
            ((((data).cast::<*mut u8>()).read()).wrapping_add(18)).read(),
        );
        SetDecorSelectionBoxTiles(data);
        CopyPalette(
            ((data).wrapping_add(2180)).cast::<u16>(),
            ((((((((((&raw mut gTilesetPointer_SecretBaseRedCave).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                (((((((((data).cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u16>())
                .read())
                .read()) as i32)
                    .wrapping_mul(8i32))
                .wrapping_add(7i32)) as isize,
            ))
            .read()) as i32)
                >> 12) as u16),
        );
        LoadSpritePalette(
            (&raw const sSpritePal_PlaceDecoration)
                .cast::<u8>()
                .cast_mut(),
        );
        return CreateSprite(
            (&raw const sDecorationSelectorSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn AddDecorationIconObjectFromIconTable(
    tilesTag: u16,
    paletteTag: u16,
    decor: u8,
) -> u8 {
    unsafe {
        let mut tilesTag = tilesTag;
        let mut paletteTag = paletteTag;
        let mut decor = decor;
        let mut sheet = crate::ffi::Align4([0u8; 8]);
        let mut palette = crate::ffi::Align4([0u8; 8]);
        let mut template: *mut u8 = core::ptr::null_mut();
        let mut spriteId: u8 = 0u8;
        if !((AllocItemIconTemporaryBuffers()) != 0) {
            return 64u8;
        }
        LZDecompressWram(
            GetDecorationIconPicOrPalette(((decor) as u16), 0u8),
            ((&raw mut gItemIconDecompressionBuffer).cast::<*mut u8>()).read(),
        );
        CopyItemIconPicTo4x4Buffer(
            ((&raw mut gItemIconDecompressionBuffer).cast::<*mut u8>()).read(),
            ((&raw mut gItemIcon4x4Buffer).cast::<*mut u8>()).read(),
        );
        (((&raw mut sheet).cast::<u8>()).cast::<*mut u8>())
            .write(((&raw mut gItemIcon4x4Buffer).cast::<*mut u8>()).read());
        (((&raw mut sheet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(512u16);
        (((&raw mut sheet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(tilesTag);
        LoadSpriteSheet((&raw mut sheet).cast::<u8>());
        (((&raw mut palette).cast::<u8>()).cast::<*mut u32>())
            .write(GetDecorationIconPicOrPalette(((decor) as u16), 1u8));
        (((&raw mut palette).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(paletteTag);
        LoadCompressedSpritePalette((&raw mut palette).cast::<u8>());
        template = Alloc(24u32);
        template.cast::<crate::c::Rec4<24>>().write_unaligned(
            (&raw mut gItemIconSpriteTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .read_unaligned(),
        );
        ((template).cast::<u16>()).write(tilesTag);
        ((template).wrapping_add(2).cast::<u16>()).write(paletteTag);
        spriteId = CreateSprite(template, 0i16, 0i16, 0u8);
        FreeItemIconTemporaryBuffers();
        Free(template);
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn GetDecorationIconPicOrPalette(decor: u16, mode: u8) -> *mut u32 {
    unsafe {
        let mut decor = decor;
        let mut mode = mode;
        if ((decor) as i32) > 120i32 {
            decor = 0u16;
        }
        return ((((((&raw const gDecorIconTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((decor) as i32) as isize * 8))
        .cast::<*mut u32>())
        .wrapping_offset(((mode) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn AddDecorationIconObjectFromObjectEvent(
    tilesTag: u16,
    paletteTag: u16,
    decor: u8,
) -> u8 {
    unsafe {
        let mut tilesTag = tilesTag;
        let mut paletteTag = paletteTag;
        let mut decor = decor;
        let mut spriteId: u8 = 0u8;
        let mut sheet = crate::ffi::Align4([0u8; 8]);
        let mut palette = crate::ffi::Align4([0u8; 8]);
        let mut template: *mut u8 = core::ptr::null_mut();
        ClearPlaceDecorationGraphicsDataBuffer(
            (&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>(),
        );
        (((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>()).cast::<*mut u8>()).write(
            (((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((decor) as i32) as isize * 32),
        );
        if (((((((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>()).cast::<*mut u8>())
            .read())
        .wrapping_add(17))
        .read()) as i32)
            != 4i32
        {
            SetDecorSelectionMetatiles((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>());
            SetDecorSelectionBoxOamAttributes(
                (((((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(18))
                .read(),
            );
            SetDecorSelectionBoxTiles((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>());
            CopyPalette(
                (((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>()).wrapping_add(2180))
                    .cast::<u16>(),
                ((((((((((&raw mut gTilesetPointer_SecretBaseRedCave).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u16>())
                .read())
                .wrapping_offset(
                    ((((((((((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(28)
                    .cast::<*mut u16>())
                    .read())
                    .read()) as i32)
                        .wrapping_mul(8i32))
                    .wrapping_add(7i32)) as isize,
                ))
                .read()) as i32)
                    >> 12) as u16),
            );
            (((&raw mut sheet).cast::<u8>()).cast::<*mut u8>()).write(
                (((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>()).wrapping_add(132))
                    .cast::<u8>(),
            );
            (((&raw mut sheet).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(
                ((((((((&raw const sDecorShapeSizes)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    (((((((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(18))
                    .read()) as i32) as isize,
                ))
                .read()) as i32)
                    .wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u16),
            );
            (((&raw mut sheet).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .write(tilesTag);
            LoadSpriteSheet((&raw mut sheet).cast::<u8>());
            (((&raw mut palette).cast::<u8>()).cast::<*mut u16>()).write(
                (((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>()).wrapping_add(2180))
                    .cast::<u16>(),
            );
            (((&raw mut palette).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(paletteTag);
            LoadSpritePalette((&raw mut palette).cast::<u8>());
            template = Alloc(24u32);
            template.cast::<crate::c::Rec4<24>>().write_unaligned(
                (&raw const sDecorWhilePlacingSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
            ((template).cast::<u16>()).write(tilesTag);
            ((template).wrapping_add(2).cast::<u16>()).write(paletteTag);
            spriteId = CreateSprite(template, 0i16, 0i16, 0u8);
            Free(template);
        } else {
            spriteId = CreateObjectGraphicsSprite(
                ((((((&raw mut sPlaceDecorationGraphicsDataBuffer).cast::<u8>())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(28)
                .cast::<*mut u16>())
                .read())
                .read(),
                Some(SpriteCallbackDummy),
                0i16,
                0i16,
                1u8,
            );
        }
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddDecorationIconObject(
    decor: u8,
    x: i16,
    y: i16,
    priority: u8,
    tilesTag: u16,
    paletteTag: u16,
) -> u8 {
    unsafe {
        let mut decor = decor;
        let mut x = x;
        let mut y = y;
        let mut priority = priority;
        let mut tilesTag = tilesTag;
        let mut paletteTag = paletteTag;
        let mut spriteId: u8 = 0u8;
        if ((decor) as i32) > 120i32 {
            spriteId = AddDecorationIconObjectFromIconTable(tilesTag, paletteTag, 0u8);
            if ((spriteId) as i32) == 64i32 {
                return 64u8;
            }
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(((((x) as i32).wrapping_add(4i32)) as i16));
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(((((y) as i32).wrapping_add(4i32)) as i16));
        } else {
            if (((((((&raw const gDecorIconTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((decor) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read()) as usize)
                == 0usize
            {
                spriteId = AddDecorationIconObjectFromObjectEvent(tilesTag, paletteTag, decor);
                if ((spriteId) as i32) == 64i32 {
                    return 64u8;
                }
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(x);
                if (((decor) as i32) == 42i32) || (((decor) as i32) == 43i32) {
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(((((y) as i32).wrapping_sub(4i32)) as i16));
                } else {
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(y);
                }
            } else {
                spriteId = AddDecorationIconObjectFromIconTable(tilesTag, paletteTag, decor);
                if ((spriteId) as i32) == 64i32 {
                    return 64u8;
                }
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(((((x) as i32).wrapping_add(4i32)) as i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(((((y) as i32).wrapping_add(4i32)) as i16));
            }
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            ((priority) as u16) as i32,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn ClearDecorationContextIndex(idx: u8) {
    unsafe {
        let mut idx = idx;
        (((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_offset(((idx) as i32) as isize))
        .write(0u8);
        (((((&raw mut sDecorationContext).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((idx) as i32) as isize))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutAwayDecorationIteration() {
    unsafe {
        let mut i: u16 = 0u16;
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(0u16);
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
            == ((((&raw mut sCurDecorSelectedInRearrangement)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32)
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            if (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    (((((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>()).read())
                        .wrapping_offset(
                            ((((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                    as isize
                                    * 8,
                            ))
                            .read()) as i32) as isize,
                        ))
                    .read()) as i32) as isize
                        * 32,
                ))
            .wrapping_add(17))
            .read()) as i32)
                == 4i32
            {
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
                    (((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                as isize
                                * 8,
                        ))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read(),
                );
                ClearDecorationContextIndex(
                    ((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                as isize
                                * 8,
                        ))
                    .read(),
                );
                {
                    i = 0u16;
                    'l1: loop {
                        if !(((i) as i32)
                            < ((((((&raw mut gMapHeader).cast::<u8>())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .read()) as i32))
                        {
                            break 'l1;
                        }
                        'l2: {
                            if ((((((((((&raw mut gMapHeader).cast::<u8>())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 24))
                            .wrapping_add(20)
                            .cast::<u16>())
                            .read()) as i32)
                                == ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                            {
                                ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(
                                    (((((((((&raw mut gMapHeader).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((i) as i32) as isize * 24))
                                    .read()) as u16),
                                );
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetObjectEventLocalIdByFlag() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut gMapHeader).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gMapHeader).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 24))
                    .wrapping_add(20)
                    .cast::<u16>())
                    .read()) as i32)
                        == ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                    {
                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
                            (((((((((&raw mut gMapHeader).cast::<u8>())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 24))
                            .read()) as u16),
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearRearrangementNonSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut posX: i32 = 0i32;
        let mut posY: i32 = 0i32;
        let mut perm: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut sCurDecorSelectedInRearrangement)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    perm = (((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            (((((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>())
                                .read())
                            .wrapping_offset(
                                ((((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32) as isize
                                * 32,
                        ))
                    .wrapping_add(17))
                    .read();
                    posX = ((((((((&raw mut sDecorationContext).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        >> 4);
                    posY = ((((((((&raw mut sDecorationContext).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        & 15i32);
                    if ((perm) as i32) != 4i32 {
                        {
                            y = 0u8;
                            'l3: loop {
                                if !(((y) as i32)
                                    < (((((((&raw mut sDecorRearrangementDataBuffer)
                                        .cast::<u8>())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 8))
                                    .wrapping_add(2))
                                    .read()) as i32))
                                {
                                    break 'l3;
                                }
                                'l4: {
                                    {
                                        x = 0u8;
                                        'l5: loop {
                                            if !((((x) as i32)) < ((((((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>()).wrapping_offset((((i) as i32)) as isize * 8)).wrapping_add(1)).read()) as i32))) { break 'l5; }
                                            'l6: {
                                                MapGridSetMetatileEntryAt(
                                                    ((posX).wrapping_add(7i32))
                                                        .wrapping_add(((x) as i32)),
                                                    ((posY).wrapping_add(7i32))
                                                        .wrapping_sub(((y) as i32)),
                                                    (((((((((((&raw mut gMapHeader)
                                                        .cast::<u8>())
                                                    .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(12)
                                                    .cast::<*mut u16>())
                                                    .read())
                                                    .wrapping_offset(
                                                        (((posX).wrapping_add(((x) as i32)))
                                                            .wrapping_add(
                                                                ((((((&raw mut gMapHeader)
                                                                    .cast::<u8>())
                                                                .cast::<*mut u8>())
                                                                .read())
                                                                .cast::<i32>())
                                                                .read())
                                                                .wrapping_mul(
                                                                    (posY)
                                                                        .wrapping_sub(((y) as i32)),
                                                                ),
                                                            ))
                                                            as isize,
                                                    ))
                                                    .read())
                                                        as i32)
                                                        | 12288i32)
                                                        as u16),
                                                );
                                            }
                                            x = (x).wrapping_add(1);
                                        }
                                    }
                                }
                                y = (y).wrapping_add(1);
                            }
                        }
                        ClearDecorationContextIndex(
                            ((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PutAwayDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32);
            if __sw1 == 0i32 {
                ClearRearrangementNonSprites();
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(1i16);
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
                    DrawWholeMapView();
                    ScriptContext_SetupScript(
                        (&raw mut SecretBase_EventScript_PutAwayDecoration).cast::<u8>(),
                    );
                    ClearDialogWindowAndFrame(0u8, 1u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                LockPlayerFieldControls();
                IdentifyOwnedDecorationsCurrentlyInUseInternal(taskId);
                FadeInFromBlack();
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(3i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_DecorationReturnedToPC).cast::<u8>(),
                    );
                    DisplayItemMessageOnField(
                        taskId,
                        (&raw mut gStringVar4).cast::<u8>(),
                        Some(ContinuePuttingAwayDecorationsPrompt),
                    );
                    if (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32)
                        == 86i32
                    {
                        TryPutSecretBaseVisitOnAir();
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HasDecorationsInUse(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(8)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>())
                        .read())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetUpPuttingAwayDecorationPlayerAvatar() {
    unsafe {
        GetPlayerFacingDirection();
        ((&raw mut sDecor_CameraSpriteObjectIdx1)
            .cast::<u8>()
            .cast::<u8>())
        .write(
            (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gFieldCamera).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u32>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .read()) as u8),
        );
        LoadPlayerSpritePalette();
        (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(
            ((CreateSprite(
                (&raw const sPuttingAwayCursorSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                120i16,
                80i16,
                0u8,
            )) as u32),
        );
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 0i32
        {
            ((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .write(CreateObjectGraphicsSprite(
                193u16,
                Some(SpriteCallbackDummy),
                136i16,
                72i16,
                0u8,
            ));
        } else {
            ((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .write(CreateObjectGraphicsSprite(
                194u16,
                Some(SpriteCallbackDummy),
                136i16,
                72i16,
                0u8,
            ));
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx2)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ),
        );
        ((&raw mut sDecor_CameraSpriteObjectIdx1)
            .cast::<u8>()
            .cast::<u8>())
        .write(
            (((((&raw mut gFieldCamera).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()) as u8),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ContinuePuttingAwayDecorations(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = ((((data).wrapping_offset(2)).read()) as i32);
            if __sw1 == 0i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetInitialPositions(taskId);
                    ((data).wrapping_offset(2)).write(1i16);
                    ((data).wrapping_offset(6)).write(1i16);
                    ((data).wrapping_offset(5)).write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetUpPuttingAwayDecorationPlayerAvatar();
                FadeInFromBlack();
                ((data).wrapping_offset(2)).write(2i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
                    ((data).wrapping_offset(12)).write(1i16);
                    ContinuePuttingAwayDecorations(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ContinuePuttingAwayDecorations(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrame(0u8, 1u8);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(InitializeCameraSprite1));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(136i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(72i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_SelectLocation));
    }
}
pub(crate) unsafe extern "C" fn AttemptPutAwayDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(0i16);
        ResetCursorMovement();
        AttemptPutAwayDecoration_(taskId);
    }
}
pub(crate) unsafe extern "C" fn AttemptCancelPutAwayDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(0i16);
        ResetCursorMovement();
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_StopPuttingAwayDecorations).cast::<u8>(),
        );
        DisplayItemMessageOnField(
            taskId,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(StopPuttingAwayDecorationsPrompt),
        );
    }
}
pub(crate) unsafe extern "C" fn AttemptPutAwayDecoration_(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        let mut behavior: u8 = 0u8;
        AttemptMarkDecorUnderCursorForRemoval(taskId);
        if ((((&raw mut sCurDecorSelectedInRearrangement)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            != 0i32
        {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_ReturnDecorationToPC).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(ReturnDecorationPrompt),
            );
        } else {
            data = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            behavior = ((MapGridGetMetatileBehaviorAt(
                (((data).read()) as i32),
                ((((data).wrapping_offset(1)).read()) as i32),
            )) as u8);
            if (((MetatileBehavior_IsSecretBasePC(behavior)) as i32) == 1i32)
                || (((MetatileBehavior_IsPlayerRoomPCOn(behavior)) as i32) == 1i32)
            {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                            .cast::<u8>()
                            .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_StopPuttingAwayDecorations).cast::<u8>(),
                );
                DisplayItemMessageOnField(
                    taskId,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(StopPuttingAwayDecorationsPrompt),
                );
            } else {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_NoDecorationHere).cast::<u8>(),
                );
                DisplayItemMessageOnField(
                    taskId,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(ContinuePuttingAwayDecorationsPrompt),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ContinuePuttingAwayDecorationsPrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0)
            || (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0)
        {
            ContinuePuttingAwayDecorations(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SetDecorRearrangementShape(decor: u8, data: *mut u8) {
    unsafe {
        let mut decor = decor;
        let mut data = data;
        if (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((decor) as i32) as isize * 32))
        .wrapping_add(18))
        .read()) as i32)
            == 0i32
        {
            ((data).wrapping_add(1)).write(1u8);
            ((data).wrapping_add(2)).write(1u8);
        } else {
            if (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((decor) as i32) as isize * 32))
            .wrapping_add(18))
            .read()) as i32)
                == 1i32
            {
                ((data).wrapping_add(1)).write(2u8);
                ((data).wrapping_add(2)).write(1u8);
            } else {
                if (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((decor) as i32) as isize * 32))
                .wrapping_add(18))
                .read()) as i32)
                    == 2i32
                {
                    ((data).wrapping_add(1)).write(3u8);
                    ((data).wrapping_add(2)).write(1u8);
                } else {
                    if (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((decor) as i32) as isize * 32))
                    .wrapping_add(18))
                    .read()) as i32)
                        == 3i32
                    {
                        ((data).wrapping_add(1)).write(4u8);
                        ((data).wrapping_add(2)).write(2u8);
                    } else {
                        if (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((decor) as i32) as isize * 32))
                        .wrapping_add(18))
                        .read()) as i32)
                            == 4i32
                        {
                            ((data).wrapping_add(1)).write(2u8);
                            ((data).wrapping_add(2)).write(2u8);
                        } else {
                            if (((((((&raw const gDecorations).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((decor) as i32) as isize * 32))
                            .wrapping_add(18))
                            .read()) as i32)
                                == 5i32
                            {
                                ((data).wrapping_add(1)).write(1u8);
                                ((data).wrapping_add(2)).write(2u8);
                            } else {
                                if (((((((&raw const gDecorations).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((decor) as i32) as isize * 32))
                                .wrapping_add(18))
                                .read()) as i32)
                                    == 6i32
                                {
                                    ((data).wrapping_add(1)).write(1u8);
                                    ((data).wrapping_add(2)).write(3u8);
                                } else {
                                    if (((((((&raw const gDecorations).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(((decor) as i32) as isize * 32))
                                    .wrapping_add(18))
                                    .read()) as i32)
                                        == 7i32
                                    {
                                        ((data).wrapping_add(1)).write(2u8);
                                        ((data).wrapping_add(2)).write(4u8);
                                    } else {
                                        if (((((((&raw const gDecorations)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((decor) as i32) as isize * 32))
                                        .wrapping_add(18))
                                        .read()) as i32)
                                            == 8i32
                                        {
                                            ((data).wrapping_add(1)).write(3u8);
                                            ((data).wrapping_add(2)).write(3u8);
                                        } else {
                                            if (((((((&raw const gDecorations)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(((decor) as i32) as isize * 32))
                                            .wrapping_add(18))
                                            .read())
                                                as i32)
                                                == 9i32
                                            {
                                                ((data).wrapping_add(1)).write(3u8);
                                                ((data).wrapping_add(2)).write(2u8);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetCameraSpritePosition(x: u8, y: u8) {
    unsafe {
        let mut x = x;
        let mut y = y;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx1)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write((((((x) as i32).wrapping_mul(16i32)).wrapping_add(136i32)) as i16));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut sDecor_CameraSpriteObjectIdx2)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write((((((y) as i32).wrapping_mul(16i32)).wrapping_add(72i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn DecorationIsUnderCursor(taskId: u8, idx: u8, data: *mut u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut idx = idx;
        let mut data = data;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut xOff: u8 = 0u8;
        let mut yOff: u8 = 0u8;
        let mut ht: u8 = 0u8;
        x = (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            .wrapping_sub(7i32)) as u8);
        y = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            .wrapping_sub(7i32)) as u8);
        xOff = (((((((((&raw mut sDecorationContext).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((idx) as i32) as isize))
        .read()) as i32)
            >> 4) as u8);
        yOff = (((((((((&raw mut sDecorationContext).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((idx) as i32) as isize))
        .read()) as i32)
            & 15i32) as u8);
        ht = ((data).wrapping_add(2)).read();
        if ((((((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_offset(((idx) as i32) as isize))
        .read()) as i32)
            == 41i32)
            && (MapGridGetMetatileIdAt(
                ((xOff) as i32).wrapping_add(7i32),
                ((yOff) as i32).wrapping_add(7i32),
            ) == 652i32)
        {
            ht = (ht).wrapping_sub(1);
        }
        if (((((x) as i32) >= ((xOff) as i32))
            && (((x) as i32)
                < ((xOff) as i32).wrapping_add(((((data).wrapping_add(1)).read()) as i32))))
            && (((y) as i32) > ((yOff) as i32).wrapping_sub(((ht) as i32))))
            && (((y) as i32) <= ((yOff) as i32))
        {
            SetCameraSpritePosition(
                ((((((data).wrapping_add(1)).read()) as i32)
                    .wrapping_sub((((x) as i32).wrapping_sub(((xOff) as i32))).wrapping_add(1i32)))
                    as u8),
                ((((yOff) as i32).wrapping_sub(((y) as i32))) as u8),
            );
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetDecorRearrangementFlagIdIfFlagUnset() {
    unsafe {
        let mut xOff: u8 = 0u8;
        let mut yOff: u8 = 0u8;
        let mut i: u16 = 0u16;
        xOff = (((((((((&raw mut sDecorationContext).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurDecorSelectedInRearrangement)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            >> 4) as u8);
        yOff = (((((((((&raw mut sDecorationContext).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurDecorSelectedInRearrangement)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .read()) as i32) as isize,
        ))
        .read()) as i32)
            & 15i32) as u8);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 64i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(3184))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 24))
                    .wrapping_add(4)
                    .cast::<i16>())
                    .read()) as i32)
                        == ((xOff) as i32))
                        && ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(3184))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .wrapping_add(6)
                        .cast::<i16>())
                        .read()) as i32)
                            == ((yOff) as i32)))
                        && (!((FlagGet(
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(3184))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24))
                            .wrapping_add(20)
                            .cast::<u16>())
                            .read(),
                        )) != 0))
                    {
                        (((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut sCurDecorSelectedInRearrangement)
                                    .cast::<u8>()
                                    .cast::<u8>())
                                .read()) as i32) as isize
                                    * 8,
                            ))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .write(
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(3184))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24))
                            .wrapping_add(20)
                            .cast::<u16>())
                            .read(),
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AttemptMarkSpriteDecorUnderCursorForRemoval(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(8)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>())
                        .read())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        if (((((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw mut sDecorationContext).cast::<u8>())
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 32,
                            ))
                        .wrapping_add(17))
                        .read()) as i32)
                            == 4i32
                        {
                            SetDecorRearrangementShape(
                                (((((&raw mut sDecorationContext).cast::<u8>())
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                ((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                                    .cast::<u8>(),
                            );
                            if ((DecorationIsUnderCursor(
                                taskId,
                                ((i) as u8),
                                ((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                                    .cast::<u8>(),
                            )) as i32)
                                == 1i32
                            {
                                (((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                                    .cast::<u8>())
                                .write(((i) as u8));
                                SetDecorRearrangementFlagIdIfFlagUnset();
                                ((&raw mut sCurDecorSelectedInRearrangement)
                                    .cast::<u8>()
                                    .cast::<u8>())
                                .write(1u8);
                                return 1u8;
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MarkSpriteDecorsInBoundsForRemoval(
    left: u8,
    top: u8,
    right: u8,
    bottom: u8,
) {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut right = right;
        let mut bottom = bottom;
        let mut i: u8 = 0u8;
        let mut xOff: u8 = 0u8;
        let mut yOff: u8 = 0u8;
        let mut decor: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(8)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    decor = (((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>())
                        .read())
                    .wrapping_offset(((i) as i32) as isize))
                    .read();
                    xOff = (((((((((&raw mut sDecorationContext).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        >> 4) as u8);
                    yOff = (((((((((&raw mut sDecorationContext).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        & 15i32) as u8);
                    if (((((((decor) as i32) != 0i32)
                        && ((((((((&raw const gDecorations).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((decor) as i32) as isize * 32))
                        .wrapping_add(17))
                        .read()) as i32)
                            == 4i32))
                        && (((left) as i32) <= ((xOff) as i32)))
                        && (((top) as i32) <= ((yOff) as i32)))
                        && (((right) as i32) >= ((xOff) as i32)))
                        && (((bottom) as i32) >= ((yOff) as i32))
                    {
                        ((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut sCurDecorSelectedInRearrangement)
                                    .cast::<u8>()
                                    .cast::<u8>())
                                .read()) as i32) as isize
                                    * 8,
                            ))
                        .write(i);
                        SetDecorRearrangementFlagIdIfFlagUnset();
                        let __p1 = (&raw mut sCurDecorSelectedInRearrangement)
                            .cast::<u8>()
                            .cast::<u8>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AttemptMarkDecorUnderCursorForRemoval(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut xOff: u8 = 0u8;
        let mut yOff: u8 = 0u8;
        let mut var1: u8 = 0u8;
        let mut var2: u32 = 0u32;
        ((&raw mut sCurDecorSelectedInRearrangement)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        if ((AttemptMarkSpriteDecorUnderCursorForRemoval(taskId)) as i32) != 1i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32)
                        < (((((&raw mut sDecorationContext).cast::<u8>()).wrapping_add(8)).read())
                            as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        var1 = (((((&raw mut sDecorationContext).cast::<u8>()).cast::<*mut u8>())
                            .read())
                        .wrapping_offset(((i) as i32) as isize))
                        .read();
                        if ((var1) as i32) != 0i32 {
                            SetDecorRearrangementShape(
                                var1,
                                ((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                                    .cast::<u8>(),
                            );
                            if ((DecorationIsUnderCursor(
                                taskId,
                                i,
                                ((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                                    .cast::<u8>(),
                            )) as i32)
                                == 1i32
                            {
                                (((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                                    .cast::<u8>())
                                .write(i);
                                let __p1 = (&raw mut sCurDecorSelectedInRearrangement)
                                    .cast::<u8>()
                                    .cast::<u8>();
                                (__p1).write(((__p1).read()).wrapping_add(1));
                                break 'l1;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((((&raw mut sCurDecorSelectedInRearrangement)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32)
                != 0i32
            {
                xOff = (((((((((&raw mut sDecorationContext).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>()).read())
                        as i32) as isize,
                ))
                .read()) as i32)
                    >> 4) as u8);
                yOff = (((((((((&raw mut sDecorationContext).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>()).read())
                        as i32) as isize,
                ))
                .read()) as i32)
                    & 15i32) as u8);
                var1 = (((((yOff) as i32).wrapping_sub(
                    ((((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>()).cast::<u8>())
                        .wrapping_add(2))
                    .read()) as i32),
                ))
                .wrapping_add(1i32)) as u8);
                var2 = (((((((((&raw mut sDecorRearrangementDataBuffer).cast::<u8>())
                    .cast::<u8>())
                .wrapping_add(1))
                .read()) as i32)
                    .wrapping_add(((xOff) as i32)))
                .wrapping_sub(1i32)) as u32);
                MarkSpriteDecorsInBoundsForRemoval(xOff, var1, ((var2) as u8), yOff);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReturnDecorationPrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayYesNoMenuDefaultYes();
        DoYesNoFuncWithChoice(
            taskId,
            (&raw const sReturnDecorationYesNoFunctions)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn PutAwayDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FadeScreen(1u8, 0i8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_PutAwayDecoration));
    }
}
pub(crate) unsafe extern "C" fn StopPuttingAwayDecorationsPrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayYesNoMenuDefaultYes();
        DoYesNoFuncWithChoice(
            taskId,
            (&raw const sStopPuttingAwayDecorationsYesNoFunctions)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn StopPuttingAwayDecorations(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrame(0u8, 0u8);
        StopPuttingAwayDecorations_(taskId);
    }
}
pub(crate) unsafe extern "C" fn StopPuttingAwayDecorations_(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FadeScreen(1u8, 0i8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_StopPuttingAwayDecorations));
    }
}
pub(crate) unsafe extern "C" fn Task_StopPuttingAwayDecorations(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    WarpToInitialPosition(taskId);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                FreePlayerSpritePalette();
                ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(FieldCB_StopPuttingAwayDecorations));
                SetMainCallback2(Some(CB2_ReturnToField));
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReinitializeDecorationMenuHandler(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = ((((data).wrapping_offset(2)).read()) as i32);
            if __sw1 == 0i32 {
                HideSecretBaseDecorationSprites();
                let __p2 = (data).wrapping_offset(2);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ScriptContext_SetupScript(
                    (&raw mut SecretBase_EventScript_InitDecorations).cast::<u8>(),
                );
                let __p3 = (data).wrapping_offset(2);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LockPlayerFieldControls();
                let __p4 = (data).wrapping_offset(2);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(HandleDecorationActionsMenuInput));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCB_StopPuttingAwayDecorations() {
    unsafe {
        let mut taskId: u8 = 0u8;
        FadeInFromBlack();
        DrawDialogueFrame(0u8, 1u8);
        InitDecorationActionsWindow();
        taskId = CreateTask(Some(Task_ReinitializeDecorationMenuHandler), 8u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn InitializeCameraSprite1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32) & 31i32) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 15i32 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPlayerSpritePalette() {
    unsafe {
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 0i32
        {
            LoadSpritePalette(
                (&raw const sSpritePal_PuttingAwayCursorBrendan)
                    .cast::<u8>()
                    .cast_mut(),
            );
        } else {
            LoadSpritePalette(
                (&raw const sSpritePal_PuttingAwayCursorMay)
                    .cast::<u8>()
                    .cast_mut(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FreePlayerSpritePalette() {
    unsafe {
        FreeSpritePaletteByTag(8u16);
    }
}
pub(crate) unsafe extern "C" fn DecorationItemsMenuAction_AttemptToss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsSelectedDecorInThePC()) as i32) == 1i32 {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((((&raw const gDecorations).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gCurDecorationItems)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>()).read())
                                as i32) as isize,
                        ))
                        .read()) as i32) as isize
                            * 32,
                    ))
                .wrapping_add(1))
                .cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_DecorationWillBeDiscarded).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(TossDecorationPrompt),
            );
        } else {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_CantThrowAwayInUse).cast::<u8>(),
            );
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(ReturnToDecorationItemsAfterInvalidSelection),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TossDecorationPrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayYesNoMenuDefaultYes();
        DoYesNoFuncWithChoice(
            taskId,
            (&raw const sTossDecorationYesNoFunctions)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn TossDecoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gCurDecorationItems)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((&raw mut gCurDecorationIndex).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u8);
        ((&raw mut sNumOwnedDecorationsInCurCategory)
            .cast::<u8>()
            .cast::<u8>())
        .write(GetNumOwnedDecorationsInCategory(
            ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read(),
        ));
        CondenseDecorationsInCategory(
            ((&raw mut sCurDecorationCategory).cast::<u8>().cast::<u8>()).read(),
        );
        IdentifyOwnedDecorationsCurrentlyInUseInternal(taskId);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_DecorationThrownAway).cast::<u8>(),
        );
        DisplayItemMessageOnField(
            taskId,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(ReturnToDecorationItemsAfterInvalidSelection),
        );
    }
}
