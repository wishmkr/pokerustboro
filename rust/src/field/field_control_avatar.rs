//! Translated from `src/field_control_avatar.c` by tools/rustport/c2rs.py, then reviewed.
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

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWildEncounterImmunitySteps: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPrevMetatileBehavior: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSelectedObjectEvent: u8 = 0u8;

unsafe extern "C" {
    static mut AbnormalWeather_EventScript_EndEventAndCleanup_1: u8;
    static mut BattlePyramid_WarpToNextFloor: u8;
    static mut EventScript_Blueprint: u8;
    static mut EventScript_BookShelf: u8;
    static mut EventScript_CableBoxResults: u8;
    static mut EventScript_CannotUseWaterfall: u8;
    static mut EventScript_ClosedSootopolisDoor: u8;
    static mut EventScript_EggHatch: u8;
    static mut EventScript_EmptyTrashCan: u8;
    static mut EventScript_FallDownHole: u8;
    static mut EventScript_FallDownHoleMtPyre: u8;
    static mut EventScript_FieldPoison: u8;
    static mut EventScript_HiddenItemScript: u8;
    static mut EventScript_PC: u8;
    static mut EventScript_PictureBookShelf: u8;
    static mut EventScript_PokeBlockFeeder: u8;
    static mut EventScript_PokemonCenterBookShelf: u8;
    static mut EventScript_Questionnaire: u8;
    static mut EventScript_RegionMap: u8;
    static mut EventScript_RunningShoesManual: u8;
    static mut EventScript_ShopShelf: u8;
    static mut EventScript_TV: u8;
    static mut EventScript_TestSignpostMsg: u8;
    static mut EventScript_TrainerHillTimer: u8;
    static mut EventScript_UseDive: u8;
    static mut EventScript_UseDiveUnderwater: u8;
    static mut EventScript_UseSurf: u8;
    static mut EventScript_UseWaterfall: u8;
    static mut EventScript_Vase: u8;
    static mut EventScript_WirelessBoxResults: u8;
    static mut IslandCave_EventScript_OpenRegiEntrance: u8;
    static mut LittlerootTown_BrendansHouse_2F_EventScript_PC: u8;
    static mut LittlerootTown_MaysHouse_2F_EventScript_PC: u8;
    static mut LittlerootTown_ProfessorBirchsLab_EventScript_ScottAboardSSTidalCall: u8;
    static mut MauvilleCity_EventScript_RegisterWallyCall: u8;
    static mut MossdeepCity_SpaceCenter_2F_EventScript_RivalRayquazaCall: u8;
    static mut Route110_TrickHousePuzzle_EventScript_Door: u8;
    static mut Route119_EventScript_ScottWonAtFortreeGymCall: u8;
    static mut RustboroCity_Gym_EventScript_RegisterRoxanne: u8;
    static mut SSTidalCorridor_EventScript_ReachedStepCount: u8;
    static mut SecretBase_EventScript_CheckEntrance: u8;
    static mut SecretBase_EventScript_CushionInteract: u8;
    static mut SecretBase_EventScript_DollInteract: u8;
    static mut SecretBase_EventScript_PC: u8;
    static mut SecretBase_EventScript_RecordMixingPC: u8;
    static mut SecretBase_EventScript_SandOrnament: u8;
    static mut SecretBase_EventScript_ShieldOrToyTV: u8;
    static mut SkyPillar_Outside_EventScript_ClosedDoor: u8;
    static mut gDirectionToVectors: u8;
    static mut gLinkPlayerObjectEvents: u8;
    static mut gMapHeader: u8;
    static mut gObjectEvents: u8;
    static mut gPlayerAvatar: u8;
    static mut gPlayerParty: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_Facing: u8;
    static mut gSpecialVar_LastTalked: u8;
    fn AbnormalWeatherHasExpired() -> u8;
    fn AdjustFriendship(a0: *mut u8, a1: u8);
    fn CheckForTrainersWantingBattle() -> u8;
    fn CheckInteractedWithFriendsFurnitureBottom();
    fn CheckInteractedWithFriendsFurnitureMiddle();
    fn CheckInteractedWithFriendsFurnitureTop();
    fn CheckInteractedWithFriendsPosterDecor();
    fn CountSSTidalStep(a0: u16) -> u32;
    fn DoCoordEventWeather(a0: u8);
    fn DoDiveWarp();
    fn DoDoorWarp();
    fn DoEscalatorWarp(a0: u8);
    fn DoLavaridgeGym1FWarp();
    fn DoLavaridgeGymB1FWarp();
    fn DoMossdeepGymWarp();
    fn DoPoisonFieldEffect() -> i32;
    fn DoSecretBaseGlitterMatSparkle();
    fn DoSpinExitWarp();
    fn DoTeleportTileWarp();
    fn DoWarp();
    fn FlagGet(a0: u16) -> u8;
    fn GetCurrentTrainerHillMapId() -> u8;
    fn GetNumFloorsInTrainerHillChallenge() -> u8;
    fn GetObjectEventIdByPosition(a0: u16, a1: u16, a2: u8) -> u8;
    fn GetObjectEventScriptPointerByObjectEventId(a0: u8) -> *mut u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetPlayerMovementDirection() -> u8;
    fn GetPlayerSpeed() -> i16;
    fn GetRamScript(a0: u8, a1: *mut u8) -> *mut u8;
    fn GetTrainerHillTrainerScript() -> *mut u8;
    fn GetVarPointer(a0: u16) -> *mut u16;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn InTrainerHill() -> u32;
    fn InUnionRoom() -> u32;
    fn IncrementBirthIslandRockStepCount();
    fn IncrementGameStat(a0: u8);
    fn IncrementRematchStepCounter();
    fn IsPlayerFacingSurfableFishableWater() -> u8;
    fn IsPlayerSurfingNorth() -> u8;
    fn MapGridGetElevationAt(a0: i32, a1: i32) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_HoldsLargeDecoration(a0: u8) -> u8;
    fn MetatileBehavior_HoldsSmallDecoration(a0: u8) -> u8;
    fn MetatileBehavior_IsAquaHideoutWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsBattlePyramidWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsBlueprint(a0: u8) -> u8;
    fn MetatileBehavior_IsBookShelf(a0: u8) -> u8;
    fn MetatileBehavior_IsCableBoxResults1(a0: u8) -> u8;
    fn MetatileBehavior_IsCableBoxResults2(a0: u8, a1: u8) -> u8;
    fn MetatileBehavior_IsClosedSootopolisDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsCounter(a0: u8) -> u8;
    fn MetatileBehavior_IsCrackedFloorHole(a0: u8) -> u8;
    fn MetatileBehavior_IsDiveable(a0: u8) -> u8;
    fn MetatileBehavior_IsEastArrowWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsEscalator(a0: u8) -> u8;
    fn MetatileBehavior_IsForcedMovementTile(a0: u8) -> u8;
    fn MetatileBehavior_IsLadder(a0: u8) -> u8;
    fn MetatileBehavior_IsLavaridge1FWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsLavaridgeB1FWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsMossdeepGymWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsMtPyreHole(a0: u8) -> u8;
    fn MetatileBehavior_IsNonAnimDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsNorthArrowWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsOpenSecretBaseDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsPC(a0: u8) -> u8;
    fn MetatileBehavior_IsPictureBookShelf(a0: u8) -> u8;
    fn MetatileBehavior_IsPlayerFacingTVScreen(a0: u8, a1: u8) -> u8;
    fn MetatileBehavior_IsPlayerFacingWirelessBoxResults(a0: u8, a1: u8) -> u8;
    fn MetatileBehavior_IsPokeCenterBookShelf(a0: u8) -> u8;
    fn MetatileBehavior_IsPokeblockFeeder(a0: u8) -> u8;
    fn MetatileBehavior_IsQuestionnaire(a0: u8) -> u8;
    fn MetatileBehavior_IsRecordMixingSecretBasePC(a0: u8) -> u8;
    fn MetatileBehavior_IsRegionMap(a0: u8) -> u8;
    fn MetatileBehavior_IsRunningShoesManual(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseDecorationBase(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseGlitterMat(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBasePC(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBasePoster(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseSandOrnament(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseShieldOrToyTV(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseSoundMat(a0: u8) -> u8;
    fn MetatileBehavior_IsShopShelf(a0: u8) -> u8;
    fn MetatileBehavior_IsSkyPillarClosedDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsSouthArrowWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsTrainerHillTimer(a0: u8) -> u8;
    fn MetatileBehavior_IsTrashCan(a0: u8) -> u8;
    fn MetatileBehavior_IsTrickHousePuzzleDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsUnableToEmerge(a0: u8) -> u8;
    fn MetatileBehavior_IsUnionRoomWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsVase(a0: u8) -> u8;
    fn MetatileBehavior_IsWarpDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsWaterfall(a0: u8) -> u8;
    fn MetatileBehavior_IsWestArrowWarp(a0: u8) -> u8;
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut u8;
    fn PartyHasMonWithSurf() -> u8;
    fn PlaySE(a0: u16);
    fn PlaySecretBaseMusicNoteMatSound(a0: i16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PlayerGetElevation() -> u8;
    fn RunScriptImmediately(a0: *mut u8);
    fn SafariZoneTakeStep() -> u8;
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetDiveWarpDive(a0: u16, a1: u16) -> u8;
    fn SetDiveWarpEmerge(a0: u16, a1: u16) -> u8;
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetWarpDestinationToDynamicWarp(a0: u8);
    fn SetWarpDestinationToMapWarp(a0: i8, a1: i8, a2: i8);
    fn SetWarpDestinationTrainerHill4F() -> *mut u8;
    fn SetWarpDestinationTrainerHillFinalFloor(a0: u8) -> *mut u8;
    fn ShouldDoBrailleRegicePuzzle() -> u8;
    fn ShouldDoRivalRayquazaCall() -> u32;
    fn ShouldDoRoxanneCall() -> u32;
    fn ShouldDoScottBattleFrontierCall() -> u32;
    fn ShouldDoScottFortreeCall() -> u32;
    fn ShouldDoWallyCall() -> u32;
    fn ShouldEggHatch() -> u8;
    fn ShowStartMenu();
    fn StandardWildEncounter(a0: u16, a1: u16) -> u8;
    fn StoreInitialPlayerAvatarState();
    fn TryRunOnFrameMapScript() -> u8;
    fn TrySetCurSecretBase() -> u8;
    fn TryStartMatchCall() -> u32;
    fn UpdateEscapeWarp(a0: i16, a1: i16);
    fn UpdateFarawayIslandStepCounter();
    fn UpdateRepelCounter() -> u8;
    fn UseRegisteredKeyItemOnField() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WarpIntoSecretBase(a0: *mut u8, a1: *mut u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldClearPlayerInput(input: *mut u8) {
    unsafe {
        let mut input = input;
        crate::c::bf_write((input).wrapping_add(0), 0, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(0), 1, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(0), 2, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(0), 3, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(0), 4, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(0), 5, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(0), 6, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(0), 7, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(1), 0, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(1), 1, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(1), 2, 1, (0u8) as i32);
        crate::c::bf_write((input).wrapping_add(1), 3, 1, (0u8) as i32);
        ((input).wrapping_add(2)).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldGetPlayerInput(input: *mut u8, newKeys: u16, heldKeys: u16) {
    unsafe {
        let mut input = input;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut tileTransitionState: u8 =
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).read();
        let mut runningState: u8 = (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).read();
        let mut forcedMove: u8 = MetatileBehavior_IsForcedMovementTile(
            ((GetPlayerCurMetatileBehavior(((runningState) as i32))) as u8),
        );
        if ((((tileTransitionState) as i32) == 2i32) && (((forcedMove) as i32) == 0i32))
            || (((tileTransitionState) as i32) == 0i32)
        {
            if ((GetPlayerSpeed()) as i32) != 4i32 {
                if (((newKeys) as i32) & 8i32) != 0 {
                    crate::c::bf_write((input).wrapping_add(0), 2, 1, (1u8) as i32);
                }
                if (((newKeys) as i32) & 4i32) != 0 {
                    crate::c::bf_write((input).wrapping_add(0), 3, 1, (1u8) as i32);
                }
                if (((newKeys) as i32) & 1i32) != 0 {
                    crate::c::bf_write((input).wrapping_add(0), 0, 1, (1u8) as i32);
                }
                if (((newKeys) as i32) & 2i32) != 0 {
                    crate::c::bf_write((input).wrapping_add(0), 7, 1, (1u8) as i32);
                }
            }
            if (((heldKeys) as i32) & 240i32) != 0 {
                crate::c::bf_write((input).wrapping_add(0), 4, 1, (1u8) as i32);
                crate::c::bf_write((input).wrapping_add(0), 5, 1, (1u8) as i32);
            }
        }
        if ((forcedMove) as i32) == 0i32 {
            if (((tileTransitionState) as i32) == 2i32) && (((runningState) as i32) == 2i32) {
                crate::c::bf_write((input).wrapping_add(0), 6, 1, (1u8) as i32);
            }
            if (((forcedMove) as i32) == 0i32) && (((tileTransitionState) as i32) == 2i32) {
                crate::c::bf_write((input).wrapping_add(0), 1, 1, (1u8) as i32);
            }
        }
        if (((heldKeys) as i32) & 64i32) != 0 {
            ((input).wrapping_add(2)).write(2u8);
        } else {
            if (((heldKeys) as i32) & 128i32) != 0 {
                ((input).wrapping_add(2)).write(1u8);
            } else {
                if (((heldKeys) as i32) & 32i32) != 0 {
                    ((input).wrapping_add(2)).write(3u8);
                } else {
                    if (((heldKeys) as i32) & 16i32) != 0 {
                        ((input).wrapping_add(2)).write(4u8);
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ProcessPlayerFieldInput(input: *mut u8) -> i32 {
    unsafe {
        let mut input = input;
        let mut position = crate::ffi::Align4([0u8; 8]);
        let mut playerDirection: u8 = 0u8;
        let mut metatileBehavior: u16 = 0u16;
        ((&raw mut gSpecialVar_LastTalked).cast::<u16>()).write(0u16);
        ((&raw mut gSelectedObjectEvent).cast::<u8>().cast::<u8>()).write(0u8);
        playerDirection = GetPlayerFacingDirection();
        GetPlayerPosition((&raw mut position).cast::<u8>());
        metatileBehavior = ((MapGridGetMetatileBehaviorAt(
            (((((&raw mut position).cast::<u8>()).cast::<i16>()).read()) as i32),
            (((((&raw mut position).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32),
        )) as u16);
        if ((CheckForTrainersWantingBattle()) as i32) == 1i32 {
            return 1i32;
        }
        if ((TryRunOnFrameMapScript()) as i32) == 1i32 {
            return 1i32;
        }
        if ((crate::c::bf_read((input).wrapping_add(0), 7, 1, false) as u8) != 0)
            && (TrySetupDiveEmergeScript() == 1u32)
        {
            return 1i32;
        }
        if (crate::c::bf_read((input).wrapping_add(0), 6, 1, false) as u8) != 0 {
            IncrementGameStat(5u8);
            IncrementBirthIslandRockStepCount();
            if ((TryStartStepBasedScript(
                (&raw mut position).cast::<u8>(),
                metatileBehavior,
                ((playerDirection) as u16),
            )) as i32)
                == 1i32
            {
                return 1i32;
            }
        }
        if ((crate::c::bf_read((input).wrapping_add(0), 1, 1, false) as u8) != 0)
            && (((CheckStandardWildEncounter(metatileBehavior)) as i32) == 1i32)
        {
            return 1i32;
        }
        if ((crate::c::bf_read((input).wrapping_add(0), 4, 1, false) as u8) != 0)
            && (((((input).wrapping_add(2)).read()) as i32) == ((playerDirection) as i32))
        {
            if ((TryArrowWarp(
                (&raw mut position).cast::<u8>(),
                metatileBehavior,
                playerDirection,
            )) as i32)
                == 1i32
            {
                return 1i32;
            }
        }
        GetInFrontOfPlayerPosition((&raw mut position).cast::<u8>());
        metatileBehavior = ((MapGridGetMetatileBehaviorAt(
            (((((&raw mut position).cast::<u8>()).cast::<i16>()).read()) as i32),
            (((((&raw mut position).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32),
        )) as u16);
        if ((crate::c::bf_read((input).wrapping_add(0), 0, 1, false) as u8) != 0)
            && (((TryStartInteractionScript(
                (&raw mut position).cast::<u8>(),
                metatileBehavior,
                playerDirection,
            )) as i32)
                == 1i32)
        {
            return 1i32;
        }
        if ((crate::c::bf_read((input).wrapping_add(0), 5, 1, false) as u8) != 0)
            && (((((input).wrapping_add(2)).read()) as i32) == ((playerDirection) as i32))
        {
            if ((TryDoorWarp(
                (&raw mut position).cast::<u8>(),
                metatileBehavior,
                playerDirection,
            )) as i32)
                == 1i32
            {
                return 1i32;
            }
        }
        if ((crate::c::bf_read((input).wrapping_add(0), 0, 1, false) as u8) != 0)
            && (TrySetupDiveDownScript() == 1u32)
        {
            return 1i32;
        }
        if (crate::c::bf_read((input).wrapping_add(0), 2, 1, false) as u8) != 0 {
            PlaySE(6u16);
            ShowStartMenu();
            return 1i32;
        }
        if ((crate::c::bf_read((input).wrapping_add(0), 3, 1, false) as u8) != 0)
            && (((UseRegisteredKeyItemOnField()) as i32) == 1i32)
        {
            return 1i32;
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn GetPlayerPosition(position: *mut u8) {
    unsafe {
        let mut position = position;
        PlayerGetDestCoords(
            (position).cast::<i16>(),
            (position).wrapping_add(2).cast::<i16>(),
        );
        ((position).wrapping_add(4).cast::<i8>()).write(((PlayerGetElevation()) as i8));
    }
}
pub(crate) unsafe extern "C" fn GetInFrontOfPlayerPosition(position: *mut u8) {
    unsafe {
        let mut position = position;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(
            (position).cast::<i16>(),
            (position).wrapping_add(2).cast::<i16>(),
        );
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        if ((MapGridGetElevationAt(((x) as i32), ((y) as i32))) as i32) != 0i32 {
            ((position).wrapping_add(4).cast::<i8>()).write(((PlayerGetElevation()) as i8));
        } else {
            ((position).wrapping_add(4).cast::<i8>()).write(0i8);
        }
    }
}
pub(crate) unsafe extern "C" fn GetPlayerCurMetatileBehavior(runningState: i32) -> u16 {
    unsafe {
        let mut runningState = runningState;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        return ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u16);
    }
}
pub(crate) unsafe extern "C" fn TryStartInteractionScript(
    position: *mut u8,
    metatileBehavior: u16,
    direction: u8,
) -> u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        let mut script: *mut u8 =
            GetInteractionScript(position, ((metatileBehavior) as u8), direction);
        if ((script) as usize) == 0usize {
            return 0u8;
        }
        if ((((((((script) as usize)
            != (((&raw mut LittlerootTown_BrendansHouse_2F_EventScript_PC).cast::<u8>())
                as usize))
            && (((script) as usize)
                != (((&raw mut LittlerootTown_MaysHouse_2F_EventScript_PC).cast::<u8>())
                    as usize)))
            && (((script) as usize)
                != (((&raw mut SecretBase_EventScript_PC).cast::<u8>()) as usize)))
            && (((script) as usize)
                != (((&raw mut SecretBase_EventScript_RecordMixingPC).cast::<u8>()) as usize)))
            && (((script) as usize)
                != (((&raw mut SecretBase_EventScript_DollInteract).cast::<u8>()) as usize)))
            && (((script) as usize)
                != (((&raw mut SecretBase_EventScript_CushionInteract).cast::<u8>()) as usize)))
            && (((script) as usize) != (((&raw mut EventScript_PC).cast::<u8>()) as usize))
        {
            PlaySE(5u16);
        }
        ScriptContext_SetupScript(script);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GetInteractionScript(
    position: *mut u8,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        let mut script: *mut u8 =
            GetInteractedObjectEventScript(position, metatileBehavior, direction);
        if ((script) as usize) != 0usize {
            return script;
        }
        script = GetInteractedBackgroundEventScript(position, metatileBehavior, direction);
        if ((script) as usize) != 0usize {
            return script;
        }
        script = GetInteractedMetatileScript(position, metatileBehavior, direction);
        if ((script) as usize) != 0usize {
            return script;
        }
        script = GetInteractedWaterScript(position, metatileBehavior, direction);
        if ((script) as usize) != 0usize {
            return script;
        }
        return core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInteractedLinkPlayerScript(
    position: *mut u8,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        let mut objectEventId: u8 = 0u8;
        let mut i: i32 = 0i32;
        if !((MetatileBehavior_IsCounter(
            ((MapGridGetMetatileBehaviorAt(
                ((((position).cast::<i16>()).read()) as i32),
                ((((position).wrapping_add(2).cast::<i16>()).read()) as i32),
            )) as u8),
        )) != 0)
        {
            objectEventId = GetObjectEventIdByPosition(
                ((((position).cast::<i16>()).read()) as u16),
                ((((position).wrapping_add(2).cast::<i16>()).read()) as u16),
                ((((position).wrapping_add(4).cast::<i8>()).read()) as u8),
            );
        } else {
            objectEventId = GetObjectEventIdByPosition(
                ((((((position).cast::<i16>()).read()) as u32).wrapping_add(
                    ((((&raw mut gDirectionToVectors).cast::<u8>())
                        .wrapping_offset(((direction) as i32) as isize * 8))
                    .cast::<u32>())
                    .read(),
                )) as u16),
                ((((((position).wrapping_add(2).cast::<i16>()).read()) as u32).wrapping_add(
                    ((((&raw mut gDirectionToVectors).cast::<u8>())
                        .wrapping_offset(((direction) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .read(),
                )) as u16),
                ((((position).wrapping_add(4).cast::<i8>()).read()) as u8),
            );
        }
        if (((objectEventId) as i32) == 16i32)
            || (((((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36))
            .wrapping_add(8))
            .read()) as i32)
                == 255i32)
        {
            return core::ptr::null_mut();
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                    .read()) as i32)
                        == 1i32)
                        && (((((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                        .wrapping_add(2))
                        .read()) as i32)
                            == ((objectEventId) as i32))
                    {
                        return core::ptr::null_mut();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSelectedObjectEvent).cast::<u8>().cast::<u8>()).write(objectEventId);
        ((&raw mut gSpecialVar_LastTalked).cast::<u16>()).write(
            ((((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36))
            .wrapping_add(8))
            .read()) as u16),
        );
        ((&raw mut gSpecialVar_Facing).cast::<u16>()).write(((direction) as u16));
        return GetObjectEventScriptPointerByObjectEventId(objectEventId);
    }
}
pub(crate) unsafe extern "C" fn GetInteractedObjectEventScript(
    position: *mut u8,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        let mut objectEventId: u8 = 0u8;
        let mut script: *mut u8 = core::ptr::null_mut();
        objectEventId = GetObjectEventIdByPosition(
            ((((position).cast::<i16>()).read()) as u16),
            ((((position).wrapping_add(2).cast::<i16>()).read()) as u16),
            ((((position).wrapping_add(4).cast::<i8>()).read()) as u8),
        );
        if (((objectEventId) as i32) == 16i32)
            || (((((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36))
            .wrapping_add(8))
            .read()) as i32)
                == 255i32)
        {
            if ((MetatileBehavior_IsCounter(metatileBehavior)) as i32) != 1i32 {
                return core::ptr::null_mut();
            }
            objectEventId = GetObjectEventIdByPosition(
                ((((((position).cast::<i16>()).read()) as u32).wrapping_add(
                    ((((&raw mut gDirectionToVectors).cast::<u8>())
                        .wrapping_offset(((direction) as i32) as isize * 8))
                    .cast::<u32>())
                    .read(),
                )) as u16),
                ((((((position).wrapping_add(2).cast::<i16>()).read()) as u32).wrapping_add(
                    ((((&raw mut gDirectionToVectors).cast::<u8>())
                        .wrapping_offset(((direction) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .read(),
                )) as u16),
                ((((position).wrapping_add(4).cast::<i8>()).read()) as u8),
            );
            if (((objectEventId) as i32) == 16i32)
                || (((((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(8))
                .read()) as i32)
                    == 255i32)
            {
                return core::ptr::null_mut();
            }
        }
        ((&raw mut gSelectedObjectEvent).cast::<u8>().cast::<u8>()).write(objectEventId);
        ((&raw mut gSpecialVar_LastTalked).cast::<u16>()).write(
            ((((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36))
            .wrapping_add(8))
            .read()) as u16),
        );
        ((&raw mut gSpecialVar_Facing).cast::<u16>()).write(((direction) as u16));
        if InTrainerHill() == 1u32 {
            script = GetTrainerHillTrainerScript();
        } else {
            script = GetObjectEventScriptPointerByObjectEventId(objectEventId);
        }
        script = GetRamScript(
            ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as u8),
            script,
        );
        return script;
    }
}
pub(crate) unsafe extern "C" fn GetInteractedBackgroundEventScript(
    position: *mut u8,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        let mut bgEvent: *mut u8 = GetBackgroundEventAtPosition(
            (&raw mut gMapHeader).cast::<u8>(),
            ((((((position).cast::<i16>()).read()) as i32).wrapping_sub(7i32)) as u16),
            ((((((position).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_sub(7i32))
                as u16),
            ((((position).wrapping_add(4).cast::<i8>()).read()) as u8),
        );
        if ((bgEvent) as usize) == 0usize {
            return core::ptr::null_mut();
        }
        if (((((bgEvent).wrapping_add(8)).cast::<*mut u8>()).read()) as usize) == 0usize {
            return (&raw mut EventScript_TestSignpostMsg).cast::<u8>();
        }
        'l1: {
            let __sw1 = ((((bgEvent).wrapping_add(5)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32;
            if __sw1 == 0i32 || !__matched {
                return (((bgEvent).wrapping_add(8)).cast::<*mut u8>()).read();
            }
            if __sw1 == 1i32 {
                if ((direction) as i32) != 2i32 {
                    return core::ptr::null_mut();
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((direction) as i32) != 1i32 {
                    return core::ptr::null_mut();
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((direction) as i32) != 4i32 {
                    return core::ptr::null_mut();
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((direction) as i32) != 3i32 {
                    return core::ptr::null_mut();
                }
                break 'l1;
            }
            if __sw1 == 5i32 || __sw1 == 6i32 || __sw1 == 7i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(
                    ((((((((bgEvent).wrapping_add(8)).cast::<*mut u8>()).read()) as usize as u32)
                        >> 16)
                        .wrapping_add(500u32)) as u16),
                );
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
                    ((((((bgEvent).wrapping_add(8)).cast::<*mut u8>()).read()) as usize as u32)
                        as u16),
                );
                if ((FlagGet(((&raw mut gSpecialVar_0x8004).cast::<u16>()).read())) as i32) == 1i32
                {
                    return core::ptr::null_mut();
                }
                return (&raw mut EventScript_HiddenItemScript).cast::<u8>();
            }
            if __sw1 == 8i32 {
                if ((direction) as i32) == 2i32 {
                    ((&raw mut gSpecialVar_0x8004).cast::<u16>())
                        .write((((((bgEvent).wrapping_add(8)).cast::<u32>()).read()) as u16));
                    if (TrySetCurSecretBase()) != 0 {
                        return (&raw mut SecretBase_EventScript_CheckEntrance).cast::<u8>();
                    }
                }
                return core::ptr::null_mut();
            }
        }
        return (((bgEvent).wrapping_add(8)).cast::<*mut u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn GetInteractedMetatileScript(
    position: *mut u8,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        let mut elevation: i8 = 0i8;
        if ((MetatileBehavior_IsPlayerFacingTVScreen(metatileBehavior, direction)) as i32) == 1i32 {
            return (&raw mut EventScript_TV).cast::<u8>();
        }
        if ((MetatileBehavior_IsPC(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_PC).cast::<u8>();
        }
        if ((MetatileBehavior_IsClosedSootopolisDoor(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_ClosedSootopolisDoor).cast::<u8>();
        }
        if ((MetatileBehavior_IsSkyPillarClosedDoor(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut SkyPillar_Outside_EventScript_ClosedDoor).cast::<u8>();
        }
        if ((MetatileBehavior_IsCableBoxResults1(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_CableBoxResults).cast::<u8>();
        }
        if ((MetatileBehavior_IsPokeblockFeeder(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_PokeBlockFeeder).cast::<u8>();
        }
        if ((MetatileBehavior_IsTrickHousePuzzleDoor(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut Route110_TrickHousePuzzle_EventScript_Door).cast::<u8>();
        }
        if ((MetatileBehavior_IsRegionMap(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_RegionMap).cast::<u8>();
        }
        if ((MetatileBehavior_IsRunningShoesManual(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_RunningShoesManual).cast::<u8>();
        }
        if ((MetatileBehavior_IsPictureBookShelf(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_PictureBookShelf).cast::<u8>();
        }
        if ((MetatileBehavior_IsBookShelf(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_BookShelf).cast::<u8>();
        }
        if ((MetatileBehavior_IsPokeCenterBookShelf(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_PokemonCenterBookShelf).cast::<u8>();
        }
        if ((MetatileBehavior_IsVase(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_Vase).cast::<u8>();
        }
        if ((MetatileBehavior_IsTrashCan(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_EmptyTrashCan).cast::<u8>();
        }
        if ((MetatileBehavior_IsShopShelf(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_ShopShelf).cast::<u8>();
        }
        if ((MetatileBehavior_IsBlueprint(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_Blueprint).cast::<u8>();
        }
        if ((MetatileBehavior_IsPlayerFacingWirelessBoxResults(metatileBehavior, direction)) as i32)
            == 1i32
        {
            return (&raw mut EventScript_WirelessBoxResults).cast::<u8>();
        }
        if ((MetatileBehavior_IsCableBoxResults2(metatileBehavior, direction)) as i32) == 1i32 {
            return (&raw mut EventScript_CableBoxResults).cast::<u8>();
        }
        if ((MetatileBehavior_IsQuestionnaire(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_Questionnaire).cast::<u8>();
        }
        if ((MetatileBehavior_IsTrainerHillTimer(metatileBehavior)) as i32) == 1i32 {
            return (&raw mut EventScript_TrainerHillTimer).cast::<u8>();
        }
        elevation = ((position).wrapping_add(4).cast::<i8>()).read();
        if ((elevation) as i32)
            == ((MapGridGetElevationAt(
                ((((position).cast::<i16>()).read()) as i32),
                ((((position).wrapping_add(2).cast::<i16>()).read()) as i32),
            )) as i32)
        {
            if ((MetatileBehavior_IsSecretBasePC(metatileBehavior)) as i32) == 1i32 {
                return (&raw mut SecretBase_EventScript_PC).cast::<u8>();
            }
            if ((MetatileBehavior_IsRecordMixingSecretBasePC(metatileBehavior)) as i32) == 1i32 {
                return (&raw mut SecretBase_EventScript_RecordMixingPC).cast::<u8>();
            }
            if ((MetatileBehavior_IsSecretBaseSandOrnament(metatileBehavior)) as i32) == 1i32 {
                return (&raw mut SecretBase_EventScript_SandOrnament).cast::<u8>();
            }
            if ((MetatileBehavior_IsSecretBaseShieldOrToyTV(metatileBehavior)) as i32) == 1i32 {
                return (&raw mut SecretBase_EventScript_ShieldOrToyTV).cast::<u8>();
            }
            if ((MetatileBehavior_IsSecretBaseDecorationBase(metatileBehavior)) as i32) == 1i32 {
                CheckInteractedWithFriendsFurnitureBottom();
                return core::ptr::null_mut();
            }
            if ((MetatileBehavior_HoldsLargeDecoration(metatileBehavior)) as i32) == 1i32 {
                CheckInteractedWithFriendsFurnitureMiddle();
                return core::ptr::null_mut();
            }
            if ((MetatileBehavior_HoldsSmallDecoration(metatileBehavior)) as i32) == 1i32 {
                CheckInteractedWithFriendsFurnitureTop();
                return core::ptr::null_mut();
            }
        } else {
            if ((MetatileBehavior_IsSecretBasePoster(metatileBehavior)) as i32) == 1i32 {
                CheckInteractedWithFriendsPosterDecor();
                return core::ptr::null_mut();
            }
        }
        return core::ptr::null_mut();
    }
}
pub(crate) unsafe extern "C" fn GetInteractedWaterScript(
    unused1: *mut u8,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    unsafe {
        let mut unused1 = unused1;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        if ((((FlagGet(2155u16)) as i32) == 1i32) && (((PartyHasMonWithSurf()) as i32) == 1i32))
            && (((IsPlayerFacingSurfableFishableWater()) as i32) == 1i32)
        {
            return (&raw mut EventScript_UseSurf).cast::<u8>();
        }
        if ((MetatileBehavior_IsWaterfall(metatileBehavior)) as i32) == 1i32 {
            if (((FlagGet(2158u16)) as i32) == 1i32) && (((IsPlayerSurfingNorth()) as i32) == 1i32)
            {
                return (&raw mut EventScript_UseWaterfall).cast::<u8>();
            } else {
                return (&raw mut EventScript_CannotUseWaterfall).cast::<u8>();
            }
        }
        return core::ptr::null_mut();
    }
}
pub(crate) unsafe extern "C" fn TrySetupDiveDownScript() -> u32 {
    unsafe {
        if ((FlagGet(2157u16)) != 0) && (((TrySetDiveWarp()) as i32) == 2i32) {
            ScriptContext_SetupScript((&raw mut EventScript_UseDive).cast::<u8>());
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn TrySetupDiveEmergeScript() -> u32 {
    unsafe {
        if (((FlagGet(2157u16)) != 0)
            && ((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32) == 5i32))
            && (((TrySetDiveWarp()) as i32) == 1i32)
        {
            ScriptContext_SetupScript((&raw mut EventScript_UseDiveUnderwater).cast::<u8>());
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn TryStartStepBasedScript(
    position: *mut u8,
    metatileBehavior: u16,
    direction: u16,
) -> u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        if ((TryStartCoordEventScript(position)) as i32) == 1i32 {
            return 1u8;
        }
        if ((TryStartWarpEventScript(position, metatileBehavior)) as i32) == 1i32 {
            return 1u8;
        }
        if ((TryStartMiscWalkingScripts(metatileBehavior)) as i32) == 1i32 {
            return 1u8;
        }
        if ((TryStartStepCountScript(metatileBehavior)) as i32) == 1i32 {
            return 1u8;
        }
        if ((UpdateRepelCounter()) as i32) == 1i32 {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TryStartCoordEventScript(position: *mut u8) -> u8 {
    unsafe {
        let mut position = position;
        let mut script: *mut u8 = GetCoordEventScriptAtPosition(
            (&raw mut gMapHeader).cast::<u8>(),
            ((((((position).cast::<i16>()).read()) as i32).wrapping_sub(7i32)) as u16),
            ((((((position).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_sub(7i32))
                as u16),
            ((((position).wrapping_add(4).cast::<i8>()).read()) as u8),
        );
        if ((script) as usize) == 0usize {
            return 0u8;
        }
        ScriptContext_SetupScript(script);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn TryStartMiscWalkingScripts(metatileBehavior: u16) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        if (MetatileBehavior_IsCrackedFloorHole(((metatileBehavior) as u8))) != 0 {
            ScriptContext_SetupScript((&raw mut EventScript_FallDownHole).cast::<u8>());
            return 1u8;
        } else {
            if (MetatileBehavior_IsBattlePyramidWarp(((metatileBehavior) as u8))) != 0 {
                ScriptContext_SetupScript((&raw mut BattlePyramid_WarpToNextFloor).cast::<u8>());
                return 1u8;
            } else {
                if ((MetatileBehavior_IsSecretBaseGlitterMat(((metatileBehavior) as u8))) as i32)
                    == 1i32
                {
                    DoSecretBaseGlitterMatSparkle();
                    return 0u8;
                } else {
                    if ((MetatileBehavior_IsSecretBaseSoundMat(((metatileBehavior) as u8))) as i32)
                        == 1i32
                    {
                        PlayerGetDestCoords(&raw mut x, &raw mut y);
                        PlaySecretBaseMusicNoteMatSound(
                            ((MapGridGetMetatileIdAt(((x) as i32), ((y) as i32))) as i16),
                        );
                        return 0u8;
                    }
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TryStartStepCountScript(metatileBehavior: u16) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if InUnionRoom() == 1u32 {
            return 0u8;
        }
        IncrementRematchStepCounter();
        UpdateFriendshipStepCounter();
        UpdateFarawayIslandStepCounter();
        if (!((((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 64i32) != 0))
            && (!((MetatileBehavior_IsForcedMovementTile(((metatileBehavior) as u8))) != 0))
        {
            if ((UpdatePoisonStepCounter()) as i32) == 1i32 {
                ScriptContext_SetupScript((&raw mut EventScript_FieldPoison).cast::<u8>());
                return 1u8;
            }
            if (ShouldEggHatch()) != 0 {
                IncrementGameStat(13u8);
                ScriptContext_SetupScript((&raw mut EventScript_EggHatch).cast::<u8>());
                return 1u8;
            }
            if ((AbnormalWeatherHasExpired()) as i32) == 1i32 {
                ScriptContext_SetupScript(
                    (&raw mut AbnormalWeather_EventScript_EndEventAndCleanup_1).cast::<u8>(),
                );
                return 1u8;
            }
            if ((ShouldDoBrailleRegicePuzzle()) as i32) == 1i32 {
                ScriptContext_SetupScript(
                    (&raw mut IslandCave_EventScript_OpenRegiEntrance).cast::<u8>(),
                );
                return 1u8;
            }
            if ShouldDoWallyCall() == 1u32 {
                ScriptContext_SetupScript(
                    (&raw mut MauvilleCity_EventScript_RegisterWallyCall).cast::<u8>(),
                );
                return 1u8;
            }
            if ShouldDoScottFortreeCall() == 1u32 {
                ScriptContext_SetupScript(
                    (&raw mut Route119_EventScript_ScottWonAtFortreeGymCall).cast::<u8>(),
                );
                return 1u8;
            }
            if ShouldDoScottBattleFrontierCall() == 1u32 {
                ScriptContext_SetupScript(
                    (&raw mut LittlerootTown_ProfessorBirchsLab_EventScript_ScottAboardSSTidalCall)
                        .cast::<u8>(),
                );
                return 1u8;
            }
            if ShouldDoRoxanneCall() == 1u32 {
                ScriptContext_SetupScript(
                    (&raw mut RustboroCity_Gym_EventScript_RegisterRoxanne).cast::<u8>(),
                );
                return 1u8;
            }
            if ShouldDoRivalRayquazaCall() == 1u32 {
                ScriptContext_SetupScript(
                    (&raw mut MossdeepCity_SpaceCenter_2F_EventScript_RivalRayquazaCall)
                        .cast::<u8>(),
                );
                return 1u8;
            }
        }
        if ((SafariZoneTakeStep()) as i32) == 1i32 {
            return 1u8;
        }
        if CountSSTidalStep(1u16) == 1u32 {
            ScriptContext_SetupScript(
                (&raw mut SSTidalCorridor_EventScript_ReachedStepCount).cast::<u8>(),
            );
            return 1u8;
        }
        if (TryStartMatchCall()) != 0 {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ClearFriendshipStepCounter() {
    unsafe {
        VarSet(16426u16, 0u16);
    }
}
pub(crate) unsafe extern "C" fn UpdateFriendshipStepCounter() {
    unsafe {
        let mut ptr: *mut u16 = GetVarPointer(16426u16);
        let mut i: i32 = 0i32;
        (ptr).write(((ptr).read()).wrapping_add(1));
        (ptr).write(((crate::c::rem_i32((((ptr).read()) as i32), 128i32)) as u16));
        if (((ptr).read()) as i32) == 0i32 {
            let mut mon: *mut u8 = (&raw mut gPlayerParty).cast::<u8>();
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        AdjustFriendship(mon, 5u8);
                        mon = (mon).wrapping_offset(100);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearPoisonStepCounter() {
    unsafe {
        VarSet(16427u16, 0u16);
    }
}
pub(crate) unsafe extern "C" fn UpdatePoisonStepCounter() -> u8 {
    unsafe {
        let mut ptr: *mut u16 = core::ptr::null_mut();
        if (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32) != 9i32 {
            ptr = GetVarPointer(16427u16);
            (ptr).write(((ptr).read()).wrapping_add(1));
            (ptr).write(((crate::c::rem_i32((((ptr).read()) as i32), 4i32)) as u16));
            if (((ptr).read()) as i32) == 0i32 {
                'l1: {
                    let __sw1 = DoPoisonFieldEffect();
                    if __sw1 == 0i32 {
                        return 0u8;
                    }
                    if __sw1 == 1i32 {
                        return 0u8;
                    }
                    if __sw1 == 2i32 {
                        return 1u8;
                    }
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RestartWildEncounterImmunitySteps() {
    unsafe {
        ((&raw mut sWildEncounterImmunitySteps)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn CheckStandardWildEncounter(metatileBehavior: u16) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((&raw mut sWildEncounterImmunitySteps)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            < 4i32
        {
            let __p1 = (&raw mut sWildEncounterImmunitySteps)
                .cast::<u8>()
                .cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((&raw mut sPrevMetatileBehavior).cast::<u8>().cast::<u16>()).write(metatileBehavior);
            return 0u8;
        }
        if ((StandardWildEncounter(
            metatileBehavior,
            ((&raw mut sPrevMetatileBehavior).cast::<u8>().cast::<u16>()).read(),
        )) as i32)
            == 1i32
        {
            ((&raw mut sWildEncounterImmunitySteps)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            ((&raw mut sPrevMetatileBehavior).cast::<u8>().cast::<u16>()).write(metatileBehavior);
            return 1u8;
        }
        ((&raw mut sPrevMetatileBehavior).cast::<u8>().cast::<u16>()).write(metatileBehavior);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TryArrowWarp(
    position: *mut u8,
    metatileBehavior: u16,
    direction: u8,
) -> u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        let mut warpEventId: i8 =
            GetWarpEventAtMapPosition((&raw mut gMapHeader).cast::<u8>(), position);
        if (((IsArrowWarpMetatileBehavior(metatileBehavior, direction)) as i32) == 1i32)
            && (((warpEventId) as i32) != (-1i32))
        {
            StoreInitialPlayerAvatarState();
            SetupWarp((&raw mut gMapHeader).cast::<u8>(), warpEventId, position);
            DoWarp();
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TryStartWarpEventScript(
    position: *mut u8,
    metatileBehavior: u16,
) -> u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut warpEventId: i8 =
            GetWarpEventAtMapPosition((&raw mut gMapHeader).cast::<u8>(), position);
        if (((warpEventId) as i32) != (-1i32))
            && (((IsWarpMetatileBehavior(metatileBehavior)) as i32) == 1i32)
        {
            StoreInitialPlayerAvatarState();
            SetupWarp((&raw mut gMapHeader).cast::<u8>(), warpEventId, position);
            if ((MetatileBehavior_IsEscalator(((metatileBehavior) as u8))) as i32) == 1i32 {
                DoEscalatorWarp(((metatileBehavior) as u8));
                return 1u8;
            }
            if ((MetatileBehavior_IsLavaridgeB1FWarp(((metatileBehavior) as u8))) as i32) == 1i32 {
                DoLavaridgeGymB1FWarp();
                return 1u8;
            }
            if ((MetatileBehavior_IsLavaridge1FWarp(((metatileBehavior) as u8))) as i32) == 1i32 {
                DoLavaridgeGym1FWarp();
                return 1u8;
            }
            if ((MetatileBehavior_IsAquaHideoutWarp(((metatileBehavior) as u8))) as i32) == 1i32 {
                DoTeleportTileWarp();
                return 1u8;
            }
            if ((MetatileBehavior_IsUnionRoomWarp(((metatileBehavior) as u8))) as i32) == 1i32 {
                DoSpinExitWarp();
                return 1u8;
            }
            if ((MetatileBehavior_IsMtPyreHole(((metatileBehavior) as u8))) as i32) == 1i32 {
                ScriptContext_SetupScript((&raw mut EventScript_FallDownHoleMtPyre).cast::<u8>());
                return 1u8;
            }
            if ((MetatileBehavior_IsMossdeepGymWarp(((metatileBehavior) as u8))) as i32) == 1i32 {
                DoMossdeepGymWarp();
                return 1u8;
            }
            DoWarp();
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsWarpMetatileBehavior(metatileBehavior: u16) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((((((((MetatileBehavior_IsWarpDoor(((metatileBehavior) as u8))) as i32)
            != 1i32)
            && (((MetatileBehavior_IsLadder(((metatileBehavior) as u8))) as i32) != 1i32))
            && (((MetatileBehavior_IsEscalator(((metatileBehavior) as u8))) as i32) != 1i32))
            && (((MetatileBehavior_IsNonAnimDoor(((metatileBehavior) as u8))) as i32) != 1i32))
            && (((MetatileBehavior_IsLavaridgeB1FWarp(((metatileBehavior) as u8))) as i32)
                != 1i32))
            && (((MetatileBehavior_IsLavaridge1FWarp(((metatileBehavior) as u8))) as i32)
                != 1i32))
            && (((MetatileBehavior_IsAquaHideoutWarp(((metatileBehavior) as u8))) as i32)
                != 1i32))
            && (((MetatileBehavior_IsMtPyreHole(((metatileBehavior) as u8))) as i32) != 1i32))
            && (((MetatileBehavior_IsMossdeepGymWarp(((metatileBehavior) as u8))) as i32) != 1i32))
            && (((MetatileBehavior_IsUnionRoomWarp(((metatileBehavior) as u8))) as i32) != 1i32)
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn IsArrowWarpMetatileBehavior(
    metatileBehavior: u16,
    direction: u8,
) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        'l1: {
            let __sw1 = ((direction) as i32);
            if __sw1 == 2i32 {
                return MetatileBehavior_IsNorthArrowWarp(((metatileBehavior) as u8));
            }
            if __sw1 == 1i32 {
                return MetatileBehavior_IsSouthArrowWarp(((metatileBehavior) as u8));
            }
            if __sw1 == 3i32 {
                return MetatileBehavior_IsWestArrowWarp(((metatileBehavior) as u8));
            }
            if __sw1 == 4i32 {
                return MetatileBehavior_IsEastArrowWarp(((metatileBehavior) as u8));
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetWarpEventAtMapPosition(
    mapHeader: *mut u8,
    position: *mut u8,
) -> i8 {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut position = position;
        return GetWarpEventAtPosition(
            mapHeader,
            ((((((position).cast::<i16>()).read()) as i32).wrapping_sub(7i32)) as u16),
            ((((((position).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_sub(7i32))
                as u16),
            ((((position).wrapping_add(4).cast::<i8>()).read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn SetupWarp(unused: *mut u8, warpEventId: i8, position: *mut u8) {
    unsafe {
        let mut unused = unused;
        let mut warpEventId = warpEventId;
        let mut position = position;
        let mut warpEvent: *mut u8 = core::ptr::null_mut();
        let mut trainerHillMapId: u8 = GetCurrentTrainerHillMapId();
        if (trainerHillMapId) != 0 {
            if ((trainerHillMapId) as i32) == ((GetNumFloorsInTrainerHillChallenge()) as i32) {
                if ((warpEventId) as i32) == 0i32 {
                    warpEvent = (((((&raw mut gMapHeader).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                    .read();
                } else {
                    warpEvent = SetWarpDestinationTrainerHill4F();
                }
            } else {
                if ((trainerHillMapId) as i32) == 5i32 {
                    warpEvent = SetWarpDestinationTrainerHillFinalFloor(((warpEventId) as u8));
                } else {
                    warpEvent = ((((((&raw mut gMapHeader).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((warpEventId) as i32) as isize * 8);
                }
            }
        } else {
            warpEvent = ((((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8)
            .cast::<*mut u8>())
            .read())
            .wrapping_offset(((warpEventId) as i32) as isize * 8);
        }
        if ((((warpEvent).wrapping_add(6)).read()) as i32) == 127i32 {
            SetWarpDestinationToDynamicWarp(((warpEvent).wrapping_add(5)).read());
        } else {
            let mut mapHeader: *mut u8 = core::ptr::null_mut();
            SetWarpDestinationToMapWarp(
                ((((warpEvent).wrapping_add(7)).read()) as i8),
                ((((warpEvent).wrapping_add(6)).read()) as i8),
                ((((warpEvent).wrapping_add(5)).read()) as i8),
            );
            UpdateEscapeWarp(
                ((position).cast::<i16>()).read(),
                ((position).wrapping_add(2).cast::<i16>()).read(),
            );
            mapHeader = Overworld_GetMapHeaderByGroupAndId(
                ((((warpEvent).wrapping_add(7)).read()) as u16),
                ((((warpEvent).wrapping_add(6)).read()) as u16),
            );
            if (((((((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((((warpEvent).wrapping_add(5)).read()) as i32) as isize * 8))
            .wrapping_add(6))
            .read()) as i32)
                == 127i32
            {
                SetDynamicWarp(
                    (((((((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((warpEventId) as i32) as isize * 8))
                    .wrapping_add(5))
                    .read()) as i32),
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<i8>())
                    .read(),
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .wrapping_add(1)
                        .cast::<i8>())
                    .read(),
                    warpEventId,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryDoorWarp(
    position: *mut u8,
    metatileBehavior: u16,
    direction: u8,
) -> u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        let mut direction = direction;
        let mut warpEventId: i8 = 0i8;
        if ((direction) as i32) == 2i32 {
            if ((MetatileBehavior_IsOpenSecretBaseDoor(((metatileBehavior) as u8))) as i32) == 1i32
            {
                WarpIntoSecretBase(
                    position,
                    (((&raw mut gMapHeader).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read(),
                );
                return 1u8;
            }
            if ((MetatileBehavior_IsWarpDoor(((metatileBehavior) as u8))) as i32) == 1i32 {
                warpEventId =
                    GetWarpEventAtMapPosition((&raw mut gMapHeader).cast::<u8>(), position);
                if (((warpEventId) as i32) != (-1i32))
                    && (((IsWarpMetatileBehavior(metatileBehavior)) as i32) == 1i32)
                {
                    StoreInitialPlayerAvatarState();
                    SetupWarp((&raw mut gMapHeader).cast::<u8>(), warpEventId, position);
                    DoDoorWarp();
                    return 1u8;
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetWarpEventAtPosition(
    mapHeader: *mut u8,
    x: u16,
    y: u16,
    elevation: u8,
) -> i8 {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut x = x;
        let mut y = y;
        let mut elevation = elevation;
        let mut i: i32 = 0i32;
        let mut warpEvent: *mut u8 = ((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read();
        let mut warpCount: u8 =
            ((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_add(1)).read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((warpCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((warpEvent).cast::<i16>()).read()) as u16) as i32) == ((x) as i32))
                        && ((((((warpEvent).wrapping_add(2).cast::<i16>()).read()) as u16) as i32)
                            == ((y) as i32))
                    {
                        if (((((warpEvent).wrapping_add(4)).read()) as i32) == ((elevation) as i32))
                            || (((((warpEvent).wrapping_add(4)).read()) as i32) == 0i32)
                        {
                            return ((i) as i8);
                        }
                    }
                }
                i = (i).wrapping_add(1);
                warpEvent = (warpEvent).wrapping_offset(8);
            }
        }
        return (-1i8);
    }
}
pub(crate) unsafe extern "C" fn TryRunCoordEventScript(coordEvent: *mut u8) -> *mut u8 {
    unsafe {
        let mut coordEvent = coordEvent;
        if ((coordEvent) as usize) != 0usize {
            if ((((coordEvent).wrapping_add(12).cast::<*mut u8>()).read()) as usize) == 0usize {
                DoCoordEventWeather(((((coordEvent).wrapping_add(6).cast::<u16>()).read()) as u8));
                return core::ptr::null_mut();
            }
            if ((((coordEvent).wrapping_add(6).cast::<u16>()).read()) as i32) == 0i32 {
                RunScriptImmediately(((coordEvent).wrapping_add(12).cast::<*mut u8>()).read());
                return core::ptr::null_mut();
            }
            if ((VarGet(((coordEvent).wrapping_add(6).cast::<u16>()).read())) as i32)
                == (((((coordEvent).wrapping_add(8).cast::<u16>()).read()) as u8) as i32)
            {
                return ((coordEvent).wrapping_add(12).cast::<*mut u8>()).read();
            }
        }
        return core::ptr::null_mut();
    }
}
pub(crate) unsafe extern "C" fn GetCoordEventScriptAtPosition(
    mapHeader: *mut u8,
    x: u16,
    y: u16,
    elevation: u8,
) -> *mut u8 {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut x = x;
        let mut y = y;
        let mut elevation = elevation;
        let mut i: i32 = 0i32;
        let mut coordEvents: *mut u8 = ((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read();
        let mut coordEventCount: u8 =
            ((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_add(2)).read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((coordEventCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((coordEvents).wrapping_offset((i) as isize * 16)).cast::<i16>())
                        .read()) as u16) as i32)
                        == ((x) as i32))
                        && (((((((coordEvents).wrapping_offset((i) as isize * 16))
                            .wrapping_add(2)
                            .cast::<i16>())
                        .read()) as u16) as i32)
                            == ((y) as i32))
                    {
                        if ((((((coordEvents).wrapping_offset((i) as isize * 16)).wrapping_add(4))
                            .read()) as i32)
                            == ((elevation) as i32))
                            || ((((((coordEvents).wrapping_offset((i) as isize * 16))
                                .wrapping_add(4))
                            .read()) as i32)
                                == 0i32)
                        {
                            let mut script: *mut u8 = TryRunCoordEventScript(
                                (coordEvents).wrapping_offset((i) as isize * 16),
                            );
                            if ((script) as usize) != 0usize {
                                return script;
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCoordEventScriptAtMapPosition(position: *mut u8) -> *mut u8 {
    unsafe {
        let mut position = position;
        return GetCoordEventScriptAtPosition(
            (&raw mut gMapHeader).cast::<u8>(),
            ((((((position).cast::<i16>()).read()) as i32).wrapping_sub(7i32)) as u16),
            ((((((position).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_sub(7i32))
                as u16),
            ((((position).wrapping_add(4).cast::<i8>()).read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn GetBackgroundEventAtPosition(
    mapHeader: *mut u8,
    x: u16,
    y: u16,
    elevation: u8,
) -> *mut u8 {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut x = x;
        let mut y = y;
        let mut elevation = elevation;
        let mut i: u8 = 0u8;
        let mut bgEvents: *mut u8 = ((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .read();
        let mut bgEventCount: u8 =
            ((((mapHeader).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_add(3)).read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((bgEventCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((bgEvents).wrapping_offset(((i) as i32) as isize * 12)).cast::<u16>())
                        .read()) as i32)
                        == ((x) as i32))
                        && ((((((bgEvents).wrapping_offset(((i) as i32) as isize * 12))
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32)
                            == ((y) as i32))
                    {
                        if ((((((bgEvents).wrapping_offset(((i) as i32) as isize * 12))
                            .wrapping_add(4))
                        .read()) as i32)
                            == ((elevation) as i32))
                            || ((((((bgEvents).wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(4))
                            .read()) as i32)
                                == 0i32)
                        {
                            return (bgEvents).wrapping_offset(((i) as i32) as isize * 12);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryDoDiveWarp(position: *mut u8, metatileBehavior: u16) -> u8 {
    unsafe {
        let mut position = position;
        let mut metatileBehavior = metatileBehavior;
        if ((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32) == 5i32)
            && (!((MetatileBehavior_IsUnableToEmerge(((metatileBehavior) as u8))) != 0))
        {
            if (SetDiveWarpEmerge(
                ((((((position).cast::<i16>()).read()) as i32).wrapping_sub(7i32)) as u16),
                ((((((position).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_sub(7i32))
                    as u16),
            )) != 0
            {
                StoreInitialPlayerAvatarState();
                DoDiveWarp();
                PlaySE(233u16);
                return 1u8;
            }
        } else {
            if ((MetatileBehavior_IsDiveable(((metatileBehavior) as u8))) as i32) == 1i32 {
                if (SetDiveWarpDive(
                    ((((((position).cast::<i16>()).read()) as i32).wrapping_sub(7i32)) as u16),
                    ((((((position).wrapping_add(2).cast::<i16>()).read()) as i32)
                        .wrapping_sub(7i32)) as u16),
                )) != 0
                {
                    StoreInitialPlayerAvatarState();
                    DoDiveWarp();
                    PlaySE(233u16);
                    return 1u8;
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetDiveWarp() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut metatileBehavior: u8 = 0u8;
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        metatileBehavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8);
        if ((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32) == 5i32)
            && (!((MetatileBehavior_IsUnableToEmerge(metatileBehavior)) != 0))
        {
            if ((SetDiveWarpEmerge(
                ((((x) as i32).wrapping_sub(7i32)) as u16),
                ((((y) as i32).wrapping_sub(7i32)) as u16),
            )) as i32)
                == 1i32
            {
                return 1u8;
            }
        } else {
            if ((MetatileBehavior_IsDiveable(metatileBehavior)) as i32) == 1i32 {
                if ((SetDiveWarpDive(
                    ((((x) as i32).wrapping_sub(7i32)) as u16),
                    ((((y) as i32).wrapping_sub(7i32)) as u16),
                )) as i32)
                    == 1i32
                {
                    return 2u8;
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetObjectEventScriptPointerPlayerFacing() -> *mut u8 {
    unsafe {
        let mut direction: u8 = 0u8;
        let mut position = crate::ffi::Align4([0u8; 8]);
        direction = GetPlayerMovementDirection();
        GetInFrontOfPlayerPosition((&raw mut position).cast::<u8>());
        return GetInteractedObjectEventScript(
            (&raw mut position).cast::<u8>(),
            ((MapGridGetMetatileBehaviorAt(
                (((((&raw mut position).cast::<u8>()).cast::<i16>()).read()) as i32),
                (((((&raw mut position).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32),
            )) as u8),
            direction,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCableClubWarp() -> i32 {
    unsafe {
        let mut position = crate::ffi::Align4([0u8; 8]);
        GetPlayerMovementDirection();
        GetPlayerPosition((&raw mut position).cast::<u8>());
        MapGridGetMetatileBehaviorAt(
            (((((&raw mut position).cast::<u8>()).cast::<i16>()).read()) as i32),
            (((((&raw mut position).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32),
        );
        SetupWarp(
            (&raw mut gMapHeader).cast::<u8>(),
            GetWarpEventAtMapPosition(
                (&raw mut gMapHeader).cast::<u8>(),
                (&raw mut position).cast::<u8>(),
            ),
            (&raw mut position).cast::<u8>(),
        );
        return 0i32;
    }
}
