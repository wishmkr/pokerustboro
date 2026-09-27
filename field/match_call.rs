//! Translated from `src/match_call.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMatchCallTrainers sMatchCallWildBattleTexts sMatchCallNegativeBattleTexts sMatchCallPositiveBattleTexts sMatchCallSameRouteBattleRequestTexts sMatchCallDifferentRouteBattleRequestTexts sMatchCallPersonalizedTexts sMatchCallBattleFrontierStreakTexts sMatchCallBattleFrontierRecordStreakTexts sMatchCallBattleDomeTexts sMatchCallBattlePikeTexts sMatchCallBattlePyramidTexts sMatchCallBattleTopics sMatchCallBattleRequestTopics sMatchCallGeneralTopics sMatchCallWindow_Pal sMatchCallWindow_Gfx sPokenavIcon_Pal sPokenavIcon_Gfx sText_PokenavCallEllipsis sMatchCallTaskFuncs sMatchCallTextWindow sMatchCallTextStringVars sPopulateMatchCallStringVarFuncs sMultiTrainerMatchCallTexts sBattleFrontierFacilityNames sBadgeFlags sBirchDexRatingTexts
#[allow(unused_imports)]
use crate::data::match_call::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMatchCallState: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleFrontierStreakInfo: crate::ffi::Align4<[u8; 4]> =
    crate::ffi::Align4([0; 4]);

unsafe extern "C" {
    static mut gBirchDexRatingText_AreYouCurious: u8;
    static mut gBirchDexRatingText_OnANationwideBasis: u8;
    static mut gBirchDexRatingText_SoYouveSeenAndCaught: u8;
    static mut gLocalTime: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gObjectEvents: u8;
    static mut gPlayerParty: u8;
    static mut gRematchTable: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpeciesNames: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gTrainers: u8;
    static mut gWildMonHeaders: u8;
    fn AddTextPrinter(a0: *mut u8, a1: u8, a2: Option<unsafe extern "C" fn(*mut u8, u16)>) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroyTask(a0: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FreezeObjectEvents();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBgAttribute(a0: u8, a1: u8) -> u16;
    fn GetGameStat(a0: u8) -> u32;
    fn GetHoennPokedexCount(a0: u8) -> u16;
    fn GetLastBeatenRematchTrainerId(a0: u16) -> u16;
    fn GetMapName(a0: *mut u8, a1: u16, a2: u16) -> *mut u8;
    fn GetMonAbility(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetNationalPokedexCount(a0: u8) -> u16;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetTrainerId(a0: *mut u8) -> u32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn HasTrainerBeenFought(a0: u16) -> u8;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsSEPlaying() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LockPlayerFieldControls();
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8;
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut u8;
    fn Overworld_MapTypeAllowsTeleportAndFly(a0: u8) -> u8;
    fn PlaySE(a0: u16);
    fn PlayerFreeze();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn RtcCalcLocalTime();
    fn RtcGetLocalDayCount() -> u32;
    fn RunTextPrinters();
    fn ScriptMovement_UnfreezeObjectEvents();
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StopPlayerAvatar();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn UnfreezeObjectEvents();
    fn UnlockPlayerFieldControls();
    fn UpdateRematchIfDefeated(a0: i32);
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
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMatchCallCounters() {
    unsafe {
        RtcCalcLocalTime();
        (((&raw mut sMatchCallState).cast::<u8>()).cast::<u32>()).write(
            (GetCurrentTotalMinutes((&raw mut gLocalTime).cast::<u8>())).wrapping_add(10u32),
        );
        (((&raw mut sMatchCallState).cast::<u8>()).wrapping_add(6)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn GetCurrentTotalMinutes(time: *mut u8) -> u32 {
    unsafe {
        let mut time = time;
        return (((((((((time).cast::<i16>()).read()) as i32).wrapping_mul(24i32))
            .wrapping_mul(60i32))
        .wrapping_add(((((time).wrapping_add(2).cast::<i8>()).read()) as i32).wrapping_mul(60i32)))
        .wrapping_add(((((time).wrapping_add(3).cast::<i8>()).read()) as i32)))
            as u32);
    }
}
pub(crate) unsafe extern "C" fn UpdateMatchCallMinutesCounter() -> u32 {
    unsafe {
        let mut curMinutes: i32 = 0i32;
        RtcCalcLocalTime();
        curMinutes = ((GetCurrentTotalMinutes((&raw mut gLocalTime).cast::<u8>())) as i32);
        if ((((&raw mut sMatchCallState).cast::<u8>()).cast::<u32>()).read()
            > ((curMinutes) as u32))
            || (((curMinutes) as u32)
                .wrapping_sub((((&raw mut sMatchCallState).cast::<u8>()).cast::<u32>()).read())
                > 9u32)
        {
            (((&raw mut sMatchCallState).cast::<u8>()).cast::<u32>()).write(((curMinutes) as u32));
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CheckMatchCallChance() -> u32 {
    unsafe {
        let mut callChance: i32 = 1i32;
        if (!((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 6i32)) != 0))
            && (((GetMonAbility((&raw mut gPlayerParty).cast::<u8>())) as i32) == 31i32)
        {
            callChance = 2i32;
        }
        if crate::c::rem_i32(((Random()) as i32), 10i32) < (callChance).wrapping_mul(3i32) {
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
pub(crate) unsafe extern "C" fn MapAllowsMatchCall() -> u32 {
    unsafe {
        if (!((Overworld_MapTypeAllowsTeleportAndFly(
            (((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read(),
        )) != 0))
            || ((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32) == 57i32)
        {
            return 0u32;
        }
        if (((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32) == 14i32)
            && (((FlagGet(996u16)) as i32) == 1i32))
            && (((FlagGet(220u16)) as i32) == 0i32)
        {
            return 0u32;
        }
        if (((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32) == 56i32)
            && (((FlagGet(207u16)) as i32) == 1i32))
            && (((FlagGet(139u16)) as i32) == 0i32)
        {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn UpdateMatchCallStepCounter() -> u32 {
    unsafe {
        if (({
            let __p1 = ((&raw mut sMatchCallState).cast::<u8>()).wrapping_add(6);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 10i32
        {
            (((&raw mut sMatchCallState).cast::<u8>()).wrapping_add(6)).write(0u8);
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
pub(crate) unsafe extern "C" fn SelectMatchCallTrainer() -> u32 {
    unsafe {
        let mut matchCallId: u32 = 0u32;
        let mut numRegistered: u32 = GetNumRegisteredTrainers();
        if numRegistered == 0u32 {
            return 0u32;
        }
        (((&raw mut sMatchCallState).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(
            ((GetActiveMatchCallTrainerId(crate::c::rem_u32(((Random()) as u32), numRegistered)))
                as u16),
        );
        (((&raw mut sMatchCallState).cast::<u8>()).wrapping_add(7)).write(0u8);
        if (((((&raw mut sMatchCallState).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as i32)
            == 78i32
        {
            return 0u32;
        }
        matchCallId = ((GetTrainerMatchCallId(
            (((((&raw mut sMatchCallState).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32),
        )) as u32);
        if (((GetRematchTrainerLocation(((matchCallId) as i32))) as i32)
            == (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32))
            && (!((TrainerIsEligibleForRematch(((matchCallId) as i32))) != 0))
        {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GetNumRegisteredTrainers() -> u32 {
    unsafe {
        let mut i: u32 = 0u32;
        let mut count: u32 = 0u32;
        {
            i = 0u32;
            count = 0u32;
            'l1: loop {
                if !(i < 64u32) {
                    break 'l1;
                }
                'l2: {
                    if (FlagGet((((348u32).wrapping_add(i)) as u16))) != 0 {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
pub(crate) unsafe extern "C" fn GetActiveMatchCallTrainerId(activeMatchCallId: u32) -> u32 {
    unsafe {
        let mut activeMatchCallId = activeMatchCallId;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < 64u32) {
                    break 'l1;
                }
                'l2: {
                    if (FlagGet((((348u32).wrapping_add(i)) as u16))) != 0 {
                        if !((activeMatchCallId) != 0) {
                            return ((((((&raw mut gRematchTable).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 16))
                            .cast::<u16>())
                            .read()) as u32);
                        }
                        activeMatchCallId = (activeMatchCallId).wrapping_sub(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 78u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryStartMatchCall() -> u32 {
    unsafe {
        if ((((((FlagGet(303u16)) != 0) && ((UpdateMatchCallStepCounter()) != 0))
            && ((UpdateMatchCallMinutesCounter()) != 0))
            && ((CheckMatchCallChance()) != 0))
            && ((MapAllowsMatchCall()) != 0))
            && ((SelectMatchCallTrainer()) != 0)
        {
            StartMatchCall();
            return 1u32;
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMatchCallFromScript(message: *mut u8) {
    unsafe {
        let mut message = message;
        (((&raw mut sMatchCallState).cast::<u8>()).wrapping_add(7)).write(1u8);
        StartMatchCall();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMatchCallTaskActive() -> u32 {
    unsafe {
        return ((FuncIsActiveTask(Some(ExecuteMatchCall))) as u32);
    }
}
pub(crate) unsafe extern "C" fn StartMatchCall() {
    unsafe {
        if !(((((&raw mut sMatchCallState).cast::<u8>()).wrapping_add(7)).read()) != 0) {
            LockPlayerFieldControls();
            FreezeObjectEvents();
            PlayerFreeze();
            StopPlayerAvatar();
        }
        PlaySE(263u16);
        CreateTask(Some(ExecuteMatchCall), 1u8);
    }
}
pub(crate) unsafe extern "C" fn ExecuteMatchCall(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((((&raw const sMatchCallTaskFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(u8) -> u32>>())
        .cast::<Option<unsafe extern "C" fn(u8) -> u32>>())
        .wrapping_offset((((data).read()) as i32) as isize))
        .read())
        .unwrap_unchecked()(taskId))
            != 0
        {
            (data).write(((data).read()).wrapping_add(1));
            ((data).wrapping_offset(1)).write(0i16);
            if ((((data).read()) as u16) as i32) > 7i32 {
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MatchCall_LoadGfx(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(2))
            .write(((AddWindow((&raw const sMatchCallTextWindow).cast::<u8>().cast_mut())) as i16));
        if ((((data).wrapping_offset(2)).read()) as i32) == 255i32 {
            DestroyTask(taskId);
            return 0u32;
        }
        if ((LoadBgTiles(
            0u8,
            ((&raw const sMatchCallWindow_Gfx).cast::<u8>().cast_mut()).cast::<u8>(),
            256u16,
            624u16,
        )) as i32)
            == 65535i32
        {
            RemoveWindow(((((data).wrapping_offset(2)).read()) as u8));
            DestroyTask(taskId);
            return 0u32;
        }
        if !(!(DecompressAndCopyTileDataToVram(
            0u8,
            (((&raw const sPokenavIcon_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u32,
            633u16,
            0u8,
        ))
        .is_null())
        {
            RemoveWindow(((((data).wrapping_offset(2)).read()) as u8));
            DestroyTask(taskId);
            return 0u32;
        }
        FillWindowPixelBuffer(((((data).wrapping_offset(2)).read()) as u8), 136u8);
        LoadPalette(
            (((&raw const sMatchCallWindow_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            224u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sPokenavIcon_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        ChangeBgY(0u8, (-8192i32), 0u8);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_DrawWindow(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (FreeTempTileDataBuffersIfPossible()) != 0 {
            return 0u32;
        }
        PutWindowTilemap(((((data).wrapping_offset(2)).read()) as u8));
        DrawMatchCallTextBoxBorder_Internal(
            ((((data).wrapping_offset(2)).read()) as u32),
            624u32,
            14u32,
        );
        WriteSequenceToBgTilemapBuffer(0u8, 62073u16, 1u8, 15u8, 4u8, 4u8, 17u8, 1i16);
        ((data).wrapping_offset(5)).write(((CreateTask(Some(Task_SpinPokenavIcon), 10u8)) as i16));
        CopyWindowToVram(((((data).wrapping_offset(2)).read()) as u8), 2u8);
        CopyBgTilemapBufferToVram(0u8);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_ReadyIntro(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
            InitMatchCallTextPrinter(
                ((((data).wrapping_offset(2)).read()) as i32),
                ((&raw const sText_PokenavCallEllipsis)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_SlideWindowIn(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        if ChangeBgY(0u8, 1536i32, 1u8) >= 0i32 {
            ChangeBgY(0u8, 0i32, 0u8);
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_PrintIntro(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((RunMatchCallTextPrinter(((((data).wrapping_offset(2)).read()) as i32))) != 0) {
            FillWindowPixelBuffer(((((data).wrapping_offset(2)).read()) as u8), 136u8);
            if !(((((&raw mut sMatchCallState).cast::<u8>()).wrapping_add(7)).read()) != 0) {
                SelectMatchCallMessage(
                    (((((&raw mut sMatchCallState).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read()) as i32),
                    (&raw mut gStringVar4).cast::<u8>(),
                );
            }
            InitMatchCallTextPrinter(
                ((((data).wrapping_offset(2)).read()) as i32),
                (&raw mut gStringVar4).cast::<u8>(),
            );
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_PrintMessage(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((!((RunMatchCallTextPrinter(((((data).wrapping_offset(2)).read()) as i32))) != 0))
            && (!((IsSEPlaying()) != 0)))
            && (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 3i32)
                != 0)
        {
            FillWindowPixelBuffer(((((data).wrapping_offset(2)).read()) as u8), 136u8);
            CopyWindowToVram(((((data).wrapping_offset(2)).read()) as u8), 2u8);
            PlaySE(264u16);
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_SlideWindowOut(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ChangeBgY(0u8, 1536i32, 2u8) <= (-8192i32) {
            FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 14u8, 30u8, 6u8);
            DestroyTask(((((data).wrapping_offset(5)).read()) as u8));
            RemoveWindow(((((data).wrapping_offset(2)).read()) as u8));
            CopyBgTilemapBufferToVram(0u8);
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCall_EndCall(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut playerObjectId: u8 = 0u8;
        if (!((IsDma3ManagerBusyWithBgCopy()) != 0)) && (!((IsSEPlaying()) != 0)) {
            ChangeBgY(0u8, 0i32, 0u8);
            if !(((((&raw mut sMatchCallState).cast::<u8>()).wrapping_add(7)).read()) != 0) {
                LoadMessageBoxAndBorderGfx();
                playerObjectId = GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8);
                ObjectEventClearHeldMovementIfFinished(
                    ((&raw mut gObjectEvents).cast::<u8>())
                        .wrapping_offset(((playerObjectId) as i32) as isize * 36),
                );
                ScriptMovement_UnfreezeObjectEvents();
                UnfreezeObjectEvents();
                UnlockPlayerFieldControls();
            }
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn DrawMatchCallTextBoxBorder_Internal(
    windowId: u32,
    tileOffset: u32,
    paletteId: u32,
) {
    unsafe {
        let mut windowId = windowId;
        let mut tileOffset = tileOffset;
        let mut paletteId = paletteId;
        let mut bg: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut width: i32 = 0i32;
        let mut height: i32 = 0i32;
        let mut tileNum: i32 = 0i32;
        bg = ((GetWindowAttribute(((windowId) as u8), 0u8)) as i32);
        x = ((GetWindowAttribute(((windowId) as u8), 1u8)) as i32);
        y = ((GetWindowAttribute(((windowId) as u8), 2u8)) as i32);
        width = ((GetWindowAttribute(((windowId) as u8), 3u8)) as i32);
        height = ((GetWindowAttribute(((windowId) as u8), 4u8)) as i32);
        tileNum =
            (((tileOffset).wrapping_add(((GetBgAttribute(((bg) as u8), 10u8)) as u32))) as i32);
        FillBgTilemapBufferRect_Palette0(
            ((bg) as u8),
            ((((paletteId << 12) & 61440u32) | (((tileNum).wrapping_add(0i32)) as u32)) as u16),
            (((x).wrapping_sub(1i32)) as u8),
            (((y).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
        );
        FillBgTilemapBufferRect_Palette0(
            ((bg) as u8),
            ((((paletteId << 12) & 61440u32) | (((tileNum).wrapping_add(1i32)) as u32)) as u16),
            ((x) as u8),
            (((y).wrapping_sub(1i32)) as u8),
            ((width) as u8),
            1u8,
        );
        FillBgTilemapBufferRect_Palette0(
            ((bg) as u8),
            ((((paletteId << 12) & 61440u32) | (((tileNum).wrapping_add(2i32)) as u32)) as u16),
            (((x).wrapping_add(width)) as u8),
            (((y).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
        );
        FillBgTilemapBufferRect_Palette0(
            ((bg) as u8),
            ((((paletteId << 12) & 61440u32) | (((tileNum).wrapping_add(3i32)) as u32)) as u16),
            (((x).wrapping_sub(1i32)) as u8),
            ((y) as u8),
            1u8,
            ((height) as u8),
        );
        FillBgTilemapBufferRect_Palette0(
            ((bg) as u8),
            ((((paletteId << 12) & 61440u32) | (((tileNum).wrapping_add(4i32)) as u32)) as u16),
            (((x).wrapping_add(width)) as u8),
            ((y) as u8),
            1u8,
            ((height) as u8),
        );
        FillBgTilemapBufferRect_Palette0(
            ((bg) as u8),
            ((((paletteId << 12) & 61440u32) | (((tileNum).wrapping_add(5i32)) as u32)) as u16),
            (((x).wrapping_sub(1i32)) as u8),
            (((y).wrapping_add(height)) as u8),
            1u8,
            1u8,
        );
        FillBgTilemapBufferRect_Palette0(
            ((bg) as u8),
            ((((paletteId << 12) & 61440u32) | (((tileNum).wrapping_add(6i32)) as u32)) as u16),
            ((x) as u8),
            (((y).wrapping_add(height)) as u8),
            ((width) as u8),
            1u8,
        );
        FillBgTilemapBufferRect_Palette0(
            ((bg) as u8),
            ((((paletteId << 12) & 61440u32) | (((tileNum).wrapping_add(7i32)) as u32)) as u16),
            (((x).wrapping_add(width)) as u8),
            (((y).wrapping_add(height)) as u8),
            1u8,
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn InitMatchCallTextPrinter(windowId: i32, str: *mut u8) {
    unsafe {
        let mut windowId = windowId;
        let mut str = str;
        let mut printerTemplate = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printerTemplate).cast::<u8>()).cast::<*mut u8>()).write(str);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(4)).write(((windowId) as u8));
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(5)).write(1u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(6)).write(32u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(7)).write(1u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(8)).write(32u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(9)).write(1u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (10u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (8u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (14u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            1,
            1,
            (0u8) as i32,
        );
        AddTextPrinter(
            (&raw mut printerTemplate).cast::<u8>(),
            GetPlayerTextSpeedDelay(),
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn RunMatchCallTextPrinter(windowId: i32) -> u32 {
    unsafe {
        let mut windowId = windowId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                0,
                1,
                (1u8) as i32,
            );
        } else {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                0,
                1,
                (0u8) as i32,
            );
        }
        RunTextPrinters();
        return ((IsTextPrinterActive(((windowId) as u8))) as u32);
    }
}
pub(crate) unsafe extern "C" fn Task_SpinPokenavIcon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (({
            let __t1 = ((data).read()).wrapping_add(1);
            (data).write(__t1);
            __t1
        }) as i32)
            > 8i32
        {
            (data).write(0i16);
            if (({
                let __p2 = (data).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                > 7i32
            {
                ((data).wrapping_offset(1)).write(0i16);
            }
            ((data).wrapping_offset(2)).write(
                (((((((data).wrapping_offset(1)).read()) as i32).wrapping_mul(16i32))
                    .wrapping_add(633i32)) as i16),
            );
            WriteSequenceToBgTilemapBuffer(
                0u8,
                ((((((data).wrapping_offset(2)).read()) as i32) | (-4096i32)) as u16),
                1u8,
                15u8,
                4u8,
                4u8,
                17u8,
                1i16,
            );
            CopyBgTilemapBufferToVram(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerIsEligibleForRematch(matchCallId: i32) -> u32 {
    unsafe {
        let mut matchCallId = matchCallId;
        return ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2506))
            .cast::<u8>())
        .wrapping_offset((matchCallId) as isize))
        .read()) as i32)
            > 0i32) as u32);
    }
}
pub(crate) unsafe extern "C" fn GetRematchTrainerLocation(matchCallId: i32) -> u16 {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut mapHeader: *mut u8 = Overworld_GetMapHeaderByGroupAndId(
            ((((&raw mut gRematchTable).cast::<u8>())
                .wrapping_offset((matchCallId) as isize * 16))
            .wrapping_add(10)
            .cast::<u16>())
            .read(),
            ((((&raw mut gRematchTable).cast::<u8>())
                .wrapping_offset((matchCallId) as isize * 16))
            .wrapping_add(12)
            .cast::<u16>())
            .read(),
        );
        return ((((mapHeader).wrapping_add(20)).read()) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetNumRematchTrainersFought() -> u32 {
    unsafe {
        let mut i: u32 = 0u32;
        let mut count: u32 = 0u32;
        {
            i = 0u32;
            count = 0u32;
            'l1: loop {
                if !(i < 64u32) {
                    break 'l1;
                }
                'l2: {
                    if (HasTrainerBeenFought(
                        ((((&raw mut gRematchTable).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 16))
                        .cast::<u16>())
                        .read(),
                    )) != 0
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
pub(crate) unsafe extern "C" fn GetNthRematchTrainerFought(n: i32) -> u32 {
    unsafe {
        let mut n = n;
        let mut i: u32 = 0u32;
        let mut count: u32 = 0u32;
        {
            i = 0u32;
            count = 0u32;
            'l1: loop {
                if !(i < 78u32) {
                    break 'l1;
                }
                'l2: {
                    if (HasTrainerBeenFought(
                        ((((&raw mut gRematchTable).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 16))
                        .cast::<u16>())
                        .read(),
                    )) != 0
                    {
                        if count == ((n) as u32) {
                            return i;
                        }
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 78u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SelectMatchCallMessage(trainerId: i32, str: *mut u8) -> u32 {
    unsafe {
        let mut trainerId = trainerId;
        let mut str = str;
        let mut matchCallId: u32 = 0u32;
        let mut matchCallText: *mut u8 = core::ptr::null_mut();
        let mut newRematchRequest: u32 = 0u32;
        matchCallId = ((GetTrainerMatchCallId(trainerId)) as u32);
        (((&raw mut sBattleFrontierStreakInfo).cast::<u8>()).cast::<u16>()).write(0u16);
        if ((TrainerIsEligibleForRematch(((matchCallId) as i32))) != 0)
            && (((GetRematchTrainerLocation(((matchCallId) as i32))) as i32)
                == (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32))
        {
            matchCallText = GetSameRouteMatchCallText(((matchCallId) as i32), str);
        } else {
            if (ShouldTrainerRequestBattle(((matchCallId) as i32))) != 0 {
                matchCallText = GetDifferentRouteMatchCallText(((matchCallId) as i32), str);
                newRematchRequest = 1u32;
                UpdateRematchIfDefeated(((matchCallId) as i32));
            } else {
                if (crate::c::rem_i32(((Random()) as i32), 3i32)) != 0 {
                    matchCallText = GetBattleMatchCallText(((matchCallId) as i32), str);
                } else {
                    matchCallText = GetGeneralMatchCallText(((matchCallId) as i32), str);
                }
            }
        }
        BuildMatchCallString(((matchCallId) as i32), matchCallText, str);
        return newRematchRequest;
    }
}
pub(crate) unsafe extern "C" fn GetTrainerMatchCallId(trainerId: i32) -> i32 {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if (((((((&raw const sMatchCallTrainers).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((i) as isize * 20))
            .cast::<u16>())
            .read()) as i32)
                == trainerId
            {
                return i;
            } else {
                i = (i).wrapping_add(1);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetSameRouteMatchCallText(
    matchCallId: i32,
    str: *mut u8,
) -> *mut u8 {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut str = str;
        let mut textId: u16 = (((((&raw const sMatchCallTrainers).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((matchCallId) as isize * 20))
        .wrapping_add(14)
        .cast::<u16>())
        .read();
        let mut mask: i32 = 255i32;
        let mut topic: u32 = (((((textId) as i32) >> 8).wrapping_sub(1i32)) as u32);
        let mut id: u32 = (((((textId) as i32) & mask).wrapping_sub(1i32)) as u32);
        return (((((&raw const sMatchCallBattleRequestTopics)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((topic) as i32) as isize))
        .read())
        .wrapping_offset(((id) as i32) as isize * 8);
    }
}
pub(crate) unsafe extern "C" fn GetDifferentRouteMatchCallText(
    matchCallId: i32,
    str: *mut u8,
) -> *mut u8 {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut str = str;
        let mut textId: u16 = (((((&raw const sMatchCallTrainers).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((matchCallId) as isize * 20))
        .wrapping_add(16)
        .cast::<u16>())
        .read();
        let mut mask: i32 = 255i32;
        let mut topic: u32 = (((((textId) as i32) >> 8).wrapping_sub(1i32)) as u32);
        let mut id: u32 = (((((textId) as i32) & mask).wrapping_sub(1i32)) as u32);
        return (((((&raw const sMatchCallBattleRequestTopics)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((topic) as i32) as isize))
        .read())
        .wrapping_offset(((id) as i32) as isize * 8);
    }
}
pub(crate) unsafe extern "C" fn GetBattleMatchCallText(matchCallId: i32, str: *mut u8) -> *mut u8 {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut str = str;
        let mut mask: i32 = 0i32;
        let mut textId: u32 = 0u32;
        let mut topic: u32 = 0u32;
        let mut id: u32 = 0u32;
        topic = ((crate::c::rem_i32(((Random()) as i32), 3i32)) as u32);
        textId = (((((((((&raw const sMatchCallTrainers).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((matchCallId) as isize * 20))
        .wrapping_add(4))
        .cast::<u16>())
        .wrapping_offset(((topic) as i32) as isize))
        .read()) as u32);
        if !((textId) != 0) {
            SpriteCallbackDummy(core::ptr::null_mut());
        }
        mask = 255i32;
        topic = (textId >> 8).wrapping_sub(1u32);
        id = (textId & ((mask) as u32)).wrapping_sub(1u32);
        return (((((&raw const sMatchCallBattleTopics)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((topic) as i32) as isize))
        .read())
        .wrapping_offset(((id) as i32) as isize * 8);
    }
}
pub(crate) unsafe extern "C" fn GetGeneralMatchCallText(matchCallId: i32, str: *mut u8) -> *mut u8 {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut str = str;
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        let mut topic: u32 = 0u32;
        let mut id: u32 = 0u32;
        let mut rand: u16 = 0u16;
        rand = Random();
        if !((((rand) as i32) & 1i32) != 0) {
            {
                count = 0i32;
                i = 0i32;
                'l1: loop {
                    if !(i < 7i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((GetFrontierStreakInfo(((i) as u16), &raw mut topic)) as i32) > 1i32 {
                            count = (count).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (count) != 0 {
                count = crate::c::rem_i32(((Random()) as i32), count);
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 7i32) {
                            break 'l3;
                        }
                        'l4: {
                            (((&raw mut sBattleFrontierStreakInfo).cast::<u8>())
                                .wrapping_add(2)
                                .cast::<u16>())
                            .write(GetFrontierStreakInfo(((i) as u16), &raw mut topic));
                            if (((((&raw mut sBattleFrontierStreakInfo).cast::<u8>())
                                .wrapping_add(2)
                                .cast::<u16>())
                            .read()) as i32)
                                < 2i32
                            {
                                break 'l4;
                            }
                            if !((count) != 0) {
                                break 'l3;
                            }
                            count = (count).wrapping_sub(1);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (((&raw mut sBattleFrontierStreakInfo).cast::<u8>()).cast::<u16>())
                    .write(((i) as u16));
                id = (((((((((&raw const sMatchCallTrainers).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset((matchCallId) as isize * 20))
                .wrapping_add(12))
                .read()) as i32)
                    .wrapping_sub(1i32)) as u32);
                return (((((&raw const sMatchCallGeneralTopics)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((topic) as i32) as isize))
                .read())
                .wrapping_offset(((id) as i32) as isize * 8);
            }
        }
        topic = ((((((((((&raw const sMatchCallTrainers).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((matchCallId) as isize * 20))
        .wrapping_add(10)
        .cast::<u16>())
        .read()) as i32)
            >> 8)
            .wrapping_sub(1i32)) as u32);
        id = ((((((((((&raw const sMatchCallTrainers).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((matchCallId) as isize * 20))
        .wrapping_add(10)
        .cast::<u16>())
        .read()) as i32)
            & 255i32)
            .wrapping_sub(1i32)) as u32);
        return (((((&raw const sMatchCallGeneralTopics)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((topic) as i32) as isize))
        .read())
        .wrapping_offset(((id) as i32) as isize * 8);
    }
}
pub(crate) unsafe extern "C" fn BuildMatchCallString(
    matchCallId: i32,
    matchCallText: *mut u8,
    str: *mut u8,
) {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut matchCallText = matchCallText;
        let mut str = str;
        PopulateMatchCallStringVars(matchCallId, ((matchCallText).wrapping_add(4)).cast::<i8>());
        StringExpandPlaceholders(str, ((matchCallText).cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn PopulateMatchCallStringVars(
    matchCallId: i32,
    stringVarFuncIds: *mut i8,
) {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut stringVarFuncIds = stringVarFuncIds;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((stringVarFuncIds).wrapping_offset((i) as isize)).read()) as i32) >= 0i32
                    {
                        PopulateMatchCallStringVar(
                            matchCallId,
                            ((((stringVarFuncIds).wrapping_offset((i) as isize)).read()) as i32),
                            ((((&raw const sMatchCallTextStringVars)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PopulateMatchCallStringVar(
    matchCallId: i32,
    funcId: i32,
    destStr: *mut u8,
) {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut funcId = funcId;
        let mut destStr = destStr;
        (((((&raw const sPopulateMatchCallStringVarFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(i32, *mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(i32, *mut u8)>>())
        .wrapping_offset((funcId) as isize))
        .read())
        .unwrap_unchecked()(matchCallId, destStr);
    }
}
pub(crate) unsafe extern "C" fn PopulateTrainerName(matchCallId: i32, destStr: *mut u8) {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut destStr = destStr;
        let mut i: u32 = 0u32;
        let mut trainerId: u16 = (((((&raw const sMatchCallTrainers).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((matchCallId) as isize * 20))
        .cast::<u16>())
        .read();
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(48u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sMultiTrainerMatchCallTexts)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((trainerId) as i32)
                    {
                        StringCopy(
                            destStr,
                            (((((&raw const sMultiTrainerMatchCallTexts)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read(),
                        );
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy(
            destStr,
            ((((&raw mut gTrainers).cast::<u8>())
                .wrapping_offset(((trainerId) as i32) as isize * 40))
            .wrapping_add(4))
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn PopulateMapName(matchCallId: i32, destStr: *mut u8) {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut destStr = destStr;
        GetMapName(destStr, GetRematchTrainerLocation(matchCallId), 0u16);
    }
}
pub(crate) unsafe extern "C" fn GetLandEncounterSlot() -> u8 {
    unsafe {
        let mut rand: i32 = crate::c::rem_i32(((Random()) as i32), 100i32);
        if rand < 20i32 {
            return 0u8;
        } else {
            if (rand >= 20i32) && (rand < 40i32) {
                return 1u8;
            } else {
                if (rand >= 40i32) && (rand < 50i32) {
                    return 2u8;
                } else {
                    if (rand >= 50i32) && (rand < 60i32) {
                        return 3u8;
                    } else {
                        if (rand >= 60i32) && (rand < 70i32) {
                            return 4u8;
                        } else {
                            if (rand >= 70i32) && (rand < 80i32) {
                                return 5u8;
                            } else {
                                if (rand >= 80i32) && (rand < 85i32) {
                                    return 6u8;
                                } else {
                                    if (rand >= 85i32) && (rand < 90i32) {
                                        return 7u8;
                                    } else {
                                        if (rand >= 90i32) && (rand < 94i32) {
                                            return 8u8;
                                        } else {
                                            if (rand >= 94i32) && (rand < 98i32) {
                                                return 9u8;
                                            } else {
                                                if (rand >= 98i32) && (rand < 99i32) {
                                                    return 10u8;
                                                } else {
                                                    return 11u8;
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
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetWaterEncounterSlot() -> u8 {
    unsafe {
        let mut rand: i32 = crate::c::rem_i32(((Random()) as i32), 100i32);
        if rand < 60i32 {
            return 0u8;
        } else {
            if (rand >= 60i32) && (rand < 90i32) {
                return 1u8;
            } else {
                if (rand >= 90i32) && (rand < 95i32) {
                    return 2u8;
                } else {
                    if (rand >= 95i32) && (rand < 99i32) {
                        return 3u8;
                    } else {
                        return 4u8;
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
pub(crate) unsafe extern "C" fn PopulateSpeciesFromTrainerLocation(
    matchCallId: i32,
    destStr: *mut u8,
) {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut destStr = destStr;
        let mut species = crate::ffi::Align4([0u8; 4]);
        let mut numSpecies: i32 = 0i32;
        let mut slot: u8 = 0u8;
        let mut i: i32 = 0i32;
        if (((((&raw mut gWildMonHeaders).cast::<u8>()).wrapping_offset((i) as isize * 20)).read())
            as i32)
            != 255i32
        {
            'l1: loop {
                if !((((((&raw mut gWildMonHeaders).cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                .read()) as i32)
                    != 255i32)
                {
                    break 'l1;
                }
                if ((((((&raw mut gWildMonHeaders).cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                .read()) as i32)
                    == ((((((&raw mut gRematchTable).cast::<u8>())
                        .wrapping_offset((matchCallId) as isize * 16))
                    .wrapping_add(10)
                    .cast::<u16>())
                    .read()) as i32))
                    && (((((((&raw mut gWildMonHeaders).cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                    .wrapping_add(1))
                    .read()) as i32)
                        == ((((((&raw mut gRematchTable).cast::<u8>())
                            .wrapping_offset((matchCallId) as isize * 16))
                        .wrapping_add(12)
                        .cast::<u16>())
                        .read()) as i32))
                {
                    break 'l1;
                }
                i = (i).wrapping_add(1);
            }
            if (((((&raw mut gWildMonHeaders).cast::<u8>()).wrapping_offset((i) as isize * 20))
                .read()) as i32)
                != 255i32
            {
                numSpecies = 0i32;
                if !(((((&raw mut gWildMonHeaders).cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .is_null()
                {
                    slot = GetLandEncounterSlot();
                    (((&raw mut species).cast::<u16>()).wrapping_offset((numSpecies) as isize))
                        .write(
                            (((((((((&raw mut gWildMonHeaders).cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((slot) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read(),
                        );
                    numSpecies = (numSpecies).wrapping_add(1);
                }
                if !(((((&raw mut gWildMonHeaders).cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                .wrapping_add(8)
                .cast::<*mut u8>())
                .read())
                .is_null()
                {
                    slot = GetWaterEncounterSlot();
                    (((&raw mut species).cast::<u16>()).wrapping_offset((numSpecies) as isize))
                        .write(
                            (((((((((&raw mut gWildMonHeaders).cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((slot) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read(),
                        );
                    numSpecies = (numSpecies).wrapping_add(1);
                }
                if (numSpecies) != 0 {
                    StringCopy(
                        destStr,
                        (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                            (((((&raw mut species).cast::<u16>()).wrapping_offset(
                                (crate::c::rem_i32(((Random()) as i32), numSpecies)) as isize,
                            ))
                            .read()) as i32) as isize
                                * 11,
                        ))
                        .cast::<u8>(),
                    );
                    return;
                }
            }
        }
        (destStr).write(255u8);
    }
}
pub(crate) unsafe extern "C" fn PopulateSpeciesFromTrainerParty(
    matchCallId: i32,
    destStr: *mut u8,
) {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut destStr = destStr;
        let mut trainerId: u16 = 0u16;
        let mut party = crate::ffi::Align4([0u8; 4]);
        let mut monId: u8 = 0u8;
        let mut speciesName: *mut u8 = core::ptr::null_mut();
        trainerId = GetLastBeatenRematchTrainerId(
            (((((&raw const sMatchCallTrainers).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((matchCallId) as isize * 20))
            .cast::<u16>())
            .read(),
        );
        (&raw mut party)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (((&raw mut gTrainers).cast::<u8>())
                    .wrapping_offset(((trainerId) as i32) as isize * 40))
                .wrapping_add(36)
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
            );
        monId = ((crate::c::rem_i32(
            ((Random()) as i32),
            ((((((&raw mut gTrainers).cast::<u8>())
                .wrapping_offset(((trainerId) as i32) as isize * 40))
            .wrapping_add(32))
            .read()) as i32),
        )) as u8);
        'l1: {
            let __sw1 = (((((&raw mut gTrainers).cast::<u8>())
                .wrapping_offset(((trainerId) as i32) as isize * 40))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 || !__matched {
                speciesName = (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut party).cast::<u8>()).cast::<*mut u8>()).read())
                        .wrapping_offset(((monId) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 11,
                ))
                .cast::<u8>();
                break 'l1;
            }
            if __sw1 == 1i32 {
                speciesName = (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut party).cast::<u8>()).cast::<*mut u8>()).read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 11,
                ))
                .cast::<u8>();
                break 'l1;
            }
            if __sw1 == 2i32 {
                speciesName = (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut party).cast::<u8>()).cast::<*mut u8>()).read())
                        .wrapping_offset(((monId) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 11,
                ))
                .cast::<u8>();
                break 'l1;
            }
            if __sw1 == 3i32 {
                speciesName = (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut party).cast::<u8>()).cast::<*mut u8>()).read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 11,
                ))
                .cast::<u8>();
                break 'l1;
            }
        }
        StringCopy(destStr, speciesName);
    }
}
pub(crate) unsafe extern "C" fn PopulateBattleFrontierFacilityName(
    matchCallId: i32,
    destStr: *mut u8,
) {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut destStr = destStr;
        StringCopy(
            destStr,
            ((((&raw const sBattleFrontierFacilityNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                (((((&raw mut sBattleFrontierStreakInfo).cast::<u8>()).cast::<u16>()).read())
                    as i32) as isize,
            ))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn PopulateBattleFrontierStreak(matchCallId: i32, destStr: *mut u8) {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut destStr = destStr;
        let mut i: i32 = 0i32;
        let mut streak: i32 = (((((&raw mut sBattleFrontierStreakInfo).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32);
        'l1: loop {
            if !(streak != 0i32) {
                break 'l1;
            }
            streak = crate::c::div_i32(streak, 10i32);
            i = (i).wrapping_add(1);
        }
        ConvertIntToDecimalStringN(
            destStr,
            (((((&raw mut sBattleFrontierStreakInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32),
            0i32,
            ((i) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn GetNumOwnedBadges() -> i32 {
    unsafe {
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < 8u32) {
                    break 'l1;
                }
                'l2: {
                    if !((FlagGet(
                        ((((&raw const sBadgeFlags)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    )) != 0)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((i) as i32);
    }
}
pub(crate) unsafe extern "C" fn ShouldTrainerRequestBattle(matchCallId: i32) -> u32 {
    unsafe {
        let mut matchCallId = matchCallId;
        let mut dayCount: i32 = 0i32;
        let mut otId: i32 = 0i32;
        let mut dewfordRand: u16 = 0u16;
        let mut numRematchTrainersFought: i32 = 0i32;
        let mut max: i32 = 0i32;
        let mut rand: i32 = 0i32;
        let mut n: i32 = 0i32;
        if GetNumOwnedBadges() < 5i32 {
            return 0u32;
        }
        dayCount = ((RtcGetLocalDayCount()) as i32);
        otId = ((GetTrainerId(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10)).cast::<u8>(),
        ) & 65535u32) as i32);
        dewfordRand = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11880))
        .cast::<u8>())
        .wrapping_add(2)
        .cast::<u16>())
        .read();
        numRematchTrainersFought = ((GetNumRematchTrainersFought()) as i32);
        max = crate::c::div_i32((numRematchTrainersFought).wrapping_mul(13i32), 10i32);
        rand = ((((dayCount ^ ((dewfordRand) as i32)) as u32)
            .wrapping_add((((dewfordRand) as u32) ^ GetGameStat(9u8)))
            ^ ((otId) as u32)) as i32);
        n = crate::c::rem_i32(rand, max);
        if n < numRematchTrainersFought {
            if GetNthRematchTrainerFought(n) == ((matchCallId) as u32) {
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetFrontierStreakInfo(
    facilityId: u16,
    topicTextId: *mut u32,
) -> u16 {
    unsafe {
        let mut facilityId = facilityId;
        let mut topicTextId = topicTextId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut streak: u16 = 0u16;
        'l1: {
            let __sw1 = ((facilityId) as i32);
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            {
                                j = 0i32;
                                'l4: loop {
                                    if !(j < 2i32) {
                                        break 'l4;
                                    }
                                    'l5: {
                                        if ((streak) as i32)
                                            < (((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1736))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                        {
                                            streak = (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1736))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read();
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (topicTextId).write(3u32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                {
                    i = 0i32;
                    'l6: loop {
                        if !(i < 2i32) {
                            break 'l6;
                        }
                        'l7: {
                            if ((streak) as i32)
                                < (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1980))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                            {
                                streak = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(1980))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read();
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (topicTextId).write(4u32);
                break 'l1;
            }
            if __sw1 == 0i32 {
                {
                    i = 0i32;
                    'l8: loop {
                        if !(i < ((crate::c::div_u32(16u32, 4u32)) as i32)) {
                            break 'l8;
                        }
                        'l9: {
                            {
                                j = 0i32;
                                'l10: loop {
                                    if !(j < 2i32) {
                                        break 'l10;
                                    }
                                    'l11: {
                                        if ((streak) as i32)
                                            < (((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1700))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                        {
                                            streak = (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1700))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read();
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (topicTextId).write(2u32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l12: loop {
                        if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                            break 'l12;
                        }
                        'l13: {
                            {
                                j = 0i32;
                                'l14: loop {
                                    if !(j < 2i32) {
                                        break 'l14;
                                    }
                                    'l15: {
                                        if ((streak) as i32)
                                            < (((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1924))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                        {
                                            streak = (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1924))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read();
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (topicTextId).write(2u32);
                break 'l1;
            }
            if __sw1 == 5i32 {
                {
                    i = 0i32;
                    'l16: loop {
                        if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                            break 'l16;
                        }
                        'l17: {
                            {
                                j = 0i32;
                                'l18: loop {
                                    if !(j < 2i32) {
                                        break 'l18;
                                    }
                                    'l19: {
                                        if ((streak) as i32)
                                            < (((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1950))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                        {
                                            streak = (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1950))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read();
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (topicTextId).write(2u32);
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    i = 0i32;
                    'l20: loop {
                        if !(i < 2i32) {
                            break 'l20;
                        }
                        'l21: {
                            if ((streak) as i32)
                                < (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1938))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                            {
                                streak = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(1938))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read();
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (topicTextId).write(2u32);
                break 'l1;
            }
            if __sw1 == 6i32 {
                {
                    i = 0i32;
                    'l22: loop {
                        if !(i < 2i32) {
                            break 'l22;
                        }
                        'l23: {
                            if ((streak) as i32)
                                < (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(2002))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                            {
                                streak = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(2002))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read();
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (topicTextId).write(5u32);
                break 'l1;
            }
        }
        return streak;
    }
}
pub(crate) unsafe extern "C" fn GetPokedexRatingLevel(numSeen: u16) -> u8 {
    unsafe {
        let mut numSeen = numSeen;
        if ((numSeen) as i32) < 10i32 {
            return 0u8;
        }
        if ((numSeen) as i32) < 20i32 {
            return 1u8;
        }
        if ((numSeen) as i32) < 30i32 {
            return 2u8;
        }
        if ((numSeen) as i32) < 40i32 {
            return 3u8;
        }
        if ((numSeen) as i32) < 50i32 {
            return 4u8;
        }
        if ((numSeen) as i32) < 60i32 {
            return 5u8;
        }
        if ((numSeen) as i32) < 70i32 {
            return 6u8;
        }
        if ((numSeen) as i32) < 80i32 {
            return 7u8;
        }
        if ((numSeen) as i32) < 90i32 {
            return 8u8;
        }
        if ((numSeen) as i32) < 100i32 {
            return 9u8;
        }
        if ((numSeen) as i32) < 110i32 {
            return 10u8;
        }
        if ((numSeen) as i32) < 120i32 {
            return 11u8;
        }
        if ((numSeen) as i32) < 130i32 {
            return 12u8;
        }
        if ((numSeen) as i32) < 140i32 {
            return 13u8;
        }
        if ((numSeen) as i32) < 150i32 {
            return 14u8;
        }
        if ((numSeen) as i32) < 160i32 {
            return 15u8;
        }
        if ((numSeen) as i32) < 170i32 {
            return 16u8;
        }
        if ((numSeen) as i32) < 180i32 {
            return 17u8;
        }
        if ((numSeen) as i32) < 190i32 {
            return 18u8;
        }
        if ((numSeen) as i32) < 200i32 {
            return 19u8;
        }
        if (GetSetPokedexFlag(SpeciesToNationalPokedexNum(410u16), 1u8)) != 0 {
            numSeen = (numSeen).wrapping_sub(1);
        }
        if (GetSetPokedexFlag(SpeciesToNationalPokedexNum(409u16), 1u8)) != 0 {
            numSeen = (numSeen).wrapping_sub(1);
        }
        if ((numSeen) as i32) < 200i32 {
            return 19u8;
        } else {
            return 20u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferPokedexRatingForMatchCall(destStr: *mut u8) {
    unsafe {
        let mut destStr = destStr;
        let mut numSeen: i32 = 0i32;
        let mut numCaught: i32 = 0i32;
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut dexRatingLevel: u8 = 0u8;
        let mut buffer: *mut u8 = Alloc(1000u32);
        if !(!(buffer).is_null()) {
            (destStr).write(255u8);
            return;
        }
        numSeen = ((GetHoennPokedexCount(0u8)) as i32);
        numCaught = ((GetHoennPokedexCount(1u8)) as i32);
        ConvertIntToDecimalStringN((&raw mut gStringVar1).cast::<u8>(), numSeen, 0i32, 3u8);
        ConvertIntToDecimalStringN((&raw mut gStringVar2).cast::<u8>(), numCaught, 0i32, 3u8);
        dexRatingLevel = GetPokedexRatingLevel(((numCaught) as u16));
        str = StringCopy(
            buffer,
            (&raw mut gBirchDexRatingText_AreYouCurious).cast::<u8>(),
        );
        ({
            let __t1 = str;
            str = (str).wrapping_offset(1);
            __t1
        })
        .write(251u8);
        str = StringCopy(
            str,
            (&raw mut gBirchDexRatingText_SoYouveSeenAndCaught).cast::<u8>(),
        );
        ({
            let __t2 = str;
            str = (str).wrapping_offset(1);
            __t2
        })
        .write(251u8);
        StringCopy(
            str,
            ((((&raw const sBirchDexRatingTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((dexRatingLevel) as i32) as isize))
            .read(),
        );
        str = StringExpandPlaceholders(destStr, buffer);
        if (IsNationalPokedexEnabled()) != 0 {
            ({
                let __t3 = str;
                str = (str).wrapping_offset(1);
                __t3
            })
            .write(251u8);
            numSeen = ((GetNationalPokedexCount(0u8)) as i32);
            numCaught = ((GetNationalPokedexCount(1u8)) as i32);
            ConvertIntToDecimalStringN((&raw mut gStringVar1).cast::<u8>(), numSeen, 0i32, 3u8);
            ConvertIntToDecimalStringN((&raw mut gStringVar2).cast::<u8>(), numCaught, 0i32, 3u8);
            StringExpandPlaceholders(
                str,
                (&raw mut gBirchDexRatingText_OnANationwideBasis).cast::<u8>(),
            );
        }
        Free(buffer);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMatchCallWindowGfx(windowId: u32, destOffset: u32, paletteId: u32) {
    unsafe {
        let mut windowId = windowId;
        let mut destOffset = destOffset;
        let mut paletteId = paletteId;
        let mut bg: u8 = ((GetWindowAttribute(((windowId) as u8), 0u8)) as u8);
        LoadBgTiles(
            bg,
            ((&raw const sMatchCallWindow_Gfx).cast::<u8>().cast_mut()).cast::<u8>(),
            256u16,
            ((destOffset) as u16),
        );
        LoadPalette(
            (((&raw const sMatchCallWindow_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            (((0u32).wrapping_add((paletteId).wrapping_mul(16u32))) as u16),
            32u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawMatchCallTextBoxBorder(
    windowId: u32,
    tileOffset: u32,
    paletteId: u32,
) {
    unsafe {
        let mut windowId = windowId;
        let mut tileOffset = tileOffset;
        let mut paletteId = paletteId;
        DrawMatchCallTextBoxBorder_Internal(windowId, tileOffset, paletteId);
    }
}
