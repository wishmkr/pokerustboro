//! Translated from `src/field_effect.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sNewGameBirch_Gfx sUnusedBirchBeauty sNewGameBirch_Pal sPokeballGlow_Gfx sPokeballGlow_Pal sPokecenterMonitor0_Gfx sPokecenterMonitor1_Gfx sHofMonitorBig_Gfx sHofMonitorSmall_Gfx sHofMonitor_Pal sFieldMoveStreaksOutdoors_Gfx sFieldMoveStreaksOutdoors_Pal sFieldMoveStreaksOutdoors_Tilemap sFieldMoveStreaksIndoors_Gfx sFieldMoveStreaksIndoors_Pal sFieldMoveStreaksIndoors_Tilemap sSpotlight_Pal sSpotlight_Gfx sRockFragment_TopLeft sRockFragment_TopRight sRockFragment_BottomLeft sRockFragment_BottomRight gFieldEffectScriptFuncs sOam_64x64 sOam_8x8 sOam_16x16 sPicTable_NewGameBirch sSpritePalette_NewGameBirch sAnim_NewGameBirch sAnimTable_NewGameBirch sSpriteTemplate_NewGameBirch gSpritePalette_PokeballGlow gSpritePalette_HofMonitor sOam_32x16 sPicTable_PokeballGlow sPicTable_PokecenterMonitor sPicTable_HofMonitorBig sPicTable_HofMonitorSmall sSubsprites_PokecenterMonitor sSubspriteTable_PokecenterMonitor sSubsprites_HofMonitorBig sSubspriteTable_HofMonitorBig sAnim_Static sAnim_Flicker sAnims_Flicker sAnims_HofMonitor sSpriteTemplate_PokeballGlow sSpriteTemplate_PokecenterMonitor sSpriteTemplate_HofMonitorBig sSpriteTemplate_HofMonitorSmall sPokecenterHealEffectFuncs sHallOfFameRecordEffectFuncs sPokeballGlowEffectFuncs sPokeballCoordOffsets sPokeballGlowReds sPokeballGlowGreens sPokeballGlowBlues sFallWarpFieldEffectFuncs sEscalatorWarpOutFieldEffectFuncs sEscalatorWarpInFieldEffectFuncs sWaterfallFieldEffectFuncs sDiveFieldEffectFuncs sLavaridgeGymB1FWarpEffectFuncs sLavaridgeGymB1FWarpExitEffectFuncs sLavaridgeGym1FWarpEffectFuncs sEscapeRopeWarpOutEffectFuncs sEscapeRopeWarpInEffectFuncs sTeleportWarpOutFieldEffectFuncs sTeleportWarpInFieldEffectFuncs sFieldMoveShowMonOutdoorsEffectFuncs sFieldMoveShowMonIndoorsEffectFuncs sSurfFieldEffectFuncs sFlyOutFieldEffectFuncs sAffineAnim_FlyBirdLeaveBall sAffineAnim_FlyBirdReturnToBall sAffineAnims_FlyBird sFlyInFieldEffectFuncs sDestroyDeoxysRockEffectFuncs sImages_DeoxysRockFragment sAnim_RockFragment_TopLeft sAnim_RockFragment_TopRight sAnim_RockFragment_BottomLeft sAnim_RockFragment_BottomRight sAnims_DeoxysRockFragment sSpriteTemplate_DeoxysRockFragment
#[allow(unused_imports)]
use crate::data::field_effect::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gFieldEffectArguments: crate::ffi::Align4<[u8; 32]> = crate::ffi::Align4([0; 32]);
pub(crate) static mut sActiveList: crate::ffi::Align4<[u8; 32]> = crate::ffi::Align4([0; 32]);

unsafe extern "C" {
    static mut gDummySpriteAffineAnimTable: u8;
    static mut gDummySpriteAnimTable: u8;
    static mut gFieldCallback: u8;
    static mut gFieldEffectObjectTemplatePointers: u8;
    static mut gFieldEffectScriptPointers: u8;
    static mut gMain: u8;
    static mut gMonPaletteTable: u8;
    static mut gObjectEvents: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerAvatar: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSpriteCoordOffsetY: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gTotalCameraPixelOffsetX: u8;
    static mut gTotalCameraPixelOffsetY: u8;
    static mut gTrainerFrontPicPaletteTable: u8;
    static mut gTrainerFrontPicTable: u8;
    fn BGMusicStopped() -> u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn CB2_LoadMap();
    fn CB2_ReturnToField();
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn CalculatePlayerPartyCount() -> u8;
    fn CameraObjectFreeze();
    fn CameraObjectReset();
    fn ClearMirageTowerPulseBlendEffect();
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateInvisibleSprite(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
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
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAtEnd(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
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
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonSpritePalStructFromOtIdPersonality(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetPlayerAvatarGraphicsIdByStateId(a0: u8) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetSpritePaletteTagByPaletteNum(a0: u8) -> u16;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetSpriteTileTagByTileStart(a0: u16) -> u16;
    fn GetWalkInPlaceFasterMovementAction(a0: u32) -> u8;
    fn GetWalkNormalMovementAction(a0: u32) -> u8;
    fn GetWalkSlowMovementAction(a0: u32) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn InitTextBoxGfxAndPrinters();
    fn InstallCameraPanAheadCallback();
    fn IsEscalatorMoving() -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsMapTypeOutdoors(a0: u8) -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn LZDecompressVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePaletteOverrideBuffer(a0: *mut u8, a1: *mut u8);
    fn LoadCompressedSpriteSheetOverrideBuffer(a0: *mut u8, a1: *mut u8);
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadWordFromTwoHalfwords(a0: *mut u16, a1: *mut u32);
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsWaterfall(a0: u8) -> u8;
    fn MoveCoords(a0: u8, a1: *mut i16, a2: *mut i16);
    fn MoveObjectEventToMapCoords(a0: *mut u8, a1: i16, a2: i16);
    fn ObjectEventCheckHeldMovementStatus(a0: *mut u8) -> u8;
    fn ObjectEventClearHeldMovementIfActive(a0: *mut u8);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8;
    fn ObjectEventIsMovementOverridden(a0: *mut u8) -> u8;
    fn ObjectEventSetGraphicsId(a0: *mut u8, a1: u8);
    fn ObjectEventSetHeldMovement(a0: *mut u8, a1: u8) -> u8;
    fn ObjectEventTurn(a0: *mut u8, a1: u8);
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
    fn SetObjectEventDirection(a0: *mut u8, a1: u8);
    fn SetPlayerAvatarFieldMove();
    fn SetPlayerAvatarStateMask(a0: u8);
    fn SetPlayerAvatarTransitionFlags(a0: u16);
    fn SetSpritePosToOffsetMapCoords(a0: *mut i16, a1: *mut i16, a2: i16, a3: i16);
    fn SetSubspriteTables(a0: *mut u8, a1: *mut u8);
    fn SetSurfBlob_BobState(a0: u8, a1: u8);
    fn SetSurfBlob_DontSyncAnim(a0: u8, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWarpDestinationToEscapeWarp();
    fn SetWarpDestinationToLastHealLocation();
    fn ShiftObjectEventCoords(a0: *mut u8, a1: i16, a2: i16);
    fn ShiftStillObjectEventCoords(a0: *mut u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartEscalator(a0: u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StopEscalator();
    fn StoreWordInTwoHalfwords(a0: *mut u16, a1: u32);
    fn TryDoDiveWarp(a0: *mut u8, a1: u16) -> u8;
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
    unsafe {
        let mut id = id;
        let mut script: *mut u8 = core::ptr::null_mut();
        let mut val: u32 = 0u32;
        FieldEffectActiveListAdd(id);
        script = ((((&raw mut gFieldEffectScriptPointers).cast::<*mut u8>()).cast::<*mut u8>())
            .wrapping_offset(((id) as i32) as isize))
        .read();
        'l1: loop {
            if !(((((((&raw const gFieldEffectScriptFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut *mut u8, *mut u32) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut *mut u8, *mut u32) -> u8>>())
            .wrapping_offset((((script).read()) as i32) as isize))
            .read())
            .unwrap_unchecked()(&raw mut script, &raw mut val))
                != 0)
            {
                break 'l1;
            }
        }
        return val;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadtiles(script: *mut *mut u8, val: *mut u32) -> u8 {
    unsafe {
        let mut script = script;
        let mut val = val;
        (script).write(((script).read()).wrapping_offset(1));
        FieldEffectScript_LoadTiles(script);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadfadedpal(script: *mut *mut u8, val: *mut u32) -> u8 {
    unsafe {
        let mut script = script;
        let mut val = val;
        (script).write(((script).read()).wrapping_offset(1));
        FieldEffectScript_LoadFadedPalette(script);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadpal(script: *mut *mut u8, val: *mut u32) -> u8 {
    unsafe {
        let mut script = script;
        let mut val = val;
        (script).write(((script).read()).wrapping_offset(1));
        FieldEffectScript_LoadPalette(script);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_callnative(script: *mut *mut u8, val: *mut u32) -> u8 {
    unsafe {
        let mut script = script;
        let mut val = val;
        (script).write(((script).read()).wrapping_offset(1));
        FieldEffectScript_CallNative(script, val);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_end(script: *mut *mut u8, val: *mut u32) -> u8 {
    unsafe {
        let mut script = script;
        let mut val = val;
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadgfx_callnative(
    script: *mut *mut u8,
    val: *mut u32,
) -> u8 {
    unsafe {
        let mut script = script;
        let mut val = val;
        (script).write(((script).read()).wrapping_offset(1));
        FieldEffectScript_LoadTiles(script);
        FieldEffectScript_LoadFadedPalette(script);
        FieldEffectScript_CallNative(script, val);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadtiles_callnative(
    script: *mut *mut u8,
    val: *mut u32,
) -> u8 {
    unsafe {
        let mut script = script;
        let mut val = val;
        (script).write(((script).read()).wrapping_offset(1));
        FieldEffectScript_LoadTiles(script);
        FieldEffectScript_CallNative(script, val);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectCmd_loadfadedpal_callnative(
    script: *mut *mut u8,
    val: *mut u32,
) -> u8 {
    unsafe {
        let mut script = script;
        let mut val = val;
        (script).write(((script).read()).wrapping_offset(1));
        FieldEffectScript_LoadFadedPalette(script);
        FieldEffectScript_CallNative(script, val);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_ReadWord(script: *mut *mut u8) -> u32 {
    unsafe {
        let mut script = script;
        return ((((((((script).read()).read()) as i32)
            .wrapping_add(((((((script).read()).wrapping_offset(1)).read()) as i32) << 8)))
        .wrapping_add(((((((script).read()).wrapping_offset(2)).read()) as i32) << 16)))
        .wrapping_add(((((((script).read()).wrapping_offset(3)).read()) as i32) << 24)))
            as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_LoadTiles(script: *mut *mut u8) {
    unsafe {
        let mut script = script;
        let mut sheet: *mut u8 = ((FieldEffectScript_ReadWord(script)) as usize as *mut u8);
        if ((GetSpriteTileStartByTag(((sheet).wrapping_add(6).cast::<u16>()).read())) as i32)
            == 65535i32
        {
            LoadSpriteSheet(sheet);
        }
        (script).write(((script).read()).wrapping_offset(4));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_LoadFadedPalette(script: *mut *mut u8) {
    unsafe {
        let mut script = script;
        let mut palette: *mut u8 = ((FieldEffectScript_ReadWord(script)) as usize as *mut u8);
        LoadSpritePalette(palette);
        UpdateSpritePaletteWithWeather(IndexOfSpritePaletteTag(
            ((palette).wrapping_add(4).cast::<u16>()).read(),
        ));
        (script).write(((script).read()).wrapping_offset(4));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_LoadPalette(script: *mut *mut u8) {
    unsafe {
        let mut script = script;
        let mut palette: *mut u8 = ((FieldEffectScript_ReadWord(script)) as usize as *mut u8);
        LoadSpritePalette(palette);
        (script).write(((script).read()).wrapping_offset(4));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectScript_CallNative(script: *mut *mut u8, val: *mut u32) {
    unsafe {
        let mut script = script;
        let mut val = val;
        let mut func: Option<unsafe extern "C" fn() -> u32> =
            (core::mem::transmute::<usize, Option<unsafe extern "C" fn() -> u32>>(
                (FieldEffectScript_ReadWord(script)) as usize,
            ));
        (val).write((func).unwrap_unchecked()());
        (script).write(((script).read()).wrapping_offset(4));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectFreeGraphicsResources(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut sheetTileStart: u16 = ((sprite).wrapping_add(64).cast::<u16>()).read();
        let mut paletteNum: u32 =
            ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as u32);
        DestroySprite(sprite);
        FieldEffectFreeTilesIfUnused(sheetTileStart);
        FieldEffectFreePaletteIfUnused(((paletteNum) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectStop(sprite: *mut u8, id: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id = id;
        FieldEffectFreeGraphicsResources(sprite);
        FieldEffectActiveListRemove(id);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectFreeTilesIfUnused(tileStart: u16) {
    unsafe {
        let mut tileStart = tileStart;
        let mut i: u8 = 0u8;
        let mut tag: u16 = GetSpriteTileTagByTileStart(tileStart);
        if ((tag) as i32) != 65535i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 64i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 68))
                            .wrapping_add(62),
                            0,
                            1,
                            false,
                        ) as u16)
                            != 0)
                            && ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(63),
                                6,
                                1,
                                false,
                            ) as u16)
                                != 0))
                            && (((tileStart) as i32)
                                == ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(64)
                                .cast::<u16>())
                                .read()) as i32))
                        {
                            return;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FreeSpriteTilesByTag(tag);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectFreePaletteIfUnused(paletteNum: u8) {
    unsafe {
        let mut paletteNum = paletteNum;
        let mut i: u8 = 0u8;
        let mut tag: u16 = GetSpritePaletteTagByPaletteNum(paletteNum);
        if ((tag) as i32) != 65535i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 64i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 68))
                            .wrapping_add(62),
                            0,
                            1,
                            false,
                        ) as u16)
                            != 0)
                            && (((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                == ((paletteNum) as i32))
                        {
                            return;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FreeSpritePaletteByTag(tag);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectActiveListClear() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(32u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sActiveList).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectActiveListAdd(id: u8) {
    unsafe {
        let mut id = id;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(32u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sActiveList).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 255i32
                    {
                        ((((&raw mut sActiveList).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(id);
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectActiveListRemove(id: u8) {
    unsafe {
        let mut id = id;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(32u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sActiveList).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((id) as i32)
                    {
                        ((((&raw mut sActiveList).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(255u8);
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldEffectActiveListContains(id: u8) -> u8 {
    unsafe {
        let mut id = id;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(32u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sActiveList).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((id) as i32)
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTrainerSprite(
    trainerSpriteID: u8,
    x: i16,
    y: i16,
    subpriority: u8,
    buffer: *mut u8,
) -> u8 {
    unsafe {
        let mut trainerSpriteID = trainerSpriteID;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut buffer = buffer;
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        LoadCompressedSpritePaletteOverrideBuffer(
            ((&raw mut gTrainerFrontPicPaletteTable).cast::<u8>())
                .wrapping_offset(((trainerSpriteID) as i32) as isize * 8),
            buffer,
        );
        LoadCompressedSpriteSheetOverrideBuffer(
            ((&raw mut gTrainerFrontPicTable).cast::<u8>())
                .wrapping_offset(((trainerSpriteID) as i32) as isize * 8),
            buffer,
        );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(
            ((((&raw mut gTrainerFrontPicTable).cast::<u8>())
                .wrapping_offset(((trainerSpriteID) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        );
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(
            ((((&raw mut gTrainerFrontPicPaletteTable).cast::<u8>())
                .wrapping_offset(((trainerSpriteID) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        );
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write((&raw const sOam_64x64).cast::<u8>().cast_mut());
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut *mut u8>())
        .write(((&raw mut gDummySpriteAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
        .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        return CreateSprite((&raw mut spriteTemplate).cast::<u8>(), x, y, subpriority);
    }
}
pub(crate) unsafe extern "C" fn LoadTrainerGfx_TrainerCard(
    gender: u8,
    palOffset: u16,
    dest: *mut u8,
) {
    unsafe {
        let mut gender = gender;
        let mut palOffset = palOffset;
        let mut dest = dest;
        LZDecompressVram(
            ((((&raw mut gTrainerFrontPicTable).cast::<u8>())
                .wrapping_offset(((gender) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read(),
            dest,
        );
        LoadCompressedPalette(
            ((((&raw mut gTrainerFrontPicPaletteTable).cast::<u8>())
                .wrapping_offset(((gender) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read(),
            palOffset,
            32u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddNewGameBirchObject(x: i16, y: i16, subpriority: u8) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        LoadSpritePalette(
            (&raw const sSpritePalette_NewGameBirch)
                .cast::<u8>()
                .cast_mut(),
        );
        return CreateSprite(
            (&raw const sSpriteTemplate_NewGameBirch)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            subpriority,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonSprite_PicBox(
    species: u16,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    unsafe {
        let mut species = species;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut spriteId: i32 = ((CreateMonPicSprite_HandleDeoxys(
            species,
            0u32,
            32768u32,
            1u8,
            x,
            y,
            0u8,
            ((((&raw mut gMonPaletteTable).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        )) as i32);
        PreservePaletteInWeather(
            ((((IndexOfSpritePaletteTag(
                ((((&raw mut gMonPaletteTable).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8))
                .wrapping_add(4)
                .cast::<u16>())
                .read(),
            )) as i32)
                .wrapping_add(16i32)) as u8),
        );
        if spriteId == 65535i32 {
            return 64u8;
        } else {
            return ((spriteId) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
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
    unsafe {
        let mut species = species;
        let mut otId = otId;
        let mut personality = personality;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut spritePalette: *mut u8 =
            GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
        let mut spriteId: u16 = CreateMonPicSprite_HandleDeoxys(
            species,
            otId,
            personality,
            1u8,
            x,
            y,
            0u8,
            ((spritePalette).wrapping_add(4).cast::<u16>()).read(),
        );
        PreservePaletteInWeather(
            ((((IndexOfSpritePaletteTag(((spritePalette).wrapping_add(4).cast::<u16>()).read()))
                as i32)
                .wrapping_add(16i32)) as u8),
        );
        if ((spriteId) as i32) == 65535i32 {
            return 64u8;
        } else {
            return ((spriteId) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeResourcesAndDestroySprite(sprite: *mut u8, spriteId: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId = spriteId;
        ResetPreservedPalettesInWeather();
        if (crate::c::bf_read((sprite).wrapping_add(1), 0, 2, false) as u32) != 0u32 {
            FreeOamMatrix(
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            );
        }
        FreeAndDestroyMonPicSprite(((spriteId) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiplyInvertedPaletteRGBComponents(i: u16, r: u8, g: u8, b: u8) {
    unsafe {
        let mut i = i;
        let mut r = r;
        let mut g = g;
        let mut b = b;
        let mut curRed: i32 = 0i32;
        let mut curGreen: i32 = 0i32;
        let mut curBlue: i32 = 0i32;
        let mut color: u16 = ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
        .read();
        curRed = (((color) as i32) & 31i32);
        curGreen = ((((color) as i32) & 992i32) >> 5);
        curBlue = ((((color) as i32) & 31744i32) >> 10);
        curRed =
            (curRed).wrapping_add((((31i32).wrapping_sub(curRed)).wrapping_mul(((r) as i32)) >> 4));
        curGreen = (curGreen)
            .wrapping_add((((31i32).wrapping_sub(curGreen)).wrapping_mul(((g) as i32)) >> 4));
        curBlue = (curBlue)
            .wrapping_add((((31i32).wrapping_sub(curBlue)).wrapping_mul(((b) as i32)) >> 4));
        color = ((curRed) as u16);
        color = ((((color) as i32) | (curGreen << 5)) as u16);
        color = ((((color) as i32) | (curBlue << 10)) as u16);
        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
        .write(color);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiplyPaletteRGBComponents(i: u16, r: u8, g: u8, b: u8) {
    unsafe {
        let mut i = i;
        let mut r = r;
        let mut g = g;
        let mut b = b;
        let mut curRed: i32 = 0i32;
        let mut curGreen: i32 = 0i32;
        let mut curBlue: i32 = 0i32;
        let mut color: u16 = ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
        .read();
        curRed = (((color) as i32) & 31i32);
        curGreen = ((((color) as i32) & 992i32) >> 5);
        curBlue = ((((color) as i32) & 31744i32) >> 10);
        curRed = (curRed).wrapping_sub(((curRed).wrapping_mul(((r) as i32)) >> 4));
        curGreen = (curGreen).wrapping_sub(((curGreen).wrapping_mul(((g) as i32)) >> 4));
        curBlue = (curBlue).wrapping_sub(((curBlue).wrapping_mul(((b) as i32)) >> 4));
        color = ((curRed) as u16);
        color = ((((color) as i32) | (curGreen << 5)) as u16);
        color = ((((color) as i32) | (curBlue << 10)) as u16);
        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
        .write(color);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_PokecenterHeal() -> u8 {
    unsafe {
        let mut nPokemon: u8 = 0u8;
        let mut task: *mut u8 = core::ptr::null_mut();
        nPokemon = CalculatePlayerPartyCount();
        task = ((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((CreateTask(Some(Task_PokecenterHeal), 255u8)) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(((nPokemon) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(93i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(36i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(124i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(24i16);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_PokecenterHeal(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((((&raw const sPokecenterHealEffectFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize))
        .read())
        .unwrap_unchecked()(task);
    }
}
pub(crate) unsafe extern "C" fn PokecenterHealEffect_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((CreateGlowingPokeballsEffect(
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read(),
                1u16,
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
            ((CreatePokecenterMonitorSprite(
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read(),
            )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn PokecenterHealEffect_WaitForBallPlacement(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as i32)
            > 1i32
        {
            let __p1 = ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn PokecenterHealEffect_WaitForBallFlashing(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as i32)
            > 4i32
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn PokecenterHealEffect_WaitForSoundAndEnd(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as i32)
            > 6i32
        {
            DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 68,
            ));
            FieldEffectActiveListRemove(25u8);
            DestroyTask(FindTaskIdByFunc(Some(Task_PokecenterHeal)));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_HallOfFameRecord() -> u8 {
    unsafe {
        let mut nPokemon: u8 = 0u8;
        let mut task: *mut u8 = core::ptr::null_mut();
        nPokemon = CalculatePlayerPartyCount();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((CreateTask(Some(Task_HallOfFameRecord), 255u8)) as i32) as isize * 40,
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(((nPokemon) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(117i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(52i16);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_HallOfFameRecord(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((((&raw const sHallOfFameRecordEffectFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize))
        .read())
        .unwrap_unchecked()(task);
    }
}
pub(crate) unsafe extern "C" fn HallOfFameRecordEffect_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut taskId: u8 = 0u8;
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((CreateGlowingPokeballsEffect(
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read(),
                0u16,
            )) as i16),
        );
        taskId = FindTaskIdByFunc(Some(Task_HallOfFameRecord));
        CreateHofMonitorSprite(((taskId) as i16), 120i16, 24i16, 0u8);
        CreateHofMonitorSprite(((taskId) as i16), 40i16, 8i16, 1u8);
        CreateHofMonitorSprite(((taskId) as i16), 72i16, 8i16, 1u8);
        CreateHofMonitorSprite(((taskId) as i16), 168i16, 8i16, 1u8);
        CreateHofMonitorSprite(((taskId) as i16), 200i16, 8i16, 1u8);
    }
}
pub(crate) unsafe extern "C" fn HallOfFameRecordEffect_WaitForBallPlacement(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as i32)
            > 1i32
        {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn HallOfFameRecordEffect_WaitForBallFlashing(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as i32)
            > 4i32
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn HallOfFameRecordEffect_WaitForSoundAndEnd(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as i32)
            > 6i32
        {
            DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 68,
            ));
            FieldEffectActiveListRemove(62u8);
            DestroyTask(FindTaskIdByFunc(Some(Task_HallOfFameRecord)));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateGlowingPokeballsEffect(
    numMons: i16,
    x: i16,
    y: i16,
    playHealSe: u16,
) -> u8 {
    unsafe {
        let mut numMons = numMons;
        let mut x = x;
        let mut y = y;
        let mut playHealSe = playHealSe;
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        spriteId = CreateInvisibleSprite(Some(SpriteCB_PokeballGlowEffect));
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        ((sprite).wrapping_add(36).cast::<i16>()).write(x);
        ((sprite).wrapping_add(38).cast::<i16>()).write(y);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((playHealSe) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(numMons);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(((spriteId) as i16));
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokeballGlowEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((((&raw const sPokeballGlowEffectFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize))
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_PlaceBalls(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 = 0u8;
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32)
            || ((({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(25i16);
            spriteId = CreateSpriteAtEnd(
                (&raw const sSpriteTemplate_PokeballGlow)
                    .cast::<u8>()
                    .cast_mut(),
                (((((((((&raw const sPokeballCoordOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize
                            * 4,
                    ))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
                (((((((((&raw const sPokeballCoordOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize
                            * 4,
                    ))
                .wrapping_add(2)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
                0u8,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (2u16) as i32,
            );
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read());
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p4).write(((__p4).read()).wrapping_sub(1));
            PlaySE(23u16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(32i16);
            let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_TryPlaySe(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) != 0 {
                PlayFanfare(368u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_Flash1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut phase: u8 = 0u8;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(8i16);
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p4).write((((((__p4).read()) as i32) & 3i32) as i16));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
        }
        phase = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            .wrapping_add(3i32)
            & 3i32) as u8);
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(8i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
        phase = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            .wrapping_add(2i32)
            & 3i32) as u8);
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(6i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
        phase = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            .wrapping_add(1i32)
            & 3i32) as u8);
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(2i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
        phase = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(5i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(3i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32) > 2i32
        {
            let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p6).write(((__p6).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_Flash2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut phase: u8 = 0u8;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(8i16);
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p4).write((((((__p4).read()) as i32) & 3i32) as i16));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 3i32
            {
                let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(30i16);
            }
        }
        phase = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(8i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(6i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(2i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(5i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
        MultiplyInvertedPaletteRGBComponents(
            ((((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(4103u16)) as i32).wrapping_mul(16i32)))
            .wrapping_add(3i32)) as u16),
            ((((&raw const sPokeballGlowReds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowGreens).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
            ((((&raw const sPokeballGlowBlues).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((phase) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_WaitAfterFlash(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_Dummy(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_WaitForSound(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == 0i32)
            || ((IsFanfareTaskInactive()) != 0)
        {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn PokeballGlowEffect_Idle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokeballGlow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as i32)
            > 4i32
        {
            FieldEffectFreeGraphicsResources(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePokecenterMonitorSprite(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        spriteId = CreateSpriteAtEnd(
            (&raw const sSpriteTemplate_PokecenterMonitor)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            0u8,
        );
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (2u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        SetSubspriteTables(
            sprite,
            (&raw const sSubspriteTable_PokecenterMonitor)
                .cast::<u8>()
                .cast_mut(),
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokecenterMonitor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            StartSpriteAnim(sprite, 1u8);
        }
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            FieldEffectFreeGraphicsResources(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateHofMonitorSprite(
    taskId: i16,
    x: i16,
    y: i16,
    isSmallMonitor: u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut x = x;
        let mut y = y;
        let mut isSmallMonitor = isSmallMonitor;
        let mut spriteId: u8 = 0u8;
        if !((isSmallMonitor) != 0) {
            spriteId = CreateSpriteAtEnd(
                (&raw const sSpriteTemplate_HofMonitorBig)
                    .cast::<u8>()
                    .cast_mut(),
                x,
                y,
                0u8,
            );
            SetSubspriteTables(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                (&raw const sSubspriteTable_HofMonitorBig)
                    .cast::<u8>()
                    .cast_mut(),
            );
        } else {
            spriteId = CreateSpriteAtEnd(
                (&raw const sSpriteTemplate_HofMonitorSmall)
                    .cast::<u8>()
                    .cast_mut(),
                x,
                y,
                0u8,
            );
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(taskId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HallOfFameMonitor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read())
            != 0
        {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 0i32)
                || ((({
                    let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t2 = ((__p1).read()).wrapping_sub(1);
                    (__p1).write(__t2);
                    __t2
                }) as i32)
                    == 0i32)
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(16i16);
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                        ^ 1i32) as u16) as i32,
                );
            }
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 127i32
        {
            FieldEffectFreeGraphicsResources(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReturnToFieldFromFlyMapSelect() {
    unsafe {
        SetMainCallback2(Some(CB2_ReturnToField));
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCallback_UseFly));
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_UseFly() {
    unsafe {
        FadeInFromBlack();
        CreateTask(Some(Task_UseFly), 0u8);
        LockPlayerFieldControls();
        FreezeObjectEvents();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>()).write(None);
    }
}
pub(crate) unsafe extern "C" fn Task_UseFly(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !(((((task).wrapping_add(8)).cast::<i16>()).read()) != 0) {
            if !((IsWeatherNotFadingIn()) != 0) {
                return;
            }
            (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .write(((GetCursorSelectionMonId()) as i32));
            if (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read()
                > 5i32
            {
                (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .write(0i32);
            }
            FieldEffectStart(31u8);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if !((FieldEffectActiveListContains(31u8)) != 0) {
            Overworld_ResetStateAfterFly();
            WarpIntoMap();
            SetMainCallback2(Some(CB2_LoadMap));
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCallback_FlyIntoMap));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_FlyIntoMap() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        FadeInFromBlack();
        CreateTask(Some(Task_FlyIntoMap), 0u8);
        crate::c::bf_write(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(1),
            5,
            1,
            (1u32) as i32,
        );
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 8i32) != 0 {
            ObjectEventTurn(
                ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
                3u8,
            );
        }
        LockPlayerFieldControls();
        FreezeObjectEvents();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>()).write(None);
    }
}
pub(crate) unsafe extern "C" fn Task_FlyIntoMap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) == 0i32 {
            if (crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0
            {
                return;
            }
            FieldEffectStart(32u8);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if !((FieldEffectActiveListContains(32u8)) != 0) {
            UnlockPlayerFieldControls();
            UnfreezeObjectEvents();
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_FallWarpExit() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        WarpFadeInScreen();
        LockPlayerFieldControls();
        FreezeObjectEvents();
        CreateTask(Some(Task_FallWarpFieldEffect), 0u8);
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>()).write(None);
    }
}
pub(crate) unsafe extern "C" fn Task_FallWarpFieldEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: loop {
            if !(((((((&raw const sFallWarpFieldEffectFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(task))
                != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FallWarpEffect_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut playerObject: *mut u8 = core::ptr::null_mut();
        let mut playerSprite: *mut u8 = core::ptr::null_mut();
        playerObject = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        playerSprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        CameraObjectFreeze();
        crate::c::bf_write(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(1),
            5,
            1,
            (1u32) as i32,
        );
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        ObjectEventSetHeldMovement(
            playerObject,
            GetFaceDirectionMovementAction(((GetPlayerFacingDirection()) as u32)),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((crate::c::bf_read((playerSprite).wrapping_add(66), 6, 2, false) as u8) as i16),
        );
        crate::c::bf_write((playerObject).wrapping_add(3), 2, 1, (1u32) as i32);
        crate::c::bf_write((playerSprite).wrapping_add(5), 2, 2, (1u16) as i32);
        crate::c::bf_write((playerSprite).wrapping_add(66), 6, 2, (2u8) as i32);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FallWarpEffect_WaitWeather(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (IsWeatherNotFadingIn()) != 0 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FallWarpEffect_StartFall(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        let mut centerToCornerVecY: i16 = 0i16;
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        centerToCornerVecY = (((((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32) << 1)
            .wrapping_neg()) as i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)))
            .wrapping_add(((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)))
            .wrapping_add(((centerToCornerVecY) as i32)))
            .wrapping_neg()) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        crate::c::bf_write(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(1),
            5,
            1,
            (0u32) as i32,
        );
        PlaySE(43u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FallWarpEffect_Fall(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        let mut sprite: *mut u8 = core::ptr::null_mut();
        objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) < 8i32 {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
                )) as i16),
            );
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                & 15i32)
                != 0
            {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32) << 1) as i16));
            }
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) == 0i32)
            && (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= (-16i32))
        {
            let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            (__p4).write(((__p4).read()).wrapping_add(1));
            crate::c::bf_write((objectEvent).wrapping_add(3), 2, 1, (0u32) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(66),
                6,
                2,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8)
                    as i32,
            );
            crate::c::bf_write((objectEvent).wrapping_add(0), 2, 1, (1u32) as i32);
        }
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= 0i32 {
            PlaySE(214u16);
            crate::c::bf_write((objectEvent).wrapping_add(0), 3, 1, (1u32) as i32);
            crate::c::bf_write((objectEvent).wrapping_add(0), 5, 1, (1u32) as i32);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            let __p5 = ((task).wrapping_add(8)).cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FallWarpEffect_Land(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(4i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        SetCameraPanningCallback(None);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FallWarpEffect_CameraShake(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        SetCameraPanning(
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        );
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
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FallWarpEffect_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
        UnlockPlayerFieldControls();
        CameraObjectReset();
        UnfreezeObjectEvents();
        InstallCameraPanAheadCallback();
        DestroyTask(FindTaskIdByFunc(Some(Task_FallWarpFieldEffect)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartEscalatorWarp(metatileBehavior: u8, priority: u8) {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        let mut priority = priority;
        let mut taskId: u8 = 0u8;
        taskId = CreateTask(Some(Task_EscalatorWarpOut), priority);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        if ((metatileBehavior) as i32) == 106i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(1i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_EscalatorWarpOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: loop {
            if !(((((((&raw const sEscalatorWarpOutFieldEffectFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(task))
                != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        FreezeObjectEvents();
        CameraObjectFreeze();
        StartEscalator(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_WaitForPlayer(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (!((ObjectEventIsMovementOverridden(objectEvent)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0)
        {
            ObjectEventSetHeldMovement(
                objectEvent,
                GetFaceDirectionMovementAction(((GetPlayerFacingDirection()) as u32)),
            );
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8)
                as i32)
                == 0i32
            {
                (((task).wrapping_add(8)).cast::<i16>()).write(4i16);
            }
            PlaySE(80u16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Up_Ride(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        RideUpEscalatorOut(task);
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) > 3i32 {
            FadeOutAtEndOfEscalator();
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Up_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        RideUpEscalatorOut(task);
        WarpAtEndOfEscalator();
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Down_Ride(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        RideDownEscalatorOut(task);
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) > 3i32 {
            FadeOutAtEndOfEscalator();
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpOut_Down_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        RideDownEscalatorOut(task);
        WarpAtEndOfEscalator();
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn RideUpEscalatorOut(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            148i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
        ));
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) & 1i32)
            != 0
        {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn RideDownEscalatorOut(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            124i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            118i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
        ));
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) & 1i32)
            != 0
        {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FadeOutAtEndOfEscalator() {
    unsafe {
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
    }
}
pub(crate) unsafe extern "C" fn WarpAtEndOfEscalator() {
    unsafe {
        if (!((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0))
            && (((BGMusicStopped()) as i32) == 1i32)
        {
            StopEscalator();
            WarpIntoMap();
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCallback_EscalatorWarpIn));
            SetMainCallback2(Some(CB2_LoadMap));
            DestroyTask(FindTaskIdByFunc(Some(Task_EscalatorWarpOut)));
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_EscalatorWarpIn() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        WarpFadeInScreen();
        LockPlayerFieldControls();
        CreateTask(Some(Task_EscalatorWarpIn), 0u8);
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>()).write(None);
    }
}
pub(crate) unsafe extern "C" fn Task_EscalatorWarpIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: loop {
            if !(((((((&raw const sEscalatorWarpInFieldEffectFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(task))
                != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut behavior: u8 = 0u8;
        CameraObjectFreeze();
        objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        ObjectEventSetHeldMovement(objectEvent, GetFaceDirectionMovementAction(4u32));
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        behavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(16i16);
        if ((behavior) as i32) == 107i32 {
            behavior = 1u8;
            (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
        } else {
            behavior = 0u8;
        }
        StartEscalator(behavior);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Down_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            148i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Down_Ride(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            148i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) & 1i32)
            != 0
        {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_sub(1));
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            (((task).wrapping_add(8)).cast::<i16>()).write(5i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Up_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            124i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            118i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_Up_Ride(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            124i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            118i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) & 1i32)
            != 0
        {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_sub(1));
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_WaitForMovement(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (IsEscalatorMoving()) != 0 {
            return 0u8;
        }
        StopEscalator();
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn EscalatorWarpIn_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0 {
            CameraObjectReset();
            UnlockPlayerFieldControls();
            ObjectEventSetHeldMovement(objectEvent, GetWalkNormalMovementAction(4u32));
            DestroyTask(FindTaskIdByFunc(Some(Task_EscalatorWarpIn)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseWaterfall() -> u8 {
    unsafe {
        let mut taskId: u8 = 0u8;
        taskId = CreateTask(Some(Task_UseWaterfall), 255u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as i16),
        );
        Task_UseWaterfall(taskId);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_UseWaterfall(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sWaterfallFieldEffectFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u8>>())
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
                ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_Init(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        LockPlayerFieldControls();
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_ShowMon(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        LockPlayerFieldControls();
        if !((ObjectEventIsMovementOverridden(objectEvent)) != 0) {
            ObjectEventClearHeldMovementIfFinished(objectEvent);
            (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            );
            FieldEffectStart(59u8);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_WaitForShowMon(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        if (FieldEffectActiveListContains(6u8)) != 0 {
            return 0u8;
        }
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_RideUp(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        ObjectEventSetHeldMovement(objectEvent, GetWalkSlowMovementAction(2u32));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaterfallFieldEffect_ContinueRideOrEnd(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        if !((ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0) {
            return 0u8;
        }
        if (MetatileBehavior_IsWaterfall(((objectEvent).wrapping_add(30)).read())) != 0 {
            (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
            return 1u8;
        }
        UnlockPlayerFieldControls();
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
        DestroyTask(FindTaskIdByFunc(Some(Task_UseWaterfall)));
        FieldEffectActiveListRemove(43u8);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseDive() -> u8 {
    unsafe {
        let mut taskId: u8 = 0u8;
        taskId = CreateTask(Some(Task_UseDive), 255u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .write(
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(1))
            .read()) as i16),
        );
        Task_UseDive(taskId);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_UseDive(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sDiveFieldEffectFuncs)
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
pub(crate) unsafe extern "C" fn DiveFieldEffect_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DiveFieldEffect_ShowMon(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        LockPlayerFieldControls();
        (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).write(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32),
        );
        FieldEffectStart(59u8);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DiveFieldEffect_TryWarp(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut mapPosition = crate::ffi::Align4([0u8; 8]);
        PlayerGetDestCoords(
            ((&raw mut mapPosition).cast::<u8>()).cast::<i16>(),
            ((&raw mut mapPosition).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>(),
        );
        if !((FieldEffectActiveListContains(6u8)) != 0) {
            TryDoDiveWarp(
                (&raw mut mapPosition).cast::<u8>(),
                ((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(30))
                .read()) as u16),
            );
            DestroyTask(FindTaskIdByFunc(Some(Task_UseDive)));
            FieldEffectActiveListRemove(44u8);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartLavaridgeGymB1FWarp(priority: u8) {
    unsafe {
        let mut priority = priority;
        CreateTask(Some(Task_LavaridgeGymB1FWarp), priority);
    }
}
pub(crate) unsafe extern "C" fn Task_LavaridgeGymB1FWarp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sLavaridgeGymB1FWarpEffectFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u8) -> u8>>())
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
                ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_Init(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        FreezeObjectEvents();
        CameraObjectFreeze();
        SetCameraPanningCallback(None);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        crate::c::bf_write((objectEvent).wrapping_add(3), 2, 1, (1u32) as i32);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_CameraShake(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        SetCameraPanning(
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_neg()) as i16),
        );
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) > 7i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_Launch(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
        (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
            .write((((((objectEvent).wrapping_add(16)).cast::<i16>()).read()) as i32));
        ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
            .wrapping_offset(1))
        .write(
            (((((objectEvent).wrapping_add(16))
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32),
        );
        ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
            .wrapping_offset(2))
        .write(((((sprite).wrapping_add(67)).read()) as i32).wrapping_sub(1i32));
        ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
            .wrapping_offset(3))
        .write(((crate::c::bf_read((sprite).wrapping_add(5), 2, 2, false) as u16) as i32));
        FieldEffectStart(50u8);
        PlaySE(178u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_Rise(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        let mut centerToCornerVecY: i16 = 0i16;
        SetCameraPanning(
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        );
        if {
            let _ = {
                let __v1 =
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_neg()) as i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(__v1);
                __v1
            };
            (({
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                <= 17i32
        } {
            if (!((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                & 1i32)
                != 0))
                && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    <= 3i32)
            {
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p4).write((((((__p4).read()) as i32) << 1) as i16));
            }
        } else {
            if (!((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                & 4i32)
                != 0))
                && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    > 0i32)
            {
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p5).write((((((__p5).read()) as i32) >> 1) as i16));
            }
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) > 6i32 {
            centerToCornerVecY = (((((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)
                << 1)
                .wrapping_neg()) as i16);
            if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)
                > (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)))
                .wrapping_add(((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)))
                .wrapping_add(((centerToCornerVecY) as i32)))
                .wrapping_neg()
            {
                let __p6 = (sprite).wrapping_add(38).cast::<i16>();
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    <= 7i32
                {
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            }
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32) == 0i32)
            && (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) < (-16i32))
        {
            let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            (__p8).write(((__p8).read()).wrapping_add(1));
            crate::c::bf_write((objectEvent).wrapping_add(3), 2, 1, (1u32) as i32);
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
            crate::c::bf_write((sprite).wrapping_add(66), 6, 2, (2u8) as i32);
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32)
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                != 0i32)
        {
            let __p9 = ((task).wrapping_add(8)).cast::<i16>();
            (__p9).write(((__p9).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_FadeOut(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpEffect_Warp(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        if (!((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0))
            && (((BGMusicStopped()) as i32) == 1i32)
        {
            WarpIntoMap();
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_LavaridgeGymB1FWarpExit));
            SetMainCallback2(Some(CB2_LoadMap));
            DestroyTask(FindTaskIdByFunc(Some(Task_LavaridgeGymB1FWarp)));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FieldCB_LavaridgeGymB1FWarpExit() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        WarpFadeInScreen();
        LockPlayerFieldControls();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>()).write(None);
        CreateTask(Some(Task_LavaridgeGymB1FWarpExit), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_LavaridgeGymB1FWarpExit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sLavaridgeGymB1FWarpExitEffectFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u8) -> u8>>())
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
                ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpExitEffect_Init(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        CameraObjectFreeze();
        FreezeObjectEvents();
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        crate::c::bf_write((objectEvent).wrapping_add(1), 5, 1, (1u32) as i32);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpExitEffect_StartPopOut(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        if (IsWeatherNotFadingIn()) != 0 {
            (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .write((((((objectEvent).wrapping_add(16)).cast::<i16>()).read()) as i32));
            ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(1))
            .write(
                (((((objectEvent).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32),
            );
            ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(2))
            .write(((((sprite).wrapping_add(67)).read()) as i32).wrapping_sub(1i32));
            ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(3))
            .write(((crate::c::bf_read((sprite).wrapping_add(5), 2, 2, false) as u16) as i32));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                .write(((FieldEffectStart(49u8)) as i16));
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpExitEffect_PopOut(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                as isize
                * 68,
        );
        if ((((sprite).wrapping_add(43)).read()) as i32) > 1i32 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            crate::c::bf_write((objectEvent).wrapping_add(1), 5, 1, (0u32) as i32);
            CameraObjectReset();
            PlaySE(175u16);
            ObjectEventSetHeldMovement(objectEvent, GetJumpMovementAction(4u32));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGymB1FWarpExitEffect_End(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        if (ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0 {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
            UnlockPlayerFieldControls();
            UnfreezeObjectEvents();
            DestroyTask(FindTaskIdByFunc(Some(Task_LavaridgeGymB1FWarpExit)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_AshLaunch() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(1))
            .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(33))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(1))
            .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(2))
            .read()) as u8),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(3))
            .read()) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            1,
            1,
            (1u16) as i32,
        );
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_AshLaunch(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            FieldEffectStop(sprite, 50u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartLavaridgeGym1FWarp(priority: u8) {
    unsafe {
        let mut priority = priority;
        CreateTask(Some(Task_LavaridgeGym1FWarp), priority);
    }
}
pub(crate) unsafe extern "C" fn Task_LavaridgeGym1FWarp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sLavaridgeGym1FWarpEffectFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u8) -> u8>>())
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
                ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_Init(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        FreezeObjectEvents();
        CameraObjectFreeze();
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        crate::c::bf_write((objectEvent).wrapping_add(3), 2, 1, (1u32) as i32);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_AshPuff(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        if (ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0 {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 3i32
            {
                (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .write((((((objectEvent).wrapping_add(16)).cast::<i16>()).read()) as i32));
                ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .write(
                    (((((objectEvent).wrapping_add(16))
                        .wrapping_add(2)
                        .cast::<i16>())
                    .read()) as i32),
                );
                ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .write(((((sprite).wrapping_add(67)).read()) as i32).wrapping_sub(1i32));
                ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .write(((crate::c::bf_read((sprite).wrapping_add(5), 2, 2, false) as u16) as i32));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                    .write(((FieldEffectStart(49u8)) as i16));
                let __p1 = ((task).wrapping_add(8)).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ObjectEventSetHeldMovement(
                    objectEvent,
                    GetWalkInPlaceFasterMovementAction(
                        ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16)
                            as u32),
                    ),
                );
                PlaySE(39u16);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_Disappear(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(43))
        .read()) as i32)
            == 2i32
        {
            crate::c::bf_write((objectEvent).wrapping_add(1), 5, 1, (1u32) as i32);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_FadeOut(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        if !((FieldEffectActiveListContains(49u8)) != 0) {
            TryFadeOutOldMapMusic();
            WarpFadeOutScreen();
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn LavaridgeGym1FWarpEffect_Warp(
    task: *mut u8,
    objectEvent: *mut u8,
    sprite: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        if (!((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0))
            && (((BGMusicStopped()) as i32) == 1i32)
        {
            WarpIntoMap();
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_FallWarpExit));
            SetMainCallback2(Some(CB2_LoadMap));
            DestroyTask(FindTaskIdByFunc(Some(Task_LavaridgeGym1FWarp)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_AshPuff() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(1))
            .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(32))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(1))
            .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(2))
            .read()) as u8),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(3))
            .read()) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            1,
            1,
            (1u16) as i32,
        );
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_AshPuff(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            FieldEffectStop(sprite, 49u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartEscapeRopeFieldEffect() {
    unsafe {
        LockPlayerFieldControls();
        FreezeObjectEvents();
        CreateTask(Some(Task_EscapeRopeWarpOut), 80u8);
    }
}
pub(crate) unsafe extern "C" fn Task_EscapeRopeWarpOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sEscapeRopeWarpOutEffectFuncs)
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
pub(crate) unsafe extern "C" fn EscapeRopeWarpOutEffect_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(64i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetPlayerFacingDirection()) as i16));
    }
}
pub(crate) unsafe extern "C" fn EscapeRopeWarpOutEffect_Spin(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        let mut spinDirections = crate::ffi::Align4([0u8; 5]);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(0)
            .write(1u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(1)
            .write(3u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(2)
            .write(4u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(3)
            .write(2u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(4)
            .write(1u8);
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
            != 0i32)
            && ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            TryFadeOutOldMapMusic();
            WarpFadeOutScreen();
        }
        objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (!((ObjectEventIsMovementOverridden(objectEvent)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0)
        {
            if ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
                == 0i32)
                && (!((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)))
                && (((BGMusicStopped()) as i32) == 1i32)
            {
                SetObjectEventDirection(
                    objectEvent,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                );
                SetWarpDestinationToEscapeWarp();
                WarpIntoMap();
                ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(FieldCallback_EscapeRopeWarpIn));
                SetMainCallback2(Some(CB2_LoadMap));
                DestroyTask(FindTaskIdByFunc(Some(Task_EscapeRopeWarpOut)));
            } else {
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 0i32)
                    || ((({
                        let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                        let __t4 = ((__p3).read()).wrapping_sub(1);
                        (__p3).write(__t4);
                        __t4
                    }) as i32)
                        == 0i32)
                {
                    ObjectEventSetHeldMovement(
                        objectEvent,
                        GetFaceDirectionMovementAction(
                            (((((&raw mut spinDirections).cast::<u8>()).wrapping_offset(
                                ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false)
                                    as u16) as i32) as isize,
                            ))
                            .read()) as u32),
                        ),
                    );
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        < 12i32
                    {
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    }
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                        ((crate::c::shr_i32(
                            8i32,
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32)
                                >> 2) as u32),
                        )) as i16),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_EscapeRopeWarpIn() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        WarpFadeInScreen();
        LockPlayerFieldControls();
        FreezeObjectEvents();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>()).write(None);
        crate::c::bf_write(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(1),
            5,
            1,
            (1u32) as i32,
        );
        CreateTask(Some(Task_EscapeRopeWarpIn), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_EscapeRopeWarpIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sEscapeRopeWarpInEffectFuncs)
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
pub(crate) unsafe extern "C" fn EscapeRopeWarpInEffect_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (IsWeatherNotFadingIn()) != 0 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                .write(((GetPlayerFacingDirection()) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn EscapeRopeWarpInEffect_Spin(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut spinDirections = crate::ffi::Align4([0u8; 5]);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(0)
            .write(1u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(1)
            .write(3u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(2)
            .write(4u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(3)
            .write(2u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(4)
            .write(1u8);
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            if ((ObjectEventIsMovementOverridden(objectEvent)) != 0)
                && (!((ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0))
            {
                return;
            }
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >= 32i32)
                && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                    as i32)
                    == ((GetPlayerFacingDirection()) as i32))
            {
                crate::c::bf_write((objectEvent).wrapping_add(1), 5, 1, (0u32) as i32);
                UnlockPlayerFieldControls();
                UnfreezeObjectEvents();
                DestroyTask(FindTaskIdByFunc(Some(Task_EscapeRopeWarpIn)));
                return;
            }
            ObjectEventSetHeldMovement(
                objectEvent,
                GetFaceDirectionMovementAction(
                    (((((&raw mut spinDirections).cast::<u8>()).wrapping_offset(
                        ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16)
                            as i32) as isize,
                    ))
                    .read()) as u32),
                ),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                < 32i32
            {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    >> 2) as i16),
            );
        }
        crate::c::bf_write(
            (objectEvent).wrapping_add(1),
            5,
            1,
            ((crate::c::bf_read((objectEvent).wrapping_add(1), 5, 1, false) as u32) ^ 1u32) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_TeleportWarpOut() {
    unsafe {
        CreateTask(Some(Task_TeleportWarpOut), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_TeleportWarpOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sTeleportWarpOutFieldEffectFuncs)
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
pub(crate) unsafe extern "C" fn TeleportWarpOutFieldEffect_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        LockPlayerFieldControls();
        FreezeObjectEvents();
        CameraObjectFreeze();
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetPlayerFacingDirection()) as i16));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn TeleportWarpOutFieldEffect_SpinGround(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut spinDirections = crate::ffi::Align4([0u8; 5]);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(0)
            .write(1u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(1)
            .write(3u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(2)
            .write(4u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(3)
            .write(2u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(4)
            .write(1u8);
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            ObjectEventTurn(
                objectEvent,
                (((&raw mut spinDirections).cast::<u8>()).wrapping_offset(
                    ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16) as i32)
                        as isize,
                ))
                .read(),
            );
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(8i16);
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) > 7i32)
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                == ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16) as i32))
        {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(4i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(8i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
            PlaySE(45u16);
        }
    }
}
pub(crate) unsafe extern "C" fn TeleportWarpOutFieldEffect_SpinExit(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut spinDirections = crate::ffi::Align4([0u8; 5]);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(0)
            .write(1u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(1)
            .write(3u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(2)
            .write(4u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(3)
            .write(2u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(4)
            .write(1u8);
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            <= 0i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(4i16);
            ObjectEventTurn(
                objectEvent,
                (((&raw mut spinDirections).cast::<u8>()).wrapping_offset(
                    ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16) as i32)
                        as isize,
                ))
                .read(),
            );
        }
        let __p3 = (sprite).wrapping_add(34).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_sub(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            )) as i16),
        );
        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_add(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            )) as i16),
        );
        if ((({
            let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t6 = ((__p5).read()).wrapping_sub(1);
            (__p5).write(__t6);
            __t6
        }) as i32)
            <= 0i32)
            && ({
                let _ = {
                    let __v7 = 4i16;
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(__v7);
                    __v7
                };
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    < 8i32
            })
        {
            let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            (__p8).write((((((__p8).read()) as i32) << 1) as i16));
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) > 8i32)
            && ({
                let _ = {
                    let __v9 = 1u16;
                    crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (__v9) as i32);
                    __v9
                };
                ((crate::c::bf_read((sprite).wrapping_add(66), 6, 2, false) as u8) as i32) != 0i32
            })
        {
            crate::c::bf_write((sprite).wrapping_add(66), 6, 2, (2u8) as i32);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >= 168i32
        {
            let __p10 = ((task).wrapping_add(8)).cast::<i16>();
            (__p10).write(((__p10).read()).wrapping_add(1));
            TryFadeOutOldMapMusic();
            WarpFadeOutScreen();
        }
    }
}
pub(crate) unsafe extern "C" fn TeleportWarpOutFieldEffect_End(task: *mut u8) {
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
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 0i32
            {
                ClearMirageTowerPulseBlendEffect();
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(1i16);
            }
            if ((BGMusicStopped()) as i32) == 1i32 {
                SetWarpDestinationToLastHealLocation();
                WarpIntoMap();
                SetMainCallback2(Some(CB2_LoadMap));
                ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(FieldCallback_TeleportWarpIn));
                DestroyTask(FindTaskIdByFunc(Some(Task_TeleportWarpOut)));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_TeleportWarpIn() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        WarpFadeInScreen();
        LockPlayerFieldControls();
        FreezeObjectEvents();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>()).write(None);
        crate::c::bf_write(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(1),
            5,
            1,
            (1u32) as i32,
        );
        CameraObjectFreeze();
        CreateTask(Some(Task_TeleportWarpIn), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_TeleportWarpIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sTeleportWarpInFieldEffectFuncs)
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
pub(crate) unsafe extern "C" fn TeleportWarpInFieldEffect_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        let mut centerToCornerVecY: i16 = 0i16;
        if (IsWeatherNotFadingIn()) != 0 {
            sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            );
            centerToCornerVecY = (((((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)
                << 1)
                .wrapping_neg()) as i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)))
                .wrapping_add(((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)))
                .wrapping_add(((centerToCornerVecY) as i32)))
                .wrapping_neg()) as i16),
            );
            crate::c::bf_write(
                (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(1),
                5,
                1,
                (0u32) as i32,
            );
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(8i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                .write(((crate::c::bf_read((sprite).wrapping_add(66), 6, 2, false) as u8) as i16));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                .write(((GetPlayerFacingDirection()) as i16));
            PlaySE(45u16);
        }
    }
}
pub(crate) unsafe extern "C" fn TeleportWarpInFieldEffect_SpinEnter(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut spinDirections = crate::ffi::Align4([0u8; 5]);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(0)
            .write(1u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(1)
            .write(3u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(2)
            .write(4u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(3)
            .write(2u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(4)
            .write(1u8);
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        if (({
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            let __v2 = (((((__p1).read()) as i32).wrapping_add(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16);
            (__p1).write(__v2);
            __v2
        }) as i32)
            >= (-8i32)
        {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32)
                == 0i32
            {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13);
                (__p3).write(((__p3).read()).wrapping_add(1));
                crate::c::bf_write((objectEvent).wrapping_add(0), 2, 1, (1u32) as i32);
                crate::c::bf_write(
                    (sprite).wrapping_add(66),
                    6,
                    2,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8)
                        as i32,
                );
            }
        } else {
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
            if ((crate::c::bf_read((sprite).wrapping_add(66), 6, 2, false) as u8) as i32) != 0i32 {
                crate::c::bf_write((sprite).wrapping_add(66), 6, 2, (2u8) as i32);
            }
        }
        if ((((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= (-48i32))
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 1i32))
            && (!((((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) & 1i32) != 0))
        {
            let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p4).write(((__p4).read()).wrapping_sub(1));
        }
        if (({
            let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t6 = ((__p5).read()).wrapping_sub(1);
            (__p5).write(__t6);
            __t6
        }) as i32)
            == 0i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(4i16);
            ObjectEventTurn(
                objectEvent,
                (((&raw mut spinDirections).cast::<u8>()).wrapping_offset(
                    ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16) as i32)
                        as isize,
                ))
                .read(),
            );
        }
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= 0i32 {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            let __p7 = ((task).wrapping_add(8)).cast::<i16>();
            (__p7).write(((__p7).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn TeleportWarpInFieldEffect_SpinGround(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut spinDirections = crate::ffi::Align4([0u8; 5]);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(0)
            .write(1u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(1)
            .write(3u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(2)
            .write(4u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(3)
            .write(2u8);
        (&raw mut spinDirections)
            .cast::<u8>()
            .wrapping_add(4)
            .write(1u8);
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            ObjectEventTurn(
                objectEvent,
                (((&raw mut spinDirections).cast::<u8>()).wrapping_offset(
                    ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16) as i32)
                        as isize,
                ))
                .read(),
            );
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(8i16);
            if ((({
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                > 4i32)
                && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                    as i32)
                    == ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16)
                        as i32))
            {
                UnlockPlayerFieldControls();
                CameraObjectReset();
                UnfreezeObjectEvents();
                DestroyTask(FindTaskIdByFunc(Some(Task_TeleportWarpIn)));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_FieldMoveShowMon() -> u8 {
    unsafe {
        let mut taskId: u8 = 0u8;
        if ((IsMapTypeOutdoors(GetCurrentMapType())) as i32) == 1i32 {
            taskId = CreateTask(Some(Task_FieldMoveShowMonOutdoors), 255u8);
        } else {
            taskId = CreateTask(Some(Task_FieldMoveShowMonIndoors), 255u8);
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(
            ((InitFieldMoveMonSprite(
                (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .read()) as u32),
                ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as u32),
                ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as u32),
            )) as i16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_FieldMoveShowMonInit() -> u8 {
    unsafe {
        let mut pokemon: *mut u8 = core::ptr::null_mut();
        let mut noDucking: u32 =
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read()
                & (-2147483648i32)) as u32);
        pokemon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as u8) as i32) as isize
                * 100,
        );
        (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
            .write(((GetMonData2(pokemon, 11i32)) as i32));
        ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
            .wrapping_offset(1))
        .write(((GetMonData2(pokemon, 1i32)) as i32));
        ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
            .wrapping_offset(2))
        .write(((GetMonData2(pokemon, 0i32)) as i32));
        let __p1 = ((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>();
        (__p1).write((((((__p1).read()) as u32) | noDucking) as i32));
        FieldEffectStart(6u8);
        FieldEffectActiveListRemove(59u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_FieldMoveShowMonOutdoors(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sFieldMoveShowMonOutdoorsEffectFuncs)
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
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
            .write(((((67108936i32) as usize as *mut u16).read_volatile()) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
            .write(((((67108938i32) as usize as *mut u16).read_volatile()) as i16));
        StoreWordInTwoHalfwords(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).cast::<u16>(),
            (core::mem::transmute::<_, usize>(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(12)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) as u32),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write((-3855i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
            (((crate::c::div_i32(160i32, 2i32) << 8)
                | (crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(63i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(62i16);
        SetGpuReg(
            64u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
        );
        SetGpuReg(
            68u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
        );
        SetGpuReg(
            72u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u16),
        );
        SetGpuReg(
            74u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u16),
        );
        SetVBlankCallback(Some(VBlankCB_FieldMoveShowMonOutdoors));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_LoadGfx(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut offset: u16 =
            (((((((67108872i32) as usize as *mut u16).read_volatile()) as i32) >> 2) << 14) as u16);
        let mut delta: u16 =
            (((((((67108872i32) as usize as *mut u16).read_volatile()) as i32) >> 8) << 11) as u16);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw const sFieldMoveStreaksOutdoors_Gfx)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (((100663296i32).wrapping_add(((offset) as i32))) as usize as *mut u8),
                            ((0i32
                                | (crate::c::div_i32(512i32, crate::c::div_i32(16i32, 8i32))
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
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((100663296i32).wrapping_add(((delta) as i32))) as usize
                                    as *mut u8),
                                ((83886080i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        LoadPalette(
            (((&raw const sFieldMoveStreaksOutdoors_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        LoadFieldMoveOutdoorStreaksTilemap(delta);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_CreateBanner(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut horiz: i16 = 0i16;
        let mut vertHi: i16 = 0i16;
        let mut vertLo: i16 = 0i16;
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(16i32)) as i16));
        horiz = (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16)
            as i32)
            >> 8) as i16);
        vertHi = (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16)
            as i32)
            >> 8) as i16);
        vertLo = (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16)
            as i32)
            & 255i32) as i16);
        horiz = ((((horiz) as i32).wrapping_sub(16i32)) as i16);
        vertHi = ((((vertHi) as i32).wrapping_sub(2i32)) as i16);
        vertLo = ((((vertLo) as i32).wrapping_add(2i32)) as i16);
        if ((horiz) as i32) < 0i32 {
            horiz = 0i16;
        }
        if ((vertHi) as i32) < crate::c::div_i32(160i32, 4i32) {
            vertHi = ((crate::c::div_i32(160i32, 4i32)) as i16);
        }
        if ((vertLo) as i32) > crate::c::div_i32(240i32, 2i32) {
            vertLo = ((crate::c::div_i32(240i32, 2i32)) as i16);
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
            (((((horiz) as i32) << 8)
                | (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    & 255i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
            .write((((((vertHi) as i32) << 8) | ((vertLo) as i32)) as i16));
        if ((((horiz) as i32) == 0i32) && (((vertHi) as i32) == crate::c::div_i32(160i32, 4i32)))
            && (((vertLo) as i32) == crate::c::div_i32(240i32, 2i32))
        {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_FieldMoveMonSlideOnscreen));
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_WaitForMon(task: *mut u8) {
    unsafe {
        let mut task = task;
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(16i32)) as i16));
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .read())
            != 0
        {
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_ShrinkBanner(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut vertHi: i16 = 0i16;
        let mut vertLo: i16 = 0i16;
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(16i32)) as i16));
        vertHi = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >> 8) as i16);
        vertLo = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            & 255i32) as i16);
        vertHi = ((((vertHi) as i32).wrapping_add(6i32)) as i16);
        vertLo = ((((vertLo) as i32).wrapping_sub(6i32)) as i16);
        if ((vertHi) as i32) > crate::c::div_i32(160i32, 2i32) {
            vertHi = ((crate::c::div_i32(160i32, 2i32)) as i16);
        }
        if ((vertLo) as i32) < (crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32) {
            vertLo = (((crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32)) as i16);
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
            .write((((((vertHi) as i32) << 8) | ((vertLo) as i32)) as i16));
        if (((vertHi) as i32) == crate::c::div_i32(160i32, 2i32))
            && (((vertLo) as i32) == (crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32))
        {
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_RestoreBg(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut bg0cnt: u16 =
            (((((((67108872i32) as usize as *mut u16).read_volatile()) as i32) >> 8) << 11) as u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((100663296i32) as usize as *mut u8)
                                    .wrapping_offset(((bg0cnt) as i32) as isize * 1),
                                ((83886080i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(32i32, 8i32))
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
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(241i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(161i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
            .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read());
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
            .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read());
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonOutdoorsEffect_End(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut callback: Option<unsafe extern "C" fn()> = None;
        LoadWordFromTwoHalfwords(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).cast::<u16>(),
            (&raw mut callback).cast::<u32>(),
        );
        SetVBlankCallback(callback);
        InitTextBoxGfxAndPrinters();
        FreeResourcesAndDestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
        );
        FieldEffectActiveListRemove(6u8);
        DestroyTask(FindTaskIdByFunc(Some(Task_FieldMoveShowMonOutdoors)));
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_FieldMoveShowMonOutdoors() {
    unsafe {
        let mut callback: Option<unsafe extern "C" fn()> = None;
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((FindTaskIdByFunc(Some(Task_FieldMoveShowMonOutdoors))) as i32) as isize * 40,
        );
        LoadWordFromTwoHalfwords(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).cast::<u16>(),
            (&raw mut callback).cast::<u32>(),
        );
        (callback).unwrap_unchecked()();
        SetGpuReg(
            64u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
        );
        SetGpuReg(
            68u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
        );
        SetGpuReg(
            72u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u16),
        );
        SetGpuReg(
            74u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u16),
        );
        SetGpuReg(
            16u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as u16),
        );
        SetGpuReg(
            18u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn LoadFieldMoveOutdoorStreaksTilemap(offs: u16) {
    unsafe {
        let mut offs = offs;
        let mut i: u16 = 0u16;
        let mut dest: *mut u16 = core::ptr::null_mut();
        dest = ((((100663296u32).wrapping_add(crate::c::div_u32(640u32, 2u32)))
            .wrapping_add(((offs) as u32))) as usize as *mut u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(640u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    (dest).write(
                        ((((((((&raw const sFieldMoveStreaksOutdoors_Tilemap)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            | 61440i32) as u16),
                    );
                }
                i = (i).wrapping_add(1);
                dest = (dest).wrapping_offset(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FieldMoveShowMonIndoors(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sFieldMoveShowMonIndoorsEffectFuncs)
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
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        SetGpuReg(
            16u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
        );
        SetGpuReg(
            18u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
        );
        StoreWordInTwoHalfwords(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).cast::<u16>(),
            (core::mem::transmute::<_, usize>(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(12)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) as u32),
        );
        SetVBlankCallback(Some(VBlankCB_FieldMoveShowMonIndoors));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_LoadGfx(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut offset: u16 = 0u16;
        let mut delta: u16 = 0u16;
        offset =
            (((((((67108872i32) as usize as *mut u16).read_volatile()) as i32) >> 2) << 14) as u16);
        delta =
            (((((((67108872i32) as usize as *mut u16).read_volatile()) as i32) >> 8) << 11) as u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(((delta) as i16));
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw const sFieldMoveStreaksIndoors_Gfx)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (((100663296i32).wrapping_add(((offset) as i32))) as usize as *mut u8),
                            ((0i32
                                | (crate::c::div_i32(128i32, crate::c::div_i32(16i32, 8i32))
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
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((100663296i32).wrapping_add(((delta) as i32))) as usize
                                    as *mut u8),
                                ((83886080i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        LoadPalette(
            (((&raw const sFieldMoveStreaksIndoors_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_SlideBannerOn(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (SlideIndoorBannerOnscreen(task)) != 0 {
            SetGpuReg(66u8, 240u16);
            SetGpuReg(
                70u8,
                (((crate::c::div_i32(160i32, 4i32) << 8)
                    | (160i32).wrapping_sub(crate::c::div_i32(160i32, 4i32)))
                    as u16),
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_FieldMoveMonSlideOnscreen));
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        AnimateIndoorShowMonBg(task);
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_WaitForMon(task: *mut u8) {
    unsafe {
        let mut task = task;
        AnimateIndoorShowMonBg(task);
        if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .read())
            != 0
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_RestoreBg(task: *mut u8) {
    unsafe {
        let mut task = task;
        AnimateIndoorShowMonBg(task);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                & 7i32) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        SetGpuReg(66u8, 65535u16);
        SetGpuReg(70u8, 65535u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_SlideBannerOff(task: *mut u8) {
    unsafe {
        let mut task = task;
        AnimateIndoorShowMonBg(task);
        if (SlideIndoorBannerOffscreen(task)) != 0 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FieldMoveShowMonIndoorsEffect_End(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut intrCallback: Option<unsafe extern "C" fn()> = None;
        let mut bg0cnt: u16 = 0u16;
        bg0cnt =
            (((((((67108872i32) as usize as *mut u16).read_volatile()) as i32) >> 8) << 11) as u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((100663296i32) as usize as *mut u8)
                                    .wrapping_offset(((bg0cnt) as i32) as isize * 1),
                                ((83886080i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(32i32, 8i32))
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
        LoadWordFromTwoHalfwords(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).cast::<u16>(),
            (&raw mut intrCallback).cast::<u32>(),
        );
        SetVBlankCallback(intrCallback);
        InitTextBoxGfxAndPrinters();
        FreeResourcesAndDestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
        );
        FieldEffectActiveListRemove(6u8);
        DestroyTask(FindTaskIdByFunc(Some(Task_FieldMoveShowMonIndoors)));
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_FieldMoveShowMonIndoors() {
    unsafe {
        let mut intrCallback: Option<unsafe extern "C" fn()> = None;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((FindTaskIdByFunc(Some(Task_FieldMoveShowMonIndoors))) as i32) as isize * 40,
        );
        LoadWordFromTwoHalfwords(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).cast::<u16>(),
            (&raw mut intrCallback).cast::<u32>(),
        );
        (intrCallback).unwrap_unchecked()();
        SetGpuReg(
            16u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
        );
        SetGpuReg(
            18u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimateIndoorShowMonBg(task: *mut u8) {
    unsafe {
        let mut task = task;
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(16i32)) as i16));
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(16i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn SlideIndoorBannerOnscreen(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        let mut srcOffs: u16 = 0u16;
        let mut dstOffs: u16 = 0u16;
        let mut dest: *mut u16 = core::ptr::null_mut();
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >= 32i32
        {
            return 1u8;
        }
        dstOffs = (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            >> 3)
            & 31i32) as u16);
        if ((dstOffs) as i32)
            >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            dstOffs = (((32i32).wrapping_sub(((dstOffs) as i32)) & 31i32) as u16);
            srcOffs = (((32i32).wrapping_sub(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            ) & 31i32) as u16);
            dest = (((100663616i32).wrapping_add(
                (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as u16)
                    as i32),
            )) as usize as *mut u16);
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((dest).wrapping_offset(
                            (((dstOffs) as i32).wrapping_add(((i) as i32).wrapping_mul(32i32)))
                                as isize,
                        ))
                        .write(
                            ((((&raw const sFieldMoveStreaksIndoors_Tilemap)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(
                                (((srcOffs) as i32).wrapping_add(((i) as i32).wrapping_mul(32i32)))
                                    as isize,
                            ))
                            .read(),
                        );
                        let __p1 = (dest).wrapping_offset(
                            (((dstOffs) as i32).wrapping_add(((i) as i32).wrapping_mul(32i32)))
                                as isize,
                        );
                        (__p1).write((((((__p1).read()) as i32) | 61440i32) as u16));
                        ((dest).wrapping_offset(
                            ((((dstOffs) as i32).wrapping_add(1i32) & 31i32)
                                .wrapping_add(((i) as i32).wrapping_mul(32i32)))
                                as isize,
                        ))
                        .write(
                            ((((((((&raw const sFieldMoveStreaksIndoors_Tilemap)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((srcOffs) as i32).wrapping_add(1i32) & 31i32)
                                    .wrapping_add(((i) as i32).wrapping_mul(32i32)))
                                    as isize,
                            ))
                            .read()) as i32)
                                | 61440i32) as u16),
                        );
                        let __p2 = (dest).wrapping_offset(
                            ((((dstOffs) as i32).wrapping_add(1i32) & 31i32)
                                .wrapping_add(((i) as i32).wrapping_mul(32i32)))
                                as isize,
                        );
                        (__p2).write((((((__p2).read()) as i32) | 61440i32) as u16));
                    }
                    i = (i).wrapping_add(1);
                }
            }
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SlideIndoorBannerOffscreen(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        let mut dstOffs: u16 = 0u16;
        let mut dest: *mut u16 = core::ptr::null_mut();
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) >= 32i32
        {
            return 1u8;
        }
        dstOffs = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            >> 3) as u16);
        if ((dstOffs) as i32)
            >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            dstOffs = (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                as i32)
                >> 3)
                & 31i32) as u16);
            dest = (((100663616i32).wrapping_add(
                (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as u16)
                    as i32),
            )) as usize as *mut u16);
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((dest).wrapping_offset(
                            (((dstOffs) as i32).wrapping_add(((i) as i32).wrapping_mul(32i32)))
                                as isize,
                        ))
                        .write(61440u16);
                        ((dest).wrapping_offset(
                            ((((dstOffs) as i32).wrapping_add(1i32) & 31i32)
                                .wrapping_add(((i) as i32).wrapping_mul(32i32)))
                                as isize,
                        ))
                        .write(61440u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn InitFieldMoveMonSprite(
    species: u32,
    otId: u32,
    personality: u32,
) -> u8 {
    unsafe {
        let mut species = species;
        let mut otId = otId;
        let mut personality = personality;
        let mut noDucking: u16 = 0u16;
        let mut monSprite: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        noDucking = (((species & 2147483648u32) >> 16) as u16);
        species = (species & 2147483647u32);
        monSprite =
            CreateMonSprite_FieldMove(((species) as u16), otId, personality, 320i16, 80i16, 0u8);
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((monSprite) as i32) as isize * 68);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (0u16) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((species) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((noDucking) as i16));
        return monSprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FieldMoveMonSlideOnscreen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            let __v2 = (((((__p1).read()) as i32).wrapping_sub(20i32)) as i16);
            (__p1).write(__v2);
            __v2
        }) as i32)
            <= crate::c::div_i32(240i32, 2i32)
        {
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((crate::c::div_i32(240i32, 2i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(30i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_FieldMoveMonWaitAfterCry));
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) != 0 {
                PlayCry_NormalNoDucking(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16),
                    0i8,
                    125i8,
                    10u8,
                );
            } else {
                PlayCry_Normal(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16),
                    0i8,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FieldMoveMonWaitAfterCry(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_FieldMoveMonSlideOffscreen));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FieldMoveMonSlideOffscreen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-64i32) {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
        } else {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(20i32)) as i16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseSurf() -> u8 {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_SurfFieldEffect), 255u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as i16),
        );
        Overworld_ClearSavedMusic();
        Overworld_ChangeMusicTo(365u16);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_SurfFieldEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sSurfFieldEffectFuncs)
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
pub(crate) unsafe extern "C" fn SurfFieldEffect_Init(task: *mut u8) {
    unsafe {
        let mut task = task;
        LockPlayerFieldControls();
        FreezeObjectEvents();
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        SetPlayerAvatarStateMask(8u8);
        PlayerGetDestCoords(
            (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1),
            (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2),
        );
        MoveCoords(
            ((crate::c::bf_read(
                (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(24),
                4,
                4,
                false,
            ) as u16) as u8),
            (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1),
            (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn SurfFieldEffect_FieldMovePose(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (!((ObjectEventIsMovementOverridden(objectEvent)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0)
        {
            SetPlayerAvatarFieldMove();
            ObjectEventSetHeldMovement(objectEvent, 57u8);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SurfFieldEffect_ShowMon(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (ObjectEventCheckHeldMovementStatus(objectEvent)) != 0 {
            (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).write(
                (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    | (-2147483648i32)),
            );
            FieldEffectStart(59u8);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SurfFieldEffect_JumpOnSurfBlob(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        if !((FieldEffectActiveListContains(6u8)) != 0) {
            objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
            ObjectEventSetGraphicsId(objectEvent, GetPlayerAvatarGraphicsIdByStateId(3u8));
            ObjectEventClearHeldMovementIfFinished(objectEvent);
            ObjectEventSetHeldMovement(
                objectEvent,
                GetJumpSpecialMovementAction(
                    ((crate::c::bf_read((objectEvent).wrapping_add(24), 4, 4, false) as u16)
                        as u32),
                ),
            );
            (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            );
            ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(1))
            .write(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32));
            ((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(2))
            .write((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32));
            ((objectEvent).wrapping_add(26)).write(((FieldEffectStart(8u8)) as u8));
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SurfFieldEffect_End(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0 {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
            let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>());
            (__p1).write((((((__p1).read()) as i32) & (-33i32)) as u8));
            ObjectEventSetHeldMovement(
                objectEvent,
                GetFaceDirectionMovementAction(
                    ((crate::c::bf_read((objectEvent).wrapping_add(24), 4, 4, false) as u16)
                        as u32),
                ),
            );
            SetSurfBlob_BobState(((objectEvent).wrapping_add(26)).read(), 1u8);
            UnfreezeObjectEvents();
            UnlockPlayerFieldControls();
            FieldEffectActiveListRemove(9u8);
            DestroyTask(FindTaskIdByFunc(Some(Task_SurfFieldEffect)));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_RayquazaSpotlight() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut k: u8 = 0u8;
        let mut spriteId: u8 = CreateSprite(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(36))
            .read(),
            120i16,
            (-24i16),
            1u8,
        );
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(5), 4, 4, (4u16) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write((-1i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        SetGpuReg(80u8, 15937u16);
        SetGpuReg(82u8, 3598u16);
        SetGpuReg(72u8, 16191u16);
        LoadPalette(
            (((&raw const sSpotlight_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            192u16,
            32u16,
        );
        SetGpuReg(18u8, 120u16);
        {
            i = 3u8;
            'l1: loop {
                if !(((i) as i32) < 15i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 12u8;
                        'l3: loop {
                            if !(((j) as i32) < 18i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((100726784i32) as usize as *mut u16).wrapping_offset(
                                    ((((i) as i32).wrapping_mul(32i32)).wrapping_add(((j) as i32)))
                                        as isize,
                                ))
                                .write(
                                    (((((49140i32).wrapping_add(((i) as i32).wrapping_mul(6i32)))
                                        .wrapping_add(((j) as i32)))
                                    .wrapping_add(1i32))
                                        as u16),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            k = 0u8;
            'l5: loop {
                if !(((k) as i32) < 90i32) {
                    break 'l5;
                }
                'l6: {
                    {
                        i = 0u8;
                        'l7: loop {
                            if !(((i) as i32) < 8i32) {
                                break 'l7;
                            }
                            'l8: {
                                ((((100696064i32).wrapping_add(
                                    (((k) as i32).wrapping_add(1i32)).wrapping_mul(32i32),
                                ))
                                .wrapping_add(((i) as i32).wrapping_mul(4i32)))
                                    as usize as *mut u16)
                                    .write(
                                        (((((((((&raw const sSpotlight_Gfx)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((k) as i32).wrapping_mul(32i32))
                                                .wrapping_add(((i) as i32).wrapping_mul(4i32)))
                                            .wrapping_add(1i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            << 8)
                                            .wrapping_add(
                                                ((((((&raw const sSpotlight_Gfx)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((k) as i32).wrapping_mul(32i32))
                                                        .wrapping_add(
                                                            ((i) as i32).wrapping_mul(4i32),
                                                        ))
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32),
                                            )) as u16),
                                    );
                                (((((100696064i32).wrapping_add(
                                    (((k) as i32).wrapping_add(1i32)).wrapping_mul(32i32),
                                ))
                                .wrapping_add(((i) as i32).wrapping_mul(4i32)))
                                .wrapping_add(2i32)) as usize
                                    as *mut u16)
                                    .write(
                                        (((((((((&raw const sSpotlight_Gfx)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((k) as i32).wrapping_mul(32i32))
                                                .wrapping_add(((i) as i32).wrapping_mul(4i32)))
                                            .wrapping_add(3i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            << 8)
                                            .wrapping_add(
                                                ((((((&raw const sSpotlight_Gfx)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (((((k) as i32).wrapping_mul(32i32))
                                                        .wrapping_add(
                                                            ((i) as i32).wrapping_mul(4i32),
                                                        ))
                                                    .wrapping_add(2i32))
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32),
                                            )) as u16),
                                    );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                k = (k).wrapping_add(1);
            }
        }
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_NPCFlyOut() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(26))
            .read(),
            120i16,
            0i16,
            1u8,
        );
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 4, 4, (0u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_NPCFlyOut));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as i16),
        );
        PlaySE(158u16);
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_NPCFlyOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut npcSprite: *mut u8 = core::ptr::null_mut();
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            140i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            72i16,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_add(4i32)
                & 255i32) as i16),
        );
        if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
            npcSprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize
                    * 68,
            );
            crate::c::bf_write((npcSprite).wrapping_add(62), 1, 1, (0u16) as i32);
            ((npcSprite).wrapping_add(32).cast::<i16>()).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((npcSprite).wrapping_add(34).cast::<i16>()).write(
                (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                .wrapping_sub(8i32)) as i16),
            );
            ((npcSprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((npcSprite).wrapping_add(38).cast::<i16>()).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= 128i32
        {
            FieldEffectStop(sprite, 30u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseFly() -> u8 {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_FlyOut), 254u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as i16),
        );
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_FlyOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sFlyOutFieldEffectFuncs)
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
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_FieldMovePose(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (!((ObjectEventIsMovementOverridden(objectEvent)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                .write(((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i16));
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
            SetPlayerAvatarStateMask(1u8);
            SetPlayerAvatarFieldMove();
            ObjectEventSetHeldMovement(objectEvent, 57u8);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_ShowMon(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            (((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            );
            FieldEffectStart(59u8);
        }
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_BirdLeaveBall(task: *mut u8) {
    unsafe {
        let mut task = task;
        if !((FieldEffectActiveListContains(6u8)) != 0) {
            let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                & 8i32)
                != 0
            {
                SetSurfBlob_BobState(((objectEvent).wrapping_add(26)).read(), 2u8);
                SetSurfBlob_DontSyncAnim(((objectEvent).wrapping_add(26)).read(), 0u8);
            }
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                .write(((CreateFlyBirdSprite()) as i16));
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_WaitBirdLeave(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (GetFlyBirdAnimCompleted(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
        )) != 0
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(16i16);
            SetPlayerAvatarTransitionFlags(1u16);
            ObjectEventSetHeldMovement(
                ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
                2u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_BirdSwoopDown(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32))
            && ((ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0)
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            PlaySE(158u16);
            StartFlyBirdSwoopDown(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_JumpOnBird(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 8i32
        {
            let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
            ObjectEventSetGraphicsId(objectEvent, GetPlayerAvatarGraphicsIdByStateId(3u8));
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                ),
                22u8,
            );
            crate::c::bf_write((objectEvent).wrapping_add(1), 4, 1, (1u32) as i32);
            ObjectEventSetHeldMovement(objectEvent, 72u8);
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                & 8i32)
                != 0
            {
                DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((objectEvent).wrapping_add(26)).read()) as i32) as isize * 68,
                ));
            }
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_FlyOffWithBird(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 10i32
        {
            let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
            ObjectEventClearHeldMovementIfActive(objectEvent);
            crate::c::bf_write((objectEvent).wrapping_add(1), 4, 1, (0u32) as i32);
            crate::c::bf_write((objectEvent).wrapping_add(2), 6, 1, (0u32) as i32);
            SetFlyBirdPlayerSpriteId(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
                ((objectEvent).wrapping_add(4)).read(),
            );
            CameraObjectFreeze();
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_WaitFlyOff(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (GetFlyBirdAnimCompleted(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
        )) != 0
        {
            WarpFadeOutScreen();
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FlyOutFieldEffect_End(task: *mut u8) {
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
            FieldEffectActiveListRemove(31u8);
            DestroyTask(FindTaskIdByFunc(Some(Task_FlyOut)));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateFlyBirdSprite() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        spriteId = CreateSprite(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(26))
            .read(),
            255i16,
            180i16,
            1u8,
        );
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        crate::c::bf_write((sprite).wrapping_add(5), 4, 4, (0u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_FlyBirdLeaveBall));
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn GetFlyBirdAnimCompleted(spriteId: u8) -> u8 {
    unsafe {
        let mut spriteId = spriteId;
        return ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn StartFlyBirdSwoopDown(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_FlyBirdSwoopDown));
        ((sprite).wrapping_add(32).cast::<i16>()).write(((crate::c::div_i32(240i32, 2i32)) as i16));
        ((sprite).wrapping_add(34).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        crate::c::memset(
            (((sprite).wrapping_add(46)).cast::<i16>()).cast::<u8>(),
            0i32,
            16u32,
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(64i16);
    }
}
pub(crate) unsafe extern "C" fn SetFlyBirdPlayerSpriteId(birdSpriteId: u8, playerSpriteId: u8) {
    unsafe {
        let mut birdSpriteId = birdSpriteId;
        let mut playerSpriteId = playerSpriteId;
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((birdSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((playerSpriteId) as i16));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlyBirdLeaveBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            == 0i32
        {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (3u32) as i32);
                ((sprite).wrapping_add(16).cast::<*mut *mut u8>()).write(
                    ((&raw const sAffineAnims_FlyBird)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>(),
                );
                InitSpriteAffineAnim(sprite);
                StartSpriteAffineAnim(sprite, 0u8);
                ((sprite).wrapping_add(32).cast::<i16>()).write(118i16);
                ((sprite).wrapping_add(34).cast::<i16>()).write((-48i16));
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(64i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(256i16);
            }
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                120i16,
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                120i16,
            ));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                < 2048i32
            {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(96i32)) as i16));
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 129i32
            {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p4).write(((__p4).read()).wrapping_add(1));
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
                FreeOamMatrix(
                    ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
                );
                CalcCenterToCornerVec(
                    sprite,
                    ((crate::c::bf_read((sprite).wrapping_add(1), 6, 2, false) as u32) as u8),
                    ((crate::c::bf_read((sprite).wrapping_add(3), 6, 2, false) as u32) as u8),
                    0u8,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlyBirdSwoopDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            140i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            72i16,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_add(4i32)
                & 255i32) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            != 64i32
        {
            let mut sprite1: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 68,
            );
            crate::c::bf_write((sprite1).wrapping_add(62), 1, 1, (0u16) as i32);
            ((sprite1).wrapping_add(32).cast::<i16>()).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite1).wrapping_add(34).cast::<i16>()).write(
                (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                .wrapping_sub(8i32)) as i16),
            );
            ((sprite1).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite1).wrapping_add(38).cast::<i16>()).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= 128i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlyBirdReturnToBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            == 0i32
        {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (3u32) as i32);
                ((sprite).wrapping_add(16).cast::<*mut *mut u8>()).write(
                    ((&raw const sAffineAnims_FlyBird)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>(),
                );
                InitSpriteAffineAnim(sprite);
                StartSpriteAffineAnim(sprite, 1u8);
                ((sprite).wrapping_add(32).cast::<i16>()).write(94i16);
                ((sprite).wrapping_add(34).cast::<i16>()).write((-32i16));
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(240i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(2048i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(128i16);
            }
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p4).write((((((__p4).read()) as i32) & 255i32) as i16));
            ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                32i16,
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                120i16,
            ));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 256i32
            {
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                < 256i32
            {
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p6).write((((((__p6).read()) as i32).wrapping_add(24i32)) as i16));
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                < 256i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(256i16);
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >= 60i32
            {
                let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p7).write(((__p7).read()).wrapping_add(1));
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
                FreeOamMatrix(
                    ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
                );
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartFlyBirdReturnToBall(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        StartFlyBirdSwoopDown(spriteId);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_FlyBirdReturnToBall));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_FlyIn() -> u8 {
    unsafe {
        CreateTask(Some(Task_FlyIn), 254u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_FlyIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw const sFlyInFieldEffectFuncs)
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
pub(crate) unsafe extern "C" fn FlyInFieldEffect_BirdSwoopDown(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (!((ObjectEventIsMovementOverridden(objectEvent)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0)
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(17i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                .write(((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i16));
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
            SetPlayerAvatarStateMask(1u8);
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                & 8i32)
                != 0
            {
                SetSurfBlob_BobState(((objectEvent).wrapping_add(26)).read(), 0u8);
            }
            ObjectEventSetGraphicsId(objectEvent, GetPlayerAvatarGraphicsIdByStateId(3u8));
            CameraObjectFreeze();
            ObjectEventTurn(objectEvent, 3u8);
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                ),
                22u8,
            );
            crate::c::bf_write((objectEvent).wrapping_add(1), 5, 1, (0u32) as i32);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                .write(((CreateFlyBirdSprite()) as i16));
            StartFlyBirdSwoopDown(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            );
            SetFlyBirdPlayerSpriteId(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
                ((objectEvent).wrapping_add(4)).read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_FlyInWithBird(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
            sprite = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68);
            SetFlyBirdPlayerSpriteId(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
                64u8,
            );
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            let __p5 = ((task).wrapping_add(8)).cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_JumpOffBird(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut sYPositions = crate::ffi::Align4([0u8; 36]);
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write((-2i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write((-4i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write((-5i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write((-6i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<i16>()
            .write((-7i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(10)
            .cast::<i16>()
            .write((-8i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<i16>()
            .write((-8i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(14)
            .cast::<i16>()
            .write((-8i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<i16>()
            .write((-7i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(18)
            .cast::<i16>()
            .write((-7i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<i16>()
            .write((-6i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(22)
            .cast::<i16>()
            .write((-5i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(24)
            .cast::<i16>()
            .write((-3i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(26)
            .cast::<i16>()
            .write((-2i16));
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(28)
            .cast::<i16>()
            .write(0i16);
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(30)
            .cast::<i16>()
            .write(2i16);
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(32)
            .cast::<i16>()
            .write(4i16);
        (&raw mut sYPositions)
            .cast::<u8>()
            .wrapping_add(34)
            .cast::<i16>()
            .write(8i16);
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((&raw mut sYPositions).cast::<i16>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    as isize,
            ))
            .read(),
        );
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= ((crate::c::div_u32(36u32, 2u32)) as i32)
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_FieldMovePose(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if (GetFlyBirdAnimCompleted(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
        )) != 0
        {
            objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
            sprite = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68);
            crate::c::bf_write((objectEvent).wrapping_add(1), 4, 1, (0u32) as i32);
            MoveObjectEventToMapCoords(
                objectEvent,
                (((objectEvent).wrapping_add(16)).cast::<i16>()).read(),
                (((objectEvent).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read(),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            SetPlayerAvatarFieldMove();
            ObjectEventSetHeldMovement(objectEvent, 57u8);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_BirdReturnToBall(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (ObjectEventClearHeldMovementIfFinished(
            ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
        )) != 0
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            StartFlyBirdReturnToBall(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_WaitBirdReturn(task: *mut u8) {
    unsafe {
        let mut task = task;
        if (GetFlyBirdAnimCompleted(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
        )) != 0
        {
            DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize
                    * 68,
            ));
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(16i16);
        }
    }
}
pub(crate) unsafe extern "C" fn FlyInFieldEffect_End(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut state: u8 = 0u8;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            objectEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
            state = 0u8;
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                & 8i32)
                != 0
            {
                state = 3u8;
                SetSurfBlob_BobState(((objectEvent).wrapping_add(26)).read(), 1u8);
            }
            ObjectEventSetGraphicsId(objectEvent, GetPlayerAvatarGraphicsIdByStateId(state));
            ObjectEventTurn(objectEvent, 1u8);
            ((&raw mut gPlayerAvatar).cast::<u8>()).write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            );
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
            FieldEffectActiveListRemove(32u8);
            DestroyTask(FindTaskIdByFunc(Some(Task_FlyIn)));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_DestroyDeoxysRock() -> u8 {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut objectEventId: u8 = 0u8;
        if !((TryGetObjectEventIdByLocalIdAndMap(
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(1))
            .read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(2))
            .read()) as u8),
            &raw mut objectEventId,
        )) != 0)
        {
            taskId = CreateTask(Some(Task_DestroyDeoxysRock), 80u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((objectEventId) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(
                (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .read()) as i16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(
                ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write(
                ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
        } else {
            FieldEffectActiveListRemove(65u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_DeoxysRockCameraShake(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).wrapping_offset(7)).read()) != 0 {
            if (({
                let __p1 = (data).wrapping_offset(6);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 20i32
            {
                ((data).wrapping_offset(6)).write(0i16);
                if ((((data).wrapping_offset(5)).read()) as i32) != 0i32 {
                    let __p3 = (data).wrapping_offset(5);
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                }
            }
        } else {
            ((data).wrapping_offset(5)).write(4i16);
        }
        if (({
            let __t4 = ((data).read()).wrapping_add(1);
            (data).write(__t4);
            __t4
        }) as i32)
            > 1i32
        {
            (data).write(0i16);
            if ((({
                let __p5 = (data).wrapping_offset(1);
                let __t6 = ((__p5).read()).wrapping_add(1);
                (__p5).write(__t6);
                __t6
            }) as i32)
                & 1i32)
                != 0
            {
                SetCameraPanning(
                    0i16,
                    ((((((data).wrapping_offset(5)).read()) as i32).wrapping_neg()) as i16),
                );
            } else {
                SetCameraPanning(0i16, ((data).wrapping_offset(5)).read());
            }
        }
        UpdateCameraPanning();
        if ((((data).wrapping_offset(5)).read()) as i32) == 0i32 {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn StartEndingDeoxysRockCameraShake(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn Task_DestroyDeoxysRock(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        InstallCameraPanAheadCallback();
        SetCameraPanningCallback(None);
        (((((&raw const sDestroyDeoxysRockEffectFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut i16, u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut i16, u8)>>())
        .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize))
        .read())
        .unwrap_unchecked()(data, taskId);
    }
}
pub(crate) unsafe extern "C" fn DestroyDeoxysRockEffect_CameraShake(data: *mut i16, taskId: u8) {
    unsafe {
        let mut data = data;
        let mut taskId = taskId;
        let mut newTaskId: u8 = CreateTask(Some(Task_DeoxysRockCameraShake), 90u8);
        PlaySE(88u16);
        ((data).wrapping_offset(5)).write(((newTaskId) as i16));
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn DestroyDeoxysRockEffect_RockFragments(data: *mut i16, taskId: u8) {
    unsafe {
        let mut data = data;
        let mut taskId = taskId;
        if (({
            let __p1 = (data).wrapping_offset(3);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 120i32
        {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 36))
                .wrapping_add(4))
                .read()) as i32) as isize
                    * 68,
            );
            crate::c::bf_write(
                (((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 36))
                .wrapping_add(1),
                5,
                1,
                (1u32) as i32,
            );
            BlendPalettes(65535u32, 16u8, 32767u16);
            BeginNormalPaletteFade(65535u32, 0i8, 16u8, 0u8, 32767u16);
            CreateDeoxysRockFragments(sprite);
            PlaySE(87u16);
            StartEndingDeoxysRockCameraShake(((((data).wrapping_offset(5)).read()) as u8));
            ((data).wrapping_offset(3)).write(0i16);
            let __p3 = (data).wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyDeoxysRockEffect_WaitAndEnd(data: *mut i16, taskId: u8) {
    unsafe {
        let mut data = data;
        let mut taskId = taskId;
        if (!((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0))
            && (!((FuncIsActiveTask(Some(Task_DeoxysRockCameraShake))) != 0))
        {
            InstallCameraPanAheadCallback();
            RemoveObjectEventByLocalIdAndMap(
                ((((data).wrapping_offset(6)).read()) as u8),
                ((((data).wrapping_offset(7)).read()) as u8),
                ((((data).wrapping_offset(8)).read()) as u8),
            );
            FieldEffectActiveListRemove(65u8);
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateDeoxysRockFragments(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        let mut xPos: i32 =
            ((((((&raw mut gTotalCameraPixelOffsetX).cast::<u16>()).read()) as i16) as i32)
                .wrapping_add(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32));
        let mut yPos: i32 =
            (((((((&raw mut gTotalCameraPixelOffsetY).cast::<u16>()).read()) as i16) as i32)
                .wrapping_add(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
            .wrapping_sub(4i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw const sSpriteTemplate_DeoxysRockFragment)
                            .cast::<u8>()
                            .cast_mut(),
                        ((xPos) as i16),
                        ((yPos) as i16),
                        0u8,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((i) as u8),
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((i) as i16));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            4,
                            4,
                            (crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16)
                                as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DeoxysRockFragment(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(16i32)) as i16));
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(12i32)) as i16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p4 = (sprite).wrapping_add(32).cast::<i16>();
                (__p4).write((((((__p4).read()) as i32).wrapping_add(16i32)) as i16));
                let __p5 = (sprite).wrapping_add(34).cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_sub(12i32)) as i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p6 = (sprite).wrapping_add(32).cast::<i16>();
                (__p6).write((((((__p6).read()) as i32).wrapping_sub(16i32)) as i16));
                let __p7 = (sprite).wrapping_add(34).cast::<i16>();
                (__p7).write((((((__p7).read()) as i32).wrapping_add(12i32)) as i16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p8 = (sprite).wrapping_add(32).cast::<i16>();
                (__p8).write((((((__p8).read()) as i32).wrapping_add(16i32)) as i16));
                let __p9 = (sprite).wrapping_add(34).cast::<i16>();
                (__p9).write((((((__p9).read()) as i32).wrapping_add(12i32)) as i16));
                break 'l1;
            }
        }
        if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-4i32))
            || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 244i32))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < (-4i32)))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 164i32)
        {
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_MoveDeoxysRock(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut objectEventId: u8 = 0u8;
        if !((TryGetObjectEventIdByLocalIdAndMap(
            (((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>()).read())
                as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(1))
            .read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                .wrapping_offset(2))
            .read()) as u8),
            &raw mut objectEventId,
        )) != 0)
        {
            let mut object: *mut u8 = core::ptr::null_mut();
            let mut xPos: i32 = 0i32;
            let mut yPos: i32 = 0i32;
            let mut taskId: u8 = 0u8;
            object = ((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36);
            xPos = (((((object).wrapping_add(16)).cast::<i16>()).read()) as i32).wrapping_sub(7i32);
            yPos = (((((object).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                .wrapping_sub(7i32);
            xPos = ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>())
                .cast::<i32>())
            .wrapping_offset(3))
            .read())
            .wrapping_sub(xPos))
            .wrapping_mul(16i32);
            yPos = ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>())
                .cast::<i32>())
            .wrapping_offset(4))
            .read())
            .wrapping_sub(yPos))
            .wrapping_mul(16i32);
            ShiftObjectEventCoords(
                object,
                (((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read())
                .wrapping_add(7i32)) as i16),
                (((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(4))
                .read())
                .wrapping_add(7i32)) as i16),
            );
            taskId = CreateTask(Some(Task_MoveDeoxysRock), 80u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((((object).wrapping_add(4)).read()) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((object).wrapping_add(4)).read()) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(xPos)) as i16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((object).wrapping_add(4)).read()) as i32) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(yPos)) as i16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write(
                ((((((&raw mut gFieldEffectArguments).cast::<u8>().cast::<i32>()).cast::<i32>())
                    .wrapping_offset(5))
                .read()) as i16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .write(((objectEventId) as i16));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_MoveDeoxysRock(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68);
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((data).wrapping_offset(4)).write(
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((data).wrapping_offset(5)).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((data).wrapping_offset(6)).write(
                    ((if ((((data).wrapping_offset(8)).read()) as i32) != 0i32 {
                        crate::c::div_i32(
                            (((((data).wrapping_offset(2)).read()) as i32).wrapping_mul(16i32))
                                .wrapping_sub(((((data).wrapping_offset(4)).read()) as i32)),
                            ((((data).wrapping_offset(8)).read()) as i32),
                        )
                    } else {
                        0i32
                    }) as i16),
                );
                ((data).wrapping_offset(7)).write(
                    ((if ((((data).wrapping_offset(8)).read()) as i32) != 0i32 {
                        crate::c::div_i32(
                            (((((data).wrapping_offset(3)).read()) as i32).wrapping_mul(16i32))
                                .wrapping_sub(((((data).wrapping_offset(5)).read()) as i32)),
                            ((((data).wrapping_offset(8)).read()) as i32),
                        )
                    } else {
                        0i32
                    }) as i16),
                );
                (data).write(((data).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if ((((data).wrapping_offset(8)).read()) as i32) != 0i32 {
                    let __p2 = (data).wrapping_offset(8);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                    let __p3 = (data).wrapping_offset(4);
                    (__p3).write(
                        (((((__p3).read()) as i32)
                            .wrapping_add(((((data).wrapping_offset(6)).read()) as i32)))
                            as i16),
                    );
                    let __p4 = (data).wrapping_offset(5);
                    (__p4).write(
                        (((((__p4).read()) as i32)
                            .wrapping_add(((((data).wrapping_offset(7)).read()) as i32)))
                            as i16),
                    );
                    ((sprite).wrapping_add(32).cast::<i16>())
                        .write(((((((data).wrapping_offset(4)).read()) as i32) >> 4) as i16));
                    ((sprite).wrapping_add(34).cast::<i16>())
                        .write(((((((data).wrapping_offset(5)).read()) as i32) >> 4) as i16));
                } else {
                    let mut object: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
                        .wrapping_offset(
                            ((((data).wrapping_offset(9)).read()) as i32) as isize * 36,
                        );
                    ((sprite).wrapping_add(32).cast::<i16>())
                        .write(((data).wrapping_offset(2)).read());
                    ((sprite).wrapping_add(34).cast::<i16>())
                        .write(((data).wrapping_offset(3)).read());
                    ShiftStillObjectEventCoords(object);
                    crate::c::bf_write((object).wrapping_add(0), 3, 1, (1u32) as i32);
                    FieldEffectActiveListRemove(66u8);
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
