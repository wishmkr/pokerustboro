//! Translated from `src/apprentice.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gApprentices sApprenticeFirstMeetingTexts sApprenticeWhichMonTexts sApprenticeHeldItemTexts sApprenticeWhichMoveTexts sApprenticeWhichMonFirstTexts sApprenticePickWinSpeechTexts sApprenticeChallengeTexts sValidApprenticeMoves sQuestionPossibilities sApprenticeFunctions sInitialApprenticeIds
#[allow(unused_imports)]
use crate::data::apprentice::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gApprenticePartyMovesData: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gApprenticeQuestionData: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gApprenticeFunc: Option<unsafe extern "C" fn()> = None;

unsafe extern "C" {
    static mut gGameLanguage: u8;
    static mut gLevelUpLearnsets: u8;
    static mut gMain: u8;
    static mut gMoveNames: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesNames: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_Give: u8;
    static mut gText_Lv50: u8;
    static mut gText_No: u8;
    static mut gText_NoNeed: u8;
    static mut gText_OpenLevel: u8;
    static mut gText_Yes: u8;
    static mut gTowerFemaleFacilityClasses: u8;
    static mut gTowerFemaleTrainerGfxIds: u8;
    static mut gTowerMaleFacilityClasses: u8;
    static mut gTowerMaleTrainerGfxIds: u8;
    fn AddTextPrinterForMessage(a0: u8);
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn ApprenticeOpenBagMenu();
    fn CalcApprenticeChecksum(a0: *mut u8);
    fn CanSpeciesLearnTMHM(a0: u16, a1: u8) -> u32;
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn ConvertPixelWidthToTileWidth(a0: i32) -> i32;
    fn CopyWindowToVram(a0: u8, a1: u8);
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
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreezeObjectEvents();
    fn FrontierSpeechToString(a0: *mut u16);
    fn GetItemName(a0: u16) -> *mut u8;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetTrainerId(a0: *mut u8) -> u32;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn ItemIdToBattleMoveId(a0: u16) -> u16;
    fn LockPlayerFieldControls();
    fn Menu_ProcessInput() -> i8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn PlaySE(a0: u16);
    fn PlayerFreeze();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn RunTextPrintersAndIsPrinter0Active() -> u16;
    fn ScriptContext_Enable();
    fn ScriptMenu_AdjustLeftCoordFromWidth(a0: i32, a1: i32) -> i32;
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn StopPlayerAvatar();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_PlayerName(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TVShowConvertInternationalString(a0: *mut u8, a1: *mut u8, a2: i32);
    fn VarSet(a0: u16, a1: u16) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferApprenticeChallengeText(saveApprenticeId: u8) {
    unsafe {
        let mut saveApprenticeId = saveApprenticeId;
        let mut i: u8 = 0u8;
        let mut num: u8 = 0u8;
        let mut challengeText: *mut u8 = core::ptr::null_mut();
        num = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
            .cast::<u8>())
        .wrapping_offset(((saveApprenticeId) as i32) as isize * 68))
        .wrapping_add(2))
        .read();
        {
            i = 0u8;
            'l1: loop {
                if !((((num) as i32) != 0i32) && (((i) as i32) < 4i32)) {
                    break 'l1;
                }
                'l2: {}
                num = ((crate::c::div_i32(((num) as i32), 10i32)) as u8);
                i = (i).wrapping_add(1);
            }
        }
        StringCopy_PlayerName(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                .cast::<u8>())
            .wrapping_offset(((saveApprenticeId) as i32) as isize * 68))
            .wrapping_add(56))
            .cast::<u8>(),
        );
        ConvertInternationalString(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                .cast::<u8>())
            .wrapping_offset(((saveApprenticeId) as i32) as isize * 68))
            .wrapping_add(63))
            .read(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                .cast::<u8>())
            .wrapping_offset(((saveApprenticeId) as i32) as isize * 68))
            .wrapping_add(2))
            .read()) as i32),
            1i32,
            i,
        );
        challengeText = ((((&raw const sApprenticeChallengeTexts)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                    .cast::<u8>())
                .wrapping_offset(((saveApprenticeId) as i32) as isize * 68))
                .wrapping_add(0),
                0,
                5,
                false,
            ) as u8) as i32) as isize,
        ))
        .read();
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), challengeText);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Apprentice_ScriptContext_Enable() {
    unsafe {
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetApprenticeStruct(apprentice: *mut u8) {
    unsafe {
        let mut apprentice = apprentice;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(12u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((apprentice).wrapping_add(40)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((apprentice).wrapping_add(56)).cast::<u8>()).write(255u8);
        crate::c::bf_write((apprentice).wrapping_add(0), 0, 5, (16u8) as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetAllApprenticeData() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(2),
            3,
            2,
            (0u8) as i32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as u32) < crate::c::div_u32(12u32, 2u32)) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(40))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(65535u16);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    crate::c::bf_write(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 68))
                        .wrapping_add(0),
                        0,
                        5,
                        (16u8) as i32,
                    );
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 68))
                    .wrapping_add(56))
                    .cast::<u8>())
                    .write(255u8);
                    crate::c::bf_write(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 68))
                        .wrapping_add(0),
                        5,
                        2,
                        (0u8) as i32,
                    );
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 68))
                    .wrapping_add(2))
                    .write(0u8);
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 68))
                    .wrapping_add(1))
                    .write(0u8);
                    {
                        j = 0u8;
                        'l5: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(52))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(0u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 68))
                    .wrapping_add(63))
                    .write(((&raw mut gGameLanguage).cast::<u8>()).read());
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 68))
                    .wrapping_add(64)
                    .cast::<u32>())
                    .write(0u32);
                }
                i = (i).wrapping_add(1);
            }
        }
        Script_ResetPlayerApprentice();
    }
}
pub(crate) unsafe extern "C" fn GivenApprenticeLvlMode() -> u8 {
    unsafe {
        return ((((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            0,
            2,
            false,
        ) as u8) as i32)
            != 0i32) as u8);
    }
}
pub(crate) unsafe extern "C" fn SetApprenticeId() {
    unsafe {
        if ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
            .cast::<u8>())
        .wrapping_add(2))
        .read()) as i32)
            == 0i32
        {
            'l1: loop {
                'l2: {
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .write(
                            ((((&raw const sInitialApprenticeIds).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::rem_u32(
                                    ((Random()) as u32),
                                    crate::c::div_u32(8u32, 1u32),
                                )) as i32) as isize,
                            ))
                            .read(),
                        );
                }
                if !(((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .read()) as i32)
                    == ((crate::c::bf_read(
                        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_add(0),
                        0,
                        5,
                        false,
                    ) as u8) as i32))
                {
                    break 'l1;
                }
            }
        } else {
            'l3: loop {
                'l4: {
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .write(((crate::c::rem_i32(((Random()) as i32), 16i32)) as u8));
                }
                if !(((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .read()) as i32)
                    == ((crate::c::bf_read(
                        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_add(0),
                        0,
                        5,
                        false,
                    ) as u8) as i32))
                {
                    break 'l3;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPlayersApprenticeLvlMode(mode: u8) {
    unsafe {
        let mut mode = mode;
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            0,
            2,
            (mode) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn ShuffleApprenticeSpecies() {
    unsafe {
        let mut species = crate::ffi::Align4([0u8; 10]);
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(10u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut species).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 50i32) {
                    break 'l3;
                }
                'l4: {
                    let mut temp: u8 = 0u8;
                    let mut rand1: u8 =
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(10u32, 1u32)))
                            as u8);
                    let mut rand2: u8 =
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(10u32, 1u32)))
                            as u8);
                    {
                        temp = (((&raw mut species).cast::<u8>())
                            .wrapping_offset(((rand1) as i32) as isize))
                        .read();
                        (((&raw mut species).cast::<u8>())
                            .wrapping_offset(((rand1) as i32) as isize))
                        .write(
                            (((&raw mut species).cast::<u8>())
                                .wrapping_offset(((rand2) as i32) as isize))
                            .read(),
                        );
                        (((&raw mut species).cast::<u8>())
                            .wrapping_offset(((rand2) as i32) as isize))
                        .write(temp);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < crate::c::div_i32(6i32, 2i32)) {
                    break 'l5;
                }
                'l6: {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(176))
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((((((((&raw mut species).cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_mul(2i32)) as isize))
                        .read()) as i32)
                            & 15i32)
                            << 4)
                            | ((((((&raw mut species).cast::<u8>()).wrapping_offset(
                                ((((i) as i32).wrapping_mul(2i32)).wrapping_add(1i32)) as isize,
                            ))
                            .read()) as i32)
                                & 15i32)) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetMonIdForQuestion(
    questionId: u8,
    party: *mut u8,
    partySlot: *mut u8,
) -> u8 {
    unsafe {
        let mut questionId = questionId;
        let mut party = party;
        let mut partySlot = partySlot;
        let mut i: u8 = 0u8;
        let mut count: u8 = 0u8;
        let mut monId: u8 = 0u8;
        if ((questionId) as i32) == 2i32 {
            'l1: loop {
                'l2: {
                    monId = ((crate::c::rem_i32(((Random()) as i32), crate::c::div_i32(6i32, 2i32)))
                        as u8);
                    {
                        count = 0u8;
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32) < 5i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((((((((&raw mut gApprenticePartyMovesData)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2))
                                .cast::<u8>())
                                .wrapping_offset(((monId) as i32) as isize * 10))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    != 0i32
                                {
                                    count = (count).wrapping_add(1);
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                if !(((count) as i32) > crate::c::div_i32(6i32, 2i32)) {
                    break 'l1;
                }
            }
        } else {
            if ((questionId) as i32) == 1i32 {
                monId = ((party).wrapping_offset((((partySlot).read()) as i32) as isize)).read();
                (partySlot).write(((partySlot).read()).wrapping_add(1));
            }
        }
        return monId;
    }
}
pub(crate) unsafe extern "C" fn SetRandomQuestionData() {
    unsafe {
        let mut questionOrder = crate::ffi::Align4([0u8; 10]);
        let mut partyOrder = crate::ffi::Align4([0u8; 3]);
        let mut partySlot: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut rand1: u8 = 0u8;
        let mut rand2: u8 = 0u8;
        let mut id: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(3u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut partyOrder).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l3;
                }
                'l4: {
                    let mut temp: u8 = 0u8;
                    rand1 = ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(3u32, 1u32)))
                        as u8);
                    rand2 = ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(3u32, 1u32)))
                        as u8);
                    {
                        temp = (((&raw mut partyOrder).cast::<u8>())
                            .wrapping_offset(((rand1) as i32) as isize))
                        .read();
                        (((&raw mut partyOrder).cast::<u8>())
                            .wrapping_offset(((rand1) as i32) as isize))
                        .write(
                            (((&raw mut partyOrder).cast::<u8>())
                                .wrapping_offset(((rand2) as i32) as isize))
                            .read(),
                        );
                        (((&raw mut partyOrder).cast::<u8>())
                            .wrapping_offset(((rand2) as i32) as isize))
                        .write(temp);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as u32) < crate::c::div_u32(10u32, 1u32)) {
                    break 'l5;
                }
                'l6: {
                    (((&raw mut questionOrder).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((&raw const sQuestionPossibilities).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as i32) < 50i32) {
                    break 'l7;
                }
                'l8: {
                    let mut temp: u8 = 0u8;
                    rand1 =
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(10u32, 1u32)))
                            as u8);
                    rand2 =
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(10u32, 1u32)))
                            as u8);
                    {
                        temp = (((&raw mut questionOrder).cast::<u8>())
                            .wrapping_offset(((rand1) as i32) as isize))
                        .read();
                        (((&raw mut questionOrder).cast::<u8>())
                            .wrapping_offset(((rand1) as i32) as isize))
                        .write(
                            (((&raw mut questionOrder).cast::<u8>())
                                .wrapping_offset(((rand2) as i32) as isize))
                            .read(),
                        );
                        (((&raw mut questionOrder).cast::<u8>())
                            .wrapping_offset(((rand2) as i32) as isize))
                        .write(temp);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gApprenticePartyMovesData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(48u32));
        (((&raw mut gApprenticePartyMovesData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .write(0u8);
        {
            i = 0u8;
            'l9: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l9;
                }
                'l10: {
                    {
                        j = 0u8;
                        'l11: loop {
                            if !(((j) as i32) < crate::c::div_i32(6i32, 2i32)) {
                                break 'l11;
                            }
                            'l12: {
                                ((((((((&raw mut gApprenticePartyMovesData)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(32))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 5))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(4u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        partySlot = 0u8;
        {
            i = 0u8;
            'l13: loop {
                if !(((i) as i32) < 9i32) {
                    break 'l13;
                }
                'l14: {
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        ((((&raw mut questionOrder).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32,
                    );
                    if (((((&raw mut questionOrder).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 3i32
                    {
                        crate::c::bf_write(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(0),
                            2,
                            2,
                            (GetMonIdForQuestion(
                                (((&raw mut questionOrder).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                (&raw mut partyOrder).cast::<u8>(),
                                &raw mut partySlot,
                            )) as i32,
                        );
                        id = (crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(0),
                            2,
                            2,
                            false,
                        ) as u8);
                        if (((((&raw mut questionOrder).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 2i32
                        {
                            'l15: loop {
                                'l16: {
                                    rand1 = ((crate::c::rem_i32(((Random()) as i32), 4i32)) as u8);
                                    {
                                        j = 0u8;
                                        'l17: loop {
                                            if !(((j) as i32)
                                                < (((((&raw mut gApprenticePartyMovesData)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .read())
                                                    as i32)
                                                    .wrapping_add(1i32))
                                            {
                                                break 'l17;
                                            }
                                            'l18: {
                                                if (((((((((((&raw mut gApprenticePartyMovesData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32)).cast::<u8>()).wrapping_offset((((id) as i32)) as isize * 5)).cast::<u8>()).wrapping_offset((((j) as i32)) as isize)).read()) as i32)) == (((rand1) as i32)) {
break 'l17;
}
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                }
                                if !(((j) as i32)
                                    != (((((&raw mut gApprenticePartyMovesData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .read()) as i32)
                                        .wrapping_add(1i32))
                                {
                                    break 'l15;
                                }
                            }
                            ((((((((&raw mut gApprenticePartyMovesData)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(32))
                            .cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 5))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut gApprenticePartyMovesData)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .read()) as i32) as isize,
                            ))
                            .write(rand1);
                            crate::c::bf_write(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(8))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(0),
                                4,
                                2,
                                (rand1) as i32,
                            );
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .write(GetRandomAlternateMove(
                                (crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(176))
                                    .wrapping_add(8))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(0),
                                    2,
                                    2,
                                    false,
                                ) as u8),
                            ));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            Free(
                ((&raw mut gApprenticePartyMovesData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut gApprenticePartyMovesData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn GetRandomAlternateMove(monId: u8) -> u16 {
    unsafe {
        let mut monId = monId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut id: u8 = 0u8;
        let mut numLearnsetMoves: u8 = 0u8;
        let mut species: u16 = 0u16;
        let mut learnset: *mut u16 = core::ptr::null_mut();
        let mut needTMs: u32 = 0u32;
        let mut r#move: u16 = 0u16;
        let mut shouldUseMove: u32 = 0u32;
        let mut level: u8 = 0u8;
        id = ((if ((monId) as i32) < crate::c::div_i32(6i32, 2i32) {
            (crate::c::shr_i32(
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(4))
                .cast::<u8>())
                .wrapping_offset(((monId) as i32) as isize))
                .read()) as i32),
                (((crate::c::shr_i32(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                            .wrapping_add(2),
                        0,
                        3,
                        false,
                    ) as u8) as i32),
                    ((monId) as u32),
                ) & 1i32)
                    << 2) as u32),
            ) & 15i32)
        } else {
            0i32
        }) as u8);
        species = (((((((&raw const gApprentices).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176)).read())
                    as i32) as isize
                    * 88,
            ))
        .wrapping_add(52))
        .cast::<u16>())
        .wrapping_offset(((id) as i32) as isize))
        .read();
        learnset = ((((&raw mut gLevelUpLearnsets).cast::<*mut u16>()).cast::<*mut u16>())
            .wrapping_offset(((species) as i32) as isize))
        .read();
        j = 0u8;
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            0,
            2,
            false,
        ) as u8) as i32)
            == 1i32
        {
            level = 50u8;
        } else {
            level = 60u8;
        }
        {
            j = 0u8;
            'l1: loop {
                if !(((((learnset).wrapping_offset(((j) as i32) as isize)).read()) as i32)
                    != 65535i32)
                {
                    break 'l1;
                }
                'l2: {
                    if (((((learnset).wrapping_offset(((j) as i32) as isize)).read()) as i32)
                        & 65024i32)
                        > (((level) as i32) << 9)
                    {
                        break 'l1;
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        numLearnsetMoves = j;
        i = 0u8;
        'l3: loop {
            if !(((i) as i32) < 5i32) {
                break 'l3;
            }
            if (crate::c::rem_i32(((Random()) as i32), 2i32) == 0i32) || (needTMs == 1u32) {
                'l4: loop {
                    'l5: {
                        'l6: loop {
                            'l7: {
                                id = ((crate::c::rem_i32(((Random()) as i32), 58i32)) as u8);
                                shouldUseMove = CanSpeciesLearnTMHM(species, id);
                            }
                            if !(!((shouldUseMove) != 0)) {
                                break 'l6;
                            }
                        }
                        r#move =
                            ItemIdToBattleMoveId((((289i32).wrapping_add(((id) as i32))) as u16));
                        shouldUseMove = 1u32;
                        if ((numLearnsetMoves) as i32) <= 4i32 {
                            j = 0u8;
                        } else {
                            j = ((((numLearnsetMoves) as i32).wrapping_sub(4i32)) as u8);
                        }
                        {
                            'l8: loop {
                                if !(((j) as i32) < ((numLearnsetMoves) as i32)) {
                                    break 'l8;
                                }
                                'l9: {
                                    if (((((learnset).wrapping_offset(((j) as i32) as isize))
                                        .read()) as i32)
                                        & 511i32)
                                        == ((r#move) as i32)
                                    {
                                        shouldUseMove = 0u32;
                                        break 'l8;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    if !(shouldUseMove != 1u32) {
                        break 'l4;
                    }
                }
            } else {
                if ((numLearnsetMoves) as i32) <= 4i32 {
                    needTMs = 1u32;
                    continue 'l3;
                } else {
                    'l10: loop {
                        'l11: {
                            let mut learnsetId: u8 = ((crate::c::rem_i32(
                                ((Random()) as i32),
                                ((numLearnsetMoves) as i32).wrapping_sub(4i32),
                            )) as u8);
                            r#move = ((((((learnset)
                                .wrapping_offset(((learnsetId) as i32) as isize))
                            .read()) as i32)
                                & 511i32) as u16);
                            shouldUseMove = 1u32;
                            {
                                j = ((((numLearnsetMoves) as i32).wrapping_sub(4i32)) as u8);
                                'l12: loop {
                                    if !(((j) as i32) < ((numLearnsetMoves) as i32)) {
                                        break 'l12;
                                    }
                                    'l13: {
                                        if (((((learnset).wrapping_offset(((j) as i32) as isize))
                                            .read())
                                            as i32)
                                            & 511i32)
                                            == ((r#move) as i32)
                                        {
                                            shouldUseMove = 0u32;
                                            break 'l12;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        if !(shouldUseMove != 1u32) {
                            break 'l10;
                        }
                    }
                }
            }
            if (TrySetMove(monId, r#move)) != 0 {
                if (((((&raw const sValidApprenticeMoves).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize))
                .read())
                    != 0
                {
                    break 'l3;
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = (((&raw mut gApprenticePartyMovesData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read());
        (__p1).write(((__p1).read()).wrapping_add(1));
        return r#move;
    }
}
pub(crate) unsafe extern "C" fn TrySetMove(monId: u8, r#move: u16) -> u8 {
    unsafe {
        let mut monId = monId;
        let mut r#move = r#move;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gApprenticePartyMovesData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2))
                    .cast::<u8>())
                    .wrapping_offset(((monId) as i32) as isize * 10))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((r#move) as i32)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((((&raw mut gApprenticePartyMovesData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(2))
        .cast::<u8>())
        .wrapping_offset(((monId) as i32) as isize * 10))
        .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gApprenticePartyMovesData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32) as isize,
        ))
        .write(r#move);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GetLatestLearnedMoves(species: u16, moves: *mut u16) {
    unsafe {
        let mut species = species;
        let mut moves = moves;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut level: u8 = 0u8;
        let mut numLearnsetMoves: u8 = 0u8;
        let mut learnset: *mut u16 = core::ptr::null_mut();
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            0,
            2,
            false,
        ) as u8) as i32)
            == 1i32
        {
            level = 50u8;
        } else {
            level = 60u8;
        }
        learnset = ((((&raw mut gLevelUpLearnsets).cast::<*mut u16>()).cast::<*mut u16>())
            .wrapping_offset(((species) as i32) as isize))
        .read();
        {
            i = 0u8;
            'l1: loop {
                if !(((((learnset).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                    != 65535i32)
                {
                    break 'l1;
                }
                'l2: {
                    if (((((learnset).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                        & 65024i32)
                        > (((level) as i32) << 9)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        numLearnsetMoves = i;
        if ((numLearnsetMoves) as i32) > 4i32 {
            numLearnsetMoves = 4u8;
        }
        {
            j = 0u8;
            'l3: loop {
                if !(((j) as i32) < ((numLearnsetMoves) as i32)) {
                    break 'l3;
                }
                'l4: {
                    ((moves).wrapping_offset(((j) as i32) as isize)).write(
                        ((((((learnset).wrapping_offset(
                            ((((i) as i32).wrapping_sub(1i32)).wrapping_sub(((j) as i32))) as isize,
                        ))
                        .read()) as i32)
                            & 511i32) as u16),
                    );
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetDefaultMove(monId: u8, speciesArrayId: u8, moveSlot: u8) -> u16 {
    unsafe {
        let mut monId = monId;
        let mut speciesArrayId = speciesArrayId;
        let mut moveSlot = moveSlot;
        let mut moves = crate::ffi::Align4([0u8; 8]);
        let mut i: u8 = 0u8;
        let mut numQuestions: u8 = 0u8;
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            2,
            4,
            false,
        ) as u8) as i32)
            < crate::c::div_i32(6i32, 2i32)
        {
            return 0u16;
        }
        numQuestions = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !((((i) as i32) < 9i32)
                    && (((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        != 0i32))
                {
                    break 'l1;
                }
                'l2: {
                    numQuestions = (numQuestions).wrapping_add(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        GetLatestLearnedMoves(
            (((((((&raw const gApprentices).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .read()) as i32) as isize
                        * 88,
                ))
            .wrapping_add(52))
            .cast::<u16>())
            .wrapping_offset(((speciesArrayId) as i32) as isize))
            .read(),
            (&raw mut moves).cast::<u16>(),
        );
        {
            i = 0u8;
            'l3: loop {
                if !((((i) as i32) < ((numQuestions) as i32))
                    && (((i) as i32)
                        < ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(1),
                            2,
                            4,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(crate::c::div_i32(6i32, 2i32))))
                {
                    break 'l3;
                }
                'l4: {
                    if ((((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        == 2i32)
                        && (((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(0),
                            2,
                            2,
                            false,
                        ) as u8) as i32)
                            == ((monId) as i32)))
                        && ((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(0),
                            6,
                            2,
                            false,
                        ) as u8)
                            != 0)
                    {
                        (((&raw mut moves).cast::<u16>()).wrapping_offset(
                            ((crate::c::bf_read(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(8))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(0),
                                4,
                                2,
                                false,
                            ) as u8) as i32) as isize,
                        ))
                        .write(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (((&raw mut moves).cast::<u16>()).wrapping_offset(((moveSlot) as i32) as isize))
            .read();
    }
}
pub(crate) unsafe extern "C" fn SaveApprenticeParty(numQuestions: u8) {
    unsafe {
        let mut numQuestions = numQuestions;
        let mut apprenticeMons = crate::ffi::Align4([0u8; 12]);
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut speciesTableId: u32 = 0u32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < crate::c::div_i32(6i32, 2i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .write(0u16);
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .wrapping_add(10)
                    .cast::<u16>())
                    .write(0u16);
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_add(4))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(2))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(0u16);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        j = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            6,
            2,
            false,
        ) as u8);
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < crate::c::div_i32(6i32, 2i32)) {
                    break 'l5;
                }
                'l6: {
                    (((&raw mut apprenticeMons).cast::<*mut u8>())
                        .wrapping_offset(((j) as i32) as isize))
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12),
                    );
                    j = ((crate::c::rem_i32(
                        ((j) as i32).wrapping_add(1i32),
                        crate::c::div_i32(6i32, 2i32),
                    )) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as i32) < crate::c::div_i32(6i32, 2i32)) {
                    break 'l7;
                }
                'l8: {
                    speciesTableId = ((if ((i) as i32) < crate::c::div_i32(6i32, 2i32) {
                        (crate::c::shr_i32(
                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(4))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                            (((crate::c::shr_i32(
                                ((crate::c::bf_read(
                                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(176))
                                    .wrapping_add(2),
                                    0,
                                    3,
                                    false,
                                ) as u8) as i32),
                                ((i) as u32),
                            ) & 1i32)
                                << 2) as u32),
                        ) & 15i32)
                    } else {
                        0i32
                    }) as u32);
                    (((((&raw mut apprenticeMons).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .cast::<u16>())
                    .write(
                        (((((((&raw const gApprentices).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .read()) as i32) as isize
                                    * 88,
                            ))
                        .wrapping_add(52))
                        .cast::<u16>())
                        .wrapping_offset(((speciesTableId) as i32) as isize))
                        .read(),
                    );
                    GetLatestLearnedMoves(
                        (((((&raw mut apprenticeMons).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .cast::<u16>())
                        .read(),
                        (((((&raw mut apprenticeMons).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(2))
                        .cast::<u16>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l9: loop {
                if !(((i) as i32) < ((numQuestions) as i32)) {
                    break 'l9;
                }
                'l10: {
                    let mut questionId: u8 = (crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u8);
                    let mut monId: u8 = (crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        2,
                        2,
                        false,
                    ) as u8);
                    if ((questionId) as i32) == 1i32 {
                        if (crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(0),
                            6,
                            2,
                            false,
                        ) as u8)
                            != 0
                        {
                            (((((&raw mut apprenticeMons).cast::<*mut u8>())
                                .wrapping_offset(((monId) as i32) as isize))
                            .read())
                            .wrapping_add(10)
                            .cast::<u16>())
                            .write(
                                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(8))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read(),
                            );
                        }
                    } else {
                        if ((questionId) as i32) == 2i32 {
                            if (crate::c::bf_read(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(8))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(0),
                                6,
                                2,
                                false,
                            ) as u8)
                                != 0
                            {
                                let mut moveSlot: u32 = ((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(176))
                                    .wrapping_add(8))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(0),
                                    4,
                                    2,
                                    false,
                                ) as u8)
                                    as u32);
                                (((((((&raw mut apprenticeMons).cast::<*mut u8>())
                                    .wrapping_offset(((monId) as i32) as isize))
                                .read())
                                .wrapping_add(2))
                                .cast::<u16>())
                                .wrapping_offset(((moveSlot) as i32) as isize))
                                .write(
                                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(176))
                                    .wrapping_add(8))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read(),
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
pub(crate) unsafe extern "C" fn CreateApprenticeMenu(menu: u8) {
    unsafe {
        let mut menu = menu;
        let mut i: u8 = 0u8;
        let mut windowId: u8 = 0u8;
        let mut strings = crate::ffi::Align4([0u8; 12]);
        let mut count: u8 = 2u8;
        let mut width: u8 = 0u8;
        let mut left: u8 = 0u8;
        let mut top: u8 = 0u8;
        let mut pixelWidth: i32 = 0i32;
        'l1: {
            let __sw1 = ((menu) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 {
                left = 18u8;
                top = 8u8;
                ((&raw mut strings).cast::<*mut u8>()).write((&raw mut gText_Lv50).cast::<u8>());
                (((&raw mut strings).cast::<*mut u8>()).wrapping_offset(1))
                    .write((&raw mut gText_OpenLevel).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 1i32 {
                count = ((crate::c::div_i32(6i32, 2i32)) as u8);
                left = 18u8;
                top = 6u8;
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < crate::c::div_i32(6i32, 2i32)) {
                            break 'l2;
                        }
                        'l3: {
                            let mut species: u16 = 0u16;
                            let mut speciesTableId: u32 = 0u32;
                            speciesTableId = ((if ((i) as i32) < crate::c::div_i32(6i32, 2i32) {
                                (crate::c::shr_i32(
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(176))
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32),
                                    (((crate::c::shr_i32(
                                        ((crate::c::bf_read(
                                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(176))
                                            .wrapping_add(2),
                                            0,
                                            3,
                                            false,
                                        ) as u8) as i32),
                                        ((i) as u32),
                                    ) & 1i32)
                                        << 2) as u32),
                                ) & 15i32)
                            } else {
                                0i32
                            }) as u32);
                            species = (((((((&raw const gApprentices).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(52))
                            .cast::<u16>())
                            .wrapping_offset(((speciesTableId) as i32) as isize))
                            .read();
                            (((&raw mut strings).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((&raw mut gSpeciesNames).cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 11))
                                .cast::<u8>(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                left = 18u8;
                top = 8u8;
                if ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(1),
                    2,
                    4,
                    false,
                ) as u8) as i32)
                    >= crate::c::div_i32(6i32, 2i32)
                {
                    return;
                }
                (((&raw mut strings).cast::<*mut u8>()).wrapping_offset(1)).write(
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut strings).cast::<*mut u8>()).write(
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                left = 17u8;
                top = 8u8;
                ((&raw mut strings).cast::<*mut u8>()).write(
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                (((&raw mut strings).cast::<*mut u8>()).wrapping_offset(1)).write(
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                left = 18u8;
                top = 8u8;
                ((&raw mut strings).cast::<*mut u8>()).write((&raw mut gText_Give).cast::<u8>());
                (((&raw mut strings).cast::<*mut u8>()).wrapping_offset(1))
                    .write((&raw mut gText_NoNeed).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 6i32 {
                left = 20u8;
                top = 8u8;
                ((&raw mut strings).cast::<*mut u8>()).write((&raw mut gText_Yes).cast::<u8>());
                (((&raw mut strings).cast::<*mut u8>()).wrapping_offset(1))
                    .write((&raw mut gText_No).cast::<u8>());
                break 'l1;
            }
            if !__matched {
                left = 0u8;
                top = 0u8;
                return;
                break 'l1;
            }
        }
        pixelWidth = 0i32;
        {
            i = 0u8;
            'l4: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l4;
                }
                'l5: {
                    let mut width: i32 = GetStringWidth(
                        1u8,
                        (((&raw mut strings).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        0i16,
                    );
                    if width > pixelWidth {
                        pixelWidth = width;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        width = ((ConvertPixelWidthToTileWidth(pixelWidth)) as u8);
        left = ((ScriptMenu_AdjustLeftCoordFromWidth(((left) as i32), ((width) as i32))) as u8);
        windowId = CreateAndShowWindow(
            left,
            top,
            width,
            ((((count) as i32).wrapping_mul(2i32)) as u8),
        );
        SetStandardWindowBorderStyle(windowId, 0u8);
        {
            i = 0u8;
            'l6: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l6;
                }
                'l7: {
                    AddTextPrinterParameterized(
                        windowId,
                        1u8,
                        (((&raw mut strings).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        8u8,
                        (((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        InitMenuInUpperLeftCornerNormal(windowId, count, 0u8);
        CreateChooseAnswerTask(1u8, count, windowId);
    }
}
pub(crate) unsafe extern "C" fn Task_ChooseAnswer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut input: i8 = 0i8;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((((data).wrapping_offset(5)).read()) != 0) {
            input = Menu_ProcessInputNoWrap();
        } else {
            input = Menu_ProcessInput();
        }
        'l1: {
            let __sw1 = ((input) as i32);
            let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
            if __sw1 == (-2i32) {
                return;
            }
            if __sw1 == (-1i32) {
                if (((data).wrapping_offset(4)).read()) != 0 {
                    return;
                }
                PlaySE(5u16);
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(127u16);
                break 'l1;
            }
            if !__matched {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((input) as u16));
                break 'l1;
            }
        }
        RemoveAndHideWindow(((((data).wrapping_offset(6)).read()) as u8));
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn CreateAndShowWindow(
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) -> u8 {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut windowId: u8 = 0u8;
        let mut winTemplate = crate::ffi::Align4([0u8; 8]);
        (&raw mut winTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(CreateWindowTemplate(
                0u8,
                ((((left) as i32).wrapping_add(1i32)) as u8),
                ((((top) as i32).wrapping_add(1i32)) as u8),
                width,
                height,
                15u8,
                100u16,
            ));
        windowId = ((AddWindow((&raw mut winTemplate).cast::<u8>())) as u8);
        PutWindowTilemap(windowId);
        CopyWindowToVram(windowId, 3u8);
        return windowId;
    }
}
pub(crate) unsafe extern "C" fn RemoveAndHideWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        ClearStdWindowAndFrameToTransparent(windowId, 1u8);
        RemoveWindow(windowId);
    }
}
pub(crate) unsafe extern "C" fn CreateChooseAnswerTask(noBButton: u8, answers: u8, windowId: u8) {
    unsafe {
        let mut noBButton = noBButton;
        let mut answers = answers;
        let mut windowId = windowId;
        let mut taskId: u8 = CreateTask(Some(Task_ChooseAnswer), 80u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((noBButton) as i16));
        if ((answers) as i32) > 3i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(1i16);
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((windowId) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallApprenticeFunction() {
    unsafe {
        (((((&raw const sApprenticeFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn Script_ResetPlayerApprentice() {
    unsafe {
        let mut i: u8 = 0u8;
        SetApprenticeId();
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            0,
            2,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            2,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            6,
            2,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(2),
            0,
            3,
            (0u8) as i32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < crate::c::div_i32(6i32, 2i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(176))
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 9i32) {
                    break 'l3;
                }
                'l4: {
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        2,
                        2,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        4,
                        2,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        6,
                        2,
                        (0u8) as i32,
                    );
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(176))
                    .wrapping_add(8))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Script_GivenApprenticeLvlMode() {
    unsafe {
        if !((GivenApprenticeLvlMode()) != 0) {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Script_SetApprenticeLvlMode() {
    unsafe {
        SetPlayersApprenticeLvlMode(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8));
    }
}
pub(crate) unsafe extern "C" fn Script_SetApprenticeId() {
    unsafe {
        SetApprenticeId();
    }
}
pub(crate) unsafe extern "C" fn Script_SetRandomQuestionData() {
    unsafe {
        SetRandomQuestionData();
    }
}
pub(crate) unsafe extern "C" fn IncrementQuestionsAnswered() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            2,
            4,
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(1),
                2,
                4,
                false,
            ) as u8)
                .wrapping_add(1)) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn GetNumApprenticePartyMonsAssigned() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(1),
                2,
                4,
                false,
            ) as u8) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn IsFinalQuestion() {
    unsafe {
        let mut questionNum: i32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            2,
            4,
            false,
        ) as u8) as i32)
            .wrapping_sub(crate::c::div_i32(6i32, 2i32));
        if questionNum < 0i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            if questionNum > 8i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
            }
            if ((crate::c::bf_read(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(8))
                .cast::<u8>())
                .wrapping_offset((questionNum) as isize * 4))
                .wrapping_add(0),
                0,
                2,
                false,
            ) as u8) as i32)
                == 0i32
            {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
            } else {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Script_CreateApprenticeMenu() {
    unsafe {
        CreateApprenticeMenu(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8));
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForPrintingMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
            DestroyTask(taskId);
            if (((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) != 0 {
                ExecuteFuncAfterButtonPress(Some(ScriptContext_Enable));
            } else {
                ScriptContext_Enable();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintApprenticeMessage() {
    unsafe {
        let mut string: *mut u8 = core::ptr::null_mut();
        if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) == 6i32 {
            string = (((((&raw const sApprenticeWhichMonTexts)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176)).read())
                    as i32) as isize
                    * 8,
            ))
            .cast::<*mut u8>())
            .read();
        } else {
            if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) == 7i32 {
                string = ((((((&raw const sApprenticeWhichMonTexts)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .read()) as i32) as isize
                        * 8,
                ))
                .cast::<*mut u8>())
                .wrapping_offset(1))
                .read();
            } else {
                if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) == 8i32 {
                    string = (((((&raw const sApprenticeWhichMoveTexts)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .cast::<*mut u8>())
                    .read();
                } else {
                    if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) == 9i32 {
                        string = ((((((&raw const sApprenticeWhichMoveTexts)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .read()) as i32) as isize
                                * 8,
                        ))
                        .cast::<*mut u8>())
                        .wrapping_offset(1))
                        .read();
                    } else {
                        if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) == 4i32 {
                            string = (((((&raw const sApprenticeWhichMonFirstTexts)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .read()) as i32) as isize
                                    * 8,
                            ))
                            .cast::<*mut u8>())
                            .read();
                        } else {
                            if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)
                                == 5i32
                            {
                                string = ((((((&raw const sApprenticeWhichMonFirstTexts)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(176))
                                    .read()) as i32) as isize
                                        * 8,
                                ))
                                .cast::<*mut u8>())
                                .wrapping_offset(1))
                                .read();
                            } else {
                                if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)
                                    == 10i32
                                {
                                    string = (((((&raw const sApprenticeHeldItemTexts)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(176))
                                        .read()) as i32)
                                            as isize
                                            * 20,
                                    ))
                                    .cast::<*mut u8>())
                                    .read();
                                } else {
                                    if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read())
                                        as i32)
                                        == 11i32
                                    {
                                        string = (((((&raw const sApprenticePickWinSpeechTexts)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(176))
                                            .read())
                                                as i32)
                                                as isize
                                                * 8,
                                        ))
                                        .cast::<*mut u8>())
                                        .read();
                                    } else {
                                        if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read())
                                            as i32)
                                            == 12i32
                                        {
                                            string =
                                                ((((((&raw const sApprenticeHeldItemTexts)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gSaveBlock2Ptr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(176))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 20,
                                                ))
                                                .cast::<*mut u8>())
                                                .wrapping_offset(3))
                                                .read();
                                        } else {
                                            if ((((&raw mut gSpecialVar_0x8006).cast::<u16>())
                                                .read())
                                                as i32)
                                                == 13i32
                                            {
                                                string =
                                                    ((((((&raw const sApprenticeHeldItemTexts)
                                                        .cast::<u8>()
                                                        .cast_mut())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((((&raw mut gSaveBlock2Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(176))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 20,
                                                    ))
                                                    .cast::<*mut u8>())
                                                    .wrapping_offset(1))
                                                    .read();
                                            } else {
                                                if ((((&raw mut gSpecialVar_0x8006).cast::<u16>())
                                                    .read())
                                                    as i32)
                                                    == 16i32
                                                {
                                                    string = ((((((&raw const sApprenticeHeldItemTexts).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))).read()) as i32)) as isize * 20)).cast::<*mut u8>()).wrapping_offset(4)).read();
                                                } else {
                                                    if ((((&raw mut gSpecialVar_0x8006)
                                                        .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        == 14i32
                                                    {
                                                        string = ((((((&raw const sApprenticeHeldItemTexts).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))).read()) as i32)) as isize * 20)).cast::<*mut u8>()).wrapping_offset(2)).read();
                                                    } else {
                                                        if ((((&raw mut gSpecialVar_0x8006)
                                                            .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            == 15i32
                                                        {
                                                            string = ((((((&raw const sApprenticePickWinSpeechTexts).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))).read()) as i32)) as isize * 8)).cast::<*mut u8>()).wrapping_offset(1)).read();
                                                        } else {
                                                            if ((((&raw mut gSpecialVar_0x8006)
                                                                .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                == 0i32
                                                            {
                                                                string = (((((&raw const sApprenticeFirstMeetingTexts).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))).read()) as i32)) as isize * 16)).cast::<*mut u8>()).read();
                                                            } else {
                                                                if (((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)) == 1i32 {
string = ((((((&raw const sApprenticeFirstMeetingTexts).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))).read()) as i32)) as isize * 16)).cast::<*mut u8>()).wrapping_offset(1)).read();
} else {
if (((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)) == 2i32 {
string = ((((((&raw const sApprenticeFirstMeetingTexts).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))).read()) as i32)) as isize * 16)).cast::<*mut u8>()).wrapping_offset(2)).read();
} else {
if (((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)) == 3i32 {
string = ((((((&raw const sApprenticeFirstMeetingTexts).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))).read()) as i32)) as isize * 16)).cast::<*mut u8>()).wrapping_offset(3)).read();
} else {
ScriptContext_Enable();
return;
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
                        }
                    }
                }
            }
        }
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), string);
        AddTextPrinterForMessage(1u8);
        CreateTask(Some(Task_WaitForPrintingMessage), 1u8);
    }
}
pub(crate) unsafe extern "C" fn Script_PrintApprenticeMessage() {
    unsafe {
        LockPlayerFieldControls();
        FreezeObjectEvents();
        PlayerFreeze();
        StopPlayerAvatar();
        DrawDialogueFrame(0u8, 1u8);
        PrintApprenticeMessage();
    }
}
pub(crate) unsafe extern "C" fn ApprenticeGetQuestion() {
    unsafe {
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            2,
            4,
            false,
        ) as u8) as i32)
            < crate::c::div_i32(6i32, 2i32)
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
        } else {
            if ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(1),
                2,
                4,
                false,
            ) as u8) as i32)
                > ((9i32).wrapping_add(crate::c::div_i32(6i32, 2i32))).wrapping_sub(1i32)
            {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
            } else {
                let mut id: i32 = ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(1),
                    2,
                    4,
                    false,
                ) as u8) as i32)
                    .wrapping_sub(crate::c::div_i32(6i32, 2i32));
                'l1: {
                    let __sw1 = ((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset((id) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u8) as i32);
                    let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
                    if __sw1 == 1i32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(4u16);
                        break 'l1;
                    }
                    if __sw1 == 2i32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(3u16);
                        break 'l1;
                    }
                    if __sw1 == 3i32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                        break 'l1;
                    }
                    if !__matched {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                        break 'l1;
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetApprenticePartyMon() {
    unsafe {
        if (((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) != 0 {
            let mut partySlot: u8 = ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8);
            crate::c::bf_write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(2),
                0,
                3,
                ((((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(2),
                    0,
                    3,
                    false,
                ) as u8) as i32)
                    | crate::c::shl_i32(1i32, ((partySlot) as u32))) as u8) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetApprenticeMonMove() {
    unsafe {
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            2,
            4,
            false,
        ) as u8) as i32)
            >= crate::c::div_i32(6i32, 2i32)
        {
            let mut id: u8 = ((((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(1),
                2,
                4,
                false,
            ) as u8) as i32)
                .wrapping_sub(crate::c::div_i32(6i32, 2i32))) as u8);
            if (((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) != 0 {
                crate::c::bf_write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(176))
                    .wrapping_add(8))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 4))
                    .wrapping_add(0),
                    6,
                    2,
                    (1u8) as i32,
                );
            } else {
                crate::c::bf_write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(176))
                    .wrapping_add(8))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 4))
                    .wrapping_add(0),
                    6,
                    2,
                    (0u8) as i32,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitQuestionData() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut count: u8 = 0u8;
        let mut id1: u8 = 0u8;
        let mut id2: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !((((i) as i32) < 9i32)
                    && (((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        != 0i32))
                {
                    break 'l1;
                }
                'l2: {}
                count = (count).wrapping_add(1);
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gApprenticeQuestionData)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 2i32 {
            if ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(1),
                2,
                4,
                false,
            ) as u8) as i32)
                < crate::c::div_i32(6i32, 2i32)
            {
                id1 = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(176))
                .wrapping_add(4))
                .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                            .wrapping_add(1),
                        2,
                        4,
                        false,
                    ) as u8) as i32) as isize,
                ))
                .read()) as i32)
                    >> 4) as u8);
                ((((&raw mut gApprenticeQuestionData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<u16>())
                .write(
                    (((((((&raw const gApprentices).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .read()) as i32) as isize
                                * 88,
                        ))
                    .wrapping_add(52))
                    .cast::<u16>())
                    .wrapping_offset(((id1) as i32) as isize))
                    .read(),
                );
                id2 = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(176))
                .wrapping_add(4))
                .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                            .wrapping_add(1),
                        2,
                        4,
                        false,
                    ) as u8) as i32) as isize,
                ))
                .read()) as i32)
                    & 15i32) as u8);
                ((((&raw mut gApprenticeQuestionData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .cast::<u16>())
                .write(
                    (((((((&raw const gApprentices).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .read()) as i32) as isize
                                * 88,
                        ))
                    .wrapping_add(52))
                    .cast::<u16>())
                    .wrapping_offset(((id2) as i32) as isize))
                    .read(),
                );
            }
        } else {
            if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 3i32 {
                if ((((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(1),
                    2,
                    4,
                    false,
                ) as u8) as i32)
                    >= crate::c::div_i32(6i32, 2i32))
                    && (((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                            .wrapping_add(1),
                        2,
                        4,
                        false,
                    ) as u8) as i32)
                        < ((count) as i32).wrapping_add(crate::c::div_i32(6i32, 2i32))))
                    && (((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(1),
                                2,
                                4,
                                false,
                            ) as u8) as i32)
                                .wrapping_sub(crate::c::div_i32(6i32, 2i32)))
                                as isize
                                * 4,
                        ))
                        .wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        == 2i32)
                {
                    count = (crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(1),
                                2,
                                4,
                                false,
                            ) as u8) as i32)
                                .wrapping_sub(crate::c::div_i32(6i32, 2i32)))
                                as isize
                                * 4,
                        ))
                        .wrapping_add(0),
                        2,
                        2,
                        false,
                    ) as u8);
                    id1 = ((crate::c::shr_i32(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(2),
                            0,
                            3,
                            false,
                        ) as u8) as i32),
                        ((count) as u32),
                    ) & 1i32) as u8);
                    id1 = ((crate::c::shr_i32(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(((count) as i32) as isize))
                        .read()) as i32),
                        ((((id1) as i32) << 2) as u32),
                    ) & 15i32) as u8);
                    ((((&raw mut gApprenticeQuestionData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u16>())
                    .write(
                        (((((((&raw const gApprentices).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .read()) as i32) as isize
                                    * 88,
                            ))
                        .wrapping_add(52))
                        .cast::<u16>())
                        .wrapping_offset(((id1) as i32) as isize))
                        .read(),
                    );
                    ((((&raw mut gApprenticeQuestionData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(GetDefaultMove(
                        count,
                        id1,
                        (crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((crate::c::bf_read(
                                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(176))
                                    .wrapping_add(1),
                                    2,
                                    4,
                                    false,
                                ) as u8) as i32)
                                    .wrapping_sub(crate::c::div_i32(6i32, 2i32)))
                                    as isize
                                    * 4,
                            ))
                            .wrapping_add(0),
                            4,
                            2,
                            false,
                        ) as u8),
                    ));
                    ((((&raw mut gApprenticeQuestionData)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6)
                    .cast::<u16>())
                    .write(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(1),
                                2,
                                4,
                                false,
                            ) as u8) as i32)
                                .wrapping_sub(crate::c::div_i32(6i32, 2i32)))
                                as isize
                                * 4,
                        ))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read(),
                    );
                }
            } else {
                if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 4i32 {
                    if ((((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                            .wrapping_add(1),
                        2,
                        4,
                        false,
                    ) as u8) as i32)
                        >= crate::c::div_i32(6i32, 2i32))
                        && (((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(1),
                            2,
                            4,
                            false,
                        ) as u8) as i32)
                            < ((count) as i32).wrapping_add(crate::c::div_i32(6i32, 2i32))))
                        && (((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((crate::c::bf_read(
                                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(176))
                                    .wrapping_add(1),
                                    2,
                                    4,
                                    false,
                                ) as u8) as i32)
                                    .wrapping_sub(crate::c::div_i32(6i32, 2i32)))
                                    as isize
                                    * 4,
                            ))
                            .wrapping_add(0),
                            0,
                            2,
                            false,
                        ) as u8) as i32)
                            == 1i32)
                    {
                        count = (crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((crate::c::bf_read(
                                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(176))
                                    .wrapping_add(1),
                                    2,
                                    4,
                                    false,
                                ) as u8) as i32)
                                    .wrapping_sub(crate::c::div_i32(6i32, 2i32)))
                                    as isize
                                    * 4,
                            ))
                            .wrapping_add(0),
                            2,
                            2,
                            false,
                        ) as u8);
                        id2 = ((crate::c::shr_i32(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(2),
                                0,
                                3,
                                false,
                            ) as u8) as i32),
                            ((count) as u32),
                        ) & 1i32) as u8);
                        id2 = ((crate::c::shr_i32(
                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(4))
                            .cast::<u8>())
                            .wrapping_offset(((count) as i32) as isize))
                            .read()) as i32),
                            ((((id2) as i32) << 2) as u32),
                        ) & 15i32) as u8);
                        ((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u16>())
                        .write(
                            (((((((&raw const gApprentices).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(52))
                            .cast::<u16>())
                            .wrapping_offset(((id2) as i32) as isize))
                            .read(),
                        );
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeQuestionData() {
    unsafe {
        {
            Free(
                ((&raw mut gApprenticeQuestionData)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut gApprenticeQuestionData)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn ApprenticeBufferString() {
    unsafe {
        let mut stringDst: *mut u8 = core::ptr::null_mut();
        let mut text = crate::ffi::Align4([0u8; 16]);
        let mut speciesArrayId: u32 = 0u32;
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                stringDst = (&raw mut gStringVar1).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 1i32 {
                stringDst = (&raw mut gStringVar2).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 2i32 {
                stringDst = (&raw mut gStringVar3).cast::<u8>();
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        'l2: {
            let __sw2 = ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32);
            if __sw2 == 0i32 {
                StringCopy(
                    stringDst,
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                break 'l2;
            }
            if __sw2 == 1i32 {
                StringCopy(
                    stringDst,
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                break 'l2;
            }
            if __sw2 == 2i32 {
                StringCopy(
                    stringDst,
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                break 'l2;
            }
            if __sw2 == 3i32 {
                StringCopy(
                    stringDst,
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                break 'l2;
            }
            if __sw2 == 4i32 {
                StringCopy(
                    stringDst,
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gApprenticeQuestionData)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                break 'l2;
            }
            if __sw2 == 5i32 {
                StringCopy(
                    stringDst,
                    GetItemName(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(1),
                                2,
                                4,
                                false,
                            ) as u8) as i32)
                                .wrapping_sub(crate::c::div_i32(6i32, 2i32)))
                                as isize
                                * 4,
                        ))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read(),
                    ),
                );
                break 'l2;
            }
            if __sw2 == 6i32 {
                TVShowConvertInternationalString(
                    (&raw mut text).cast::<u8>(),
                    GetApprenticeNameInLanguage(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .read()) as u32),
                        2i32,
                    ),
                    2i32,
                );
                StringCopy(stringDst, (&raw mut text).cast::<u8>());
                break 'l2;
            }
            if __sw2 == 8i32 {
                if ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(1),
                    0,
                    2,
                    false,
                ) as u8) as i32)
                    == 1i32
                {
                    StringCopy(stringDst, (&raw mut gText_Lv50).cast::<u8>());
                } else {
                    StringCopy(stringDst, (&raw mut gText_OpenLevel).cast::<u8>());
                }
                break 'l2;
            }
            if __sw2 == 7i32 {
                FrontierSpeechToString(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                        .cast::<u8>())
                    .wrapping_add(40))
                    .cast::<u16>(),
                );
                StringCopy(stringDst, (&raw mut gStringVar4).cast::<u8>());
                break 'l2;
            }
            if __sw2 == 9i32 {
                speciesArrayId = ((if ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(1),
                    6,
                    2,
                    false,
                ) as u8) as i32)
                    < crate::c::div_i32(6i32, 2i32)
                {
                    (crate::c::shr_i32(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(1),
                                6,
                                2,
                                false,
                            ) as u8) as i32) as isize,
                        ))
                        .read()) as i32),
                        (((crate::c::shr_i32(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(2),
                                0,
                                3,
                                false,
                            ) as u8) as i32),
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(1),
                                6,
                                2,
                                false,
                            ) as u8) as u32),
                        ) & 1i32)
                            << 2) as u32),
                    ) & 15i32)
                } else {
                    0i32
                }) as u32);
                StringCopy(
                    stringDst,
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        (((((((((&raw const gApprentices).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(52))
                        .cast::<u16>())
                        .wrapping_offset(((speciesArrayId) as i32) as isize))
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                break 'l2;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetLeadApprenticeMon() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            6,
            2,
            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Script_ApprenticeOpenBagMenu() {
    unsafe {
        ApprenticeOpenBagMenu();
    }
}
pub(crate) unsafe extern "C" fn TrySetApprenticeHeldItem() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut count: u8 = 0u8;
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(1),
            2,
            4,
            false,
        ) as u8) as i32)
            < crate::c::div_i32(6i32, 2i32)
        {
            return;
        }
        count = 0u8;
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32) < 9i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((j) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        == 0i32
                    {
                        break 'l1;
                    }
                    count = (count).wrapping_add(1);
                }
                j = (j).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < ((count) as i32)) {
                    break 'l3;
                }
                'l4: {
                    if ((i) as i32)
                        >= ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(1),
                            2,
                            4,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(crate::c::div_i32(6i32, 2i32))
                    {
                        break 'l3;
                    }
                    if (((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        != 1i32)
                        || (((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(0),
                            6,
                            2,
                            false,
                        ) as u8) as i32)
                            == 0i32)
                    {
                        break 'l4;
                    }
                    if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(176))
                    .wrapping_add(8))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                        == ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                    {
                        crate::c::bf_write(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(176))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((crate::c::bf_read(
                                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(176))
                                    .wrapping_add(1),
                                    2,
                                    4,
                                    false,
                                ) as u8) as i32)
                                    .wrapping_sub(crate::c::div_i32(6i32, 2i32)))
                                    as isize
                                    * 4,
                            ))
                            .wrapping_add(0),
                            6,
                            2,
                            (0u8) as i32,
                        );
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(176))
                                .wrapping_add(1),
                                2,
                                4,
                                false,
                            ) as u8) as i32)
                                .wrapping_sub(crate::c::div_i32(6i32, 2i32)))
                                as isize
                                * 4,
                        ))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(((&raw mut gSpecialVar_0x8005).cast::<u16>()).read());
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                .wrapping_add(8))
            .cast::<u8>())
            .wrapping_offset(
                (((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                        .wrapping_add(1),
                    2,
                    4,
                    false,
                ) as u8) as i32)
                    .wrapping_sub(crate::c::div_i32(6i32, 2i32))) as isize
                    * 4,
            ))
            .wrapping_add(0),
            6,
            2,
            (1u8) as i32,
        );
        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
            .wrapping_add(8))
        .cast::<u8>())
        .wrapping_offset(
            (((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(1),
                2,
                4,
                false,
            ) as u8) as i32)
                .wrapping_sub(crate::c::div_i32(6i32, 2i32))) as isize
                * 4,
        ))
        .wrapping_add(2)
        .cast::<u16>())
        .write(((&raw mut gSpecialVar_0x8005).cast::<u16>()).read());
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
    }
}
pub(crate) unsafe extern "C" fn ShiftSavedApprentices() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut apprenticeNum: i32 = 0i32;
        let mut apprenticeIdx: i32 = 0i32;
        if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
            .cast::<u8>())
        .wrapping_add(56))
        .cast::<u8>())
        .read()) as i32)
            == 255i32
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
                    if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(1i32)) as isize * 68))
                    .wrapping_add(56))
                    .cast::<u8>())
                    .read()) as i32)
                        == 255i32
                    {
                        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset(((i).wrapping_add(1i32)) as isize * 68)
                        .cast::<crate::c::Rec4<68>>()
                        .write_unaligned(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>()
                            .cast::<crate::c::Rec4<68>>()
                            .read_unaligned(),
                        );
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        apprenticeNum = 65535i32;
        apprenticeIdx = (-1i32);
        {
            i = 1i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if (GetTrainerId(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 68))
                        .wrapping_add(52))
                        .cast::<u8>(),
                    ) == GetTrainerId(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                            .cast::<u8>(),
                    )) && ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 68))
                    .wrapping_add(2))
                    .read()) as i32)
                        < apprenticeNum)
                    {
                        apprenticeNum = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                            .read())
                        .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 68))
                        .wrapping_add(2))
                        .read()) as i32);
                        apprenticeIdx = i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if apprenticeIdx > 0i32 {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                .cast::<u8>())
            .wrapping_offset((apprenticeIdx) as isize * 68)
            .cast::<crate::c::Rec4<68>>()
            .write_unaligned(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<68>>()
                    .read_unaligned(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SaveApprentice() {
    unsafe {
        let mut i: u8 = 0u8;
        crate::c::bf_write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                .cast::<u8>())
            .wrapping_add(0),
            0,
            5,
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176)).read())
                as i32,
        );
        crate::c::bf_write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                .cast::<u8>())
            .wrapping_add(0),
            5,
            2,
            (crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176))
                    .wrapping_add(1),
                0,
                2,
                false,
            ) as u8) as i32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !((((i) as i32) < 9i32)
                    && (((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(176))
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(0),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        != 0i32))
                {
                    break 'l1;
                }
                'l2: {}
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220)).cast::<u8>())
            .wrapping_add(1))
        .write(i);
        if ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
            .cast::<u8>())
        .wrapping_add(2))
        .read()) as i32)
            < 255i32
        {
            let __p1 = (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                .cast::<u8>())
            .wrapping_add(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        SaveApprenticeParty(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                .cast::<u8>())
            .wrapping_add(1))
            .read(),
        );
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_add(52))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                .cast::<u8>())
            .wrapping_add(56))
            .cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220)).cast::<u8>())
            .wrapping_add(63))
        .write(((&raw mut gGameLanguage).cast::<u8>()).read());
        CalcApprenticeChecksum(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220)).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn SetSavedApprenticeTrainerGfxId() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut objectEventGfxId: u8 = 0u8;
        let mut class: u8 = (((((&raw const gApprentices).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                        .cast::<u8>())
                    .wrapping_add(0),
                    0,
                    5,
                    false,
                ) as u8) as i32) as isize
                    * 88,
            ))
        .wrapping_add(50))
        .read();
        {
            i = 0u8;
            'l1: loop {
                if !((((i) as u32) < crate::c::div_u32(30u32, 1u32))
                    && ((((((&raw mut gTowerMaleFacilityClasses).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((class) as i32)))
                {
                    break 'l1;
                }
                'l2: {}
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as u32) != crate::c::div_u32(30u32, 1u32) {
            objectEventGfxId = (((&raw mut gTowerMaleTrainerGfxIds).cast::<u8>())
                .wrapping_offset(((i) as i32) as isize))
            .read();
            VarSet(16400u16, ((objectEventGfxId) as u16));
            return;
        }
        {
            i = 0u8;
            'l3: loop {
                if !((((i) as u32) < crate::c::div_u32(20u32, 1u32))
                    && ((((((&raw mut gTowerFemaleFacilityClasses).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((class) as i32)))
                {
                    break 'l3;
                }
                'l4: {}
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as u32) != crate::c::div_u32(20u32, 1u32) {
            objectEventGfxId = (((&raw mut gTowerFemaleTrainerGfxIds).cast::<u8>())
                .wrapping_offset(((i) as i32) as isize))
            .read();
            VarSet(16400u16, ((objectEventGfxId) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn SetPlayerApprenticeTrainerGfxId() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut objectEventGfxId: u8 = 0u8;
        let mut class: u8 = (((((&raw const gApprentices).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(176)).read())
                    as i32) as isize
                    * 88,
            ))
        .wrapping_add(50))
        .read();
        {
            i = 0u8;
            'l1: loop {
                if !((((i) as u32) < crate::c::div_u32(30u32, 1u32))
                    && ((((((&raw mut gTowerMaleFacilityClasses).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((class) as i32)))
                {
                    break 'l1;
                }
                'l2: {}
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as u32) != crate::c::div_u32(30u32, 1u32) {
            objectEventGfxId = (((&raw mut gTowerMaleTrainerGfxIds).cast::<u8>())
                .wrapping_offset(((i) as i32) as isize))
            .read();
            VarSet(16400u16, ((objectEventGfxId) as u16));
            return;
        }
        {
            i = 0u8;
            'l3: loop {
                if !((((i) as u32) < crate::c::div_u32(20u32, 1u32))
                    && ((((((&raw mut gTowerFemaleFacilityClasses).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((class) as i32)))
                {
                    break 'l3;
                }
                'l4: {}
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as u32) != crate::c::div_u32(20u32, 1u32) {
            objectEventGfxId = (((&raw mut gTowerFemaleTrainerGfxIds).cast::<u8>())
                .wrapping_offset(((i) as i32) as isize))
            .read();
            VarSet(16400u16, ((objectEventGfxId) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn GetShouldCheckApprenticeGone() {
    unsafe {
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
    }
}
pub(crate) unsafe extern "C" fn GetShouldApprenticeLeave() {
    unsafe {
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetApprenticeNameInLanguage(apprenticeId: u32, language: i32) -> *mut u8 {
    unsafe {
        let mut apprenticeId = apprenticeId;
        let mut language = language;
        let mut apprentice: *mut u8 = (((&raw const gApprentices).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((apprenticeId) as i32) as isize * 88);
        'l1: {
            let __sw1 = language;
            let __matched = __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 7i32;
            if __sw1 == 1i32 {
                return ((apprentice).cast::<u8>()).cast::<u8>();
            }
            if __sw1 == 2i32 {
                return (((apprentice).cast::<u8>()).wrapping_offset(8)).cast::<u8>();
            }
            if __sw1 == 3i32 {
                return (((apprentice).cast::<u8>()).wrapping_offset(16)).cast::<u8>();
            }
            if __sw1 == 4i32 {
                return (((apprentice).cast::<u8>()).wrapping_offset(24)).cast::<u8>();
            }
            if __sw1 == 5i32 {
                return (((apprentice).cast::<u8>()).wrapping_offset(32)).cast::<u8>();
            }
            if __sw1 == 7i32 || !__matched {
                return (((apprentice).cast::<u8>()).wrapping_offset(40)).cast::<u8>();
            }
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchToFollowupFuncAfterButtonPress(taskId: u8) {
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
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ExecuteFuncAfterButtonPress(taskId: u8) {
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
            ((&raw mut gApprenticeFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(core::mem::transmute::<_, Option<unsafe extern "C" fn()>>(
                (((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16) as i32)
                    | (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 16)) as u32) as usize as *mut u8),
            ));
            (((&raw mut gApprenticeFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .unwrap_unchecked()();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ExecuteFuncAfterButtonPress(func: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut func = func;
        let mut taskId: u8 = CreateTask(Some(Task_ExecuteFuncAfterButtonPress), 1u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((core::mem::transmute::<_, usize>(func) as u32) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((((core::mem::transmute::<_, usize>(func) as u32) >> 16) as i16));
    }
}
pub(crate) unsafe extern "C" fn ExecuteFollowupFuncAfterButtonPress(
    task: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut task = task;
        let mut taskId: u8 = CreateTask(Some(Task_SwitchToFollowupFuncAfterButtonPress), 1u8);
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_SwitchToFollowupFuncAfterButtonPress),
            task,
        );
    }
}
