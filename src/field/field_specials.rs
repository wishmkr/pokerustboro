//! Translated from `src/field_specials.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMauvilleGymSwitchCoords sSlidingDoorNextFrameDelay sPetalburgGymSlidingDoorMetatiles sWindowTemplate_ElevatorFloor sDeptStoreFloorNames sElevatorWindowTiles_Ascending sElevatorWindowTiles_Descending sScrollableMultichoiceOptions sBattleFrontier_TutorMoves1 sBattleFrontier_TutorMoves2 sDeoxysRockPalettes sDeoxysRockCoords sAbnormalWeatherMapNumbers.5 sAbnormalWeatherMapNumbers.6 sBattleFrontierTutor_WindowTemplate.10 sBattleFrontier_TutorMoveDescriptions1.8 sBattleFrontier_TutorMoveDescriptions2.9 sBattlePoints_WindowTemplate.20 sBattleTowerStreakThresholds.26 sCounterIncrements.2 sElevatorLightCycles.30 sElevatorTripLength.31 sFanClubMemberIds.0 sFanClubMemberIds.1 sFrontierChallenges.21 sFrontierExchangeCorner_Decor1.17 sFrontierExchangeCorner_Decor1Descriptions.18 sFrontierExchangeCorner_Decor2.15 sFrontierExchangeCorner_Decor2Descriptions.16 sFrontierExchangeCorner_HoldItems.11 sFrontierExchangeCorner_HoldItemsDescriptions.12 sFrontierExchangeCorner_ItemIconWindowTemplate.19 sFrontierExchangeCorner_Vitamins.13 sFrontierExchangeCorner_VitaminsDescriptions.14 sFrontierGamblerGoMessages.22 sFrontierGamblerLookingMessages.23 sFrontierManiacMessages.27 sFrontierManiacStreakThresholds.28 sNatureGirlMessages.24 sPokeMarts.4 sPokemonCenters.29 sPokemonCenters.3 sScrollableMultichoice_ScrollArrowsTemplate.25 sSlotMachineIds.32 sSlotMachineRandomSeeds.34 sSlotMachineServiceDayIds.33 sStoneMaxStepCounts.7
#[allow(unused_imports)]
use crate::data::field_specials::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBikeCyclingChallenge: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBikeCollisions: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBikeCyclingTimer: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlidingDoorNextFrameCounter: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlidingDoorFrame: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTutorMoveAndElevatorWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLilycoveDeptStore_NeverRead: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLilycoveDeptStore_DefaultFloorChoice: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScrollableMultichoice_ListMenuItem: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScrollableMultichoice_ScrollOffset: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierExchangeCorner_NeverRead: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScrollableMultichoice_ItemSpriteId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlePointsWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierExchangeCorner_ItemIconWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPCBoxToSendMon: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleTowerMultiBattleTypeFlags: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gScrollableMultichoice_ListMenuTemplate: crate::ffi::Align4<[u8; 24]> =
    crate::ffi::Align4([0; 24]);

unsafe extern "C" {
    static mut gBattleOutcome: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gFieldEffectArguments: u8;
    static mut gLastUsedWarp: u8;
    static mut gLinkPlayers: u8;
    static mut gLocalTime: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gMoveNames: u8;
    static mut gObjectEventPal_Brendan: u8;
    static mut gObjectEventPal_May: u8;
    static mut gObjectEventPal_RubySapphireBrendan: u8;
    static mut gObjectEventPal_RubySapphireMay: u8;
    static mut gObjectEvents: u8;
    static mut gPlayerParty: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_0x8007: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTVStringVarPtrs: u8;
    static mut gTasks: u8;
    static mut gText_1MinutePlus: u8;
    static mut gText_99TimesPlus: u8;
    static mut gText_BP: u8;
    static mut gText_BigGirl: u8;
    static mut gText_BigGuy: u8;
    static mut gText_Brawly: u8;
    static mut gText_Daughter: u8;
    static mut gText_ElevatorNowOn: u8;
    static mut gText_Glacia: u8;
    static mut gText_Phoebe: u8;
    static mut gText_SelectorArrow: u8;
    static mut gText_Son: u8;
    static mut gText_SpaceSeconds: u8;
    static mut gText_SpaceTimes: u8;
    static mut gText_Steven: u8;
    static mut gText_Wallace: u8;
    static mut gText_Winona: u8;
    static mut gText_YourPartnerHasRetired: u8;
    static mut gTutorMoves: u8;
    static mut gWirelessCommType: u8;
    fn AddDecorationIconObject(a0: u8, a1: i16, a2: i16, a3: u8, a4: u16, a5: u16) -> u8;
    fn AddItemIconSprite(a0: u16, a1: u16, a2: u16) -> u8;
    fn AddScrollIndicatorArrowPair(a0: *mut u8, a1: *mut u16) -> u8;
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
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn BitmaskAllOtherLinkPlayers() -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CB2_ShowDiploma();
    fn CB2_ViewWallClock();
    fn CalculatePlayerPartyCount() -> u8;
    fn CameraObjectSetFollowedSpriteId(a0: u8);
    fn CheckFreePokemonStorageSpace() -> u8;
    fn CheckPartyPokerus(a0: *mut u8, a1: u8) -> u8;
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalString(a0: u8, a1: i32);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn ConvertPixelWidthToTileWidth(a0: i32) -> i32;
    fn CopyMonFavoritePokeblockName(a0: u8, a1: *mut u8) -> u8;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountDigits(a0: i32) -> u32;
    fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWindowTemplate(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u16,
    ) -> crate::c::Rec4<8>;
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DisplayTextAndGetWidth(a0: *mut u8, a1: i32) -> i32;
    fn DoRayquazaScene(a0: u8, a1: u8, a2: Option<unsafe extern "C" fn()>);
    fn DrawWholeMapView();
    fn FieldEffectActiveListContains(a0: u8) -> u8;
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldInitRegionMap(a0: Option<unsafe extern "C" fn()>);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetBoxMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut u8;
    fn GetEreaderTrainerName(a0: *mut u8);
    fn GetGameStat(a0: u8) -> u32;
    fn GetLastUsedWarpMapType() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetMapName(a0: *mut u8, a1: u16, a2: u16) -> *mut u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonEVCount(a0: *mut u8) -> u16;
    fn GetMultiplayerId() -> u8;
    fn GetNature(a0: *mut u8) -> u8;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetPlayerAvatarSpriteId() -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetRematchIdxByTrainerIdx(a0: i32) -> i32;
    fn GetRibbonCount(a0: *mut u8) -> u8;
    fn GetStarterPokemon(a0: u16) -> u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetVarPointer(a0: u16) -> *mut u16;
    fn IncrementGameStat(a0: u8);
    fn InstallCameraPanAheadCallback();
    fn IsLinkTaskFinished() -> u8;
    fn IsMapTypeOutdoors(a0: u8) -> u8;
    fn IsPokeNewsActive(a0: u8) -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn ItemIdToBattleMoveId(a0: u16) -> u16;
    fn ListMenuGetCurrentItemArrayId(a0: u8, a1: *mut u16);
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn MysteryGift_GetCardStat(a0: u32) -> u16;
    fn Overworld_SetSavedMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveObjectEventByLocalIdAndMap(a0: u8, a1: u8, a2: u8);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetBlockReceivedFlag(a0: u8);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SetCameraPanning(a0: i16, a1: i16);
    fn SetCameraPanningCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetCloseLinkCallback();
    fn SetCurrentAndNextWeather(a0: u8);
    fn SetLastHealLocationWarp(a0: u8);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetObjEventTemplateCoords(a0: u8, a1: i16, a2: i16);
    fn SetSavedWeather(a0: u32);
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SetWarpDestination(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn ShowFieldAutoScrollMessage(a0: *mut u8) -> u8;
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn SpawnSpecialObjectEventParameterized(a0: u8, a1: u8, a2: u8, a3: i16, a4: i16, a5: u8)
    -> u8;
    fn StorageGetCurrentBox() -> u8;
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8;
    fn Task_ReconnectWithLinkPlayers(a0: u8);
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn TryPutSpotTheCutiesOnAir(a0: *mut u8, a1: u8);
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Special_ShowDiploma() {
    unsafe {
        SetMainCallback2(Some(CB2_ShowDiploma));
        LockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Special_ViewWallClock() {
    unsafe {
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_ReturnToField));
        SetMainCallback2(Some(CB2_ViewWallClock));
        LockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetCyclingRoadChallengeData() {
    unsafe {
        ((&raw mut gBikeCyclingChallenge).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gBikeCollisions).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sBikeCyclingTimer).cast::<u8>().cast::<u32>()).write(0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Special_BeginCyclingRoadChallenge() {
    unsafe {
        ((&raw mut gBikeCyclingChallenge).cast::<u8>().cast::<u8>()).write(1u8);
        ((&raw mut gBikeCollisions).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sBikeCyclingTimer).cast::<u8>().cast::<u32>()).write(
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(32)
                .cast::<u32>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerAvatarBike() -> u16 {
    unsafe {
        if (TestPlayerAvatarFlags(4u8)) != 0 {
            return 1u16;
        }
        if (TestPlayerAvatarFlags(2u8)) != 0 {
            return 2u16;
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn DetermineCyclingRoadResults(numFrames: u32, numBikeCollisions: u8) {
    unsafe {
        let mut numFrames = numFrames;
        let mut numBikeCollisions = numBikeCollisions;
        let mut result: u8 = 0u8;
        if ((numBikeCollisions) as i32) < 100i32 {
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((numBikeCollisions) as i32),
                0i32,
                2u8,
            );
            StringAppend(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_SpaceTimes).cast::<u8>(),
            );
        } else {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_99TimesPlus).cast::<u8>(),
            );
        }
        if numFrames < 3600u32 {
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                ((crate::c::div_u32(numFrames, 60u32)) as i32),
                1i32,
                2u8,
            );
            (((&raw mut gStringVar2).cast::<u8>()).wrapping_offset(2)).write(173u8);
            ConvertIntToDecimalStringN(
                ((&raw mut gStringVar2).cast::<u8>()).wrapping_offset(3),
                ((crate::c::div_u32(
                    (crate::c::rem_u32(numFrames, 60u32)).wrapping_mul(100u32),
                    60u32,
                )) as i32),
                2i32,
                2u8,
            );
            StringAppend(
                (&raw mut gStringVar2).cast::<u8>(),
                (&raw mut gText_SpaceSeconds).cast::<u8>(),
            );
        } else {
            StringCopy(
                (&raw mut gStringVar2).cast::<u8>(),
                (&raw mut gText_1MinutePlus).cast::<u8>(),
            );
        }
        result = 0u8;
        if ((numBikeCollisions) as i32) == 0i32 {
            result = 5u8;
        } else {
            if ((numBikeCollisions) as i32) < 4i32 {
                result = 4u8;
            } else {
                if ((numBikeCollisions) as i32) < 10i32 {
                    result = 3u8;
                } else {
                    if ((numBikeCollisions) as i32) < 20i32 {
                        result = 2u8;
                    } else {
                        if ((numBikeCollisions) as i32) < 100i32 {
                            result = 1u8;
                        }
                    }
                }
            }
        }
        if crate::c::div_u32(numFrames, 60u32) <= 10u32 {
            result = ((((result) as i32).wrapping_add(5i32)) as u8);
        } else {
            if crate::c::div_u32(numFrames, 60u32) <= 15u32 {
                result = ((((result) as i32).wrapping_add(4i32)) as u8);
            } else {
                if crate::c::div_u32(numFrames, 60u32) <= 20u32 {
                    result = ((((result) as i32).wrapping_add(3i32)) as u8);
                } else {
                    if crate::c::div_u32(numFrames, 60u32) <= 40u32 {
                        result = ((((result) as i32).wrapping_add(2i32)) as u8);
                    } else {
                        if crate::c::div_u32(numFrames, 60u32) < 60u32 {
                            result = ((((result) as i32).wrapping_add(1i32)) as u8);
                        }
                    }
                }
            }
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((result) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FinishCyclingRoadChallenge() {
    unsafe {
        let mut numFrames: u32 = ((((&raw mut gMain).cast::<u8>())
            .wrapping_add(32)
            .cast::<u32>())
        .read())
        .wrapping_sub(((&raw mut sBikeCyclingTimer).cast::<u8>().cast::<u32>()).read());
        DetermineCyclingRoadResults(
            numFrames,
            ((&raw mut gBikeCollisions).cast::<u8>().cast::<u8>()).read(),
        );
        RecordCyclingRoadResults(
            numFrames,
            ((&raw mut gBikeCollisions).cast::<u8>().cast::<u8>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn RecordCyclingRoadResults(numFrames: u32, numBikeCollisions: u8) {
    unsafe {
        let mut numFrames = numFrames;
        let mut numBikeCollisions = numBikeCollisions;
        let mut low: u16 = VarGet(16424u16);
        let mut high: u16 = VarGet(16425u16);
        let mut framesRecord: u32 = ((((low) as i32).wrapping_add((((high) as i32) << 16))) as u32);
        if (framesRecord > numFrames) || (framesRecord == 0u32) {
            VarSet(16424u16, ((numFrames) as u16));
            VarSet(16425u16, ((numFrames >> 16) as u16));
            VarSet(16423u16, ((numBikeCollisions) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedCyclingRoadResults() -> u16 {
    unsafe {
        let mut low: u16 = VarGet(16424u16);
        let mut high: u16 = VarGet(16425u16);
        let mut framesRecord: u32 = ((((low) as i32).wrapping_add((((high) as i32) << 16))) as u32);
        if framesRecord == 0u32 {
            return 0u16;
        }
        DetermineCyclingRoadResults(framesRecord, ((VarGet(16423u16)) as u8));
        return 1u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateCyclingRoadState() {
    unsafe {
        if ((((((&raw mut gLastUsedWarp).cast::<u8>())
            .wrapping_add(1)
            .cast::<i8>())
        .read()) as i32)
            == 12i32)
            && ((((((&raw mut gLastUsedWarp).cast::<u8>()).cast::<i8>()).read()) as i32) == 29i32)
        {
            return;
        }
        if (((VarGet(16553u16)) as i32) == 2i32) || (((VarGet(16553u16)) as i32) == 3i32) {
            VarSet(16553u16, 0u16);
            Overworld_SetSavedMusic(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSSTidalFlag() {
    unsafe {
        FlagSet(2189u16);
        (GetVarPointer(16458u16)).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSSTidalFlag() {
    unsafe {
        FlagClear(2189u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountSSTidalStep(delta: u16) -> u32 {
    unsafe {
        let mut delta = delta;
        if (!((FlagGet(2189u16)) != 0))
            || ((({
                let __p1 = GetVarPointer(16458u16);
                let __v2 = (((((__p1).read()) as i32).wrapping_add(((delta) as i32))) as u16);
                (__p1).write(__v2);
                __v2
            }) as i32)
                < 205i32)
        {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSSTidalLocation(
    mapGroup: *mut i8,
    mapNum: *mut i8,
    x: *mut i16,
    y: *mut i16,
) -> u8 {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut x = x;
        let mut y = y;
        let mut varCruiseStepCount: *mut u16 = GetVarPointer(16458u16);
        'l1: {
            let __sw1 = (((GetVarPointer(16564u16)).read()) as i32);
            if __sw1 == 1i32 || __sw1 == 8i32 {
                return 1u8;
            }
            if __sw1 == 3i32 || __sw1 == 9i32 {
                return 4u8;
            }
            if __sw1 == 4i32 || __sw1 == 5i32 {
                return 2u8;
            }
            if __sw1 == 6i32 || __sw1 == 10i32 {
                return 3u8;
            }
            if __sw1 == 2i32 {
                if (((varCruiseStepCount).read()) as i32) < 60i32 {
                    (mapNum).write(49i8);
                    (x).write(
                        (((((varCruiseStepCount).read()) as i32).wrapping_add(19i32)) as i16),
                    );
                } else {
                    if (((varCruiseStepCount).read()) as i32) < 140i32 {
                        (mapNum).write(48i8);
                        (x).write(
                            (((((varCruiseStepCount).read()) as i32).wrapping_sub(60i32)) as i16),
                        );
                    } else {
                        (mapNum).write(47i8);
                        (x).write(
                            (((((varCruiseStepCount).read()) as i32).wrapping_sub(140i32)) as i16),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (((varCruiseStepCount).read()) as i32) < 66i32 {
                    (mapNum).write(47i8);
                    (x).write(
                        (((65i32).wrapping_sub((((varCruiseStepCount).read()) as i32))) as i16),
                    );
                } else {
                    if (((varCruiseStepCount).read()) as i32) < 146i32 {
                        (mapNum).write(48i8);
                        (x).write(
                            (((145i32).wrapping_sub((((varCruiseStepCount).read()) as i32)))
                                as i16),
                        );
                    } else {
                        (mapNum).write(49i8);
                        (x).write(
                            (((224i32).wrapping_sub((((varCruiseStepCount).read()) as i32)))
                                as i16),
                        );
                    }
                }
                break 'l1;
            }
        }
        (mapGroup).write(0i8);
        (y).write(20i16);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoWallyCall() -> u32 {
    unsafe {
        if (FlagGet(136u16)) != 0 {
            'l1: {
                let __sw1 =
                    (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32;
                if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32 {
                    if (({
                        let __p2 = GetVarPointer(16626u16);
                        let __t3 = ((__p2).read()).wrapping_add(1);
                        (__p2).write(__t3);
                        __t3
                    }) as i32)
                        < 250i32
                    {
                        return 0u32;
                    }
                    break 'l1;
                }
                if !__matched {
                    return 0u32;
                }
            }
        } else {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoScottFortreeCall() -> u32 {
    unsafe {
        if (FlagGet(138u16)) != 0 {
            'l1: {
                let __sw1 =
                    (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32;
                if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32 {
                    if (({
                        let __p2 = GetVarPointer(16627u16);
                        let __t3 = ((__p2).read()).wrapping_add(1);
                        (__p2).write(__t3);
                        __t3
                    }) as i32)
                        < 10i32
                    {
                        return 0u32;
                    }
                    break 'l1;
                }
                if !__matched {
                    return 0u32;
                }
            }
        } else {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoScottBattleFrontierCall() -> u32 {
    unsafe {
        if (FlagGet(114u16)) != 0 {
            'l1: {
                let __sw1 =
                    (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32;
                if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32 {
                    if (({
                        let __p2 = GetVarPointer(16629u16);
                        let __t3 = ((__p2).read()).wrapping_add(1);
                        (__p2).write(__t3);
                        __t3
                    }) as i32)
                        < 10i32
                    {
                        return 0u32;
                    }
                    break 'l1;
                }
                if !__matched {
                    return 0u32;
                }
            }
        } else {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoRoxanneCall() -> u32 {
    unsafe {
        if (FlagGet(128u16)) != 0 {
            'l1: {
                let __sw1 =
                    (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32;
                if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32 {
                    if (({
                        let __p2 = GetVarPointer(16628u16);
                        let __t3 = ((__p2).read()).wrapping_add(1);
                        (__p2).write(__t3);
                        __t3
                    }) as i32)
                        < 250i32
                    {
                        return 0u32;
                    }
                    break 'l1;
                }
                if !__matched {
                    return 0u32;
                }
            }
        } else {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoRivalRayquazaCall() -> u32 {
    unsafe {
        if (FlagGet(117u16)) != 0 {
            'l1: {
                let __sw1 =
                    (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32;
                if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 6i32 {
                    if (({
                        let __p2 = GetVarPointer(16630u16);
                        let __t3 = ((__p2).read()).wrapping_add(1);
                        (__p2).write(__t3);
                        __t3
                    }) as i32)
                        < 250i32
                    {
                        return 0u32;
                    }
                    break 'l1;
                }
                if !__matched {
                    return 0u32;
                }
            }
        } else {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPartnerNames() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut myLinkPlayerNumber: u8 = GetMultiplayerId();
        let mut nLinkPlayers: u8 = GetLinkPlayerCount();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((nLinkPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((myLinkPlayerNumber) as i32) != ((i) as i32) {
                        StringCopy(
                            ((((&raw mut gTVStringVarPtrs).cast::<*mut u8>()).cast::<*mut u8>())
                                .wrapping_offset(((j) as i32) as isize))
                            .read(),
                            ((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(8))
                            .cast::<u8>(),
                        );
                        j = (j).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return nLinkPlayers;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpawnLinkPartnerObjectEvent() {
    unsafe {
        let mut j: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut movementTypes = crate::ffi::Align4([0u8; 4]);
        (&raw mut movementTypes)
            .cast::<u8>()
            .wrapping_add(0)
            .write(7u8);
        (&raw mut movementTypes)
            .cast::<u8>()
            .wrapping_add(1)
            .write(9u8);
        (&raw mut movementTypes)
            .cast::<u8>()
            .wrapping_add(2)
            .write(8u8);
        (&raw mut movementTypes)
            .cast::<u8>()
            .wrapping_add(3)
            .write(10u8);
        let mut coordOffsets = crate::ffi::Align4([0u8; 8]);
        (&raw mut coordOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<i8>()
            .write(0i8);
        (&raw mut coordOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(1)
            .cast::<i8>()
            .write(1i8);
        (&raw mut coordOffsets)
            .cast::<u8>()
            .wrapping_add(2)
            .wrapping_add(0)
            .cast::<i8>()
            .write(1i8);
        (&raw mut coordOffsets)
            .cast::<u8>()
            .wrapping_add(2)
            .wrapping_add(1)
            .cast::<i8>()
            .write(0i8);
        (&raw mut coordOffsets)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(0)
            .cast::<i8>()
            .write(0i8);
        (&raw mut coordOffsets)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(1)
            .cast::<i8>()
            .write((-1i8));
        (&raw mut coordOffsets)
            .cast::<u8>()
            .wrapping_add(6)
            .wrapping_add(0)
            .cast::<i8>()
            .write((-1i8));
        (&raw mut coordOffsets)
            .cast::<u8>()
            .wrapping_add(6)
            .wrapping_add(1)
            .cast::<i8>()
            .write(0i8);
        let mut myLinkPlayerNumber: u8 = 0u8;
        let mut playerFacingDirection: u8 = 0u8;
        let mut linkSpriteId: u8 = 0u8;
        let mut i: u8 = 0u8;
        myLinkPlayerNumber = GetMultiplayerId();
        playerFacingDirection = GetPlayerFacingDirection();
        'l1: {
            let __sw1 = ((playerFacingDirection) as i32);
            if __sw1 == 3i32 {
                j = 2u8;
                x = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
                    .read()) as i32)
                    .wrapping_sub(1i32)) as i16);
                y = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read();
                break 'l1;
            }
            if __sw1 == 2i32 {
                j = 1u8;
                x = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read();
                y = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(1i32)) as i16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                x = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
                    .read()) as i32)
                    .wrapping_add(1i32)) as i16);
                y = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read();
                break 'l1;
            }
            if __sw1 == 1i32 {
                j = 3u8;
                x = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read();
                y = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(1i32)) as i16);
            }
        }
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as i32) < ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32))
                {
                    break 'l2;
                }
                'l3: {
                    if ((myLinkPlayerNumber) as i32) != ((i) as i32) {
                        'l4: {
                            let __sw2 = (((((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 28))
                            .cast::<u16>())
                            .read()) as u8) as i32);
                            let __matched = __sw2 == 2i32 || __sw2 == 1i32 || __sw2 == 3i32;
                            if __sw2 == 2i32 || __sw2 == 1i32 {
                                if ((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 28))
                                .wrapping_add(19))
                                .read()) as i32)
                                    == 0i32
                                {
                                    linkSpriteId = 235u8;
                                } else {
                                    linkSpriteId = 236u8;
                                }
                                break 'l4;
                            }
                            if __sw2 == 3i32 {
                                if ((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 28))
                                .wrapping_add(19))
                                .read()) as i32)
                                    == 0i32
                                {
                                    linkSpriteId = 100u8;
                                } else {
                                    linkSpriteId = 105u8;
                                }
                                break 'l4;
                            }
                            if !__matched {
                                if ((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 28))
                                .wrapping_add(19))
                                .read()) as i32)
                                    == 0i32
                                {
                                    linkSpriteId = 100u8;
                                } else {
                                    linkSpriteId = 105u8;
                                }
                                break 'l4;
                            }
                        }
                        SpawnSpecialObjectEventParameterized(
                            linkSpriteId,
                            (((&raw mut movementTypes).cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize))
                            .read(),
                            (((240i32).wrapping_sub(((i) as i32))) as u8),
                            (((((((((&raw mut coordOffsets).cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 2))
                            .cast::<i8>())
                            .read()) as i32)
                                .wrapping_add(((x) as i32)))
                            .wrapping_add(7i32)) as i16),
                            ((((((((((&raw mut coordOffsets).cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 2))
                            .cast::<i8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_add(((y) as i32)))
                            .wrapping_add(7i32)) as i16),
                            0u8,
                        );
                        LoadLinkPartnerObjectEventSpritePalette(
                            linkSpriteId,
                            (((240i32).wrapping_sub(((i) as i32))) as u8),
                            i,
                        );
                        j = (j).wrapping_add(1);
                        if ((j) as i32) == 4i32 {
                            j = 0u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadLinkPartnerObjectEventSpritePalette(
    graphicsId: u8,
    localEventId: u8,
    paletteNum: u8,
) {
    unsafe {
        let mut graphicsId = graphicsId;
        let mut localEventId = localEventId;
        let mut paletteNum = paletteNum;
        let mut adjustedPaletteNum: u8 = 0u8;
        adjustedPaletteNum = ((((paletteNum) as i32).wrapping_add(6i32)) as u8);
        if (((((graphicsId) as i32) == 235i32) || (((graphicsId) as i32) == 236i32))
            || (((graphicsId) as i32) == 100i32))
            || (((graphicsId) as i32) == 105i32)
        {
            let mut obj: u8 = GetObjectEventIdByLocalIdAndMap(
                localEventId,
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as u8),
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as u8),
            );
            if ((obj) as i32) != 16i32 {
                let mut spriteId: u8 = ((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((obj) as i32) as isize * 36))
                .wrapping_add(4))
                .read();
                let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68);
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    4,
                    4,
                    ((adjustedPaletteNum) as u16) as i32,
                );
                'l1: {
                    let __sw1 = ((graphicsId) as i32);
                    if __sw1 == 235i32 {
                        LoadPalette(
                            (((&raw mut gObjectEventPal_RubySapphireBrendan).cast::<u16>())
                                .cast::<u16>())
                            .cast::<u8>(),
                            (((256i32)
                                .wrapping_add(((adjustedPaletteNum) as i32).wrapping_mul(16i32)))
                                as u16),
                            32u16,
                        );
                        break 'l1;
                    }
                    if __sw1 == 236i32 {
                        LoadPalette(
                            (((&raw mut gObjectEventPal_RubySapphireMay).cast::<u16>())
                                .cast::<u16>())
                            .cast::<u8>(),
                            (((256i32)
                                .wrapping_add(((adjustedPaletteNum) as i32).wrapping_mul(16i32)))
                                as u16),
                            32u16,
                        );
                        break 'l1;
                    }
                    if __sw1 == 100i32 {
                        LoadPalette(
                            (((&raw mut gObjectEventPal_Brendan).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            (((256i32)
                                .wrapping_add(((adjustedPaletteNum) as i32).wrapping_mul(16i32)))
                                as u16),
                            32u16,
                        );
                        break 'l1;
                    }
                    if __sw1 == 105i32 {
                        LoadPalette(
                            (((&raw mut gObjectEventPal_May).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            (((256i32)
                                .wrapping_add(((adjustedPaletteNum) as i32).wrapping_mul(16i32)))
                                as u16),
                            32u16,
                        );
                        break 'l1;
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MauvilleGymPressSwitch() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((i) as i32)
                        == ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                    {
                        MapGridSetMetatileIdAt(
                            ((((((&raw const sMauvilleGymSwitchCoords)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .read()) as i32),
                            (((((((&raw const sMauvilleGymSwitchCoords)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(1))
                            .read()) as i32),
                            518u16,
                        );
                    } else {
                        MapGridSetMetatileIdAt(
                            ((((((&raw const sMauvilleGymSwitchCoords)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .read()) as i32),
                            (((((((&raw const sMauvilleGymSwitchCoords)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(1))
                            .read()) as i32),
                            517u16,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MauvilleGymSetDefaultBarriers() {
    unsafe {
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        {
            y = 12i32;
            'l1: loop {
                if !(y < 24i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        x = 7i32;
                        'l3: loop {
                            if !(x < 16i32) {
                                break 'l3;
                            }
                            'l4: {
                                'l5: {
                                    let __sw1 = MapGridGetMetatileIdAt(x, y);
                                    if __sw1 == 544i32 {
                                        MapGridSetMetatileIdAt(x, y, 560u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 545i32 {
                                        MapGridSetMetatileIdAt(x, y, 561u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 552i32 {
                                        MapGridSetMetatileIdAt(x, y, 568u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 553i32 {
                                        MapGridSetMetatileIdAt(x, y, 569u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 560i32 {
                                        MapGridSetMetatileIdAt(x, y, 544u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 561i32 {
                                        MapGridSetMetatileIdAt(x, y, 545u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 568i32 {
                                        MapGridSetMetatileIdAt(x, y, 3624u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 569i32 {
                                        MapGridSetMetatileIdAt(x, y, 3625u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 546i32 {
                                        MapGridSetMetatileIdAt(x, y, 562u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 547i32 {
                                        MapGridSetMetatileIdAt(x, y, 563u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 554i32 {
                                        MapGridSetMetatileIdAt(x, y, 570u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 555i32 {
                                        MapGridSetMetatileIdAt(x, y, 571u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 562i32 {
                                        MapGridSetMetatileIdAt(x, y, 546u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 563i32 {
                                        MapGridSetMetatileIdAt(x, y, 547u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 570i32 {
                                        MapGridSetMetatileIdAt(x, y, 3626u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 571i32 {
                                        MapGridSetMetatileIdAt(x, y, 3627u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 576i32 {
                                        MapGridSetMetatileIdAt(x, y, 3650u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 584i32 {
                                        MapGridSetMetatileIdAt(x, y, 538u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 577i32 {
                                        MapGridSetMetatileIdAt(x, y, 3651u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 585i32 {
                                        MapGridSetMetatileIdAt(x, y, 538u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 578i32 {
                                        MapGridSetMetatileIdAt(x, y, 3648u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 538i32 {
                                        if MapGridGetMetatileIdAt(x, (y).wrapping_sub(1i32))
                                            == 576i32
                                        {
                                            MapGridSetMetatileIdAt(x, y, 3656u16);
                                        } else {
                                            MapGridSetMetatileIdAt(x, y, 3657u16);
                                        }
                                        break 'l5;
                                    }
                                    if __sw1 == 579i32 {
                                        MapGridSetMetatileIdAt(x, y, 3649u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 593i32 {
                                        MapGridSetMetatileIdAt(x, y, 3664u16);
                                        break 'l5;
                                    }
                                    if __sw1 == 592i32 {
                                        MapGridSetMetatileIdAt(x, y, 593u16);
                                        break 'l5;
                                    }
                                }
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                }
                y = (y).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MauvilleGymDeactivatePuzzle() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut switchCoords: *mut u8 = ((&raw const sMauvilleGymSwitchCoords)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>();
        {
            i = (((crate::c::div_u32(16u32, 4u32)).wrapping_sub(1u32)) as i32);
            'l1: loop {
                if !(i >= 0i32) {
                    break 'l1;
                }
                'l2: {
                    MapGridSetMetatileIdAt(
                        (((switchCoords).read()) as i32),
                        ((((switchCoords).wrapping_add(1)).read()) as i32),
                        518u16,
                    );
                    switchCoords = (switchCoords).wrapping_offset(4);
                }
                i = (i).wrapping_sub(1);
            }
        }
        {
            y = 12i32;
            'l3: loop {
                if !(y < 24i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        x = 7i32;
                        'l5: loop {
                            if !(x < 16i32) {
                                break 'l5;
                            }
                            'l6: {
                                'l7: {
                                    let __sw1 = MapGridGetMetatileIdAt(x, y);
                                    if __sw1 == 544i32 {
                                        MapGridSetMetatileIdAt(x, y, 560u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 545i32 {
                                        MapGridSetMetatileIdAt(x, y, 561u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 552i32 {
                                        MapGridSetMetatileIdAt(x, y, 568u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 553i32 {
                                        MapGridSetMetatileIdAt(x, y, 569u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 546i32 {
                                        MapGridSetMetatileIdAt(x, y, 562u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 547i32 {
                                        MapGridSetMetatileIdAt(x, y, 563u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 554i32 {
                                        MapGridSetMetatileIdAt(x, y, 570u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 555i32 {
                                        MapGridSetMetatileIdAt(x, y, 571u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 576i32 {
                                        MapGridSetMetatileIdAt(x, y, 3650u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 577i32 {
                                        MapGridSetMetatileIdAt(x, y, 3651u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 584i32 || __sw1 == 585i32 {
                                        MapGridSetMetatileIdAt(x, y, 538u16);
                                        break 'l7;
                                    }
                                    if __sw1 == 592i32 {
                                        MapGridSetMetatileIdAt(x, y, 593u16);
                                        break 'l7;
                                    }
                                }
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                }
                y = (y).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PetalburgGymSlideOpenRoomDoors() {
    unsafe {
        ((&raw mut sSlidingDoorNextFrameCounter)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        ((&raw mut sSlidingDoorFrame).cast::<u8>().cast::<u8>()).write(0u8);
        PlaySE(44u16);
        CreateTask(Some(Task_PetalburgGymSlideOpenRoomDoors), 8u8);
    }
}
pub(crate) unsafe extern "C" fn Task_PetalburgGymSlideOpenRoomDoors(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw const sSlidingDoorNextFrameDelay)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut sSlidingDoorFrame).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .read()) as i32)
            == ((((&raw mut sSlidingDoorNextFrameCounter)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32)
        {
            PetalburgGymSetDoorMetatiles(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
                ((((&raw const sPetalburgGymSlidingDoorMetatiles)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    ((((&raw mut sSlidingDoorFrame).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read(),
            );
            ((&raw mut sSlidingDoorNextFrameCounter)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            if (({
                let __p1 = (&raw mut sSlidingDoorFrame).cast::<u8>().cast::<u8>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as u32)
                == crate::c::div_u32(10u32, 2u32)
            {
                DestroyTask(taskId);
                ScriptContext_Enable();
            }
        } else {
            let __p3 = (&raw mut sSlidingDoorNextFrameCounter)
                .cast::<u8>()
                .cast::<u8>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn PetalburgGymSetDoorMetatiles(roomNumber: u8, metatileId: u16) {
    unsafe {
        let mut roomNumber = roomNumber;
        let mut metatileId = metatileId;
        let mut doorCoordsX = crate::ffi::Align4([0u8; 8]);
        let mut doorCoordsY = crate::ffi::Align4([0u8; 8]);
        let mut i: u8 = 0u8;
        let mut nDoors: u8 = 0u8;
        'l1: {
            let __sw1 = ((roomNumber) as i32);
            if __sw1 == 1i32 {
                nDoors = 2u8;
                ((&raw mut doorCoordsX).cast::<u16>()).write(1u16);
                (((&raw mut doorCoordsX).cast::<u16>()).wrapping_offset(1)).write(7u16);
                ((&raw mut doorCoordsY).cast::<u16>()).write(104u16);
                (((&raw mut doorCoordsY).cast::<u16>()).wrapping_offset(1)).write(104u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                nDoors = 2u8;
                ((&raw mut doorCoordsX).cast::<u16>()).write(1u16);
                (((&raw mut doorCoordsX).cast::<u16>()).wrapping_offset(1)).write(7u16);
                ((&raw mut doorCoordsY).cast::<u16>()).write(78u16);
                (((&raw mut doorCoordsY).cast::<u16>()).wrapping_offset(1)).write(78u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                nDoors = 2u8;
                ((&raw mut doorCoordsX).cast::<u16>()).write(1u16);
                (((&raw mut doorCoordsX).cast::<u16>()).wrapping_offset(1)).write(7u16);
                ((&raw mut doorCoordsY).cast::<u16>()).write(91u16);
                (((&raw mut doorCoordsY).cast::<u16>()).wrapping_offset(1)).write(91u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                nDoors = 1u8;
                ((&raw mut doorCoordsX).cast::<u16>()).write(7u16);
                ((&raw mut doorCoordsY).cast::<u16>()).write(39u16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                nDoors = 2u8;
                ((&raw mut doorCoordsX).cast::<u16>()).write(1u16);
                (((&raw mut doorCoordsX).cast::<u16>()).wrapping_offset(1)).write(7u16);
                ((&raw mut doorCoordsY).cast::<u16>()).write(52u16);
                (((&raw mut doorCoordsY).cast::<u16>()).wrapping_offset(1)).write(52u16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                nDoors = 1u8;
                ((&raw mut doorCoordsX).cast::<u16>()).write(1u16);
                ((&raw mut doorCoordsY).cast::<u16>()).write(65u16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                nDoors = 1u8;
                ((&raw mut doorCoordsX).cast::<u16>()).write(7u16);
                ((&raw mut doorCoordsY).cast::<u16>()).write(13u16);
                break 'l1;
            }
            if __sw1 == 8i32 {
                nDoors = 1u8;
                ((&raw mut doorCoordsX).cast::<u16>()).write(1u16);
                ((&raw mut doorCoordsY).cast::<u16>()).write(26u16);
                break 'l1;
            }
        }
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as i32) < ((nDoors) as i32)) {
                    break 'l2;
                }
                'l3: {
                    MapGridSetMetatileIdAt(
                        (((((&raw mut doorCoordsX).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            .wrapping_add(7i32),
                        (((((&raw mut doorCoordsY).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            .wrapping_add(7i32),
                        ((((metatileId) as i32) | 3072i32) as u16),
                    );
                    MapGridSetMetatileIdAt(
                        (((((&raw mut doorCoordsX).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            .wrapping_add(7i32),
                        ((((((&raw mut doorCoordsY).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            .wrapping_add(7i32))
                        .wrapping_add(1i32),
                        ((((metatileId) as i32).wrapping_add(8i32) | 3072i32) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DrawWholeMapView();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PetalburgGymUnlockRoomDoors() {
    unsafe {
        PetalburgGymSetDoorMetatiles(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
            ((((&raw const sPetalburgGymSlidingDoorMetatiles)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(4))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFieldMessageStringVar4() {
    unsafe {
        ShowFieldMessage((&raw mut gStringVar4).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StorePlayerCoordsInVars() {
    unsafe {
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as u16),
        );
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerTrainerIdOnesDigit() -> u8 {
    unsafe {
        return ((crate::c::rem_i32(
            ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)
                | (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>())
                .read()) as i32)) as u16) as i32),
            10i32,
        )) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerBigGuyGirlString() {
    unsafe {
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 0i32
        {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_BigGuy).cast::<u8>(),
            );
        } else {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_BigGirl).cast::<u8>(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRivalSonDaughterString() {
    unsafe {
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 0i32
        {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_Daughter).cast::<u8>(),
            );
        } else {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_Son).cast::<u8>(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleOutcome() -> u8 {
    unsafe {
        return ((&raw mut gBattleOutcome).cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CableCarWarp() {
    unsafe {
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) != 0i32 {
            SetWarpDestination(19i8, 0i8, (-1i8), 6i8, 4i8);
        } else {
            SetWarpDestination(19i8, 1i8, (-1i8), 6i8, 4i8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetHiddenItemFlag() {
    unsafe {
        FlagSet(((&raw mut gSpecialVar_0x8004).cast::<u16>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWeekCount() -> u16 {
    unsafe {
        let mut weekCount: u16 = ((crate::c::div_i32(
            (((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read()) as i32),
            7i32,
        )) as u16);
        if ((weekCount) as i32) > 9999i32 {
            weekCount = 9999u16;
        }
        return weekCount;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLeadMonFriendshipScore() -> u8 {
    unsafe {
        let mut pokemon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
            .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100);
        if GetMonData2(pokemon, 32i32) == 255u32 {
            return 6u8;
        }
        if GetMonData2(pokemon, 32i32) >= 200u32 {
            return 5u8;
        }
        if GetMonData2(pokemon, 32i32) >= 150u32 {
            return 4u8;
        }
        if GetMonData2(pokemon, 32i32) >= 100u32 {
            return 3u8;
        }
        if GetMonData2(pokemon, 32i32) >= 50u32 {
            return 2u8;
        }
        if GetMonData2(pokemon, 32i32) >= 1u32 {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CB2_FieldShowRegionMap() {
    unsafe {
        FieldInitRegionMap(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldShowRegionMap() {
    unsafe {
        SetMainCallback2(Some(CB2_FieldShowRegionMap));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPCTurnOnEffect() {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_PCTurnOnEffect))) as i32) != 1i32 {
            let mut taskId: u8 = CreateTask(Some(Task_PCTurnOnEffect), 8u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((taskId) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PCTurnOnEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !(((((task).wrapping_add(8)).cast::<i16>()).read()) != 0) {
            PCTurnOnEffect(task);
        }
    }
}
pub(crate) unsafe extern "C" fn PCTurnOnEffect(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut playerDirection: u8 = 0u8;
        let mut dx: i8 = 0i8;
        let mut dy: i8 = 0i8;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) == 6i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            playerDirection = GetPlayerFacingDirection();
            'l1: {
                let __sw1 = ((playerDirection) as i32);
                if __sw1 == 2i32 {
                    dx = 0i8;
                    dy = (-1i8);
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    dx = (-1i8);
                    dy = (-1i8);
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    dx = 1i8;
                    dy = (-1i8);
                    break 'l1;
                }
            }
            PCTurnOnEffect_SetMetatile(
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                dx,
                dy,
            );
            DrawWholeMapView();
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p2).write((((((__p2).read()) as i32) ^ 1i32) as i16));
            if (({
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 5i32
            {
                DestroyTask(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
                );
            }
        }
        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        (__p5).write(((__p5).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn PCTurnOnEffect_SetMetatile(isScreenOn: i16, dx: i8, dy: i8) {
    unsafe {
        let mut isScreenOn = isScreenOn;
        let mut dx = dx;
        let mut dy = dy;
        let mut metatileId: u16 = 0u16;
        if (isScreenOn) != 0 {
            if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
                metatileId = 4u16;
            } else {
                if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32 {
                    metatileId = 602u16;
                } else {
                    if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 2i32 {
                        metatileId = 601u16;
                    }
                }
            }
        } else {
            if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
                metatileId = 5u16;
            } else {
                if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32 {
                    metatileId = 639u16;
                } else {
                    if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 2i32 {
                        metatileId = 638u16;
                    }
                }
            }
        }
        MapGridSetMetatileIdAt(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                .wrapping_add(((dx) as i32)))
            .wrapping_add(7i32),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_add(((dy) as i32)))
            .wrapping_add(7i32),
            ((((metatileId) as i32) | 3072i32) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPCTurnOffEffect() {
    unsafe {
        PCTurnOffEffect();
    }
}
pub(crate) unsafe extern "C" fn PCTurnOffEffect() {
    unsafe {
        let mut dx: i8 = 0i8;
        let mut dy: i8 = 0i8;
        let mut metatileId: u16 = 0u16;
        let mut playerDirection: u8 = GetPlayerFacingDirection();
        'l1: {
            let __sw1 = ((playerDirection) as i32);
            if __sw1 == 2i32 {
                dx = 0i8;
                dy = (-1i8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                dx = (-1i8);
                dy = (-1i8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                dx = 1i8;
                dy = (-1i8);
                break 'l1;
            }
        }
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
            metatileId = 4u16;
        } else {
            if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32 {
                metatileId = 602u16;
            } else {
                if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 2i32 {
                    metatileId = 601u16;
                }
            }
        }
        MapGridSetMetatileIdAt(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                .wrapping_add(((dx) as i32)))
            .wrapping_add(7i32),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_add(((dy) as i32)))
            .wrapping_add(7i32),
            ((((metatileId) as i32) | 3072i32) as u16),
        );
        DrawWholeMapView();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoLotteryCornerComputerEffect() {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_LotteryCornerComputerEffect))) as i32) != 1i32 {
            let mut taskId: u8 = CreateTask(Some(Task_LotteryCornerComputerEffect), 8u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((taskId) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LotteryCornerComputerEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !(((((task).wrapping_add(8)).cast::<i16>()).read()) != 0) {
            LotteryCornerComputerEffect(task);
        }
    }
}
pub(crate) unsafe extern "C" fn LotteryCornerComputerEffect(task: *mut u8) {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) == 6i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) != 0 {
                MapGridSetMetatileIdAt(18i32, 8i32, 3741u16);
                MapGridSetMetatileIdAt(18i32, 9i32, 3749u16);
            } else {
                MapGridSetMetatileIdAt(18i32, 8i32, 3672u16);
                MapGridSetMetatileIdAt(18i32, 9i32, 3680u16);
            }
            DrawWholeMapView();
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p1).write((((((__p1).read()) as i32) ^ 1i32) as i16));
            if (({
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 5i32
            {
                DestroyTask(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
                );
            }
        }
        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        (__p4).write(((__p4).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EndLotteryCornerComputerEffect() {
    unsafe {
        MapGridSetMetatileIdAt(18i32, 8i32, 3741u16);
        MapGridSetMetatileIdAt(18i32, 9i32, 3749u16);
        DrawWholeMapView();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTrickHouseNuggetFlag() {
    unsafe {
        let mut specVar: *mut u16 = (&raw mut gSpecialVar_0x8004).cast::<u16>();
        let mut flag: u16 = 501u16;
        (specVar).write(flag);
        FlagSet(flag);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetTrickHouseNuggetFlag() {
    unsafe {
        let mut specVar: *mut u16 = (&raw mut gSpecialVar_0x8004).cast::<u16>();
        let mut flag: u16 = 501u16;
        (specVar).write(flag);
        FlagClear(flag);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonCool() -> u8 {
    unsafe {
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
            22i32,
        ) < 200u32
        {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonBeauty() -> u8 {
    unsafe {
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
            23i32,
        ) < 200u32
        {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonCute() -> u8 {
    unsafe {
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
            24i32,
        ) < 200u32
        {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonSmart() -> u8 {
    unsafe {
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
            33i32,
        ) < 200u32
        {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonTough() -> u8 {
    unsafe {
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
            47i32,
        ) < 200u32
        {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsGrassTypeInParty() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut species: u16 = 0u16;
        let mut pokemon: *mut u8 = core::ptr::null_mut();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    pokemon = ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 100);
                    if ((GetMonData2(pokemon, 5i32)) != 0)
                        && (!((GetMonData2(pokemon, 45i32)) != 0))
                    {
                        species = ((GetMonData2(pokemon, 11i32)) as u16);
                        if ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(6))
                        .cast::<u8>())
                        .read()) as i32)
                            == 12i32)
                            || (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 28))
                            .wrapping_add(6))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                == 12i32)
                        {
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                            return;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpawnCameraObject() {
    unsafe {
        let mut obj: u8 = SpawnSpecialObjectEventParameterized(
            7u8,
            8u8,
            127u8,
            ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                .wrapping_add(7i32)) as i16),
            ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_add(7i32)) as i16),
            3u8,
        );
        crate::c::bf_write(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(((obj) as i32) as isize * 36))
                .wrapping_add(1),
            5,
            1,
            (1u32) as i32,
        );
        CameraObjectSetFollowedSpriteId(
            ((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((obj) as i32) as isize * 36))
            .wrapping_add(4))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveCameraObject() {
    unsafe {
        CameraObjectSetFollowedSpriteId(GetPlayerAvatarSpriteId());
        RemoveObjectEventByLocalIdAndMap(
            127u8,
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblockNameByMonNature() -> u8 {
    unsafe {
        return CopyMonFavoritePokeblockName(
            GetNature(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
            ),
            (&raw mut gStringVar1).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseNearbyMapName() {
    unsafe {
        GetMapName((&raw mut gStringVar1).cast::<u8>(), VarGet(16422u16), 0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleTowerSinglesStreak() -> u16 {
    unsafe {
        return ((GetGameStat(32u8)) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferEReaderTrainerName() {
    unsafe {
        GetEreaderTrainerName((&raw mut gStringVar1).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSlotMachineId() -> u16 {
    unsafe {
        let mut rnd: u32 = (((((crate::c::bf_read(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11880))
                .cast::<u8>())
            .wrapping_add(0),
            0,
            7,
            false,
        ) as u16) as i32)
            .wrapping_add(
                ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11880))
                    .cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32),
            ))
        .wrapping_add(
            ((((((&raw const sSlotMachineRandomSeeds_34)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
            ))
            .read()) as i32),
        )) as u32);
        if (IsPokeNewsActive(2u8)) != 0 {
            return ((((((&raw const sSlotMachineServiceDayIds_33)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((crate::c::rem_u32(rnd, 12u32)) as i32) as isize))
            .read()) as u16);
        }
        return ((((((&raw const sSlotMachineIds_32).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((crate::c::rem_u32(rnd, 12u32)) as i32) as isize))
        .read()) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundAbandonedShipRoom1Key() -> u8 {
    unsafe {
        let mut specVar: *mut u16 = (&raw mut gSpecialVar_0x8004).cast::<u16>();
        let mut flag: u16 = 531u16;
        (specVar).write(flag);
        if !((FlagGet(flag)) != 0) {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundAbandonedShipRoom2Key() -> u8 {
    unsafe {
        let mut specVar: *mut u16 = (&raw mut gSpecialVar_0x8004).cast::<u16>();
        let mut flag: u16 = 532u16;
        (specVar).write(flag);
        if !((FlagGet(flag)) != 0) {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundAbandonedShipRoom4Key() -> u8 {
    unsafe {
        let mut specVar: *mut u16 = (&raw mut gSpecialVar_0x8004).cast::<u16>();
        let mut flag: u16 = 533u16;
        (specVar).write(flag);
        if !((FlagGet(flag)) != 0) {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundAbandonedShipRoom6Key() -> u8 {
    unsafe {
        let mut specVar: *mut u16 = (&raw mut gSpecialVar_0x8004).cast::<u16>();
        let mut flag: u16 = 534u16;
        (specVar).write(flag);
        if !((FlagGet(flag)) != 0) {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LeadMonHasEffortRibbon() -> u8 {
    unsafe {
        return ((GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
            71i32,
            core::ptr::null_mut(),
        )) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveLeadMonEffortRibbon() {
    unsafe {
        let mut ribbonSet: u8 = 0u8;
        let mut leadMon: *mut u8 = core::ptr::null_mut();
        IncrementGameStat(42u8);
        FlagSet(2203u16);
        ribbonSet = 1u8;
        leadMon = ((&raw mut gPlayerParty).cast::<u8>())
            .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100);
        SetMonData(leadMon, 71i32, &raw mut ribbonSet);
        if ((GetRibbonCount(leadMon)) as i32) > 4i32 {
            TryPutSpotTheCutiesOnAir(leadMon, 71u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Special_AreLeadMonEVsMaxedOut() -> u8 {
    unsafe {
        if ((GetMonEVCount(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
        )) as i32)
            >= 510i32
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryUpdateRusturfTunnelState() -> u8 {
    unsafe {
        if ((!((FlagGet(199u16)) != 0))
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 24i32))
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 4i32)
        {
            if (FlagGet(931u16)) != 0 {
                VarSet(16538u16, 4u16);
                return 1u8;
            } else {
                if (FlagGet(932u16)) != 0 {
                    VarSet(16538u16, 5u16);
                    return 1u8;
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetShoalItemFlag(unused: u16) {
    unsafe {
        let mut unused = unused;
        FlagSet(2239u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadWallyZigzagoon() {
    unsafe {
        let mut monData: u16 = 0u16;
        CreateMon(
            (&raw mut gPlayerParty).cast::<u8>(),
            288u16,
            7u8,
            32u8,
            0u8,
            0u32,
            0u8,
            0u32,
        );
        monData = 1u16;
        SetMonData(
            (&raw mut gPlayerParty).cast::<u8>(),
            46i32,
            (&raw mut monData).cast::<u8>(),
        );
        monData = 33u16;
        SetMonData(
            (&raw mut gPlayerParty).cast::<u8>(),
            13i32,
            (&raw mut monData).cast::<u8>(),
        );
        monData = 0u16;
        SetMonData(
            (&raw mut gPlayerParty).cast::<u8>(),
            14i32,
            (&raw mut monData).cast::<u8>(),
        );
        SetMonData(
            (&raw mut gPlayerParty).cast::<u8>(),
            15i32,
            (&raw mut monData).cast::<u8>(),
        );
        SetMonData(
            (&raw mut gPlayerParty).cast::<u8>(),
            16i32,
            (&raw mut monData).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsStarterInParty() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut starter: u16 = GetStarterPokemon(VarGet(16419u16));
        let mut partyCount: u8 = CalculatePlayerPartyCount();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((partyCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        65i32,
                        core::ptr::null_mut(),
                    ) == ((starter) as u32)
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
pub unsafe extern "C" fn ScriptCheckFreePokemonStorageSpace() -> u8 {
    unsafe {
        return CheckFreePokemonStorageSpace();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokerusInParty() -> u8 {
    unsafe {
        if !((CheckPartyPokerus((&raw mut gPlayerParty).cast::<u8>(), 63u8)) != 0) {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShakeCamera() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_ShakeCamera), 9u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((((&raw mut gSpecialVar_0x8007).cast::<u16>()).read()) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i16));
        SetCameraPanningCallback(None);
        PlaySE(214u16);
    }
}
pub(crate) unsafe extern "C" fn Task_ShakeCamera(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if crate::c::rem_i32(
            ((((data).wrapping_offset(1)).read()) as i32),
            ((((data).wrapping_offset(3)).read()) as i32),
        ) == 0i32
        {
            ((data).wrapping_offset(1)).write(0i16);
            let __p2 = (data).wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_sub(1));
            (data).write((((((data).read()) as i32).wrapping_neg()) as i16));
            ((data).wrapping_offset(4))
                .write(((((((data).wrapping_offset(4)).read()) as i32).wrapping_neg()) as i16));
            SetCameraPanning((data).read(), ((data).wrapping_offset(4)).read());
            if ((((data).wrapping_offset(2)).read()) as i32) == 0i32 {
                StopCameraShake(taskId);
                InstallCameraPanAheadCallback();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StopCameraShake(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundBlackGlasses() -> u8 {
    unsafe {
        return FlagGet(596u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRoute119Weather() {
    unsafe {
        if ((IsMapTypeOutdoors(GetLastUsedWarpMapType())) as i32) != 1i32 {
            SetSavedWeather(20u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRoute123Weather() {
    unsafe {
        if ((IsMapTypeOutdoors(GetLastUsedWarpMapType())) as i32) != 1i32 {
            SetSavedWeather(21u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLeadMonIndex() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut partyCount: u8 = CalculatePlayerPartyCount();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((partyCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        65i32,
                        core::ptr::null_mut(),
                    ) != 412u32)
                        && (GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            65i32,
                            core::ptr::null_mut(),
                        ) != 0u32)
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptGetPartyMonSpecies() -> u16 {
    unsafe {
        return ((GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            65i32,
            core::ptr::null_mut(),
        )) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryInitBattleTowerAwardManObjectEvent() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDaysUntilPacifidlogTMAvailable() -> u16 {
    unsafe {
        let mut tmReceivedDay: u16 = VarGet(16578u16);
        if (((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read()) as i32)
            .wrapping_sub(((tmReceivedDay) as i32))
            >= 7i32
        {
            return 0u16;
        } else {
            if (((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read()) as i32) < 0i32 {
                return 8u16;
            }
        }
        return (((7i32).wrapping_sub(
            (((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read()) as i32)
                .wrapping_sub(((tmReceivedDay) as i32)),
        )) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPacifidlogTMReceivedDay() -> u16 {
    unsafe {
        VarSet(
            16578u16,
            (((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read()) as u16),
        );
        return (((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read()) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonOTNameNotPlayer() -> u8 {
    unsafe {
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            3i32,
        ) != 2u32
        {
            return 1u8;
        }
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            7i32,
            (&raw mut gStringVar1).cast::<u8>(),
        );
        if !((StringCompare(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            (&raw mut gStringVar1).cast::<u8>(),
        )) != 0)
        {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferLottoTicketNumber() {
    unsafe {
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) >= 10000i32 {
            ConvertIntToDecimalString(
                0u8,
                ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32),
            );
        } else {
            if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) >= 1000i32 {
                ((&raw mut gStringVar1).cast::<u8>()).write(161u8);
                ConvertIntToDecimalStringN(
                    ((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(1),
                    ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32),
                    0i32,
                    ((CountDigits(((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32)))
                        as u8),
                );
            } else {
                if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) >= 100i32 {
                    ((&raw mut gStringVar1).cast::<u8>()).write(161u8);
                    (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(1)).write(161u8);
                    ConvertIntToDecimalStringN(
                        ((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(2),
                        ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32),
                        0i32,
                        ((CountDigits(
                            ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32),
                        )) as u8),
                    );
                } else {
                    if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) >= 10i32 {
                        ((&raw mut gStringVar1).cast::<u8>()).write(161u8);
                        (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(1)).write(161u8);
                        (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(2)).write(161u8);
                        ConvertIntToDecimalStringN(
                            ((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(3),
                            ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32),
                            0i32,
                            ((CountDigits(
                                ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32),
                            )) as u8),
                        );
                    } else {
                        ((&raw mut gStringVar1).cast::<u8>()).write(161u8);
                        (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(1)).write(161u8);
                        (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(2)).write(161u8);
                        (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(3)).write(161u8);
                        ConvertIntToDecimalStringN(
                            ((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(4),
                            ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32),
                            0i32,
                            ((CountDigits(
                                ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32),
                            )) as u8),
                        );
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMysteryGiftCardStat() -> u16 {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                return MysteryGift_GetCardStat(3u32);
            }
            if __sw1 == 1i32 {
                return MysteryGift_GetCardStat(4u32);
            }
            if __sw1 == 2i32 {
                return MysteryGift_GetCardStat(0u32);
            }
            if __sw1 == 3i32 {
                return MysteryGift_GetCardStat(1u32);
            }
            if __sw1 == 4i32 {
                return MysteryGift_GetCardStat(2u32);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferTMHMMoveName() -> u8 {
    unsafe {
        if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) >= 289i32)
            && (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) <= 346i32)
        {
            StringCopy(
                (&raw mut gStringVar2).cast::<u8>(),
                (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                    ((ItemIdToBattleMoveId(((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()))
                        as i32) as isize
                        * 13,
                ))
                .cast::<u8>(),
            );
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBadEggInParty() -> u8 {
    unsafe {
        let mut partyCount: u8 = CalculatePlayerPartyCount();
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((partyCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        4i32,
                    ) == 1u32
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
pub unsafe extern "C" fn InMultiPartnerRoom() -> u8 {
    unsafe {
        if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 26i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 15i32))
            && (((VarGet(16590u16)) as i32) == 2i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OffsetCameraForBattle() {
    unsafe {
        SetCameraPanningCallback(None);
        SetCameraPanning(8i16, 0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDeptStoreFloor() {
    unsafe {
        let mut deptStoreFloor: u8 = 0u8;
        'l1: {
            let __sw1 = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(20))
            .wrapping_add(1)
            .cast::<i8>())
            .read()) as i32);
            let __matched = __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32
                || __sw1 == 21i32;
            if __sw1 == 16i32 {
                deptStoreFloor = 4u8;
                break 'l1;
            }
            if __sw1 == 17i32 {
                deptStoreFloor = 5u8;
                break 'l1;
            }
            if __sw1 == 18i32 {
                deptStoreFloor = 6u8;
                break 'l1;
            }
            if __sw1 == 19i32 {
                deptStoreFloor = 7u8;
                break 'l1;
            }
            if __sw1 == 20i32 {
                deptStoreFloor = 8u8;
                break 'l1;
            }
            if __sw1 == 21i32 {
                deptStoreFloor = 15u8;
                break 'l1;
            }
            if !__matched {
                deptStoreFloor = 4u8;
                break 'l1;
            }
        }
        VarSet(16451u16, ((deptStoreFloor) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDeptStoreDefaultFloorChoice() -> u16 {
    unsafe {
        ((&raw mut sLilycoveDeptStore_NeverRead)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sLilycoveDeptStore_DefaultFloorChoice)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20))
            .cast::<i8>())
        .read()) as i32)
            == 13i32
        {
            'l1: {
                let __sw1 = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .wrapping_add(1)
                .cast::<i8>())
                .read()) as i32);
                if __sw1 == 20i32 {
                    ((&raw mut sLilycoveDeptStore_NeverRead)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(0u16);
                    ((&raw mut sLilycoveDeptStore_DefaultFloorChoice)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(0u16);
                    break 'l1;
                }
                if __sw1 == 19i32 {
                    ((&raw mut sLilycoveDeptStore_NeverRead)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(0u16);
                    ((&raw mut sLilycoveDeptStore_DefaultFloorChoice)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(1u16);
                    break 'l1;
                }
                if __sw1 == 18i32 {
                    ((&raw mut sLilycoveDeptStore_NeverRead)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(0u16);
                    ((&raw mut sLilycoveDeptStore_DefaultFloorChoice)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(2u16);
                    break 'l1;
                }
                if __sw1 == 17i32 {
                    ((&raw mut sLilycoveDeptStore_NeverRead)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(0u16);
                    ((&raw mut sLilycoveDeptStore_DefaultFloorChoice)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(3u16);
                    break 'l1;
                }
                if __sw1 == 16i32 {
                    ((&raw mut sLilycoveDeptStore_NeverRead)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(0u16);
                    ((&raw mut sLilycoveDeptStore_DefaultFloorChoice)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(4u16);
                    break 'l1;
                }
            }
        }
        return ((&raw mut sLilycoveDeptStore_DefaultFloorChoice)
            .cast::<u8>()
            .cast::<u16>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveElevator() {
    unsafe {
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((CreateTask(Some(Task_MoveElevator), 9u8)) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut floorDelta: u16 = 0u16;
        ((data).wrapping_offset(1)).write(0i16);
        ((data).wrapping_offset(2)).write(0i16);
        ((data).wrapping_offset(4)).write(1i16);
        if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
            > ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)
        {
            floorDelta = ((((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                .wrapping_sub(((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)))
                as u16);
            ((data).wrapping_offset(6)).write(1i16);
        } else {
            floorDelta = ((((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)
                .wrapping_sub(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)))
                as u16);
            ((data).wrapping_offset(6)).write(0i16);
        }
        if ((floorDelta) as i32) > 8i32 {
            floorDelta = 8u16;
        }
        ((data).wrapping_offset(5)).write(
            ((((((&raw const sElevatorTripLength_31).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((floorDelta) as i32) as isize))
            .read()) as i16),
        );
        SetCameraPanningCallback(None);
        MoveElevatorWindowLights(floorDelta, ((((data).wrapping_offset(6)).read()) as u8));
        PlaySE(89u16);
    }
}
pub(crate) unsafe extern "C" fn Task_MoveElevator(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if crate::c::rem_i32(((((data).wrapping_offset(1)).read()) as i32), 3i32) == 0i32 {
            ((data).wrapping_offset(1)).write(0i16);
            let __p2 = (data).wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((data).wrapping_offset(4))
                .write(((((((data).wrapping_offset(4)).read()) as i32).wrapping_neg()) as i16));
            SetCameraPanning(0i16, ((data).wrapping_offset(4)).read());
            if ((((data).wrapping_offset(2)).read()) as i32)
                == ((((data).wrapping_offset(5)).read()) as i32)
            {
                PlaySE(73u16);
                DestroyTask(taskId);
                ScriptContext_Enable();
                InstallCameraPanAheadCallback();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowDeptStoreElevatorFloorSelect() {
    unsafe {
        let mut xPos: i32 = 0i32;
        ((&raw mut sTutorMoveAndElevatorWindowId)
            .cast::<u8>()
            .cast::<u8>())
        .write(
            ((AddWindow(
                (&raw const sWindowTemplate_ElevatorFloor)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        SetStandardWindowBorderStyle(
            ((&raw mut sTutorMoveAndElevatorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            0u8,
        );
        xPos =
            GetStringCenterAlignXOffset(1i32, (&raw mut gText_ElevatorNowOn).cast::<u8>(), 64i32);
        AddTextPrinterParameterized(
            ((&raw mut sTutorMoveAndElevatorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            1u8,
            (&raw mut gText_ElevatorNowOn).cast::<u8>(),
            ((xPos) as u8),
            1u8,
            255u8,
            None,
        );
        xPos = GetStringCenterAlignXOffset(
            1i32,
            ((((&raw const sDeptStoreFloorNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize,
            ))
            .read(),
            64i32,
        );
        AddTextPrinterParameterized(
            ((&raw mut sTutorMoveAndElevatorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            1u8,
            ((((&raw const sDeptStoreFloorNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize,
            ))
            .read(),
            ((xPos) as u8),
            17u8,
            255u8,
            None,
        );
        PutWindowTilemap(
            ((&raw mut sTutorMoveAndElevatorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
        );
        CopyWindowToVram(
            ((&raw mut sTutorMoveAndElevatorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            3u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseDeptStoreElevatorWindow() {
    unsafe {
        ClearStdWindowAndFrameToTransparent(
            ((&raw mut sTutorMoveAndElevatorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            1u8,
        );
        RemoveWindow(
            ((&raw mut sTutorMoveAndElevatorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn MoveElevatorWindowLights(floorDelta: u16, descending: u8) {
    unsafe {
        let mut floorDelta = floorDelta;
        let mut descending = descending;
        if ((FuncIsActiveTask(Some(Task_MoveElevatorWindowLights))) as i32) != 1i32 {
            let mut taskId: u8 = CreateTask(Some(Task_MoveElevatorWindowLights), 8u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((descending) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(
                ((((((&raw const sElevatorLightCycles_30).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((floorDelta) as i32) as isize))
                .read()) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_MoveElevatorWindowLights(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((data).wrapping_offset(1)).read()) as i32) == 6i32 {
            (data).write(((data).read()).wrapping_add(1));
            if !((((data).wrapping_offset(2)).read()) != 0) {
                {
                    y = 0u8;
                    'l1: loop {
                        if !(((y) as i32) < 3i32) {
                            break 'l1;
                        }
                        'l2: {
                            {
                                x = 0u8;
                                'l3: loop {
                                    if !(((x) as i32) < 3i32) {
                                        break 'l3;
                                    }
                                    'l4: {
                                        MapGridSetMetatileIdAt((((((x) as i32))).wrapping_add(7i32)).wrapping_add(1i32), ((((y) as i32))).wrapping_add(7i32), ((((((((((((&raw const sElevatorWindowTiles_Ascending).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset((((y) as i32)) as isize * 6)).cast::<u16>()).wrapping_offset((crate::c::rem_i32(((((data).read()) as i32)), 3i32)) as isize)).read()) as i32)) | 3072i32)) as u16));
                                    }
                                    x = (x).wrapping_add(1);
                                }
                            }
                        }
                        y = (y).wrapping_add(1);
                    }
                }
            } else {
                {
                    y = 0u8;
                    'l5: loop {
                        if !(((y) as i32) < 3i32) {
                            break 'l5;
                        }
                        'l6: {
                            {
                                x = 0u8;
                                'l7: loop {
                                    if !(((x) as i32) < 3i32) {
                                        break 'l7;
                                    }
                                    'l8: {
                                        MapGridSetMetatileIdAt((((((x) as i32))).wrapping_add(7i32)).wrapping_add(1i32), ((((y) as i32))).wrapping_add(7i32), ((((((((((((&raw const sElevatorWindowTiles_Descending).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset((((y) as i32)) as isize * 6)).cast::<u16>()).wrapping_offset((crate::c::rem_i32(((((data).read()) as i32)), 3i32)) as isize)).read()) as i32)) | 3072i32)) as u16));
                                    }
                                    x = (x).wrapping_add(1);
                                }
                            }
                        }
                        y = (y).wrapping_add(1);
                    }
                }
            }
            DrawWholeMapView();
            ((data).wrapping_offset(1)).write(0i16);
            if (((data).read()) as i32) == ((((data).wrapping_offset(3)).read()) as i32) {
                DestroyTask(taskId);
            }
        }
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferVarsForIVRater() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut ivStorage = crate::ffi::Align4([0u8; 24]);
        ((&raw mut ivStorage).cast::<u32>()).write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            39i32,
        ));
        (((&raw mut ivStorage).cast::<u32>()).wrapping_offset(1)).write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            40i32,
        ));
        (((&raw mut ivStorage).cast::<u32>()).wrapping_offset(2)).write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            41i32,
        ));
        (((&raw mut ivStorage).cast::<u32>()).wrapping_offset(3)).write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            42i32,
        ));
        (((&raw mut ivStorage).cast::<u32>()).wrapping_offset(4)).write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            43i32,
        ));
        (((&raw mut ivStorage).cast::<u32>()).wrapping_offset(5)).write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            44i32,
        ));
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(0u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (&raw mut gSpecialVar_0x8005).cast::<u16>();
                    (__p1).write(
                        (((((__p1).read()) as u32).wrapping_add(
                            (((&raw mut ivStorage).cast::<u32>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        )) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(0u16);
        ((&raw mut gSpecialVar_0x8007).cast::<u16>())
            .write(((((&raw mut ivStorage).cast::<u32>()).read()) as u16));
        {
            i = 1u8;
            'l3: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l3;
                }
                'l4: {
                    if (((&raw mut ivStorage).cast::<u32>()).wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize,
                    ))
                    .read()
                        < (((&raw mut ivStorage).cast::<u32>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()
                    {
                        ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(((i) as u16));
                        ((&raw mut gSpecialVar_0x8007).cast::<u16>()).write(
                            (((((&raw mut ivStorage).cast::<u32>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as u16),
                        );
                    } else {
                        if (((&raw mut ivStorage).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)
                                as isize,
                        ))
                        .read()
                            == (((&raw mut ivStorage).cast::<u32>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()
                        {
                            let mut randomNumber: u16 = Random();
                            if (((randomNumber) as i32) & 1i32) != 0 {
                                ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(((i) as u16));
                                ((&raw mut gSpecialVar_0x8007).cast::<u16>()).write(
                                    (((((&raw mut ivStorage).cast::<u32>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as u16),
                                );
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UsedPokemonCenterWarp() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut map: u16 =
            ((((((((&raw mut gLastUsedWarp).cast::<u8>()).cast::<i8>()).read()) as i32) << 8)
                .wrapping_add(
                    (((((&raw mut gLastUsedWarp).cast::<u8>())
                        .wrapping_add(1)
                        .cast::<i8>())
                    .read()) as i32),
                )) as u16);
        {
            i = 0i32;
            'l1: loop {
                if !(((((((&raw const sPokemonCenters_29)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((i) as isize))
                .read()) as i32)
                    != 65535i32)
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sPokemonCenters_29)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((map) as i32)
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
pub unsafe extern "C" fn PlayerNotAtTrainerHillEntrance() -> u32 {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 26i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 60i32)
        {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateFrontierManiac(daysSince: u16) {
    unsafe {
        let mut daysSince = daysSince;
        let mut var: *mut u16 = GetVarPointer(16431u16);
        (var).write((((((var).read()) as i32).wrapping_add(((daysSince) as i32))) as u16));
        (var).write(((crate::c::rem_i32((((var).read()) as i32), 10i32)) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierManiacMessage() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut winStreak: u16 = 0u16;
        let mut facility: u16 = VarGet(16431u16);
        'l1: {
            let __sw1 = ((facility) as i32);
            if __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1684))
                .cast::<u8>())
                .wrapping_offset(((facility) as i32) as isize * 4))
                .cast::<u16>())
                .read()) as i32)
                    >= (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1684))
                    .cast::<u8>())
                    .wrapping_offset(((facility) as i32) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                {
                    winStreak = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1684))
                    .cast::<u8>())
                    .wrapping_offset(((facility) as i32) as isize * 4))
                    .cast::<u16>())
                    .read();
                } else {
                    winStreak = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1684))
                    .cast::<u8>())
                    .wrapping_offset(((facility) as i32) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read();
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1728))
                .cast::<u8>())
                .cast::<u16>())
                .read()) as i32)
                    >= ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1728))
                    .cast::<u8>())
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                {
                    winStreak = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1728))
                    .cast::<u8>())
                    .cast::<u16>())
                    .read();
                } else {
                    winStreak = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1728))
                    .cast::<u8>())
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read();
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1942))
                .cast::<u8>())
                .cast::<u16>())
                .read()) as i32)
                    >= ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1942))
                    .cast::<u8>())
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                {
                    winStreak = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1942))
                    .cast::<u8>())
                    .cast::<u16>())
                    .read();
                } else {
                    winStreak = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1942))
                    .cast::<u8>())
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read();
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1916))
                .cast::<u8>())
                .cast::<u16>())
                .read()) as i32)
                    >= ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1916))
                    .cast::<u8>())
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                {
                    winStreak = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1916))
                    .cast::<u8>())
                    .cast::<u16>())
                    .read();
                } else {
                    winStreak = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1916))
                    .cast::<u8>())
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read();
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1934))
                .cast::<u16>())
                .read()) as i32)
                    >= (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1934))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                {
                    winStreak = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1934))
                    .cast::<u16>())
                    .read();
                } else {
                    winStreak = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1934))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read();
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1976))
                .cast::<u16>())
                .read()) as i32)
                    >= (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1976))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                {
                    winStreak = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1976))
                    .cast::<u16>())
                    .read();
                } else {
                    winStreak = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1976))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read();
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1998))
                .cast::<u16>())
                .read()) as i32)
                    >= (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1998))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                {
                    winStreak = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1998))
                    .cast::<u16>())
                    .read();
                } else {
                    winStreak = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1998))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read();
                }
                break 'l1;
            }
        }
        {
            i = 0u8;
            'l2: loop {
                if !((((i) as i32) < 2i32)
                    && (((((((((&raw const sFrontierManiacStreakThresholds_28)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((facility) as i32) as isize * 2))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        < ((winStreak) as i32)))
                {
                    break 'l2;
                }
                'l3: {}
                i = (i).wrapping_add(1);
            }
        }
        ShowFieldMessage(
            ((((((&raw const sFrontierManiacMessages_27)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((facility) as i32) as isize * 12))
            .cast::<*mut u8>())
            .wrapping_offset(((i) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferBattleTowerElevatorFloors() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut battleMode: u16 = VarGet(16590u16);
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        if (((battleMode) as i32) == 2i32) && (!((FlagGet(338u16)) != 0)) {
            ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(5u16);
            ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(4u16);
            return;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < (crate::c::div_u32(20u32, 2u32)).wrapping_sub(1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sBattleTowerStreakThresholds_26)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        > (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1684))
                        .cast::<u8>())
                        .wrapping_offset(((battleMode) as i32) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(((lvlMode) as i32) as isize))
                        .read()) as i32)
                    {
                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(4u16);
                        ((&raw mut gSpecialVar_0x8006).cast::<u16>())
                            .write(((((i) as i32).wrapping_add(5i32)) as u16));
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(4u16);
        ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(12u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowScrollableMultichoice() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_ShowScrollableMultichoice), 8u8);
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
            .write(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i16));
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32);
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
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32;
            if __sw1 == 0i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(5i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(8i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(9i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(10i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(7i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(11i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(14i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(15i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 4i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(14i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(15i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 5i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(7i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(14i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(15i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 6i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(10i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(14i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(15i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 7i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(15i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(14i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 8i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(10i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(17i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(11i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 9i32 || __sw1 == 10i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(11i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(15i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(14i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 11i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(7i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(19i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(10i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if __sw1 == 12i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(7i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(17i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((taskId) as i16));
                break 'l1;
            }
            if !__matched {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(127u16);
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowScrollableMultichoice(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut width: u32 = 0u32;
        let mut i: u8 = 0u8;
        let mut windowId: u8 = 0u8;
        let mut template = crate::ffi::Align4([0u8; 8]);
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        LockPlayerFieldControls();
        ((&raw mut sScrollableMultichoice_ScrollOffset)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        ((&raw mut sScrollableMultichoice_ItemSpriteId)
            .cast::<u8>()
            .cast::<u8>())
        .write(64u8);
        FillFrontierExchangeCornerWindowAndItemIcon(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u16),
            0u16,
        );
        ShowBattleFrontierTutorWindow(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u8),
            0u16,
        );
        ((&raw mut sScrollableMultichoice_ListMenuItem)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u32)
                .wrapping_mul(8u32),
        ));
        ((&raw mut sFrontierExchangeCorner_NeverRead)
            .cast::<u8>()
            .cast::<u16>())
        .write(0u16);
        InitScrollableMultichoice();
        {
            width = 0u32;
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut text: *mut u8 = ((((((&raw const sScrollableMultichoiceOptions)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize
                            * 64,
                    ))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read();
                    (((((&raw mut sScrollableMultichoice_ListMenuItem)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .write(text);
                    (((((&raw mut sScrollableMultichoice_ListMenuItem)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .write(((i) as i32));
                    width = ((DisplayTextAndGetWidth(text, ((width) as i32))) as u32);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
            .write(((ConvertPixelWidthToTileWidth(((width) as i32))) as i16));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            .wrapping_add(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )
            > 29i32
        {
            let mut adjustedLeft: i32 = (29i32).wrapping_sub(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            );
            if adjustedLeft < 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                    .write(((adjustedLeft) as i16));
            }
        }
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(CreateWindowTemplate(
                0u8,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as u8),
                15u8,
                100u16,
            ));
        windowId = ((AddWindow((&raw mut template).cast::<u8>())) as u8);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(((windowId) as i16));
        SetStandardWindowBorderStyle(windowId, 0u8);
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16));
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write((((((task).wrapping_add(8)).cast::<i16>()).read()) as u16));
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(16))
            .write(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8));
        ScrollableMultichoice_UpdateScrollArrows(taskId);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
            ((ListMenuInit(
                (&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>(),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as u16),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as u16),
            )) as i16),
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ScrollableMultichoice_ProcessInput));
    }
}
pub(crate) unsafe extern "C" fn InitScrollableMultichoice() {
    unsafe {
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).cast::<*mut u8>())
            .write(
                ((&raw mut sScrollableMultichoice_ListMenuItem)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn(i32, u8, *mut u8)>>())
        .write(Some(ScrollableMultichoice_MoveCursor));
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn(u8, u32, u8)>>())
        .write(None);
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(1u16);
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write(1u16);
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(16))
            .write(0u8);
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(17))
            .write(0u8);
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(18))
            .write(8u8);
        (((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(19))
            .write(0u8);
        crate::c::bf_write(
            ((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(20),
            0,
            4,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(20),
            4,
            4,
            (2u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(21),
            0,
            4,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(21),
            4,
            4,
            (3u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(22),
            0,
            3,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(22),
            3,
            3,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(22),
            6,
            2,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(23),
            0,
            6,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gScrollableMultichoice_ListMenuTemplate).cast::<u8>()).wrapping_add(23),
            6,
            2,
            (0u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn ScrollableMultichoice_MoveCursor(
    itemIndex: i32,
    onInit: u8,
    list: *mut u8,
) {
    unsafe {
        let mut itemIndex = itemIndex;
        let mut onInit = onInit;
        let mut list = list;
        let mut taskId: u8 = 0u8;
        PlaySE(5u16);
        taskId = FindTaskIdByFunc(Some(ScrollableMultichoice_ProcessInput));
        if ((taskId) as i32) != 255i32 {
            let mut selection: u16 = 0u16;
            let mut task: *mut u8 =
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
            ListMenuGetScrollAndRow(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
                &raw mut selection,
                core::ptr::null_mut(),
            );
            ((&raw mut sScrollableMultichoice_ScrollOffset)
                .cast::<u8>()
                .cast::<u16>())
            .write(selection);
            ListMenuGetCurrentItemArrayId(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
                &raw mut selection,
            );
            HideFrontierExchangeCornerItemIcon(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u16),
                ((&raw mut sFrontierExchangeCorner_NeverRead)
                    .cast::<u8>()
                    .cast::<u16>())
                .read(),
            );
            FillFrontierExchangeCornerWindowAndItemIcon(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u16),
                selection,
            );
            ShowBattleFrontierTutorMoveDescription(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u8),
                selection,
            );
            ((&raw mut sFrontierExchangeCorner_NeverRead)
                .cast::<u8>()
                .cast::<u16>())
            .write(selection);
        }
    }
}
pub(crate) unsafe extern "C" fn ScrollableMultichoice_ProcessInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut input: i32 = ListMenu_ProcessInput(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
        );
        'l1: {
            let __sw1 = input;
            let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
            if __sw1 == (-1i32) {
                break 'l1;
            }
            if __sw1 == (-2i32) {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(127u16);
                PlaySE(5u16);
                CloseScrollableMultichoice(taskId);
                break 'l1;
            }
            if !__matched {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((input) as u16));
                PlaySE(5u16);
                if !((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) != 0) {
                    CloseScrollableMultichoice(taskId);
                } else {
                    if input
                        == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            .wrapping_sub(1i32)
                    {
                        CloseScrollableMultichoice(taskId);
                    } else {
                        ScrollableMultichoice_RemoveScrollArrows(taskId);
                        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                            .write(Some(Task_ScrollableMultichoice_WaitReturnToList));
                        ScriptContext_Enable();
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CloseScrollableMultichoice(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selection: u16 = 0u16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ListMenuGetCurrentItemArrayId(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
            &raw mut selection,
        );
        HideFrontierExchangeCornerItemIcon(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u16),
            selection,
        );
        ScrollableMultichoice_RemoveScrollArrows(taskId);
        DestroyListMenuTask(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        Free(
            ((&raw mut sScrollableMultichoice_ListMenuItem)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ClearStdWindowAndFrameToTransparent(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
            1u8,
        );
        FillWindowPixelBuffer(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
            0u8,
        );
        CopyWindowToVram(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
            2u8,
        );
        RemoveWindow(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
        );
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn Task_ScrollableMultichoice_WaitReturnToList(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 1i32 || !__matched {
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(1i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ScrollableMultichoice_ReturnToList));
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrollableMultichoice_TryReturnToList() {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_ScrollableMultichoice_WaitReturnToList));
        if ((taskId) as i32) == 255i32 {
            ScriptContext_Enable();
        } else {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ScrollableMultichoice_ReturnToList(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        LockPlayerFieldControls();
        ScrollableMultichoice_UpdateScrollArrows(taskId);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ScrollableMultichoice_ProcessInput));
    }
}
pub(crate) unsafe extern "C" fn ScrollableMultichoice_UpdateScrollArrows(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut template = crate::ffi::Align4([0u8; 16]);
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<16>>()
            .write_unaligned(
                (&raw const sScrollableMultichoice_ScrollArrowsTemplate_25)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<16>>()
                    .read_unaligned(),
            );
        if (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32)
            != ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            (((&raw mut template).cast::<u8>()).wrapping_add(1)).write(
                (((((crate::c::div_i32(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                    2i32,
                ))
                .wrapping_mul(8i32))
                .wrapping_add(12i32))
                .wrapping_add(
                    (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_sub(1i32))
                    .wrapping_mul(8i32),
                )) as u8),
            );
            (((&raw mut template).cast::<u8>()).wrapping_add(2)).write(8u8);
            (((&raw mut template).cast::<u8>()).wrapping_add(4)).write(
                (((((crate::c::div_i32(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                    2i32,
                ))
                .wrapping_mul(8i32))
                .wrapping_add(12i32))
                .wrapping_add(
                    (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_sub(1i32))
                    .wrapping_mul(8i32),
                )) as u8),
            );
            (((&raw mut template).cast::<u8>()).wrapping_add(5)).write(
                (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    .wrapping_mul(8i32))
                .wrapping_add(10i32)) as u8),
            );
            (((&raw mut template).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .write(0u16);
            (((&raw mut template).cast::<u8>())
                .wrapping_add(8)
                .cast::<u16>())
            .write(
                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    .wrapping_sub((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32)))
                    as u16),
            );
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
                ((AddScrollIndicatorArrowPair(
                    (&raw mut template).cast::<u8>(),
                    (&raw mut sScrollableMultichoice_ScrollOffset)
                        .cast::<u8>()
                        .cast::<u16>(),
                )) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ScrollableMultichoice_RemoveScrollArrows(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32)
            != ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            RemoveScrollIndicatorArrowPair(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as u8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowGlassWorkshopMenu() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattleTowerLinkPlayerGfx() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32)
                        == 0i32
                    {
                        VarSet((((16415i32).wrapping_sub(((i) as i32))) as u16), 0u16);
                    } else {
                        VarSet((((16415i32).wrapping_sub(((i) as i32))) as u16), 105u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowNatureGirlMessage() {
    unsafe {
        let mut nature: u8 = 0u8;
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) >= 6i32 {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
        }
        nature = GetNature(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
        ));
        ShowFieldMessage(
            ((((&raw const sNatureGirlMessages_24)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((nature) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateFrontierGambler(daysSince: u16) {
    unsafe {
        let mut daysSince = daysSince;
        let mut var: *mut u16 = GetVarPointer(16432u16);
        (var).write((((((var).read()) as i32).wrapping_add(((daysSince) as i32))) as u16));
        (var).write(((crate::c::rem_i32((((var).read()) as i32), 12i32)) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierGamblerLookingMessage() {
    unsafe {
        let mut challenge: u16 = VarGet(16432u16);
        ShowFieldMessage(
            ((((&raw const sFrontierGamblerLookingMessages_23)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((challenge) as i32) as isize))
            .read(),
        );
        VarSet(16433u16, challenge);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierGamblerGoMessage() {
    unsafe {
        ShowFieldMessage(
            ((((&raw const sFrontierGamblerGoMessages_22)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((VarGet(16433u16)) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FrontierGamblerSetWonOrLost(won: u8) {
    unsafe {
        let mut won = won;
        let mut battleMode: u16 = VarGet(16590u16);
        let mut challenge: u16 = VarGet(16433u16);
        let mut frontierFacilityId: u16 = VarGet(16591u16);
        if ((VarGet(16435u16)) as i32) == 1i32 {
            if ((((((&raw const sFrontierChallenges_21)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((challenge) as i32) as isize))
            .read()) as i32)
                == (((frontierFacilityId) as i32) << 8).wrapping_add(((battleMode) as i32))
            {
                if (won) != 0 {
                    VarSet(16435u16, 2u16);
                } else {
                    VarSet(16435u16, 3u16);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateBattlePointsWindow() {
    unsafe {
        let mut string = crate::ffi::Align4([0u8; 32]);
        let mut x: u32 = 0u32;
        StringCopy(
            ConvertIntToDecimalStringN(
                (&raw mut string).cast::<u8>(),
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2156)
                    .cast::<u16>())
                .read()) as i32),
                1i32,
                4u8,
            ),
            (&raw mut gText_BP).cast::<u8>(),
        );
        x = ((GetStringRightAlignXOffset(1i32, (&raw mut string).cast::<u8>(), 48i32)) as u32);
        AddTextPrinterParameterized(
            ((&raw mut sBattlePointsWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut string).cast::<u8>(),
            ((x) as u8),
            1u8,
            0u8,
            None,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowBattlePointsWindow() {
    unsafe {
        ((&raw mut sBattlePointsWindowId).cast::<u8>().cast::<u8>()).write(
            ((AddWindow(
                (&raw const sBattlePoints_WindowTemplate_20)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        SetStandardWindowBorderStyle(
            ((&raw mut sBattlePointsWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        UpdateBattlePointsWindow();
        CopyWindowToVram(
            ((&raw mut sBattlePointsWindowId).cast::<u8>().cast::<u8>()).read(),
            2u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseBattlePointsWindow() {
    unsafe {
        ClearStdWindowAndFrameToTransparent(
            ((&raw mut sBattlePointsWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
        );
        RemoveWindow(((&raw mut sBattlePointsWindowId).cast::<u8>().cast::<u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TakeFrontierBattlePoints() {
    unsafe {
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2156)
            .cast::<u16>())
        .read()) as i32)
            < ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
        {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2156)
                .cast::<u16>())
            .write(0u16);
        } else {
            let __p1 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2156)
                .cast::<u16>();
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_sub(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)))
                    as u16),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveFrontierBattlePoints() {
    unsafe {
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2156)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_add(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32))
            > 9999i32
        {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2156)
                .cast::<u16>())
            .write(9999u16);
        } else {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2156)
                .cast::<u16>())
            .write(
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2156)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)))
                    as u16),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBattlePoints() -> u16 {
    unsafe {
        return (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2156)
            .cast::<u16>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierExchangeCornerItemIconWindow() {
    unsafe {
        ((&raw mut sFrontierExchangeCorner_ItemIconWindowId)
            .cast::<u8>()
            .cast::<u8>())
        .write(
            ((AddWindow(
                (&raw const sFrontierExchangeCorner_ItemIconWindowTemplate_19)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        SetStandardWindowBorderStyle(
            ((&raw mut sFrontierExchangeCorner_ItemIconWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            0u8,
        );
        CopyWindowToVram(
            ((&raw mut sFrontierExchangeCorner_ItemIconWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            2u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseFrontierExchangeCornerItemIconWindow() {
    unsafe {
        ClearStdWindowAndFrameToTransparent(
            ((&raw mut sFrontierExchangeCorner_ItemIconWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            1u8,
        );
        RemoveWindow(
            ((&raw mut sFrontierExchangeCorner_ItemIconWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn FillFrontierExchangeCornerWindowAndItemIcon(
    menu: u16,
    selection: u16,
) {
    unsafe {
        let mut menu = menu;
        let mut selection = selection;
        if (((menu) as i32) >= 3i32) && (((menu) as i32) <= 6i32) {
            FillWindowPixelRect(0u8, 17u8, 0u16, 0u16, 216u16, 32u16);
            'l1: {
                let __sw1 = ((menu) as i32);
                if __sw1 == 3i32 {
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        ((((&raw const sFrontierExchangeCorner_Decor1Descriptions_18)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((selection) as i32) as isize))
                        .read(),
                        0u8,
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                    if ((((((&raw const sFrontierExchangeCorner_Decor1_17)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((selection) as i32) as isize))
                    .read()) as i32)
                        == 65535i32
                    {
                        ShowFrontierExchangeCornerItemIcon(
                            ((((&raw const sFrontierExchangeCorner_Decor1_17)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((selection) as i32) as isize))
                            .read(),
                        );
                    } else {
                        FreeSpriteTilesByTag(5500u16);
                        FreeSpritePaletteByTag(5500u16);
                        ((&raw mut sScrollableMultichoice_ItemSpriteId)
                            .cast::<u8>()
                            .cast::<u8>())
                        .write(AddDecorationIconObject(
                            ((((((&raw const sFrontierExchangeCorner_Decor1_17)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((selection) as i32) as isize))
                            .read()) as u8),
                            33i16,
                            88i16,
                            0u8,
                            5500u16,
                            5500u16,
                        ));
                    }
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        ((((&raw const sFrontierExchangeCorner_Decor2Descriptions_16)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((selection) as i32) as isize))
                        .read(),
                        0u8,
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                    if ((((((&raw const sFrontierExchangeCorner_Decor2_15)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((selection) as i32) as isize))
                    .read()) as i32)
                        == 65535i32
                    {
                        ShowFrontierExchangeCornerItemIcon(
                            ((((&raw const sFrontierExchangeCorner_Decor2_15)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((selection) as i32) as isize))
                            .read(),
                        );
                    } else {
                        FreeSpriteTilesByTag(5500u16);
                        FreeSpritePaletteByTag(5500u16);
                        ((&raw mut sScrollableMultichoice_ItemSpriteId)
                            .cast::<u8>()
                            .cast::<u8>())
                        .write(AddDecorationIconObject(
                            ((((((&raw const sFrontierExchangeCorner_Decor2_15)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((selection) as i32) as isize))
                            .read()) as u8),
                            33i16,
                            88i16,
                            0u8,
                            5500u16,
                            5500u16,
                        ));
                    }
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        ((((&raw const sFrontierExchangeCorner_VitaminsDescriptions_14)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((selection) as i32) as isize))
                        .read(),
                        0u8,
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                    ShowFrontierExchangeCornerItemIcon(
                        ((((&raw const sFrontierExchangeCorner_Vitamins_13)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((selection) as i32) as isize))
                        .read(),
                    );
                    break 'l1;
                }
                if __sw1 == 6i32 {
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        ((((&raw const sFrontierExchangeCorner_HoldItemsDescriptions_12)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((selection) as i32) as isize))
                        .read(),
                        0u8,
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                    ShowFrontierExchangeCornerItemIcon(
                        ((((&raw const sFrontierExchangeCorner_HoldItems_11)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((selection) as i32) as isize))
                        .read(),
                    );
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowFrontierExchangeCornerItemIcon(item: u16) {
    unsafe {
        let mut item = item;
        FreeSpriteTilesByTag(5500u16);
        FreeSpritePaletteByTag(5500u16);
        ((&raw mut sScrollableMultichoice_ItemSpriteId)
            .cast::<u8>()
            .cast::<u8>())
        .write(AddItemIconSprite(5500u16, 5500u16, item));
        if ((((&raw mut sScrollableMultichoice_ItemSpriteId)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            != 64i32
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sScrollableMultichoice_ItemSpriteId)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                2,
                2,
                (0u16) as i32,
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sScrollableMultichoice_ItemSpriteId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .write(36i16);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sScrollableMultichoice_ItemSpriteId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .write(92i16);
        }
    }
}
pub(crate) unsafe extern "C" fn HideFrontierExchangeCornerItemIcon(menu: u16, unused: u16) {
    unsafe {
        let mut menu = menu;
        let mut unused = unused;
        if ((((&raw mut sScrollableMultichoice_ItemSpriteId)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            != 64i32
        {
            'l1: {
                let __sw1 = ((menu) as i32);
                if __sw1 == 3i32 || __sw1 == 4i32 || __sw1 == 5i32 || __sw1 == 6i32 {
                    DestroySpriteAndFreeResources(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((&raw mut sScrollableMultichoice_ItemSpriteId)
                                .cast::<u8>()
                                .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    break 'l1;
                }
            }
            ((&raw mut sScrollableMultichoice_ItemSpriteId)
                .cast::<u8>()
                .cast::<u8>())
            .write(64u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferBattleFrontierTutorMoveName() {
    unsafe {
        if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) != 0i32 {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                    ((((((&raw const sBattleFrontier_TutorMoves2)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 13,
                ))
                .cast::<u8>(),
            );
        } else {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                    ((((((&raw const sBattleFrontier_TutorMoves1)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 13,
                ))
                .cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ShowBattleFrontierTutorWindow(menu: u8, selection: u16) {
    unsafe {
        let mut menu = menu;
        let mut selection = selection;
        if (((menu) as i32) == 9i32) || (((menu) as i32) == 10i32) {
            if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) == 0i32 {
                ((&raw mut sTutorMoveAndElevatorWindowId)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(
                    ((AddWindow(
                        (&raw const sBattleFrontierTutor_WindowTemplate_10)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as u8),
                );
                SetStandardWindowBorderStyle(
                    ((&raw mut sTutorMoveAndElevatorWindowId)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read(),
                    0u8,
                );
            }
            ShowBattleFrontierTutorMoveDescription(menu, selection);
        }
    }
}
pub(crate) unsafe extern "C" fn ShowBattleFrontierTutorMoveDescription(menu: u8, selection: u16) {
    unsafe {
        let mut menu = menu;
        let mut selection = selection;
        if (((menu) as i32) == 9i32) || (((menu) as i32) == 10i32) {
            FillWindowPixelRect(
                ((&raw mut sTutorMoveAndElevatorWindowId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read(),
                17u8,
                0u16,
                0u16,
                96u16,
                48u16,
            );
            if ((menu) as i32) == 10i32 {
                AddTextPrinterParameterized(
                    ((&raw mut sTutorMoveAndElevatorWindowId)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read(),
                    1u8,
                    ((((&raw const sBattleFrontier_TutorMoveDescriptions2_9)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((selection) as i32) as isize))
                    .read(),
                    0u8,
                    1u8,
                    0u8,
                    None,
                );
            } else {
                AddTextPrinterParameterized(
                    ((&raw mut sTutorMoveAndElevatorWindowId)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read(),
                    1u8,
                    ((((&raw const sBattleFrontier_TutorMoveDescriptions1_8)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((selection) as i32) as isize))
                    .read(),
                    0u8,
                    1u8,
                    0u8,
                    None,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseBattleFrontierTutorWindow() {
    unsafe {
        ClearStdWindowAndFrameToTransparent(
            ((&raw mut sTutorMoveAndElevatorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            1u8,
        );
        RemoveWindow(
            ((&raw mut sTutorMoveAndElevatorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrollableMultichoice_RedrawPersistentMenu() {
    unsafe {
        let mut scrollOffset: u16 = 0u16;
        let mut selectedRow: u16 = 0u16;
        let mut i: u8 = 0u8;
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_ScrollableMultichoice_WaitReturnToList));
        if ((taskId) as i32) != 255i32 {
            let mut task: *mut u8 =
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
            ListMenuGetScrollAndRow(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
                &raw mut scrollOffset,
                &raw mut selectedRow,
            );
            SetStandardWindowBorderStyle(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
                0u8,
            );
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        AddTextPrinterParameterized5(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                                as u8),
                            1u8,
                            ((((((&raw const sScrollableMultichoiceOptions)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                    as isize
                                    * 64,
                            ))
                            .cast::<*mut u8>())
                            .wrapping_offset(
                                (((scrollOffset) as i32).wrapping_add(((i) as i32))) as isize,
                            ))
                            .read(),
                            10u8,
                            ((((i) as i32).wrapping_mul(16i32)) as u8),
                            255u8,
                            None,
                            0u8,
                            0u8,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            AddTextPrinterParameterized(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
                1u8,
                (&raw mut gText_SelectorArrow).cast::<u8>(),
                0u8,
                ((((selectedRow) as i32).wrapping_mul(16i32)) as u8),
                255u8,
                None,
            );
            PutWindowTilemap(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
            );
            CopyWindowToVram(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
                3u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleFrontierTutorMoveIndex() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut moveTutor: u16 = 0u16;
        let mut moveIndex: u16 = 0u16;
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(0u16);
        moveTutor = VarGet(16398u16);
        moveIndex = VarGet(16397u16);
        if ((moveTutor) as i32) != 0i32 {
            i = 0u8;
            'l1: loop {
                'l2: {
                    if ((((((&raw mut gTutorMoves).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((((((&raw const sBattleFrontier_TutorMoves2)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((moveIndex) as i32) as isize))
                        .read()) as i32)
                    {
                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(((i) as u16));
                        break 'l1;
                    }
                    i = (i).wrapping_add(1);
                }
                if !(((i) as i32) < 30i32) {
                    break 'l1;
                }
            }
        } else {
            i = 0u8;
            'l3: loop {
                'l4: {
                    if ((((((&raw mut gTutorMoves).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((((((&raw const sBattleFrontier_TutorMoves1)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((moveIndex) as i32) as isize))
                        .read()) as i32)
                    {
                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(((i) as u16));
                        break 'l3;
                    }
                    i = (i).wrapping_add(1);
                }
                if !(((i) as i32) < 30i32) {
                    break 'l3;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrollableMultichoice_ClosePersistentMenu() {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_ScrollableMultichoice_WaitReturnToList));
        if ((taskId) as i32) != 255i32 {
            let mut task: *mut u8 =
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
            DestroyListMenuTask(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
            Free(
                ((&raw mut sScrollableMultichoice_ListMenuItem)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ClearStdWindowAndFrameToTransparent(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
                1u8,
            );
            FillWindowPixelBuffer(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
                0u8,
            );
            ClearWindowTilemap(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
            );
            CopyWindowToVram(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
                2u8,
            );
            RemoveWindow(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
            );
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoDeoxysRockInteraction() {
    unsafe {
        CreateTask(Some(Task_DeoxysRockInteraction), 8u8);
    }
}
pub(crate) unsafe extern "C" fn Task_DeoxysRockInteraction(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((FlagGet(2260u16)) as i32) == 1i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(3u16);
            ScriptContext_Enable();
            DestroyTask(taskId);
        } else {
            let mut rockLevel: u16 = VarGet(16437u16);
            let mut stepCount: u16 = VarGet(16436u16);
            VarSet(16436u16, 0u16);
            if (((rockLevel) as i32) != 0i32)
                && (((((((&raw const sStoneMaxStepCounts_7).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((((rockLevel) as i32).wrapping_sub(1i32)) as isize))
                .read()) as i32)
                    < ((stepCount) as i32))
            {
                ChangeDeoxysRockLevel(0u8);
                VarSet(16437u16, 0u16);
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                DestroyTask(taskId);
            } else {
                if ((rockLevel) as i32) == 10i32 {
                    FlagSet(2260u16);
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
                    ScriptContext_Enable();
                    DestroyTask(taskId);
                } else {
                    rockLevel = (rockLevel).wrapping_add(1);
                    ChangeDeoxysRockLevel(((rockLevel) as u8));
                    VarSet(16437u16, rockLevel);
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                    DestroyTask(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ChangeDeoxysRockLevel(rockLevel: u8) {
    unsafe {
        let mut rockLevel = rockLevel;
        let mut objectEventId: u8 = 0u8;
        LoadPalette(
            (((((&raw const sDeoxysRockPalettes).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((rockLevel) as i32) as isize * 32))
            .cast::<u16>())
            .cast::<u8>(),
            416u16,
            8u16,
        );
        TryGetObjectEventIdByLocalIdAndMap(
            1u8,
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
            &raw mut objectEventId,
        );
        if ((rockLevel) as i32) == 0i32 {
            PlaySE(196u16);
        } else {
            PlaySE(260u16);
        }
        CreateTask(Some(WaitForDeoxysRockMovement), 8u8);
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).write(1i32);
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
            .write(58i32);
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
            .write(26i32);
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(3))
            .write(
                (((((((&raw const sDeoxysRockCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((rockLevel) as i32) as isize * 2))
                .cast::<u8>())
                .read()) as i32),
            );
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(4))
            .write(
                ((((((((&raw const sDeoxysRockCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((rockLevel) as i32) as isize * 2))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32),
            );
        if ((rockLevel) as i32) == 0i32 {
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(5))
                .write(60i32);
        } else {
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(5))
                .write(5i32);
        }
        FieldEffectStart(66u8);
        SetObjEventTemplateCoords(
            1u8,
            (((((((&raw const sDeoxysRockCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((rockLevel) as i32) as isize * 2))
            .cast::<u8>())
            .read()) as i16),
            ((((((((&raw const sDeoxysRockCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((rockLevel) as i32) as isize * 2))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn WaitForDeoxysRockMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((FieldEffectActiveListContains(66u8)) as i32) == 0i32 {
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementBirthIslandRockStepCount() {
    unsafe {
        let mut stepCount: u16 = VarGet(16436u16);
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .wrapping_add(1)
            .cast::<i8>())
        .read()) as i32)
            == 58i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 26i32)
        {
            if (({
                let __t1 = (stepCount).wrapping_add(1);
                stepCount = __t1;
                __t1
            }) as i32)
                > 99i32
            {
                VarSet(16436u16, 0u16);
            } else {
                VarSet(16436u16, stepCount);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDeoxysRockPalette() {
    unsafe {
        LoadPalette(
            (((((&raw const sDeoxysRockPalettes).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((VarGet(16437u16)) as u8) as i32) as isize * 32))
            .cast::<u16>())
            .cast::<u8>(),
            416u16,
            8u16,
        );
        BlendPalettes(67108864u32, 16u8, 0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPCBoxToSendMon(boxId: u8) {
    unsafe {
        let mut boxId = boxId;
        ((&raw mut sPCBoxToSendMon).cast::<u8>().cast::<u8>()).write(boxId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPCBoxToSendMon() -> u16 {
    unsafe {
        return ((((&raw mut sPCBoxToSendMon).cast::<u8>().cast::<u8>()).read()) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldShowBoxWasFullMessage() -> u8 {
    unsafe {
        if !((FlagGet(2263u16)) != 0) {
            if ((StorageGetCurrentBox()) as i32) != ((VarGet(16438u16)) as i32) {
                FlagSet(2263u16);
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsDestinationBoxFull() -> u8 {
    unsafe {
        let mut r#box: i32 = 0i32;
        let mut i: i32 = 0i32;
        SetPCBoxToSendMon(((VarGet(16438u16)) as u8));
        r#box = ((StorageGetCurrentBox()) as i32);
        'l1: loop {
            'l2: {
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 30i32) {
                            break 'l3;
                        }
                        'l4: {
                            if GetBoxMonData3(
                                GetBoxedMonPtr(((r#box) as u8), ((i) as u8)),
                                11i32,
                                core::ptr::null_mut(),
                            ) == 0u32
                            {
                                if ((GetPCBoxToSendMon()) as i32) != r#box {
                                    FlagClear(2263u16);
                                }
                                VarSet(16438u16, ((r#box) as u16));
                                return ShouldShowBoxWasFullMessage();
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if {
                    let __t1 = (r#box).wrapping_add(1);
                    r#box = __t1;
                    __t1
                } == 14i32
                {
                    r#box = 0i32;
                }
            }
            if !(r#box != ((StorageGetCurrentBox()) as i32)) {
                break 'l1;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateAbnormalWeatherEvent() {
    unsafe {
        let mut randomValue: u16 = Random();
        VarSet(16440u16, 0u16);
        if ((FlagGet(446u16)) as i32) == 1i32 {
            VarSet(
                16439u16,
                (((crate::c::rem_i32(((randomValue) as i32), 8i32)).wrapping_add(1i32)) as u16),
            );
        } else {
            if ((FlagGet(447u16)) as i32) == 1i32 {
                VarSet(
                    16439u16,
                    (((crate::c::rem_i32(((randomValue) as i32), 8i32)).wrapping_add(9i32)) as u16),
                );
            } else {
                if (((randomValue) as i32) & 1i32) == 0i32 {
                    randomValue = Random();
                    VarSet(
                        16439u16,
                        (((crate::c::rem_i32(((randomValue) as i32), 8i32)).wrapping_add(1i32))
                            as u16),
                    );
                } else {
                    randomValue = Random();
                    VarSet(
                        16439u16,
                        (((crate::c::rem_i32(((randomValue) as i32), 8i32)).wrapping_add(9i32))
                            as u16),
                    );
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAbnormalWeatherMapNameAndType() -> u32 {
    unsafe {
        let mut abnormalWeather: u16 = VarGet(16439u16);
        GetMapName(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((((&raw const sAbnormalWeatherMapNumbers_6)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((((abnormalWeather) as i32).wrapping_sub(1i32)) as isize))
            .read()) as u16),
            0u16,
        );
        if ((abnormalWeather) as i32) < 9i32 {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AbnormalWeatherHasExpired() -> u8 {
    unsafe {
        let mut steps: u16 = VarGet(16440u16);
        let mut abnormalWeather: u16 = VarGet(16439u16);
        if ((abnormalWeather) as i32) == 0i32 {
            return 0u8;
        }
        if (({
            let __t1 = (steps).wrapping_add(1);
            steps = __t1;
            __t1
        }) as i32)
            > 999i32
        {
            VarSet(16440u16, 0u16);
            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 24i32
            {
                'l1: {
                    let __sw2 = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                    .read()) as i32);
                    let __matched = __sw2 == 101i32
                        || __sw2 == 102i32
                        || __sw2 == 103i32
                        || __sw2 == 104i32
                        || __sw2 == 105i32;
                    if __sw2 == 101i32
                        || __sw2 == 102i32
                        || __sw2 == 103i32
                        || __sw2 == 104i32
                        || __sw2 == 105i32
                    {
                        VarSet(16441u16, 1u16);
                        return 0u8;
                    }
                    if !__matched {
                        break 'l1;
                    }
                }
            }
            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 0i32
            {
                'l2: {
                    let __sw3 = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                    .read()) as i32);
                    let __matched =
                        __sw3 == 52i32 || __sw3 == 54i32 || __sw3 == 55i32 || __sw3 == 56i32;
                    if __sw3 == 52i32 || __sw3 == 54i32 || __sw3 == 55i32 || __sw3 == 56i32 {
                        VarSet(16441u16, 1u16);
                        return 0u8;
                    }
                    if !__matched {
                        break 'l2;
                    }
                }
            }
            if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == ((((((&raw const sAbnormalWeatherMapNumbers_5)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((((abnormalWeather) as i32).wrapping_sub(1i32)) as isize))
                .read()) as i32))
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as i32)
                    == 0i32)
            {
                return 1u8;
            } else {
                VarSet(16439u16, 0u16);
                return 0u8;
            }
        } else {
            VarSet(16440u16, steps);
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unused_SetWeatherSunny() {
    unsafe {
        SetCurrentAndNextWeather(2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMartEmployeeObjectEventId() -> u32 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(36u32, 3u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<i8>())
                    .read()) as i32)
                        == (((((((&raw const sPokeMarts_4).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 3))
                        .cast::<u8>())
                        .read()) as i32)
                    {
                        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .wrapping_add(1)
                        .cast::<i8>())
                        .read()) as i32)
                            == ((((((((&raw const sPokeMarts_4).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 3))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                        {
                            return ((((((((&raw const sPokeMarts_4).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 3))
                            .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as u32);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTrainerRegistered() -> u32 {
    unsafe {
        let mut index: i32 = GetRematchIdxByTrainerIdx(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32),
        );
        if index >= 0i32 {
            if ((FlagGet((((348i32).wrapping_add(index)) as u16))) as i32) == 1i32 {
                return 1u32;
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDistributeEonTicket() -> u32 {
    unsafe {
        if !((VarGet(16447u16)) != 0) {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTowerReconnectLink() {
    unsafe {
        ((&raw mut sBattleTowerMultiBattleTypeFlags)
            .cast::<u8>()
            .cast::<u32>())
        .write(((&raw mut gBattleTypeFlags).cast::<u32>()).read());
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
        if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
            CreateTask(Some(Task_ReconnectWithLinkPlayers), 5u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkRetireStatusWithBattleTowerPartner() {
    unsafe {
        CreateTask(Some(Task_LinkRetireStatusWithBattleTowerPartner), 5u8);
    }
}
pub(crate) unsafe extern "C" fn Task_LinkRetireStatusWithBattleTowerPartner(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((FuncIsActiveTask(Some(Task_ReconnectWithLinkPlayers))) != 0) {
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((IsLinkTaskFinished()) as i32) == 1i32 {
                    if ((GetMultiplayerId()) as i32) == 0i32 {
                        let __p3 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    } else {
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).cast::<u8>(),
                            2u16,
                        );
                        let __p4 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((GetBlockReceivedStatus()) as i32) & 2i32) != 0 {
                    if ((GetMultiplayerId()) as i32) == 0i32 {
                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(256))
                                .cast::<u16>())
                            .read(),
                        );
                        ResetBlockReceivedFlag(1u8);
                        if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32)
                            && (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                                == 1i32)
                        {
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                        } else {
                            if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                == 0i32)
                                && (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                                    == 1i32)
                            {
                                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
                            } else {
                                if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                    == 1i32)
                                    && (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read())
                                        as i32)
                                        == 0i32)
                                {
                                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(3u16);
                                } else {
                                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                                }
                            }
                        }
                    }
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((IsLinkTaskFinished()) as i32) == 1i32 {
                    if ((GetMultiplayerId()) as i32) != 0i32 {
                        let __p6 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    } else {
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).cast::<u8>(),
                            2u16,
                        );
                        let __p7 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((GetBlockReceivedStatus()) as i32) & 1i32) != 0 {
                    if ((GetMultiplayerId()) as i32) != 0i32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                            (((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).read(),
                        );
                        ResetBlockReceivedFlag(0u8);
                        let __p8 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    } else {
                        let __p9 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 2i32 {
                        ShowFieldAutoScrollMessage(
                            (&raw mut gText_YourPartnerHasRetired).cast::<u8>(),
                        );
                    }
                } else {
                    if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 3i32 {
                        ShowFieldAutoScrollMessage(
                            (&raw mut gText_YourPartnerHasRetired).cast::<u8>(),
                        );
                    }
                }
                let __p10 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((IsTextPrinterActive(0u8)) != 0) {
                    let __p11 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((IsLinkTaskFinished()) as i32) == 1i32 {
                    SetLinkStandbyCallback();
                    let __p12 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if ((IsLinkTaskFinished()) as i32) == 1i32 {
                    let __p13 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 0i32 {
                    SetCloseLinkCallback();
                }
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(
                    ((&raw mut sBattleTowerMultiBattleTypeFlags)
                        .cast::<u8>()
                        .cast::<u32>())
                    .read(),
                );
                ScriptContext_Enable();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_DoRayquazaScene() {
    unsafe {
        if !((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) != 0) {
            DoRayquazaScene(0u8, 1u8, Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        } else {
            DoRayquazaScene(1u8, 0u8, Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoopWingFlapSE() {
    unsafe {
        CreateTask(Some(Task_LoopWingFlapSE), 8u8);
        PlaySE(157u16);
    }
}
pub(crate) unsafe extern "C" fn Task_LoopWingFlapSE(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((data).wrapping_offset(1)).read()) as i32)
            == ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
        {
            (data).write(((data).read()).wrapping_add(1));
            ((data).wrapping_offset(1)).write(0i16);
            PlaySE(157u16);
        }
        if (((data).read()) as i32)
            == ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
        {
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseBattlePikeCurtain() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_CloseBattlePikeCurtain), 8u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(4i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(4i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(4i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn Task_CloseBattlePikeCurtain(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(((((data).wrapping_offset(3)).read()) as i32) as isize);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        if ((((data).wrapping_offset(((((data).wrapping_offset(3)).read()) as i32) as isize))
            .read()) as i32)
            == 0i32
        {
            {
                y = 0u8;
                'l1: loop {
                    if !(((y) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        {
                            x = 0u8;
                            'l3: loop {
                                if !(((x) as i32) < 3i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    MapGridSetMetatileIdAt(
                                        ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                            .read())
                                        .cast::<i16>())
                                        .read())
                                            as i32)
                                            .wrapping_add(((x) as i32)))
                                        .wrapping_add(7i32))
                                        .wrapping_sub(1i32),
                                        ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(2)
                                        .cast::<i16>())
                                        .read())
                                            as i32)
                                            .wrapping_add(((y) as i32)))
                                        .wrapping_add(7i32))
                                        .wrapping_sub(3i32),
                                        ((((((x) as i32).wrapping_add(513i32))
                                            .wrapping_add(((y) as i32).wrapping_mul(8i32)))
                                        .wrapping_add(
                                            (((((data).wrapping_offset(3)).read()) as i32)
                                                .wrapping_mul(4i32))
                                            .wrapping_mul(8i32),
                                        )) as u16),
                                    );
                                }
                                x = (x).wrapping_add(1);
                            }
                        }
                    }
                    y = (y).wrapping_add(1);
                }
            }
            DrawWholeMapView();
            let __p2 = (data).wrapping_offset(3);
            (__p2).write(((__p2).read()).wrapping_add(1));
            if ((((data).wrapping_offset(3)).read()) as i32) == 3i32 {
                DestroyTask(taskId);
                ScriptContext_Enable();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlePyramidHint() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            ((crate::c::div_i32(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32),
                7i32,
            )) as u16),
        );
        let __p1 = (&raw mut gSpecialVar_Result).cast::<u16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_sub(
                (crate::c::div_i32(
                    ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32),
                    20i32,
                ))
                .wrapping_mul(20i32),
            )) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetHealLocationFromDewford() {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(28))
            .cast::<i8>())
        .read()) as i32)
            == 0i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(28))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 11i32)
        {
            SetLastHealLocationWarp(3u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InPokemonCenter() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut map: u16 = ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(4))
        .cast::<i8>())
        .read()) as i32)
            << 8)
            .wrapping_add(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32),
            )) as u16);
        {
            i = 0i32;
            'l1: loop {
                if !(((((((&raw const sPokemonCenters_3)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((i) as isize))
                .read()) as i32)
                    != 65535i32)
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sPokemonCenters_3)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((map) as i32)
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
pub unsafe extern "C" fn ResetFanClub() {
    unsafe {
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(65))
        .write(0u16);
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(66))
        .write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryLoseFansFromPlayTimeAfterLinkBattle() {
    unsafe {
        if (DidPlayerGetFirstFans()) != 0 {
            TryLoseFansFromPlayTime();
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
                .cast::<u16>())
            .wrapping_offset(66))
            .write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(14)
                    .cast::<u16>())
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTrainerFanClubGameClear() {
    unsafe {
        if !(((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(65))
        .read()) as i32)
            >> 7)
            & 1i32)
            != 0)
        {
            SetPlayerGotFirstFans();
            SetInitialFansOfPlayer();
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
                .cast::<u16>())
            .wrapping_offset(66))
            .write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(14)
                    .cast::<u16>())
                .read(),
            );
            FlagClear(789u16);
            FlagClear(790u16);
            FlagClear(791u16);
            FlagClear(792u16);
            FlagClear(730u16);
            VarSet(16533u16, 1u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryGainNewFanFromCounter(incrementId: u8) -> u8 {
    unsafe {
        let mut incrementId = incrementId;
        if ((VarGet(16533u16)) as i32) == 2i32 {
            if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
                .cast::<u16>())
            .wrapping_offset(65))
            .read()) as i32)
                & 127i32)
                .wrapping_add(
                    ((((((&raw const sCounterIncrements_2).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((incrementId) as i32) as isize))
                    .read()) as i32),
                )
                > 19i32
            {
                if ((GetNumFansOfPlayerInTrainerFanClub()) as i32) < 3i32 {
                    PlayerGainRandomTrainerFan();
                    let __p1 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(5020))
                    .cast::<u16>())
                    .wrapping_offset(65);
                    (__p1).write((((((__p1).read()) as i32) & (-128i32)) as u16));
                } else {
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
                        .cast::<u16>())
                    .wrapping_offset(65))
                    .write(
                        (((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(5020))
                        .cast::<u16>())
                        .wrapping_offset(65))
                        .read()) as i32)
                            & (-128i32))
                            | 20i32) as u16),
                    );
                }
            } else {
                let __p2 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(5020))
                .cast::<u16>())
                .wrapping_offset(65);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((((((&raw const sCounterIncrements_2).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((incrementId) as i32) as isize))
                        .read()) as i32),
                    )) as u16),
                );
            }
        }
        return ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(65))
        .read()) as i32)
            & 127i32) as u8);
    }
}
pub(crate) unsafe extern "C" fn PlayerGainRandomTrainerFan() -> u16 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut idx: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if !((crate::c::shr_i32(
                        ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(5020))
                        .cast::<u16>())
                        .wrapping_offset(65))
                        .read()) as i32),
                        ((((((&raw const sFanClubMemberIds_1).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u32),
                    ) & 1i32)
                        != 0)
                    {
                        idx = i;
                        if (((Random()) as i32) & 1i32) != 0 {
                            let __p1 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(5020))
                            .cast::<u16>())
                            .wrapping_offset(65);
                            (__p1).write(
                                (((((__p1).read()) as i32)
                                    | crate::c::shl_i32(
                                        1i32,
                                        ((((((&raw const sFanClubMemberIds_1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((idx) as i32) as isize))
                                        .read()) as u32),
                                    )) as u16),
                            );
                            return ((idx) as u16);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p2 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(65);
        (__p2).write(
            (((((__p2).read()) as i32)
                | crate::c::shl_i32(
                    1i32,
                    ((((((&raw const sFanClubMemberIds_1).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((idx) as i32) as isize))
                    .read()) as u32),
                )) as u16),
        );
        return ((idx) as u16);
    }
}
pub(crate) unsafe extern "C" fn PlayerLoseRandomTrainerFan() -> u16 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut idx: u8 = 0u8;
        if ((GetNumFansOfPlayerInTrainerFanClub()) as i32) == 1i32 {
            return 0u16;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(
                        ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(5020))
                        .cast::<u16>())
                        .wrapping_offset(65))
                        .read()) as i32),
                        ((((((&raw const sFanClubMemberIds_0).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u32),
                    ) & 1i32)
                        != 0
                    {
                        idx = i;
                        if (((Random()) as i32) & 1i32) != 0 {
                            let __p1 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(5020))
                            .cast::<u16>())
                            .wrapping_offset(65);
                            (__p1).write(
                                (((((__p1).read()) as i32)
                                    ^ crate::c::shl_i32(
                                        1i32,
                                        ((((((&raw const sFanClubMemberIds_0)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((idx) as i32) as isize))
                                        .read()) as u32),
                                    )) as u16),
                            );
                            return ((idx) as u16);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (crate::c::shr_i32(
            ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
                .cast::<u16>())
            .wrapping_offset(65))
            .read()) as i32),
            ((((((&raw const sFanClubMemberIds_0).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((idx) as i32) as isize))
            .read()) as u32),
        ) & 1i32)
            != 0
        {
            let __p2 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(5020))
            .cast::<u16>())
            .wrapping_offset(65);
            (__p2).write(
                (((((__p2).read()) as i32)
                    ^ crate::c::shl_i32(
                        1i32,
                        ((((((&raw const sFanClubMemberIds_0).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((idx) as i32) as isize))
                        .read()) as u32),
                    )) as u16),
            );
        }
        return ((idx) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumFansOfPlayerInTrainerFanClub() -> u16 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numFans: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(
                        ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(5020))
                        .cast::<u16>())
                        .wrapping_offset(65))
                        .read()) as i32),
                        ((((i) as i32).wrapping_add(8i32)) as u32),
                    ) & 1i32)
                        != 0
                    {
                        numFans = (numFans).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((numFans) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryLoseFansFromPlayTime() {
    unsafe {
        let mut i: u8 = 0u8;
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .read()) as i32)
            < 999i32
        {
            'l1: loop {
                if !((1i32) != 0) {
                    break 'l1;
                }
                if ((GetNumFansOfPlayerInTrainerFanClub()) as i32) < 5i32 {
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
                        .cast::<u16>())
                    .wrapping_offset(66))
                    .write(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(14)
                            .cast::<u16>())
                        .read(),
                    );
                    break 'l1;
                } else {
                    if ((i) as i32) == 8i32 {
                        break 'l1;
                    } else {
                        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(14)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_sub(
                                ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(5020))
                                .cast::<u16>())
                                .wrapping_offset(66))
                                .read()) as i32),
                            )
                            < 12i32
                        {
                            return;
                        }
                    }
                }
                PlayerLoseRandomTrainerFan();
                let __p1 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(5020))
                .cast::<u16>())
                .wrapping_offset(66);
                (__p1).write((((((__p1).read()) as i32).wrapping_add(12i32)) as u16));
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFanClubMemberFanOfPlayer() -> u8 {
    unsafe {
        return ((crate::c::shr_i32(
            ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
                .cast::<u16>())
            .wrapping_offset(65))
            .read()) as i32),
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u32),
        ) & 1i32) as u8);
    }
}
pub(crate) unsafe extern "C" fn SetInitialFansOfPlayer() {
    unsafe {
        let __p1 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(65);
        (__p1).write((((((__p1).read()) as i32) | 8192i32) as u16));
        let __p2 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(65);
        (__p2).write((((((__p2).read()) as i32) | 256i32) as u16));
        let __p3 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(65);
        (__p3).write((((((__p3).read()) as i32) | 1024i32) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferFanClubTrainerName() {
    unsafe {
        let mut whichLinkTrainer: u8 = 0u8;
        let mut whichNPCTrainer: u8 = 0u8;
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32);
            if __sw1 == 8i32 {
                break 'l1;
            }
            if __sw1 == 9i32 {
                break 'l1;
            }
            if __sw1 == 10i32 {
                whichLinkTrainer = 0u8;
                whichNPCTrainer = 3u8;
                break 'l1;
            }
            if __sw1 == 11i32 {
                whichLinkTrainer = 0u8;
                whichNPCTrainer = 1u8;
                break 'l1;
            }
            if __sw1 == 12i32 {
                whichLinkTrainer = 1u8;
                whichNPCTrainer = 0u8;
                break 'l1;
            }
            if __sw1 == 13i32 {
                whichLinkTrainer = 0u8;
                whichNPCTrainer = 4u8;
                break 'l1;
            }
            if __sw1 == 14i32 {
                whichLinkTrainer = 1u8;
                whichNPCTrainer = 5u8;
                break 'l1;
            }
            if __sw1 == 15i32 {
                break 'l1;
            }
        }
        BufferFanClubTrainerName_(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12624),
            whichLinkTrainer,
            whichNPCTrainer,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferFanClubTrainerName_(
    linkRecords: *mut u8,
    whichLinkTrainer: u8,
    whichNPCTrainer: u8,
) {
    unsafe {
        let mut linkRecords = linkRecords;
        let mut whichLinkTrainer = whichLinkTrainer;
        let mut whichNPCTrainer = whichNPCTrainer;
        let mut record: *mut u8 =
            ((linkRecords).cast::<u8>()).wrapping_offset(((whichLinkTrainer) as i32) as isize * 16);
        if ((((record).cast::<u8>()).read()) as i32) == 255i32 {
            'l1: {
                let __sw1 = ((whichNPCTrainer) as i32);
                let __matched = __sw1 == 0i32
                    || __sw1 == 1i32
                    || __sw1 == 2i32
                    || __sw1 == 3i32
                    || __sw1 == 4i32
                    || __sw1 == 5i32;
                if __sw1 == 0i32 {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Wallace).cast::<u8>(),
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Steven).cast::<u8>(),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Brawly).cast::<u8>(),
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Winona).cast::<u8>(),
                    );
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Phoebe).cast::<u8>(),
                    );
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Glacia).cast::<u8>(),
                    );
                    break 'l1;
                }
                if !__matched {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Wallace).cast::<u8>(),
                    );
                    break 'l1;
                }
            }
        } else {
            StringCopyN(
                (&raw mut gStringVar1).cast::<u8>(),
                (record).cast::<u8>(),
                7u8,
            );
            (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(7)).write(255u8);
            ConvertInternationalString(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((linkRecords).wrapping_add(80)).cast::<u8>())
                    .wrapping_offset(((whichLinkTrainer) as i32) as isize))
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTrainerFansAfterLinkBattle() {
    unsafe {
        if ((VarGet(16533u16)) as i32) == 2i32 {
            TryLoseFansFromPlayTimeAfterLinkBattle();
            if ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 1i32 {
                PlayerGainRandomTrainerFan();
            } else {
                PlayerLoseRandomTrainerFan();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DidPlayerGetFirstFans() -> u8 {
    unsafe {
        return (((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(65))
        .read()) as i32)
            >> 7)
            & 1i32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerGotFirstFans() {
    unsafe {
        let __p1 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(65);
        (__p1).write((((((__p1).read()) as i32) | 128i32) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_TryGainNewFanFromCounter() -> u8 {
    unsafe {
        return TryGainNewFanFromCounter(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
        );
    }
}
