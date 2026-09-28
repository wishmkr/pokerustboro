//! Translated from `src/field_effect.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sNewGameBirch_Gfx sUnusedBirchBeauty sNewGameBirch_Pal sPokeballGlow_Gfx sPokeballGlow_Pal sPokecenterMonitor0_Gfx sPokecenterMonitor1_Gfx sHofMonitorBig_Gfx sHofMonitorSmall_Gfx sHofMonitor_Pal sFieldMoveStreaksOutdoors_Gfx sFieldMoveStreaksOutdoors_Pal sFieldMoveStreaksOutdoors_Tilemap sFieldMoveStreaksIndoors_Gfx sFieldMoveStreaksIndoors_Pal sFieldMoveStreaksIndoors_Tilemap sSpotlight_Pal sSpotlight_Gfx sRockFragment_TopLeft sRockFragment_TopRight sRockFragment_BottomLeft sRockFragment_BottomRight gFieldEffectScriptFuncs sOam_64x64 sOam_8x8 sOam_16x16 sPicTable_NewGameBirch sSpritePalette_NewGameBirch sAnim_NewGameBirch sAnimTable_NewGameBirch sSpriteTemplate_NewGameBirch gSpritePalette_PokeballGlow gSpritePalette_HofMonitor sOam_32x16 sPicTable_PokeballGlow sPicTable_PokecenterMonitor sPicTable_HofMonitorBig sPicTable_HofMonitorSmall sSubsprites_PokecenterMonitor sSubspriteTable_PokecenterMonitor sSubsprites_HofMonitorBig sSubspriteTable_HofMonitorBig sAnim_Static sAnim_Flicker sAnims_Flicker sAnims_HofMonitor sSpriteTemplate_PokeballGlow sSpriteTemplate_PokecenterMonitor sSpriteTemplate_HofMonitorBig sSpriteTemplate_HofMonitorSmall sPokecenterHealEffectFuncs sHallOfFameRecordEffectFuncs sPokeballGlowEffectFuncs sPokeballCoordOffsets sPokeballGlowReds sPokeballGlowGreens sPokeballGlowBlues sFallWarpFieldEffectFuncs sEscalatorWarpOutFieldEffectFuncs sEscalatorWarpInFieldEffectFuncs sWaterfallFieldEffectFuncs sDiveFieldEffectFuncs sLavaridgeGymB1FWarpEffectFuncs sLavaridgeGymB1FWarpExitEffectFuncs sLavaridgeGym1FWarpEffectFuncs sEscapeRopeWarpOutEffectFuncs sEscapeRopeWarpInEffectFuncs sTeleportWarpOutFieldEffectFuncs sTeleportWarpInFieldEffectFuncs sFieldMoveShowMonOutdoorsEffectFuncs sFieldMoveShowMonIndoorsEffectFuncs sSurfFieldEffectFuncs sFlyOutFieldEffectFuncs sAffineAnim_FlyBirdLeaveBall sAffineAnim_FlyBirdReturnToBall sAffineAnims_FlyBird sFlyInFieldEffectFuncs sDestroyDeoxysRockEffectFuncs sImages_DeoxysRockFragment sAnim_RockFragment_TopLeft sAnim_RockFragment_TopRight sAnim_RockFragment_BottomLeft sAnim_RockFragment_BottomRight sAnims_DeoxysRockFragment sSpriteTemplate_DeoxysRockFragment

static gFieldEffectScriptFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut *mut u8, *mut u32) -> u8>, 8>,
> = Table((&raw const crate::data::field_effect::gFieldEffectScriptFuncs).cast());
static sAffineAnims_FlyBird: Table<CArray<*mut AffineAnimCmd, 2>> =
    Table((&raw const crate::data::field_effect::sAffineAnims_FlyBird).cast());
static sDestroyDeoxysRockEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut i16, u8)>, 3>> =
    Table((&raw const crate::data::field_effect::sDestroyDeoxysRockEffectFuncs).cast());
static sDiveFieldEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::field_effect::sDiveFieldEffectFuncs).cast());
static sEscalatorWarpInFieldEffectFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 7>,
> = Table((&raw const crate::data::field_effect::sEscalatorWarpInFieldEffectFuncs).cast());
static sEscalatorWarpOutFieldEffectFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 6>,
> = Table((&raw const crate::data::field_effect::sEscalatorWarpOutFieldEffectFuncs).cast());
static sEscapeRopeWarpInEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 2>> =
    Table((&raw const crate::data::field_effect::sEscapeRopeWarpInEffectFuncs).cast());
static sEscapeRopeWarpOutEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 2>> =
    Table((&raw const crate::data::field_effect::sEscapeRopeWarpOutEffectFuncs).cast());
static sFallWarpFieldEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 7>> =
    Table((&raw const crate::data::field_effect::sFallWarpFieldEffectFuncs).cast());
static sFieldMoveShowMonIndoorsEffectFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task)>, 7>,
> = Table((&raw const crate::data::field_effect::sFieldMoveShowMonIndoorsEffectFuncs).cast());
static sFieldMoveShowMonOutdoorsEffectFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task)>, 7>,
> = Table((&raw const crate::data::field_effect::sFieldMoveShowMonOutdoorsEffectFuncs).cast());
static sFieldMoveStreaksIndoors_Gfx: Table<CArray<u32, 32>> =
    Table((&raw const crate::data::field_effect::sFieldMoveStreaksIndoors_Gfx).cast());
static sFieldMoveStreaksIndoors_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::field_effect::sFieldMoveStreaksIndoors_Pal).cast());
static sFieldMoveStreaksIndoors_Tilemap: Table<CArray<u16, 320>> =
    Table((&raw const crate::data::field_effect::sFieldMoveStreaksIndoors_Tilemap).cast());
static sFieldMoveStreaksOutdoors_Gfx: Table<CArray<u32, 128>> =
    Table((&raw const crate::data::field_effect::sFieldMoveStreaksOutdoors_Gfx).cast());
static sFieldMoveStreaksOutdoors_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::field_effect::sFieldMoveStreaksOutdoors_Pal).cast());
static sFieldMoveStreaksOutdoors_Tilemap: Table<CArray<u16, 320>> =
    Table((&raw const crate::data::field_effect::sFieldMoveStreaksOutdoors_Tilemap).cast());
static sFlyInFieldEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 7>> =
    Table((&raw const crate::data::field_effect::sFlyInFieldEffectFuncs).cast());
static sFlyOutFieldEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 9>> =
    Table((&raw const crate::data::field_effect::sFlyOutFieldEffectFuncs).cast());
static sHallOfFameRecordEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 4>> =
    Table((&raw const crate::data::field_effect::sHallOfFameRecordEffectFuncs).cast());
static sLavaridgeGym1FWarpEffectFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task, *mut ObjectEvent, *mut Sprite) -> u8>, 5>,
> = Table((&raw const crate::data::field_effect::sLavaridgeGym1FWarpEffectFuncs).cast());
static sLavaridgeGymB1FWarpEffectFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task, *mut ObjectEvent, *mut Sprite) -> u8>, 6>,
> = Table((&raw const crate::data::field_effect::sLavaridgeGymB1FWarpEffectFuncs).cast());
static sLavaridgeGymB1FWarpExitEffectFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task, *mut ObjectEvent, *mut Sprite) -> u8>, 4>,
> = Table((&raw const crate::data::field_effect::sLavaridgeGymB1FWarpExitEffectFuncs).cast());
static sOam_64x64: Table<OamData> =
    Table((&raw const crate::data::field_effect::sOam_64x64).cast());
static sPokeballCoordOffsets: Table<CArray<Coords16, 6>> =
    Table((&raw const crate::data::field_effect::sPokeballCoordOffsets).cast());
static sPokeballGlowBlues: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::field_effect::sPokeballGlowBlues).cast());
static sPokeballGlowEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Sprite)>, 8>> =
    Table((&raw const crate::data::field_effect::sPokeballGlowEffectFuncs).cast());
static sPokeballGlowGreens: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::field_effect::sPokeballGlowGreens).cast());
static sPokeballGlowReds: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::field_effect::sPokeballGlowReds).cast());
static sPokecenterHealEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 4>> =
    Table((&raw const crate::data::field_effect::sPokecenterHealEffectFuncs).cast());
static sSpotlight_Gfx: Table<CArray<u8, 2880>> =
    Table((&raw const crate::data::field_effect::sSpotlight_Gfx).cast());
static sSpotlight_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::field_effect::sSpotlight_Pal).cast());
static sSpritePalette_NewGameBirch: Table<SpritePalette> =
    Table((&raw const crate::data::field_effect::sSpritePalette_NewGameBirch).cast());
static sSpriteTemplate_DeoxysRockFragment: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_effect::sSpriteTemplate_DeoxysRockFragment).cast());
static sSpriteTemplate_HofMonitorBig: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_effect::sSpriteTemplate_HofMonitorBig).cast());
static sSpriteTemplate_HofMonitorSmall: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_effect::sSpriteTemplate_HofMonitorSmall).cast());
static sSpriteTemplate_NewGameBirch: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_effect::sSpriteTemplate_NewGameBirch).cast());
static sSpriteTemplate_PokeballGlow: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_effect::sSpriteTemplate_PokeballGlow).cast());
static sSpriteTemplate_PokecenterMonitor: Table<SpriteTemplate> =
    Table((&raw const crate::data::field_effect::sSpriteTemplate_PokecenterMonitor).cast());
static sSubspriteTable_HofMonitorBig: Table<SubspriteTable> =
    Table((&raw const crate::data::field_effect::sSubspriteTable_HofMonitorBig).cast());
static sSubspriteTable_PokecenterMonitor: Table<SubspriteTable> =
    Table((&raw const crate::data::field_effect::sSubspriteTable_PokecenterMonitor).cast());
static sSurfFieldEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 5>> =
    Table((&raw const crate::data::field_effect::sSurfFieldEffectFuncs).cast());
static sTeleportWarpInFieldEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 3>> =
    Table((&raw const crate::data::field_effect::sTeleportWarpInFieldEffectFuncs).cast());
static sTeleportWarpOutFieldEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 4>> =
    Table((&raw const crate::data::field_effect::sTeleportWarpOutFieldEffectFuncs).cast());
static sWaterfallFieldEffectFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut Task, *mut ObjectEvent) -> u8>, 5>,
> = Table((&raw const crate::data::field_effect::sWaterfallFieldEffectFuncs).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gFieldEffectArguments: CArray<i32, 8> = unsafe { zeroed() };
pub(crate) static mut sActiveList: Aligned<CArray<u8, 32>> = Aligned(unsafe { zeroed() });

unsafe extern "C" {
    static gDummySpriteAffineAnimTable: CArray<*mut AffineAnimCmd, 0>;
    static gDummySpriteAnimTable: CArray<*mut AnimCmd, 0>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static gFieldEffectObjectTemplatePointers: CArray<*mut SpriteTemplate, 0>;
    static mut gFieldEffectScriptPointers: CArray<*mut u8, 0>;
    static mut gMain: Main;
    static mut gMonPaletteTable: CArray<CompressedSpritePalette, 0>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSpriteCoordOffsetY: i16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTotalCameraPixelOffsetX: u16;
    static mut gTotalCameraPixelOffsetY: u16;
    static gTrainerFrontPicPaletteTable: CArray<CompressedSpritePalette, 0>;
    static gTrainerFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    fn BGMusicStopped() -> u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn CB2_LoadMap();
    fn CB2_ReturnToField();
    fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8);
    fn CalculatePlayerPartyCount() -> u8;
    fn CameraObjectFreeze();
    fn CameraObjectReset();
    fn ClearMirageTowerPulseBlendEffect();
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateInvisibleSprite(a0: Option<unsafe extern "C" fn(*mut Sprite)>) -> u8;
    fn CreateMonPicSprite_HandleDeoxys(
        a0: u16,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: i16,
        a5: i16,
        a6: u8,
        a7: u16,
    ) -> u16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn FadeInFromBlack();
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreeOamMatrix(a0: u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreezeObjectEvents();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetCurrentMapType() -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn GetFaceDirectionMovementAction(a0: u32) -> u8;
    fn GetJumpMovementAction(a0: u32) -> u8;
    fn GetJumpSpecialMovementAction(a0: u32) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonSpritePalStructFromOtIdPersonality(
        a0: u16,
        a1: u32,
        a2: u32,
    ) -> *mut CompressedSpritePalette;
    fn GetPlayerAvatarGraphicsIdByStateId(a0: u8) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetSpritePaletteTagByPaletteNum(a0: u8) -> u16;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetSpriteTileTagByTileStart(a0: u16) -> u16;
    fn GetWalkInPlaceFasterMovementAction(a0: u32) -> u8;
    fn GetWalkNormalMovementAction(a0: u32) -> u8;
    fn GetWalkSlowMovementAction(a0: u32) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn InitTextBoxGfxAndPrinters();
    fn InstallCameraPanAheadCallback();
    fn IsEscalatorMoving() -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsMapTypeOutdoors(a0: u8) -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn LZDecompressVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePaletteOverrideBuffer(a0: *mut CompressedSpritePalette, a1: *mut c_void);
    fn LoadCompressedSpriteSheetOverrideBuffer(a0: *mut CompressedSpriteSheet, a1: *mut c_void);
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadWordFromTwoHalfwords(a0: *mut u16, a1: *mut u32);
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsWaterfall(a0: u8) -> u8;
    fn MoveCoords(a0: u8, a1: *mut i16, a2: *mut i16);
    fn MoveObjectEventToMapCoords(a0: *mut ObjectEvent, a1: i16, a2: i16);
    fn ObjectEventCheckHeldMovementStatus(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventClearHeldMovementIfActive(a0: *mut ObjectEvent);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventIsMovementOverridden(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventSetGraphicsId(a0: *mut ObjectEvent, a1: u8);
    fn ObjectEventSetHeldMovement(a0: *mut ObjectEvent, a1: u8) -> u8;
    fn ObjectEventTurn(a0: *mut ObjectEvent, a1: u8);
    fn Overworld_ChangeMusicTo(a0: u16);
    fn Overworld_ClearSavedMusic();
    fn Overworld_PlaySpecialMapMusic();
    fn Overworld_ResetStateAfterFly();
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayCry_NormalNoDucking(a0: u16, a1: i8, a2: i8, a3: u8);
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PreservePaletteInWeather(a0: u8);
    fn RemoveObjectEventByLocalIdAndMap(a0: u8, a1: u8, a2: u8);
    fn ResetPreservedPalettesInWeather();
    fn SetCameraPanning(a0: i16, a1: i16);
    fn SetCameraPanningCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetObjectEventDirection(a0: *mut ObjectEvent, a1: u8);
    fn SetPlayerAvatarFieldMove();
    fn SetPlayerAvatarStateMask(a0: u8);
    fn SetPlayerAvatarTransitionFlags(a0: u16);
    fn SetSpritePosToOffsetMapCoords(a0: *mut i16, a1: *mut i16, a2: i16, a3: i16);
    fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable);
    fn SetSurfBlob_BobState(a0: u8, a1: u8);
    fn SetSurfBlob_DontSyncAnim(a0: u8, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWarpDestinationToEscapeWarp();
    fn SetWarpDestinationToLastHealLocation();
    fn ShiftObjectEventCoords(a0: *mut ObjectEvent, a1: i16, a2: i16);
    fn ShiftStillObjectEventCoords(a0: *mut ObjectEvent);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartEscalator(a0: u8);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StopEscalator();
    fn StoreWordInTwoHalfwords(a0: *mut u16, a1: u32);
    fn TryDoDiveWarp(a0: *mut MapPosition, a1: u16) -> u8;
    fn TryFadeOutOldMapMusic();
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn UnfreezeObjectEvents();
    fn UnlockPlayerFieldControls();
    fn UpdateCameraPanning();
    fn UpdateSpritePaletteWithWeather(a0: u8);
    fn WarpFadeInScreen();
    fn WarpFadeOutScreen();
    fn WarpIntoMap();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectStart(id: u8) -> u32 {
    let mut script: *mut u8 = null_mut();
    let mut val: u32 = 0;
    FieldEffectActiveListAdd(id);
    script = gFieldEffectScriptPointers[id];
    while gFieldEffectScriptFuncs[*script].unwrap_unchecked()(&raw mut script, &raw mut val) != 0 {}
    return val;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadtiles(script: *mut *mut u8, val: *mut u32) -> u8 {
    *script = (*script).at(1);
    FieldEffectScript_LoadTiles(script);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadfadedpal(script: *mut *mut u8, val: *mut u32) -> u8 {
    *script = (*script).at(1);
    FieldEffectScript_LoadFadedPalette(script);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadpal(script: *mut *mut u8, val: *mut u32) -> u8 {
    *script = (*script).at(1);
    FieldEffectScript_LoadPalette(script);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_callnative(script: *mut *mut u8, val: *mut u32) -> u8 {
    *script = (*script).at(1);
    FieldEffectScript_CallNative(script, val);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_end(script: *mut *mut u8, val: *mut u32) -> u8 {
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadgfx_callnative(
    script: *mut *mut u8,
    val: *mut u32,
) -> u8 {
    *script = (*script).at(1);
    FieldEffectScript_LoadTiles(script);
    FieldEffectScript_LoadFadedPalette(script);
    FieldEffectScript_CallNative(script, val);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadtiles_callnative(
    script: *mut *mut u8,
    val: *mut u32,
) -> u8 {
    *script = (*script).at(1);
    FieldEffectScript_LoadTiles(script);
    FieldEffectScript_CallNative(script, val);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadfadedpal_callnative(
    script: *mut *mut u8,
    val: *mut u32,
) -> u8 {
    *script = (*script).at(1);
    FieldEffectScript_LoadFadedPalette(script);
    FieldEffectScript_CallNative(script, val);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_ReadWord(script: *mut *mut u8) -> u32 {
    return *(*script) as u32
        + ((*(*script).at(1) as u32) << 8)
        + ((*(*script).at(2) as u32) << 16)
        + ((*(*script).at(3) as u32) << 24);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_LoadTiles(script: *mut *mut u8) {
    let mut sheet: *mut SpriteSheet =
        FieldEffectScript_ReadWord(script) as usize as *mut SpriteSheet;
    if GetSpriteTileStartByTag((*sheet).tag) == 0xFFFF {
        LoadSpriteSheet(sheet);
    }
    *script = (*script).at(4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_LoadFadedPalette(script: *mut *mut u8) {
    let mut palette: *mut SpritePalette =
        FieldEffectScript_ReadWord(script) as usize as *mut SpritePalette;
    LoadSpritePalette(palette);
    UpdateSpritePaletteWithWeather(IndexOfSpritePaletteTag((*palette).tag));
    *script = (*script).at(4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_LoadPalette(script: *mut *mut u8) {
    let mut palette: *mut SpritePalette =
        FieldEffectScript_ReadWord(script) as usize as *mut SpritePalette;
    LoadSpritePalette(palette);
    *script = (*script).at(4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_CallNative(script: *mut *mut u8, val: *mut u32) {
    let mut func: Option<unsafe extern "C" fn() -> u32> =
        core::mem::transmute::<usize, Option<unsafe extern "C" fn() -> u32>>(
            FieldEffectScript_ReadWord(script) as usize,
        );
    *val = func.unwrap_unchecked()();
    *script = (*script).at(4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectFreeGraphicsResources(sprite: *mut Sprite) {
    let mut sheetTileStart: u16 = (*sprite).sheetTileStart;
    let mut paletteNum: u32 = (*sprite).oam.paletteNum() as u32;
    DestroySprite(sprite);
    FieldEffectFreeTilesIfUnused(sheetTileStart);
    FieldEffectFreePaletteIfUnused(paletteNum as u8);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectStop(sprite: *mut Sprite, id: u8) {
    FieldEffectFreeGraphicsResources(sprite);
    FieldEffectActiveListRemove(id);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectFreeTilesIfUnused(tileStart: u16) {
    let mut i: u8 = 0;
    let mut tag: u16 = GetSpriteTileTagByTileStart(tileStart);
    if tag != TAG_NONE {
        i = 0;
        while i < MAX_SPRITES {
            if gSprites[i].inUse() != 0
                && gSprites[i].usingSheet() != 0
                && tileStart == gSprites[i].sheetTileStart
            {
                return;
            }
            i += 1;
        }
        FreeSpriteTilesByTag(tag);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectFreePaletteIfUnused(paletteNum: u8) {
    let mut i: u8 = 0;
    let mut tag: u16 = GetSpritePaletteTagByPaletteNum(paletteNum);
    if tag != TAG_NONE {
        i = 0;
        while i < MAX_SPRITES {
            if gSprites[i].inUse() != 0 && gSprites[i].oam.paletteNum() == paletteNum as u16 {
                return;
            }
            i += 1;
        }
        FreeSpritePaletteByTag(tag);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectActiveListClear() {
    let mut i: u8 = 0;
    i = 0;
    while i < 32 {
        sActiveList[i] = 0xFF;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectActiveListAdd(id: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < 32 {
        if sActiveList[i] == 0xFF {
            sActiveList[i] = id;
            return;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectActiveListRemove(id: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < 32 {
        if sActiveList[i] == id {
            sActiveList[i] = 0xFF;
            return;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectActiveListContains(id: u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < 32 {
        if sActiveList[i] == id {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTrainerSprite(
    trainerSpriteID: u8,
    x: i16,
    y: i16,
    subpriority: u8,
    buffer: *mut u8,
) -> u8 {
    let mut spriteTemplate: SpriteTemplate = zeroed();
    LoadCompressedSpritePaletteOverrideBuffer(
        (&raw const gTrainerFrontPicPaletteTable[trainerSpriteID]).cast_mut(),
        buffer as *mut c_void,
    );
    LoadCompressedSpriteSheetOverrideBuffer(
        (&raw const gTrainerFrontPicTable[trainerSpriteID]).cast_mut(),
        buffer as *mut c_void,
    );
    spriteTemplate.tileTag = gTrainerFrontPicTable[trainerSpriteID].tag;
    spriteTemplate.paletteTag = gTrainerFrontPicPaletteTable[trainerSpriteID].tag;
    spriteTemplate.oam = (&raw const *sOam_64x64).cast_mut();
    spriteTemplate.anims = gDummySpriteAnimTable.as_ptr().cast_mut();
    spriteTemplate.images = null_mut();
    spriteTemplate.affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
    spriteTemplate.callback = Some(SpriteCallbackDummy);
    return CreateSprite(&raw mut spriteTemplate, x, y, subpriority);
}
pub(crate) unsafe extern "C" fn LoadTrainerGfx_TrainerCard(
    gender: u8,
    palOffset: u16,
    dest: *mut u8,
) {
    LZDecompressVram(gTrainerFrontPicTable[gender].data, dest as *mut c_void);
    LoadCompressedPalette(gTrainerFrontPicPaletteTable[gender].data, palOffset, 32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddNewGameBirchObject(x: i16, y: i16, subpriority: u8) -> u8 {
    LoadSpritePalette((&raw const *sSpritePalette_NewGameBirch).cast_mut());
    return CreateSprite(
        (&raw const *sSpriteTemplate_NewGameBirch).cast_mut(),
        x,
        y,
        subpriority,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonSprite_PicBox(
    species: u16,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    let mut spriteId: i32 = CreateMonPicSprite_HandleDeoxys(
        species,
        0,
        0x8000,
        TRUE,
        x,
        y,
        0,
        gMonPaletteTable[species].tag,
    ) as i32;
    PreservePaletteInWeather(IndexOfSpritePaletteTag(gMonPaletteTable[species].tag) + 0x10);
    if spriteId == 0xFFFF {
        return MAX_SPRITES;
    } else {
        return spriteId as u8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonSprite_FieldMove(
    species: u16,
    otId: u32,
    personality: u32,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    let mut spritePalette: *mut CompressedSpritePalette =
        GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
    let mut spriteId: u16 = CreateMonPicSprite_HandleDeoxys(
        species,
        otId,
        personality,
        TRUE,
        x,
        y,
        0,
        (*spritePalette).tag,
    );
    PreservePaletteInWeather(IndexOfSpritePaletteTag((*spritePalette).tag) + 0x10);
    if spriteId == 0xFFFF {
        return MAX_SPRITES;
    } else {
        return spriteId as u8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeResourcesAndDestroySprite(sprite: *mut Sprite, spriteId: u8) {
    ResetPreservedPalettesInWeather();
    if (*sprite).oam.affineMode() != ST_OAM_AFFINE_OFF {
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
    }
    FreeAndDestroyMonPicSprite(spriteId as u16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiplyInvertedPaletteRGBComponents(i: u16, r: u8, g: u8, b: u8) {
    let mut curRed: i32 = 0;
    let mut curGreen: i32 = 0;
    let mut curBlue: i32 = 0;
    let mut color: u16 = gPlttBufferUnfaded[i];
    curRed = color as i32 & 31;
    curGreen = (color as i32 & 992) >> 5;
    curBlue = (color as i32 & 31744) >> 10;
    curRed += (0x1F - curRed) * r as i32 >> 4;
    curGreen += (0x1F - curGreen) * g as i32 >> 4;
    curBlue += (0x1F - curBlue) * b as i32 >> 4;
    color = curRed as u16;
    color |= (curGreen as u16) << 5;
    color |= (curBlue as u16) << 10;
    gPlttBufferFaded[i] = color;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiplyPaletteRGBComponents(i: u16, r: u8, g: u8, b: u8) {
    let mut curRed: i32 = 0;
    let mut curGreen: i32 = 0;
    let mut curBlue: i32 = 0;
    let mut color: u16 = gPlttBufferUnfaded[i];
    curRed = color as i32 & 31;
    curGreen = (color as i32 & 992) >> 5;
    curBlue = (color as i32 & 31744) >> 10;
    curRed -= curRed * r as i32 >> 4;
    curGreen -= curGreen * g as i32 >> 4;
    curBlue -= curBlue * b as i32 >> 4;
    color = curRed as u16;
    color |= (curGreen as u16) << 5;
    color |= (curBlue as u16) << 10;
    gPlttBufferFaded[i] = color;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_PokecenterHeal() -> u8 {
    let mut nPokemon: u8 = 0;
    let mut task: *mut Task = null_mut();
    nPokemon = CalculatePlayerPartyCount();
    task = &raw mut gTasks[CreateTask(Some(Task_PokecenterHeal), 0xff)];
    (*task).data[1] = nPokemon as i16;
    (*task).data[2] = 93;
    (*task).data[3] = 36;
    (*task).data[4] = 124;
    (*task).data[5] = 24;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_PokecenterHeal(taskId: u8) {
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[taskId];
    sPokecenterHealEffectFuncs[(*task).data[0]].unwrap_unchecked()(task);
}
pub(crate) unsafe extern "C" fn PokecenterHealEffect_Init(task: *mut Task) {
    (*task).data[0] += 1;
    (*task).data[6] = CreateGlowingPokeballsEffect(
        (*task).data[1],
        (*task).data[2],
        (*task).data[3],
        TRUE as u16,
    ) as i16;
    (*task).data[7] = CreatePokecenterMonitorSprite((*task).data[4], (*task).data[5]) as i16;
}
pub(crate) unsafe extern "C" fn PokecenterHealEffect_WaitForBallPlacement(task: *mut Task) {
    if gSprites[(*task).data[6]].data[0] > 1 {
        gSprites[(*task).data[7]].data[0] += 1;
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn PokecenterHealEffect_WaitForBallFlashing(task: *mut Task) {
    if gSprites[(*task).data[6]].data[0] > 4 {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn PokecenterHealEffect_WaitForSoundAndEnd(task: *mut Task) {
    if gSprites[(*task).data[6]].data[0] > 6 {
        DestroySprite(&raw mut gSprites[(*task).data[6]]);
        FieldEffectActiveListRemove(FLDEFF_POKECENTER_HEAL);
        DestroyTask(FindTaskIdByFunc(Some(Task_PokecenterHeal)));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_HallOfFameRecord() -> u8 {
    let mut nPokemon: u8 = 0;
    let mut task: *mut Task = null_mut();
    nPokemon = CalculatePlayerPartyCount();
    task = &raw mut gTasks[CreateTask(Some(Task_HallOfFameRecord), 0xff)];
    (*task).data[1] = nPokemon as i16;
    (*task).data[2] = 117;
    (*task).data[3] = 52;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_HallOfFameRecord(taskId: u8) {
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[taskId];
    sHallOfFameRecordEffectFuncs[(*task).data[0]].unwrap_unchecked()(task);
}
pub(crate) unsafe extern "C" fn HallOfFameRecordEffect_Init(task: *mut Task) {
    let mut taskId: u8 = 0;
    (*task).data[0] += 1;
    (*task).data[6] = CreateGlowingPokeballsEffect(
        (*task).data[1],
        (*task).data[2],
        (*task).data[3],
        FALSE as u16,
    ) as i16;
    taskId = FindTaskIdByFunc(Some(Task_HallOfFameRecord));
    CreateHofMonitorSprite(taskId as i16, 120, 24, FALSE);
    CreateHofMonitorSprite(taskId as i16, 40, 8, TRUE);
    CreateHofMonitorSprite(taskId as i16, 72, 8, TRUE);
    CreateHofMonitorSprite(taskId as i16, 168, 8, TRUE);
    CreateHofMonitorSprite(taskId as i16, 200, 8, TRUE);
}
pub(crate) unsafe extern "C" fn HallOfFameRecordEffect_WaitForBallPlacement(task: *mut Task) {
    if gSprites[(*task).data[6]].data[0] > 1 {
        (*task).data[15] += 1;
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn HallOfFameRecordEffect_WaitForBallFlashing(task: *mut Task) {
    if gSprites[(*task).data[6]].data[0] > 4 {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn HallOfFameRecordEffect_WaitForSoundAndEnd(task: *mut Task) {
    if gSprites[(*task).data[6]].data[0] > 6 {
        DestroySprite(&raw mut gSprites[(*task).data[6]]);
        FieldEffectActiveListRemove(FLDEFF_HALL_OF_FAME_RECORD);
        DestroyTask(FindTaskIdByFunc(Some(Task_HallOfFameRecord)));
    }
}
pub(crate) unsafe extern "C" fn CreateGlowingPokeballsEffect(
    numMons: i16,
    x: i16,
    y: i16,
    playHealSe: u16,
) -> u8 {
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    spriteId = CreateInvisibleSprite(Some(SpriteCB_PokeballGlowEffect));
    sprite = &raw mut gSprites[spriteId];
    (*sprite).x2 = x;
    (*sprite).y2 = y;
    (*sprite).data[5] = playHealSe as i16;
    (*sprite).data[6] = numMons;
    (*sprite).data[7] = spriteId as i16;
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_PokeballGlowEffect(sprite: *mut Sprite) {
    sPokeballGlowEffectFuncs[(*sprite).data[0]].unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_PlaceBalls(sprite: *mut Sprite) {
    let mut spriteId: u8 = 0;
    if (*sprite).data[1] == 0
        || ({
            (*sprite).data[1] -= 1;
            (*sprite).data[1]
        }) == 0
    {
        (*sprite).data[1] = 25;
        spriteId = CreateSpriteAtEnd(
            (&raw const *sSpriteTemplate_PokeballGlow).cast_mut(),
            sPokeballCoordOffsets[(*sprite).data[2]].x + (*sprite).x2,
            sPokeballCoordOffsets[(*sprite).data[2]].y + (*sprite).y2,
            0,
        );
        gSprites[spriteId].oam.set_priority(2);
        gSprites[spriteId].data[0] = (*sprite).data[7];
        (*sprite).data[2] += 1;
        (*sprite).data[6] -= 1;
        PlaySE(SE_BALL);
    }
    if (*sprite).data[6] == 0 {
        (*sprite).data[1] = 32;
        (*sprite).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_TryPlaySe(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] -= 1;
        (*sprite).data[1]
    }) == 0
    {
        (*sprite).data[0] += 1;
        (*sprite).data[1] = 8;
        (*sprite).data[2] = 0;
        (*sprite).data[3] = 0;
        if (*sprite).data[5] != 0 {
            PlayFanfare(MUS_HEAL);
        }
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_Flash1(sprite: *mut Sprite) {
    let mut phase: u8 = 0;
    if ({
        (*sprite).data[1] -= 1;
        (*sprite).data[1]
    }) == 0
    {
        (*sprite).data[1] = 8;
        (*sprite).data[2] += 1;
        (*sprite).data[2] &= 3;
        if (*sprite).data[2] == 0 {
            (*sprite).data[3] += 1;
        }
    }
    phase = (*sprite).data[2] as u8 + 3 & 3;
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 8,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
    phase = (*sprite).data[2] as u8 + 2 & 3;
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 6,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
    phase = (*sprite).data[2] as u8 + 1 & 3;
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 2,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
    phase = (*sprite).data[2] as u8;
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 5,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 3,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
    if (*sprite).data[3] > 2 {
        (*sprite).data[0] += 1;
        (*sprite).data[1] = 8;
        (*sprite).data[2] = 0;
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_Flash2(sprite: *mut Sprite) {
    let mut phase: u8 = 0;
    if ({
        (*sprite).data[1] -= 1;
        (*sprite).data[1]
    }) == 0
    {
        (*sprite).data[1] = 8;
        (*sprite).data[2] += 1;
        (*sprite).data[2] &= 3;
        if (*sprite).data[2] == 3 {
            (*sprite).data[0] += 1;
            (*sprite).data[1] = 30;
        }
    }
    phase = (*sprite).data[2] as u8;
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 8,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 6,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 2,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 5,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(FLDEFF_PAL_TAG_POKEBALL_GLOW) as u16 * 16 + 3,
        sPokeballGlowReds[phase],
        sPokeballGlowGreens[phase],
        sPokeballGlowBlues[phase],
    );
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_WaitAfterFlash(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] -= 1;
        (*sprite).data[1]
    }) == 0
    {
        (*sprite).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_Dummy(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_WaitForSound(sprite: *mut Sprite) {
    if (*sprite).data[5] == FALSE as i16 || IsFanfareTaskInactive() != 0 {
        (*sprite).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_Idle(sprite: *mut Sprite) {}
pub(crate) unsafe extern "C" fn SpriteCB_PokeballGlow(sprite: *mut Sprite) {
    if gSprites[(*sprite).data[0]].data[0] > 4 {
        FieldEffectFreeGraphicsResources(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreatePokecenterMonitorSprite(x: i16, y: i16) -> u8 {
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    spriteId = CreateSpriteAtEnd(
        (&raw const *sSpriteTemplate_PokecenterMonitor).cast_mut(),
        x,
        y,
        0,
    );
    sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(2);
    (*sprite).set_invisible(TRUE as u16);
    SetSubspriteTables(
        sprite,
        (&raw const *sSubspriteTable_PokecenterMonitor).cast_mut(),
    );
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_PokecenterMonitor(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).data[0] = 0;
        (*sprite).set_invisible(FALSE as u16);
        StartSpriteAnim(sprite, 1);
    }
    if (*sprite).animEnded() != 0 {
        FieldEffectFreeGraphicsResources(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateHofMonitorSprite(
    taskId: i16,
    x: i16,
    y: i16,
    isSmallMonitor: u8,
) {
    let mut spriteId: u8 = 0;
    if isSmallMonitor == 0 {
        spriteId = CreateSpriteAtEnd(
            (&raw const *sSpriteTemplate_HofMonitorBig).cast_mut(),
            x,
            y,
            0,
        );
        SetSubspriteTables(
            &raw mut gSprites[spriteId],
            (&raw const *sSubspriteTable_HofMonitorBig).cast_mut(),
        );
    } else {
        spriteId = CreateSpriteAtEnd(
            (&raw const *sSpriteTemplate_HofMonitorSmall).cast_mut(),
            x,
            y,
            0,
        );
    }
    gSprites[spriteId].set_invisible(TRUE as u16);
    gSprites[spriteId].data[0] = taskId;
}
pub(crate) unsafe extern "C" fn SpriteCB_HallOfFameMonitor(sprite: *mut Sprite) {
    if gTasks[(*sprite).data[0]].data[15] != 0 {
        if (*sprite).data[1] == 0
            || ({
                (*sprite).data[1] -= 1;
                (*sprite).data[1]
            }) == 0
        {
            (*sprite).data[1] = 16;
            (*sprite).set_invisible((*sprite).invisible() ^ 1);
        }
        (*sprite).data[2] += 1;
    }
    if (*sprite).data[2] > 127 {
        FieldEffectFreeGraphicsResources(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReturnToFieldFromFlyMapSelect() {
    SetMainCallback2(Some(CB2_ReturnToField));
    gFieldCallback = Some(FieldCallback_UseFly);
}
pub(crate) unsafe extern "C" fn FieldCallback_UseFly() {
    FadeInFromBlack();
    CreateTask(Some(Task_UseFly), 0);
    LockPlayerFieldControls();
    FreezeObjectEvents();
    gFieldCallback = None;
}
pub(crate) unsafe extern "C" fn Task_UseFly(taskId: u8) {
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[taskId];
    if (*task).data[0] == 0 {
        if IsWeatherNotFadingIn() == 0 {
            return;
        }
        gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
        if gFieldEffectArguments[0] > 5 {
            gFieldEffectArguments[0] = 0;
        }
        FieldEffectStart(FLDEFF_USE_FLY);
        (*task).data[0] += 1;
    }
    if FieldEffectActiveListContains(FLDEFF_USE_FLY) == 0 {
        Overworld_ResetStateAfterFly();
        WarpIntoMap();
        SetMainCallback2(Some(CB2_LoadMap));
        gFieldCallback = Some(FieldCallback_FlyIntoMap);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_FlyIntoMap() {
    Overworld_PlaySpecialMapMusic();
    FadeInFromBlack();
    CreateTask(Some(Task_FlyIntoMap), 0);
    gObjectEvents[gPlayerAvatar.objectEventId].set_invisible(TRUE as u32);
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0 {
        ObjectEventTurn(
            &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
            DIR_WEST,
        );
    }
    LockPlayerFieldControls();
    FreezeObjectEvents();
    gFieldCallback = None;
}
pub(crate) unsafe extern "C" fn Task_FlyIntoMap(taskId: u8) {
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[taskId];
    if (*task).data[0] == 0 {
        if gPaletteFade.active() != 0 {
            return;
        }
        FieldEffectStart(FLDEFF_FLY_IN);
        (*task).data[0] += 1;
    }
    if FieldEffectActiveListContains(FLDEFF_FLY_IN) == 0 {
        UnlockPlayerFieldControls();
        UnfreezeObjectEvents();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_FallWarpExit() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    LockPlayerFieldControls();
    FreezeObjectEvents();
    CreateTask(Some(Task_FallWarpFieldEffect), 0);
    gFieldCallback = None;
}
pub(crate) unsafe extern "C" fn Task_FallWarpFieldEffect(taskId: u8) {
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[taskId];
    while sFallWarpFieldEffectFuncs[(*task).data[0]].unwrap_unchecked()(task) != 0 {}
}
pub(crate) unsafe extern "C" fn FallWarpEffect_Init(task: *mut Task) -> u8 {
    let mut playerObject: *mut ObjectEvent = null_mut();
    let mut playerSprite: *mut Sprite = null_mut();
    playerObject = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    playerSprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    CameraObjectFreeze();
    gObjectEvents[gPlayerAvatar.objectEventId].set_invisible(TRUE as u32);
    gPlayerAvatar.preventStep = TRUE;
    ObjectEventSetHeldMovement(
        playerObject,
        GetFaceDirectionMovementAction(GetPlayerFacingDirection() as u32),
    );
    (*task).data[4] = (*playerSprite).subspriteMode() as i16;
    (*playerObject).set_fixedPriority(1);
    (*playerSprite).oam.set_priority(1);
    (*playerSprite).set_subspriteMode(SUBSPRITES_IGNORE_PRIORITY);
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn FallWarpEffect_WaitWeather(task: *mut Task) -> u8 {
    if IsWeatherNotFadingIn() != 0 {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FallWarpEffect_StartFall(task: *mut Task) -> u8 {
    let mut sprite: *mut Sprite = null_mut();
    let mut centerToCornerVecY: i16 = 0;
    sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    centerToCornerVecY = -(((*sprite).centerToCornerVecY as i16) << 1);
    (*sprite).y2 = -((*sprite).y
        + (*sprite).centerToCornerVecY as i16
        + gSpriteCoordOffsetY
        + centerToCornerVecY);
    (*task).data[1] = 1;
    (*task).data[2] = 0;
    gObjectEvents[gPlayerAvatar.objectEventId].set_invisible(FALSE as u32);
    PlaySE(SE_FALL);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn FallWarpEffect_Fall(task: *mut Task) -> u8 {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    let mut sprite: *mut Sprite = null_mut();
    objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    (*sprite).y2 += (*task).data[1];
    if (*task).data[1] < 8 {
        (*task).data[2] += (*task).data[1];
        if (*task).data[2] as i32 & 0xf != 0 {
            (*task).data[1] <<= 1;
        }
    }
    if (*task).data[3] == FALSE as i16 && (*sprite).y2 >= -16 {
        (*task).data[3] += 1;
        (*objectEvent).set_fixedPriority(0);
        (*sprite).set_subspriteMode((*task).data[4] as u8);
        (*objectEvent).set_triggerGroundEffectsOnMove(1);
    }
    if (*sprite).y2 >= 0 {
        PlaySE(SE_M_STRENGTH);
        (*objectEvent).set_triggerGroundEffectsOnStop(1);
        (*objectEvent).set_landingJump(1);
        (*sprite).y2 = 0;
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FallWarpEffect_Land(task: *mut Task) -> u8 {
    (*task).data[0] += 1;
    (*task).data[1] = 4;
    (*task).data[2] = 0;
    SetCameraPanningCallback(None);
    return TRUE;
}
pub(crate) unsafe extern "C" fn FallWarpEffect_CameraShake(task: *mut Task) -> u8 {
    SetCameraPanning(0, (*task).data[1]);
    (*task).data[1] = -(*task).data[1];
    (*task).data[2] += 1;
    if (*task).data[2] as i32 & 3 == 0 {
        (*task).data[1] >>= 1;
    }
    if (*task).data[1] == 0 {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FallWarpEffect_End(task: *mut Task) -> u8 {
    gPlayerAvatar.preventStep = FALSE;
    UnlockPlayerFieldControls();
    CameraObjectReset();
    UnfreezeObjectEvents();
    InstallCameraPanAheadCallback();
    DestroyTask(FindTaskIdByFunc(Some(Task_FallWarpFieldEffect)));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartEscalatorWarp(metatileBehavior: u8, priority: u8) {
    let mut taskId: u8 = 0;
    taskId = CreateTask(Some(Task_EscalatorWarpOut), priority);
    gTasks[taskId].data[1] = FALSE as i16;
    if metatileBehavior == MB_UP_ESCALATOR {
        gTasks[taskId].data[1] = TRUE as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_EscalatorWarpOut(taskId: u8) {
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[taskId];
    while sEscalatorWarpOutFieldEffectFuncs[(*task).data[0]].unwrap_unchecked()(task) != 0 {}
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Init(task: *mut Task) -> u8 {
    FreezeObjectEvents();
    CameraObjectFreeze();
    StartEscalator((*task).data[1] as u8);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_WaitForPlayer(task: *mut Task) -> u8 {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(objectEvent) == 0
        || ObjectEventClearHeldMovementIfFinished(objectEvent) != 0
    {
        ObjectEventSetHeldMovement(
            objectEvent,
            GetFaceDirectionMovementAction(GetPlayerFacingDirection() as u32),
        );
        (*task).data[0] += 1;
        (*task).data[2] = 0;
        (*task).data[3] = 0;
        if (*task).data[1] as u8 == FALSE {
            (*task).data[0] = 4;
        }
        PlaySE(SE_ESCALATOR);
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Up_Ride(task: *mut Task) -> u8 {
    RideUpEscalatorOut(task);
    if (*task).data[2] > 3 {
        FadeOutAtEndOfEscalator();
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Up_End(task: *mut Task) -> u8 {
    RideUpEscalatorOut(task);
    WarpAtEndOfEscalator();
    return FALSE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Down_Ride(task: *mut Task) -> u8 {
    RideDownEscalatorOut(task);
    if (*task).data[2] > 3 {
        FadeOutAtEndOfEscalator();
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Down_End(task: *mut Task) -> u8 {
    RideDownEscalatorOut(task);
    WarpAtEndOfEscalator();
    return FALSE;
}
pub(crate) unsafe extern "C" fn RideUpEscalatorOut(task: *mut Task) {
    let mut sprite: *mut Sprite = null_mut();
    sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    (*sprite).x2 = Cos(0x84, (*task).data[2]);
    (*sprite).y2 = Sin(0x94, (*task).data[2]);
    (*task).data[3] += 1;
    if (*task).data[3] as i32 & 1 != 0 {
        (*task).data[2] += 1;
    }
}
pub(crate) unsafe extern "C" fn RideDownEscalatorOut(task: *mut Task) {
    let mut sprite: *mut Sprite = null_mut();
    sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    (*sprite).x2 = Cos(0x7c, (*task).data[2]);
    (*sprite).y2 = Sin(0x76, (*task).data[2]);
    (*task).data[3] += 1;
    if (*task).data[3] as i32 & 1 != 0 {
        (*task).data[2] += 1;
    }
}
pub(crate) unsafe extern "C" fn FadeOutAtEndOfEscalator() {
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
}
pub(crate) unsafe extern "C" fn WarpAtEndOfEscalator() {
    if gPaletteFade.active() == 0 && BGMusicStopped() == TRUE {
        StopEscalator();
        WarpIntoMap();
        gFieldCallback = Some(FieldCallback_EscalatorWarpIn);
        SetMainCallback2(Some(CB2_LoadMap));
        DestroyTask(FindTaskIdByFunc(Some(Task_EscalatorWarpOut)));
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_EscalatorWarpIn() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    LockPlayerFieldControls();
    CreateTask(Some(Task_EscalatorWarpIn), 0);
    gFieldCallback = None;
}
pub(crate) unsafe extern "C" fn Task_EscalatorWarpIn(taskId: u8) {
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[taskId];
    while sEscalatorWarpInFieldEffectFuncs[(*task).data[0]].unwrap_unchecked()(task) != 0 {}
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Init(task: *mut Task) -> u8 {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut behavior: u8 = 0;
    CameraObjectFreeze();
    objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    ObjectEventSetHeldMovement(objectEvent, GetFaceDirectionMovementAction(DIR_EAST as u32));
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    behavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
    (*task).data[0] += 1;
    (*task).data[1] = 16;
    if behavior == MB_DOWN_ESCALATOR {
        behavior = TRUE;
        (*task).data[0] = 3;
    } else {
        behavior = FALSE;
    }
    StartEscalator(behavior);
    return TRUE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Down_Init(task: *mut Task) -> u8 {
    let mut sprite: *mut Sprite = null_mut();
    sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    (*sprite).x2 = Cos(0x84, (*task).data[1]);
    (*sprite).y2 = Sin(0x94, (*task).data[1]);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Down_Ride(task: *mut Task) -> u8 {
    let mut sprite: *mut Sprite = null_mut();
    sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    (*sprite).x2 = Cos(0x84, (*task).data[1]);
    (*sprite).y2 = Sin(0x94, (*task).data[1]);
    (*task).data[2] += 1;
    if (*task).data[2] as i32 & 1 != 0 {
        (*task).data[1] -= 1;
    }
    if (*task).data[1] == 0 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*task).data[0] = 5;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Up_Init(task: *mut Task) -> u8 {
    let mut sprite: *mut Sprite = null_mut();
    sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    (*sprite).x2 = Cos(0x7c, (*task).data[1]);
    (*sprite).y2 = Sin(0x76, (*task).data[1]);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Up_Ride(task: *mut Task) -> u8 {
    let mut sprite: *mut Sprite = null_mut();
    sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    (*sprite).x2 = Cos(0x7c, (*task).data[1]);
    (*sprite).y2 = Sin(0x76, (*task).data[1]);
    (*task).data[2] += 1;
    if (*task).data[2] as i32 & 1 != 0 {
        (*task).data[1] -= 1;
    }
    if (*task).data[1] == 0 {
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_WaitForMovement(task: *mut Task) -> u8 {
    if IsEscalatorMoving() != 0 {
        return FALSE;
    }
    StopEscalator();
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_End(task: *mut Task) -> u8 {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventClearHeldMovementIfFinished(objectEvent) != 0 {
        CameraObjectReset();
        UnlockPlayerFieldControls();
        ObjectEventSetHeldMovement(objectEvent, GetWalkNormalMovementAction(DIR_EAST as u32));
        DestroyTask(FindTaskIdByFunc(Some(Task_EscalatorWarpIn)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseWaterfall() -> u8 {
    let mut taskId: u8 = 0;
    taskId = CreateTask(Some(Task_UseWaterfall), 0xff);
    gTasks[taskId].data[1] = gFieldEffectArguments[0] as i16;
    Task_UseWaterfall(taskId);
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_UseWaterfall(taskId: u8) {
    while sWaterfallFieldEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_Init(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    LockPlayerFieldControls();
    gPlayerAvatar.preventStep = TRUE;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_ShowMon(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    LockPlayerFieldControls();
    if ObjectEventIsMovementOverridden(objectEvent) == 0 {
        ObjectEventClearHeldMovementIfFinished(objectEvent);
        gFieldEffectArguments[0] = (*task).data[1] as i32;
        FieldEffectStart(FLDEFF_FIELD_MOVE_SHOW_MON_INIT);
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_WaitForShowMon(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    if FieldEffectActiveListContains(FLDEFF_FIELD_MOVE_SHOW_MON) != 0 {
        return FALSE;
    }
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_RideUp(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    ObjectEventSetHeldMovement(objectEvent, GetWalkSlowMovementAction(DIR_NORTH as u32));
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_ContinueRideOrEnd(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    if ObjectEventClearHeldMovementIfFinished(objectEvent) == 0 {
        return FALSE;
    }
    if MetatileBehavior_IsWaterfall((*objectEvent).currentMetatileBehavior) != 0 {
        (*task).data[0] = 3;
        return TRUE;
    }
    UnlockPlayerFieldControls();
    gPlayerAvatar.preventStep = FALSE;
    DestroyTask(FindTaskIdByFunc(Some(Task_UseWaterfall)));
    FieldEffectActiveListRemove(FLDEFF_USE_WATERFALL);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseDive() -> u8 {
    let mut taskId: u8 = 0;
    taskId = CreateTask(Some(Task_UseDive), 0xff);
    gTasks[taskId].data[15] = gFieldEffectArguments[0] as i16;
    gTasks[taskId].data[14] = gFieldEffectArguments[1] as i16;
    Task_UseDive(taskId);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_UseDive(taskId: u8) {
    while sDiveFieldEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn DiveFieldEffect_Init(task: *mut Task) -> u8 {
    gPlayerAvatar.preventStep = TRUE;
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn DiveFieldEffect_ShowMon(task: *mut Task) -> u8 {
    LockPlayerFieldControls();
    gFieldEffectArguments[0] = (*task).data[15] as i32;
    FieldEffectStart(FLDEFF_FIELD_MOVE_SHOW_MON_INIT);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn DiveFieldEffect_TryWarp(task: *mut Task) -> u8 {
    let mut mapPosition: MapPosition = zeroed();
    PlayerGetDestCoords(&raw mut mapPosition.x, &raw mut mapPosition.y);
    if FieldEffectActiveListContains(FLDEFF_FIELD_MOVE_SHOW_MON) == 0 {
        TryDoDiveWarp(
            &raw mut mapPosition,
            gObjectEvents[gPlayerAvatar.objectEventId].currentMetatileBehavior as u16,
        );
        DestroyTask(FindTaskIdByFunc(Some(Task_UseDive)));
        FieldEffectActiveListRemove(FLDEFF_USE_DIVE);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartLavaridgeGymB1FWarp(priority: u8) {
    CreateTask(Some(Task_LavaridgeGymB1FWarp), priority);
}
pub(crate) unsafe extern "C" fn Task_LavaridgeGymB1FWarp(taskId: u8) {
    while sLavaridgeGymB1FWarpEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        &raw mut gSprites[gPlayerAvatar.spriteId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_Init(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    FreezeObjectEvents();
    CameraObjectFreeze();
    SetCameraPanningCallback(None);
    gPlayerAvatar.preventStep = TRUE;
    (*objectEvent).set_fixedPriority(1);
    (*task).data[1] = 1;
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_CameraShake(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    SetCameraPanning(0, (*task).data[1]);
    (*task).data[1] = -(*task).data[1];
    (*task).data[2] += 1;
    if (*task).data[2] > 7 {
        (*task).data[2] = 0;
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_Launch(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    (*sprite).y2 = 0;
    (*task).data[3] = 1;
    gFieldEffectArguments[0] = (*objectEvent).currentCoords.x as i32;
    gFieldEffectArguments[1] = (*objectEvent).currentCoords.y as i32;
    gFieldEffectArguments[2] = (*sprite).subpriority as i32 - 1;
    gFieldEffectArguments[3] = (*sprite).oam.priority() as i32;
    FieldEffectStart(FLDEFF_ASH_LAUNCH);
    PlaySE(SE_M_EXPLOSION);
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_Rise(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    let mut centerToCornerVecY: i16 = 0;
    SetCameraPanning(0, (*task).data[1]);
    if {
        let _ = {
            (*task).data[1] = -(*task).data[1];
            (*task).data[1]
        };
        ({
            (*task).data[2] += 1;
            (*task).data[2]
        }) <= 17
    } {
        if (*task).data[2] as i32 & 1 == 0 && (*task).data[1] <= 3 {
            (*task).data[1] <<= 1;
        }
    } else if (*task).data[2] as i32 & 4 == 0 && (*task).data[1] > 0 {
        (*task).data[1] >>= 1;
    }
    if (*task).data[2] > 6 {
        centerToCornerVecY = -(((*sprite).centerToCornerVecY as i16) << 1);
        if (*sprite).y2 as i32
            > -((*sprite).y as i32
                + (*sprite).centerToCornerVecY as i32
                + gSpriteCoordOffsetY as i32
                + centerToCornerVecY as i32)
        {
            (*sprite).y2 -= (*task).data[3];
            if (*task).data[3] <= 7 {
                (*task).data[3] += 1;
            }
        } else {
            (*task).data[4] = 1;
        }
    }
    if (*task).data[5] == 0 && (*sprite).y2 < -16 {
        (*task).data[5] += 1;
        (*objectEvent).set_fixedPriority(1);
        (*sprite).oam.set_priority(1);
        (*sprite).set_subspriteMode(SUBSPRITES_IGNORE_PRIORITY);
    }
    if (*task).data[1] == 0 && (*task).data[4] != 0 {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_FadeOut(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_Warp(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    if gPaletteFade.active() == 0 && BGMusicStopped() == TRUE {
        WarpIntoMap();
        gFieldCallback = Some(FieldCB_LavaridgeGymB1FWarpExit);
        SetMainCallback2(Some(CB2_LoadMap));
        DestroyTask(FindTaskIdByFunc(Some(Task_LavaridgeGymB1FWarp)));
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FieldCB_LavaridgeGymB1FWarpExit() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    LockPlayerFieldControls();
    gFieldCallback = None;
    CreateTask(Some(Task_LavaridgeGymB1FWarpExit), 0);
}
pub(crate) unsafe extern "C" fn Task_LavaridgeGymB1FWarpExit(taskId: u8) {
    while sLavaridgeGymB1FWarpExitEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        &raw mut gSprites[gPlayerAvatar.spriteId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpExitEffect_Init(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    CameraObjectFreeze();
    FreezeObjectEvents();
    gPlayerAvatar.preventStep = TRUE;
    (*objectEvent).set_invisible(TRUE as u32);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpExitEffect_StartPopOut(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    if IsWeatherNotFadingIn() != 0 {
        gFieldEffectArguments[0] = (*objectEvent).currentCoords.x as i32;
        gFieldEffectArguments[1] = (*objectEvent).currentCoords.y as i32;
        gFieldEffectArguments[2] = (*sprite).subpriority as i32 - 1;
        gFieldEffectArguments[3] = (*sprite).oam.priority() as i32;
        (*task).data[1] = FieldEffectStart(FLDEFF_ASH_PUFF) as i16;
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpExitEffect_PopOut(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    mut sprite: *mut Sprite,
) -> u8 {
    sprite = &raw mut gSprites[(*task).data[1]];
    if (*sprite).animCmdIndex > 1 {
        (*task).data[0] += 1;
        (*objectEvent).set_invisible(FALSE as u32);
        CameraObjectReset();
        PlaySE(SE_M_DIG);
        ObjectEventSetHeldMovement(objectEvent, GetJumpMovementAction(DIR_EAST as u32));
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpExitEffect_End(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    if ObjectEventClearHeldMovementIfFinished(objectEvent) != 0 {
        gPlayerAvatar.preventStep = FALSE;
        UnlockPlayerFieldControls();
        UnfreezeObjectEvents();
        DestroyTask(FindTaskIdByFunc(Some(Task_LavaridgeGymB1FWarpExit)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_AshLaunch() -> u8 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[33],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    gSprites[spriteId]
        .oam
        .set_priority(gFieldEffectArguments[3] as u16);
    gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_AshLaunch(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, FLDEFF_ASH_LAUNCH);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartLavaridgeGym1FWarp(priority: u8) {
    CreateTask(Some(Task_LavaridgeGym1FWarp), priority);
}
pub(crate) unsafe extern "C" fn Task_LavaridgeGym1FWarp(taskId: u8) {
    while sLavaridgeGym1FWarpEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        &raw mut gSprites[gPlayerAvatar.spriteId],
    ) != 0
    {}
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_Init(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    FreezeObjectEvents();
    CameraObjectFreeze();
    gPlayerAvatar.preventStep = TRUE;
    (*objectEvent).set_fixedPriority(1);
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_AshPuff(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    if ObjectEventClearHeldMovementIfFinished(objectEvent) != 0 {
        if (*task).data[1] > 3 {
            gFieldEffectArguments[0] = (*objectEvent).currentCoords.x as i32;
            gFieldEffectArguments[1] = (*objectEvent).currentCoords.y as i32;
            gFieldEffectArguments[2] = (*sprite).subpriority as i32 - 1;
            gFieldEffectArguments[3] = (*sprite).oam.priority() as i32;
            (*task).data[1] = FieldEffectStart(FLDEFF_ASH_PUFF) as i16;
            (*task).data[0] += 1;
        } else {
            (*task).data[1] += 1;
            ObjectEventSetHeldMovement(
                objectEvent,
                GetWalkInPlaceFasterMovementAction((*objectEvent).facingDirection() as u32),
            );
            PlaySE(SE_LAVARIDGE_FALL_WARP);
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_Disappear(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    if gSprites[(*task).data[1]].animCmdIndex == 2 {
        (*objectEvent).set_invisible(TRUE as u32);
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_FadeOut(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    if FieldEffectActiveListContains(FLDEFF_ASH_PUFF) == 0 {
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_Warp(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
) -> u8 {
    if gPaletteFade.active() == 0 && BGMusicStopped() == TRUE {
        WarpIntoMap();
        gFieldCallback = Some(FieldCB_FallWarpExit);
        SetMainCallback2(Some(CB2_LoadMap));
        DestroyTask(FindTaskIdByFunc(Some(Task_LavaridgeGym1FWarp)));
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_AshPuff() -> u8 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[32],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    gSprites[spriteId]
        .oam
        .set_priority(gFieldEffectArguments[3] as u16);
    gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_AshPuff(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, FLDEFF_ASH_PUFF);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartEscapeRopeFieldEffect() {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    CreateTask(Some(Task_EscapeRopeWarpOut), 80);
}
pub(crate) unsafe extern "C" fn Task_EscapeRopeWarpOut(taskId: u8) {
    sEscapeRopeWarpOutEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    );
}
pub(crate) unsafe extern "C" fn EscapeRopeWarpOutEffect_Init(task: *mut Task) {
    (*task).data[0] += 1;
    (*task).data[14] = 64;
    (*task).data[15] = GetPlayerFacingDirection() as i16;
}
pub(crate) unsafe extern "C" fn EscapeRopeWarpOutEffect_Spin(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    let mut spinDirections: CArray<u8, 5> = CArray([1, 3, 4, 2, 1]);
    if (*task).data[14] != 0
        && ({
            (*task).data[14] -= 1;
            (*task).data[14]
        }) == 0
    {
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
    }
    objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(objectEvent) == 0
        || ObjectEventClearHeldMovementIfFinished(objectEvent) != 0
    {
        if (*task).data[14] == 0 && gPaletteFade.active() == 0 && BGMusicStopped() == TRUE {
            SetObjectEventDirection(objectEvent, (*task).data[15] as u8);
            SetWarpDestinationToEscapeWarp();
            WarpIntoMap();
            gFieldCallback = Some(FieldCallback_EscapeRopeWarpIn);
            SetMainCallback2(Some(CB2_LoadMap));
            DestroyTask(FindTaskIdByFunc(Some(Task_EscapeRopeWarpOut)));
        } else if (*task).data[1] == 0
            || ({
                (*task).data[1] -= 1;
                (*task).data[1]
            }) == 0
        {
            ObjectEventSetHeldMovement(
                objectEvent,
                GetFaceDirectionMovementAction(
                    spinDirections[(*objectEvent).facingDirection()] as u32,
                ),
            );
            if (*task).data[2] < 12 {
                (*task).data[2] += 1;
            }
            (*task).data[1] = shr_i32(8, ((*task).data[2] >> 2) as u32) as i16;
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_EscapeRopeWarpIn() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    LockPlayerFieldControls();
    FreezeObjectEvents();
    gFieldCallback = None;
    gObjectEvents[gPlayerAvatar.objectEventId].set_invisible(TRUE as u32);
    CreateTask(Some(Task_EscapeRopeWarpIn), 0);
}
pub(crate) unsafe extern "C" fn Task_EscapeRopeWarpIn(taskId: u8) {
    sEscapeRopeWarpInEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    );
}
pub(crate) unsafe extern "C" fn EscapeRopeWarpInEffect_Init(task: *mut Task) {
    if IsWeatherNotFadingIn() != 0 {
        (*task).data[0] += 1;
        (*task).data[15] = GetPlayerFacingDirection() as i16;
    }
}
pub(crate) unsafe extern "C" fn EscapeRopeWarpInEffect_Spin(task: *mut Task) {
    let mut spinDirections: CArray<u8, 5> = CArray([1, 3, 4, 2, 1]);
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if (*task).data[1] == 0
        || ({
            (*task).data[1] -= 1;
            (*task).data[1]
        }) == 0
    {
        if ObjectEventIsMovementOverridden(objectEvent) != 0
            && ObjectEventClearHeldMovementIfFinished(objectEvent) == 0
        {
            return;
        }
        if (*task).data[2] >= 32 && (*task).data[15] == GetPlayerFacingDirection() as i16 {
            (*objectEvent).set_invisible(FALSE as u32);
            UnlockPlayerFieldControls();
            UnfreezeObjectEvents();
            DestroyTask(FindTaskIdByFunc(Some(Task_EscapeRopeWarpIn)));
            return;
        }
        ObjectEventSetHeldMovement(
            objectEvent,
            GetFaceDirectionMovementAction(spinDirections[(*objectEvent).facingDirection()] as u32),
        );
        if (*task).data[2] < 32 {
            (*task).data[2] += 1;
        }
        (*task).data[1] = (*task).data[2] >> 2;
    }
    (*objectEvent).set_invisible((*objectEvent).invisible() ^ 1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_TeleportWarpOut() {
    CreateTask(Some(Task_TeleportWarpOut), 0);
}
pub(crate) unsafe extern "C" fn Task_TeleportWarpOut(taskId: u8) {
    sTeleportWarpOutFieldEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    );
}
pub(crate) unsafe extern "C" fn TeleportWarpOutFieldEffect_Init(task: *mut Task) {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    CameraObjectFreeze();
    (*task).data[15] = GetPlayerFacingDirection() as i16;
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn TeleportWarpOutFieldEffect_SpinGround(task: *mut Task) {
    let mut spinDirections: CArray<u8, 5> = CArray([1, 3, 4, 2, 1]);
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if (*task).data[1] == 0
        || ({
            (*task).data[1] -= 1;
            (*task).data[1]
        }) == 0
    {
        ObjectEventTurn(
            objectEvent,
            spinDirections[(*objectEvent).facingDirection()],
        );
        (*task).data[1] = 8;
        (*task).data[2] += 1;
    }
    if (*task).data[2] > 7 && (*task).data[15] as i32 == (*objectEvent).facingDirection() as i32 {
        (*task).data[0] += 1;
        (*task).data[1] = 4;
        (*task).data[2] = 8;
        (*task).data[3] = 1;
        PlaySE(SE_WARP_IN);
    }
}
pub(crate) unsafe extern "C" fn TeleportWarpOutFieldEffect_SpinExit(task: *mut Task) {
    let mut spinDirections: CArray<u8, 5> = CArray([1, 3, 4, 2, 1]);
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    let mut sprite: *mut Sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    if ({
        (*task).data[1] -= 1;
        (*task).data[1]
    }) <= 0
    {
        (*task).data[1] = 4;
        ObjectEventTurn(
            objectEvent,
            spinDirections[(*objectEvent).facingDirection()],
        );
    }
    (*sprite).y -= (*task).data[3];
    (*task).data[4] += (*task).data[3];
    if ({
        (*task).data[2] -= 1;
        (*task).data[2]
    }) <= 0
        && ({
            let _ = {
                (*task).data[2] = 4;
                (*task).data[2]
            };
            (*task).data[3] < 8
        })
    {
        (*task).data[3] <<= 1;
    }
    if (*task).data[4] > 8
        && ({
            let _ = {
                (*sprite).oam.set_priority(1);
                (*sprite).oam.priority()
            };
            (*sprite).subspriteMode() != SUBSPRITES_OFF
        })
    {
        (*sprite).set_subspriteMode(SUBSPRITES_IGNORE_PRIORITY);
    }
    if (*task).data[4] >= 168 {
        (*task).data[0] += 1;
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
    }
}
pub(crate) unsafe extern "C" fn TeleportWarpOutFieldEffect_End(task: *mut Task) {
    if gPaletteFade.active() == 0 {
        if (*task).data[5] == FALSE as i16 {
            ClearMirageTowerPulseBlendEffect();
            (*task).data[5] = TRUE as i16;
        }
        if BGMusicStopped() == TRUE {
            SetWarpDestinationToLastHealLocation();
            WarpIntoMap();
            SetMainCallback2(Some(CB2_LoadMap));
            gFieldCallback = Some(FieldCallback_TeleportWarpIn);
            DestroyTask(FindTaskIdByFunc(Some(Task_TeleportWarpOut)));
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_TeleportWarpIn() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    LockPlayerFieldControls();
    FreezeObjectEvents();
    gFieldCallback = None;
    gObjectEvents[gPlayerAvatar.objectEventId].set_invisible(TRUE as u32);
    CameraObjectFreeze();
    CreateTask(Some(Task_TeleportWarpIn), 0);
}
pub(crate) unsafe extern "C" fn Task_TeleportWarpIn(taskId: u8) {
    sTeleportWarpInFieldEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    );
}
pub(crate) unsafe extern "C" fn TeleportWarpInFieldEffect_Init(task: *mut Task) {
    let mut sprite: *mut Sprite = null_mut();
    let mut centerToCornerVecY: i16 = 0;
    if IsWeatherNotFadingIn() != 0 {
        sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
        centerToCornerVecY = -(((*sprite).centerToCornerVecY as i16) << 1);
        (*sprite).y2 = -((*sprite).y
            + (*sprite).centerToCornerVecY as i16
            + gSpriteCoordOffsetY
            + centerToCornerVecY);
        gObjectEvents[gPlayerAvatar.objectEventId].set_invisible(FALSE as u32);
        (*task).data[0] += 1;
        (*task).data[1] = 8;
        (*task).data[2] = 1;
        (*task).data[14] = (*sprite).subspriteMode() as i16;
        (*task).data[15] = GetPlayerFacingDirection() as i16;
        PlaySE(SE_WARP_IN);
    }
}
pub(crate) unsafe extern "C" fn TeleportWarpInFieldEffect_SpinEnter(task: *mut Task) {
    let mut spinDirections: CArray<u8, 5> = CArray([1, 3, 4, 2, 1]);
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    let mut sprite: *mut Sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    if ({
        (*sprite).y2 += (*task).data[1];
        (*sprite).y2
    }) >= -8
    {
        if (*task).data[13] == 0 {
            (*task).data[13] += 1;
            (*objectEvent).set_triggerGroundEffectsOnMove(TRUE as u32);
            (*sprite).set_subspriteMode((*task).data[14] as u8);
        }
    } else {
        (*sprite).oam.set_priority(1);
        if (*sprite).subspriteMode() != SUBSPRITES_OFF {
            (*sprite).set_subspriteMode(SUBSPRITES_IGNORE_PRIORITY);
        }
    }
    if (*sprite).y2 >= -48 && (*task).data[1] > 1 && (*sprite).y2 as i32 & 1 == 0 {
        (*task).data[1] -= 1;
    }
    if ({
        (*task).data[2] -= 1;
        (*task).data[2]
    }) == 0
    {
        (*task).data[2] = 4;
        ObjectEventTurn(
            objectEvent,
            spinDirections[(*objectEvent).facingDirection()],
        );
    }
    if (*sprite).y2 >= 0 {
        (*sprite).y2 = 0;
        (*task).data[0] += 1;
        (*task).data[1] = 1;
        (*task).data[2] = 0;
    }
}
pub(crate) unsafe extern "C" fn TeleportWarpInFieldEffect_SpinGround(task: *mut Task) {
    let mut spinDirections: CArray<u8, 5> = CArray([1, 3, 4, 2, 1]);
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ({
        (*task).data[1] -= 1;
        (*task).data[1]
    }) == 0
    {
        ObjectEventTurn(
            objectEvent,
            spinDirections[(*objectEvent).facingDirection()],
        );
        (*task).data[1] = 8;
        if ({
            (*task).data[2] += 1;
            (*task).data[2]
        }) > 4
            && (*task).data[14] as i32 == (*objectEvent).facingDirection() as i32
        {
            UnlockPlayerFieldControls();
            CameraObjectReset();
            UnfreezeObjectEvents();
            DestroyTask(FindTaskIdByFunc(Some(Task_TeleportWarpIn)));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_FieldMoveShowMon() -> u8 {
    let mut taskId: u8 = 0;
    if IsMapTypeOutdoors(GetCurrentMapType()) == TRUE {
        taskId = CreateTask(Some(Task_FieldMoveShowMonOutdoors), 0xff);
    } else {
        taskId = CreateTask(Some(Task_FieldMoveShowMonIndoors), 0xff);
    }
    gTasks[taskId].data[15] = InitFieldMoveMonSprite(
        gFieldEffectArguments[0] as u32,
        gFieldEffectArguments[1] as u32,
        gFieldEffectArguments[2] as u32,
    ) as i16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_FieldMoveShowMonInit() -> u8 {
    let mut pokemon: *mut Pokemon = null_mut();
    let mut noDucking: u32 = gFieldEffectArguments[0] as u32 & 0x80000000;
    pokemon = &raw mut gPlayerParty[gFieldEffectArguments[0] as u8];
    gFieldEffectArguments[0] = GetMonData2(pokemon, MON_DATA_SPECIES) as i32;
    gFieldEffectArguments[1] = GetMonData2(pokemon, MON_DATA_OT_ID) as i32;
    gFieldEffectArguments[2] = GetMonData2(pokemon, MON_DATA_PERSONALITY) as i32;
    gFieldEffectArguments[0] |= noDucking as i32;
    FieldEffectStart(FLDEFF_FIELD_MOVE_SHOW_MON);
    FieldEffectActiveListRemove(FLDEFF_FIELD_MOVE_SHOW_MON_INIT);
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_FieldMoveShowMonOutdoors(taskId: u8) {
    sFieldMoveShowMonOutdoorsEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    );
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_Init(task: *mut Task) {
    (*task).data[11] = (67108936 as usize as *mut u16).read_volatile() as i16;
    (*task).data[12] = (67108938 as usize as *mut u16).read_volatile() as i16;
    StoreWordInTwoHalfwords(
        &raw mut (*task).data[13] as *mut u16,
        core::mem::transmute::<_, usize>(gMain.vblankCallback) as u32,
    );
    (*task).data[1] = -3855;
    (*task).data[2] = 20561;
    (*task).data[3] = 63;
    (*task).data[4] = 62;
    SetGpuReg(REG_OFFSET_WIN0H, (*task).data[1] as u16);
    SetGpuReg(REG_OFFSET_WIN0V, (*task).data[2] as u16);
    SetGpuReg(REG_OFFSET_WININ, (*task).data[3] as u16);
    SetGpuReg(REG_OFFSET_WINOUT, (*task).data[4] as u16);
    SetVBlankCallback(Some(VBlankCB_FieldMoveShowMonOutdoors));
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_LoadGfx(task: *mut Task) {
    let mut offset: u16 = (67108872 as usize as *mut u16).read_volatile() >> 2 << 14;
    let mut delta: u16 = (67108872 as usize as *mut u16).read_volatile() >> 8 << 11;
    CpuSet(
        sFieldMoveStreaksOutdoors_Gfx.as_ptr().cast_mut() as *mut c_void,
        (VRAM + offset as i32) as usize as *mut c_void,
        256,
    );
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (VRAM + delta as i32) as usize as *mut c_void,
                0x5000200,
            );
        }
    }
    LoadPalette(
        sFieldMoveStreaksOutdoors_Pal.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    LoadFieldMoveOutdoorStreaksTilemap(delta);
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_CreateBanner(task: *mut Task) {
    let mut horiz: i16 = 0;
    let mut vertHi: i16 = 0;
    let mut vertLo: i16 = 0;
    (*task).data[5] -= 16;
    horiz = ((*task).data[1] as u16 >> 8) as i16;
    vertHi = ((*task).data[2] as u16 >> 8) as i16;
    vertLo = (*task).data[2] as u16 as i16 & 0xff;
    horiz -= 16;
    vertHi -= 2;
    vertLo += 2;
    if horiz < 0 {
        horiz = 0;
    }
    if vertHi < 40 {
        vertHi = 40;
    }
    if vertLo > 120 {
        vertLo = 120;
    }
    (*task).data[1] = horiz << 8 | (*task).data[1] & 0xff;
    (*task).data[2] = vertHi << 8 | vertLo;
    if horiz == 0 && vertHi == 40 && vertLo == 120 {
        gSprites[(*task).data[15]].callback = Some(SpriteCB_FieldMoveMonSlideOnscreen);
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_WaitForMon(task: *mut Task) {
    (*task).data[5] -= 16;
    if gSprites[(*task).data[15]].data[7] != 0 {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_ShrinkBanner(task: *mut Task) {
    let mut vertHi: i16 = 0;
    let mut vertLo: i16 = 0;
    (*task).data[5] -= 16;
    vertHi = (*task).data[2] >> 8;
    vertLo = (*task).data[2] & 0xFF;
    vertHi += 6;
    vertLo -= 6;
    if vertHi > 80 {
        vertHi = 80;
    }
    if vertLo < 81 {
        vertLo = 81;
    }
    (*task).data[2] = vertHi << 8 | vertLo;
    if vertHi == 80 && vertLo == 81 {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_RestoreBg(task: *mut Task) {
    let mut bg0cnt: u16 = (67108872 as usize as *mut u16).read_volatile() >> 8 << 11;
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (VRAM as usize as *mut c_void as *mut u8).at(bg0cnt) as *mut c_void,
                0x5000200,
            );
        }
    }
    (*task).data[1] = 241;
    (*task).data[2] = 161;
    (*task).data[3] = (*task).data[11];
    (*task).data[4] = (*task).data[12];
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_End(task: *mut Task) {
    let mut callback: Option<unsafe extern "C" fn()> = None;
    LoadWordFromTwoHalfwords(
        &raw mut (*task).data[13] as *mut u16,
        &raw mut callback as *mut u32,
    );
    SetVBlankCallback(callback);
    InitTextBoxGfxAndPrinters();
    FreeResourcesAndDestroySprite(&raw mut gSprites[(*task).data[15]], (*task).data[15] as u8);
    FieldEffectActiveListRemove(FLDEFF_FIELD_MOVE_SHOW_MON);
    DestroyTask(FindTaskIdByFunc(Some(Task_FieldMoveShowMonOutdoors)));
}
pub(crate) unsafe extern "C" fn VBlankCB_FieldMoveShowMonOutdoors() {
    let mut callback: Option<unsafe extern "C" fn()> = None;
    let mut task: *mut Task =
        &raw mut gTasks[FindTaskIdByFunc(Some(Task_FieldMoveShowMonOutdoors))];
    LoadWordFromTwoHalfwords(
        &raw mut (*task).data[13] as *mut u16,
        &raw mut callback as *mut u32,
    );
    callback.unwrap_unchecked()();
    SetGpuReg(REG_OFFSET_WIN0H, (*task).data[1] as u16);
    SetGpuReg(REG_OFFSET_WIN0V, (*task).data[2] as u16);
    SetGpuReg(REG_OFFSET_WININ, (*task).data[3] as u16);
    SetGpuReg(REG_OFFSET_WINOUT, (*task).data[4] as u16);
    SetGpuReg(REG_OFFSET_BG0HOFS, (*task).data[5] as u16);
    SetGpuReg(REG_OFFSET_BG0VOFS, (*task).data[6] as u16);
}
pub(crate) unsafe extern "C" fn LoadFieldMoveOutdoorStreaksTilemap(offs: u16) {
    let mut i: u16 = 0;
    let mut dest: *mut u16 = null_mut();
    dest = (0x6000140 + offs as u32) as usize as *mut u16;
    i = 0;
    while i < 320 {
        *dest = sFieldMoveStreaksOutdoors_Tilemap[i] | 0xF000;
        i += 1;
        dest = dest.at(1);
    }
}
pub(crate) unsafe extern "C" fn Task_FieldMoveShowMonIndoors(taskId: u8) {
    sFieldMoveShowMonIndoorsEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
    );
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_Init(task: *mut Task) {
    SetGpuReg(REG_OFFSET_BG0HOFS, (*task).data[1] as u16);
    SetGpuReg(REG_OFFSET_BG0VOFS, (*task).data[2] as u16);
    StoreWordInTwoHalfwords(
        &raw mut (*task).data[13] as *mut u16,
        core::mem::transmute::<_, usize>(gMain.vblankCallback) as u32,
    );
    SetVBlankCallback(Some(VBlankCB_FieldMoveShowMonIndoors));
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_LoadGfx(task: *mut Task) {
    let mut offset: u16 = 0;
    let mut delta: u16 = 0;
    offset = (67108872 as usize as *mut u16).read_volatile() >> 2 << 14;
    delta = (67108872 as usize as *mut u16).read_volatile() >> 8 << 11;
    (*task).data[12] = delta as i16;
    CpuSet(
        sFieldMoveStreaksIndoors_Gfx.as_ptr().cast_mut() as *mut c_void,
        (VRAM + offset as i32) as usize as *mut c_void,
        64,
    );
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (VRAM + delta as i32) as usize as *mut c_void,
                0x5000200,
            );
        }
    }
    LoadPalette(
        sFieldMoveStreaksIndoors_Pal.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_SlideBannerOn(task: *mut Task) {
    if SlideIndoorBannerOnscreen(task) != 0 {
        SetGpuReg(REG_OFFSET_WIN1H, DISPLAY_WIDTH);
        SetGpuReg(REG_OFFSET_WIN1V, 10360);
        gSprites[(*task).data[15]].callback = Some(SpriteCB_FieldMoveMonSlideOnscreen);
        (*task).data[0] += 1;
    }
    AnimateIndoorShowMonBg(task);
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_WaitForMon(task: *mut Task) {
    AnimateIndoorShowMonBg(task);
    if gSprites[(*task).data[15]].data[7] != 0 {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_RestoreBg(task: *mut Task) {
    AnimateIndoorShowMonBg(task);
    (*task).data[3] = (*task).data[1] & 7;
    (*task).data[4] = 0;
    SetGpuReg(REG_OFFSET_WIN1H, 65535);
    SetGpuReg(REG_OFFSET_WIN1V, 65535);
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_SlideBannerOff(task: *mut Task) {
    AnimateIndoorShowMonBg(task);
    if SlideIndoorBannerOffscreen(task) != 0 {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_End(task: *mut Task) {
    let mut intrCallback: Option<unsafe extern "C" fn()> = None;
    let mut bg0cnt: u16 = 0;
    bg0cnt = (67108872 as usize as *mut u16).read_volatile() >> 8 << 11;
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (VRAM as usize as *mut c_void as *mut u8).at(bg0cnt) as *mut c_void,
                0x5000200,
            );
        }
    }
    LoadWordFromTwoHalfwords(
        &raw mut (*task).data[13] as *mut u16,
        &raw mut intrCallback as *mut u32,
    );
    SetVBlankCallback(intrCallback);
    InitTextBoxGfxAndPrinters();
    FreeResourcesAndDestroySprite(&raw mut gSprites[(*task).data[15]], (*task).data[15] as u8);
    FieldEffectActiveListRemove(FLDEFF_FIELD_MOVE_SHOW_MON);
    DestroyTask(FindTaskIdByFunc(Some(Task_FieldMoveShowMonIndoors)));
}
pub(crate) unsafe extern "C" fn VBlankCB_FieldMoveShowMonIndoors() {
    let mut intrCallback: Option<unsafe extern "C" fn()> = None;
    let mut task: *mut Task = null_mut();
    task = &raw mut gTasks[FindTaskIdByFunc(Some(Task_FieldMoveShowMonIndoors))];
    LoadWordFromTwoHalfwords(
        &raw mut (*task).data[13] as *mut u16,
        &raw mut intrCallback as *mut u32,
    );
    intrCallback.unwrap_unchecked()();
    SetGpuReg(REG_OFFSET_BG0HOFS, (*task).data[1] as u16);
    SetGpuReg(REG_OFFSET_BG0VOFS, (*task).data[2] as u16);
}
pub(crate) unsafe extern "C" fn AnimateIndoorShowMonBg(task: *mut Task) {
    (*task).data[1] -= 16;
    (*task).data[3] += 16;
}
pub(crate) unsafe extern "C" fn SlideIndoorBannerOnscreen(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    let mut srcOffs: u16 = 0;
    let mut dstOffs: u16 = 0;
    let mut dest: *mut u16 = null_mut();
    if (*task).data[4] >= 32 {
        return TRUE;
    }
    dstOffs = ((*task).data[3] >> 3) as u16 & 0x1f;
    if dstOffs as i32 >= (*task).data[4] as i32 {
        dstOffs = 32 - dstOffs & 0x1f;
        srcOffs = 32 - (*task).data[4] as u16 & 0x1f;
        dest = (100663616 + (*task).data[12] as u16 as i32) as usize as *mut u16;
        i = 0;
        while i < 10 {
            *dest.at(dstOffs as i32 + i as i32 * 32) =
                sFieldMoveStreaksIndoors_Tilemap[srcOffs as i32 + i as i32 * 32];
            *dest.at(dstOffs as i32 + i as i32 * 32) |= 0xf000;
            *dest.at((dstOffs as i32 + 1 & 0x1f) + i as i32 * 32) = sFieldMoveStreaksIndoors_Tilemap
                [(srcOffs as i32 + 1 & 0x1f) + i as i32 * 32]
                | 0xf000;
            *dest.at((dstOffs as i32 + 1 & 0x1f) + i as i32 * 32) |= 0xf000;
            i += 1;
        }
        (*task).data[4] += 2;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlideIndoorBannerOffscreen(task: *mut Task) -> u8 {
    let mut i: u16 = 0;
    let mut dstOffs: u16 = 0;
    let mut dest: *mut u16 = null_mut();
    if (*task).data[4] >= 32 {
        return TRUE;
    }
    dstOffs = ((*task).data[3] >> 3) as u16;
    if dstOffs as i32 >= (*task).data[4] as i32 {
        dstOffs = ((*task).data[1] >> 3) as u16 & 0x1f;
        dest = (100663616 + (*task).data[12] as u16 as i32) as usize as *mut u16;
        i = 0;
        while i < 10 {
            *dest.at(dstOffs as i32 + i as i32 * 32) = 0xf000;
            *dest.at((dstOffs as i32 + 1 & 0x1f) + i as i32 * 32) = 0xf000;
            i += 1;
        }
        (*task).data[4] += 2;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn InitFieldMoveMonSprite(
    mut species: u32,
    otId: u32,
    personality: u32,
) -> u8 {
    let mut noDucking: u16 = 0;
    let mut monSprite: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    noDucking = ((species & 0x80000000) >> 16) as u16;
    species &= 0x7fffffff;
    monSprite = CreateMonSprite_FieldMove(species as u16, otId, personality, 320, 80, 0);
    sprite = &raw mut gSprites[monSprite];
    (*sprite).callback = Some(SpriteCallbackDummy);
    (*sprite).oam.set_priority(0);
    (*sprite).data[0] = species as i16;
    (*sprite).data[6] = noDucking as i16;
    return monSprite;
}
pub(crate) unsafe extern "C" fn SpriteCB_FieldMoveMonSlideOnscreen(sprite: *mut Sprite) {
    if ({
        (*sprite).x -= 20;
        (*sprite).x
    }) <= 120
    {
        (*sprite).x = 120;
        (*sprite).data[1] = 30;
        (*sprite).callback = Some(SpriteCB_FieldMoveMonWaitAfterCry);
        if (*sprite).data[6] != 0 {
            PlayCry_NormalNoDucking(
                (*sprite).data[0] as u16,
                0,
                CRY_VOLUME_RS,
                CRY_PRIORITY_NORMAL,
            );
        } else {
            PlayCry_Normal((*sprite).data[0] as u16, 0);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FieldMoveMonWaitAfterCry(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] -= 1;
        (*sprite).data[1]
    }) == 0
    {
        (*sprite).callback = Some(SpriteCB_FieldMoveMonSlideOffscreen);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FieldMoveMonSlideOffscreen(sprite: *mut Sprite) {
    if (*sprite).x < -64 {
        (*sprite).data[7] = TRUE as i16;
    } else {
        (*sprite).x -= 20;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseSurf() -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_SurfFieldEffect), 0xff);
    gTasks[taskId].data[15] = gFieldEffectArguments[0] as i16;
    Overworld_ClearSavedMusic();
    Overworld_ChangeMusicTo(MUS_SURF);
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_SurfFieldEffect(taskId: u8) {
    sSurfFieldEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]);
}
pub(crate) unsafe extern "C" fn SurfFieldEffect_Init(task: *mut Task) {
    LockPlayerFieldControls();
    FreezeObjectEvents();
    gPlayerAvatar.preventStep = TRUE;
    SetPlayerAvatarStateMask(PLAYER_AVATAR_FLAG_SURFING);
    PlayerGetDestCoords(&raw mut (*task).data[1], &raw mut (*task).data[2]);
    MoveCoords(
        gObjectEvents[gPlayerAvatar.objectEventId].movementDirection() as u8,
        &raw mut (*task).data[1],
        &raw mut (*task).data[2],
    );
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn SurfFieldEffect_FieldMovePose(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(objectEvent) == 0
        || ObjectEventClearHeldMovementIfFinished(objectEvent) != 0
    {
        SetPlayerAvatarFieldMove();
        ObjectEventSetHeldMovement(objectEvent, MOVEMENT_ACTION_START_ANIM_IN_DIRECTION);
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn SurfFieldEffect_ShowMon(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventCheckHeldMovementStatus(objectEvent) != 0 {
        gFieldEffectArguments[0] = (*task).data[15] as i32 | -0x80000000;
        FieldEffectStart(FLDEFF_FIELD_MOVE_SHOW_MON_INIT);
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn SurfFieldEffect_JumpOnSurfBlob(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    if FieldEffectActiveListContains(FLDEFF_FIELD_MOVE_SHOW_MON) == 0 {
        objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        ObjectEventSetGraphicsId(
            objectEvent,
            GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_SURFING),
        );
        ObjectEventClearHeldMovementIfFinished(objectEvent);
        ObjectEventSetHeldMovement(
            objectEvent,
            GetJumpSpecialMovementAction((*objectEvent).movementDirection() as u32),
        );
        gFieldEffectArguments[0] = (*task).data[1] as i32;
        gFieldEffectArguments[1] = (*task).data[2] as i32;
        gFieldEffectArguments[2] = gPlayerAvatar.objectEventId as i32;
        (*objectEvent).fieldEffectSpriteId = FieldEffectStart(FLDEFF_SURF_BLOB) as u8;
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn SurfFieldEffect_End(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventClearHeldMovementIfFinished(objectEvent) != 0 {
        gPlayerAvatar.preventStep = FALSE;
        gPlayerAvatar.flags &= 223;
        ObjectEventSetHeldMovement(
            objectEvent,
            GetFaceDirectionMovementAction((*objectEvent).movementDirection() as u32),
        );
        SetSurfBlob_BobState((*objectEvent).fieldEffectSpriteId, BOB_PLAYER_AND_MON);
        UnfreezeObjectEvents();
        UnlockPlayerFieldControls();
        FieldEffectActiveListRemove(FLDEFF_USE_SURF);
        DestroyTask(FindTaskIdByFunc(Some(Task_SurfFieldEffect)));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_RayquazaSpotlight() -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut k: u8 = 0;
    let mut spriteId: u8 = CreateSprite(gFieldEffectObjectTemplatePointers[36], 120, -24, 1);
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(1);
    (*sprite).oam.set_paletteNum(4);
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = 0;
    (*sprite).data[3] = -1;
    (*sprite).data[4] = (*sprite).y;
    (*sprite).data[5] = 0;
    SetGpuReg(REG_OFFSET_BLDCNT, 15937);
    SetGpuReg(REG_OFFSET_BLDALPHA, 3598);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    LoadPalette(sSpotlight_Pal.as_ptr().cast_mut() as *mut c_void, 192, 32);
    SetGpuReg(REG_OFFSET_BG0VOFS, 120);
    i = 3;
    while i < 15 {
        j = 12;
        while j < 18 {
            *(0x600f800 as usize as *mut u16).at(i as i32 * 32 + j as i32) =
                0xBFF4 + i as u16 * 6 + j as u16 + 1;
            j += 1;
        }
        i += 1;
    }
    k = 0;
    while k < 90 {
        i = 0;
        while i < 8 {
            *((0x6008000 + (k as i32 + 1) * 32 + i as i32 * 4) as usize as *mut u16) =
                ((sSpotlight_Gfx[k as i32 * 32 + i as i32 * 4 + 1] as u16) << 8)
                    + sSpotlight_Gfx[k as i32 * 32 + i as i32 * 4] as u16;
            *((0x6008000 + (k as i32 + 1) * 32 + i as i32 * 4 + 2) as usize as *mut u16) =
                ((sSpotlight_Gfx[k as i32 * 32 + i as i32 * 4 + 3] as u16) << 8)
                    + sSpotlight_Gfx[k as i32 * 32 + i as i32 * 4 + 2] as u16;
            i += 1;
        }
        k += 1;
    }
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_NPCFlyOut() -> u8 {
    let mut spriteId: u8 = CreateSprite(gFieldEffectObjectTemplatePointers[26], 0x78, 0, 1);
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_paletteNum(0);
    (*sprite).oam.set_priority(1);
    (*sprite).callback = Some(SpriteCB_NPCFlyOut);
    (*sprite).data[1] = gFieldEffectArguments[0] as i16;
    PlaySE(SE_M_FLY);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_NPCFlyOut(sprite: *mut Sprite) {
    let mut npcSprite: *mut Sprite = null_mut();
    (*sprite).x2 = Cos((*sprite).data[2], 0x8c);
    (*sprite).y2 = Sin((*sprite).data[2], 0x48);
    (*sprite).data[2] = (*sprite).data[2] + 4 & 0xff;
    if (*sprite).data[0] != 0 {
        npcSprite = &raw mut gSprites[(*sprite).data[1]];
        (*npcSprite).set_coordOffsetEnabled(FALSE as u16);
        (*npcSprite).x = (*sprite).x + (*sprite).x2;
        (*npcSprite).y = (*sprite).y + (*sprite).y2 - 8;
        (*npcSprite).x2 = 0;
        (*npcSprite).y2 = 0;
    }
    if (*sprite).data[2] >= 0x80 {
        FieldEffectStop(sprite, FLDEFF_NPCFLY_OUT);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseFly() -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_FlyOut), 254);
    gTasks[taskId].data[1] = gFieldEffectArguments[0] as i16;
    return 0;
}
pub(crate) unsafe extern "C" fn Task_FlyOut(taskId: u8) {
    sFlyOutFieldEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]);
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_FieldMovePose(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(objectEvent) == 0
        || ObjectEventClearHeldMovementIfFinished(objectEvent) != 0
    {
        (*task).data[15] = gPlayerAvatar.flags as i16;
        gPlayerAvatar.preventStep = TRUE;
        SetPlayerAvatarStateMask(PLAYER_AVATAR_FLAG_ON_FOOT);
        SetPlayerAvatarFieldMove();
        ObjectEventSetHeldMovement(objectEvent, MOVEMENT_ACTION_START_ANIM_IN_DIRECTION);
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_ShowMon(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventClearHeldMovementIfFinished(objectEvent) != 0 {
        (*task).data[0] += 1;
        gFieldEffectArguments[0] = (*task).data[1] as i32;
        FieldEffectStart(FLDEFF_FIELD_MOVE_SHOW_MON_INIT);
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_BirdLeaveBall(task: *mut Task) {
    if FieldEffectActiveListContains(FLDEFF_FIELD_MOVE_SHOW_MON) == 0 {
        let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        if (*task).data[15] as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0 {
            SetSurfBlob_BobState((*objectEvent).fieldEffectSpriteId, BOB_JUST_MON);
            SetSurfBlob_DontSyncAnim((*objectEvent).fieldEffectSpriteId, FALSE);
        }
        (*task).data[1] = CreateFlyBirdSprite() as i16;
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_WaitBirdLeave(task: *mut Task) {
    if GetFlyBirdAnimCompleted((*task).data[1] as u8) != 0 {
        (*task).data[0] += 1;
        (*task).data[2] = 16;
        SetPlayerAvatarTransitionFlags(PLAYER_AVATAR_FLAG_ON_FOOT as u16);
        ObjectEventSetHeldMovement(
            &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
            MOVEMENT_ACTION_FACE_LEFT,
        );
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_BirdSwoopDown(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ((*task).data[2] == 0
        || ({
            (*task).data[2] -= 1;
            (*task).data[2]
        }) == 0)
        && ObjectEventClearHeldMovementIfFinished(objectEvent) != 0
    {
        (*task).data[0] += 1;
        PlaySE(SE_M_FLY);
        StartFlyBirdSwoopDown((*task).data[1] as u8);
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_JumpOnBird(task: *mut Task) {
    if ({
        (*task).data[2] += 1;
        (*task).data[2]
    }) >= 8
    {
        let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        ObjectEventSetGraphicsId(
            objectEvent,
            GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_SURFING),
        );
        StartSpriteAnim(
            &raw mut gSprites[(*objectEvent).spriteId],
            ANIM_GET_ON_OFF_POKEMON_WEST,
        );
        (*objectEvent).set_inanimate(TRUE as u32);
        ObjectEventSetHeldMovement(objectEvent, MOVEMENT_ACTION_JUMP_IN_PLACE_LEFT);
        if (*task).data[15] as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0 {
            DestroySprite(&raw mut gSprites[(*objectEvent).fieldEffectSpriteId]);
        }
        (*task).data[0] += 1;
        (*task).data[2] = 0;
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_FlyOffWithBird(task: *mut Task) {
    if ({
        (*task).data[2] += 1;
        (*task).data[2]
    }) >= 10
    {
        let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        ObjectEventClearHeldMovementIfActive(objectEvent);
        (*objectEvent).set_inanimate(FALSE as u32);
        (*objectEvent).set_hasShadow(FALSE as u32);
        SetFlyBirdPlayerSpriteId((*task).data[1] as u8, (*objectEvent).spriteId);
        CameraObjectFreeze();
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_WaitFlyOff(task: *mut Task) {
    if GetFlyBirdAnimCompleted((*task).data[1] as u8) != 0 {
        WarpFadeOutScreen();
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_End(task: *mut Task) {
    if gPaletteFade.active() == 0 {
        FieldEffectActiveListRemove(FLDEFF_USE_FLY);
        DestroyTask(FindTaskIdByFunc(Some(Task_FlyOut)));
    }
}
pub(crate) unsafe extern "C" fn CreateFlyBirdSprite() -> u8 {
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    spriteId = CreateSprite(gFieldEffectObjectTemplatePointers[26], 0xff, 0xb4, 0x1);
    sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_paletteNum(0);
    (*sprite).oam.set_priority(1);
    (*sprite).callback = Some(SpriteCB_FlyBirdLeaveBall);
    return spriteId;
}
pub(crate) unsafe extern "C" fn GetFlyBirdAnimCompleted(spriteId: u8) -> u8 {
    return gSprites[spriteId].data[7] as u8;
}
pub(crate) unsafe extern "C" fn StartFlyBirdSwoopDown(spriteId: u8) {
    let mut sprite: *mut Sprite = null_mut();
    sprite = &raw mut gSprites[spriteId];
    (*sprite).callback = Some(SpriteCB_FlyBirdSwoopDown);
    (*sprite).x = 120;
    (*sprite).y = 0;
    (*sprite).x2 = 0;
    (*sprite).y2 = 0;
    memset(&raw mut (*sprite).data[0] as *mut u8, 0, 16);
    (*sprite).data[6] = MAX_SPRITES as i16;
}
pub(crate) unsafe extern "C" fn SetFlyBirdPlayerSpriteId(birdSpriteId: u8, playerSpriteId: u8) {
    gSprites[birdSpriteId].data[6] = playerSpriteId as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_FlyBirdLeaveBall(sprite: *mut Sprite) {
    if (*sprite).data[7] == FALSE as i16 {
        if (*sprite).data[0] == 0 {
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
            (*sprite).affineAnims = sAffineAnims_FlyBird.as_ptr().cast_mut();
            InitSpriteAffineAnim(sprite);
            StartSpriteAffineAnim(sprite, 0);
            (*sprite).x = 0x76;
            (*sprite).y = -48;
            (*sprite).data[0] += 1;
            (*sprite).data[1] = 0x40;
            (*sprite).data[2] = 0x100;
        }
        (*sprite).data[1] += (*sprite).data[2] >> 8;
        (*sprite).x2 = Cos((*sprite).data[1], 0x78);
        (*sprite).y2 = Sin((*sprite).data[1], 0x78);
        if (*sprite).data[2] < 0x800 {
            (*sprite).data[2] += 0x60;
        }
        if (*sprite).data[1] > 0x81 {
            (*sprite).data[7] += 1;
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
            FreeOamMatrix((*sprite).oam.matrixNum() as u8);
            CalcCenterToCornerVec(
                sprite,
                (*sprite).oam.shape() as u8,
                (*sprite).oam.size() as u8,
                ST_OAM_AFFINE_OFF as u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlyBirdSwoopDown(sprite: *mut Sprite) {
    (*sprite).x2 = Cos((*sprite).data[2], 0x8c);
    (*sprite).y2 = Sin((*sprite).data[2], 0x48);
    (*sprite).data[2] = (*sprite).data[2] + 4 & 0xff;
    if (*sprite).data[6] != MAX_SPRITES as i16 {
        let mut sprite1: *mut Sprite = &raw mut gSprites[(*sprite).data[6]];
        (*sprite1).set_coordOffsetEnabled(FALSE as u16);
        (*sprite1).x = (*sprite).x + (*sprite).x2;
        (*sprite1).y = (*sprite).y + (*sprite).y2 - 8;
        (*sprite1).x2 = 0;
        (*sprite1).y2 = 0;
    }
    if (*sprite).data[2] >= 0x80 {
        (*sprite).data[7] = TRUE as i16;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlyBirdReturnToBall(sprite: *mut Sprite) {
    if (*sprite).data[7] == FALSE as i16 {
        if (*sprite).data[0] == 0 {
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
            (*sprite).affineAnims = sAffineAnims_FlyBird.as_ptr().cast_mut();
            InitSpriteAffineAnim(sprite);
            StartSpriteAffineAnim(sprite, 1);
            (*sprite).x = 0x5e;
            (*sprite).y = -32;
            (*sprite).data[0] += 1;
            (*sprite).data[1] = 0xf0;
            (*sprite).data[2] = 0x800;
            (*sprite).data[4] = 0x80;
        }
        (*sprite).data[1] += (*sprite).data[2] >> 8;
        (*sprite).data[3] += (*sprite).data[2] >> 8;
        (*sprite).data[1] &= 0xff;
        (*sprite).x2 = Cos((*sprite).data[1], 0x20);
        (*sprite).y2 = Sin((*sprite).data[1], 0x78);
        if (*sprite).data[2] > 0x100 {
            (*sprite).data[2] -= (*sprite).data[4];
        }
        if (*sprite).data[4] < 0x100 {
            (*sprite).data[4] += 24;
        }
        if (*sprite).data[2] < 0x100 {
            (*sprite).data[2] = 0x100;
        }
        if (*sprite).data[3] >= 60 {
            (*sprite).data[7] += 1;
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
            FreeOamMatrix((*sprite).oam.matrixNum() as u8);
            (*sprite).set_invisible(TRUE as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn StartFlyBirdReturnToBall(spriteId: u8) {
    StartFlyBirdSwoopDown(spriteId);
    gSprites[spriteId].callback = Some(SpriteCB_FlyBirdReturnToBall);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_FlyIn() -> u8 {
    CreateTask(Some(Task_FlyIn), 254);
    return 0;
}
pub(crate) unsafe extern "C" fn Task_FlyIn(taskId: u8) {
    sFlyInFieldEffectFuncs[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]);
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_BirdSwoopDown(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(objectEvent) == 0
        || ObjectEventClearHeldMovementIfFinished(objectEvent) != 0
    {
        (*task).data[0] += 1;
        (*task).data[2] = 17;
        (*task).data[15] = gPlayerAvatar.flags as i16;
        gPlayerAvatar.preventStep = TRUE;
        SetPlayerAvatarStateMask(PLAYER_AVATAR_FLAG_ON_FOOT);
        if (*task).data[15] as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0 {
            SetSurfBlob_BobState((*objectEvent).fieldEffectSpriteId, BOB_NONE);
        }
        ObjectEventSetGraphicsId(
            objectEvent,
            GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_SURFING),
        );
        CameraObjectFreeze();
        ObjectEventTurn(objectEvent, DIR_WEST);
        StartSpriteAnim(
            &raw mut gSprites[(*objectEvent).spriteId],
            ANIM_GET_ON_OFF_POKEMON_WEST,
        );
        (*objectEvent).set_invisible(FALSE as u32);
        (*task).data[1] = CreateFlyBirdSprite() as i16;
        StartFlyBirdSwoopDown((*task).data[1] as u8);
        SetFlyBirdPlayerSpriteId((*task).data[1] as u8, (*objectEvent).spriteId);
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_FlyInWithBird(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    let mut sprite: *mut Sprite = null_mut();
    if (*task).data[2] == 0
        || ({
            (*task).data[2] -= 1;
            (*task).data[2]
        }) == 0
    {
        objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        sprite = &raw mut gSprites[(*objectEvent).spriteId];
        SetFlyBirdPlayerSpriteId((*task).data[1] as u8, MAX_SPRITES);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*task).data[0] += 1;
        (*task).data[2] = 0;
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_JumpOffBird(task: *mut Task) {
    let mut sYPositions: CArray<i16, 18> = CArray([
        -2, -4, -5, -6, -7, -8, -8, -8, -7, -7, -6, -5, -3, -2, 0, 2, 4, 8,
    ]);
    let mut sprite: *mut Sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    (*sprite).y2 = sYPositions[(*task).data[2]];
    if ({
        (*task).data[2] += 1;
        (*task).data[2]
    }) >= 18
    {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_FieldMovePose(task: *mut Task) {
    let mut objectEvent: *mut ObjectEvent = null_mut();
    let mut sprite: *mut Sprite = null_mut();
    if GetFlyBirdAnimCompleted((*task).data[1] as u8) != 0 {
        objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        sprite = &raw mut gSprites[(*objectEvent).spriteId];
        (*objectEvent).set_inanimate(FALSE as u32);
        MoveObjectEventToMapCoords(
            objectEvent,
            (*objectEvent).currentCoords.x,
            (*objectEvent).currentCoords.y,
        );
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        SetPlayerAvatarFieldMove();
        ObjectEventSetHeldMovement(objectEvent, MOVEMENT_ACTION_START_ANIM_IN_DIRECTION);
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_BirdReturnToBall(task: *mut Task) {
    if ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[gPlayerAvatar.objectEventId])
        != 0
    {
        (*task).data[0] += 1;
        StartFlyBirdReturnToBall((*task).data[1] as u8);
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_WaitBirdReturn(task: *mut Task) {
    if GetFlyBirdAnimCompleted((*task).data[1] as u8) != 0 {
        DestroySprite(&raw mut gSprites[(*task).data[1]]);
        (*task).data[0] += 1;
        (*task).data[1] = 16;
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_End(task: *mut Task) {
    let mut state: u8 = 0;
    let mut objectEvent: *mut ObjectEvent = null_mut();
    if ({
        (*task).data[1] -= 1;
        (*task).data[1]
    }) == 0
    {
        objectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        state = PLAYER_AVATAR_STATE_NORMAL;
        if (*task).data[15] as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0 {
            state = PLAYER_AVATAR_STATE_SURFING;
            SetSurfBlob_BobState((*objectEvent).fieldEffectSpriteId, BOB_PLAYER_AND_MON);
        }
        ObjectEventSetGraphicsId(objectEvent, GetPlayerAvatarGraphicsIdByStateId(state));
        ObjectEventTurn(objectEvent, DIR_SOUTH);
        gPlayerAvatar.flags = (*task).data[15] as u8;
        gPlayerAvatar.preventStep = FALSE;
        FieldEffectActiveListRemove(FLDEFF_FLY_IN);
        DestroyTask(FindTaskIdByFunc(Some(Task_FlyIn)));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_DestroyDeoxysRock() -> u8 {
    let mut taskId: u8 = 0;
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        gFieldEffectArguments[0] as u8,
        gFieldEffectArguments[1] as u8,
        gFieldEffectArguments[2] as u8,
        &raw mut objectEventId,
    ) == 0
    {
        taskId = CreateTask(Some(Task_DestroyDeoxysRock), 80);
        gTasks[taskId].data[2] = objectEventId as i16;
        gTasks[taskId].data[6] = gFieldEffectArguments[0] as i16;
        gTasks[taskId].data[7] = gFieldEffectArguments[1] as i16;
        gTasks[taskId].data[8] = gFieldEffectArguments[2] as i16;
    } else {
        FieldEffectActiveListRemove(FLDEFF_DESTROY_DEOXYS_ROCK);
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_DeoxysRockCameraShake(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(7) != 0 {
        if ({
            *data.at(6) += 1;
            *data.at(6)
        }) > 20
        {
            *data.at(6) = 0;
            if *data.at(5) != 0 {
                *data.at(5) -= 1;
            }
        }
    } else {
        *data.at(5) = 4;
    }
    if ({
        *data += 1;
        *data
    }) > 1
    {
        *data = 0;
        if ({
            *data.at(1) += 1;
            *data.at(1)
        }) as i32
            & 1
            != 0
        {
            SetCameraPanning(0, -*data.at(5));
        } else {
            SetCameraPanning(0, *data.at(5));
        }
    }
    UpdateCameraPanning();
    if *data.at(5) == 0 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn StartEndingDeoxysRockCameraShake(taskId: u8) {
    gTasks[taskId].data[7] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn Task_DestroyDeoxysRock(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    InstallCameraPanAheadCallback();
    SetCameraPanningCallback(None);
    sDestroyDeoxysRockEffectFuncs[*data.at(1)].unwrap_unchecked()(data, taskId);
}
pub(crate) unsafe extern "C" fn DestroyDeoxysRockEffect_CameraShake(
    mut data: *mut i16,
    taskId: u8,
) {
    let mut newTaskId: u8 = CreateTask(Some(Task_DeoxysRockCameraShake), 90);
    PlaySE(SE_THUNDER2);
    *data.at(5) = newTaskId as i16;
    *data.at(1) += 1;
}
pub(crate) unsafe extern "C" fn DestroyDeoxysRockEffect_RockFragments(
    mut data: *mut i16,
    taskId: u8,
) {
    if ({
        *data.at(3) += 1;
        *data.at(3)
    }) > 120
    {
        let mut sprite: *mut Sprite = &raw mut gSprites[gObjectEvents[*data.at(2)].spriteId];
        gObjectEvents[*data.at(2)].set_invisible(TRUE as u32);
        BlendPalettes(PALETTES_BG, 0x10, 32767);
        BeginNormalPaletteFade(PALETTES_BG, 0, 0x10, 0, 32767);
        CreateDeoxysRockFragments(sprite);
        PlaySE(SE_THUNDER);
        StartEndingDeoxysRockCameraShake(*data.at(5) as u8);
        *data.at(3) = 0;
        *data.at(1) += 1;
    }
}
pub(crate) unsafe extern "C" fn DestroyDeoxysRockEffect_WaitAndEnd(data: *mut i16, taskId: u8) {
    if gPaletteFade.active() == 0 && FuncIsActiveTask(Some(Task_DeoxysRockCameraShake)) == 0 {
        InstallCameraPanAheadCallback();
        RemoveObjectEventByLocalIdAndMap(*data.at(6) as u8, *data.at(7) as u8, *data.at(8) as u8);
        FieldEffectActiveListRemove(FLDEFF_DESTROY_DEOXYS_ROCK);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CreateDeoxysRockFragments(sprite: *mut Sprite) {
    let mut i: i32 = 0;
    let mut xPos: i32 =
        gTotalCameraPixelOffsetX as i16 as i32 + (*sprite).x as i32 + (*sprite).x2 as i32;
    let mut yPos: i32 =
        gTotalCameraPixelOffsetY as i16 as i32 + (*sprite).y as i32 + (*sprite).y2 as i32 - 4;
    i = 0;
    while i < 4 {
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_DeoxysRockFragment).cast_mut(),
            xPos as i16,
            yPos as i16,
            0,
        );
        if spriteId != MAX_SPRITES {
            StartSpriteAnim(&raw mut gSprites[spriteId], i as u8);
            gSprites[spriteId].data[0] = i as i16;
            gSprites[spriteId]
                .oam
                .set_paletteNum((*sprite).oam.paletteNum());
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DeoxysRockFragment(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).x -= 16;
            (*sprite).y -= 12;
        }
        1 => {
            (*sprite).x += 16;
            (*sprite).y -= 12;
        }
        2 => {
            (*sprite).x -= 16;
            (*sprite).y += 12;
        }
        3 => {
            (*sprite).x += 16;
            (*sprite).y += 12;
        }
        _ => {}
    }
    if (*sprite).x < -4 || (*sprite).x > 244 || (*sprite).y < -4 || (*sprite).y > 164 {
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_MoveDeoxysRock(sprite: *mut Sprite) -> u8 {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        gFieldEffectArguments[0] as u8,
        gFieldEffectArguments[1] as u8,
        gFieldEffectArguments[2] as u8,
        &raw mut objectEventId,
    ) == 0
    {
        let mut object: *mut ObjectEvent = null_mut();
        let mut xPos: i32 = 0;
        let mut yPos: i32 = 0;
        let mut taskId: u8 = 0;
        object = &raw mut gObjectEvents[objectEventId];
        xPos = (*object).currentCoords.x as i32 - MAP_OFFSET;
        yPos = (*object).currentCoords.y as i32 - MAP_OFFSET;
        xPos = (gFieldEffectArguments[3] - xPos) * 16;
        yPos = (gFieldEffectArguments[4] - yPos) * 16;
        ShiftObjectEventCoords(
            object,
            gFieldEffectArguments[3] as i16 + MAP_OFFSET as i16,
            gFieldEffectArguments[4] as i16 + MAP_OFFSET as i16,
        );
        taskId = CreateTask(Some(Task_MoveDeoxysRock), 80);
        gTasks[taskId].data[1] = (*object).spriteId as i16;
        gTasks[taskId].data[2] = gSprites[(*object).spriteId].x + xPos as i16;
        gTasks[taskId].data[3] = gSprites[(*object).spriteId].y + yPos as i16;
        gTasks[taskId].data[8] = gFieldEffectArguments[5] as i16;
        gTasks[taskId].data[9] = objectEventId as i16;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_MoveDeoxysRock(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut sprite: *mut Sprite = &raw mut gSprites[*data.at(1)];
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            *data.at(4) = (*sprite).x << 4;
            *data.at(5) = (*sprite).y << 4;
            *data.at(6) = (if *data.at(8) != 0 {
                div_i32(
                    *data.at(2) as i32 * 16 - *data.at(4) as i32,
                    *data.at(8) as i32,
                )
            } else {
                0
            }) as i16;
            *data.at(7) = (if *data.at(8) != 0 {
                div_i32(
                    *data.at(3) as i32 * 16 - *data.at(5) as i32,
                    *data.at(8) as i32,
                )
            } else {
                0
            }) as i16;
            *data += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if *data.at(8) != 0 {
                *data.at(8) -= 1;
                *data.at(4) += *data.at(6);
                *data.at(5) += *data.at(7);
                (*sprite).x = *data.at(4) >> 4;
                (*sprite).y = *data.at(5) >> 4;
            } else {
                let mut object: *mut ObjectEvent = &raw mut gObjectEvents[*data.at(9)];
                (*sprite).x = *data.at(2);
                (*sprite).y = *data.at(3);
                ShiftStillObjectEventCoords(object);
                (*object).set_triggerGroundEffectsOnStop(TRUE as u32);
                FieldEffectActiveListRemove(FLDEFF_MOVE_DEOXYS_ROCK);
                DestroyTask(taskId);
            }
            break 'l1;
        }
    }
}
