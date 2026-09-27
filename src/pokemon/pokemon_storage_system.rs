//! Translated from `src/pokemon_storage_system.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMainMenuTexts sWindowTemplate_MainMenu sAnim_ChooseBoxMenu_TopLeft sAnim_ChooseBoxMenu_BottomLeft sAnim_ChooseBoxMenu_TopRight sAnim_ChooseBoxMenu_BottomRight sAnims_ChooseBoxMenu sAffineAnim_ChooseBoxMenu sAffineAnims_ChooseBoxMenu sChooseBoxMenu_TextColors sText_OutOf30 sChooseBoxMenu_Pal sChooseBoxMenuCenter_Gfx sChooseBoxMenuSides_Gfx sScrollingBg_Gfx sScrollingBg_Tilemap sDisplayMenu_Pal sDisplayMenu_Tilemap sPkmnData_Tilemap sInterface_Pal sPkmnDataGray_Pal sScrollingBg_Pal sScrollingBgMoveItems_Pal sCloseBoxButton_Tilemap sPartySlotFilled_Tilemap sPartySlotEmpty_Tilemap sWaveform_Pal sWaveform_Gfx sUnused_Pal sTextWindows_Pal sWindowTemplates sBgTemplates sWaveformSpritePalette sSpriteSheet_Waveform sOamData_DisplayMon sSpriteTemplate_DisplayMon sMessages sYesNoWindowTemplate sOamData_DisplayMon sOamData_Waveform sAnim_Waveform_LeftOff sAnim_Waveform_LeftOn sAnim_Waveform_RightOff sAnim_Waveform_RightOn sAnims_Waveform sSpriteTemplate_Waveform sOamData_MonIcon sSpriteTemplate_MonIcon sOamData_MonIcon sAffineAnim_ReleaseMon_Release sAffineAnim_ReleaseMon_CameBack sAffineAnims_ReleaseMon sWallpaperPalettes_Forest sWallpaperTiles_Forest sWallpaperTilemap_Forest sWallpaperPalettes_City sWallpaperTiles_City sWallpaperTilemap_City sWallpaperPalettes_Desert sWallpaperTiles_Desert sWallpaperTilemap_Desert sWallpaperPalettes_Savanna sWallpaperTiles_Savanna sWallpaperTilemap_Savanna sWallpaperPalettes_Crag sWallpaperTiles_Crag sWallpaperTilemap_Crag sWallpaperPalettes_Volcano sWallpaperTiles_Volcano sWallpaperTilemap_Volcano sWallpaperPalettes_Snow sWallpaperTiles_Snow sWallpaperTilemap_Snow sWallpaperPalettes_Cave sWallpaperTiles_Cave sWallpaperTilemap_Cave sWallpaperPalettes_Beach sWallpaperTiles_Beach sWallpaperTilemap_Beach sWallpaperPalettes_Seafloor sWallpaperTiles_Seafloor sWallpaperTilemap_Seafloor sWallpaperPalettes_River sWallpaperTiles_River sWallpaperTilemap_River sWallpaperPalettes_Sky sWallpaperTiles_Sky sWallpaperTilemap_Sky sWallpaperPalettes_PolkaDot sWallpaperTiles_PolkaDot sWallpaperTilemap_PolkaDot sWallpaperPalettes_Pokecenter sWallpaperTiles_Pokecenter sWallpaperTilemap_Pokecenter sWallpaperPalettes_Machine sWallpaperTiles_Machine sWallpaperTilemap_Machine sWallpaperPalettes_Plain sWallpaperTiles_Plain sWallpaperTilemap_Plain sWallpaperTilemap_Unused sBoxTitleColors sWallpapers sArrow_Gfx sWallpaperPalettes_Zigzagoon sWallpaperTiles_Zigzagoon sWallpaperTilemap_Zigzagoon sWallpaperPalettes_Screen sWallpaperTiles_Screen sWallpaperTilemap_Screen sWallpaperPalettes_Diagonal sWallpaperTiles_Diagonal sWallpaperTilemap_Diagonal sWallpaperPalettes_Block sWallpaperTiles_Block sWallpaperTilemap_Block sWallpaperPalettes_Pokecenter2 sWallpaperTiles_Pokecenter2 sWallpaperTilemap_Pokecenter2 sWallpaperPalettes_Frame sWallpaperTiles_Frame sWallpaperTilemap_Frame sWallpaperPalettes_Blank sWallpaperTiles_Blank sWallpaperTilemap_Blank sWallpaperPalettes_Circles sWallpaperTiles_Circles sWallpaperTilemap_Circles sWallpaperPalettes_Azumarill sWallpaperTiles_Azumarill sWallpaperTilemap_Azumarill sWallpaperPalettes_Pikachu sWallpaperTiles_Pikachu sWallpaperTilemap_Pikachu sWallpaperPalettes_Legendary sWallpaperTiles_Legendary sWallpaperTilemap_Legendary sWallpaperPalettes_Dusclops sWallpaperTiles_Dusclops sWallpaperTilemap_Dusclops sWallpaperPalettes_Ludicolo sWallpaperTiles_Ludicolo sWallpaperTilemap_Ludicolo sWallpaperPalettes_Whiscash sWallpaperTiles_Whiscash sWallpaperTilemap_Whiscash sWallpaperIcon_Aqua sWallpaperIcon_Heart sWallpaperIcon_FiveStar sWallpaperIcon_Brick sWallpaperIcon_FourStar sWallpaperIcon_Asterisk sWallpaperIcon_Dot sWallpaperIcon_LineCircle sWallpaperIcon_PokeBall sWallpaperIcon_Maze sWallpaperIcon_Footprint sWallpaperIcon_BigAsterisk sWallpaperIcon_Circle sWallpaperIcon_Koffing sWallpaperIcon_Ribbon sWallpaperIcon_FourCircles sWallpaperIcon_Lotad sWallpaperIcon_Crystal sWallpaperIcon_Pichu sWallpaperIcon_Diglett sWallpaperIcon_Luvdisc sWallpaperIcon_StarInCircle sWallpaperIcon_Spinda sWallpaperIcon_Latis sWallpaperIcon_Minun sWallpaperIcon_Togepi sWallpaperIcon_Magma sWaldaWallpapers sWaldaWallpaperIcons sUnusedColor sSpriteSheet_Arrow sOamData_BoxTitle sAnim_BoxTitle_Left sAnim_BoxTitle_Right sAnims_BoxTitle sSpriteTemplate_BoxTitle sOamData_Arrow sAnim_Arrow_Left sAnim_Arrow_Right sAnims_Arrow sSpriteTemplate_Arrow sHandCursor_Pal sHandCursor_Gfx sHandCursorShadow_Gfx sRestrictedReleaseMoves sMenuTexts sWindowTemplate_MultiMove sItemInfoFrame_Gfx sOamData_ItemIcon sAffineAnim_ItemIcon_Small sAffineAnim_ItemIcon_Appear sAffineAnim_ItemIcon_Disappear sAffineAnim_ItemIcon_PickUp sAffineAnim_ItemIcon_PutDown sAffineAnim_ItemIcon_PutAway sAffineAnim_ItemIcon_Large sAffineAnims_ItemIcon sSpriteTemplate_ItemIcon sTilemapDimensions inputFuncs.9 placeChangeFuncs.10 sAnim_Cursor_Bouncing.2 sAnim_Cursor_Fist.5 sAnim_Cursor_Open.4 sAnim_Cursor_Still.3 sAnims_Cursor.6 sOamData_Cursor.0 sOamData_CursorShadow.1 sSpriteTemplate_Cursor.8 sSpriteTemplate_CursorShadow.7
#[allow(unused_imports)]
use crate::data::pokemon_storage_system::*;

pub(crate) static mut sItemIconGfxBuffer: crate::ffi::Align4<[u8; 392]> =
    crate::ffi::Align4([0; 392]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPreviousBoxOption: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sChooseBoxMenu: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStorage: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInPartyMenu: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurrentBoxOption: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDepositBoxId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWhichToReshow: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLastUsedBox: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingItemId: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedMovingMon: crate::ffi::Align4<[u8; 100]> = crate::ffi::Align4([0; 100]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCursorArea: i8 = 0i8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCursorPosition: i8 = 0i8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIsMonBeingMoved: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingMonOrigBoxId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingMonOrigBoxPos: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAutoActionOn: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedCursorPosition: u8 = 0u8;
pub(crate) static mut sMultiMove: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTilemapUtil: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNumTilemapUtilIds: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnkUtil: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gDummySpriteAffineAnimTable: u8;
    static mut gDummySpriteAnimTable: u8;
    static mut gFieldCallback: u8;
    static mut gKeyRepeatStartDelay: u8;
    static mut gLastViewedMonIndex: u8;
    static mut gMain: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonIconPaletteIndices: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyCount: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gPokemonStoragePtr: u8;
    static mut gReservedSpriteTileCount: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSineTable: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStorageSystemMenu_Gfx: u8;
    static mut gStorageSystemPartyMenu_Pal: u8;
    static mut gStorageSystemPartyMenu_Tilemap: u8;
    static mut gTasks: u8;
    static mut gText_Box: u8;
    static mut gText_EggNickname: u8;
    static mut gText_JustOnePkmn: u8;
    static mut gText_PartyFull: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
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
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn AddTextPrinterParameterized5(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
        a7: u8,
        a8: u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn AddWindow8Bit(a0: *mut u8) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlitBitmapRectToWindow4BitTo8Bit(
        a0: u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u16,
        a5: i32,
        a6: u16,
        a7: u16,
        a8: u16,
        a9: u16,
        a10: u8,
    );
    fn BoxMonRestorePP(a0: *mut u8);
    fn BoxMonToMon(a0: *mut u8, a1: *mut u8);
    fn BufferMonMarkingsMenuTiles();
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckForSpaceForDma3Request(a0: i16) -> i16;
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearDma3Requests();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ComputerScreenCloseEffect(a0: u16, a1: u16, a2: u8);
    fn ComputerScreenOpenEffect(a0: u16, a1: u16, a2: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyRectToBgTilemapBufferRect(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u8,
        a8: u8,
        a9: u8,
        a10: u8,
        a11: i16,
        a12: i16,
    );
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CopyWindowToVram8Bit(a0: u8, a1: u8);
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateBoxMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateMonMarkingComboSprite(a0: u16, a1: u16, a2: *mut u16) -> *mut u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoNamingScreen(
        a0: u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u32,
        a5: Option<unsafe extern "C" fn()>,
    );
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelBuffer8Bit(a0: u8, a1: u8);
    fn FillWindowPixelRect8Bit(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FlagClear(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonMarkingsMenu();
    fn FreeOamMatrix(a0: u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTileRanges();
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBgAttribute(a0: u8, a1: u8) -> u16;
    fn GetBoxMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetBoxMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetIconSpecies(a0: u16, a1: u32) -> u16;
    fn GetItemDescription(a0: u16) -> *mut u8;
    fn GetItemIconPicOrPalette(a0: u16, a1: u8) -> *mut u8;
    fn GetItemName(a0: u16) -> *mut u8;
    fn GetLevelFromBoxMonExp(a0: *mut u8) -> u8;
    fn GetMaxWidthInMenuTable(a0: *mut u8, a1: i32) -> i32;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonFrontSpritePal(a0: *mut u8) -> *mut u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetMonIconPtr(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetMonIconTiles(a0: u16, a1: u32) -> *mut u8;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn GetValidMonIconPalIndex(a0: u16) -> u8;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn GoToBagMenu(a0: u8, a1: u8, a2: Option<unsafe extern "C" fn()>);
    fn HandleMonMarkingsMenuInput() -> u8;
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitMonMarkingsMenu(a0: *mut u8);
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsComputerScreenCloseEffectActive() -> u8;
    fn IsComputerScreenOpenEffectActive() -> u8;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn ItemIsMail(a0: u16) -> u8;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadMonIconPalettes();
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpecialPokePic(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32, a4: u8);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadSpriteSheets(a0: *mut u8);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LockPlayerFieldControls();
    fn Menu_GetCursorPos() -> u8;
    fn Menu_MoveCursor(a0: i8) -> u8;
    fn Menu_MoveCursorNoWrapAround(a0: i8) -> u8;
    fn Menu_ProcessInput() -> i8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn OpenMonMarkingsMenu(a0: u8, a1: i16, a2: i16);
    fn PlaySE(a0: u16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut u8);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn RemoveWindow(a0: u8);
    fn RequestDma3Fill(a0: i32, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetBoxMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowPokemonSummaryScreen(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn ShowPokemonSummaryScreenHandleDeoxys(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut u8, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyPadded(a0: *mut u8, a1: *mut u8, a2: u8, a3: u16) -> *mut u8;
    fn StringFill(a0: *mut u8, a1: u8, a2: u16) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn StringLength_Multibyte(a0: *mut u8) -> u32;
    fn TransferPlttBuffer();
    fn TryLoadAllMonIconPalettesAtOffset(a0: u16);
    fn UnlockPlayerFieldControls();
    fn UpdateMonMarkingTiles(a0: u8, a1: *mut u8);
    fn UpdatePaletteFade() -> u8;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WriteSequenceToBgTilemapBuffer(
        a0: u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: i16,
    );
    fn ZeroBoxMonData(a0: *mut u8);
    fn ZeroMonData(a0: *mut u8);
    fn malloc_and_decompress(a0: *mut u8, a1: *mut u32) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawTextWindowAndBufferTiles(
    string: *mut u8,
    dst: *mut u8,
    zero1: u8,
    zero2: u8,
    bytesToBuffer: i32,
) {
    unsafe {
        let mut string = string;
        let mut dst = dst;
        let mut zero1 = zero1;
        let mut zero2 = zero2;
        let mut bytesToBuffer = bytesToBuffer;
        let mut i: i32 = 0i32;
        let mut tileBytesToBuffer: i32 = 0i32;
        let mut remainingBytes: i32 = 0i32;
        let mut windowId: u16 = 0u16;
        let mut txtColor = crate::ffi::Align4([0u8; 3]);
        let mut tileData1: *mut u8 = core::ptr::null_mut();
        let mut tileData2: *mut u8 = core::ptr::null_mut();
        let mut winTemplate = crate::ffi::Align4([0u8; 8]);
        (&raw mut winTemplate)
            .cast::<u8>()
            .wrapping_add(0)
            .write(0u8);
        (((&raw mut winTemplate).cast::<u8>()).wrapping_add(3)).write(24u8);
        (((&raw mut winTemplate).cast::<u8>()).wrapping_add(4)).write(2u8);
        windowId = AddWindow((&raw mut winTemplate).cast::<u8>());
        FillWindowPixelBuffer(
            ((windowId) as u8),
            ((((zero2) as i32) | (((zero2) as i32) << 4)) as u8),
        );
        tileData1 = ((GetWindowAttribute(((windowId) as u8), 7u8)) as usize as *mut u8);
        tileData2 = (tileData1).wrapping_offset(
            ((((((&raw mut winTemplate).cast::<u8>()).wrapping_add(3)).read()) as i32)
                .wrapping_mul(crate::c::div_i32(256i32, 8i32))) as isize,
        );
        if !((zero1) != 0) {
            ((&raw mut txtColor).cast::<u8>()).write(0u8);
        } else {
            ((&raw mut txtColor).cast::<u8>()).write(zero2);
        }
        (((&raw mut txtColor).cast::<u8>()).wrapping_offset(1)).write(15u8);
        (((&raw mut txtColor).cast::<u8>()).wrapping_offset(2)).write(14u8);
        AddTextPrinterParameterized4(
            ((windowId) as u8),
            1u8,
            0u8,
            1u8,
            0u8,
            0u8,
            (&raw mut txtColor).cast::<u8>(),
            (-1i8),
            string,
        );
        tileBytesToBuffer = bytesToBuffer;
        if ((tileBytesToBuffer) as u32) > 6u32 {
            tileBytesToBuffer = 6i32;
        }
        remainingBytes = (bytesToBuffer).wrapping_sub(6i32);
        if tileBytesToBuffer > 0i32 {
            {
                i = tileBytesToBuffer;
                'l1: loop {
                    if !(i != 0i32) {
                        break 'l1;
                    }
                    'l2: {
                        'l3: loop {
                            'l4: {
                                'l5: loop {
                                    'l6: {
                                        CpuSet(
                                            tileData1,
                                            dst,
                                            ((0i32
                                                | (crate::c::div_i32(
                                                    128i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l5;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                        'l7: loop {
                            'l8: {
                                'l9: loop {
                                    'l10: {
                                        CpuSet(
                                            tileData2,
                                            (dst).wrapping_offset(128),
                                            ((0i32
                                                | (crate::c::div_i32(
                                                    128i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l9;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                        tileData1 = (tileData1).wrapping_offset(128);
                        tileData2 = (tileData2).wrapping_offset(128);
                        dst = (dst).wrapping_offset(256);
                    }
                    i = (i).wrapping_sub(1);
                }
            }
        }
        if remainingBytes > 0i32 {
            'l11: loop {
                'l12: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp)
                            .write_volatile((((((zero2) as i32) << 4) | ((zero2) as i32)) as u16));
                        'l13: loop {
                            'l14: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    dst,
                                    (16777216u32
                                        | (crate::c::div_u32(
                                            ((remainingBytes) as u32).wrapping_mul(256u32),
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l13;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l11;
                }
            }
        }
        RemoveWindow(((windowId) as u8));
    }
}
pub(crate) unsafe extern "C" fn UnusedDrawTextWindow(
    string: *mut u8,
    dst: *mut u8,
    offset: u16,
    bgColor: u8,
    fgColor: u8,
    shadowColor: u8,
) {
    unsafe {
        let mut string = string;
        let mut dst = dst;
        let mut offset = offset;
        let mut bgColor = bgColor;
        let mut fgColor = fgColor;
        let mut shadowColor = shadowColor;
        let mut tilesSize: u32 = 0u32;
        let mut windowId: u8 = 0u8;
        let mut txtColor = crate::ffi::Align4([0u8; 3]);
        let mut tileData1: *mut u8 = core::ptr::null_mut();
        let mut tileData2: *mut u8 = core::ptr::null_mut();
        let mut winTemplate = crate::ffi::Align4([0u8; 8]);
        (&raw mut winTemplate)
            .cast::<u8>()
            .wrapping_add(0)
            .write(0u8);
        (((&raw mut winTemplate).cast::<u8>()).wrapping_add(3))
            .write(((StringLength_Multibyte(string)) as u8));
        (((&raw mut winTemplate).cast::<u8>()).wrapping_add(4)).write(2u8);
        tilesSize = (((((((&raw mut winTemplate).cast::<u8>()).wrapping_add(3)).read()) as i32)
            .wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u32);
        windowId = ((AddWindow((&raw mut winTemplate).cast::<u8>())) as u8);
        FillWindowPixelBuffer(
            windowId,
            ((((bgColor) as i32) | (((bgColor) as i32) << 4)) as u8),
        );
        tileData1 = ((GetWindowAttribute(windowId, 7u8)) as usize as *mut u8);
        tileData2 = (tileData1).wrapping_offset(
            ((((((&raw mut winTemplate).cast::<u8>()).wrapping_add(3)).read()) as i32)
                .wrapping_mul(crate::c::div_i32(256i32, 8i32))) as isize,
        );
        ((&raw mut txtColor).cast::<u8>()).write(bgColor);
        (((&raw mut txtColor).cast::<u8>()).wrapping_offset(1)).write(fgColor);
        (((&raw mut txtColor).cast::<u8>()).wrapping_offset(2)).write(shadowColor);
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            0u8,
            2u8,
            0u8,
            0u8,
            (&raw mut txtColor).cast::<u8>(),
            (-1i8),
            string,
        );
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            tileData1,
                            dst,
                            (0u32
                                | (crate::c::div_u32(
                                    tilesSize,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            tileData2,
                            (dst).wrapping_offset(((offset) as i32) as isize * 1),
                            (0u32
                                | (crate::c::div_u32(
                                    tilesSize,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        RemoveWindow(windowId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountMonsInBox(boxId: u8) -> u8 {
    unsafe {
        let mut boxId = boxId;
        let mut i: u16 = 0u16;
        let mut count: u16 = 0u16;
        {
            i = 0u16;
            count = 0u16;
            'l1: loop {
                if !(((i) as i32) < 30i32) {
                    break 'l1;
                }
                'l2: {
                    if GetBoxMonDataAt(boxId, ((i) as u8), 11i32) != 0u32 {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((count) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFirstFreeBoxSpot(boxId: u8) -> i16 {
    unsafe {
        let mut boxId = boxId;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 30i32) {
                    break 'l1;
                }
                'l2: {
                    if GetBoxMonDataAt(boxId, ((i) as u8), 11i32) == 0u32 {
                        return ((i) as i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyNonEggMons() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut count: u16 = 0u16;
        {
            i = 0u16;
            count = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        11i32,
                    ) != 0u32)
                        && (!((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            45i32,
                        )) != 0))
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((count) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyAliveNonEggMonsExcept(slotToIgnore: u8) -> u8 {
    unsafe {
        let mut slotToIgnore = slotToIgnore;
        let mut i: u16 = 0u16;
        let mut count: u16 = 0u16;
        {
            i = 0u16;
            count = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((i) as i32) != ((slotToIgnore) as i32))
                        && (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            11i32,
                        ) != 0u32))
                        && (!((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            45i32,
                        )) != 0)))
                        && (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            57i32,
                        ) != 0u32)
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((count) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyAliveNonEggMons_IgnoreVar0x8004Slot() -> u16 {
    unsafe {
        return ((CountPartyAliveNonEggMonsExcept(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
        )) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyMons() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut count: u16 = 0u16;
        {
            i = 0u16;
            count = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        11i32,
                    ) != 0u32
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((count) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCopyAndFillWithSpaces(
    dst: *mut u8,
    src: *mut u8,
    n: u16,
) -> *mut u8 {
    unsafe {
        let mut dst = dst;
        let mut src = src;
        let mut n = n;
        let mut str: *mut u8 = core::ptr::null_mut();
        {
            str = StringCopy(dst, src);
            'l1: loop {
                if !(((str) as usize) < (((dst).wrapping_offset(((n) as i32) as isize)) as usize)) {
                    break 'l1;
                }
                'l2: {
                    (str).write(0u8);
                }
                str = (str).wrapping_offset(1);
            }
        }
        (str).write(255u8);
        return str;
    }
}
pub(crate) unsafe extern "C" fn UnusedWriteRectCpu(
    dest: *mut u16,
    dest_left: u16,
    dest_top: u16,
    src: *mut u16,
    src_left: u16,
    src_top: u16,
    dest_width: u16,
    dest_height: u16,
    src_width: u16,
) {
    unsafe {
        let mut dest = dest;
        let mut dest_left = dest_left;
        let mut dest_top = dest_top;
        let mut src = src;
        let mut src_left = src_left;
        let mut src_top = src_top;
        let mut dest_width = dest_width;
        let mut dest_height = dest_height;
        let mut src_width = src_width;
        let mut i: u16 = 0u16;
        dest_width = ((((dest_width) as i32).wrapping_mul(2i32)) as u16);
        dest = (dest).wrapping_offset(
            ((((dest_top) as i32).wrapping_mul(32i32)).wrapping_add(((dest_left) as i32))) as isize,
        );
        src = (src).wrapping_offset(
            ((((src_top) as i32).wrapping_mul(((src_width) as i32)))
                .wrapping_add(((src_left) as i32))) as isize,
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((dest_height) as i32)) {
                    break 'l1;
                }
                'l2: {
                    'l3: loop {
                        'l4: {
                            'l5: loop {
                                'l6: {
                                    CpuSet(
                                        (src).cast::<u8>(),
                                        (dest).cast::<u8>(),
                                        ((0i32
                                            | (crate::c::div_i32(
                                                ((dest_width) as i32),
                                                crate::c::div_i32(16i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l5;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                    dest = (dest).wrapping_offset(32);
                    src = (src).wrapping_offset(((src_width) as i32) as isize);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UnusedWriteRectDma(
    dest: *mut u16,
    dest_left: u16,
    dest_top: u16,
    width: u16,
    height: u16,
) {
    unsafe {
        let mut dest = dest;
        let mut dest_left = dest_left;
        let mut dest_top = dest_top;
        let mut width = width;
        let mut height = height;
        let mut i: u16 = 0u16;
        dest = (dest).wrapping_offset(
            ((((dest_top) as i32).wrapping_mul(32i32)).wrapping_add(((dest_left) as i32))) as isize,
        );
        width = ((((width) as i32).wrapping_mul(2i32)) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((height) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut _dest: *mut u8 = (dest).cast::<u8>();
                    let mut _size: u32 = ((width) as u32);
                    'l3: loop {
                        if !((1i32) != 0) {
                            break 'l3;
                        }
                        if _size <= 4096u32 {
                            'l4: loop {
                                'l5: {
                                    {
                                        let mut tmp: u16 = 0u16;
                                        (&raw mut tmp).write_volatile(0u16);
                                        'l6: loop {
                                            'l7: {
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
                                                                ((crate::c::div_i32(16i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l6;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                            break 'l3;
                        }
                        'l8: loop {
                            'l9: {
                                {
                                    let mut tmp: u16 = 0u16;
                                    (&raw mut tmp).write_volatile(0u16);
                                    'l10: loop {
                                        'l11: {
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
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l10;
                                        }
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l8;
                            }
                        }
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                    }
                }
                dest = (dest).wrapping_offset(32);
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PCMainMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                CreateMainMenu(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15),
                );
                LoadMessageBoxAndBorderGfx();
                DrawDialogueFrame(0u8, 0u8);
                FillWindowPixelBuffer(0u8, 17u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    (((((&raw const sMainMenuTexts).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                                as i32) as isize
                                * 8,
                        ))
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read(),
                    255u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                CopyWindowToVram(0u8, 3u8);
                CopyWindowToVram(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    3u8,
                );
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsWeatherNotFadingIn()) != 0 {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                    .write(((Menu_ProcessInput()) as i16));
                'l2: {
                    let __sw4 = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                        .read()) as i32);
                    let __matched = __sw4 == (-2i32) || __sw4 == (-1i32) || __sw4 == 4i32;
                    if __sw4 == (-2i32) {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
                        );
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0)
                            && ((({
                                let __p5 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                                let __t6 = ((__p5).read()).wrapping_sub(1);
                                (__p5).write(__t6);
                                __t6
                            }) as i32)
                                < 0i32)
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                .write(4i16);
                        }
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 128i32)
                            != 0)
                            && ((({
                                let __p7 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                                let __t8 = ((__p7).read()).wrapping_add(1);
                                (__p7).write(__t8);
                                __t8
                            }) as i32)
                                > 4i32)
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                .write(0i16);
                        }
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            != ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32)
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                    .read(),
                            );
                            FillWindowPixelBuffer(0u8, 17u8);
                            AddTextPrinterParameterized2(
                                0u8,
                                1u8,
                                (((((&raw const sMainMenuTexts).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                        .read()) as i32)
                                        as isize
                                        * 8,
                                ))
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read(),
                                0u8,
                                None,
                                2u8,
                                1u8,
                                3u8,
                            );
                        }
                        break 'l2;
                    }
                    if __sw4 == (-1i32) || __sw4 == 4i32 {
                        ClearStdWindowAndFrame(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as u8),
                            1u8,
                        );
                        UnlockPlayerFieldControls();
                        ScriptContext_Enable();
                        RemoveWindow(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as u8),
                        );
                        DestroyTask(taskId);
                        break 'l2;
                    }
                    if !__matched {
                        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            == 0i32)
                            && (((CountPartyMons()) as i32) == 6i32)
                        {
                            FillWindowPixelBuffer(0u8, 17u8);
                            AddTextPrinterParameterized2(
                                0u8,
                                1u8,
                                (&raw mut gText_PartyFull).cast::<u8>(),
                                0u8,
                                None,
                                2u8,
                                1u8,
                                3u8,
                            );
                            (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                        } else {
                            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32)
                                == 1i32)
                                && (((CountPartyMons()) as i32) == 1i32)
                            {
                                FillWindowPixelBuffer(0u8, 17u8);
                                AddTextPrinterParameterized2(
                                    0u8,
                                    1u8,
                                    (&raw mut gText_JustOnePkmn).cast::<u8>(),
                                    0u8,
                                    None,
                                    2u8,
                                    1u8,
                                    3u8,
                                );
                                (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                            } else {
                                FadeScreen(1u8, 0i8);
                                (((task).wrapping_add(8)).cast::<i16>()).write(4i16);
                            }
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    FillWindowPixelBuffer(0u8, 17u8);
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        (((((&raw const sMainMenuTexts).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                    .read()) as i32) as isize
                                    * 8,
                            ))
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read(),
                        0u8,
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                    (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 64i32)
                        != 0
                    {
                        if (({
                            let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                            let __t10 = ((__p9).read()).wrapping_sub(1);
                            (__p9).write(__t10);
                            __t10
                        }) as i32)
                            < 0i32
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                .write(4i16);
                        }
                        Menu_MoveCursor((-1i8));
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                            .write(((Menu_GetCursorPos()) as i16));
                        FillWindowPixelBuffer(0u8, 17u8);
                        AddTextPrinterParameterized2(
                            0u8,
                            1u8,
                            (((((&raw const sMainMenuTexts).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                    .read()) as i32) as isize
                                    * 8,
                            ))
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read(),
                            0u8,
                            None,
                            2u8,
                            1u8,
                            3u8,
                        );
                        (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 128i32)
                            != 0
                        {
                            if (({
                                let __p11 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                                let __t12 = ((__p11).read()).wrapping_add(1);
                                (__p11).write(__t12);
                                __t12
                            }) as i32)
                                >= 4i32
                            {
                                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                    .write(0i16);
                            }
                            Menu_MoveCursor(1i8);
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                .write(((Menu_GetCursorPos()) as i16));
                            FillWindowPixelBuffer(0u8, 17u8);
                            AddTextPrinterParameterized2(
                                0u8,
                                1u8,
                                (((((&raw const sMainMenuTexts).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                        .read()) as i32)
                                        as isize
                                        * 8,
                                ))
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read(),
                                0u8,
                                None,
                                2u8,
                                1u8,
                                3u8,
                            );
                            (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                        }
                    }
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
                    CleanupOverworldWindowsAndTilemaps();
                    EnterPokeStorage(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as u8),
                    );
                    RemoveWindow(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as u8),
                    );
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokemonStorageSystemPC() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_PCMainMenu), 80u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn FieldTask_ReturnToPcMenu() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut vblankCb: Option<unsafe extern "C" fn()> = (((&raw mut gMain).cast::<u8>())
            .wrapping_add(12)
            .cast::<Option<unsafe extern "C" fn()>>())
        .read();
        SetVBlankCallback(None);
        taskId = CreateTask(Some(Task_PCMainMenu), 80u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((((&raw mut sPreviousBoxOption).cast::<u8>().cast::<u8>()).read()) as i16));
        Task_PCMainMenu(taskId);
        SetVBlankCallback(vblankCb);
        FadeInFromBlack();
    }
}
pub(crate) unsafe extern "C" fn CreateMainMenu(whichMenu: u8, windowIdPtr: *mut i16) {
    unsafe {
        let mut whichMenu = whichMenu;
        let mut windowIdPtr = windowIdPtr;
        let mut windowId: i16 = 0i16;
        let mut template = crate::ffi::Align4([0u8; 8]);
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sWindowTemplate_MainMenu)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(
            ((GetMaxWidthInMenuTable(
                ((&raw const sMainMenuTexts).cast::<u8>().cast_mut()).cast::<u8>(),
                5i32,
            )) as u8),
        );
        windowId = ((AddWindow((&raw mut template).cast::<u8>())) as i16);
        DrawStdWindowFrame(((windowId) as u8), 0u8);
        PrintMenuTable(
            ((windowId) as u8),
            5u8,
            ((&raw const sMainMenuTexts).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        InitMenuInUpperLeftCornerNormal(((windowId) as u8), 5u8, whichMenu);
        (windowIdPtr).write(windowId);
    }
}
pub(crate) unsafe extern "C" fn CB2_ExitPokeStorage() {
    unsafe {
        ((&raw mut sPreviousBoxOption).cast::<u8>().cast::<u8>()).write(GetCurrentBoxOption());
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldTask_ReturnToPcMenu));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn StorageSystemGetNextMonIndex(
    r#box: *mut u8,
    startIdx: i8,
    stopIdx: u8,
    mode: u8,
) -> i16 {
    unsafe {
        let mut r#box = r#box;
        let mut startIdx = startIdx;
        let mut stopIdx = stopIdx;
        let mut mode = mode;
        let mut i: i16 = 0i16;
        let mut direction: i16 = 0i16;
        if (((mode) as i32) == 0i32) || (((mode) as i32) == 1i32) {
            direction = 1i16;
        } else {
            direction = (-1i16);
        }
        if (((mode) as i32) == 1i32) || (((mode) as i32) == 3i32) {
            {
                i = ((((startIdx) as i32).wrapping_add(((direction) as i32))) as i16);
                'l1: loop {
                    if !((((i) as i32) >= 0i32) && (((i) as i32) <= ((stopIdx) as i32))) {
                        break 'l1;
                    }
                    'l2: {
                        if GetBoxMonData2(
                            (r#box).wrapping_offset(((i) as i32) as isize * 80),
                            11i32,
                        ) != 0u32
                        {
                            return i;
                        }
                    }
                    i = ((((i) as i32).wrapping_add(((direction) as i32))) as i16);
                }
            }
        } else {
            {
                i = ((((startIdx) as i32).wrapping_add(((direction) as i32))) as i16);
                'l3: loop {
                    if !((((i) as i32) >= 0i32) && (((i) as i32) <= ((stopIdx) as i32))) {
                        break 'l3;
                    }
                    'l4: {
                        if (GetBoxMonData2(
                            (r#box).wrapping_offset(((i) as i32) as isize * 80),
                            11i32,
                        ) != 0u32)
                            && (!((GetBoxMonData2(
                                (r#box).wrapping_offset(((i) as i32) as isize * 80),
                                45i32,
                            )) != 0))
                        {
                            return i;
                        }
                    }
                    i = ((((i) as i32).wrapping_add(((direction) as i32))) as i16);
                }
            }
        }
        return (-1i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokemonStorageSystem() {
    unsafe {
        let mut boxId: u16 = 0u16;
        let mut boxPosition: u16 = 0u16;
        SetCurrentBox(0u8);
        {
            boxId = 0u16;
            'l1: loop {
                if !(((boxId) as i32) < 14i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        boxPosition = 0u16;
                        'l3: loop {
                            if !(((boxPosition) as i32) < 30i32) {
                                break 'l3;
                            }
                            'l4: {
                                ZeroBoxMonAt(((boxId) as u8), ((boxPosition) as u8));
                            }
                            boxPosition = (boxPosition).wrapping_add(1);
                        }
                    }
                }
                boxId = (boxId).wrapping_add(1);
            }
        }
        {
            boxId = 0u16;
            'l5: loop {
                if !(((boxId) as i32) < 14i32) {
                    break 'l5;
                }
                'l6: {
                    let mut dest: *mut u8 = StringCopy(
                        GetBoxNamePtr(((boxId) as u8)),
                        (&raw mut gText_Box).cast::<u8>(),
                    );
                    ConvertIntToDecimalStringN(
                        dest,
                        ((boxId) as i32).wrapping_add(1i32),
                        0i32,
                        2u8,
                    );
                }
                boxId = (boxId).wrapping_add(1);
            }
        }
        {
            boxId = 0u16;
            'l7: loop {
                if !(((boxId) as i32) < 14i32) {
                    break 'l7;
                }
                'l8: {
                    SetBoxWallpaper(
                        ((boxId) as u8),
                        ((crate::c::rem_i32(((boxId) as i32), 4i32)) as u8),
                    );
                }
                boxId = (boxId).wrapping_add(1);
            }
        }
        ResetWaldaWallpaper();
    }
}
pub(crate) unsafe extern "C" fn LoadChooseBoxMenuGfx(
    menu: *mut u8,
    tileTag: u16,
    palTag: u16,
    subpriority: u8,
    loadPal: u32,
) {
    unsafe {
        let mut menu = menu;
        let mut tileTag = tileTag;
        let mut palTag = palTag;
        let mut subpriority = subpriority;
        let mut loadPal = loadPal;
        let mut palette = crate::ffi::Align4([0u8; 8]);
        (&raw mut palette)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const sChooseBoxMenu_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut palette)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(palTag);
        let mut sheets = crate::ffi::Align4([0u8; 24]);
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                ((&raw const sChooseBoxMenuCenter_Gfx)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(4)
            .cast::<u16>()
            .write(2048u16);
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(6)
            .cast::<u16>()
            .write(tileTag);
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(((&raw const sChooseBoxMenuSides_Gfx).cast::<u8>().cast_mut()).cast::<u8>());
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(4)
            .cast::<u16>()
            .write(384u16);
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(6)
            .cast::<u16>()
            .write(((((tileTag) as i32).wrapping_add(1i32)) as u16));
        if (loadPal) != 0 {
            LoadSpritePalette((&raw mut palette).cast::<u8>());
        }
        LoadSpriteSheets((&raw mut sheets).cast::<u8>());
        ((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).write(menu);
        ((menu).wrapping_add(576).cast::<u16>()).write(tileTag);
        ((menu).wrapping_add(578).cast::<u16>()).write(palTag);
        ((menu).wrapping_add(582)).write(subpriority);
        ((menu).wrapping_add(572).cast::<u32>()).write(loadPal);
    }
}
pub(crate) unsafe extern "C" fn FreeChooseBoxMenu() {
    unsafe {
        if (((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(572)
            .cast::<u32>())
        .read())
            != 0
        {
            FreeSpritePaletteByTag(
                ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(578)
                    .cast::<u16>())
                .read(),
            );
        }
        FreeSpriteTilesByTag(
            ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(576)
                .cast::<u16>())
            .read(),
        );
        FreeSpriteTilesByTag(
            ((((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(576)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(1i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateChooseBoxMenuSprites(curBox: u8) {
    unsafe {
        let mut curBox = curBox;
        ChooseBoxMenu_CreateSprites(curBox);
    }
}
pub(crate) unsafe extern "C" fn DestroyChooseBoxMenuSprites() {
    unsafe {
        ChooseBoxMenu_DestroySprites();
    }
}
pub(crate) unsafe extern "C" fn HandleChooseBoxMenuInput() -> u8 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            PlaySE(5u16);
            return 201u8;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            return ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(580))
            .read();
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            PlaySE(5u16);
            ChooseBoxMenu_MoveLeft();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 16i32)
                != 0
            {
                PlaySE(5u16);
                ChooseBoxMenu_MoveRight();
            }
        }
        return 200u8;
    }
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_CreateSprites(curBox: u8) {
    unsafe {
        let mut curBox = curBox;
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut template = crate::ffi::Align4([0u8; 24]);
        let mut oamData = crate::ffi::Align4([0u8; 8]);
        crate::c::bf_write(
            ((&raw mut oamData).cast::<u8>()).wrapping_add(3),
            6,
            2,
            (3u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut oamData).cast::<u8>()).wrapping_add(5),
            4,
            4,
            (1u16) as i32,
        );
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned({
                let mut __lit1 = crate::ffi::Align4([0u8; 24]);
                (&raw mut __lit1)
                    .cast::<u8>()
                    .wrapping_add(0)
                    .cast::<u16>()
                    .write(0u16);
                (&raw mut __lit1)
                    .cast::<u8>()
                    .wrapping_add(2)
                    .cast::<u16>()
                    .write(0u16);
                (&raw mut __lit1)
                    .cast::<u8>()
                    .wrapping_add(4)
                    .cast::<*mut u8>()
                    .write((&raw mut oamData).cast::<u8>());
                (&raw mut __lit1)
                    .cast::<u8>()
                    .wrapping_add(8)
                    .cast::<*mut *mut u8>()
                    .write(((&raw mut gDummySpriteAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
                (&raw mut __lit1)
                    .cast::<u8>()
                    .wrapping_add(12)
                    .cast::<*mut u8>()
                    .write(core::ptr::null_mut());
                (&raw mut __lit1)
                    .cast::<u8>()
                    .wrapping_add(16)
                    .cast::<*mut *mut u8>()
                    .write(
                        ((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>())
                            .cast::<*mut u8>(),
                    );
                (&raw mut __lit1)
                    .cast::<u8>()
                    .wrapping_add(20)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
                    .write(Some(SpriteCallbackDummy));
                (&raw mut __lit1)
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned()
            });
        ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(580))
            .write(curBox);
        (((&raw mut template).cast::<u8>()).cast::<u16>()).write(
            ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(576)
                .cast::<u16>())
            .read(),
        );
        (((&raw mut template).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(
            ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(578)
                .cast::<u16>())
            .read(),
        );
        spriteId = CreateSprite((&raw mut template).cast::<u8>(), 160i16, 96i16, 0u8);
        ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
        crate::c::bf_write(
            ((&raw mut oamData).cast::<u8>()).wrapping_add(1),
            6,
            2,
            (2u32) as i32,
        );
        crate::c::bf_write(
            ((&raw mut oamData).cast::<u8>()).wrapping_add(3),
            6,
            2,
            (1u32) as i32,
        );
        (((&raw mut template).cast::<u8>()).cast::<u16>()).write(
            ((((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(576)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(1i32)) as u16),
        );
        (((&raw mut template).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut *mut u8>())
        .write(
            ((&raw const sAnims_ChooseBoxMenu)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut anim: u16 = 0u16;
                    spriteId = CreateSprite(
                        (&raw mut template).cast::<u8>(),
                        124i16,
                        80i16,
                        ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(582))
                        .read(),
                    );
                    ((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    anim = 0u16;
                    if (((i) as i32) & 2i32) != 0 {
                        ((((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(32)
                        .cast::<i16>())
                        .write(196i16);
                        anim = 2u16;
                    }
                    if (((i) as i32) & 1i32) != 0 {
                        ((((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(34)
                        .cast::<i16>())
                        .write(112i16);
                        crate::c::bf_write(
                            (((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(3),
                            6,
                            2,
                            (0u32) as i32,
                        );
                        anim = (anim).wrapping_add(1);
                    }
                    StartSpriteAnim(
                        ((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        ((anim) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(CreateChooseBoxArrows(
                        ((((72i32).wrapping_mul(((i) as i32))).wrapping_add(124i32)) as u16),
                        88u16,
                        ((i) as u8),
                        0u8,
                        ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(582))
                        .read(),
                    ));
                    if !(((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .is_null()
                    {
                        (((((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(32))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((if ((i) as i32) == 0i32 { (-1i32) } else { 1i32 }) as i16));
                        ((((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(32))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_ChooseBoxArrow));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ChooseBoxMenu_PrintInfo();
    }
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_DestroySprites() {
    unsafe {
        let mut i: u16 = 0u16;
        if !(((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .is_null()
        {
            DestroySprite(
                ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
                .write(core::ptr::null_mut());
        }
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if !(((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .is_null()
                    {
                        DestroySprite(
                            ((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                        ((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    if !(((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .is_null()
                    {
                        DestroySprite(
                            ((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(32))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_MoveRight() {
    unsafe {
        if (({
            let __p1 = (((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(580);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 14i32
        {
            ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(580))
                .write(0u8);
        }
        ChooseBoxMenu_PrintInfo();
    }
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_MoveLeft() {
    unsafe {
        ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(580))
            .write(
                ((if ((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(580))
                .read()) as i32)
                    == 0i32
                {
                    13i32
                } else {
                    ((((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(580))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                }) as u8),
            );
        ChooseBoxMenu_PrintInfo();
    }
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_PrintInfo() {
    unsafe {
        let mut numBoxMonsText = crate::ffi::Align4([0u8; 16]);
        let mut template = crate::ffi::Align4([0u8; 8]);
        let mut windowId: u8 = 0u8;
        let mut boxName: *mut u8 = GetBoxNamePtr(
            ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(580))
                .read(),
        );
        let mut numInBox: u8 = CountMonsInBox(
            ((((&raw mut sChooseBoxMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(580))
                .read(),
        );
        let mut winTileData: u32 = 0u32;
        let mut center: i32 = 0i32;
        crate::c::memset((&raw mut template).cast::<u8>(), 0i32, 8u32);
        (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(8u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(4)).write(4u8);
        windowId = ((AddWindow((&raw mut template).cast::<u8>())) as u8);
        FillWindowPixelBuffer(windowId, 68u8);
        center = GetStringCenterAlignXOffset(1i32, boxName, 64i32);
        AddTextPrinterParameterized3(
            windowId,
            1u8,
            ((center) as u8),
            1u8,
            ((&raw const sChooseBoxMenu_TextColors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            (-1i8),
            boxName,
        );
        ConvertIntToDecimalStringN(
            (&raw mut numBoxMonsText).cast::<u8>(),
            ((numInBox) as i32),
            1i32,
            2u8,
        );
        StringAppend(
            (&raw mut numBoxMonsText).cast::<u8>(),
            ((&raw const sText_OutOf30).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        center = GetStringCenterAlignXOffset(1i32, (&raw mut numBoxMonsText).cast::<u8>(), 64i32);
        AddTextPrinterParameterized3(
            windowId,
            1u8,
            ((center) as u8),
            17u8,
            ((&raw const sChooseBoxMenu_TextColors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            (-1i8),
            (&raw mut numBoxMonsText).cast::<u8>(),
        );
        winTileData = GetWindowAttribute(windowId, 7u8);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((winTileData) as usize as *mut u8),
                            (((100728832i32) as usize as *mut u8).wrapping_offset(256))
                                .wrapping_offset(
                                    (((GetSpriteTileStartByTag(
                                        ((((&raw mut sChooseBoxMenu)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(576)
                                        .cast::<u16>())
                                        .read(),
                                    )) as i32)
                                        .wrapping_mul(32i32))
                                        as isize
                                        * 1,
                                ),
                            ((67108864i32
                                | (crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
                                    & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        RemoveWindow(windowId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ChooseBoxArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 3i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            if (({
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t5 = ((__p4).read()).wrapping_add(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                > 5i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PokeStorage() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        UnkUtil_Run();
        TransferPlttBuffer();
        SetGpuReg(
            24u8,
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(716)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_PokeStorage() {
    unsafe {
        RunTasks();
        DoScheduledBgTilemapCopiesToVram();
        ScrollBackground();
        UpdateCloseBoxButtonFlash();
        AnimateSprites();
        BuildOamBuffer();
    }
}
pub(crate) unsafe extern "C" fn EnterPokeStorage(boxOption: u8) {
    unsafe {
        let mut boxOption = boxOption;
        ResetTasks();
        ((&raw mut sCurrentBoxOption).cast::<u8>().cast::<u8>()).write(boxOption);
        ((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).write(Alloc(25284u32));
        if ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            SetMainCallback2(Some(CB2_ExitPokeStorage));
        } else {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .write(boxOption);
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .write(0u8);
            ((&raw mut sMovingItemId).cast::<u8>().cast::<u16>()).write(0u16);
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(CreateTask(Some(Task_InitPokeStorage), 3u8));
            ((&raw mut sLastUsedBox).cast::<u8>().cast::<u8>()).write(StorageGetCurrentBox());
            SetMainCallback2(Some(CB2_PokeStorage));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPokeStorage() {
    unsafe {
        ResetTasks();
        ((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).write(Alloc(25284u32));
        if ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            SetMainCallback2(Some(CB2_ExitPokeStorage));
        } else {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .write(((&raw mut sCurrentBoxOption).cast::<u8>().cast::<u8>()).read());
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .write(1u8);
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(CreateTask(Some(Task_InitPokeStorage), 3u8));
            SetMainCallback2(Some(CB2_PokeStorage));
        }
    }
}
pub(crate) unsafe extern "C" fn ResetAllBgCoords() {
    unsafe {
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        SetGpuReg(28u8, 0u16);
        SetGpuReg(30u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn ResetForPokeStorage() {
    unsafe {
        ResetPaletteFade();
        ResetSpriteData();
        FreeSpriteTileRanges();
        FreeAllSpritePalettes();
        ClearDma3Requests();
        ((&raw mut gReservedSpriteTileCount).cast::<u16>()).write(640u16);
        UnkUtil_Init(
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8),
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                .cast::<u8>(),
            crate::c::div_u32(160u32, 20u32),
        );
        ((&raw mut gKeyRepeatStartDelay).cast::<u16>()).write(20u16);
        ClearScheduledBgCopiesToVram();
        TilemapUtil_Init(3u8);
        TilemapUtil_SetMap(
            0u8,
            1u8,
            (((&raw const sPkmnData_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            8u16,
            4u16,
        );
        TilemapUtil_SetPos(0u8, 1u16, 0u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(711))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn InitStartingPosData() {
    unsafe {
        ClearSavedCursorPos();
        ((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).write(
            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .read()) as i32)
                == 1i32) as u8),
        );
        ((&raw mut sDepositBoxId).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SetMonIconTransparency() {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            == 3i32
        {
            SetGpuReg(80u8, 16128u16);
            SetGpuReg(82u8, 2823u16);
        }
        SetGpuReg(0u8, 8000u16);
    }
}
pub(crate) unsafe extern "C" fn SetPokeStorageTask(newFunc: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut newFunc = newFunc;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .read()) as i32) as isize
                * 40,
        ))
        .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(newFunc);
        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_InitPokeStorage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32;
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                SetGpuReg(0u8, 0u16);
                ResetForPokeStorage();
                if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                    .read())
                    != 0
                {
                    'l2: {
                        let __sw2 =
                            ((((&raw mut sWhichToReshow).cast::<u8>().cast::<u8>()).read()) as i32);
                        if __sw2 == 1i32 {
                            LoadSavedMovingMon();
                            break 'l2;
                        }
                        if __sw2 == 0i32 {
                            SetSelectionAfterSummaryScreen();
                            break 'l2;
                        }
                        if __sw2 == 2i32 {
                            GiveChosenBagItem();
                            break 'l2;
                        }
                    }
                }
                LoadPokeStorageMenuGfx();
                LoadWaveformSpritePalette();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((InitPokeStorageWindows()) != 0) {
                    SetPokeStorageTask(Some(Task_ChangeScreen));
                    return;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                PutWindowTilemap(0u8);
                ClearWindowTilemap(1u8);
                'l3: loop {
                    'l4: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l5: loop {
                                'l6: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((100663296i32) as usize as *mut u8),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                512i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
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
                LoadUserWindowBorderGfx(1u8, 11u16, 224u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetAllBgCoords();
                if !((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read())
                    != 0)
                {
                    InitStartingPosData();
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                InitMonIconFields();
                if !((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read())
                    != 0)
                {
                    InitCursor();
                } else {
                    InitCursorOnReopen();
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((MultiMove_Init()) != 0) {
                    SetPokeStorageTask(Some(Task_ChangeScreen));
                    return;
                } else {
                    SetScrollingBackground();
                    InitPokeStorageBg0();
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                InitPalettesAndSprites();
                break 'l1;
            }
            if __sw1 == 7i32 {
                InitSupplementalTilemaps();
                break 'l1;
            }
            if __sw1 == 8i32 {
                CreateInitBoxTask(StorageGetCurrentBox());
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (IsInitBoxActive()) != 0 {
                    return;
                }
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32)
                    != 3i32
                {
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3492))
                    .cast::<u16>())
                    .write(13u16);
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3492))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(56014u16);
                    InitMonMarkingsMenu(
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3492),
                    );
                    BufferMonMarkingsMenuTiles();
                } else {
                    CreateItemIconSprites();
                    InitCursorItemIcon();
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                SetMonIconTransparency();
                if !((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read())
                    != 0)
                {
                    BlendPalettes(4294967295u32, 16u8, 0u16);
                    SetPokeStorageTask(Some(Task_ShowPokeStorage));
                } else {
                    BlendPalettes(4294967295u32, 16u8, 0u16);
                    SetPokeStorageTask(Some(Task_ReshowPokeStorage));
                }
                SetVBlankCallback(Some(VBlankCB_PokeStorage));
                return;
            }
            if !__matched {
                return;
            }
        }
        let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
        (__p3).write(((__p3).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_ShowPokeStorage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                PlaySE(2u16);
                ComputerScreenOpenEffect(20u16, 0u16, 1u8);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsComputerScreenOpenEffectActive()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReshowPokeStorage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdatePaletteFade()) != 0) {
                    if (((((&raw mut sWhichToReshow).cast::<u8>().cast::<u8>()).read()) as i32)
                        == 2i32)
                        && (((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) != 0i32)
                    {
                        PrintMessage(28u8);
                        let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    } else {
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (!((IsDma3ManagerBusyWithBgCopy()) != 0))
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 3i32)
                        != 0)
                {
                    ClearBottomWindow();
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PokeStorageMain(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = ((HandleInput()) as i32);
                    if __sw2 == 1i32 {
                        PlaySE(5u16);
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                        break 'l2;
                    }
                    if __sw2 == 5i32 {
                        if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read()) as i32)
                            != 2i32)
                            && (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1))
                            .read()) as i32)
                                != 3i32)
                        {
                            PrintMessage(16u8);
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(3u8);
                        } else {
                            ClearSavedCursorPos();
                            SetPokeStorageTask(Some(Task_ShowPartyPokemon));
                        }
                        break 'l2;
                    }
                    if __sw2 == 6i32 {
                        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read()) as i32)
                            == 2i32
                        {
                            if ((IsMonBeingMoved()) != 0)
                                && ((ItemIsMail(
                                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(3302)
                                        .cast::<u16>())
                                    .read(),
                                )) != 0)
                            {
                                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .write(5u8);
                            } else {
                                SetPokeStorageTask(Some(Task_HidePartyPokemon));
                            }
                        } else {
                            if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1))
                            .read()) as i32)
                                == 3i32
                            {
                                SetPokeStorageTask(Some(Task_HidePartyPokemon));
                            }
                        }
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        SetPokeStorageTask(Some(Task_OnCloseBoxPressed));
                        break 'l2;
                    }
                    if __sw2 == 19i32 {
                        SetPokeStorageTask(Some(Task_OnBPressed));
                        break 'l2;
                    }
                    if __sw2 == 7i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_HandleBoxOptions));
                        break 'l2;
                    }
                    if __sw2 == 8i32 {
                        SetPokeStorageTask(Some(Task_OnSelectedMon));
                        break 'l2;
                    }
                    if __sw2 == 9i32 {
                        PlaySE(5u16);
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(714)
                            .cast::<i16>())
                        .write(((((StorageGetCurrentBox()) as i32).wrapping_add(1i32)) as i16));
                        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(714)
                            .cast::<i16>())
                        .read()) as i32)
                            >= 14i32
                        {
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(714)
                                .cast::<i16>())
                            .write(0i16);
                        }
                        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read()) as i32)
                            != 3i32
                        {
                            SetUpScrollToBox(
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(714)
                                    .cast::<i16>())
                                .read()) as u8),
                            );
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(2u8);
                        } else {
                            TryHideItemAtCursor();
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(10u8);
                        }
                        break 'l2;
                    }
                    if __sw2 == 10i32 {
                        PlaySE(5u16);
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(714)
                            .cast::<i16>())
                        .write(((((StorageGetCurrentBox()) as i32).wrapping_sub(1i32)) as i16));
                        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(714)
                            .cast::<i16>())
                        .read()) as i32)
                            < 0i32
                        {
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(714)
                                .cast::<i16>())
                            .write(13i16);
                        }
                        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read()) as i32)
                            != 3i32
                        {
                            SetUpScrollToBox(
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(714)
                                    .cast::<i16>())
                                .read()) as u8),
                            );
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(2u8);
                        } else {
                            TryHideItemAtCursor();
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(10u8);
                        }
                        break 'l2;
                    }
                    if __sw2 == 11i32 {
                        if !((IsRemovingLastPartyMon()) != 0) {
                            if (ItemIsMail(
                                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(3302)
                                    .cast::<u16>())
                                .read(),
                            )) != 0
                            {
                                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .write(5u8);
                            } else {
                                PlaySE(5u16);
                                SetPokeStorageTask(Some(Task_DepositMenu));
                            }
                        } else {
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(4u8);
                        }
                        break 'l2;
                    }
                    if __sw2 == 13i32 {
                        if (IsRemovingLastPartyMon()) != 0 {
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(4u8);
                        } else {
                            PlaySE(5u16);
                            SetPokeStorageTask(Some(Task_MoveMon));
                        }
                        break 'l2;
                    }
                    if __sw2 == 14i32 {
                        if !((CanShiftMon()) != 0) {
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(4u8);
                        } else {
                            PlaySE(5u16);
                            SetPokeStorageTask(Some(Task_ShiftMon));
                        }
                        break 'l2;
                    }
                    if __sw2 == 12i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_WithdrawMon));
                        break 'l2;
                    }
                    if __sw2 == 15i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_PlaceMon));
                        break 'l2;
                    }
                    if __sw2 == 16i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_TakeItemForMoving));
                        break 'l2;
                    }
                    if __sw2 == 17i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_GiveMovingItemToMon));
                        break 'l2;
                    }
                    if __sw2 == 18i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_SwitchSelectedItem));
                        break 'l2;
                    }
                    if __sw2 == 20i32 {
                        PlaySE(5u16);
                        MultiMove_SetFunction(0u8);
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(7u8);
                        break 'l2;
                    }
                    if __sw2 == 22i32 {
                        MultiMove_SetFunction(1u8);
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(8u8);
                        break 'l2;
                    }
                    if __sw2 == 21i32 {
                        PlaySE(5u16);
                        MultiMove_SetFunction(2u8);
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(9u8);
                        break 'l2;
                    }
                    if __sw2 == 23i32 {
                        MultiMove_SetFunction(3u8);
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(7u8);
                        break 'l2;
                    }
                    if __sw2 == 25i32 {
                        PlaySE(5u16);
                        MultiMove_SetFunction(4u8);
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(9u8);
                        break 'l2;
                    }
                    if __sw2 == 26i32 {
                        PlaySE(5u16);
                        MultiMove_SetFunction(5u8);
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(7u8);
                        break 'l2;
                    }
                    if __sw2 == 24i32 {
                        PlaySE(32u16);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdateCursorPos()) != 0) {
                    if (IsCursorOnCloseBox()) != 0 {
                        StartFlashingCloseBoxButton();
                    } else {
                        StopFlashingCloseBoxButton();
                    }
                    if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3306))
                    .read())
                        != 0
                    {
                        StartDisplayMonMosaicEffect();
                    }
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((ScrollToBox()) != 0) {
                    SetCurrentBox(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(714)
                            .cast::<i16>())
                        .read()) as u8),
                    );
                    if (!((((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0))
                        && (!((IsMonBeingMoved()) != 0))
                    {
                        RefreshDisplayMon();
                        StartDisplayMonMosaicEffect();
                    }
                    if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32)
                        == 3i32
                    {
                        TryShowItemAtCursor();
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(11u8);
                    } else {
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                PlaySE(32u16);
                PrintMessage(13u8);
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(6u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                PlaySE(32u16);
                PrintMessage(22u8);
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((MultiMove_RunFunction()) != 0) {
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if !((MultiMove_RunFunction()) != 0) {
                    SetPokeStorageTask(Some(Task_MoveMon));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((MultiMove_RunFunction()) != 0) {
                    if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3306))
                    .read())
                        != 0
                    {
                        StartDisplayMonMosaicEffect();
                    }
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if !((IsItemIconAnimActive()) != 0) {
                    SetUpScrollToBox(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(714)
                            .cast::<i16>())
                        .read()) as u8),
                    );
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if !((IsItemIconAnimActive()) != 0) {
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowPartyPokemon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                SetUpDoShowPartyMenu();
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((DoShowPartyMenu()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HidePartyPokemon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                PlaySE(5u16);
                SetUpHidePartyMenu();
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((HidePartyMenu()) != 0) {
                    SetCursorBoxPosition(GetSavedCursorPos());
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((UpdateCursorPos()) != 0) {
                    if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3306))
                    .read())
                        != 0
                    {
                        StartDisplayMonMosaicEffect();
                    }
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OnSelectedMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                if !((IsDisplayMosaicActive()) != 0) {
                    PlaySE(5u16);
                    if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32)
                        != 3i32
                    {
                        PrintMessage(4u8);
                    } else {
                        if ((IsMovingItem()) != 0)
                            || (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3302)
                                .cast::<u16>())
                            .read()) as i32)
                                != 0i32)
                        {
                            PrintMessage(23u8);
                        } else {
                            PrintMessage(24u8);
                        }
                    }
                    AddMenu();
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsMenuLoading()) != 0) {
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                'l2: {
                    let __sw2 = ((HandleMenuInput()) as i32);
                    if __sw2 == (-1i32) || __sw2 == 0i32 {
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        if (IsRemovingLastPartyMon()) != 0 {
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(3u8);
                        } else {
                            PlaySE(5u16);
                            ClearBottomWindow();
                            SetPokeStorageTask(Some(Task_MoveMon));
                        }
                        break 'l2;
                    }
                    if __sw2 == 5i32 {
                        PlaySE(5u16);
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_PlaceMon));
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        if !((CanShiftMon()) != 0) {
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(3u8);
                        } else {
                            PlaySE(5u16);
                            ClearBottomWindow();
                            SetPokeStorageTask(Some(Task_ShiftMon));
                        }
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        PlaySE(5u16);
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_WithdrawMon));
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        if (IsRemovingLastPartyMon()) != 0 {
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(3u8);
                        } else {
                            if (ItemIsMail(
                                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(3302)
                                    .cast::<u16>())
                                .read(),
                            )) != 0
                            {
                                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .write(4u8);
                            } else {
                                PlaySE(5u16);
                                ClearBottomWindow();
                                SetPokeStorageTask(Some(Task_DepositMenu));
                            }
                        }
                        break 'l2;
                    }
                    if __sw2 == 7i32 {
                        if (IsRemovingLastPartyMon()) != 0 {
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(3u8);
                        } else {
                            if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3309))
                            .read())
                                != 0
                            {
                                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .write(5u8);
                            } else {
                                if (ItemIsMail(
                                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(3302)
                                        .cast::<u16>())
                                    .read(),
                                )) != 0
                                {
                                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                        .write(4u8);
                                } else {
                                    PlaySE(5u16);
                                    SetPokeStorageTask(Some(Task_ReleaseMon));
                                }
                            }
                        }
                        break 'l2;
                    }
                    if __sw2 == 6i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_ShowMonSummary));
                        break 'l2;
                    }
                    if __sw2 == 8i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_ShowMarkMenu));
                        break 'l2;
                    }
                    if __sw2 == 12i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_TakeItemForMoving));
                        break 'l2;
                    }
                    if __sw2 == 13i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_GiveMovingItemToMon));
                        break 'l2;
                    }
                    if __sw2 == 16i32 {
                        SetPokeStorageTask(Some(Task_ItemToBag));
                        break 'l2;
                    }
                    if __sw2 == 15i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_SwitchSelectedItem));
                        break 'l2;
                    }
                    if __sw2 == 14i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_GiveItemFromBag));
                        break 'l2;
                    }
                    if __sw2 == 17i32 {
                        SetPokeStorageTask(Some(Task_ShowItemInfo));
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                PlaySE(32u16);
                PrintMessage(13u8);
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(6u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                PlaySE(32u16);
                PrintMessage(17u8);
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(6u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                PlaySE(32u16);
                PrintMessage(22u8);
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_MoveMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                InitMonPlaceChange(0u8);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((DoMonPlaceChange()) != 0) {
                    if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
                        SetPokeStorageTask(Some(Task_HandleMovingMonFromParty));
                    } else {
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PlaceMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                InitMonPlaceChange(1u8);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((DoMonPlaceChange()) != 0) {
                    if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
                        SetPokeStorageTask(Some(Task_HandleMovingMonFromParty));
                    } else {
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShiftMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                InitMonPlaceChange(2u8);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((DoMonPlaceChange()) != 0) {
                    StartDisplayMonMosaicEffect();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WithdrawMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                if ((CalculatePlayerPartyCount()) as i32) == 6i32 {
                    PrintMessage(14u8);
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                } else {
                    SaveCursorPos();
                    InitMonPlaceChange(0u8);
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((DoMonPlaceChange()) != 0) {
                    SetMovingMonPriority(1u8);
                    SetUpDoShowPartyMenu();
                    let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((DoShowPartyMenu()) != 0) {
                    InitMonPlaceChange(1u8);
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((DoMonPlaceChange()) != 0) {
                    UpdatePartySlotColors();
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetPokeStorageTask(Some(Task_HidePartyPokemon));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DepositMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut boxId: u8 = 0u8;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                PrintMessage(6u8);
                LoadChooseBoxMenuGfx(
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7772),
                    10u16,
                    56007u16,
                    3u8,
                    0u32,
                );
                CreateChooseBoxMenuSprites(
                    ((&raw mut sDepositBoxId).cast::<u8>().cast::<u8>()).read(),
                );
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                boxId = HandleChooseBoxMenuInput();
                'l2: {
                    let __sw3 = ((boxId) as i32);
                    let __matched = __sw3 == 200i32 || __sw3 == 201i32;
                    if __sw3 == 200i32 {
                        break 'l2;
                    }
                    if __sw3 == 201i32 {
                        ClearBottomWindow();
                        DestroyChooseBoxMenuSprites();
                        FreeChooseBoxMenu();
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                        break 'l2;
                    }
                    if !__matched {
                        if (TryStorePartyMonInBox(boxId)) != 0 {
                            ((&raw mut sDepositBoxId).cast::<u8>().cast::<u8>()).write(boxId);
                            ClearBottomWindow();
                            DestroyChooseBoxMenuSprites();
                            FreeChooseBoxMenu();
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(2u8);
                        } else {
                            PrintMessage(8u8);
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(4u8);
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                CompactPartySlots();
                CompactPartySprites();
                let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((GetNumPartySpritesCompacting()) as i32) == 0i32 {
                    ResetSelectionAfterDeposit();
                    StartDisplayMonMosaicEffect();
                    UpdatePartySlotColors();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    PrintMessage(6u8);
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReleaseMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                PrintMessage(9u8);
                ShowYesNoWindow(1i8);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                'l2: {
                    let __sw3 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
                    if __sw3 == (-1i32) || __sw3 == 1i32 {
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                        break 'l2;
                    }
                    if __sw3 == 0i32 {
                        ClearBottomWindow();
                        InitCanReleaseMonVars();
                        InitReleaseMon();
                        let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                RunCanReleaseMon();
                if !((TryHideReleaseMon()) != 0) {
                    'l3: loop {
                        if !((1i32) != 0) {
                            break 'l3;
                        }
                        let mut canRelease: i8 = RunCanReleaseMon();
                        if ((canRelease) as i32) == 1i32 {
                            let __p5 =
                                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                            (__p5).write(((__p5).read()).wrapping_add(1));
                            break 'l3;
                        } else {
                            if !((canRelease) != 0) {
                                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .write(8u8);
                                break 'l3;
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                ReleaseMon();
                RefreshDisplayMonData();
                PrintMessage(10u8);
                let __p6 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    PrintMessage(11u8);
                    let __p7 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
                        CompactPartySlots();
                        CompactPartySprites();
                        let __p8 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    } else {
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(7u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                if ((GetNumPartySpritesCompacting()) as i32) == 0i32 {
                    RefreshDisplayMon();
                    StartDisplayMonMosaicEffect();
                    UpdatePartySlotColors();
                    let __p9 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                SetPokeStorageTask(Some(Task_PokeStorageMain));
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                PrintMessage(10u8);
                let __p10 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    PrintMessage(21u8);
                    let __p11 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    ReshowReleaseMon();
                    let __p12 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                __fall = true;
                if !((ResetReleaseMonSpritePtr()) != 0) {
                    TrySetCursorFistAnim();
                    PrintMessage(19u8);
                    let __p13 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    PrintMessage(20u8);
                    let __p14 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowMarkMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                PrintMessage(12u8);
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3492))
                .wrapping_add(4))
                .write(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3307))
                    .read(),
                );
                OpenMonMarkingsMenu(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3307))
                    .read(),
                    176i16,
                    16i16,
                );
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((HandleMonMarkingsMenuInput()) != 0) {
                    FreeMonMarkingsMenu();
                    ClearBottomWindow();
                    SetMonMarkings(
                        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3492))
                        .wrapping_add(4))
                        .read(),
                    );
                    RefreshDisplayMonData();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TakeItemForMoving(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                if !((ItemIsMail(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3302)
                        .cast::<u16>())
                    .read(),
                )) != 0)
                {
                    ClearBottomWindow();
                    let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p2).write(((__p2).read()).wrapping_add(1));
                } else {
                    SetPokeStorageTask(Some(Task_PrintCantStoreMail));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                StartCursorAnim(2u8);
                TakeItemFromMon(
                    ((if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
                        1i32
                    } else {
                        0i32
                    }) as u8),
                    GetCursorPosition(),
                );
                let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsItemIconAnimActive()) != 0) {
                    StartCursorAnim(3u8);
                    ClearBottomWindow();
                    RefreshDisplayMon();
                    PrintDisplayMonInfo();
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_GiveMovingItemToMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                ClearBottomWindow();
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                StartCursorAnim(2u8);
                GiveItemToMon(
                    ((if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
                        1i32
                    } else {
                        0i32
                    }) as u8),
                    GetCursorPosition(),
                );
                let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsItemIconAnimActive()) != 0) {
                    StartCursorAnim(0u8);
                    RefreshDisplayMon();
                    PrintDisplayMonInfo();
                    PrintMessage(28u8);
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    let __p5 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ItemToBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                if !((AddBagItem(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3302)
                        .cast::<u16>())
                    .read(),
                    1u16,
                )) != 0)
                {
                    PlaySE(32u16);
                    PrintMessage(26u8);
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(3u8);
                } else {
                    PlaySE(5u16);
                    MoveItemFromMonToBag(
                        ((if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
                            1i32
                        } else {
                            0i32
                        }) as u8),
                        GetCursorPosition(),
                    );
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsItemIconAnimActive()) != 0) {
                    PrintMessage(25u8);
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    RefreshDisplayMon();
                    PrintDisplayMonInfo();
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchSelectedItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                if !((ItemIsMail(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3302)
                        .cast::<u16>())
                    .read(),
                )) != 0)
                {
                    ClearBottomWindow();
                    let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p2).write(((__p2).read()).wrapping_add(1));
                } else {
                    SetPokeStorageTask(Some(Task_PrintCantStoreMail));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                StartCursorAnim(2u8);
                SwapItemsWithMon(
                    ((if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
                        1i32
                    } else {
                        0i32
                    }) as u8),
                    GetCursorPosition(),
                );
                let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsItemIconAnimActive()) != 0) {
                    StartCursorAnim(3u8);
                    RefreshDisplayMon();
                    PrintDisplayMonInfo();
                    PrintMessage(29u8);
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    let __p5 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowItemInfo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                ClearBottomWindow();
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PlaySE(6u16);
                    PrintItemDescription();
                    InitItemInfoWindow();
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((UpdateItemInfoWindowSlideIn()) != 0) {
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    let __p5 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    PlaySE(6u16);
                    let __p6 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((UpdateItemInfoWindowSlideOut()) != 0) {
                    let __p7 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CloseBoxWhileHoldingItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                PlaySE(5u16);
                PrintMessage(27u8);
                ShowYesNoWindow(0i8);
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                'l2: {
                    let __sw2 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
                    if __sw2 == (-1i32) || __sw2 == 1i32 {
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                        break 'l2;
                    }
                    if __sw2 == 0i32 {
                        if ((AddBagItem(
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8756)
                                .cast::<u16>())
                            .read(),
                            1u16,
                        )) as i32)
                            == 1i32
                        {
                            ClearBottomWindow();
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(3u8);
                        } else {
                            PrintMessage(26u8);
                            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .write(2u8);
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(5u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                MoveItemFromCursorToBag();
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsItemIconAnimActive()) != 0) {
                    StartCursorAnim(0u8);
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMovingMonFromParty(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                CompactPartySlots();
                CompactPartySprites();
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((GetNumPartySpritesCompacting()) as i32) == 0i32 {
                    UpdatePartySlotColors();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintCantStoreMail(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                PrintMessage(30u8);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleBoxOptions(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                PrintMessage(1u8);
                AddMenu();
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (IsMenuLoading()) != 0 {
                    return;
                }
                let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                'l2: {
                    let __sw4 = ((HandleMenuInput()) as i32);
                    if __sw4 == (-1i32) || __sw4 == 0i32 {
                        AnimateBoxScrollArrows(1u8);
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                        break 'l2;
                    }
                    if __sw4 == 11i32 {
                        PlaySE(5u16);
                        SetPokeStorageTask(Some(Task_NameBox));
                        break 'l2;
                    }
                    if __sw4 == 10i32 {
                        PlaySE(5u16);
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_HandleWallpapers));
                        break 'l2;
                    }
                    if __sw4 == 9i32 {
                        PlaySE(5u16);
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_JumpBox));
                        break 'l2;
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleWallpapers(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                AddWallpaperSetsMenu();
                PrintMessage(2u8);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsMenuLoading()) != 0) {
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1934)
                    .cast::<i16>())
                .write(HandleMenuInput());
                'l2: {
                    let __sw4 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1934)
                        .cast::<i16>())
                    .read()) as i32);
                    if __sw4 == (-1i32) {
                        AnimateBoxScrollArrows(1u8);
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                        break 'l2;
                    }
                    if __sw4 == 18i32 || __sw4 == 19i32 || __sw4 == 20i32 || __sw4 == 21i32 {
                        PlaySE(5u16);
                        RemoveMenu();
                        let __p5 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1934)
                            .cast::<i16>();
                        (__p5).write((((((__p5).read()) as i32).wrapping_sub(18i32)) as i16));
                        let __p6 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                        (__p6).write(((__p6).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 22i32 {
                        PlaySE(5u16);
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1936)
                            .cast::<i16>())
                        .write(16i16);
                        RemoveMenu();
                        ClearBottomWindow();
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(6u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    AddWallpapersMenu(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1934)
                            .cast::<i16>())
                        .read()) as u8),
                    );
                    PrintMessage(3u8);
                    let __p7 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1936)
                    .cast::<i16>())
                .write(HandleMenuInput());
                'l3: {
                    let __sw8 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1936)
                        .cast::<i16>())
                    .read()) as i32);
                    let __matched = __sw8 == (-2i32) || __sw8 == (-1i32);
                    if __sw8 == (-2i32) {
                        break 'l3;
                    }
                    if __sw8 == (-1i32) {
                        ClearBottomWindow();
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                        break 'l3;
                    }
                    if !__matched {
                        PlaySE(5u16);
                        ClearBottomWindow();
                        let __p9 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1936)
                            .cast::<i16>();
                        (__p9).write((((((__p9).read()) as i32).wrapping_sub(23i32)) as i16));
                        SetWallpaperForCurrentBox(
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1936)
                                .cast::<i16>())
                            .read()) as u8),
                        );
                        let __p10 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                        (__p10).write(((__p10).read()).wrapping_add(1));
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((DoWallpaperGfxChange()) != 0) {
                    AnimateBoxScrollArrows(1u8);
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetWallpaperForCurrentBox(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1936)
                            .cast::<i16>())
                        .read()) as u8),
                    );
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(5u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_JumpBox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                PrintMessage(5u8);
                LoadChooseBoxMenuGfx(
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7772),
                    10u16,
                    56007u16,
                    3u8,
                    0u32,
                );
                CreateChooseBoxMenuSprites(StorageGetCurrentBox());
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(714)
                    .cast::<i16>())
                .write(((HandleChooseBoxMenuInput()) as i16));
                'l2: {
                    let __sw3 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(714)
                        .cast::<i16>())
                    .read()) as i32);
                    let __matched = __sw3 == 200i32;
                    if __sw3 == 200i32 {
                        break 'l2;
                    }
                    if !__matched {
                        ClearBottomWindow();
                        DestroyChooseBoxMenuSprites();
                        FreeChooseBoxMenu();
                        if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(714)
                            .cast::<i16>())
                        .read()) as i32)
                            == 201i32)
                            || (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(714)
                                .cast::<i16>())
                            .read()) as i32)
                                == ((StorageGetCurrentBox()) as i32))
                        {
                            AnimateBoxScrollArrows(1u8);
                            SetPokeStorageTask(Some(Task_PokeStorageMain));
                        } else {
                            let __p4 =
                                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                            (__p4).write(((__p4).read()).wrapping_add(1));
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetUpScrollToBox(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(714)
                        .cast::<i16>())
                    .read()) as u8),
                );
                let __p5 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((ScrollToBox()) != 0) {
                    SetCurrentBox(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(714)
                            .cast::<i16>())
                        .read()) as u8),
                    );
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NameBox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                SaveMovingMon();
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdatePaletteFade()) != 0) {
                    ((&raw mut sWhichToReshow).cast::<u8>().cast::<u8>()).write(1u8);
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                        .write(2u8);
                    SetPokeStorageTask(Some(Task_ChangeScreen));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowMonSummary(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                InitSummaryScreenData();
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdatePaletteFade()) != 0) {
                    ((&raw mut sWhichToReshow).cast::<u8>().cast::<u8>()).write(0u8);
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                        .write(1u8);
                    SetPokeStorageTask(Some(Task_ChangeScreen));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_GiveItemFromBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdatePaletteFade()) != 0) {
                    ((&raw mut sWhichToReshow).cast::<u8>().cast::<u8>()).write(2u8);
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                        .write(3u8);
                    SetPokeStorageTask(Some(Task_ChangeScreen));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OnCloseBoxPressed(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                if (IsMonBeingMoved()) != 0 {
                    PlaySE(32u16);
                    PrintMessage(15u8);
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                } else {
                    if (IsMovingItem()) != 0 {
                        SetPokeStorageTask(Some(Task_CloseBoxWhileHoldingItem));
                    } else {
                        PlaySE(5u16);
                        PrintMessage(0u8);
                        ShowYesNoWindow(0i8);
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                'l2: {
                    let __sw2 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
                    if __sw2 == (-1i32) || __sw2 == 1i32 {
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                        break 'l2;
                    }
                    if __sw2 == 0i32 {
                        PlaySE(3u16);
                        ClearBottomWindow();
                        let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                        (__p3).write(((__p3).read()).wrapping_add(1));
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ComputerScreenCloseEffect(20u16, 0u16, 1u8);
                let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsComputerScreenCloseEffectActive()) != 0) {
                    UpdateBoxToSendMons();
                    ((&raw mut gPlayerPartyCount).cast::<u8>()).write(CalculatePlayerPartyCount());
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                        .write(0u8);
                    SetPokeStorageTask(Some(Task_ChangeScreen));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OnBPressed(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                if (IsMonBeingMoved()) != 0 {
                    PlaySE(32u16);
                    PrintMessage(15u8);
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                } else {
                    if (IsMovingItem()) != 0 {
                        SetPokeStorageTask(Some(Task_CloseBoxWhileHoldingItem));
                    } else {
                        PlaySE(5u16);
                        PrintMessage(18u8);
                        ShowYesNoWindow(0i8);
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 243i32)
                    != 0
                {
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                'l2: {
                    let __sw2 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
                    if __sw2 == 0i32 {
                        ClearBottomWindow();
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                        break 'l2;
                    }
                    if __sw2 == 1i32 || __sw2 == (-1i32) {
                        PlaySE(3u16);
                        ClearBottomWindow();
                        let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                        (__p3).write(((__p3).read()).wrapping_add(1));
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ComputerScreenCloseEffect(20u16, 0u16, 0u8);
                let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsComputerScreenCloseEffectActive()) != 0) {
                    UpdateBoxToSendMons();
                    ((&raw mut gPlayerPartyCount).cast::<u8>()).write(CalculatePlayerPartyCount());
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                        .write(0u8);
                    SetPokeStorageTask(Some(Task_ChangeScreen));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ChangeScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut boxMons: *mut u8 = core::ptr::null_mut();
        let mut mode: u8 = 0u8;
        let mut monIndex: u8 = 0u8;
        let mut maxMonIndex: u8 = 0u8;
        let mut screenChangeType: u8 =
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2)).read();
        if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            == 3i32)
            && (((IsMovingItem()) as i32) == 1i32)
        {
            ((&raw mut sMovingItemId).cast::<u8>().cast::<u16>()).write(GetMovingItemId());
        } else {
            ((&raw mut sMovingItemId).cast::<u8>().cast::<u16>()).write(0u16);
        }
        'l1: {
            let __sw1 = ((screenChangeType) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 || !__matched {
                FreePokeStorageData();
                SetMainCallback2(Some(CB2_ExitPokeStorage));
                break 'l1;
            }
            if __sw1 == 1i32 {
                boxMons = (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8588))
                .cast::<*mut u8>())
                .read();
                monIndex = ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8583))
                .read();
                maxMonIndex = ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8582))
                .read();
                mode = ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8584))
                .read();
                FreePokeStorageData();
                if (((mode) as i32) == 0i32)
                    && (((boxMons) as usize)
                        == (((&raw mut sSavedMovingMon).cast::<u8>()) as usize))
                {
                    ShowPokemonSummaryScreenHandleDeoxys(
                        mode,
                        boxMons,
                        monIndex,
                        maxMonIndex,
                        Some(CB2_ReturnToPokeStorage),
                    );
                } else {
                    ShowPokemonSummaryScreen(
                        mode,
                        boxMons,
                        monIndex,
                        maxMonIndex,
                        Some(CB2_ReturnToPokeStorage),
                    );
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                FreePokeStorageData();
                DoNamingScreen(
                    1u8,
                    GetBoxNamePtr(StorageGetCurrentBox()),
                    0u16,
                    0u16,
                    0u32,
                    Some(CB2_ReturnToPokeStorage),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                FreePokeStorageData();
                GoToBagMenu(11u8, 0u8, Some(CB2_ReturnToPokeStorage));
                break 'l1;
            }
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn GiveChosenBagItem() {
    unsafe {
        let mut itemId: u16 = ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read();
        if ((itemId) as i32) != 0i32 {
            let mut pos: u8 = GetCursorPosition();
            if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((pos) as i32) as isize * 100),
                    12i32,
                    (&raw mut itemId).cast::<u8>(),
                );
            } else {
                SetCurrentBoxMonData(pos, 12i32, (&raw mut itemId).cast::<u8>());
            }
            RemoveBagItem(itemId, 1u16);
        }
    }
}
pub(crate) unsafe extern "C" fn FreePokeStorageData() {
    unsafe {
        TilemapUtil_Free();
        MultiMove_Free();
        {
            Free(((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
        FreeAllWindowBuffers();
    }
}
pub(crate) unsafe extern "C" fn SetScrollingBackground() {
    unsafe {
        SetGpuReg(14u8, 7951u16);
        DecompressAndLoadBgGfxUsingHeap(
            3u8,
            (((&raw const sScrollingBg_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        LZ77UnCompVram(
            ((&raw const sScrollingBg_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100726784i32) as usize as *mut u8),
        );
    }
}
pub(crate) unsafe extern "C" fn ScrollBackground() {
    unsafe {
        ChangeBgX(3u8, 128i32, 1u8);
        ChangeBgY(3u8, 128i32, 2u8);
    }
}
pub(crate) unsafe extern "C" fn LoadPokeStorageMenuGfx() {
    unsafe {
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        DecompressAndLoadBgGfxUsingHeap(
            1u8,
            (((&raw mut gStorageSystemMenu_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        LZ77UnCompWram(
            ((&raw const sDisplayMenu_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(23236))
                .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(23236))
                .cast::<u8>(),
        );
        ShowBg(1u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn InitPokeStorageWindows() -> u8 {
    unsafe {
        if !((InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>()))
            != 0)
        {
            return 0u8;
        } else {
            DeactivateAllTextPrinters();
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn LoadWaveformSpritePalette() {
    unsafe {
        LoadSpritePalette((&raw const sWaveformSpritePalette).cast::<u8>().cast_mut());
    }
}
pub(crate) unsafe extern "C" fn InitPalettesAndSprites() {
    unsafe {
        LoadPalette(
            (((&raw const sInterface_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sPkmnDataGray_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            32u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sTextWindows_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            LoadPalette(
                (((&raw const sScrollingBg_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .cast::<u8>(),
                48u16,
                32u16,
            );
        } else {
            LoadPalette(
                (((&raw const sScrollingBgMoveItems_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .cast::<u8>(),
                48u16,
                32u16,
            );
        }
        SetGpuReg(10u8, 7685u16);
        CreateDisplayMonSprite();
        CreateMarkingComboSprite();
        CreateWaveformSprites();
        RefreshDisplayMonData();
    }
}
pub(crate) unsafe extern "C" fn CreateMarkingComboSprite() {
    unsafe {
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3476)
            .cast::<*mut u8>())
        .write(CreateMonMarkingComboSprite(
            16u16,
            56008u16,
            core::ptr::null_mut(),
        ));
        crate::c::bf_write(
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3476)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3476)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(67))
        .write(1u8);
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3476)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32)
        .cast::<i16>())
        .write(40i16);
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3476)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(34)
        .cast::<i16>())
        .write(150i16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3488)
            .cast::<*mut u16>())
        .write(
            (((100728832i32) as usize as *mut u8).wrapping_offset(
                ((32i32).wrapping_mul(((GetSpriteTileStartByTag(16u16)) as i32))) as isize * 1,
            ))
            .cast::<u16>(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateWaveformSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut sheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut sheet)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sSpriteSheet_Waveform)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        LoadSpriteSheet((&raw mut sheet).cast::<u8>());
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw const sSpriteTemplate_Waveform)
                            .cast::<u8>()
                            .cast_mut(),
                        (((((i) as i32).wrapping_mul(63i32)).wrapping_add(8i32)) as i16),
                        9i16,
                        2u8,
                    );
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3480))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RefreshDisplayMonData() {
    unsafe {
        LoadDisplayMonGfx(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3300)
                .cast::<u16>())
            .read(),
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3296)
                .cast::<u32>())
            .read(),
        );
        PrintDisplayMonInfo();
        UpdateWaveformAnimation();
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn StartDisplayMonMosaicEffect() {
    unsafe {
        RefreshDisplayMonData();
        if !(((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8768)
            .cast::<*mut u8>())
        .read())
        .is_null()
        {
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8768)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1),
                4,
                1,
                (1u32) as i32,
            );
            (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8768)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .write(10i16);
            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8768)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(1i16);
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8768)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_DisplayMonMosaic));
            SetGpuReg(
                76u8,
                ((((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8768)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .read()) as i32)
                    << 12)
                    | ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8768)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        << 8)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn IsDisplayMosaicActive() -> u8 {
    unsafe {
        return ((crate::c::bf_read(
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8768)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1),
            4,
            1,
            false,
        ) as u32) as u8);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DisplayMonMosaic(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 0i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        SetGpuReg(
            76u8,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) << 12)
                | ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) << 8))
                as u16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            crate::c::bf_write((sprite).wrapping_add(1), 4, 1, (0u32) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateDisplayMonSprite() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut tileStart: u16 = 0u16;
        let mut palSlot: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        let mut sheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut sheet)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8900))
                    .cast::<u8>(),
            );
        (&raw mut sheet)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(((crate::c::div_i32(4096i32, 2i32)) as u16));
        (&raw mut sheet)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(2u16);
        let mut palette = crate::ffi::Align4([0u8; 8]);
        (&raw mut palette)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8772))
                    .cast::<u16>(),
            );
        (&raw mut palette)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(56006u16);
        let mut template = crate::ffi::Align4([0u8; 24]);
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_DisplayMon)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < crate::c::div_i32(4096i32, 2i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8900))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8772))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8768)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        'l5: loop {
            'l6: {
                tileStart = LoadSpriteSheet((&raw mut sheet).cast::<u8>());
                if ((tileStart) as i32) == 0i32 {
                    break 'l5;
                }
                palSlot = LoadSpritePalette((&raw mut palette).cast::<u8>());
                if ((palSlot) as i32) == 255i32 {
                    break 'l5;
                }
                spriteId = CreateSprite((&raw mut template).cast::<u8>(), 40i16, 48i16, 0u8);
                if ((spriteId) as i32) == 64i32 {
                    break 'l5;
                }
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8768)
                    .cast::<*mut u8>())
                .write(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                );
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8762)
                    .cast::<u16>())
                .write((((256i32).wrapping_add(((palSlot) as i32).wrapping_mul(16i32))) as u16));
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8764)
                    .cast::<*mut u16>())
                .write(
                    (((100728832i32) as usize as *mut u8).wrapping_offset(
                        (((tileStart) as i32).wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                            as isize
                            * 1,
                    ))
                    .cast::<u16>(),
                );
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8768)
            .cast::<*mut u8>())
        .read()) as usize)
            == 0usize
        {
            FreeSpriteTilesByTag(2u16);
            FreeSpritePaletteByTag(56006u16);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadDisplayMonGfx(species: u16, pid: u32) {
    unsafe {
        let mut species = species;
        let mut pid = pid;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8768)
            .cast::<*mut u8>())
        .read()) as usize)
            == 0usize
        {
            return;
        }
        if ((species) as i32) != 0i32 {
            LoadSpecialPokePic(
                ((&raw mut gMonFrontPicTable).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8),
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8900))
                    .cast::<u8>(),
                ((species) as i32),
                pid,
                1u8,
            );
            LZ77UnCompWram(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3292)
                    .cast::<*mut u32>())
                .read(),
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8772))
                .cast::<u16>())
                .cast::<u8>(),
            );
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8900))
                                .cast::<u8>(),
                                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8764)
                                    .cast::<*mut u16>())
                                .read())
                                .cast::<u8>(),
                                ((67108864i32
                                    | (crate::c::div_i32(
                                        crate::c::div_i32(4096i32, 2i32),
                                        crate::c::div_i32(32i32, 8i32),
                                    ) & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
            LoadPalette(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8772))
                .cast::<u16>())
                .cast::<u8>(),
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8762)
                    .cast::<u16>())
                .read(),
                32u16,
            );
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8768)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8768)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintDisplayMonInfo() {
    unsafe {
        FillWindowPixelBuffer(0u8, 17u8);
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            AddTextPrinterParameterized(
                0u8,
                1u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3321))
                    .cast::<u8>(),
                6u8,
                0u8,
                255u8,
                None,
            );
            AddTextPrinterParameterized(
                0u8,
                2u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3357))
                    .cast::<u8>(),
                6u8,
                15u8,
                255u8,
                None,
            );
            AddTextPrinterParameterized(
                0u8,
                2u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3393))
                    .cast::<u8>(),
                10u8,
                29u8,
                255u8,
                None,
            );
            AddTextPrinterParameterized(
                0u8,
                0u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3429))
                    .cast::<u8>(),
                6u8,
                43u8,
                255u8,
                None,
            );
        } else {
            AddTextPrinterParameterized(
                0u8,
                0u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3429))
                    .cast::<u8>(),
                6u8,
                0u8,
                255u8,
                None,
            );
            AddTextPrinterParameterized(
                0u8,
                1u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3321))
                    .cast::<u8>(),
                6u8,
                13u8,
                255u8,
                None,
            );
            AddTextPrinterParameterized(
                0u8,
                2u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3357))
                    .cast::<u8>(),
                6u8,
                28u8,
                255u8,
                None,
            );
            AddTextPrinterParameterized(
                0u8,
                2u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3393))
                    .cast::<u8>(),
                10u8,
                42u8,
                255u8,
                None,
            );
        }
        CopyWindowToVram(0u8, 2u8);
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3300)
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            UpdateMonMarkingTiles(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3307))
                    .read(),
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3488)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3476)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3476)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateWaveformAnimation() {
    unsafe {
        let mut i: u16 = 0u16;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3300)
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            TilemapUtil_SetRect(0u8, 0u16, 0u16, 8u16, 2u16);
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                        break 'l1;
                    }
                    'l2: {
                        StartSpriteAnimIfDifferent(
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                            (((((i) as i32).wrapping_mul(2i32)).wrapping_add(1i32)) as u8),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            TilemapUtil_SetRect(0u8, 0u16, 2u16, 8u16, 2u16);
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                        break 'l3;
                    }
                    'l4: {
                        StartSpriteAnim(
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3480))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                            ((((i) as i32).wrapping_mul(2i32)) as u8),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        TilemapUtil_Update(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn InitSupplementalTilemaps() {
    unsafe {
        LZ77UnCompWram(
            ((&raw mut gStorageSystemPartyMenu_Tilemap).cast::<u32>()).cast::<u32>(),
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(176))
                .cast::<u16>())
            .cast::<u8>(),
        );
        LoadPalette(
            (((&raw mut gStorageSystemPartyMenu_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            16u16,
            32u16,
        );
        TilemapUtil_SetMap(
            1u8,
            1u8,
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(176))
                .cast::<u16>())
            .cast::<u8>(),
            12u16,
            22u16,
        );
        TilemapUtil_SetMap(
            2u8,
            1u8,
            (((&raw const sCloseBoxButton_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            9u16,
            4u16,
        );
        TilemapUtil_SetPos(1u8, 10u16, 0u16);
        TilemapUtil_SetPos(2u8, 21u16, 0u16);
        SetPartySlotTilemaps();
        if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
            UpdateCloseBoxButtonTilemap(1u8);
            CreatePartyMonsSprites(1u8);
            TilemapUtil_Update(2u8);
            TilemapUtil_Update(1u8);
        } else {
            TilemapUtil_SetRect(1u8, 0u16, 20u16, 12u16, 2u16);
            UpdateCloseBoxButtonTilemap(1u8);
            TilemapUtil_Update(1u8);
            TilemapUtil_Update(2u8);
        }
        ScheduleBgCopyTilemapToVram(1u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(711))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SetUpShowPartyMenu() {
    unsafe {
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(704)
            .cast::<u16>())
        .write(20u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(706)
            .cast::<u16>())
        .write(2u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(709))
            .write(0u8);
        CreatePartyMonsSprites(0u8);
    }
}
pub(crate) unsafe extern "C" fn ShowPartyMenu() -> u8 {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(709))
            .read()) as i32)
            == 20i32
        {
            return 0u8;
        }
        let __p1 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(704)
            .cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(706)
            .cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        TilemapUtil_Move(1u8, 3u8, 1i8);
        TilemapUtil_Update(1u8);
        ScheduleBgCopyTilemapToVram(1u8);
        MovePartySprites(8i16);
        if (({
            let __p3 =
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(709);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 20i32
        {
            ((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).write(1u8);
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SetUpHidePartyMenu() {
    unsafe {
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(704)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(706)
            .cast::<u16>())
        .write(22u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(709))
            .write(0u8);
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            == 3i32
        {
            MoveHeldItemWithPartyMenu();
        }
    }
}
pub(crate) unsafe extern "C" fn HidePartyMenu() -> u8 {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(709))
            .read()) as i32)
            != 20i32
        {
            let __p1 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(704)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(706)
                .cast::<u16>();
            (__p2).write(((__p2).read()).wrapping_sub(1));
            TilemapUtil_Move(1u8, 3u8, (-1i8));
            TilemapUtil_Update(1u8);
            FillBgTilemapBufferRect_Palette0(
                1u8,
                256u16,
                10u8,
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(706)
                    .cast::<u16>())
                .read()) as u8),
                12u8,
                1u8,
            );
            MovePartySprites((-8i16));
            if (({
                let __p3 =
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(709);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                != 20i32
            {
                ScheduleBgCopyTilemapToVram(1u8);
                return 1u8;
            } else {
                ((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).write(0u8);
                DestroyAllPartyMonIcons();
                CompactPartySlots();
                TilemapUtil_SetRect(2u8, 0u16, 0u16, 9u16, 2u16);
                TilemapUtil_Update(2u8);
                ScheduleBgCopyTilemapToVram(1u8);
                return 0u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateCloseBoxButtonTilemap(normal: u8) {
    unsafe {
        let mut normal = normal;
        if (normal) != 0 {
            TilemapUtil_SetRect(2u8, 0u16, 0u16, 9u16, 2u16);
        } else {
            TilemapUtil_SetRect(2u8, 0u16, 2u16, 9u16, 2u16);
        }
        TilemapUtil_Update(2u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn StartFlashingCloseBoxButton() {
    unsafe {
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(711))
            .write(1u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(712))
            .write(30u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(713))
            .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn StopFlashingCloseBoxButton() {
    unsafe {
        if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(711))
            .read())
            != 0
        {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(711))
                .write(0u8);
            UpdateCloseBoxButtonTilemap(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateCloseBoxButtonFlash() {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(711))
            .read())
            != 0)
            && ((({
                let __p1 =
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(712);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 30i32)
        {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(712))
                .write(0u8);
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(713))
                .write(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(713))
                    .read()) as i32)
                        == 0i32) as u8),
                );
            UpdateCloseBoxButtonTilemap(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(713))
                    .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetPartySlotTilemaps() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut species: i32 = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        11i32,
                    )) as i32);
                    SetPartySlotTilemap(i, ((species != 0i32) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPartySlotTilemap(partyId: u8, hasMon: u8) {
    unsafe {
        let mut partyId = partyId;
        let mut hasMon = hasMon;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut index: u16 = 0u16;
        let mut data: *mut u16 = core::ptr::null_mut();
        if (hasMon) != 0 {
            data = ((&raw const sPartySlotFilled_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>();
        } else {
            data = ((&raw const sPartySlotEmpty_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>();
        }
        index = (((3i32).wrapping_mul(
            ((3i32).wrapping_mul(((partyId) as i32).wrapping_sub(1i32))).wrapping_add(1i32),
        )) as u16);
        index = ((((index) as i32).wrapping_mul(4i32)) as u16);
        index = ((((index) as i32).wrapping_add(7i32)) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .cast::<u16>())
                                .wrapping_offset(
                                    (((index) as i32).wrapping_add(((j) as i32))) as isize,
                                ))
                                .write(((data).wrapping_offset(((j) as i32) as isize)).read());
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    data = (data).wrapping_offset(4);
                    index = ((((index) as i32).wrapping_add(12i32)) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePartySlotColors() {
    unsafe {
        SetPartySlotTilemaps();
        TilemapUtil_SetRect(1u8, 0u16, 0u16, 12u16, 22u16);
        TilemapUtil_Update(1u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn SetUpDoShowPartyMenu() {
    unsafe {
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(710))
            .write(0u8);
        PlaySE(6u16);
        SetUpShowPartyMenu();
    }
}
pub(crate) unsafe extern "C" fn DoShowPartyMenu() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(710))
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((ShowPartyMenu()) != 0) {
                    SetCursorInParty();
                    let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(710);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdateCursorPos()) != 0) {
                    if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3306))
                    .read())
                        != 0
                    {
                        StartDisplayMonMosaicEffect();
                    }
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(710);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateBoxToSendMons() {
    unsafe {
        if ((((&raw mut sLastUsedBox).cast::<u8>().cast::<u8>()).read()) as i32)
            != ((StorageGetCurrentBox()) as i32)
        {
            FlagClear(2263u16);
            VarSet(16438u16, ((StorageGetCurrentBox()) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn InitPokeStorageBg0() {
    unsafe {
        SetGpuReg(8u8, 7424u16);
        LoadUserWindowBorderGfx(1u8, 2u16, 208u8);
        FillBgTilemapBufferRect(0u8, 0u16, 0u8, 0u8, 32u8, 20u8, 17u8);
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintMessage(id: u8) {
    unsafe {
        let mut id = id;
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        DynamicPlaceholderTextUtil_Reset();
        'l1: {
            let __sw1 = (((((((&raw const sMessages).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 8))
            .wrapping_add(4))
            .read()) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                    0u8,
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3310))
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 5i32 || __sw1 == 6i32 {
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                    0u8,
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8672))
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (IsMovingItem()) != 0 {
                    txtPtr = StringCopy(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8683))
                        .cast::<u8>(),
                        GetMovingItemName(),
                    );
                } else {
                    txtPtr = StringCopy(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8683))
                        .cast::<u8>(),
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3429))
                        .cast::<u8>(),
                    );
                }
                'l2: loop {
                    if !(((((txtPtr).wrapping_offset(-1)).read()) as i32) == 0i32) {
                        break 'l2;
                    }
                    txtPtr = (txtPtr).wrapping_offset(-1);
                }
                (txtPtr).write(255u8);
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                    0u8,
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8683))
                    .cast::<u8>(),
                );
                break 'l1;
            }
        }
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8592))
                .cast::<u8>(),
            (((((&raw const sMessages).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 8))
            .cast::<*mut u8>())
            .read(),
        );
        FillWindowPixelBuffer(1u8, 17u8);
        AddTextPrinterParameterized(
            1u8,
            1u8,
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8592))
                .cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        DrawTextBorderOuter(1u8, 2u16, 14u8);
        PutWindowTilemap(1u8);
        CopyWindowToVram(1u8, 2u8);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn ShowYesNoWindow(cursorPos: i8) {
    unsafe {
        let mut cursorPos = cursorPos;
        CreateYesNoMenu(
            (&raw const sYesNoWindowTemplate).cast::<u8>().cast_mut(),
            11u16,
            14u8,
            0u8,
        );
        Menu_MoveCursorNoWrapAround(cursorPos);
    }
}
pub(crate) unsafe extern "C" fn ClearBottomWindow() {
    unsafe {
        ClearStdWindowAndFrameToTransparent(1u8, 0u8);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn AddWallpaperSetsMenu() {
    unsafe {
        InitMenu();
        SetMenuText(18u8);
        SetMenuText(19u8);
        SetMenuText(20u8);
        SetMenuText(21u8);
        if (IsWaldaWallpaperUnlocked()) != 0 {
            SetMenuText(22u8);
        }
        AddMenu();
    }
}
pub(crate) unsafe extern "C" fn AddWallpapersMenu(wallpaperSet: u8) {
    unsafe {
        let mut wallpaperSet = wallpaperSet;
        InitMenu();
        'l1: {
            let __sw1 = ((wallpaperSet) as i32);
            if __sw1 == 0i32 {
                SetMenuText(23u8);
                SetMenuText(24u8);
                SetMenuText(25u8);
                SetMenuText(26u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetMenuText(27u8);
                SetMenuText(28u8);
                SetMenuText(29u8);
                SetMenuText(30u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetMenuText(31u8);
                SetMenuText(32u8);
                SetMenuText(33u8);
                SetMenuText(34u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetMenuText(35u8);
                SetMenuText(36u8);
                SetMenuText(37u8);
                SetMenuText(38u8);
                break 'l1;
            }
        }
        AddMenu();
    }
}
pub(crate) unsafe extern "C" fn GetCurrentBoxOption() -> u8 {
    unsafe {
        return ((&raw mut sCurrentBoxOption).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn InitCursorItemIcon() {
    unsafe {
        if !((IsCursorOnBoxTitle()) != 0) {
            if (((&raw mut sInPartyMenu).cast::<u8>().cast::<u8>()).read()) != 0 {
                TryLoadItemIconAtPos(1u8, GetCursorPosition());
            } else {
                TryLoadItemIconAtPos(0u8, GetCursorPosition());
            }
        }
        if ((((&raw mut sMovingItemId).cast::<u8>().cast::<u16>()).read()) as i32) != 0i32 {
            InitItemIconInCursor(((&raw mut sMovingItemId).cast::<u8>().cast::<u16>()).read());
            StartCursorAnim(3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn InitMonIconFields() {
    unsafe {
        let mut i: u16 = 0u16;
        LoadMonIconPalettes();
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < (if 37i32 >= 40i32 { 37i32 } else { 40i32 })) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2824))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < (if 37i32 >= 40i32 { 37i32 } else { 40i32 })) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2904))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l5: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l5;
                }
                'l6: {
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2672))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l7: loop {
                if !(((i) as i32) < 30i32) {
                    break 'l7;
                }
                'l8: {
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2696))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1932)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn GetMonIconPriorityByCursorPos() -> u8 {
    unsafe {
        return ((if (IsCursorInBox()) != 0 { 2i32 } else { 1i32 }) as u8);
    }
}
pub(crate) unsafe extern "C" fn CreateMovingMonIcon() {
    unsafe {
        let mut personality: u32 = GetMonData2(
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8356),
            0i32,
        );
        let mut species: u16 = ((GetMonData2(
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8356),
            65i32,
        )) as u16);
        let mut priority: u8 = GetMonIconPriorityByCursorPos();
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .write(CreateMonIconSprite(
            species,
            personality,
            0i16,
            0i16,
            priority,
            7u8,
        ));
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_HeldMon));
    }
}
pub(crate) unsafe extern "C" fn InitBoxMonSprites(boxId: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition: u8 = 0u8;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut count: u16 = 0u16;
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        count = 0u16;
        boxPosition = 0u8;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as i32) < 6i32) {
                                break 'l3;
                            }
                            'l4: {
                                species = ((GetBoxMonDataAt(boxId, boxPosition, 65i32)) as u16);
                                if ((species) as i32) != 0i32 {
                                    personality = GetBoxMonDataAt(boxId, boxPosition, 0i32);
                                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(2696))
                                    .cast::<*mut u8>())
                                    .wrapping_offset(((count) as i32) as isize))
                                    .write(CreateMonIconSprite(
                                        species,
                                        personality,
                                        ((((8i32).wrapping_mul((3i32).wrapping_mul(((j) as i32))))
                                            .wrapping_add(100i32))
                                            as i16),
                                        ((((8i32).wrapping_mul((3i32).wrapping_mul(((i) as i32))))
                                            .wrapping_add(44i32))
                                            as i16),
                                        2u8,
                                        (((19i32).wrapping_sub(((j) as i32))) as u8),
                                    ));
                                } else {
                                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(2696))
                                    .cast::<*mut u8>())
                                    .wrapping_offset(((count) as i32) as isize))
                                    .write(core::ptr::null_mut());
                                }
                                boxPosition = (boxPosition).wrapping_add(1);
                                count = (count).wrapping_add(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            == 3i32
        {
            {
                boxPosition = 0u8;
                'l5: loop {
                    if !(((boxPosition) as i32) < 30i32) {
                        break 'l5;
                    }
                    'l6: {
                        if GetBoxMonDataAt(boxId, boxPosition, 12i32) == 0u32 {
                            crate::c::bf_write(
                                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2696))
                                .cast::<*mut u8>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read())
                                .wrapping_add(1),
                                2,
                                2,
                                (1u32) as i32,
                            );
                        }
                    }
                    boxPosition = (boxPosition).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateBoxMonIconAtPos(boxPosition: u8) {
    unsafe {
        let mut boxPosition = boxPosition;
        let mut species: u16 = ((GetCurrentBoxMonData(boxPosition, 65i32)) as u16);
        if ((species) as i32) != 0i32 {
            let mut x: i16 = ((((8i32).wrapping_mul(
                (3i32).wrapping_mul(crate::c::rem_i32(((boxPosition) as i32), 6i32)),
            ))
            .wrapping_add(100i32)) as i16);
            let mut y: i16 = ((((8i32).wrapping_mul(
                (3i32).wrapping_mul(crate::c::div_i32(((boxPosition) as i32), 6i32)),
            ))
            .wrapping_add(44i32)) as i16);
            let mut personality: u32 = GetCurrentBoxMonData(boxPosition, 0i32);
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2696))
                .cast::<*mut u8>())
            .wrapping_offset(((boxPosition) as i32) as isize))
            .write(CreateMonIconSprite(
                species,
                personality,
                x,
                y,
                2u8,
                (((19i32).wrapping_sub(crate::c::rem_i32(((boxPosition) as i32), 6i32))) as u8),
            ));
            if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .read()) as i32)
                == 3i32
            {
                crate::c::bf_write(
                    (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2696))
                    .cast::<*mut u8>())
                    .wrapping_offset(((boxPosition) as i32) as isize))
                    .read())
                    .wrapping_add(1),
                    2,
                    2,
                    (1u32) as i32,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartBoxMonIconsScrollOut(speed: i16) {
    unsafe {
        let mut speed = speed;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 30i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2696))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as usize)
                        != 0usize
                    {
                        ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2696))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(speed);
                        ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2696))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(1i16);
                        ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2696))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_BoxMonIconScrollOut));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BoxMonIconScrollIn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            != 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
        } else {
            let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3174)
                .cast::<u16>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read());
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BoxMonIconScrollOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            != 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                <= 68i32)
                || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >= 252i32)
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyBoxMonIconsInColumn(column: u8) {
    unsafe {
        let mut column = column;
        let mut row: u16 = 0u16;
        let mut boxPosition: u8 = column;
        {
            row = 0u16;
            'l1: loop {
                if !(((row) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2696))
                    .cast::<*mut u8>())
                    .wrapping_offset(((boxPosition) as i32) as isize))
                    .read()) as usize)
                        != 0usize
                    {
                        DestroyBoxMonIcon(
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2696))
                            .cast::<*mut u8>())
                            .wrapping_offset(((boxPosition) as i32) as isize))
                            .read(),
                        );
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2696))
                        .cast::<*mut u8>())
                        .wrapping_offset(((boxPosition) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                    boxPosition = ((((boxPosition) as i32).wrapping_add(6i32)) as u8);
                }
                row = (row).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateBoxMonIconsInColumn(
    column: u8,
    distance: u16,
    speed: i16,
) -> u8 {
    unsafe {
        let mut column = column;
        let mut distance = distance;
        let mut speed = speed;
        let mut i: i32 = 0i32;
        let mut y: u16 = 44u16;
        let mut xDest: i16 = ((((8i32).wrapping_mul((3i32).wrapping_mul(((column) as i32))))
            .wrapping_add(100i32)) as i16);
        let mut x: u16 = ((((xDest) as i32)
            .wrapping_sub((((distance) as i32).wrapping_add(1i32)).wrapping_mul(((speed) as i32))))
            as u16);
        let mut subpriority: u8 = (((19i32).wrapping_sub(((column) as i32))) as u8);
        let mut iconsCreated: u8 = 0u8;
        let mut boxPosition: u8 = column;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 5i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2984))
                        .cast::<u16>())
                        .wrapping_offset(((boxPosition) as i32) as isize))
                        .read()) as i32)
                            != 0i32
                        {
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2696))
                            .cast::<*mut u8>())
                            .wrapping_offset(((boxPosition) as i32) as isize))
                            .write(CreateMonIconSprite(
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2984))
                                .cast::<u16>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read(),
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(3044))
                                .cast::<u32>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read(),
                                ((x) as i16),
                                ((y) as i16),
                                2u8,
                                subpriority,
                            ));
                            if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2696))
                            .cast::<*mut u8>())
                            .wrapping_offset(((boxPosition) as i32) as isize))
                            .read()) as usize)
                                != 0usize
                            {
                                ((((((((((&raw mut sStorage)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2696))
                                .cast::<*mut u8>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(((distance) as i16));
                                ((((((((((&raw mut sStorage)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2696))
                                .cast::<*mut u8>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .write(speed);
                                ((((((((((&raw mut sStorage)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2696))
                                .cast::<*mut u8>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(3))
                                .write(xDest);
                                ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2696))
                                .cast::<*mut u8>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read())
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                .write(Some(SpriteCB_BoxMonIconScrollIn));
                                iconsCreated = (iconsCreated).wrapping_add(1);
                            }
                        }
                        boxPosition = ((((boxPosition) as i32).wrapping_add(6i32)) as u8);
                        y = ((((y) as i32).wrapping_add(24i32)) as u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 5i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2984))
                        .cast::<u16>())
                        .wrapping_offset(((boxPosition) as i32) as isize))
                        .read()) as i32)
                            != 0i32
                        {
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2696))
                            .cast::<*mut u8>())
                            .wrapping_offset(((boxPosition) as i32) as isize))
                            .write(CreateMonIconSprite(
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2984))
                                .cast::<u16>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read(),
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(3044))
                                .cast::<u32>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read(),
                                ((x) as i16),
                                ((y) as i16),
                                2u8,
                                subpriority,
                            ));
                            if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2696))
                            .cast::<*mut u8>())
                            .wrapping_offset(((boxPosition) as i32) as isize))
                            .read()) as usize)
                                != 0usize
                            {
                                ((((((((((&raw mut sStorage)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2696))
                                .cast::<*mut u8>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(((distance) as i16));
                                ((((((((((&raw mut sStorage)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2696))
                                .cast::<*mut u8>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .write(speed);
                                ((((((((((&raw mut sStorage)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2696))
                                .cast::<*mut u8>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read())
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(3))
                                .write(xDest);
                                ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2696))
                                .cast::<*mut u8>())
                                .wrapping_offset(((boxPosition) as i32) as isize))
                                .read())
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                .write(Some(SpriteCB_BoxMonIconScrollIn));
                                if GetBoxMonDataAt(
                                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(3164))
                                    .read(),
                                    boxPosition,
                                    12i32,
                                ) == 0u32
                                {
                                    crate::c::bf_write(
                                        (((((((&raw mut sStorage)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(2696))
                                        .cast::<*mut u8>())
                                        .wrapping_offset(((boxPosition) as i32) as isize))
                                        .read())
                                        .wrapping_add(1),
                                        2,
                                        2,
                                        (1u32) as i32,
                                    );
                                }
                                iconsCreated = (iconsCreated).wrapping_add(1);
                            }
                        }
                        boxPosition = ((((boxPosition) as i32).wrapping_add(6i32)) as u8);
                        y = ((((y) as i32).wrapping_add(24i32)) as u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return iconsCreated;
    }
}
pub(crate) unsafe extern "C" fn InitBoxMonIconScroll(boxId: u8, direction: i8) {
    unsafe {
        let mut boxId = boxId;
        let mut direction = direction;
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3178))
            .write(0u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3179))
            .write(boxId);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3177)
            .cast::<i8>())
        .write(direction);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3168)
            .cast::<u16>())
        .write(32u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3172)
            .cast::<i16>())
        .write(((((6i32).wrapping_mul(((direction) as i32))).wrapping_neg()) as i16));
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3174)
            .cast::<u16>())
        .write(0u16);
        GetIncomingBoxMonData(boxId);
        if ((direction) as i32) > 0i32 {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3176))
                .write(0u8);
        } else {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3176))
                .write(5u8);
        }
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3170)
            .cast::<i16>())
        .write(
            ((((24i32).wrapping_mul(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3176))
                .read()) as i32),
            ))
            .wrapping_add(100i32)) as i16),
        );
        StartBoxMonIconsScrollOut(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3172)
                .cast::<i16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateBoxMonIconScroll() -> u8 {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3168)
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            let __p1 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3168)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        'l1: {
            let __sw2 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3178))
            .read()) as i32);
            let __matched = __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32;
            if __sw2 == 0i32 {
                let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3170)
                    .cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3172)
                            .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
                if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3170)
                    .cast::<i16>())
                .read()) as i32)
                    <= 64i32)
                    || (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3170)
                        .cast::<i16>())
                    .read()) as i32)
                        >= 252i32)
                {
                    DestroyBoxMonIconsInColumn(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3176))
                        .read(),
                    );
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3170)
                        .cast::<i16>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_add(
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3177)
                                .cast::<i8>())
                            .read()) as i32)
                                .wrapping_mul(24i32),
                        )) as i16),
                    );
                    let __p5 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3178);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw2 == 1i32 {
                let __p6 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3170)
                    .cast::<i16>();
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3172)
                            .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
                let __p7 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3174)
                    .cast::<u16>();
                (__p7).write(
                    (((((__p7).read()) as i32).wrapping_add(
                        ((CreateBoxMonIconsInColumn(
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3176))
                            .read(),
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3168)
                                .cast::<u16>())
                            .read(),
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3172)
                                .cast::<i16>())
                            .read(),
                        )) as i32),
                    )) as u16),
                );
                if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3177)
                    .cast::<i8>())
                .read()) as i32)
                    > 0i32)
                    && (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3176))
                    .read()) as i32)
                        == 5i32))
                    || ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3177)
                        .cast::<i8>())
                    .read()) as i32)
                        < 0i32)
                        && (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3176))
                        .read()) as i32)
                            == 0i32))
                {
                    let __p8 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3178);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                } else {
                    let __p9 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3176);
                    (__p9).write(
                        (((((__p9).read()) as i32).wrapping_add(
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3177)
                                .cast::<i8>())
                            .read()) as i32),
                        )) as u8),
                    );
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3178))
                    .write(0u8);
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3174)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    let __p10 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3168)
                        .cast::<u16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GetIncomingBoxMonData(boxId: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut boxPosition: i32 = 0i32;
        boxPosition = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 6i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2984))
                                .cast::<u16>())
                                .wrapping_offset((boxPosition) as isize))
                                .write(
                                    ((GetBoxMonDataAt(boxId, ((boxPosition) as u8), 65i32)) as u16),
                                );
                                if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2984))
                                .cast::<u16>())
                                .wrapping_offset((boxPosition) as isize))
                                .read()) as i32)
                                    != 0i32
                                {
                                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(3044))
                                    .cast::<u32>())
                                    .wrapping_offset((boxPosition) as isize))
                                    .write(GetBoxMonDataAt(boxId, ((boxPosition) as u8), 0i32));
                                }
                                boxPosition = (boxPosition).wrapping_add(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3164))
            .write(boxId);
    }
}
pub(crate) unsafe extern "C" fn DestroyBoxMonIconAtPosition(boxPosition: u8) {
    unsafe {
        let mut boxPosition = boxPosition;
        if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2696))
            .cast::<*mut u8>())
        .wrapping_offset(((boxPosition) as i32) as isize))
        .read()) as usize)
            != 0usize
        {
            DestroyBoxMonIcon(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2696))
                .cast::<*mut u8>())
                .wrapping_offset(((boxPosition) as i32) as isize))
                .read(),
            );
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2696))
                .cast::<*mut u8>())
            .wrapping_offset(((boxPosition) as i32) as isize))
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn SetBoxMonIconObjMode(boxPosition: u8, objMode: u8) {
    unsafe {
        let mut boxPosition = boxPosition;
        let mut objMode = objMode;
        if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2696))
            .cast::<*mut u8>())
        .wrapping_offset(((boxPosition) as i32) as isize))
        .read()) as usize)
            != 0usize
        {
            crate::c::bf_write(
                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2696))
                .cast::<*mut u8>())
                .wrapping_offset(((boxPosition) as i32) as isize))
                .read())
                .wrapping_add(1),
                2,
                2,
                ((objMode) as u32) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonsSprites(visible: u8) {
    unsafe {
        let mut visible = visible;
        let mut i: u16 = 0u16;
        let mut count: u16 = 0u16;
        let mut species: u16 = ((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 65i32)) as u16);
        let mut personality: u32 = GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 0i32);
        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2672))
            .cast::<*mut u8>())
        .write(CreateMonIconSprite(
            species,
            personality,
            104i16,
            64i16,
            1u8,
            12u8,
        ));
        count = 1u16;
        {
            i = 1u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    species = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        65i32,
                    )) as u16);
                    if ((species) as i32) != 0i32 {
                        personality = GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            0i32,
                        );
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2672))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(CreateMonIconSprite(
                            species,
                            personality,
                            152i16,
                            ((((8i32).wrapping_mul(
                                (3i32).wrapping_mul(((i) as i32).wrapping_sub(1i32)),
                            ))
                            .wrapping_add(16i32)) as i16),
                            1u8,
                            12u8,
                        ));
                        count = (count).wrapping_add(1);
                    } else {
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2672))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((visible) != 0) {
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < ((count) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        let __p1 = (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(2672))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(34)
                        .cast::<i16>();
                        (__p1).write((((((__p1).read()) as i32).wrapping_sub(160i32)) as i16));
                        crate::c::bf_write(
                            (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2672))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            == 3i32
        {
            {
                i = 0u16;
                'l5: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l5;
                    }
                    'l6: {
                        if (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2672))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as usize)
                            != 0usize)
                            && (GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                12i32,
                            ) == 0u32)
                        {
                            crate::c::bf_write(
                                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2672))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(1),
                                2,
                                2,
                                (1u32) as i32,
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CompactPartySprites() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut targetSlot: u16 = 0u16;
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3166))
            .write(0u8);
        {
            i = 0u16;
            targetSlot = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2672))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as usize)
                        != 0usize
                    {
                        if ((i) as i32) != ((targetSlot) as i32) {
                            MovePartySpriteToNextSlot(
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2672))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                targetSlot,
                            );
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2672))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(core::ptr::null_mut());
                            let __p1 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(3166);
                            (__p1).write(((__p1).read()).wrapping_add(1));
                        }
                        targetSlot = (targetSlot).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetNumPartySpritesCompacting() -> u8 {
    unsafe {
        return ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3166))
            .read();
    }
}
pub(crate) unsafe extern "C" fn MovePartySpriteToNextSlot(sprite: *mut u8, partyId: u16) {
    unsafe {
        let mut sprite = sprite;
        let mut partyId = partyId;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(((partyId) as i16));
        if ((partyId) as i32) == 0i32 {
            x = 104i16;
            y = 64i16;
        } else {
            x = 152i16;
            y = ((((8i32).wrapping_mul((3i32).wrapping_mul(((partyId) as i32).wrapping_sub(1i32))))
                .wrapping_add(16i32)) as i16);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as u16) as i32)
                .wrapping_mul(8i32)) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as u16) as i32)
                .wrapping_mul(8i32)) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((crate::c::div_i32(
                (((x) as i32).wrapping_mul(8i32)).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                ),
                8i32,
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((crate::c::div_i32(
                (((y) as i32).wrapping_mul(8i32)).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                ),
                8i32,
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(8i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MovePartyMonToNextSlot));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MovePartyMonToNextSlot(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            != 0i32
        {
            let mut x: i16 = {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __v2 = (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16);
                (__p1).write(__v2);
                __v2
            };
            let mut y: i16 = {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                let __v4 = (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32),
                )) as i16);
                (__p3).write(__v4);
                __v4
            };
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((crate::c::div_u32(((x) as u32), 8u32)) as i16));
            ((sprite).wrapping_add(34).cast::<i16>())
                .write(((crate::c::div_u32(((y) as u32), 8u32)) as i16));
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p5).write(((__p5).read()).wrapping_sub(1));
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 0i32
            {
                ((sprite).wrapping_add(32).cast::<i16>()).write(104i16);
                ((sprite).wrapping_add(34).cast::<i16>()).write(64i16);
            } else {
                ((sprite).wrapping_add(32).cast::<i16>()).write(152i16);
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((8i32).wrapping_mul(
                        (3i32).wrapping_mul(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32)
                                .wrapping_sub(1i32),
                        ),
                    ))
                    .wrapping_add(16i32)) as i16),
                );
            }
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2672))
                .cast::<*mut u8>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            ))
            .write(sprite);
            let __p6 =
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3166);
            (__p6).write(((__p6).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyMovingMonIcon() {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            DestroyBoxMonIcon(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2668)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2668)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn MovePartySprites(yDelta: i16) {
    unsafe {
        let mut yDelta = yDelta;
        let mut i: u16 = 0u16;
        let mut posY: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2672))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as usize)
                        != 0usize
                    {
                        let __p1 = (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(2672))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(34)
                        .cast::<i16>();
                        (__p1).write(
                            (((((__p1).read()) as i32).wrapping_add(((yDelta) as i32))) as i16),
                        );
                        posY = (((((((((((((&raw mut sStorage)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2672))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(34)
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(
                                ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2672))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(38)
                                .cast::<i16>())
                                .read()) as i32),
                            ))
                        .wrapping_add(
                            ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2672))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(41)
                            .cast::<i8>())
                            .read()) as i32),
                        )) as u16);
                        posY = ((((posY) as i32).wrapping_add(16i32)) as u16);
                        if ((posY) as i32) > 192i32 {
                            crate::c::bf_write(
                                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2672))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(62),
                                2,
                                1,
                                (1u16) as i32,
                            );
                        } else {
                            crate::c::bf_write(
                                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2672))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(62),
                                2,
                                1,
                                (0u16) as i32,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyPartyMonIcon(partyId: u8) {
    unsafe {
        let mut partyId = partyId;
        if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2672))
            .cast::<*mut u8>())
        .wrapping_offset(((partyId) as i32) as isize))
        .read()) as usize)
            != 0usize
        {
            DestroyBoxMonIcon(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2672))
                .cast::<*mut u8>())
                .wrapping_offset(((partyId) as i32) as isize))
                .read(),
            );
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2672))
                .cast::<*mut u8>())
            .wrapping_offset(((partyId) as i32) as isize))
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyAllPartyMonIcons() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2672))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as usize)
                        != 0usize
                    {
                        DestroyBoxMonIcon(
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2672))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2672))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPartyMonIconObjMode(partyId: u8, objMode: u8) {
    unsafe {
        let mut partyId = partyId;
        let mut objMode = objMode;
        if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2672))
            .cast::<*mut u8>())
        .wrapping_offset(((partyId) as i32) as isize))
        .read()) as usize)
            != 0usize
        {
            crate::c::bf_write(
                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2672))
                .cast::<*mut u8>())
                .wrapping_offset(((partyId) as i32) as isize))
                .read())
                .wrapping_add(1),
                2,
                2,
                ((objMode) as u32) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetMovingMonSprite(mode: u8, id: u8) {
    unsafe {
        let mut mode = mode;
        let mut id = id;
        if ((mode) as i32) == 0i32 {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2668)
                .cast::<*mut u8>())
            .write(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2672))
                .cast::<*mut u8>())
                .wrapping_offset(((id) as i32) as isize))
                .read(),
            );
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2672))
                .cast::<*mut u8>())
            .wrapping_offset(((id) as i32) as isize))
            .write(core::ptr::null_mut());
        } else {
            if ((mode) as i32) == 1i32 {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2668)
                    .cast::<*mut u8>())
                .write(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2696))
                    .cast::<*mut u8>())
                    .wrapping_offset(((id) as i32) as isize))
                    .read(),
                );
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2696))
                .cast::<*mut u8>())
                .wrapping_offset(((id) as i32) as isize))
                .write(core::ptr::null_mut());
            } else {
                return;
            }
        }
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_HeldMon));
        crate::c::bf_write(
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2668)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5),
            2,
            2,
            ((GetMonIconPriorityByCursorPos()) as u16) as i32,
        );
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(67))
        .write(7u8);
    }
}
pub(crate) unsafe extern "C" fn SetPlacedMonSprite(boxId: u8, position: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut position = position;
        if ((boxId) as i32) == 14i32 {
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2672))
                .cast::<*mut u8>())
            .wrapping_offset(((position) as i32) as isize))
            .write(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2668)
                    .cast::<*mut u8>())
                .read(),
            );
            crate::c::bf_write(
                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2672))
                .cast::<*mut u8>())
                .wrapping_offset(((position) as i32) as isize))
                .read())
                .wrapping_add(5),
                2,
                2,
                (1u16) as i32,
            );
            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2672))
            .cast::<*mut u8>())
            .wrapping_offset(((position) as i32) as isize))
            .read())
            .wrapping_add(67))
            .write(12u8);
        } else {
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2696))
                .cast::<*mut u8>())
            .wrapping_offset(((position) as i32) as isize))
            .write(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2668)
                    .cast::<*mut u8>())
                .read(),
            );
            crate::c::bf_write(
                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2696))
                .cast::<*mut u8>())
                .wrapping_offset(((position) as i32) as isize))
                .read())
                .wrapping_add(5),
                2,
                2,
                (2u16) as i32,
            );
            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2696))
            .cast::<*mut u8>())
            .wrapping_offset(((position) as i32) as isize))
            .read())
            .wrapping_add(67))
            .write((((19i32).wrapping_sub(crate::c::rem_i32(((position) as i32), 6i32))) as u8));
        }
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
    }
}
pub(crate) unsafe extern "C" fn SaveMonSpriteAtPos(boxId: u8, position: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut position = position;
        if ((boxId) as i32) == 14i32 {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2816)
                .cast::<*mut *mut u8>())
            .write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2672))
                .cast::<*mut u8>())
                .wrapping_offset(((position) as i32) as isize),
            );
        } else {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2816)
                .cast::<*mut *mut u8>())
            .write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2696))
                .cast::<*mut u8>())
                .wrapping_offset(((position) as i32) as isize),
            );
        }
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3165))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn MoveShiftingMons() -> u8 {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3165))
            .read()) as i32)
            == 16i32
        {
            return 0u8;
        }
        let __p1 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3165);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3165))
            .read()) as i32)
            & 1i32)
            != 0
        {
            let __p2 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2816)
                .cast::<*mut *mut u8>())
            .read())
            .read())
            .wrapping_add(34)
            .cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_sub(1));
            let __p3 = (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2668)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2816)
            .cast::<*mut *mut u8>())
        .read())
        .read())
        .wrapping_add(36)
        .cast::<i16>())
        .write(
            ((crate::c::div_i32(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3165))
                    .read()) as i32)
                        .wrapping_mul(8i32)) as isize,
                ))
                .read()) as i32),
                16i32,
            )) as i16),
        );
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(36)
        .cast::<i16>())
        .write(
            (((crate::c::div_i32(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3165))
                    .read()) as i32)
                        .wrapping_mul(8i32)) as isize,
                ))
                .read()) as i32),
                16i32,
            ))
            .wrapping_neg()) as i16),
        );
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3165))
            .read()) as i32)
            == 8i32
        {
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2668)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5),
                2,
                2,
                (crate::c::bf_read(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2816)
                        .cast::<*mut *mut u8>())
                    .read())
                    .read())
                    .wrapping_add(5),
                    2,
                    2,
                    false,
                ) as u16) as i32,
            );
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2668)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(67))
            .write(
                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2816)
                    .cast::<*mut *mut u8>())
                .read())
                .read())
                .wrapping_add(67))
                .read(),
            );
            crate::c::bf_write(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2816)
                    .cast::<*mut *mut u8>())
                .read())
                .read())
                .wrapping_add(5),
                2,
                2,
                ((GetMonIconPriorityByCursorPos()) as u16) as i32,
            );
            (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2816)
                .cast::<*mut *mut u8>())
            .read())
            .read())
            .wrapping_add(67))
            .write(7u8);
        }
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3165))
            .read()) as i32)
            == 16i32
        {
            let mut sprite: *mut u8 = ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(2668)
            .cast::<*mut u8>())
            .read();
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2668)
                .cast::<*mut u8>())
            .write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2816)
                    .cast::<*mut *mut u8>())
                .read())
                .read(),
            );
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2816)
                .cast::<*mut *mut u8>())
            .read())
            .write(sprite);
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2668)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_HeldMon));
            (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2816)
                .cast::<*mut *mut u8>())
            .read())
            .read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SetReleaseMon(mode: u8, position: u8) {
    unsafe {
        let mut mode = mode;
        let mut position = position;
        'l1: {
            let __sw1 = ((mode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .write(
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2672))
                    .cast::<*mut u8>())
                    .wrapping_offset(((position) as i32) as isize),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .write(
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2696))
                    .cast::<*mut u8>())
                    .wrapping_offset(((position) as i32) as isize),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .write(
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2668)
                        .cast::<*mut u8>(),
                );
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2820)
            .cast::<*mut *mut u8>())
        .read())
        .read()) as usize)
            != 0usize
        {
            InitSpriteAffineAnim(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .read())
                .read(),
            );
            crate::c::bf_write(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .read())
                .read())
                .wrapping_add(1),
                0,
                2,
                (1u32) as i32,
            );
            (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2820)
                .cast::<*mut *mut u8>())
            .read())
            .read())
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
            .write(
                ((&raw const sAffineAnims_ReleaseMon)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
            StartSpriteAffineAnim(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .read())
                .read(),
                0u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TryHideReleaseMonSprite() -> u8 {
    unsafe {
        if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2820)
            .cast::<*mut *mut u8>())
        .read())
        .read()) as usize)
            == 0usize)
            || ((crate::c::bf_read(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .read())
                .read())
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16)
                != 0)
        {
            return 0u8;
        }
        if (crate::c::bf_read(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2820)
                .cast::<*mut *mut u8>())
            .read())
            .read())
            .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0
        {
            crate::c::bf_write(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .read())
                .read())
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DestroyReleaseMonIcon() {
    unsafe {
        if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2820)
            .cast::<*mut *mut u8>())
        .read())
        .read()) as usize)
            != 0usize
        {
            FreeOamMatrix(
                ((crate::c::bf_read(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2820)
                        .cast::<*mut *mut u8>())
                    .read())
                    .read())
                    .wrapping_add(3),
                    1,
                    5,
                    false,
                ) as u32) as u8),
            );
            DestroyBoxMonIcon(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .read())
                .read(),
            );
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2820)
                .cast::<*mut *mut u8>())
            .read())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn ReshowReleaseMon() {
    unsafe {
        if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2820)
            .cast::<*mut *mut u8>())
        .read())
        .read()) as usize)
            != 0usize
        {
            crate::c::bf_write(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .read())
                .read())
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            StartSpriteAffineAnim(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2820)
                    .cast::<*mut *mut u8>())
                .read())
                .read(),
                1u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ResetReleaseMonSpritePtr() -> u8 {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2820)
            .cast::<*mut *mut u8>())
        .read()) as usize)
            == 0usize
        {
            return 0u8;
        }
        if (crate::c::bf_read(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2820)
                .cast::<*mut *mut u8>())
            .read())
            .read())
            .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0
        {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2820)
                .cast::<*mut *mut u8>())
            .write(core::ptr::null_mut());
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SetMovingMonPriority(priority: u8) {
    unsafe {
        let mut priority = priority;
        crate::c::bf_write(
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2668)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5),
            2,
            2,
            ((priority) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HeldMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            (((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(38)
                    .cast::<i16>())
                    .read()) as i32),
                ))
            .wrapping_add(4i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn TryLoadMonIconTiles(species: u16) -> u16 {
    unsafe {
        let mut species = species;
        let mut i: u16 = 0u16;
        let mut offset: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < (if 37i32 >= 40i32 { 37i32 } else { 40i32 })) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2904))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((species) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as i32) == (if 37i32 >= 40i32 { 37i32 } else { 40i32 }) {
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < (if 37i32 >= 40i32 { 37i32 } else { 40i32 })) {
                        break 'l3;
                    }
                    'l4: {
                        if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2904))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 0i32
                        {
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((i) as i32) == (if 37i32 >= 40i32 { 37i32 } else { 40i32 }) {
                return 65535u16;
            }
        }
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2904))
            .cast::<u16>())
        .wrapping_offset(((i) as i32) as isize))
        .write(species);
        let __p1 = (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2824))
        .cast::<u16>())
        .wrapping_offset(((i) as i32) as isize);
        (__p1).write(((__p1).read()).wrapping_add(1));
        offset = (((16i32).wrapping_mul(((i) as i32))) as u16);
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            GetMonIconTiles(species, 1u32),
                            ((100728832i32) as usize as *mut u8).wrapping_offset(
                                (((offset) as i32).wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                                    as isize
                                    * 1,
                            ),
                            ((67108864i32
                                | (crate::c::div_i32(512i32, crate::c::div_i32(32i32, 8i32))
                                    & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        return offset;
    }
}
pub(crate) unsafe extern "C" fn RemoveSpeciesFromIconList(species: u16) {
    unsafe {
        let mut species = species;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < (if 37i32 >= 40i32 { 37i32 } else { 40i32 })) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2904))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((species) as i32)
                    {
                        if (({
                            let __p1 = (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2824))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize);
                            let __t2 = ((__p1).read()).wrapping_sub(1);
                            (__p1).write(__t2);
                            __t2
                        }) as i32)
                            == 0i32
                        {
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2904))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(0u16);
                        }
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMonIconSprite(
    species: u16,
    personality: u32,
    x: i16,
    y: i16,
    oamPriority: u8,
    subpriority: u8,
) -> *mut u8 {
    unsafe {
        let mut species = species;
        let mut personality = personality;
        let mut x = x;
        let mut y = y;
        let mut oamPriority = oamPriority;
        let mut subpriority = subpriority;
        let mut tileNum: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut template = crate::ffi::Align4([0u8; 24]);
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_MonIcon)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        species = GetIconSpecies(species, personality);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(
            (((56000i32).wrapping_add(
                (((((&raw mut gMonIconPaletteIndices).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize))
                .read()) as i32),
            )) as u16),
        );
        tileNum = TryLoadMonIconTiles(species);
        if ((tileNum) as i32) == 65535i32 {
            return core::ptr::null_mut();
        }
        spriteId = CreateSprite((&raw mut template).cast::<u8>(), x, y, subpriority);
        if ((spriteId) as i32) == 64i32 {
            RemoveSpeciesFromIconList(species);
            return core::ptr::null_mut();
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(4),
            0,
            10,
            (tileNum) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            ((oamPriority) as u16) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((species) as i16));
        return ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68);
    }
}
pub(crate) unsafe extern "C" fn DestroyBoxMonIcon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        RemoveSpeciesFromIconList((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16));
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateInitBoxTask(boxId: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut taskId: u8 = CreateTask(Some(Task_InitBox), 2u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((boxId) as i16));
    }
}
pub(crate) unsafe extern "C" fn IsInitBoxActive() -> u8 {
    unsafe {
        return FuncIsActiveTask(Some(Task_InitBox));
    }
}
pub(crate) unsafe extern "C" fn Task_InitBox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(722))
                    .write(0u8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(716)
                    .cast::<u16>())
                .write(0u16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                    RequestDma3Fill(
                        0i32,
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(19140))
                        .cast::<u8>(),
                        4096u16,
                        1u8,
                    ),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((CheckForSpaceForDma3Request(
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
                )) as i32)
                    == (-1i32)
                {
                    return;
                }
                SetBgTilemapBuffer(
                    2u8,
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(19140))
                    .cast::<u8>(),
                );
                ShowBg(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadWallpaperGfx(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
                    0i8,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((WaitForWallpaperGfxLoad()) != 0) {
                    return;
                }
                InitBoxTitle(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
                );
                CreateBoxScrollArrows();
                InitBoxMonSprites(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
                );
                SetGpuReg(12u8, 23306u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                DestroyTask(taskId);
                break 'l1;
            }
            if !__matched {
                (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                return;
            }
        }
        let __p2 = ((task).wrapping_add(8)).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn SetUpScrollToBox(boxId: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut direction: i8 = DetermineBoxScrollDirection(boxId);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(718)
            .cast::<i16>())
        .write(
            ((if ((direction) as i32) > 0i32 {
                6i32
            } else {
                (-6i32)
            }) as i16),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(723)).write(
            ((if ((direction) as i32) > 0i32 {
                1i32
            } else {
                2i32
            }) as u8),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(720)
            .cast::<u16>())
        .write(32u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(724))
            .write(boxId);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(726)
            .cast::<u16>())
        .write(
            ((if ((direction) as i32) <= 0i32 {
                5i32
            } else {
                0i32
            }) as u16),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<i16>())
        .write(((direction) as i16));
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(730)
            .cast::<u16>())
        .write(
            ((if ((direction) as i32) > 0i32 {
                264i32
            } else {
                56i32
            }) as u16),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(732)
            .cast::<u16>())
        .write(
            ((if ((direction) as i32) <= 0i32 {
                5i32
            } else {
                0i32
            }) as u16),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(734)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(736)
            .cast::<u16>())
        .write(2u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2660))
            .write(boxId);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2661)
            .cast::<i8>())
        .write(direction);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2659))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn ScrollToBox() -> u8 {
    unsafe {
        let mut iconsScrolling: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2659))
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                LoadWallpaperGfx(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2660))
                    .read(),
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2661)
                        .cast::<i8>())
                    .read(),
                );
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2659);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if !((WaitForWallpaperGfxLoad()) != 0) {
                    return 1u8;
                }
                InitBoxMonIconScroll(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2660))
                    .read(),
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2661)
                        .cast::<i8>())
                    .read(),
                );
                CreateIncomingBoxTitle(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2660))
                    .read(),
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2661)
                        .cast::<i8>())
                    .read(),
                );
                StartBoxScrollArrowsSlide(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2661)
                        .cast::<i8>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                iconsScrolling = UpdateBoxMonIconScroll();
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(720)
                    .cast::<u16>())
                .read()) as i32)
                    != 0i32
                {
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(716)
                        .cast::<u16>();
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_add(
                            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(718)
                                .cast::<i16>())
                            .read()) as i32),
                        )) as u16),
                    );
                    if (({
                        let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(720)
                            .cast::<u16>();
                        let __t5 = ((__p4).read()).wrapping_sub(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        != 0i32
                    {
                        return 1u8;
                    }
                    CycleBoxTitleSprites();
                    StopBoxScrollArrowsSlide();
                }
                return iconsScrolling;
            }
        }
        let __p6 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2659);
        (__p6).write(((__p6).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DetermineBoxScrollDirection(boxId: u8) -> i8 {
    unsafe {
        let mut boxId = boxId;
        let mut i: u8 = 0u8;
        let mut currentBox: u8 = StorageGetCurrentBox();
        {
            i = 0u8;
            'l1: loop {
                if !(((currentBox) as i32) != ((boxId) as i32)) {
                    break 'l1;
                }
                'l2: {
                    currentBox = (currentBox).wrapping_add(1);
                    if ((currentBox) as i32) >= 14i32 {
                        currentBox = 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((if ((i) as i32) < crate::c::div_i32(14i32, 2i32) {
            1i32
        } else {
            (-1i32)
        }) as i8);
    }
}
pub(crate) unsafe extern "C" fn SetWallpaperForCurrentBox(wallpaperId: u8) {
    unsafe {
        let mut wallpaperId = wallpaperId;
        let mut boxId: u8 = StorageGetCurrentBox();
        SetBoxWallpaper(boxId, wallpaperId);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2658))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn DoWallpaperGfxChange() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2658))
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1848)
                        .cast::<u32>())
                    .read(),
                    1i8,
                    0u8,
                    16u8,
                    65535u16,
                );
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2658);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdatePaletteFade()) != 0) {
                    let mut curBox: u8 = StorageGetCurrentBox();
                    LoadWallpaperGfx(curBox, 0i8);
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2658);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if WaitForWallpaperGfxLoad() == 1u32 {
                    CycleBoxTitleColor();
                    BeginNormalPaletteFade(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1848)
                            .cast::<u32>())
                        .read(),
                        1i8,
                        16u8,
                        0u8,
                        65535u16,
                    );
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2658);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((UpdatePaletteFade()) != 0) {
                    let __p5 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2658);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn LoadWallpaperGfx(boxId: u8, direction: i8) {
    unsafe {
        let mut boxId = boxId;
        let mut direction = direction;
        let mut wallpaperId: u8 = 0u8;
        let mut wallpaper: *mut u8 = core::ptr::null_mut();
        let mut iconGfx: *mut u8 = core::ptr::null_mut();
        let mut tilesSize: u32 = 0u32;
        let mut iconSize: u32 = 0u32;
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1785))
            .write(0u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1786))
            .write(boxId);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1787)
            .cast::<i8>())
        .write(direction);
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1787)
            .cast::<i8>())
        .read()) as i32)
            != 0i32
        {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(722))
                .write(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(722))
                    .read()) as i32)
                        == 0i32) as u8),
                );
            TrimOldWallpaper(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19140))
                    .cast::<u8>(),
            );
        }
        wallpaperId = GetBoxWallpaper(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1786))
                .read(),
        );
        if ((wallpaperId) as i32) != 16i32 {
            wallpaper = (((&raw const sWallpapers).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((wallpaperId) as i32) as isize * 12);
            LZ77UnCompWram(
                ((wallpaper).wrapping_add(4).cast::<*mut u32>()).read(),
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1938))
                .cast::<u16>())
                .cast::<u8>(),
            );
            DrawWallpaper(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1938))
                .cast::<u16>())
                .cast::<u8>(),
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1787)
                    .cast::<i8>())
                .read(),
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(722))
                    .read(),
            );
            if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1787)
                .cast::<i8>())
            .read()) as i32)
                != 0i32
            {
                LoadPalette(
                    (((wallpaper).wrapping_add(8).cast::<*mut u16>()).read()).cast::<u8>(),
                    (((64i32).wrapping_add(
                        (0i32).wrapping_add(
                            (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(722))
                            .read()) as i32)
                                .wrapping_mul(2i32))
                            .wrapping_mul(16i32),
                        ),
                    )) as u16),
                    64u16,
                );
            } else {
                'l1: loop {
                    'l2: {
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    (((wallpaper).wrapping_add(8).cast::<*mut u16>()).read())
                                        .cast::<u8>(),
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(
                                            ((64i32).wrapping_add(
                                                (0i32).wrapping_add(
                                                    (((((((&raw mut sStorage)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(722))
                                                    .read())
                                                        as i32)
                                                        .wrapping_mul(2i32))
                                                    .wrapping_mul(16i32),
                                                ),
                                            )) as isize,
                                        ))
                                    .cast::<u8>(),
                                    (0u32
                                        | (crate::c::div_u32(
                                            64u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l1;
                    }
                }
            }
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2664)
                .cast::<*mut u8>())
            .write(malloc_and_decompress(
                (((wallpaper).cast::<*mut u32>()).read()).cast::<u8>(),
                &raw mut tilesSize,
            ));
            LoadBgTiles(
                2u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2664)
                    .cast::<*mut u8>())
                .read(),
                ((tilesSize) as u16),
                ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(722))
                .read()) as i32)
                    << 8) as u16),
            );
        } else {
            wallpaper = (((&raw const sWaldaWallpapers).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((GetWaldaWallpaperPatternId()) as i32) as isize * 12);
            LZ77UnCompWram(
                ((wallpaper).wrapping_add(4).cast::<*mut u32>()).read(),
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1938))
                .cast::<u16>())
                .cast::<u8>(),
            );
            DrawWallpaper(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1938))
                .cast::<u16>())
                .cast::<u8>(),
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1787)
                    .cast::<i8>())
                .read(),
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(722))
                    .read(),
            );
            'l5: loop {
                'l6: {
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (((wallpaper).wrapping_add(8).cast::<*mut u16>()).read())
                                    .cast::<u8>(),
                                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1938))
                                .cast::<u16>())
                                .cast::<u8>(),
                                ((0i32
                                    | (crate::c::div_i32(64i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l5;
                }
            }
            'l9: loop {
                'l10: {
                    'l11: loop {
                        'l12: {
                            CpuSet(
                                (GetWaldaWallpaperColorsPtr()).cast::<u8>(),
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1938))
                                .cast::<u16>())
                                .wrapping_offset(1))
                                .cast::<u8>(),
                                ((0i32
                                    | (crate::c::div_i32(4i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l9;
                }
            }
            'l13: loop {
                'l14: {
                    'l15: loop {
                        'l16: {
                            CpuSet(
                                (GetWaldaWallpaperColorsPtr()).cast::<u8>(),
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1938))
                                .cast::<u16>())
                                .wrapping_offset(17))
                                .cast::<u8>(),
                                ((0i32
                                    | (crate::c::div_i32(4i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l15;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l13;
                }
            }
            if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1787)
                .cast::<i8>())
            .read()) as i32)
                != 0i32
            {
                LoadPalette(
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1938))
                    .cast::<u16>())
                    .cast::<u8>(),
                    (((64i32).wrapping_add(
                        (0i32).wrapping_add(
                            (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(722))
                            .read()) as i32)
                                .wrapping_mul(2i32))
                            .wrapping_mul(16i32),
                        ),
                    )) as u16),
                    64u16,
                );
            } else {
                'l17: loop {
                    'l18: {
                        'l19: loop {
                            'l20: {
                                CpuSet(
                                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1938))
                                    .cast::<u16>())
                                    .cast::<u8>(),
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(
                                            ((64i32).wrapping_add(
                                                (0i32).wrapping_add(
                                                    (((((((&raw mut sStorage)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(722))
                                                    .read())
                                                        as i32)
                                                        .wrapping_mul(2i32))
                                                    .wrapping_mul(16i32),
                                                ),
                                            )) as isize,
                                        ))
                                    .cast::<u8>(),
                                    (0u32
                                        | (crate::c::div_u32(
                                            64u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l19;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l17;
                    }
                }
            }
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2664)
                .cast::<*mut u8>())
            .write(malloc_and_decompress(
                (((wallpaper).cast::<*mut u32>()).read()).cast::<u8>(),
                &raw mut tilesSize,
            ));
            iconGfx = malloc_and_decompress(
                (((((&raw const sWaldaWallpaperIcons)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u32>())
                .cast::<*mut u32>())
                .wrapping_offset(((GetWaldaWallpaperIconId()) as i32) as isize))
                .read())
                .cast::<u8>(),
                &raw mut iconSize,
            );
            'l21: loop {
                'l22: {
                    'l23: loop {
                        'l24: {
                            CpuSet(
                                iconGfx,
                                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2664)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(2048),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        iconSize,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l23;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l21;
                }
            }
            Free(iconGfx);
            LoadBgTiles(
                2u8,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2664)
                    .cast::<*mut u8>())
                .read(),
                ((tilesSize) as u16),
                ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(722))
                .read()) as i32)
                    << 8) as u16),
            );
        }
        CopyBgTilemapBufferToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn WaitForWallpaperGfxLoad() -> u32 {
    unsafe {
        if (IsDma3ManagerBusyWithBgCopy()) != 0 {
            return 0u32;
        }
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2664)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            Free(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2664)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2664)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn DrawWallpaper(tilemap: *mut u8, direction: i8, offset: u8) {
    unsafe {
        let mut tilemap = tilemap;
        let mut direction = direction;
        let mut offset = offset;
        let mut tileOffset: i16 = ((((offset) as i32).wrapping_mul(256i32)) as i16);
        let mut paletteNum: i16 =
            (((((offset) as i32).wrapping_mul(2i32)).wrapping_add(3i32)) as i16);
        let mut x: i16 = ((((crate::c::div_i32(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(716)
                .cast::<u16>())
            .read()) as i32),
            8i32,
        ))
        .wrapping_add(10i32))
        .wrapping_add(((direction) as i32).wrapping_mul(24i32))
            & 63i32) as i16);
        CopyRectToBgTilemapBufferRect(
            2u8,
            tilemap,
            0u8,
            0u8,
            20u8,
            18u8,
            ((x) as u8),
            2u8,
            20u8,
            18u8,
            17u8,
            tileOffset,
            paletteNum,
        );
        if ((direction) as i32) == 0i32 {
            return;
        }
        if ((direction) as i32) > 0i32 {
            x = ((((x) as i32).wrapping_add(20i32)) as i16);
        } else {
            x = ((((x) as i32).wrapping_sub(4i32)) as i16);
        }
        FillBgTilemapBufferRect(2u8, 0u16, ((x) as u8), 2u8, 4u8, 18u8, 17u8);
    }
}
pub(crate) unsafe extern "C" fn TrimOldWallpaper(tilemap: *mut u8) {
    unsafe {
        let mut tilemap = tilemap;
        let mut i: u16 = 0u16;
        let mut dest: *mut u16 = (tilemap).cast::<u16>();
        let mut r3: i16 = (((crate::c::div_i32(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(716)
                .cast::<u16>())
            .read()) as i32),
            8i32,
        ))
        .wrapping_add(30i32)
            & 63i32) as i16);
        if ((r3) as i32) <= 31i32 {
            dest = (dest).wrapping_offset((((r3) as i32).wrapping_add(608i32)) as isize);
        } else {
            dest = (dest).wrapping_offset((((r3) as i32).wrapping_add(1600i32)) as isize);
        }
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 44i32) {
                    break 'l1;
                }
                'l2: {
                    ({
                        let __t1 = dest;
                        dest = (dest).wrapping_offset(1);
                        __t1
                    })
                    .write(0u16);
                    r3 = ((((r3) as i32).wrapping_add(1i32) & 63i32) as i16);
                    if ((r3) as i32) == 0i32 {
                        dest = (dest).wrapping_offset(-1056);
                    }
                    if ((r3) as i32) == 32i32 {
                        dest = (dest).wrapping_offset(992);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitBoxTitle(boxId: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut tagIndex: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut i: u16 = 0u16;
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(760))
                    .cast::<u8>(),
            );
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(512u16);
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(3u16);
        let mut palettes = crate::ffi::Align4([0u8; 16]);
        (&raw mut palettes)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1788))
                    .cast::<u16>(),
            );
        (&raw mut palettes)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(4)
            .cast::<u16>()
            .write(56009u16);
        let mut wallpaperId: u16 = ((GetBoxWallpaper(boxId)) as u16);
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1788))
            .cast::<u16>())
        .wrapping_offset(14))
        .write(
            (((((&raw const sBoxTitleColors).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((wallpaperId) as i32) as isize * 4))
            .cast::<u16>())
            .read(),
        );
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1788))
            .cast::<u16>())
        .wrapping_offset(15))
        .write(
            ((((((&raw const sBoxTitleColors).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((wallpaperId) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(1))
            .read(),
        );
        LoadSpritePalettes((&raw mut palettes).cast::<u8>());
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1848)
            .cast::<u32>())
        .write(1008u32);
        tagIndex = IndexOfSpritePaletteTag(56009u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1820)
            .cast::<u16>())
        .write(
            ((((256i32).wrapping_add(((tagIndex) as i32).wrapping_mul(16i32))).wrapping_add(14i32))
                as u16),
        );
        let __p1 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1848)
            .cast::<u32>();
        (__p1).write(((__p1).read() | ((crate::c::shl_i32(65536i32, ((tagIndex) as u32))) as u32)));
        tagIndex = IndexOfSpritePaletteTag(56009u16);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1822)
            .cast::<u16>())
        .write(
            ((((256i32).wrapping_add(((tagIndex) as i32).wrapping_mul(16i32))).wrapping_add(14i32))
                as u16),
        );
        let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1848)
            .cast::<u32>();
        (__p2).write(((__p2).read() | ((crate::c::shl_i32(65536i32, ((tagIndex) as u32))) as u32)));
        StringCopyPadded(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8632))
                .cast::<u8>(),
            GetBoxNamePtr(boxId),
            0u8,
            8u16,
        );
        DrawTextWindowAndBufferTiles(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8632))
                .cast::<u8>(),
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(760))
                .cast::<u8>(),
            0u8,
            0u8,
            2i32,
        );
        LoadSpriteSheet((&raw mut spriteSheet).cast::<u8>());
        x = GetBoxTitleBaseX(GetBoxNamePtr(boxId));
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw const sSpriteTemplate_BoxTitle)
                            .cast::<u8>()
                            .cast_mut(),
                        ((((x) as i32).wrapping_add(((i) as i32).wrapping_mul(32i32))) as i16),
                        28i16,
                        24u8,
                    );
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1824))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    StartSpriteAnim(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1824))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        ((i) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1784))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn CreateIncomingBoxTitle(boxId: u8, direction: i8) {
    unsafe {
        let mut boxId = boxId;
        let mut direction = direction;
        let mut palOffset: u16 = 0u16;
        let mut x: i16 = 0i16;
        let mut adjustedX: i16 = 0i16;
        let mut i: u16 = 0u16;
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(760))
                    .cast::<u8>(),
            );
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(512u16);
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(3u16);
        let mut template = crate::ffi::Align4([0u8; 24]);
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_BoxTitle)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1784)).write(
            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1784))
                .read()) as i32)
                == 0i32) as u8),
        );
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1784))
            .read()) as i32)
            == 0i32
        {
            (((&raw mut spriteSheet).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .write(3u16);
            palOffset = ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1820)
                .cast::<u16>())
            .read();
        } else {
            (((&raw mut spriteSheet).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .write(4u16);
            palOffset = ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1820)
                .cast::<u16>())
            .read();
            (((&raw mut template).cast::<u8>()).cast::<u16>()).write(4u16);
            (((&raw mut template).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .write(56009u16);
        }
        StringCopyPadded(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8632))
                .cast::<u8>(),
            GetBoxNamePtr(boxId),
            0u8,
            8u16,
        );
        DrawTextWindowAndBufferTiles(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8632))
                .cast::<u8>(),
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(760))
                .cast::<u8>(),
            0u8,
            0u8,
            2i32,
        );
        LoadSpriteSheet((&raw mut spriteSheet).cast::<u8>());
        LoadPalette(
            (((((&raw const sBoxTitleColors).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((GetBoxWallpaper(boxId)) as i32) as isize * 4))
            .cast::<u16>())
            .cast::<u8>(),
            palOffset,
            4u16,
        );
        x = GetBoxTitleBaseX(GetBoxNamePtr(boxId));
        adjustedX = x;
        adjustedX =
            ((((adjustedX) as i32).wrapping_add(((direction) as i32).wrapping_mul(192i32))) as i16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw mut template).cast::<u8>(),
                        (((((i) as i32).wrapping_mul(32i32)).wrapping_add(((adjustedX) as i32)))
                            as i16),
                        28i16,
                        24u8,
                    );
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1832))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1832))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write((((((direction) as i32).wrapping_neg()).wrapping_mul(6i32)) as i16));
                    ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1832))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        (((((i) as i32).wrapping_mul(32i32)).wrapping_add(((x) as i32))) as i16),
                    );
                    ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1832))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1832))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_IncomingBoxTitle));
                    StartSpriteAnim(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1832))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        ((i) as u8),
                    );
                    (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1824))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write((((((direction) as i32).wrapping_neg()).wrapping_mul(6i32)) as i16));
                    ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1824))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(1i16);
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1824))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_OutgoingBoxTitle));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CycleBoxTitleSprites() {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1784))
            .read()) as i32)
            == 0i32
        {
            FreeSpriteTilesByTag(4u16);
        } else {
            FreeSpriteTilesByTag(3u16);
        }
        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1824))
            .cast::<*mut u8>())
        .write(
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1832))
                .cast::<*mut u8>())
            .read(),
        );
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1824))
            .cast::<*mut u8>())
        .wrapping_offset(1))
        .write(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1832))
                .cast::<*mut u8>())
            .wrapping_offset(1))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_IncomingBoxTitle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            != 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            if (({
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                let __v3 = (((((__p2).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16);
                (__p2).write(__v3);
                __v3
            }) as i32)
                == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OutgoingBoxTitle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            != 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                < 64i32)
                || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    > 256i32)
            {
                DestroySprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CycleBoxTitleColor() {
    unsafe {
        let mut boxId: u8 = StorageGetCurrentBox();
        let mut wallpaperId: u8 = GetBoxWallpaper(boxId);
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1784))
            .read()) as i32)
            == 0i32
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (((((&raw const sBoxTitleColors).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((wallpaperId) as i32) as isize * 4))
                                .cast::<u16>())
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1820)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize,
                                    ))
                                .cast::<u8>(),
                                (0u32
                                    | (crate::c::div_u32(
                                        4u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        } else {
            'l5: loop {
                'l6: {
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (((((&raw const sBoxTitleColors).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((wallpaperId) as i32) as isize * 4))
                                .cast::<u16>())
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1822)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize,
                                    ))
                                .cast::<u8>(),
                                (0u32
                                    | (crate::c::div_u32(
                                        4u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l5;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetBoxTitleBaseX(string: *mut u8) -> i16 {
    unsafe {
        let mut string = string;
        return (((176i32).wrapping_sub(crate::c::div_i32(GetStringWidth(1u8, string, 0i16), 2i32)))
            as i16);
    }
}
pub(crate) unsafe extern "C" fn CreateBoxScrollArrows() {
    unsafe {
        let mut i: u16 = 0u16;
        LoadSpriteSheet((&raw const sSpriteSheet_Arrow).cast::<u8>().cast_mut());
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw const sSpriteTemplate_Arrow).cast::<u8>().cast_mut(),
                        (((92i32).wrapping_add(((i) as i32).wrapping_mul(136i32))) as i16),
                        28i16,
                        22u8,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68);
                        StartSpriteAnim(sprite, ((i) as u8));
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                            .write(((if ((i) as i32) == 0i32 { (-1i32) } else { 1i32 }) as i16));
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1840))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(sprite);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (IsCursorOnBoxTitle()) != 0 {
            AnimateBoxScrollArrows(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn StartBoxScrollArrowsSlide(direction: i8) {
    unsafe {
        let mut direction = direction;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1840))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                    (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1840))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(2i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((direction) as i32) < 0i32 {
            (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1840))
            .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(29i16);
            ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1840))
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(5i16);
            (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1840))
            .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(72i16);
            ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1840))
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(72i16);
        } else {
            (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1840))
            .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(5i16);
            ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1840))
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(29i16);
            (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1840))
            .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(248i16);
            ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1840))
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(248i16);
        }
        (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1840))
        .cast::<*mut u8>())
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1840))
        .cast::<*mut u8>())
        .wrapping_offset(1))
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn StopBoxScrollArrowsSlide() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1840))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(((((136i32).wrapping_mul(((i) as i32))).wrapping_add(92i32)) as i16));
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1840))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                    crate::c::bf_write(
                        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1840))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        AnimateBoxScrollArrows(1u8);
    }
}
pub(crate) unsafe extern "C" fn AnimateBoxScrollArrows(animate: u8) {
    unsafe {
        let mut animate = animate;
        let mut i: u16 = 0u16;
        if (animate) != 0 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 2i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1840))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(1i16);
                        ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1840))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                        ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1840))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(0i16);
                        ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1840))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(0i16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 2i32) {
                        break 'l3;
                    }
                    'l4: {
                        (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1840))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(0i16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Arrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 3i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p4 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_add(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32),
                        )) as i16),
                    );
                    if (({
                        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        let __t6 = ((__p5).read()).wrapping_add(1);
                        (__p5).write(__t6);
                        __t6
                    }) as i32)
                        > 5i32
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .write(0i16);
                        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(3i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p7 = (sprite).wrapping_add(32).cast::<i16>();
                (__p7).write(
                    (((((__p7).read()) as i32).wrapping_sub(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(718)
                            .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
                if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= 72i32)
                    || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 248i32)
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                }
                if (({
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_sub(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 0i32
                {
                    ((sprite).wrapping_add(32).cast::<i16>()).write(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    );
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(4i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p10 = (sprite).wrapping_add(32).cast::<i16>();
                (__p10).write(
                    (((((__p10).read()) as i32).wrapping_sub(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(718)
                            .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateChooseBoxArrows(
    x: u16,
    y: u16,
    animId: u8,
    priority: u8,
    subpriority: u8,
) -> *mut u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut animId = animId;
        let mut priority = priority;
        let mut subpriority = subpriority;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_Arrow).cast::<u8>().cast_mut(),
            ((x) as i16),
            ((y) as i16),
            subpriority,
        );
        if ((spriteId) as i32) == 64i32 {
            return core::ptr::null_mut();
        }
        animId = ((crate::c::rem_i32(((animId) as i32), 2i32)) as u8);
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            animId,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            ((priority) as u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        return ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68);
    }
}
pub(crate) unsafe extern "C" fn InitCursor() {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 1i32
        {
            ((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).write(0i8);
        } else {
            ((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).write(1i8);
        }
        ((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).write(0i8);
        ((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sMovingMonOrigBoxId).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sMovingMonOrigBoxPos).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sAutoActionOn).cast::<u8>().cast::<u8>()).write(0u8);
        ClearSavedCursorPos();
        CreateCursorSprites();
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3286))
            .write(1u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8703))
            .write(0u8);
        TryRefreshDisplayMon();
    }
}
pub(crate) unsafe extern "C" fn InitCursorOnReopen() {
    unsafe {
        CreateCursorSprites();
        ReshowDisplayMon();
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3286))
            .write(1u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8703))
            .write(0u8);
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8356)
                .cast::<crate::c::Rec4<100>>()
                .write_unaligned(
                    (&raw mut sSavedMovingMon)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<100>>()
                        .read_unaligned(),
                );
            CreateMovingMonIcon();
        }
    }
}
pub(crate) unsafe extern "C" fn GetCursorCoordsByPos(
    cursorArea: u8,
    cursorPosition: u8,
    x: *mut u16,
    y: *mut u16,
) {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut cursorPosition = cursorPosition;
        let mut x = x;
        let mut y = y;
        'l1: {
            let __sw1 = ((cursorArea) as i32);
            if __sw1 == 0i32 {
                (x).write(
                    ((((crate::c::rem_i32(((cursorPosition) as i32), 6i32)).wrapping_mul(24i32))
                        .wrapping_add(100i32)) as u16),
                );
                (y).write(
                    ((((crate::c::div_i32(((cursorPosition) as i32), 6i32)).wrapping_mul(24i32))
                        .wrapping_add(32i32)) as u16),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((cursorPosition) as i32) == 0i32 {
                    (x).write(104u16);
                    (y).write(52u16);
                } else {
                    if ((cursorPosition) as i32) == 6i32 {
                        (x).write(152u16);
                        (y).write(132u16);
                    } else {
                        (x).write(152u16);
                        (y).write(
                            ((((((cursorPosition) as i32).wrapping_sub(1i32)).wrapping_mul(24i32))
                                .wrapping_add(4i32)) as u16),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                (x).write(162u16);
                (y).write(12u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                (y).write(
                    ((if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
                        8i32
                    } else {
                        14i32
                    }) as u16),
                );
                (x).write(
                    (((((cursorPosition) as i32).wrapping_mul(88i32)).wrapping_add(120i32)) as u16),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                (x).write(160u16);
                (y).write(96u16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetSpeciesAtCursorPosition() -> u16 {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 0i32;
            if __sw1 == 1i32 {
                return ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            }
            if __sw1 == 0i32 {
                return ((GetCurrentBoxMonData(
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                    11i32,
                )) as u16);
            }
            if !__matched {
                return 0u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateCursorPos() -> u8 {
    unsafe {
        let mut tmp: i16 = 0i16;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3280)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .read()) as i32)
                != 3i32
            {
                return 0u8;
            } else {
                return IsItemIconAnimActive();
            }
        } else {
            if (({
                let __p1 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3280)
                    .cast::<u16>();
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                != 0i32
            {
                let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3260)
                    .cast::<i32>();
                (__p3).write(
                    (((((__p3).read()) as u32).wrapping_add(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3268)
                            .cast::<u32>())
                        .read(),
                    )) as i32),
                );
                let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3264)
                    .cast::<i32>();
                (__p4).write(
                    (((((__p4).read()) as u32).wrapping_add(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3272)
                            .cast::<u32>())
                        .read(),
                    )) as i32),
                );
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .write(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3260)
                        .cast::<i32>())
                    .read()
                        >> 8) as i16),
                );
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .write(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3264)
                        .cast::<i32>())
                    .read()
                        >> 8) as i16),
                );
                if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    > 256i32
                {
                    tmp = ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_sub(256i32)) as i16);
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(((((tmp) as i32).wrapping_add(64i32)) as i16));
                }
                if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    < 64i32
                {
                    tmp = (((64i32).wrapping_sub(
                        ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(32)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16);
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write((((256i32).wrapping_sub(((tmp) as i32))) as i16));
                }
                if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    > 176i32
                {
                    tmp = ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_sub(176i32)) as i16);
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(((((tmp) as i32).wrapping_sub(16i32)) as i16));
                }
                if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    < (-16i32)
                {
                    tmp = (((-16i32).wrapping_sub(
                        ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(34)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16);
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write((((176i32).wrapping_sub(((tmp) as i32))) as i16));
                }
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3287))
                .read())
                    != 0)
                    && ((({
                        let __p5 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3287);
                        let __t6 = ((__p5).read()).wrapping_sub(1);
                        (__p5).write(__t6);
                        __t6
                    }) as i32)
                        == 0i32)
                {
                    crate::c::bf_write(
                        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(63),
                        1,
                        1,
                        ((((crate::c::bf_read(
                            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3252)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(63),
                            1,
                            1,
                            false,
                        ) as u16) as i32)
                            == 0i32) as u16) as i32,
                    );
                }
            } else {
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .write(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3276)
                        .cast::<i16>())
                    .read(),
                );
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .write(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3278)
                        .cast::<i16>())
                    .read(),
                );
                DoCursorNewPosUpdate();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn InitNewCursorPos(newCursorArea: u8, newCursorPosition: u8) {
    unsafe {
        let mut newCursorArea = newCursorArea;
        let mut newCursorPosition = newCursorPosition;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        GetCursorCoordsByPos(newCursorArea, newCursorPosition, &raw mut x, &raw mut y);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3284))
            .write(newCursorArea);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3285))
            .write(newCursorPosition);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3276)
            .cast::<i16>())
        .write(((x) as i16));
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3278)
            .cast::<i16>())
        .write(((y) as i16));
    }
}
pub(crate) unsafe extern "C" fn InitCursorMove() {
    unsafe {
        let mut yDistance: i32 = 0i32;
        let mut xDistance: i32 = 0i32;
        if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3282)
            .cast::<i8>())
        .read()) as i32)
            != 0i32)
            || (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3283)
                .cast::<i8>())
            .read()) as i32)
                != 0i32)
        {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3280)
                .cast::<u16>())
            .write(12u16);
        } else {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3280)
                .cast::<u16>())
            .write(6u16);
        }
        if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3287))
            .read())
            != 0
        {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3287))
                .write(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3280)
                        .cast::<u16>())
                    .read()) as i32)
                        >> 1) as u8),
                );
        }
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3282)
                .cast::<i8>())
            .read()) as i32);
            let __matched = __sw1 == (-1i32) || __sw1 == 1i32;
            if !__matched {
                yDistance = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3278)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(
                        ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(34)
                        .cast::<i16>())
                        .read()) as i32),
                    );
                break 'l1;
            }
            if __sw1 == (-1i32) {
                yDistance = (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3278)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(192i32))
                .wrapping_sub(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .read()) as i32),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                yDistance = (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3278)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(192i32))
                .wrapping_sub(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .read()) as i32),
                );
                break 'l1;
            }
        }
        'l2: {
            let __sw2 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3283)
                .cast::<i8>())
            .read()) as i32);
            let __matched = __sw2 == (-1i32) || __sw2 == 1i32;
            if !__matched {
                xDistance = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3276)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(
                        ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(32)
                        .cast::<i16>())
                        .read()) as i32),
                    );
                break 'l2;
            }
            if __sw2 == (-1i32) {
                xDistance = (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3276)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(192i32))
                .wrapping_sub(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32),
                );
                break 'l2;
            }
            if __sw2 == 1i32 {
                xDistance = (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3276)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(192i32))
                .wrapping_sub(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32),
                );
                break 'l2;
            }
        }
        yDistance = (yDistance << 8);
        xDistance = (xDistance << 8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3268)
            .cast::<u32>())
        .write(
            ((crate::c::div_i32(
                xDistance,
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3280)
                    .cast::<u16>())
                .read()) as i32),
            )) as u32),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3272)
            .cast::<u32>())
        .write(
            ((crate::c::div_i32(
                yDistance,
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3280)
                    .cast::<u16>())
                .read()) as i32),
            )) as u32),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3260)
            .cast::<i32>())
        .write(
            (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                << 8),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3264)
            .cast::<i32>())
        .write(
            (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                << 8),
        );
    }
}
pub(crate) unsafe extern "C" fn SetCursorPosition(newCursorArea: u8, newCursorPosition: u8) {
    unsafe {
        let mut newCursorArea = newCursorArea;
        let mut newCursorPosition = newCursorPosition;
        InitNewCursorPos(newCursorArea, newCursorPosition);
        InitCursorMove();
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8703))
            .read()) as i32)
                == 0i32)
                && (!((((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0))
            {
                StartSpriteAnim(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read(),
                    1u8,
                );
            }
        } else {
            if !((IsMovingItem()) != 0) {
                StartSpriteAnim(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read(),
                    1u8,
                );
            }
        }
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            == 3i32
        {
            if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 0i32 {
                TryHideItemIconAtPos(
                    0u8,
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                );
            } else {
                if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32 {
                    TryHideItemIconAtPos(
                        1u8,
                        ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                    );
                }
            }
            if ((newCursorArea) as i32) == 0i32 {
                TryLoadItemIconAtPos(newCursorArea, newCursorPosition);
            } else {
                if ((newCursorArea) as i32) == 1i32 {
                    TryLoadItemIconAtPos(newCursorArea, newCursorPosition);
                }
            }
        }
        if (((newCursorArea) as i32) == 1i32)
            && (((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) != 1i32)
        {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3286))
                .write(1u8);
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3256)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        'l1: {
            let __sw1 = ((newCursorArea) as i32);
            if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                crate::c::bf_write(
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(5),
                    2,
                    2,
                    (1u16) as i32,
                );
                crate::c::bf_write(
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3256)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                crate::c::bf_write(
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3256)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(5),
                    2,
                    2,
                    (1u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 0i32 {
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8703))
                .read()) as i32)
                    != 0i32
                {
                    crate::c::bf_write(
                        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3256)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                } else {
                    crate::c::bf_write(
                        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        (2u16) as i32,
                    );
                    if (((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32)
                        == 0i32)
                        && ((((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0)
                    {
                        SetMovingMonPriority(2u8);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoCursorNewPosUpdate() {
    unsafe {
        ((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).write(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3284))
                .read()) as i8),
        );
        ((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).write(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3285))
                .read()) as i8),
        );
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8703))
            .read()) as i32)
                == 0i32)
                && (!((((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0))
            {
                StartSpriteAnim(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read(),
                    0u8,
                );
            }
        } else {
            if !((IsMovingItem()) != 0) {
                StartSpriteAnim(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read(),
                    0u8,
                );
            }
        }
        TryRefreshDisplayMon();
        'l1: {
            let __sw1 = ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32);
            if __sw1 == 3i32 {
                SetMovingMonPriority(1u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                AnimateBoxScrollArrows(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3256)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(67))
                .write(13u8);
                SetMovingMonPriority(1u8);
                break 'l1;
            }
            if __sw1 == 0i32 {
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8703))
                .read()) as i32)
                    == 0i32
                {
                    crate::c::bf_write(
                        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3256)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        (2u16) as i32,
                    );
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3256)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(67))
                    .write(21u8);
                    crate::c::bf_write(
                        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3256)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    SetMovingMonPriority(2u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetCursorInParty() {
    unsafe {
        let mut partyCount: u8 = 0u8;
        if !((((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0) {
            partyCount = 0u8;
        } else {
            partyCount = CalculatePlayerPartyCount();
            if ((partyCount) as i32) >= 6i32 {
                partyCount = 5u8;
            }
        }
        if (crate::c::bf_read(
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(63),
            1,
            1,
            false,
        ) as u16)
            != 0
        {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3287))
                .write(1u8);
        }
        SetCursorPosition(1u8, partyCount);
    }
}
pub(crate) unsafe extern "C" fn SetCursorBoxPosition(cursorBoxPosition: u8) {
    unsafe {
        let mut cursorBoxPosition = cursorBoxPosition;
        SetCursorPosition(0u8, cursorBoxPosition);
    }
}
pub(crate) unsafe extern "C" fn ClearSavedCursorPos() {
    unsafe {
        ((&raw mut sSavedCursorPosition).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SaveCursorPos() {
    unsafe {
        ((&raw mut sSavedCursorPosition).cast::<u8>().cast::<u8>())
            .write(((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8));
    }
}
pub(crate) unsafe extern "C" fn GetSavedCursorPos() -> u8 {
    unsafe {
        return ((&raw mut sSavedCursorPosition).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn InitMonPlaceChange(r#type: u8) {
    unsafe {
        let mut r#type = r#type;
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3468)
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(
            ((((&raw const placeChangeFuncs_10)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .wrapping_offset(((r#type) as i32) as isize))
            .read(),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3472))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn InitMultiMonPlaceChange(up: u8) {
    unsafe {
        let mut up = up;
        if !((up) != 0) {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3468)
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(MultiMonPlaceChange_Down));
        } else {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3468)
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(MultiMonPlaceChange_Up));
        }
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3472))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn DoMonPlaceChange() -> u8 {
    unsafe {
        return (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3468)
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn MonPlaceChange_Grab() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3472))
            .read()) as i32);
            if __sw1 == 0i32 {
                if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
                    return 0u8;
                }
                StartSpriteAnim(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read(),
                    2u8,
                );
                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3472);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((MonPlaceChange_CursorDown()) != 0) {
                    StartSpriteAnim(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read(),
                        3u8,
                    );
                    MoveMon();
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3472);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((MonPlaceChange_CursorUp()) != 0) {
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3472);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MonPlaceChange_Place() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3472))
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((MonPlaceChange_CursorDown()) != 0) {
                    StartSpriteAnim(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read(),
                        2u8,
                    );
                    PlaceMon();
                    let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3472);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((MonPlaceChange_CursorUp()) != 0) {
                    StartSpriteAnim(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read(),
                        0u8,
                    );
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3472);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MonPlaceChange_Shift() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3472))
            .read()) as i32);
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 =
                        ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32);
                    let __matched = __sw2 == 1i32 || __sw2 == 0i32;
                    if __sw2 == 1i32 {
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3473))
                        .write(14u8);
                        break 'l2;
                    }
                    if __sw2 == 0i32 {
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3473))
                        .write(StorageGetCurrentBox());
                        break 'l2;
                    }
                    if !__matched {
                        return 0u8;
                    }
                }
                StartSpriteAnim(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read(),
                    2u8,
                );
                SaveMonSpriteAtPos(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3473))
                    .read(),
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                );
                let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3472);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((MoveShiftingMons()) != 0) {
                    StartSpriteAnim(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3252)
                            .cast::<*mut u8>())
                        .read(),
                        3u8,
                    );
                    SetShiftedMonData(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3473))
                        .read(),
                        ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                    );
                    let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3472);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MultiMonPlaceChange_Down() -> u8 {
    unsafe {
        return MonPlaceChange_CursorDown();
    }
}
pub(crate) unsafe extern "C" fn MultiMonPlaceChange_Up() -> u8 {
    unsafe {
        return MonPlaceChange_CursorUp();
    }
}
pub(crate) unsafe extern "C" fn MonPlaceChange_CursorDown() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(38)
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 8i32;
            if !__matched {
                let __p2 = (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(38)
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 0i32 {
                let __p3 = (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(38)
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MonPlaceChange_CursorUp() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(38)
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32;
            if __sw1 == 0i32 {
                return 0u8;
            }
            if !__matched {
                let __p2 = (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(38)
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_sub(1));
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MoveMon() {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 0i32;
            if __sw1 == 1i32 {
                SetMovingMonData(
                    14u8,
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                );
                SetMovingMonSprite(
                    0u8,
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 0i32 {
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8703))
                .read()) as i32)
                    == 0i32
                {
                    SetMovingMonData(
                        StorageGetCurrentBox(),
                        ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                    );
                    SetMovingMonSprite(
                        1u8,
                        ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                    );
                }
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        ((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn PlaceMon() {
    unsafe {
        let mut boxId: u8 = 0u8;
        'l1: {
            let __sw1 = ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 0i32;
            if __sw1 == 1i32 {
                SetPlacedMonData(
                    14u8,
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                );
                SetPlacedMonSprite(
                    14u8,
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 0i32 {
                boxId = StorageGetCurrentBox();
                SetPlacedMonData(
                    boxId,
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                );
                SetPlacedMonSprite(
                    boxId,
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                );
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        ((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn RefreshDisplayMon() {
    unsafe {
        TryRefreshDisplayMon();
    }
}
pub(crate) unsafe extern "C" fn SetMovingMonData(boxId: u8, position: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut position = position;
        if ((boxId) as i32) == 14i32 {
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8356)
                .cast::<crate::c::Rec4<100>>()
                .write_unaligned(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                                as isize
                                * 100,
                        )
                        .cast::<crate::c::Rec4<100>>()
                        .read_unaligned(),
                );
        } else {
            BoxMonAtToMon(
                boxId,
                position,
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8356),
            );
        }
        PurgeMonOrBoxMon(boxId, position);
        ((&raw mut sMovingMonOrigBoxId).cast::<u8>().cast::<u8>()).write(boxId);
        ((&raw mut sMovingMonOrigBoxPos).cast::<u8>().cast::<u8>()).write(position);
    }
}
pub(crate) unsafe extern "C" fn SetPlacedMonData(boxId: u8, position: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut position = position;
        if ((boxId) as i32) == 14i32 {
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((position) as i32) as isize * 100)
                .cast::<crate::c::Rec4<100>>()
                .write_unaligned(
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8356)
                        .cast::<crate::c::Rec4<100>>()
                        .read_unaligned(),
                );
        } else {
            BoxMonRestorePP(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8356)),
            );
            SetBoxMonAt(
                boxId,
                position,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8356)),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PurgeMonOrBoxMon(boxId: u8, position: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut position = position;
        if ((boxId) as i32) == 14i32 {
            ZeroMonData(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((position) as i32) as isize * 100),
            );
        } else {
            ZeroBoxMonAt(boxId, position);
        }
    }
}
pub(crate) unsafe extern "C" fn SetShiftedMonData(boxId: u8, position: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut position = position;
        if ((boxId) as i32) == 14i32 {
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8456)
                .cast::<crate::c::Rec4<100>>()
                .write_unaligned(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((position) as i32) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .read_unaligned(),
                );
        } else {
            BoxMonAtToMon(
                boxId,
                position,
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8456),
            );
        }
        SetPlacedMonData(boxId, position);
        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8356)
            .cast::<crate::c::Rec4<100>>()
            .write_unaligned(
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8456)
                    .cast::<crate::c::Rec4<100>>()
                    .read_unaligned(),
            );
        SetDisplayMonData(
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8356),
            0u8,
        );
        ((&raw mut sMovingMonOrigBoxId).cast::<u8>().cast::<u8>()).write(boxId);
        ((&raw mut sMovingMonOrigBoxPos).cast::<u8>().cast::<u8>()).write(position);
    }
}
pub(crate) unsafe extern "C" fn TryStorePartyMonInBox(boxId: u8) -> u8 {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition: i16 = GetFirstFreeBoxSpot(boxId);
        if ((boxPosition) as i32) == (-1i32) {
            return 0u8;
        }
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            SetPlacedMonData(boxId, ((boxPosition) as u8));
            DestroyMovingMonIcon();
            ((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).write(0u8);
        } else {
            SetMovingMonData(
                14u8,
                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
            );
            SetPlacedMonData(boxId, ((boxPosition) as u8));
            DestroyPartyMonIcon(
                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
            );
        }
        if ((boxId) as i32) == ((StorageGetCurrentBox()) as i32) {
            CreateBoxMonIconAtPos(((boxPosition) as u8));
        }
        StartSpriteAnim(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read(),
            1u8,
        );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ResetSelectionAfterDeposit() {
    unsafe {
        StartSpriteAnim(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read(),
            0u8,
        );
        TryRefreshDisplayMon();
    }
}
pub(crate) unsafe extern "C" fn InitReleaseMon() {
    unsafe {
        let mut mode: u8 = 0u8;
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            mode = 2u8;
        } else {
            if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32 {
                mode = 0u8;
            } else {
                mode = 1u8;
            }
        }
        SetReleaseMon(
            mode,
            ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
        );
        StringCopy(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8672))
                .cast::<u8>(),
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3310))
                .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn TryHideReleaseMon() -> u8 {
    unsafe {
        if !((TryHideReleaseMonSprite()) != 0) {
            StartSpriteAnim(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read(),
                0u8,
            );
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ReleaseMon() {
    unsafe {
        let mut boxId: u8 = 0u8;
        DestroyReleaseMonIcon();
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            ((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).write(0u8);
        } else {
            if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32 {
                boxId = 14u8;
            } else {
                boxId = StorageGetCurrentBox();
            }
            PurgeMonOrBoxMon(
                boxId,
                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
            );
        }
        TryRefreshDisplayMon();
    }
}
pub(crate) unsafe extern "C" fn TrySetCursorFistAnim() {
    unsafe {
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            StartSpriteAnim(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read(),
                3u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GetRestrictedReleaseMoves(moves: *mut u16) {
    unsafe {
        let mut moves = moves;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(24u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw const sRestrictedReleaseMoves).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 4))
                    .cast::<i8>())
                    .read()) as i32)
                        == 34i32)
                        || (((((((((&raw const sRestrictedReleaseMoves).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .cast::<i8>())
                        .read()) as i32)
                            == (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<i8>())
                            .read()) as i32))
                            && ((((((((&raw const sRestrictedReleaseMoves)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(1)
                            .cast::<i8>())
                            .read()) as i32)
                                == (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .wrapping_add(1)
                                .cast::<i8>())
                                .read()) as i32)))
                    {
                        (moves).write(
                            (((((&raw const sRestrictedReleaseMoves).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read(),
                        );
                        moves = (moves).wrapping_offset(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (moves).write(355u16);
    }
}
pub(crate) unsafe extern "C" fn InitCanReleaseMonVars() {
    unsafe {
        if !((AtLeastThreeUsableMons()) != 0) {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8557))
                .write(1u8);
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8556)
                .cast::<i8>())
            .write(0i8);
            return;
        }
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8456)
                .cast::<crate::c::Rec4<100>>()
                .write_unaligned(
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8356)
                        .cast::<crate::c::Rec4<100>>()
                        .read_unaligned(),
                );
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8560)
                .cast::<i8>())
            .write((-1i8));
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8561)
                .cast::<i8>())
            .write((-1i8));
        } else {
            if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32 {
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8456)
                    .cast::<crate::c::Rec4<100>>()
                    .write_unaligned(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                    as i32) as isize
                                    * 100,
                            )
                            .cast::<crate::c::Rec4<100>>()
                            .read_unaligned(),
                    );
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8560)
                    .cast::<i8>())
                .write(14i8);
            } else {
                BoxMonAtToMon(
                    StorageGetCurrentBox(),
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8456),
                );
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8560)
                    .cast::<i8>())
                .write(((StorageGetCurrentBox()) as i8));
            }
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8561)
                .cast::<i8>())
            .write(((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read());
        }
        GetRestrictedReleaseMoves(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8566))
                .cast::<u16>(),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8564)
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8456),
                81i32,
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8566))
                .cast::<u16>())
                .cast::<u8>(),
            )) as u16),
        );
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8564)
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8557))
                .write(0u8);
        } else {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8557))
                .write(1u8);
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8556)
                .cast::<i8>())
            .write(1i8);
        }
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8562)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn AtLeastThreeUsableMons() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut count: i32 = ((((((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read())
            as i32)
            != 0i32) as i32);
        {
            j = 0i32;
            'l1: loop {
                if !(j < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((j) as isize * 100),
                        5i32,
                    )) != 0
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        if count >= 3i32 {
            return 1u32;
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 14i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 30i32) {
                                break 'l5;
                            }
                            'l6: {
                                if (CheckBoxMonSanityAt(((i) as u32), ((j) as u32))) != 0 {
                                    if {
                                        let __t1 = (count).wrapping_add(1);
                                        count = __t1;
                                        __t1
                                    } >= 3i32
                                    {
                                        return 1u32;
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn RunCanReleaseMon() -> i8 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut knownMoves: u16 = 0u16;
        if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8557))
            .read())
            != 0
        {
            return ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8556)
                .cast::<i8>())
            .read();
        }
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8562)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8560)
                                .cast::<i8>())
                            .read()) as i32)
                                != 14i32)
                                || (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8561)
                                .cast::<i8>())
                                .read()) as i32)
                                    != ((i) as i32))
                            {
                                knownMoves = ((GetMonData3(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 100),
                                    81i32,
                                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8566))
                                    .cast::<u16>())
                                    .cast::<u8>(),
                                )) as u16);
                                let __p2 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8564)
                                .cast::<u16>();
                                (__p2).write(
                                    (((((__p2).read()) as i32) & !((knownMoves) as i32)) as u16),
                                );
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8564)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8557))
                    .write(1u8);
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8556)
                        .cast::<i8>())
                    .write(1i8);
                } else {
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8558)
                        .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8559)
                        .cast::<i8>())
                    .write(0i8);
                    let __p3 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8562)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0u16;
                    'l4: loop {
                        if !(((i) as i32) < 30i32) {
                            break 'l4;
                        }
                        'l5: {
                            knownMoves = ((GetAndCopyBoxMonDataAt(
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8558)
                                    .cast::<i8>())
                                .read()) as u8),
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8559)
                                    .cast::<i8>())
                                .read()) as u8),
                                81i32,
                                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8566))
                                .cast::<u16>())
                                .cast::<u8>(),
                            )) as u16);
                            if (((knownMoves) as i32) != 0i32)
                                && (!((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8560)
                                .cast::<i8>())
                                .read()) as i32)
                                    == ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8558)
                                    .cast::<i8>())
                                    .read()) as i32))
                                    && (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8561)
                                    .cast::<i8>())
                                    .read()) as i32)
                                        == ((((((&raw mut sStorage)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(8559)
                                        .cast::<i8>())
                                        .read())
                                            as i32))))
                            {
                                let __p4 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8564)
                                .cast::<u16>();
                                (__p4).write(
                                    (((((__p4).read()) as i32) & !((knownMoves) as i32)) as u16),
                                );
                                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8564)
                                .cast::<u16>())
                                .read()) as i32)
                                    == 0i32
                                {
                                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(8557))
                                    .write(1u8);
                                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(8556)
                                        .cast::<i8>())
                                    .write(1i8);
                                    break 'l4;
                                }
                            }
                            if (({
                                let __p5 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8559)
                                .cast::<i8>();
                                let __t6 = ((__p5).read()).wrapping_add(1);
                                (__p5).write(__t6);
                                __t6
                            }) as i32)
                                >= 30i32
                            {
                                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8559)
                                    .cast::<i8>())
                                .write(0i8);
                                if (({
                                    let __p7 =
                                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(8558)
                                        .cast::<i8>();
                                    let __t8 = ((__p7).read()).wrapping_add(1);
                                    (__p7).write(__t8);
                                    __t8
                                }) as i32)
                                    >= 14i32
                                {
                                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(8557))
                                    .write(1u8);
                                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(8556)
                                        .cast::<i8>())
                                    .write(0i8);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
        return (-1i8);
    }
}
pub(crate) unsafe extern "C" fn SaveMovingMon() {
    unsafe {
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            (&raw mut sSavedMovingMon)
                .cast::<u8>()
                .cast::<crate::c::Rec4<100>>()
                .write_unaligned(
                    (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8356)
                        .cast::<crate::c::Rec4<100>>()
                        .read_unaligned(),
                );
        }
    }
}
pub(crate) unsafe extern "C" fn LoadSavedMovingMon() {
    unsafe {
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            if ((((&raw mut sMovingMonOrigBoxId).cast::<u8>().cast::<u8>()).read()) as i32) == 14i32
            {
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8356)
                    .cast::<crate::c::Rec4<100>>()
                    .write_unaligned(
                        (&raw mut sSavedMovingMon)
                            .cast::<u8>()
                            .cast::<crate::c::Rec4<100>>()
                            .read_unaligned(),
                    );
            } else {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8356))
                    .cast::<crate::c::Rec4<80>>()
                    .write_unaligned(
                        ((&raw mut sSavedMovingMon).cast::<u8>())
                            .cast::<crate::c::Rec4<80>>()
                            .read_unaligned(),
                    );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitSummaryScreenData() {
    unsafe {
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            SaveMovingMon();
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8588))
                .cast::<*mut u8>())
            .write((&raw mut sSavedMovingMon).cast::<u8>());
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8583))
                .write(0u8);
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8582))
                .write(0u8);
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8584))
                .write(0u8);
        } else {
            if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32 {
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8588))
                .cast::<*mut u8>())
                .write((&raw mut gPlayerParty).cast::<u8>());
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8583))
                    .write(((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8));
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8582))
                    .write(((((CountPartyMons()) as i32).wrapping_sub(1i32)) as u8));
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8584))
                    .write(0u8);
            } else {
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8588))
                .cast::<*mut u8>())
                .write(GetBoxedMonPtr(StorageGetCurrentBox(), 0u8));
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8583))
                    .write(((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8));
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8582))
                    .write(29u8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8584))
                    .write(2u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetSelectionAfterSummaryScreen() {
    unsafe {
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            LoadSavedMovingMon();
        } else {
            ((&raw mut sCursorPosition).cast::<u8>().cast::<i8>())
                .write(((((&raw mut gLastViewedMonIndex).cast::<u8>()).read()) as i8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CompactPartySlots() -> i16 {
    unsafe {
        let mut retVal: i16 = (-1i16);
        let mut i: u16 = 0u16;
        let mut last: u16 = 0u16;
        {
            i = 0u16;
            last = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut species: u16 = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        11i32,
                    )) as u16);
                    if ((species) as i32) != 0i32 {
                        if ((i) as i32) != ((last) as i32) {
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((last) as i32) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .write_unaligned(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 100)
                                        .cast::<crate::c::Rec4<100>>()
                                        .read_unaligned(),
                                );
                        }
                        last = (last).wrapping_add(1);
                    } else {
                        if ((retVal) as i32) == (-1i32) {
                            retVal = ((i) as i16);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            'l3: loop {
                if !(((last) as i32) < 6i32) {
                    break 'l3;
                }
                'l4: {
                    ZeroMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((last) as i32) as isize * 100),
                    );
                }
                last = (last).wrapping_add(1);
            }
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn SetMonMarkings(markings: u8) {
    unsafe {
        let mut markings = markings;
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3307))
            .write(markings);
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            SetMonData(
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8356),
                8i32,
                &raw mut markings,
            );
        } else {
            if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 100,
                    ),
                    8i32,
                    &raw mut markings,
                );
            }
            if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 0i32 {
                SetCurrentBoxMonData(
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                    8i32,
                    &raw mut markings,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsRemovingLastPartyMon() -> u8 {
    unsafe {
        if ((((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32)
            && (!((((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0)))
            && (((CountPartyAliveNonEggMonsExcept(
                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
            )) as i32)
                == 0i32)
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
pub(crate) unsafe extern "C" fn CanShiftMon() -> u8 {
    unsafe {
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            if (((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32)
                && (((CountPartyAliveNonEggMonsExcept(
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                )) as i32)
                    == 0i32)
            {
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3309))
                .read())
                    != 0)
                    || (GetMonData2(
                        (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8356),
                        57i32,
                    ) == 0u32)
                {
                    return 0u8;
                }
            }
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsMonBeingMoved() -> u8 {
    unsafe {
        return ((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn IsCursorOnBoxTitle() -> u8 {
    unsafe {
        return ((((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 2i32)
            as u8);
    }
}
pub(crate) unsafe extern "C" fn IsCursorOnCloseBox() -> u8 {
    unsafe {
        return (((((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 3i32)
            && (((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32))
            as u8);
    }
}
pub(crate) unsafe extern "C" fn IsCursorInBox() -> u8 {
    unsafe {
        return ((((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 0i32)
            as u8);
    }
}
pub(crate) unsafe extern "C" fn TryRefreshDisplayMon() {
    unsafe {
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3306)).write(
            ((((((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32)
                as u8),
        );
        if !((((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0) {
            'l1: {
                let __sw1 = ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32);
                let mut __fall = false;
                if __sw1 == 1i32 {
                    __fall = true;
                    if ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                        < 6i32
                    {
                        SetDisplayMonData(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                    as i32) as isize
                                    * 100,
                            ),
                            0u8,
                        );
                        break 'l1;
                    }
                }
                if __fall || __sw1 == 3i32 || __sw1 == 2i32 {
                    __fall = true;
                    SetDisplayMonData(core::ptr::null_mut(), 2u8);
                    break 'l1;
                }
                if __sw1 == 0i32 {
                    __fall = true;
                    SetDisplayMonData(
                        GetBoxedMonPtr(
                            StorageGetCurrentBox(),
                            ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
                        ),
                        1u8,
                    );
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReshowDisplayMon() {
    unsafe {
        if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
            SetDisplayMonData((&raw mut sSavedMovingMon).cast::<u8>(), 0u8);
        } else {
            TryRefreshDisplayMon();
        }
    }
}
pub(crate) unsafe extern "C" fn SetDisplayMonData(pokemon: *mut u8, mode: u8) {
    unsafe {
        let mut pokemon = pokemon;
        let mut mode = mode;
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut gender: u16 = 0u16;
        let mut sanityIsBadEgg: u8 = 0u8;
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3302)
            .cast::<u16>())
        .write(0u16);
        gender = 0u16;
        sanityIsBadEgg = 0u8;
        if ((mode) as i32) == 0i32 {
            let mut mon: *mut u8 = pokemon;
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3300)
                .cast::<u16>())
            .write(((GetMonData2(mon, 65i32)) as u16));
            if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3300)
                .cast::<u16>())
            .read()) as i32)
                != 0i32
            {
                sanityIsBadEgg = ((GetMonData2(mon, 4i32)) as u8);
                if (sanityIsBadEgg) != 0 {
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3309))
                    .write(1u8);
                } else {
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3309))
                    .write(((GetMonData2(mon, 45i32)) as u8));
                }
                GetMonData3(
                    mon,
                    2i32,
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3310))
                    .cast::<u8>(),
                );
                StringGet_Nickname(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3310))
                    .cast::<u8>(),
                );
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3308))
                    .write(((GetMonData2(mon, 56i32)) as u8));
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3307))
                    .write(((GetMonData2(mon, 8i32)) as u8));
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3296)
                    .cast::<u32>())
                .write(GetMonData2(mon, 0i32));
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3292)
                    .cast::<*mut u32>())
                .write(GetMonFrontSpritePal(mon));
                gender = ((GetMonGender(mon)) as u16);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3302)
                    .cast::<u16>())
                .write(((GetMonData2(mon, 12i32)) as u16));
            }
        } else {
            if ((mode) as i32) == 1i32 {
                let mut boxMon: *mut u8 = pokemon;
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3300)
                    .cast::<u16>())
                .write(((GetBoxMonData2(pokemon, 65i32)) as u16));
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3300)
                    .cast::<u16>())
                .read()) as i32)
                    != 0i32
                {
                    let mut otId: u32 = GetBoxMonData2(boxMon, 1i32);
                    sanityIsBadEgg = ((GetBoxMonData2(boxMon, 4i32)) as u8);
                    if (sanityIsBadEgg) != 0 {
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3309))
                        .write(1u8);
                    } else {
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3309))
                        .write(((GetBoxMonData2(boxMon, 45i32)) as u8));
                    }
                    GetBoxMonData3(
                        boxMon,
                        2i32,
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3310))
                        .cast::<u8>(),
                    );
                    StringGet_Nickname(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3310))
                        .cast::<u8>(),
                    );
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3308))
                    .write(GetLevelFromBoxMonExp(boxMon));
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3307))
                    .write(((GetBoxMonData2(boxMon, 8i32)) as u8));
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3296)
                        .cast::<u32>())
                    .write(GetBoxMonData2(boxMon, 0i32));
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3292)
                        .cast::<*mut u32>())
                    .write(GetMonSpritePalFromSpeciesAndPersonality(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3300)
                            .cast::<u16>())
                        .read(),
                        otId,
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3296)
                            .cast::<u32>())
                        .read(),
                    ));
                    gender = ((GetGenderFromSpeciesAndPersonality(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3300)
                            .cast::<u16>())
                        .read(),
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3296)
                            .cast::<u32>())
                        .read(),
                    )) as u16);
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3302)
                        .cast::<u16>())
                    .write(((GetBoxMonData2(boxMon, 12i32)) as u16));
                }
            } else {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3300)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3302)
                    .cast::<u16>())
                .write(0u16);
            }
        }
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3300)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            StringFill(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3310))
                    .cast::<u8>(),
                0u8,
                5u16,
            );
            StringFill(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3321))
                    .cast::<u8>(),
                0u8,
                8u16,
            );
            StringFill(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3357))
                    .cast::<u8>(),
                0u8,
                8u16,
            );
            StringFill(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3393))
                    .cast::<u8>(),
                0u8,
                8u16,
            );
            StringFill(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3429))
                    .cast::<u8>(),
                0u8,
                8u16,
            );
        } else {
            if (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3309))
                .read())
                != 0
            {
                if (sanityIsBadEgg) != 0 {
                    StringCopyPadded(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3321))
                        .cast::<u8>(),
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3310))
                        .cast::<u8>(),
                        0u8,
                        5u16,
                    );
                } else {
                    StringCopyPadded(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3321))
                        .cast::<u8>(),
                        (&raw mut gText_EggNickname).cast::<u8>(),
                        0u8,
                        8u16,
                    );
                }
                StringFill(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3357))
                    .cast::<u8>(),
                    0u8,
                    8u16,
                );
                StringFill(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3393))
                    .cast::<u8>(),
                    0u8,
                    8u16,
                );
                StringFill(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3429))
                    .cast::<u8>(),
                    0u8,
                    8u16,
                );
            } else {
                if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3300)
                    .cast::<u16>())
                .read()) as i32)
                    == 29i32)
                    || (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3300)
                        .cast::<u16>())
                    .read()) as i32)
                        == 32i32)
                {
                    gender = 255u16;
                }
                StringCopyPadded(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3321))
                    .cast::<u8>(),
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3310))
                    .cast::<u8>(),
                    0u8,
                    5u16,
                );
                txtPtr = ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3357))
                .cast::<u8>();
                ({
                    let __t1 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t1
                })
                .write(186u8);
                StringCopyPadded(
                    txtPtr,
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3300)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                    0u8,
                    5u16,
                );
                txtPtr = ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3393))
                .cast::<u8>();
                ({
                    let __t2 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t2
                })
                .write(252u8);
                ({
                    let __t3 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t3
                })
                .write(4u8);
                'l1: {
                    let __sw4 = ((gender) as i32);
                    let __matched = __sw4 == 0i32 || __sw4 == 254i32;
                    if __sw4 == 0i32 {
                        ({
                            let __t5 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t5
                        })
                        .write(4u8);
                        ({
                            let __t6 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t6
                        })
                        .write(1u8);
                        ({
                            let __t7 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t7
                        })
                        .write(5u8);
                        ({
                            let __t8 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t8
                        })
                        .write(181u8);
                        break 'l1;
                    }
                    if __sw4 == 254i32 {
                        ({
                            let __t9 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t9
                        })
                        .write(6u8);
                        ({
                            let __t10 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t10
                        })
                        .write(1u8);
                        ({
                            let __t11 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t11
                        })
                        .write(7u8);
                        ({
                            let __t12 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t12
                        })
                        .write(182u8);
                        break 'l1;
                    }
                    if !__matched {
                        ({
                            let __t13 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t13
                        })
                        .write(2u8);
                        ({
                            let __t14 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t14
                        })
                        .write(1u8);
                        ({
                            let __t15 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t15
                        })
                        .write(3u8);
                        ({
                            let __t16 = txtPtr;
                            txtPtr = (txtPtr).wrapping_offset(1);
                            __t16
                        })
                        .write(119u8);
                        break 'l1;
                    }
                }
                ({
                    let __t17 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t17
                })
                .write(252u8);
                ({
                    let __t18 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t18
                })
                .write(4u8);
                ({
                    let __t19 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t19
                })
                .write(2u8);
                ({
                    let __t20 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t20
                })
                .write(1u8);
                ({
                    let __t21 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t21
                })
                .write(3u8);
                ({
                    let __t22 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t22
                })
                .write(0u8);
                ({
                    let __t23 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t23
                })
                .write(249u8);
                ({
                    let __t24 = txtPtr;
                    txtPtr = (txtPtr).wrapping_offset(1);
                    __t24
                })
                .write(5u8);
                txtPtr = ConvertIntToDecimalStringN(
                    txtPtr,
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3308))
                    .read()) as i32),
                    0i32,
                    3u8,
                );
                (txtPtr).write(0u8);
                ((txtPtr).wrapping_offset(1)).write(255u8);
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3302)
                    .cast::<u16>())
                .read()) as i32)
                    != 0i32
                {
                    StringCopyPadded(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3429))
                        .cast::<u8>(),
                        GetItemName(
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3302)
                                .cast::<u16>())
                            .read(),
                        ),
                        0u8,
                        8u16,
                    );
                } else {
                    StringFill(
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3429))
                        .cast::<u8>(),
                        0u8,
                        8u16,
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleInput_InBox() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8703))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || !__matched {
                return InBoxInput_Normal();
            }
            if __sw1 == 1i32 {
                return InBoxInput_SelectingMultiple();
            }
            if __sw1 == 2i32 {
                return InBoxInput_MovingMultiple();
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn InBoxInput_Normal() -> u8 {
    unsafe {
        let mut retVal: u8 = 0u8;
        let mut cursorArea: i8 = 0i8;
        let mut cursorPosition: i8 = 0i8;
        'l1: loop {
            'l2: {
                cursorArea = ((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read();
                cursorPosition = ((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read();
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3282)
                    .cast::<i8>())
                .write(0i8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3283)
                    .cast::<i8>())
                .write(0i8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3287))
                    .write(0u8);
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    retVal = 1u8;
                    if ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                        >= 6i32
                    {
                        cursorPosition = ((((cursorPosition) as i32).wrapping_sub(6i32)) as i8);
                    } else {
                        cursorArea = 2i8;
                        cursorPosition = 0i8;
                    }
                    break 'l1;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        retVal = 1u8;
                        cursorPosition = ((((cursorPosition) as i32).wrapping_add(6i32)) as i8);
                        if ((cursorPosition) as i32) >= 30i32 {
                            cursorArea = 3i8;
                            cursorPosition =
                                ((((cursorPosition) as i32).wrapping_sub(30i32)) as i8);
                            cursorPosition =
                                ((crate::c::div_i32(((cursorPosition) as i32), 3i32)) as i8);
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3282)
                                .cast::<i8>())
                            .write(1i8);
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3287))
                            .write(1u8);
                        }
                        break 'l1;
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 32i32)
                            != 0
                        {
                            retVal = 1u8;
                            if crate::c::rem_i32(
                                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                    as i32),
                                6i32,
                            ) != 0i32
                            {
                                cursorPosition = (cursorPosition).wrapping_sub(1);
                            } else {
                                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(3283)
                                    .cast::<i8>())
                                .write((-1i8));
                                cursorPosition =
                                    ((((cursorPosition) as i32).wrapping_add(5i32)) as i8);
                            }
                            break 'l1;
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(48)
                                .cast::<u16>())
                            .read()) as i32)
                                & 16i32)
                                != 0
                            {
                                retVal = 1u8;
                                if crate::c::rem_i32(((((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32))).wrapping_add(1i32), 6i32) != 0i32 {
cursorPosition = (cursorPosition).wrapping_add(1);
} else {
((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3283).cast::<i8>()).write(1i8);
cursorPosition = ((((((cursorPosition) as i32))).wrapping_sub(5i32)) as i8);
}
                                break 'l1;
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 8i32)
                                    != 0
                                {
                                    retVal = 1u8;
                                    cursorArea = 2i8;
                                    cursorPosition = 0i8;
                                    break 'l1;
                                }
                            }
                        }
                    }
                }
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0)
                    && ((SetSelectionMenuTexts()) != 0)
                {
                    if !((((&raw mut sAutoActionOn).cast::<u8>().cast::<u8>()).read()) != 0) {
                        return 8u8;
                    }
                    if (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32)
                        != 2i32)
                        || (((((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            == 1i32)
                    {
                        'l3: {
                            let __sw1 = ((GetMenuItemTextId(0u8)) as i32);
                            if __sw1 == 1i32 {
                                return 11u8;
                            }
                            if __sw1 == 2i32 {
                                return 12u8;
                            }
                            if __sw1 == 3i32 {
                                return 13u8;
                            }
                            if __sw1 == 4i32 {
                                return 14u8;
                            }
                            if __sw1 == 5i32 {
                                return 15u8;
                            }
                            if __sw1 == 12i32 {
                                return 16u8;
                            }
                            if __sw1 == 13i32 {
                                return 17u8;
                            }
                            if __sw1 == 15i32 {
                                return 18u8;
                            }
                        }
                    } else {
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8703))
                        .write(1u8);
                        return 20u8;
                    }
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    return 19u8;
                }
                if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19))
                    .read()) as i32)
                    == 1i32
                {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(44)
                        .cast::<u16>())
                    .read()) as i32)
                        & 512i32)
                        != 0
                    {
                        return 10u8;
                    }
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(44)
                        .cast::<u16>())
                    .read()) as i32)
                        & 256i32)
                        != 0
                    {
                        return 9u8;
                    }
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 4i32)
                    != 0
                {
                    ToggleCursorAutoAction();
                    return 0u8;
                }
                retVal = 0u8;
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if (retVal) != 0 {
            SetCursorPosition(((cursorArea) as u8), ((cursorPosition) as u8));
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn InBoxInput_SelectingMultiple() -> u8 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(48)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0
            {
                if crate::c::div_i32(
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32),
                    6i32,
                ) != 0i32
                {
                    SetCursorPosition(
                        0u8,
                        ((((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                            .wrapping_sub(6i32)) as u8),
                    );
                    return 21u8;
                } else {
                    return 24u8;
                }
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0
                {
                    if ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                        .wrapping_add(6i32)
                        < 30i32
                    {
                        SetCursorPosition(
                            0u8,
                            ((((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                as i32)
                                .wrapping_add(6i32)) as u8),
                        );
                        return 21u8;
                    } else {
                        return 24u8;
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 32i32)
                        != 0
                    {
                        if crate::c::rem_i32(
                            ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                as i32),
                            6i32,
                        ) != 0i32
                        {
                            SetCursorPosition(
                                0u8,
                                ((((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                    as i32)
                                    .wrapping_sub(1i32)) as u8),
                            );
                            return 21u8;
                        } else {
                            return 24u8;
                        }
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 16i32)
                            != 0
                        {
                            if crate::c::rem_i32(
                                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                    as i32)
                                    .wrapping_add(1i32),
                                6i32,
                            ) != 0i32
                            {
                                SetCursorPosition(
                                    0u8,
                                    ((((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>())
                                        .read()) as i32)
                                        .wrapping_add(1i32))
                                        as u8),
                                );
                                return 21u8;
                            } else {
                                return 24u8;
                            }
                        } else {
                            return 0u8;
                        }
                    }
                }
            }
        } else {
            if ((MultiMove_GetOrigin()) as i32)
                == ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
            {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8703))
                    .write(0u8);
                crate::c::bf_write(
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3256)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                return 22u8;
            } else {
                ((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).write(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3300)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32) as u8),
                );
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8703))
                    .write(2u8);
                ((&raw mut sMovingMonOrigBoxId).cast::<u8>().cast::<u8>())
                    .write(StorageGetCurrentBox());
                return 23u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn InBoxInput_MovingMultiple() -> u8 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            if (MultiMove_TryMoveGroup(0u8)) != 0 {
                SetCursorPosition(
                    0u8,
                    ((((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                        .wrapping_sub(6i32)) as u8),
                );
                return 25u8;
            } else {
                return 24u8;
            }
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(48)
                .cast::<u16>())
            .read()) as i32)
                & 128i32)
                != 0
            {
                if (MultiMove_TryMoveGroup(1u8)) != 0 {
                    SetCursorPosition(
                        0u8,
                        ((((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                            .wrapping_add(6i32)) as u8),
                    );
                    return 25u8;
                } else {
                    return 24u8;
                }
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0
                {
                    if (MultiMove_TryMoveGroup(2u8)) != 0 {
                        SetCursorPosition(
                            0u8,
                            ((((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                as i32)
                                .wrapping_sub(1i32)) as u8),
                        );
                        return 25u8;
                    } else {
                        return 10u8;
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 16i32)
                        != 0
                    {
                        if (MultiMove_TryMoveGroup(3u8)) != 0 {
                            SetCursorPosition(
                                0u8,
                                ((((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                    as i32)
                                    .wrapping_add(1i32)) as u8),
                            );
                            return 25u8;
                        } else {
                            return 9u8;
                        }
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 1i32)
                            != 0
                        {
                            if (MultiMove_CanPlaceSelection()) != 0 {
                                ((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).write(0u8);
                                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8703))
                                .write(0u8);
                                return 26u8;
                            } else {
                                return 24u8;
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 2i32)
                                != 0
                            {
                                return 24u8;
                            } else {
                                if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(19))
                                .read()) as i32)
                                    == 1i32
                                {
                                    if ((((((&raw mut gMain).cast::<u8>())
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                    .read()) as i32)
                                        & 512i32)
                                        != 0
                                    {
                                        return 10u8;
                                    }
                                    if ((((((&raw mut gMain).cast::<u8>())
                                        .wrapping_add(44)
                                        .cast::<u16>())
                                    .read()) as i32)
                                        & 256i32)
                                        != 0
                                    {
                                        return 9u8;
                                    }
                                }
                                return 0u8;
                            }
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn HandleInput_InParty() -> u8 {
    unsafe {
        let mut retVal: u8 = 0u8;
        let mut gotoBox: u8 = 0u8;
        let mut cursorArea: i8 = 0i8;
        let mut cursorPosition: i8 = 0i8;
        'l1: loop {
            'l2: {
                cursorArea = ((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read();
                cursorPosition = ((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read();
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3283)
                    .cast::<i8>())
                .write(0i8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3282)
                    .cast::<i8>())
                .write(0i8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3287))
                    .write(0u8);
                gotoBox = 0u8;
                retVal = 0u8;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    if (({
                        let __t1 = (cursorPosition).wrapping_sub(1);
                        cursorPosition = __t1;
                        __t1
                    }) as i32)
                        < 0i32
                    {
                        cursorPosition = 6i8;
                    }
                    if ((cursorPosition) as i32)
                        != ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                    {
                        retVal = 1u8;
                    }
                    break 'l1;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        if (({
                            let __t2 = (cursorPosition).wrapping_add(1);
                            cursorPosition = __t2;
                            __t2
                        }) as i32)
                            > 6i32
                        {
                            cursorPosition = 0i8;
                        }
                        if ((cursorPosition) as i32)
                            != ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                as i32)
                        {
                            retVal = 1u8;
                        }
                        break 'l1;
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 32i32)
                            != 0)
                            && (((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                as i32)
                                != 0i32)
                        {
                            retVal = 1u8;
                            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3286))
                            .write(
                                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                    as u8),
                            );
                            cursorPosition = 0i8;
                            break 'l1;
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(48)
                                .cast::<u16>())
                            .read()) as i32)
                                & 16i32)
                                != 0
                            {
                                if ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read())
                                    as i32)
                                    == 0i32
                                {
                                    retVal = 1u8;
                                    cursorPosition =
                                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(3286))
                                        .read()) as i8);
                                } else {
                                    retVal = 6u8;
                                    cursorArea = 0i8;
                                    cursorPosition = 0i8;
                                }
                                break 'l1;
                            }
                        }
                    }
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    if ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                        == 6i32
                    {
                        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .read()) as i32)
                            == 1i32
                        {
                            return 4u8;
                        }
                        gotoBox = 1u8;
                    } else {
                        if (SetSelectionMenuTexts()) != 0 {
                            if !((((&raw mut sAutoActionOn).cast::<u8>().cast::<u8>()).read()) != 0)
                            {
                                return 8u8;
                            }
                            'l3: {
                                let __sw3 = ((GetMenuItemTextId(0u8)) as i32);
                                if __sw3 == 1i32 {
                                    return 11u8;
                                }
                                if __sw3 == 2i32 {
                                    return 12u8;
                                }
                                if __sw3 == 3i32 {
                                    return 13u8;
                                }
                                if __sw3 == 4i32 {
                                    return 14u8;
                                }
                                if __sw3 == 5i32 {
                                    return 15u8;
                                }
                                if __sw3 == 12i32 {
                                    return 16u8;
                                }
                                if __sw3 == 13i32 {
                                    return 17u8;
                                }
                                if __sw3 == 15i32 {
                                    return 18u8;
                                }
                            }
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
                    if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32)
                        == 1i32
                    {
                        return 19u8;
                    }
                    gotoBox = 1u8;
                }
                if (gotoBox) != 0 {
                    retVal = 6u8;
                    cursorArea = 0i8;
                    cursorPosition = 0i8;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 4i32)
                        != 0
                    {
                        ToggleCursorAutoAction();
                        return 0u8;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if ((retVal) as i32) != 0i32 {
            if ((retVal) as i32) != 6i32 {
                SetCursorPosition(((cursorArea) as u8), ((cursorPosition) as u8));
            }
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn HandleInput_OnBox() -> u8 {
    unsafe {
        let mut retVal: u8 = 0u8;
        let mut cursorArea: i8 = 0i8;
        let mut cursorPosition: i8 = 0i8;
        'l1: loop {
            'l2: {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3283)
                    .cast::<i8>())
                .write(0i8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3282)
                    .cast::<i8>())
                .write(0i8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3287))
                    .write(0u8);
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    retVal = 1u8;
                    cursorArea = 3i8;
                    cursorPosition = 0i8;
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3287))
                    .write(1u8);
                    break 'l1;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        retVal = 1u8;
                        cursorArea = 0i8;
                        cursorPosition = 2i8;
                        break 'l1;
                    }
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0
                {
                    return 10u8;
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>())
                .read()) as i32)
                    & 16i32)
                    != 0
                {
                    return 9u8;
                }
                if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19))
                    .read()) as i32)
                    == 1i32
                {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(44)
                        .cast::<u16>())
                    .read()) as i32)
                        & 512i32)
                        != 0
                    {
                        return 10u8;
                    }
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(44)
                        .cast::<u16>())
                    .read()) as i32)
                        & 256i32)
                        != 0
                    {
                        return 9u8;
                    }
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    AnimateBoxScrollArrows(0u8);
                    AddBoxOptionsMenu();
                    return 7u8;
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    return 19u8;
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 4i32)
                    != 0
                {
                    ToggleCursorAutoAction();
                    return 0u8;
                }
                retVal = 0u8;
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if ((retVal) as i32) != 0i32 {
            if ((cursorArea) as i32) != 2i32 {
                AnimateBoxScrollArrows(0u8);
            }
            SetCursorPosition(((cursorArea) as u8), ((cursorPosition) as u8));
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn HandleInput_OnButtons() -> u8 {
    unsafe {
        let mut retVal: u8 = 0u8;
        let mut cursorArea: i8 = 0i8;
        let mut cursorPosition: i8 = 0i8;
        'l1: loop {
            'l2: {
                cursorArea = ((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read();
                cursorPosition = ((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read();
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3283)
                    .cast::<i8>())
                .write(0i8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3282)
                    .cast::<i8>())
                .write(0i8);
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3287))
                    .write(0u8);
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    retVal = 1u8;
                    cursorArea = 0i8;
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3282)
                        .cast::<i8>())
                    .write((-1i8));
                    if ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32)
                        == 0i32
                    {
                        cursorPosition = 24i8;
                    } else {
                        cursorPosition = 29i8;
                    }
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3287))
                    .write(1u8);
                    break 'l1;
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 136i32)
                    != 0
                {
                    retVal = 1u8;
                    cursorArea = 2i8;
                    cursorPosition = 0i8;
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3287))
                    .write(1u8);
                    break 'l1;
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0
                {
                    retVal = 1u8;
                    if (({
                        let __t1 = (cursorPosition).wrapping_sub(1);
                        cursorPosition = __t1;
                        __t1
                    }) as i32)
                        < 0i32
                    {
                        cursorPosition = 1i8;
                    }
                    break 'l1;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 16i32)
                        != 0
                    {
                        retVal = 1u8;
                        if (({
                            let __t2 = (cursorPosition).wrapping_add(1);
                            cursorPosition = __t2;
                            __t2
                        }) as i32)
                            > 1i32
                        {
                            cursorPosition = 0i8;
                        }
                        break 'l1;
                    }
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    return ((if ((cursorPosition) as i32) == 0i32 {
                        5i32
                    } else {
                        4i32
                    }) as u8);
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    return 19u8;
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 4i32)
                    != 0
                {
                    ToggleCursorAutoAction();
                    return 0u8;
                }
                retVal = 0u8;
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if ((retVal) as i32) != 0i32 {
            SetCursorPosition(((cursorArea) as u8), ((cursorPosition) as u8));
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn HandleInput() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        'l1: loop {
            if !(core::mem::transmute::<_, usize>(
                (((((&raw const inputFuncs_9).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .read(),
            ) != 0usize)
            {
                break 'l1;
            }
            if (((((((&raw const inputFuncs_9).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((i) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<i8>())
            .read()) as i32)
                == ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32)
            {
                return ((((((&raw const inputFuncs_9).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .read())
                .unwrap_unchecked()();
            }
            i = (i).wrapping_add(1);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AddBoxOptionsMenu() {
    unsafe {
        InitMenu();
        SetMenuText(9u8);
        SetMenuText(10u8);
        SetMenuText(11u8);
        SetMenuText(0u8);
    }
}
pub(crate) unsafe extern "C" fn SetSelectionMenuTexts() -> u8 {
    unsafe {
        InitMenu();
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            return SetMenuTexts_Mon();
        } else {
            return SetMenuTexts_Item();
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SetMenuTexts_Mon() -> u8 {
    unsafe {
        let mut species: u16 = GetSpeciesAtCursorPosition();
        'l1: {
            let __sw1 = ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 0i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 1i32 {
                if ((species) as i32) != 0i32 {
                    SetMenuText(1u8);
                } else {
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                if ((species) as i32) != 0i32 {
                    SetMenuText(2u8);
                } else {
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
                    if ((species) as i32) != 0i32 {
                        SetMenuText(4u8);
                    } else {
                        SetMenuText(5u8);
                    }
                } else {
                    if ((species) as i32) != 0i32 {
                        SetMenuText(3u8);
                    } else {
                        return 0u8;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 || !__matched {
                return 0u8;
            }
        }
        SetMenuText(6u8);
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            == 2i32
        {
            if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 0i32 {
                SetMenuText(2u8);
            } else {
                SetMenuText(1u8);
            }
        }
        SetMenuText(8u8);
        SetMenuText(7u8);
        SetMenuText(0u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SetMenuTexts_Item() -> u8 {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3300)
            .cast::<u16>())
        .read()) as i32)
            == 412i32
        {
            return 0u8;
        }
        if !((IsMovingItem()) != 0) {
            if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3302)
                .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3300)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    return 0u8;
                }
                SetMenuText(14u8);
            } else {
                if !((ItemIsMail(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3302)
                        .cast::<u16>())
                    .read(),
                )) != 0)
                {
                    SetMenuText(12u8);
                    SetMenuText(16u8);
                }
                SetMenuText(17u8);
            }
        } else {
            if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3302)
                .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3300)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    return 0u8;
                }
                SetMenuText(13u8);
            } else {
                if ((ItemIsMail(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3302)
                        .cast::<u16>())
                    .read(),
                )) as i32)
                    == 1i32
                {
                    return 0u8;
                }
                SetMenuText(15u8);
            }
        }
        SetMenuText(0u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CursorShadow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(20i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateCursorSprites() {
    unsafe {
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut spriteSheets = crate::ffi::Align4([0u8; 24]);
        (&raw mut spriteSheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(((&raw const sHandCursor_Gfx).cast::<u8>().cast_mut()).cast::<u8>());
        (&raw mut spriteSheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(4)
            .cast::<u16>()
            .write(2048u16);
        (&raw mut spriteSheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(6)
            .cast::<u16>()
            .write(0u16);
        (&raw mut spriteSheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(((&raw const sHandCursorShadow_Gfx).cast::<u8>().cast_mut()).cast::<u8>());
        (&raw mut spriteSheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(4)
            .cast::<u16>()
            .write(128u16);
        (&raw mut spriteSheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(6)
            .cast::<u16>()
            .write(1u16);
        let mut spritePalettes = crate::ffi::Align4([0u8; 16]);
        (&raw mut spritePalettes)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const sHandCursor_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut spritePalettes)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(4)
            .cast::<u16>()
            .write(56007u16);
        LoadSpriteSheets((&raw mut spriteSheets).cast::<u8>());
        LoadSpritePalettes((&raw mut spritePalettes).cast::<u8>());
        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3288))
            .cast::<u8>())
        .write(IndexOfSpritePaletteTag(56010u16));
        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3288))
            .cast::<u8>())
        .wrapping_offset(1))
        .write(IndexOfSpritePaletteTag(56007u16));
        GetCursorCoordsByPos(
            ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as u8),
            ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
            &raw mut x,
            &raw mut y,
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_Cursor_8)
                .cast::<u8>()
                .cast_mut(),
            ((x) as i16),
            ((y) as i16),
            6u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5),
                4,
                4,
                ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3288))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sAutoActionOn).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as u16) as i32,
            );
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5),
                2,
                2,
                (1u16) as i32,
            );
            if (((&raw mut sIsMonBeingMoved).cast::<u8>().cast::<u8>()).read()) != 0 {
                StartSpriteAnim(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read(),
                    3u8,
                );
            }
        } else {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 1i32 {
            subpriority = 13u8;
            priority = 1u8;
        } else {
            subpriority = 21u8;
            priority = 2u8;
        }
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_CursorShadow_7)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            subpriority,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3256)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            crate::c::bf_write(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3256)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5),
                2,
                2,
                ((priority) as u16) as i32,
            );
            if (((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) != 0 {
                crate::c::bf_write(
                    (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3256)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
            }
        } else {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3256)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn ToggleCursorAutoAction() {
    unsafe {
        ((&raw mut sAutoActionOn).cast::<u8>().cast::<u8>())
            .write(((!((((&raw mut sAutoActionOn).cast::<u8>().cast::<u8>()).read()) != 0)) as u8));
        crate::c::bf_write(
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5),
            4,
            4,
            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3288))
            .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sAutoActionOn).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn GetCursorPosition() -> u8 {
    unsafe {
        return ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetCursorBoxColumnAndRow(column: *mut u8, row: *mut u8) {
    unsafe {
        let mut column = column;
        let mut row = row;
        if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 0i32 {
            (column).write(
                ((crate::c::rem_i32(
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32),
                    6i32,
                )) as u8),
            );
            (row).write(
                ((crate::c::div_i32(
                    ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as i32),
                    6i32,
                )) as u8),
            );
        } else {
            (column).write(0u8);
            (row).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn StartCursorAnim(animNum: u8) {
    unsafe {
        let mut animNum = animNum;
        StartSpriteAnim(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read(),
            animNum,
        );
    }
}
pub(crate) unsafe extern "C" fn GetMovingMonOriginalBoxId() -> u8 {
    unsafe {
        return ((&raw mut sMovingMonOrigBoxId).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn SetCursorPriorityTo1() {
    unsafe {
        crate::c::bf_write(
            (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn TryHideItemAtCursor() {
    unsafe {
        if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 0i32 {
            TryHideItemIconAtPos(
                0u8,
                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TryShowItemAtCursor() {
    unsafe {
        if ((((&raw mut sCursorArea).cast::<u8>().cast::<i8>()).read()) as i32) == 0i32 {
            TryLoadItemIconAtPos(
                0u8,
                ((((&raw mut sCursorPosition).cast::<u8>().cast::<i8>()).read()) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InitMenu() {
    unsafe {
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3244))
            .write(0u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3245))
            .write(0u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3180))
            .write(0u8);
        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3180))
            .wrapping_add(5))
        .write(15u8);
        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3180))
            .wrapping_add(6)
            .cast::<u16>())
        .write(92u16);
    }
}
pub(crate) unsafe extern "C" fn SetMenuText(textId: u8) {
    unsafe {
        let mut textId = textId;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3244))
            .read()) as u32)
            < crate::c::div_u32(56u32, 8u32)
        {
            let mut len: u8 = 0u8;
            let mut menu: *mut u8 = (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(3188))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3244))
                .read()) as i32) as isize
                    * 8,
            );
            ((menu).cast::<*mut u8>()).write(
                ((((&raw const sMenuTexts)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((textId) as i32) as isize))
                .read(),
            );
            ((menu).wrapping_add(4).cast::<i32>()).write(((textId) as i32));
            len = ((StringLength(((menu).cast::<*mut u8>()).read())) as u8);
            if ((len) as i32)
                > ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3245))
                .read()) as i32)
            {
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3245))
                    .write(len);
            }
            let __p1 =
                (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3244);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn GetMenuItemTextId(menuIdx: u8) -> i8 {
    unsafe {
        let mut menuIdx = menuIdx;
        if ((menuIdx) as i32)
            >= ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3244))
                .read()) as i32)
        {
            return (-1i8);
        } else {
            return (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3188))
            .cast::<u8>())
            .wrapping_offset(((menuIdx) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<i32>())
            .read()) as i8);
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
pub(crate) unsafe extern "C" fn AddMenu() {
    unsafe {
        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3180))
            .wrapping_add(3))
        .write(
            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3245))
                .read()) as i32)
                .wrapping_add(2i32)) as u8),
        );
        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3180))
            .wrapping_add(4))
        .write(
            (((2i32).wrapping_mul(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3244))
                .read()) as i32),
            )) as u8),
        );
        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3180))
            .wrapping_add(1))
        .write(
            (((29i32).wrapping_sub(
                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3180))
                .wrapping_add(3))
                .read()) as i32),
            )) as u8),
        );
        (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3180))
            .wrapping_add(2))
        .write(
            (((15i32).wrapping_sub(
                (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3180))
                .wrapping_add(4))
                .read()) as i32),
            )) as u8),
        );
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3248)
            .cast::<u16>())
        .write(AddWindow(
            (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3180),
        ));
        ClearWindowTilemap(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3248)
                .cast::<u16>())
            .read()) as u8),
        );
        DrawStdFrameWithCustomTileAndPalette(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3248)
                .cast::<u16>())
            .read()) as u8),
            0u8,
            11u16,
            14u8,
        );
        PrintMenuTable(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3248)
                .cast::<u16>())
            .read()) as u8),
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3244))
                .read(),
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3188))
                .cast::<u8>(),
        );
        InitMenuInUpperLeftCornerNormal(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3248)
                .cast::<u16>())
            .read()) as u8),
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3244))
                .read(),
            0u8,
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3246))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn IsMenuLoading() -> u8 {
    unsafe {
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn HandleMenuInput() -> i16 {
    unsafe {
        let mut input: i32 = (-2i32);
        'l1: loop {
            'l2: {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    input = ((Menu_GetCursorPos()) as i32);
                    break 'l1;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        PlaySE(5u16);
                        input = (-1i32);
                    }
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    PlaySE(5u16);
                    Menu_MoveCursor((-1i8));
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        PlaySE(5u16);
                        Menu_MoveCursor(1i8);
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if input != (-2i32) {
            RemoveMenu();
        }
        if input >= 0i32 {
            input = (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3188))
            .cast::<u8>())
            .wrapping_offset((input) as isize * 8))
            .wrapping_add(4)
            .cast::<i32>())
            .read();
        }
        return ((input) as i16);
    }
}
pub(crate) unsafe extern "C" fn RemoveMenu() {
    unsafe {
        ClearStdWindowAndFrameToTransparent(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3248)
                .cast::<u16>())
            .read()) as u8),
            1u8,
        );
        RemoveWindow(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3248)
                .cast::<u16>())
            .read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn MultiMove_Init() -> u8 {
    unsafe {
        ((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).write(Alloc(2420u32));
        if ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8704)
                .cast::<u16>())
            .write(AddWindow8Bit(
                (&raw const sWindowTemplate_MultiMove)
                    .cast::<u8>()
                    .cast_mut(),
            ));
            if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8704)
                .cast::<u16>())
            .read()) as i32)
                != 255i32
            {
                FillWindowPixelBuffer(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8704)
                        .cast::<u16>())
                    .read()) as u8),
                    0u8,
                );
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_Free() {
    unsafe {
        if ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            Free(((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn MultiMove_SetFunction(id: u8) {
    unsafe {
        let mut id = id;
        (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).write(id);
        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn MultiMove_RunFunction() -> u8 {
    unsafe {
        'l1: {
            let __sw1 =
                (((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                return MultiMove_Start();
            }
            if __sw1 == 1i32 {
                return MultiMove_Cancel();
            }
            if __sw1 == 2i32 {
                return MultiMove_ChangeSelection();
            }
            if __sw1 == 3i32 {
                return MultiMove_GrabSelection();
            }
            if __sw1 == 4i32 {
                return MultiMove_MoveMons();
            }
            if __sw1 == 5i32 {
                return MultiMove_PlaceMons();
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_Start() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                HideBg(0u8);
                TryLoadAllMonIconPalettesAtOffset(128u16);
                let __p2 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                GetCursorBoxColumnAndRow(
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2),
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3),
                );
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .write(
                        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2))
                        .read(),
                    );
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                    .write(
                        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3))
                        .read(),
                    );
                ChangeBgX(0u8, (-1024i32), 0u8);
                ChangeBgY(0u8, (-1024i32), 0u8);
                FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillWindowPixelBuffer8Bit(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8704)
                        .cast::<u16>())
                    .read()) as u8),
                    0u8,
                );
                MultiMove_SetIconToBg(
                    ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .read(),
                    ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .read(),
                );
                SetBgAttribute(0u8, 4u8, 1u8);
                PutWindowTilemap(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8704)
                        .cast::<u16>())
                    .read()) as u8),
                );
                CopyWindowToVram8Bit(
                    ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8704)
                        .cast::<u16>())
                    .read()) as u8),
                    3u8,
                );
                BlendPalettes(16128u32, 8u8, 32767u16);
                StartCursorAnim(2u8);
                SetGpuRegBits(8u8, 128u16);
                let __p3 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ShowBg(0u8);
                    return 0u8;
                }
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_Cancel() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                HideBg(0u8);
                let __p2 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                MultiMove_ResetBg();
                StartCursorAnim(0u8);
                let __p3 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetCursorPriorityTo1();
                    LoadPalette((GetTextWindowPalette(3u8)).cast::<u8>(), 208u16, 32u16);
                    ShowBg(0u8);
                    return 0u8;
                }
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_ChangeSelection() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((UpdateCursorPos()) != 0) {
                    GetCursorBoxColumnAndRow(
                        (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6),
                        (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(7),
                    );
                    MultiMove_UpdateSelectedIcons();
                    ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .write(
                        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6))
                        .read(),
                    );
                    ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .write(
                        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(7))
                        .read(),
                    );
                    CopyWindowToVram8Bit(
                        ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8704)
                            .cast::<u16>())
                        .read()) as u8),
                        2u8,
                    );
                    let __p2 = (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_GrabSelection() -> u8 {
    unsafe {
        let mut movingBg: u8 = 0u8;
        let mut movingMon: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                MultiMove_GetMonsFromSelection();
                MultiMove_RemoveMonsFromBox();
                InitMultiMonPlaceChange(0u8);
                let __p2 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((DoMonPlaceChange()) != 0) {
                    StartCursorAnim(3u8);
                    MultiMove_InitMove(0u16, 256u16, 8u16);
                    InitMultiMonPlaceChange(1u8);
                    let __p3 = (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                movingBg = MultiMove_UpdateMove();
                movingMon = DoMonPlaceChange();
                if (!((movingBg) != 0)) && (!((movingMon) != 0)) {
                    return 0u8;
                }
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_MoveMons() -> u8 {
    unsafe {
        let mut movingCursor: u8 = UpdateCursorPos();
        let mut movingBg: u8 = MultiMove_UpdateMove();
        if (!((movingCursor) != 0)) && (!((movingBg) != 0)) {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn MultiMove_PlaceMons() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                MultiMove_SetPlacedMonData();
                MultiMove_InitMove(0u16, 65280u16, 8u16);
                InitMultiMonPlaceChange(0u8);
                let __p2 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (!((DoMonPlaceChange()) != 0)) && (!((MultiMove_UpdateMove()) != 0)) {
                    MultiMove_CreatePlacedMonIcons();
                    StartCursorAnim(2u8);
                    InitMultiMonPlaceChange(1u8);
                    HideBg(0u8);
                    let __p3 = (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((DoMonPlaceChange()) != 0) {
                    StartCursorAnim(0u8);
                    MultiMove_ResetBg();
                    let __p4 = (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    LoadPalette((GetTextWindowPalette(3u8)).cast::<u8>(), 208u16, 32u16);
                    SetCursorPriorityTo1();
                    ShowBg(0u8);
                    return 0u8;
                }
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_TryMoveGroup(dir: u8) -> u8 {
    unsafe {
        let mut dir = dir;
        'l1: {
            let __sw1 = ((dir) as i32);
            if __sw1 == 0i32 {
                if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(9))
                .read()) as i32)
                    == 0i32
                {
                    return 0u8;
                }
                let __p2 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9);
                (__p2).write(((__p2).read()).wrapping_sub(1));
                MultiMove_InitMove(0u16, 1024u16, 6u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(9))
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(11))
                        .read()) as i32),
                    )
                    >= 5i32
                {
                    return 0u8;
                }
                let __p3 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9);
                (__p3).write(((__p3).read()).wrapping_add(1));
                MultiMove_InitMove(0u16, 64512u16, 6u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8))
                .read()) as i32)
                    == 0i32
                {
                    return 0u8;
                }
                let __p4 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8);
                (__p4).write(((__p4).read()).wrapping_sub(1));
                MultiMove_InitMove(1024u16, 0u16, 6u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8))
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .read()) as i32),
                    )
                    >= 6i32
                {
                    return 0u8;
                }
                let __p5 =
                    (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8);
                (__p5).write(((__p5).read()).wrapping_add(1));
                MultiMove_InitMove(64512u16, 0u16, 6u16);
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_UpdateSelectedIcons() {
    unsafe {
        let mut columnChange: i16 = (((if ((((((&raw mut sMultiMove)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(2))
        .read()) as i32)
            .wrapping_sub(
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                    .read()) as i32),
            )
            < 0i32
        {
            (((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6))
                    .read()) as i32),
                ))
            .wrapping_neg()
        } else {
            ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6))
                    .read()) as i32),
                )
        })
        .wrapping_sub(
            (if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read()) as i32),
                )
                < 0i32
            {
                (((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32),
                    ))
                .wrapping_neg()
            } else {
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32),
                    )
            }),
        )) as i16);
        let mut rowChange: i16 = (((if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(3))
        .read()) as i32)
            .wrapping_sub(
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
                    .read()) as i32),
            )
            < 0i32
        {
            (((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .read()) as i32),
                ))
            .wrapping_neg()
        } else {
            ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .read()) as i32),
                )
        })
        .wrapping_sub(
            (if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .read()) as i32),
                )
                < 0i32
            {
                (((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(5))
                        .read()) as i32),
                    ))
                .wrapping_neg()
            } else {
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                    .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(5))
                        .read()) as i32),
                    )
            }),
        )) as i16);
        if ((columnChange) as i32) > 0i32 {
            MultiMove_SelectColumn(
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                    .read(),
            );
        }
        if ((columnChange) as i32) < 0i32 {
            MultiMove_DeselectColumn(
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                    .read(),
            );
            MultiMove_SelectColumn(
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                    .read(),
            );
        }
        if ((rowChange) as i32) > 0i32 {
            MultiMove_SelectRow(
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .read(),
            );
        }
        if ((rowChange) as i32) < 0i32 {
            MultiMove_DeselectRow(
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .read(),
            );
            MultiMove_SelectRow(
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .read(),
                ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MultiMove_SelectColumn(column: u8, minRow: u8, maxRow: u8) {
    unsafe {
        let mut column = column;
        let mut minRow = minRow;
        let mut maxRow = maxRow;
        if ((minRow) as i32) > ((maxRow) as i32) {
            let mut temp: u8 = 0u8;
            {
                temp = minRow;
                minRow = maxRow;
                maxRow = temp;
            }
        }
        'l1: loop {
            if !(((minRow) as i32) <= ((maxRow) as i32)) {
                break 'l1;
            }
            MultiMove_SetIconToBg(column, {
                let __t1 = minRow;
                minRow = (minRow).wrapping_add(1);
                __t1
            });
        }
    }
}
pub(crate) unsafe extern "C" fn MultiMove_SelectRow(row: u8, minColumn: u8, maxColumn: u8) {
    unsafe {
        let mut row = row;
        let mut minColumn = minColumn;
        let mut maxColumn = maxColumn;
        if ((minColumn) as i32) > ((maxColumn) as i32) {
            let mut temp: u8 = 0u8;
            {
                temp = minColumn;
                minColumn = maxColumn;
                maxColumn = temp;
            }
        }
        'l1: loop {
            if !(((minColumn) as i32) <= ((maxColumn) as i32)) {
                break 'l1;
            }
            MultiMove_SetIconToBg(
                {
                    let __t1 = minColumn;
                    minColumn = (minColumn).wrapping_add(1);
                    __t1
                },
                row,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MultiMove_DeselectColumn(column: u8, minRow: u8, maxRow: u8) {
    unsafe {
        let mut column = column;
        let mut minRow = minRow;
        let mut maxRow = maxRow;
        if ((minRow) as i32) > ((maxRow) as i32) {
            let mut temp: u8 = 0u8;
            {
                temp = minRow;
                minRow = maxRow;
                maxRow = temp;
            }
        }
        'l1: loop {
            if !(((minRow) as i32) <= ((maxRow) as i32)) {
                break 'l1;
            }
            MultiMove_ClearIconFromBg(column, {
                let __t1 = minRow;
                minRow = (minRow).wrapping_add(1);
                __t1
            });
        }
    }
}
pub(crate) unsafe extern "C" fn MultiMove_DeselectRow(row: u8, minColumn: u8, maxColumn: u8) {
    unsafe {
        let mut row = row;
        let mut minColumn = minColumn;
        let mut maxColumn = maxColumn;
        if ((minColumn) as i32) > ((maxColumn) as i32) {
            let mut temp: u8 = 0u8;
            {
                temp = minColumn;
                minColumn = maxColumn;
                maxColumn = temp;
            }
        }
        'l1: loop {
            if !(((minColumn) as i32) <= ((maxColumn) as i32)) {
                break 'l1;
            }
            MultiMove_ClearIconFromBg(
                {
                    let __t1 = minColumn;
                    minColumn = (minColumn).wrapping_add(1);
                    __t1
                },
                row,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MultiMove_SetIconToBg(x: u8, y: u8) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut position: u8 =
            ((((x) as i32).wrapping_add((6i32).wrapping_mul(((y) as i32)))) as u8);
        let mut species: u16 = ((GetCurrentBoxMonData(position, 65i32)) as u16);
        let mut personality: u32 = GetCurrentBoxMonData(position, 0i32);
        if ((species) as i32) != 0i32 {
            let mut iconGfx: *mut u8 = GetMonIconPtr(species, personality, 1u32);
            let mut index: u8 =
                ((((GetValidMonIconPalIndex(species)) as i32).wrapping_add(8i32)) as u8);
            BlitBitmapRectToWindow4BitTo8Bit(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8704)
                    .cast::<u16>())
                .read()) as u8),
                iconGfx,
                0u16,
                0u16,
                32u16,
                32i32,
                (((24i32).wrapping_mul(((x) as i32))) as u16),
                (((24i32).wrapping_mul(((y) as i32))) as u16),
                32u16,
                32u16,
                index,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MultiMove_ClearIconFromBg(x: u8, y: u8) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut position: u8 =
            ((((x) as i32).wrapping_add((6i32).wrapping_mul(((y) as i32)))) as u8);
        let mut species: u16 = ((GetCurrentBoxMonData(position, 65i32)) as u16);
        if ((species) as i32) != 0i32 {
            FillWindowPixelRect8Bit(
                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8704)
                    .cast::<u16>())
                .read()) as u8),
                0u8,
                (((24i32).wrapping_mul(((x) as i32))) as u16),
                (((24i32).wrapping_mul(((y) as i32))) as u16),
                32u16,
                32u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MultiMove_InitMove(x: u16, y: u16, moveSteps: u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut moveSteps = moveSteps;
        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<u16>())
        .write(x);
        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .write(y);
        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u16>())
        .write(moveSteps);
    }
}
pub(crate) unsafe extern "C" fn MultiMove_UpdateMove() -> u8 {
    unsafe {
        if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            ChangeBgX(
                0u8,
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<u16>())
                .read()) as i32),
                1u8,
            );
            ChangeBgY(
                0u8,
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14)
                    .cast::<u16>())
                .read()) as i32),
                1u8,
            );
            let __p1 = (((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        return ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u16>())
        .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn MultiMove_GetMonsFromSelection() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut columnCount: i32 = 0i32;
        let mut rowCount: i32 = 0i32;
        let mut boxId: u8 = 0u8;
        let mut monArrayId: u8 = 0u8;
        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8)).write(
            ((if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32)
                < ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read()) as i32)
            {
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .read()) as i32)
            } else {
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .read()) as i32)
            }) as u8),
        );
        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(
            ((if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32)
                < ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5))
                .read()) as i32)
            {
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                    .read()) as i32)
            } else {
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                    .read()) as i32)
            }) as u8),
        );
        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10)).write(
            (((if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read()) as i32),
                )
                < 0i32
            {
                (((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32),
                    ))
                .wrapping_neg()
            } else {
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32),
                    )
            })
            .wrapping_add(1i32)) as u8),
        );
        ((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(11)).write(
            (((if ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32)
                .wrapping_sub(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .read()) as i32),
                )
                < 0i32
            {
                (((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(5))
                        .read()) as i32),
                    ))
                .wrapping_neg()
            } else {
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                    .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(5))
                        .read()) as i32),
                    )
            })
            .wrapping_add(1i32)) as u8),
        );
        boxId = StorageGetCurrentBox();
        monArrayId = 0u8;
        columnCount = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8))
        .read()) as i32)
            .wrapping_add(
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10))
                .read()) as i32),
            );
        rowCount = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read()) as i32)
            .wrapping_add(
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11))
                .read()) as i32),
            );
        {
            i = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .read()) as i32);
            'l1: loop {
                if !(i < rowCount) {
                    break 'l1;
                }
                'l2: {
                    let mut boxPosition: u8 = ((((6i32).wrapping_mul(i)).wrapping_add(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32),
                    )) as u8);
                    {
                        j = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32);
                        'l3: loop {
                            if !(j < columnCount) {
                                break 'l3;
                            }
                            'l4: {
                                let mut boxMon: *mut u8 = GetBoxedMonPtr(boxId, boxPosition);
                                if ((boxMon) as usize) != 0usize {
                                    (((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset(((monArrayId) as i32) as isize * 80)
                                    .cast::<crate::c::Rec4<80>>()
                                    .write_unaligned(
                                        boxMon.cast::<crate::c::Rec4<80>>().read_unaligned(),
                                    );
                                }
                                monArrayId = (monArrayId).wrapping_add(1);
                                boxPosition = (boxPosition).wrapping_add(1);
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
pub(crate) unsafe extern "C" fn MultiMove_RemoveMonsFromBox() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut columnCount: i32 =
            ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .read()) as i32),
                );
        let mut rowCount: i32 = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read()) as i32)
            .wrapping_add(
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11))
                .read()) as i32),
            );
        let mut boxId: u8 = StorageGetCurrentBox();
        {
            i = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .read()) as i32);
            'l1: loop {
                if !(i < rowCount) {
                    break 'l1;
                }
                'l2: {
                    let mut boxPosition: u8 = ((((6i32).wrapping_mul(i)).wrapping_add(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32),
                    )) as u8);
                    {
                        j = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32);
                        'l3: loop {
                            if !(j < columnCount) {
                                break 'l3;
                            }
                            'l4: {
                                DestroyBoxMonIconAtPosition(boxPosition);
                                ZeroBoxMonAt(boxId, boxPosition);
                                boxPosition = (boxPosition).wrapping_add(1);
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
pub(crate) unsafe extern "C" fn MultiMove_CreatePlacedMonIcons() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut columnCount: i32 =
            ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .read()) as i32),
                );
        let mut rowCount: i32 = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read()) as i32)
            .wrapping_add(
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11))
                .read()) as i32),
            );
        let mut monArrayId: u8 = 0u8;
        {
            i = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .read()) as i32);
            'l1: loop {
                if !(i < rowCount) {
                    break 'l1;
                }
                'l2: {
                    let mut boxPosition: u8 = ((((6i32).wrapping_mul(i)).wrapping_add(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32),
                    )) as u8);
                    {
                        j = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32);
                        'l3: loop {
                            if !(j < columnCount) {
                                break 'l3;
                            }
                            'l4: {
                                if (GetBoxMonData2(
                                    (((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset(((monArrayId) as i32) as isize * 80),
                                    5i32,
                                )) != 0
                                {
                                    CreateBoxMonIconAtPos(boxPosition);
                                }
                                monArrayId = (monArrayId).wrapping_add(1);
                                boxPosition = (boxPosition).wrapping_add(1);
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
pub(crate) unsafe extern "C" fn MultiMove_SetPlacedMonData() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut columnCount: i32 =
            ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .read()) as i32),
                );
        let mut rowCount: i32 = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read()) as i32)
            .wrapping_add(
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11))
                .read()) as i32),
            );
        let mut boxId: u8 = StorageGetCurrentBox();
        let mut monArrayId: u8 = 0u8;
        {
            i = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .read()) as i32);
            'l1: loop {
                if !(i < rowCount) {
                    break 'l1;
                }
                'l2: {
                    let mut boxPosition: u8 = ((((6i32).wrapping_mul(i)).wrapping_add(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32),
                    )) as u8);
                    {
                        j = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32);
                        'l3: loop {
                            if !(j < columnCount) {
                                break 'l3;
                            }
                            'l4: {
                                if (GetBoxMonData2(
                                    (((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset(((monArrayId) as i32) as isize * 80),
                                    5i32,
                                )) != 0
                                {
                                    SetBoxMonAt(
                                        boxId,
                                        boxPosition,
                                        (((((&raw mut sMultiMove)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(20))
                                        .cast::<u8>())
                                        .wrapping_offset(((monArrayId) as i32) as isize * 80),
                                    );
                                }
                                boxPosition = (boxPosition).wrapping_add(1);
                                monArrayId = (monArrayId).wrapping_add(1);
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
pub(crate) unsafe extern "C" fn MultiMove_ResetBg() {
    unsafe {
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        SetBgAttribute(0u8, 4u8, 0u8);
        ClearGpuRegBits(8u8, 128u16);
        FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn MultiMove_GetOrigin() -> u8 {
    unsafe {
        return ((((6i32).wrapping_mul(
            ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .read()) as i32),
        ))
        .wrapping_add(
            ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .read()) as i32),
        )) as u8);
    }
}
pub(crate) unsafe extern "C" fn MultiMove_CanPlaceSelection() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut columnCount: i32 =
            ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .read()) as i32),
                );
        let mut rowCount: i32 = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read()) as i32)
            .wrapping_add(
                ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11))
                .read()) as i32),
            );
        let mut monArrayId: u8 = 0u8;
        {
            i = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .read()) as i32);
            'l1: loop {
                if !(i < rowCount) {
                    break 'l1;
                }
                'l2: {
                    let mut boxPosition: u8 = ((((6i32).wrapping_mul(i)).wrapping_add(
                        ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32),
                    )) as u8);
                    {
                        j = ((((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32);
                        'l3: loop {
                            if !(j < columnCount) {
                                break 'l3;
                            }
                            'l4: {
                                if ((GetBoxMonData2(
                                    (((((&raw mut sMultiMove).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset(((monArrayId) as i32) as isize * 80),
                                    5i32,
                                )) != 0)
                                    && ((GetCurrentBoxMonData(boxPosition, 5i32)) != 0)
                                {
                                    return 0u8;
                                }
                                monArrayId = (monArrayId).wrapping_add(1);
                                boxPosition = (boxPosition).wrapping_add(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn CreateItemIconSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut spriteId: u8 = 0u8;
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            == 3i32
        {
            (((&raw mut spriteSheet).cast::<u8>()).cast::<*mut u32>())
                .write(((&raw mut sItemIconGfxBuffer).cast::<u8>().cast::<u32>()).cast::<u32>());
            (((&raw mut spriteSheet).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(512u16);
            (&raw mut spriteTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .write_unaligned(
                    (&raw const sSpriteTemplate_ItemIcon)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<24>>()
                        .read_unaligned(),
                );
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 3i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut spriteSheet).cast::<u8>())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .write((((7i32).wrapping_add(i)) as u16));
                        LoadCompressedSpriteSheet((&raw mut spriteSheet).cast::<u8>());
                        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .write(
                            ((100728832i32) as usize as *mut u8).wrapping_offset(
                                (((GetSpriteTileStartByTag(
                                    (((&raw mut spriteSheet).cast::<u8>())
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                    .read(),
                                )) as i32)
                                    .wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                                    as isize
                                    * 1,
                            ),
                        );
                        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(8)
                        .cast::<u16>())
                        .write(
                            ((AllocSpritePalette((((56011i32).wrapping_add(i)) as u16))) as u16),
                        );
                        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(8)
                        .cast::<u16>())
                        .write(
                            (((256i32).wrapping_add(
                                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8708))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                                .wrapping_add(8)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_mul(16i32),
                            )) as u16),
                        );
                        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>())
                            .write((((7i32).wrapping_add(i)) as u16));
                        (((&raw mut spriteTemplate).cast::<u8>())
                            .wrapping_add(2)
                            .cast::<u16>())
                        .write((((56011i32).wrapping_add(i)) as u16));
                        spriteId =
                            CreateSprite((&raw mut spriteTemplate).cast::<u8>(), 0i16, 0i16, 11u8);
                        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                        .cast::<*mut u8>())
                        .write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                        crate::c::bf_write(
                            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8708))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 16))
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(12))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8756)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn TryLoadItemIconAtPos(cursorArea: u8, cursorPos: u8) {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        let mut heldItem: u16 = 0u16;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            return;
        }
        if (IsItemIconAtPosition(cursorArea, cursorPos)) != 0 {
            return;
        }
        'l1: {
            let __sw1 = ((cursorArea) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                if !((GetCurrentBoxMonData(cursorPos, 5i32)) != 0) {
                    return;
                }
                heldItem = ((GetCurrentBoxMonData(cursorPos, 12i32)) as u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((cursorPos) as i32) >= 6i32)
                    || (!((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((cursorPos) as i32) as isize * 100),
                        5i32,
                    )) != 0))
                {
                    return;
                }
                heldItem = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((cursorPos) as i32) as isize * 100),
                    12i32,
                )) as u16);
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        if ((heldItem) as i32) != 0i32 {
            let mut tiles: *mut u32 = GetItemIconPic(heldItem);
            let mut pal: *mut u32 = GetItemIconPalette(heldItem);
            let mut id: u8 = GetNewItemIconIdx();
            SetItemIconPosition(id, cursorArea, cursorPos);
            LoadItemIconGfx(id, tiles, pal);
            SetItemIconAffineAnim(id, 1u8);
            SetItemIconActive(id, 1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn TryHideItemIconAtPos(cursorArea: u8, cursorPos: u8) {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        let mut id: u8 = 0u8;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            return;
        }
        id = GetItemIconIdxByPosition(cursorArea, cursorPos);
        SetItemIconAffineAnim(id, 2u8);
        SetItemIconCallback(id, 0u8, cursorArea, cursorPos);
    }
}
pub(crate) unsafe extern "C" fn TakeItemFromMon(cursorArea: u8, cursorPos: u8) {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        let mut id: u8 = 0u8;
        let mut itemId: u16 = 0u16;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            return;
        }
        id = GetItemIconIdxByPosition(cursorArea, cursorPos);
        itemId = 0u16;
        SetItemIconAffineAnim(id, 3u8);
        SetItemIconCallback(id, 1u8, cursorArea, cursorPos);
        SetItemIconPosition(id, 2u8, 0u8);
        if ((cursorArea) as i32) == 0i32 {
            SetCurrentBoxMonData(cursorPos, 12i32, (&raw mut itemId).cast::<u8>());
            SetBoxMonIconObjMode(cursorPos, 1u8);
        } else {
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((cursorPos) as i32) as isize * 100),
                12i32,
                (&raw mut itemId).cast::<u8>(),
            );
            SetPartyMonIconObjMode(cursorPos, 1u8);
        }
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8756)
            .cast::<u16>())
        .write(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3302)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn InitItemIconInCursor(itemId: u16) {
    unsafe {
        let mut itemId = itemId;
        let mut tiles: *mut u32 = GetItemIconPic(itemId);
        let mut pal: *mut u32 = GetItemIconPalette(itemId);
        let mut id: u8 = GetNewItemIconIdx();
        LoadItemIconGfx(id, tiles, pal);
        SetItemIconAffineAnim(id, 6u8);
        SetItemIconCallback(id, 1u8, 0u8, 0u8);
        SetItemIconPosition(id, 2u8, 0u8);
        SetItemIconActive(id, 1u8);
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8756)
            .cast::<u16>())
        .write(itemId);
    }
}
pub(crate) unsafe extern "C" fn SwapItemsWithMon(cursorArea: u8, cursorPos: u8) {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        let mut id: u8 = 0u8;
        let mut itemId: u16 = 0u16;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            return;
        }
        id = GetItemIconIdxByPosition(cursorArea, cursorPos);
        SetItemIconAffineAnim(id, 3u8);
        SetItemIconCallback(id, 3u8, 2u8, 0u8);
        if ((cursorArea) as i32) == 0i32 {
            itemId = ((GetCurrentBoxMonData(cursorPos, 12i32)) as u16);
            SetCurrentBoxMonData(
                cursorPos,
                12i32,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8756)
                    .cast::<u16>())
                .cast::<u8>(),
            );
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8756)
                .cast::<u16>())
            .write(itemId);
        } else {
            itemId = ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((cursorPos) as i32) as isize * 100),
                12i32,
            )) as u16);
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((cursorPos) as i32) as isize * 100),
                12i32,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8756)
                    .cast::<u16>())
                .cast::<u8>(),
            );
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8756)
                .cast::<u16>())
            .write(itemId);
        }
        id = GetItemIconIdxByPosition(2u8, 0u8);
        SetItemIconAffineAnim(id, 4u8);
        SetItemIconCallback(id, 4u8, cursorArea, cursorPos);
    }
}
pub(crate) unsafe extern "C" fn GiveItemToMon(cursorArea: u8, cursorPos: u8) {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        let mut id: u8 = 0u8;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            return;
        }
        id = GetItemIconIdxByPosition(2u8, 0u8);
        SetItemIconAffineAnim(id, 4u8);
        SetItemIconCallback(id, 2u8, cursorArea, cursorPos);
        if ((cursorArea) as i32) == 0i32 {
            SetCurrentBoxMonData(
                cursorPos,
                12i32,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8756)
                    .cast::<u16>())
                .cast::<u8>(),
            );
            SetBoxMonIconObjMode(cursorPos, 0u8);
        } else {
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((cursorPos) as i32) as isize * 100),
                12i32,
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8756)
                    .cast::<u16>())
                .cast::<u8>(),
            );
            SetPartyMonIconObjMode(cursorPos, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn MoveItemFromMonToBag(cursorArea: u8, cursorPos: u8) {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        let mut id: u8 = 0u8;
        let mut itemId: u16 = 0u16;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            return;
        }
        itemId = 0u16;
        id = GetItemIconIdxByPosition(cursorArea, cursorPos);
        SetItemIconAffineAnim(id, 2u8);
        SetItemIconCallback(id, 0u8, cursorArea, cursorPos);
        if ((cursorArea) as i32) == 0i32 {
            SetCurrentBoxMonData(cursorPos, 12i32, (&raw mut itemId).cast::<u8>());
            SetBoxMonIconObjMode(cursorPos, 1u8);
        } else {
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((cursorPos) as i32) as isize * 100),
                12i32,
                (&raw mut itemId).cast::<u8>(),
            );
            SetPartyMonIconObjMode(cursorPos, 1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn MoveItemFromCursorToBag() {
    unsafe {
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            == 3i32
        {
            let mut id: u8 = GetItemIconIdxByPosition(2u8, 0u8);
            SetItemIconAffineAnim(id, 5u8);
            SetItemIconCallback(id, 0u8, 2u8, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn MoveHeldItemWithPartyMenu() {
    unsafe {
        let mut i: i32 = 0i32;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            != 3i32
        {
            return;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 16))
                    .wrapping_add(12))
                    .read())
                        != 0)
                        && ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(10))
                        .read()) as i32)
                            == 1i32)
                    {
                        SetItemIconCallback(((i) as u8), 7u8, 2u8, 0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsItemIconAnimActive() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 16))
                    .wrapping_add(12))
                    .read())
                        != 0
                    {
                        if (!((crate::c::bf_read(
                            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8708))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 16))
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(63),
                            5,
                            1,
                            false,
                        ) as u16)
                            != 0))
                            && ((crate::c::bf_read(
                                ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8708))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(63),
                                3,
                                1,
                                false,
                            ) as u16)
                                != 0)
                        {
                            return 1u8;
                        }
                        if (core::mem::transmute::<_, usize>(
                            (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(8708))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 16))
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .read(),
                        ) != (SpriteCallbackDummy as *const () as usize))
                            && (core::mem::transmute::<_, usize>(
                                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8708))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                .read(),
                            ) != (SpriteCB_ItemIcon_SetPosToCursor as *const () as usize))
                        {
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
pub(crate) unsafe extern "C" fn IsMovingItem() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).read())
            as i32)
            == 3i32
        {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 3i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(12))
                        .read())
                            != 0)
                            && ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(8708))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 16))
                            .wrapping_add(10))
                            .read()) as i32)
                                == 2i32)
                        {
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
pub(crate) unsafe extern "C" fn GetMovingItemName() -> *mut u8 {
    unsafe {
        return GetItemName(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8756)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetMovingItemId() -> u16 {
    unsafe {
        return ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8756)
            .cast::<u16>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetNewItemIconIdx() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if !(((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 16))
                    .wrapping_add(12))
                    .read())
                        != 0)
                    {
                        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 16))
                        .wrapping_add(12))
                        .write(1u8);
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 3u8;
    }
}
pub(crate) unsafe extern "C" fn IsItemIconAtPosition(cursorArea: u8, cursorPos: u8) -> u32 {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 16))
                    .wrapping_add(12))
                    .read())
                        != 0)
                        && ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(10))
                        .read()) as i32)
                            == ((cursorArea) as i32)))
                        && ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(11))
                        .read()) as i32)
                            == ((cursorPos) as i32))
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetItemIconIdxByPosition(cursorArea: u8, cursorPos: u8) -> u8 {
    unsafe {
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 16))
                    .wrapping_add(12))
                    .read())
                        != 0)
                        && ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 16))
                        .wrapping_add(10))
                        .read()) as i32)
                            == ((cursorArea) as i32)))
                        && ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 16))
                        .wrapping_add(11))
                        .read()) as i32)
                            == ((cursorPos) as i32))
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 3u8;
    }
}
pub(crate) unsafe extern "C" fn GetItemIconIdxBySprite(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 16))
                    .wrapping_add(12))
                    .read())
                        != 0)
                        && ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8708))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 16))
                        .cast::<*mut u8>())
                        .read()) as usize)
                            == ((sprite) as usize))
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 3u8;
    }
}
pub(crate) unsafe extern "C" fn SetItemIconPosition(id: u8, cursorArea: u8, cursorPos: u8) {
    unsafe {
        let mut id = id;
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        if ((id) as i32) >= 3i32 {
            return;
        }
        'l1: {
            let __sw1 = ((cursorArea) as i32);
            if __sw1 == 0i32 {
                x = ((crate::c::rem_i32(((cursorPos) as i32), 6i32)) as u8);
                y = ((crate::c::div_i32(((cursorPos) as i32), 6i32)) as u8);
                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .write(((((24i32).wrapping_mul(((x) as i32))).wrapping_add(112i32)) as i16));
                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .write(((((24i32).wrapping_mul(((y) as i32))).wrapping_add(56i32)) as i16));
                crate::c::bf_write(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(5),
                    2,
                    2,
                    (2u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((cursorPos) as i32) == 0i32 {
                    (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(116i16);
                    (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(76i16);
                } else {
                    (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(164i16);
                    (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(
                        ((((24i32).wrapping_mul(((cursorPos) as i32).wrapping_sub(1i32)))
                            .wrapping_add(28i32)) as i16),
                    );
                }
                crate::c::bf_write(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(5),
                    2,
                    2,
                    (1u16) as i32,
                );
                break 'l1;
            }
        }
        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8708))
            .cast::<u8>())
        .wrapping_offset(((id) as i32) as isize * 16))
        .wrapping_add(10))
        .write(cursorArea);
        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8708))
            .cast::<u8>())
        .wrapping_offset(((id) as i32) as isize * 16))
        .wrapping_add(11))
        .write(cursorPos);
    }
}
pub(crate) unsafe extern "C" fn LoadItemIconGfx(id: u8, itemTiles: *mut u32, itemPal: *mut u32) {
    unsafe {
        let mut id = id;
        let mut itemTiles = itemTiles;
        let mut itemPal = itemPal;
        let mut i: i32 = 0i32;
        if ((id) as i32) >= 3i32 {
            return;
        }
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(0u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(17092))
                        .cast::<u8>(),
                        ((16777216i32
                            | (crate::c::div_i32(512i32, crate::c::div_i32(32i32, 8i32))
                                & 2097151i32)) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        LZ77UnCompWram(
            itemTiles,
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8900))
                .cast::<u8>(),
        );
        {
            i = 0i32;
            'l3: loop {
                if !(i < 3i32) {
                    break 'l3;
                }
                'l4: {
                    'l5: loop {
                        'l6: {
                            CpuFastSet(
                                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8900))
                                .cast::<u8>())
                                .wrapping_offset(((i).wrapping_mul(96i32)) as isize),
                                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(17092))
                                .cast::<u8>())
                                .wrapping_offset(((i).wrapping_mul(128i32)) as isize),
                                ((crate::c::div_i32(96i32, crate::c::div_i32(32i32, 8i32))
                                    & 2097151i32) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l5;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        'l7: loop {
            'l8: {
                CpuFastSet(
                    ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(17092))
                    .cast::<u8>(),
                    (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8708))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 16))
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read(),
                    ((crate::c::div_i32(512i32, crate::c::div_i32(32i32, 8i32)) & 2097151i32)
                        as u32),
                );
            }
            if !((0i32) != 0) {
                break 'l7;
            }
        }
        LZ77UnCompWram(
            itemPal,
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(17092))
                .cast::<u8>(),
        );
        LoadPalette(
            ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(17092))
                .cast::<u8>(),
            (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8708))
            .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
            .wrapping_add(8)
            .cast::<u16>())
            .read(),
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn SetItemIconAffineAnim(id: u8, animNum: u8) {
    unsafe {
        let mut id = id;
        let mut animNum = animNum;
        if ((id) as i32) >= 3i32 {
            return;
        }
        StartSpriteAffineAnim(
            (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8708))
            .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
            .cast::<*mut u8>())
            .read(),
            animNum,
        );
    }
}
pub(crate) unsafe extern "C" fn SetItemIconCallback(
    id: u8,
    callbackId: u8,
    cursorArea: u8,
    cursorPos: u8,
) {
    unsafe {
        let mut id = id;
        let mut callbackId = callbackId;
        let mut cursorArea = cursorArea;
        let mut cursorPos = cursorPos;
        if ((id) as i32) >= 3i32 {
            return;
        }
        'l1: {
            let __sw1 = ((callbackId) as i32);
            if __sw1 == 0i32 {
                ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .write(((id) as i16));
                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ItemIcon_WaitAnim));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ItemIcon_ToHand));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(((cursorArea) as i16));
                (((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(((cursorPos) as i16));
                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ItemIcon_ToMon));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ItemIcon_SwapToHand));
                (((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(((cursorArea) as i16));
                (((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(((cursorPos) as i16));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                (((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(((cursorArea) as i16));
                (((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(((cursorPos) as i16));
                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ItemIcon_SwapToMon));
                break 'l1;
            }
            if __sw1 == 7i32 {
                (((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8708))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 16))
                .cast::<*mut u8>())
                .read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ItemIcon_HideParty));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetItemIconActive(id: u8, active: u8) {
    unsafe {
        let mut id = id;
        let mut active = active;
        if ((id) as i32) >= 3i32 {
            return;
        }
        (((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8708))
            .cast::<u8>())
        .wrapping_offset(((id) as i32) as isize * 16))
        .wrapping_add(12))
        .write(active);
        crate::c::bf_write(
            ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8708))
            .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 16))
            .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            ((((active) as i32) == 0i32) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn GetItemIconPic(itemId: u16) -> *mut u32 {
    unsafe {
        let mut itemId = itemId;
        return (GetItemIconPicOrPalette(itemId, 0u8)).cast::<u32>();
    }
}
pub(crate) unsafe extern "C" fn GetItemIconPalette(itemId: u16) -> *mut u32 {
    unsafe {
        let mut itemId = itemId;
        return (GetItemIconPicOrPalette(itemId, 1u8)).cast::<u32>();
    }
}
pub(crate) unsafe extern "C" fn PrintItemDescription() {
    unsafe {
        let mut description: *mut u8 = core::ptr::null_mut();
        if (IsMovingItem()) != 0 {
            description = GetItemDescription(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8756)
                    .cast::<u16>())
                .read(),
            );
        } else {
            description = GetItemDescription(
                ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3302)
                    .cast::<u16>())
                .read(),
            );
        }
        FillWindowPixelBuffer(2u8, 17u8);
        AddTextPrinterParameterized5(2u8, 1u8, description, 4u8, 0u8, 0u8, None, 0u8, 1u8);
    }
}
pub(crate) unsafe extern "C" fn InitItemInfoWindow() {
    unsafe {
        ((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8758)
            .cast::<u16>())
        .write(21u16);
        LoadBgTiles(
            0u8,
            (((&raw const sItemInfoFrame_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            128u16,
            314u16,
        );
        DrawItemInfoWindow(0u32);
    }
}
pub(crate) unsafe extern "C" fn UpdateItemInfoWindowSlideIn() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut pos: i32 = 0i32;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8758)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            return 0u8;
        }
        let __p1 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8758)
            .cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        pos = (21i32).wrapping_sub(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8758)
                .cast::<u16>())
            .read()) as i32),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < pos) {
                    break 'l1;
                }
                'l2: {
                    WriteSequenceToBgTilemapBuffer(
                        0u8,
                        ((((((GetBgAttribute(0u8, 10u8)) as i32).wrapping_add(20i32))
                            .wrapping_add(
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8758)
                                    .cast::<u16>())
                                .read()) as i32),
                            ))
                        .wrapping_add(i)) as u16),
                        ((i) as u8),
                        13u8,
                        1u8,
                        7u8,
                        15u8,
                        21i16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DrawItemInfoWindow(((pos) as u32));
        return ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8758)
            .cast::<u16>())
        .read()) as i32)
            != 0i32) as u8);
    }
}
pub(crate) unsafe extern "C" fn UpdateItemInfoWindowSlideOut() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut pos: i32 = 0i32;
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8758)
            .cast::<u16>())
        .read()) as i32)
            == 22i32
        {
            return 0u8;
        }
        if ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8758)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            FillBgTilemapBufferRect(0u8, 0u16, 21u8, 12u8, 1u8, 9u8, 17u8);
        }
        let __p1 = (((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8758)
            .cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        pos = (21i32).wrapping_sub(
            ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8758)
                .cast::<u16>())
            .read()) as i32),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < pos) {
                    break 'l1;
                }
                'l2: {
                    WriteSequenceToBgTilemapBuffer(
                        0u8,
                        ((((((GetBgAttribute(0u8, 10u8)) as i32).wrapping_add(20i32))
                            .wrapping_add(
                                ((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8758)
                                    .cast::<u16>())
                                .read()) as i32),
                            ))
                        .wrapping_add(i)) as u16),
                        ((i) as u8),
                        13u8,
                        1u8,
                        7u8,
                        15u8,
                        21i16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if pos >= 0i32 {
            DrawItemInfoWindow(((pos) as u32));
        }
        FillBgTilemapBufferRect(
            0u8,
            0u16,
            (((pos).wrapping_add(1i32)) as u8),
            12u8,
            1u8,
            9u8,
            17u8,
        );
        ScheduleBgCopyTilemapToVram(0u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DrawItemInfoWindow(x: u32) {
    unsafe {
        let mut x = x;
        if x != 0u32 {
            FillBgTilemapBufferRect(0u8, 314u16, 0u8, 12u8, ((x) as u8), 1u8, 15u8);
            FillBgTilemapBufferRect(0u8, 2362u16, 0u8, 20u8, ((x) as u8), 1u8, 15u8);
        }
        FillBgTilemapBufferRect(0u8, 315u16, ((x) as u8), 13u8, 1u8, 7u8, 15u8);
        FillBgTilemapBufferRect(0u8, 316u16, ((x) as u8), 12u8, 1u8, 1u8, 15u8);
        FillBgTilemapBufferRect(0u8, 317u16, ((x) as u8), 20u8, 1u8, 1u8, 15u8);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_WaitAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            SetItemIconActive(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                0u8,
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_ToHand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(10i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(21i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 4) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 4) as i16),
                );
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 11i32
                {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_ItemIcon_SetPosToCursor));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_SetPosToCursor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(4i32)) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            (((((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3252)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3252)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(38)
                    .cast::<i16>())
                    .read()) as i32),
                ))
            .wrapping_add(8i32)) as i16),
        );
        crate::c::bf_write(
            (sprite).wrapping_add(5),
            2,
            2,
            (crate::c::bf_read(
                (((((&raw mut sStorage).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3252)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5),
                2,
                2,
                false,
            ) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_ToMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(10i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(21i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 4) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 4) as i16),
                );
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 11i32
                {
                    SetItemIconPosition(
                        GetItemIconIdxBySprite(sprite),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as u8),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as u8),
                    );
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_SwapToHand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(10i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(21i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 4) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 4) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_mul(8i32)) as isize,
                    ))
                    .read()) as i32)
                        >> 4) as i16),
                );
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 11i32
                {
                    SetItemIconPosition(
                        GetItemIconIdxBySprite(sprite),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as u8),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as u8),
                    );
                    ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_ItemIcon_SetPosToCursor));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_SwapToMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(10i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(21i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 4) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 4) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    (((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_mul(8i32)) as isize,
                    ))
                    .read()) as i32)
                        >> 4)
                        .wrapping_neg()) as i16),
                );
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 11i32
                {
                    SetItemIconPosition(
                        GetItemIconIdxBySprite(sprite),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as u8),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as u8),
                    );
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy));
                    ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_HideParty(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
            < (-16i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            SetItemIconActive(GetItemIconIdxBySprite(sprite), 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn BackupPokemonStorage() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn RestorePokemonStorage() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StorageGetCurrentBox() -> u8 {
    unsafe {
        return (((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).read();
    }
}
pub(crate) unsafe extern "C" fn SetCurrentBox(boxId: u8) {
    unsafe {
        let mut boxId = boxId;
        if ((boxId) as i32) < 14i32 {
            (((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).write(boxId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonDataAt(boxId: u8, boxPosition: u8, request: i32) -> u32 {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut request = request;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            return GetBoxMonData2(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                request,
            );
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoxMonDataAt(boxId: u8, boxPosition: u8, request: i32, value: *mut u8) {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut request = request;
        let mut value = value;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            SetBoxMonData(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                request,
                value,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentBoxMonData(boxPosition: u8, request: i32) -> u32 {
    unsafe {
        let mut boxPosition = boxPosition;
        let mut request = request;
        return GetBoxMonDataAt(
            (((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).read(),
            boxPosition,
            request,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurrentBoxMonData(boxPosition: u8, request: i32, value: *mut u8) {
    unsafe {
        let mut boxPosition = boxPosition;
        let mut request = request;
        let mut value = value;
        SetBoxMonDataAt(
            (((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).read(),
            boxPosition,
            request,
            value,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonNickAt(boxId: u8, boxPosition: u8, dst: *mut u8) {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut dst = dst;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            GetBoxMonData3(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                2i32,
                dst,
            );
        } else {
            (dst).write(255u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonLevelAt(boxId: u8, boxPosition: u8) -> u32 {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut lvl: u32 = 0u32;
        if ((((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32))
            && ((GetBoxMonData2(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                5i32,
            )) != 0)
        {
            lvl = ((GetLevelFromBoxMonExp(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
            )) as u32);
        }
        lvl = 0u32;
        return lvl;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoxMonNickAt(boxId: u8, boxPosition: u8, nick: *mut u8) {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut nick = nick;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            SetBoxMonData(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                2i32,
                nick,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAndCopyBoxMonDataAt(
    boxId: u8,
    boxPosition: u8,
    request: i32,
    dst: *mut u8,
) -> u32 {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut request = request;
        let mut dst = dst;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            return GetBoxMonData3(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                request,
                dst,
            );
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoxMonAt(boxId: u8, boxPosition: u8, src: *mut u8) {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut src = src;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(((boxId) as i32) as isize * 2400))
            .cast::<u8>())
            .wrapping_offset(((boxPosition) as i32) as isize * 80)
            .cast::<crate::c::Rec4<80>>()
            .write_unaligned(src.cast::<crate::c::Rec4<80>>().read_unaligned());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyBoxMonAt(boxId: u8, boxPosition: u8, dst: *mut u8) {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut dst = dst;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            dst.cast::<crate::c::Rec4<80>>().write_unaligned(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80)
                .cast::<crate::c::Rec4<80>>()
                .read_unaligned(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBoxMonAt(
    boxId: u8,
    boxPosition: u8,
    species: u16,
    level: u8,
    fixedIV: u8,
    hasFixedPersonality: u8,
    personality: u32,
    otIDType: u8,
    otID: u32,
) {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut species = species;
        let mut level = level;
        let mut fixedIV = fixedIV;
        let mut hasFixedPersonality = hasFixedPersonality;
        let mut personality = personality;
        let mut otIDType = otIDType;
        let mut otID = otID;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            CreateBoxMon(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                species,
                level,
                fixedIV,
                hasFixedPersonality,
                personality,
                otIDType,
                otID,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ZeroBoxMonAt(boxId: u8, boxPosition: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            ZeroBoxMonData(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BoxMonAtToMon(boxId: u8, boxPosition: u8, dst: *mut u8) {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        let mut dst = dst;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            BoxMonToMon(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                dst,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxedMonPtr(boxId: u8, boxPosition: u8) -> *mut u8 {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        if (((boxId) as i32) < 14i32) && (((boxPosition) as i32) < 30i32) {
            return (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read())
                .wrapping_add(4))
            .cast::<u8>())
            .wrapping_offset(((boxId) as i32) as isize * 2400))
            .cast::<u8>())
            .wrapping_offset(((boxPosition) as i32) as isize * 80);
        } else {
            return core::ptr::null_mut();
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxNamePtr(boxId: u8) -> *mut u8 {
    unsafe {
        let mut boxId = boxId;
        if ((boxId) as i32) < 14i32 {
            return ((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read())
                .wrapping_add(33604))
            .cast::<u8>())
            .wrapping_offset(((boxId) as i32) as isize * 9))
            .cast::<u8>();
        } else {
            return core::ptr::null_mut();
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn GetBoxWallpaper(boxId: u8) -> u8 {
    unsafe {
        let mut boxId = boxId;
        if ((boxId) as i32) < 14i32 {
            return ((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read())
                .wrapping_add(33730))
            .cast::<u8>())
            .wrapping_offset(((boxId) as i32) as isize))
            .read();
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SetBoxWallpaper(boxId: u8, wallpaperId: u8) {
    unsafe {
        let mut boxId = boxId;
        let mut wallpaperId = wallpaperId;
        if (((boxId) as i32) < 14i32) && (((wallpaperId) as i32) < 17i32) {
            ((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(33730))
                .cast::<u8>())
            .wrapping_offset(((boxId) as i32) as isize))
            .write(wallpaperId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdvanceStorageMonIndex(
    boxMons: *mut u8,
    currIndex: u8,
    maxIndex: u8,
    mode: u8,
) -> i16 {
    unsafe {
        let mut boxMons = boxMons;
        let mut currIndex = currIndex;
        let mut maxIndex = maxIndex;
        let mut mode = mode;
        let mut i: i16 = 0i16;
        let mut direction: i16 = (-1i16);
        if (((mode) as i32) == 0i32) || (((mode) as i32) == 1i32) {
            direction = 1i16;
        }
        if (((mode) as i32) == 1i32) || (((mode) as i32) == 3i32) {
            {
                i = (((((currIndex) as i8) as i32).wrapping_add(((direction) as i32))) as i16);
                'l1: loop {
                    if !((((i) as i32) >= 0i32) && (((i) as i32) <= ((maxIndex) as i32))) {
                        break 'l1;
                    }
                    'l2: {
                        if GetBoxMonData2(
                            (boxMons).wrapping_offset(((i) as i32) as isize * 80),
                            11i32,
                        ) != 0u32
                        {
                            return i;
                        }
                    }
                    i = ((((i) as i32).wrapping_add(((direction) as i32))) as i16);
                }
            }
        } else {
            {
                i = (((((currIndex) as i8) as i32).wrapping_add(((direction) as i32))) as i16);
                'l3: loop {
                    if !((((i) as i32) >= 0i32) && (((i) as i32) <= ((maxIndex) as i32))) {
                        break 'l3;
                    }
                    'l4: {
                        if (GetBoxMonData2(
                            (boxMons).wrapping_offset(((i) as i32) as isize * 80),
                            11i32,
                        ) != 0u32)
                            && (!((GetBoxMonData2(
                                (boxMons).wrapping_offset(((i) as i32) as isize * 80),
                                45i32,
                            )) != 0))
                        {
                            return i;
                        }
                    }
                    i = ((((i) as i32).wrapping_add(((direction) as i32))) as i16);
                }
            }
        }
        return (-1i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckFreePokemonStorageSpace() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 14i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 30i32) {
                                break 'l3;
                            }
                            'l4: {
                                if !((GetBoxMonData2(
                                    (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 2400))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 80),
                                    5i32,
                                )) != 0)
                                {
                                    return 1u8;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckBoxMonSanityAt(boxId: u32, boxPosition: u32) -> u32 {
    unsafe {
        let mut boxId = boxId;
        let mut boxPosition = boxPosition;
        if ((((boxId < 14u32) && (boxPosition < 30u32))
            && ((GetBoxMonData2(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                5i32,
            )) != 0))
            && (!((GetBoxMonData2(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                6i32,
            )) != 0)))
            && (!((GetBoxMonData2(
                (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((boxId) as i32) as isize * 2400))
                .cast::<u8>())
                .wrapping_offset(((boxPosition) as i32) as isize * 80),
                4i32,
            )) != 0))
        {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountStorageNonEggMons() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut count: u32 = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 14i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 30i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((GetBoxMonData2(
                                    (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 2400))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 80),
                                    5i32,
                                )) != 0)
                                    && (!((GetBoxMonData2(
                                        (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 2400))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 80),
                                        6i32,
                                    )) != 0))
                                {
                                    count = (count).wrapping_add(1);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountAllStorageMons() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut count: u32 = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 14i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 30i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((GetBoxMonData2(
                                    (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 2400))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 80),
                                    5i32,
                                )) != 0)
                                    || ((GetBoxMonData2(
                                        (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 2400))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 80),
                                        6i32,
                                    )) != 0)
                                {
                                    count = (count).wrapping_add(1);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnyStorageMonWithMove(r#move: u16) -> u32 {
    unsafe {
        let mut r#move = r#move;
        let mut moves = crate::ffi::Align4([0u8; 4]);
        (&raw mut moves)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(r#move);
        (&raw mut moves)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(355u16);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 14i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 30i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((GetBoxMonData2(
                                    (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 2400))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 80),
                                    5i32,
                                )) != 0)
                                    && (!((GetBoxMonData2(
                                        (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 2400))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 80),
                                        6i32,
                                    )) != 0)))
                                    && ((GetBoxMonData3(
                                        (((((((&raw mut gPokemonStoragePtr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 2400))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 80),
                                        81i32,
                                        ((&raw mut moves).cast::<u16>()).cast::<u8>(),
                                    )) != 0)
                                {
                                    return 1u32;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetWaldaWallpaper() {
    unsafe {
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(20))
        .write(0u8);
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(21))
        .write(0u8);
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(22))
        .write(0u8);
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .cast::<u16>())
        .write(31541u16);
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .cast::<u16>())
        .wrapping_offset(1))
        .write(24966u16);
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(4))
        .cast::<u8>())
        .write(255u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaWallpaperLockedOrUnlocked(unlocked: u32) {
    unsafe {
        let mut unlocked = unlocked;
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(22))
        .write(((unlocked) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWaldaWallpaperUnlocked() -> u32 {
    unsafe {
        return (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(22))
        .read()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWaldaWallpaperPatternId() -> u32 {
    unsafe {
        return (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(21))
        .read()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaWallpaperPatternId(id: u8) {
    unsafe {
        let mut id = id;
        if ((id) as u32) < crate::c::div_u32(192u32, 12u32) {
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
                .wrapping_add(21))
            .write(id);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWaldaWallpaperIconId() -> u32 {
    unsafe {
        return (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(20))
        .read()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaWallpaperIconId(id: u8) {
    unsafe {
        let mut id = id;
        if ((id) as u32) < crate::c::div_u32(120u32, 4u32) {
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
                .wrapping_add(20))
            .write(id);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWaldaWallpaperColorsPtr() -> *mut u16 {
    unsafe {
        return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .cast::<u16>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaWallpaperColors(color1: u16, color2: u16) {
    unsafe {
        let mut color1 = color1;
        let mut color2 = color2;
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .cast::<u16>())
        .write(color1);
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .cast::<u16>())
        .wrapping_offset(1))
        .write(color2);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWaldaPhrasePtr() -> *mut u8 {
    unsafe {
        return (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(4))
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaPhrase(src: *mut u8) {
    unsafe {
        let mut src = src;
        StringCopy(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
                .wrapping_add(4))
            .cast::<u8>(),
            src,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWaldaPhraseEmpty() -> u32 {
    unsafe {
        return ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15728))
            .wrapping_add(4))
        .cast::<u8>())
        .read()) as i32)
            == 255i32) as u32);
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_Init(count: u8) {
    unsafe {
        let mut count = count;
        let mut i: u16 = 0u16;
        ((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>())
            .write(Alloc((48u32).wrapping_mul(((count) as u32))));
        ((&raw mut sNumTilemapUtilIds).cast::<u8>().cast::<u16>()).write(
            ((if ((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read()) as usize)
                == 0usize
            {
                0i32
            } else {
                ((count) as i32)
            }) as u16),
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut sNumTilemapUtilIds).cast::<u8>().cast::<u16>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 48))
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                    (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 48))
                    .wrapping_add(44))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_Free() {
    unsafe {
        Free(((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_UpdateAll() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((&raw mut sNumTilemapUtilIds).cast::<u8>().cast::<u16>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset((i) as isize * 48))
                    .wrapping_add(44))
                    .read()) as i32)
                        == 1i32
                    {
                        TilemapUtil_Update(((i) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_SetMap(
    id: u8,
    bg: u8,
    tilemap: *mut u8,
    width: u16,
    height: u16,
) {
    unsafe {
        let mut id = id;
        let mut bg = bg;
        let mut tilemap = tilemap;
        let mut width = width;
        let mut height = height;
        let mut bgScreenSize: u16 = 0u16;
        let mut bgType: u16 = 0u16;
        if ((id) as i32)
            >= ((((&raw mut sNumTilemapUtilIds).cast::<u8>().cast::<u16>()).read()) as i32)
        {
            return;
        }
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(24)
        .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(28)
        .cast::<*mut u8>())
        .write(tilemap);
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(43))
        .write(bg);
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(36)
        .cast::<u16>())
        .write(width);
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(38)
        .cast::<u16>())
        .write(height);
        bgScreenSize = GetBgAttribute(bg, 3u8);
        bgType = GetBgAttribute(bg, 9u8);
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(32)
        .cast::<u16>())
        .write(
            (((((((&raw const sTilemapDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((bgType) as i32) as isize * 16))
            .cast::<u8>())
            .wrapping_offset(((bgScreenSize) as i32) as isize * 4))
            .cast::<u16>())
            .read(),
        );
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(34)
        .cast::<u16>())
        .write(
            (((((((&raw const sTilemapDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((bgType) as i32) as isize * 16))
            .cast::<u8>())
            .wrapping_offset(((bgScreenSize) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        if ((bgType) as i32) != 0i32 {
            (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((id) as i32) as isize * 48))
            .wrapping_add(42))
            .write(1u8);
        } else {
            (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((id) as i32) as isize * 48))
            .wrapping_add(42))
            .write(2u8);
        }
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(40)
        .cast::<u16>())
        .write(
            (((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((id) as i32) as isize * 48))
            .wrapping_add(42))
            .read()) as i32)
                .wrapping_mul(((width) as i32))) as u16),
        );
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(4)
        .cast::<u16>())
        .write(width);
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(6)
        .cast::<u16>())
        .write(height);
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(2)
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(10)
        .cast::<i16>())
        .write(0i16);
        ((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .cast::<crate::c::Rec4<12>>()
        .write_unaligned(
            ((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((id) as i32) as isize * 48))
            .wrapping_add(12)
            .cast::<crate::c::Rec4<12>>()
            .read_unaligned(),
        );
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(44))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_SetSavedMap(id: u8, tilemap: *mut u8) {
    unsafe {
        let mut id = id;
        let mut tilemap = tilemap;
        if ((id) as i32)
            >= ((((&raw mut sNumTilemapUtilIds).cast::<u8>().cast::<u16>()).read()) as i32)
        {
            return;
        }
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(24)
        .cast::<*mut u8>())
        .write(tilemap);
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(44))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_SetPos(id: u8, x: u16, y: u16) {
    unsafe {
        let mut id = id;
        let mut x = x;
        let mut y = y;
        if ((id) as i32)
            >= ((((&raw mut sNumTilemapUtilIds).cast::<u8>().cast::<u16>()).read()) as i32)
        {
            return;
        }
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(((x) as i16));
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(10)
        .cast::<i16>())
        .write(((y) as i16));
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(44))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_SetRect(
    id: u8,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) {
    unsafe {
        let mut id = id;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        if ((id) as i32)
            >= ((((&raw mut sNumTilemapUtilIds).cast::<u8>().cast::<u16>()).read()) as i32)
        {
            return;
        }
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .cast::<i16>())
        .write(((x) as i16));
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(2)
        .cast::<i16>())
        .write(((y) as i16));
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(4)
        .cast::<u16>())
        .write(width);
        ((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(12))
        .wrapping_add(6)
        .cast::<u16>())
        .write(height);
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(44))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_Move(id: u8, mode: u8, val: i8) {
    unsafe {
        let mut id = id;
        let mut mode = mode;
        let mut val = val;
        if ((id) as i32)
            >= ((((&raw mut sNumTilemapUtilIds).cast::<u8>().cast::<u16>()).read()) as i32)
        {
            return;
        }
        'l1: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(8)
                .cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(((val) as i32))) as i16));
                let __p3 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(4)
                .cast::<u16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(((val) as i32))) as u16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p4 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .cast::<i16>();
                (__p4).write((((((__p4).read()) as i32).wrapping_add(((val) as i32))) as i16));
                let __p5 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(4)
                .cast::<u16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_add(((val) as i32))) as u16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p6 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(10)
                .cast::<i16>();
                (__p6).write((((((__p6).read()) as i32).wrapping_add(((val) as i32))) as i16));
                let __p7 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(6)
                .cast::<u16>();
                (__p7).write((((((__p7).read()) as i32).wrapping_sub(((val) as i32))) as u16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p8 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(2)
                .cast::<i16>();
                (__p8).write((((((__p8).read()) as i32).wrapping_sub(((val) as i32))) as i16));
                let __p9 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(6)
                .cast::<u16>();
                (__p9).write((((((__p9).read()) as i32).wrapping_add(((val) as i32))) as u16));
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p10 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(8)
                .cast::<i16>();
                (__p10).write((((((__p10).read()) as i32).wrapping_add(((val) as i32))) as i16));
                break 'l1;
            }
            if __sw1 == 5i32 {
                let __p11 = (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(10)
                .cast::<i16>();
                (__p11).write((((((__p11).read()) as i32).wrapping_add(((val) as i32))) as i16));
                break 'l1;
            }
        }
        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(44))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_Update(id: u8) {
    unsafe {
        let mut id = id;
        if ((id) as i32)
            >= ((((&raw mut sNumTilemapUtilIds).cast::<u8>().cast::<u16>()).read()) as i32)
        {
            return;
        }
        if (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(24)
        .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            TilemapUtil_DrawPrev(id);
        }
        TilemapUtil_Draw(id);
        ((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((id) as i32) as isize * 48))
        .cast::<crate::c::Rec4<12>>()
        .write_unaligned(
            ((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((id) as i32) as isize * 48))
            .wrapping_add(12)
            .cast::<crate::c::Rec4<12>>()
            .read_unaligned(),
        );
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_DrawPrev(id: u8) {
    unsafe {
        let mut id = id;
        let mut i: i32 = 0i32;
        let mut adder: u32 = (((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(42))
        .read()) as i32)
            .wrapping_mul(
                (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(32)
                .cast::<u16>())
                .read()) as i32),
            )) as u32);
        let mut tiles: *mut u8 = (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(24)
        .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            (((adder).wrapping_mul(
                (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(10)
                .cast::<i16>())
                .read()) as u32),
            )) as i32) as isize
                * 1,
        ))
        .wrapping_offset(
            ((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((id) as i32) as isize * 48))
            .wrapping_add(42))
            .read()) as i32)
                .wrapping_mul(
                    (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((id) as i32) as isize * 48))
                    .wrapping_add(8)
                    .cast::<i16>())
                    .read()) as i32),
                )) as isize
                * 1,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((id) as i32) as isize * 48))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    CopyToBgTilemapBufferRect(
                        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((id) as i32) as isize * 48))
                        .wrapping_add(43))
                        .read(),
                        tiles,
                        (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((id) as i32) as isize * 48))
                        .wrapping_add(8)
                        .cast::<i16>())
                        .read()) as u8),
                        (((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((id) as i32) as isize * 48))
                        .wrapping_add(10)
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(i)) as u8),
                        (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((id) as i32) as isize * 48))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as u8),
                        1u8,
                    );
                    tiles = (tiles).wrapping_offset(((adder) as i32) as isize * 1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_Draw(id: u8) {
    unsafe {
        let mut id = id;
        let mut i: i32 = 0i32;
        let mut adder: u32 = (((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(42))
        .read()) as i32)
            .wrapping_mul(
                (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(36)
                .cast::<u16>())
                .read()) as i32),
            )) as u32);
        let mut tiles: *mut u8 = (((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(((id) as i32) as isize * 48))
        .wrapping_add(28)
        .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            (((adder).wrapping_mul(
                ((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((id) as i32) as isize * 48))
                .wrapping_add(12))
                .wrapping_add(2)
                .cast::<i16>())
                .read()) as u32),
            )) as i32) as isize
                * 1,
        ))
        .wrapping_offset(
            ((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((id) as i32) as isize * 48))
            .wrapping_add(42))
            .read()) as i32)
                .wrapping_mul(
                    ((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((id) as i32) as isize * 48))
                    .wrapping_add(12))
                    .cast::<i16>())
                    .read()) as i32),
                )) as isize
                * 1,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((id) as i32) as isize * 48))
                    .wrapping_add(12))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    CopyToBgTilemapBufferRect(
                        (((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((id) as i32) as isize * 48))
                        .wrapping_add(43))
                        .read(),
                        tiles,
                        ((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((id) as i32) as isize * 48))
                        .wrapping_add(12))
                        .wrapping_add(8)
                        .cast::<i16>())
                        .read()) as u8),
                        ((((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((id) as i32) as isize * 48))
                        .wrapping_add(12))
                        .wrapping_add(10)
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(i)) as u8),
                        ((((((((&raw mut sTilemapUtil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((id) as i32) as isize * 48))
                        .wrapping_add(12))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as u8),
                        1u8,
                    );
                    tiles = (tiles).wrapping_offset(((adder) as i32) as isize * 1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UnkUtil_Init(util: *mut u8, data: *mut u8, max: u32) {
    unsafe {
        let mut util = util;
        let mut data = data;
        let mut max = max;
        ((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).write(util);
        ((util).cast::<*mut u8>()).write(data);
        ((util).wrapping_add(5)).write(((max) as u8));
        ((util).wrapping_add(4)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn UnkUtil_Run() {
    unsafe {
        let mut i: u16 = 0u16;
        if (((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4)).read())
            != 0
        {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32)
                        < ((((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        let mut data: *mut u8 =
                            (((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 20);
                        (((data)
                            .wrapping_add(16)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .read())
                        .unwrap_unchecked()(data);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn UnkUtil_CpuAdd(
    dest: *mut u8,
    dLeft: u16,
    dTop: u16,
    src: *mut u8,
    sLeft: u16,
    sTop: u16,
    width: u16,
    height: u16,
    unkArg: u16,
) -> u8 {
    unsafe {
        let mut dest = dest;
        let mut dLeft = dLeft;
        let mut dTop = dTop;
        let mut src = src;
        let mut sLeft = sLeft;
        let mut sTop = sTop;
        let mut width = width;
        let mut height = height;
        let mut unkArg = unkArg;
        let mut data: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4)).read())
            as i32)
            >= ((((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .read()) as i32)
        {
            return 0u8;
        }
        data = (((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_offset(
            (({
                let __p1 =
                    (((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize
                * 20,
        );
        ((data).wrapping_add(8).cast::<u16>())
            .write(((((width) as i32).wrapping_mul(2i32)) as u16));
        ((data).wrapping_add(4).cast::<*mut u8>()).write(
            (dest).wrapping_offset(
                ((2i32).wrapping_mul(
                    (((dTop) as i32).wrapping_mul(32i32)).wrapping_add(((dLeft) as i32)),
                )) as isize,
            ),
        );
        ((data).cast::<*mut u8>()).write((src).wrapping_offset(
            ((2i32).wrapping_mul(
                (((sTop) as i32).wrapping_mul(((unkArg) as i32))).wrapping_add(((sLeft) as i32)),
            )) as isize,
        ));
        ((data).wrapping_add(12).cast::<u16>()).write(height);
        ((data).wrapping_add(10).cast::<u16>()).write(unkArg);
        ((data)
            .wrapping_add(16)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(UnkUtil_CpuRun));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UnkUtil_CpuRun(data: *mut u8) {
    unsafe {
        let mut data = data;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((((data).wrapping_add(12).cast::<u16>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    'l3: loop {
                        'l4: {
                            'l5: loop {
                                'l6: {
                                    CpuSet(
                                        ((data).cast::<*mut u8>()).read(),
                                        ((data).wrapping_add(4).cast::<*mut u8>()).read(),
                                        ((0i32
                                            | (crate::c::div_i32(
                                                ((((data).wrapping_add(8).cast::<u16>()).read())
                                                    as i32),
                                                crate::c::div_i32(16i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l5;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                    let __p1 = (data).wrapping_add(4).cast::<*mut u8>();
                    (__p1).write(((__p1).read()).wrapping_offset(64));
                    let __p2 = (data).cast::<*mut u8>();
                    (__p2).write(
                        ((__p2).read()).wrapping_offset(
                            (((((data).wrapping_add(10).cast::<u16>()).read()) as i32)
                                .wrapping_mul(2i32)) as isize,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UnkUtil_DmaAdd(
    dest: *mut u8,
    dLeft: u16,
    dTop: u16,
    width: u16,
    height: u16,
) -> u8 {
    unsafe {
        let mut dest = dest;
        let mut dLeft = dLeft;
        let mut dTop = dTop;
        let mut width = width;
        let mut height = height;
        let mut data: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4)).read())
            as i32)
            >= ((((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .read()) as i32)
        {
            return 0u8;
        }
        data = (((((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_offset(
            (({
                let __p1 =
                    (((&raw mut sUnkUtil).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32) as isize
                * 20,
        );
        ((data).wrapping_add(8).cast::<u16>())
            .write(((((width) as i32).wrapping_mul(2i32)) as u16));
        ((data).wrapping_add(4).cast::<*mut u8>()).write(
            (dest).wrapping_offset(
                (((((dTop) as i32).wrapping_mul(32i32)).wrapping_add(((dLeft) as i32)))
                    .wrapping_mul(2i32)) as isize
                    * 1,
            ),
        );
        ((data).wrapping_add(12).cast::<u16>()).write(height);
        ((data)
            .wrapping_add(16)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(UnkUtil_DmaRun));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UnkUtil_DmaRun(data: *mut u8) {
    unsafe {
        let mut data = data;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((((data).wrapping_add(12).cast::<u16>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        let mut _dest: *mut u8 = ((data).wrapping_add(4).cast::<*mut u8>()).read();
                        let mut _size: u32 =
                            ((((data).wrapping_add(8).cast::<u16>()).read()) as u32);
                        'l3: loop {
                            if !((1i32) != 0) {
                                break 'l3;
                            }
                            if _size <= 4096u32 {
                                'l4: loop {
                                    'l5: {
                                        {
                                            let mut tmp: u16 = 0u16;
                                            (&raw mut tmp).write_volatile(0u16);
                                            'l6: loop {
                                                'l7: {
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
                                                                    ((crate::c::div_i32(
                                                                        16i32, 8i32,
                                                                    ))
                                                                        as u32),
                                                                )),
                                                        );
                                                        let _ = ((dmaRegs).wrapping_offset(2))
                                                            .read_volatile();
                                                    }
                                                }
                                                if !((0i32) != 0) {
                                                    break 'l6;
                                                }
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l4;
                                    }
                                }
                                break 'l3;
                            }
                            'l8: loop {
                                'l9: {
                                    {
                                        let mut tmp: u16 = 0u16;
                                        (&raw mut tmp).write_volatile(0u16);
                                        'l10: loop {
                                            'l11: {
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
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l10;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l8;
                                }
                            }
                            _dest = (_dest).wrapping_offset(4096);
                            _size = (_size).wrapping_sub(4096u32);
                        }
                    }
                    let __p1 = (data).wrapping_add(4).cast::<*mut u8>();
                    (__p1).write(((__p1).read()).wrapping_offset(64));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
