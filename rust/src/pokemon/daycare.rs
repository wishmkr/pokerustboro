//! Translated from `src/daycare.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gEggMoves sDaycareLevelMenuWindowTemplate sLevelMenuItems sDaycareListMenuLevelTemplate sCompatibilityMessages sJapaneseEggNickname
#[allow(unused_imports)]
use crate::data::daycare::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggLevelUpMoves: crate::ffi::Align4<[u8; 100]> =
    crate::ffi::Align4([0; 100]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggFatherMoves: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggFinalMoves: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggEggMoves: crate::ffi::Align4<[u8; 20]> =
    crate::ffi::Align4([0; 20]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggMotherMoves: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);

unsafe extern "C" {
    static mut gEvolutionTable: u8;
    static mut gMain: u8;
    static mut gMoveToLearn: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyCount: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gText_Exit4: u8;
    static mut gText_FemaleSymbol4: u8;
    static mut gText_GenderlessSymbol: u8;
    static mut gText_Lv: u8;
    static mut gText_MaleSymbol4: u8;
    static mut gText_NewLine2: u8;
    fn AddTextPrinter(a0: *mut u8, a1: u8, a2: Option<unsafe extern "C" fn(*mut u8, u16)>) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn BoxMonRestorePP(a0: *mut u8);
    fn BoxMonToMon(a0: *mut u8, a1: *mut u8);
    fn CB2_ReturnToField();
    fn CalculateMonStats(a0: *mut u8);
    fn CalculatePlayerPartyCount() -> u8;
    fn CanMonLearnTMHM(a0: *mut u8, a1: u8) -> u32;
    fn ChooseMonForDaycare();
    fn ClearMail(a0: *mut u8);
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn CompactPartySlots() -> i16;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeleteFirstMoveAndGiveMoveToMon(a0: *mut u8, a1: u16);
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroyTask(a0: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn FlagSet(a0: u16) -> u8;
    fn GetBoxMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetBoxMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxMonGender(a0: *mut u8) -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn GetEggCyclesToSubtract() -> u8;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetLevelFromBoxMonExp(a0: *mut u8) -> u8;
    fn GetLevelUpMovesBySpecies(a0: u16, a1: *mut u16) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GiveMailToMon(a0: *mut u8, a1: *mut u8) -> u8;
    fn GiveMoveToMon(a0: *mut u8, a1: u16) -> u16;
    fn ItemIdToBattleMoveId(a0: u16) -> u16;
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn MonHasMail(a0: *mut u8) -> u8;
    fn MonTryLearningNewMove(a0: *mut u8, a1: u8) -> u16;
    fn Random() -> u16;
    fn Random2() -> u16;
    fn RemoveWindow(a0: u8);
    fn ScriptContext_Enable();
    fn SeedRng2(a0: u16);
    fn SetBoxMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn TakeMailFromMon(a0: *mut u8);
    fn TryIncrementMonLevel(a0: *mut u8) -> u8;
    fn ZeroBoxMonData(a0: *mut u8);
    fn ZeroMonData(a0: *mut u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonNickname2(mon: *mut u8, dest: *mut u8) -> *mut u8 {
    unsafe {
        let mut mon = mon;
        let mut dest = dest;
        let mut nickname = crate::ffi::Align4([0u8; 20]);
        GetMonData3(mon, 2i32, (&raw mut nickname).cast::<u8>());
        return StringCopy_Nickname(dest, (&raw mut nickname).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonNickname(mon: *mut u8, dest: *mut u8) -> *mut u8 {
    unsafe {
        let mut mon = mon;
        let mut dest = dest;
        let mut nickname = crate::ffi::Align4([0u8; 20]);
        GetBoxMonData3(mon, 2i32, (&raw mut nickname).cast::<u8>());
        return StringCopy_Nickname(dest, (&raw mut nickname).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPokemonInDaycare(daycare: *mut u8) -> u8 {
    unsafe {
        let mut daycare = daycare;
        let mut i: u8 = 0u8;
        let mut count: u8 = 0u8;
        count = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    if GetBoxMonData2(
                        (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                        11i32,
                    ) != 0u32
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitDaycareMailRecordMixing(daycare: *mut u8, mixMail: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        let mut mixMail = mixMail;
        let mut i: u8 = 0u8;
        let mut numDaycareMons: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    if GetBoxMonData2(
                        (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                        11i32,
                    ) != 0u32
                    {
                        numDaycareMons = (numDaycareMons).wrapping_add(1);
                        if GetBoxMonData2(
                            (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                            12i32,
                        ) == 0u32
                        {
                            ((((mixMail).wrapping_add(116)).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(0u16);
                        } else {
                            ((((mixMail).wrapping_add(116)).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(1u16);
                        }
                    } else {
                        ((((mixMail).wrapping_add(116)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(1u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((mixMail).wrapping_add(112).cast::<u32>()).write(((numDaycareMons) as u32));
    }
}
pub(crate) unsafe extern "C" fn Daycare_FindEmptySpot(daycare: *mut u8) -> i8 {
    unsafe {
        let mut daycare = daycare;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    if GetBoxMonData2(
                        (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                        11i32,
                    ) == 0u32
                    {
                        return ((i) as i8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i8);
    }
}
pub(crate) unsafe extern "C" fn StorePokemonInDaycare(mon: *mut u8, daycareMon: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut daycareMon = daycareMon;
        if (MonHasMail(mon)) != 0 {
            let mut mailId: u8 = 0u8;
            StringCopy(
                (((daycareMon).wrapping_add(80)).wrapping_add(36)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            GetMonNickname2(
                mon,
                (((daycareMon).wrapping_add(80)).wrapping_add(44)).cast::<u8>(),
            );
            StripExtCtrlCodes((((daycareMon).wrapping_add(80)).wrapping_add(44)).cast::<u8>());
            crate::c::bf_write(
                ((daycareMon).wrapping_add(80)).wrapping_add(55),
                0,
                4,
                (2u8) as i32,
            );
            crate::c::bf_write(
                ((daycareMon).wrapping_add(80)).wrapping_add(55),
                4,
                4,
                ((GetMonData2(mon, 3i32)) as u8) as i32,
            );
            mailId = ((GetMonData2(mon, 64i32)) as u8);
            ((daycareMon).wrapping_add(80))
                .cast::<crate::c::Rec4<36>>()
                .write_unaligned(
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                        .cast::<u8>())
                    .wrapping_offset(((mailId) as i32) as isize * 36)
                    .cast::<crate::c::Rec4<36>>()
                    .read_unaligned(),
                );
            TakeMailFromMon(mon);
        }
        (daycareMon)
            .cast::<crate::c::Rec4<80>>()
            .write_unaligned((mon).cast::<crate::c::Rec4<80>>().read_unaligned());
        BoxMonRestorePP((daycareMon));
        ((daycareMon).wrapping_add(136).cast::<u32>()).write(0u32);
        ZeroMonData(mon);
        CompactPartySlots();
        CalculatePlayerPartyCount();
    }
}
pub(crate) unsafe extern "C" fn StorePokemonInEmptyDaycareSlot(mon: *mut u8, daycare: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut daycare = daycare;
        let mut slotId: i8 = Daycare_FindEmptySpot(daycare);
        StorePokemonInDaycare(
            mon,
            ((daycare).cast::<u8>()).wrapping_offset(((slotId) as i32) as isize * 140),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StoreSelectedPokemonInDaycare() {
    unsafe {
        let mut monId: u8 = GetCursorSelectionMonId();
        StorePokemonInEmptyDaycareSlot(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((monId) as i32) as isize * 100),
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
    }
}
pub(crate) unsafe extern "C" fn ShiftDaycareSlots(daycare: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        if (GetBoxMonData2((((daycare).cast::<u8>()).wrapping_offset(140)), 11i32) != 0u32)
            && (GetBoxMonData2(((daycare).cast::<u8>()), 11i32) == 0u32)
        {
            ((daycare).cast::<u8>())
                .cast::<crate::c::Rec4<80>>()
                .write_unaligned(
                    (((daycare).cast::<u8>()).wrapping_offset(140))
                        .cast::<crate::c::Rec4<80>>()
                        .read_unaligned(),
                );
            ZeroBoxMonData((((daycare).cast::<u8>()).wrapping_offset(140)));
            ((daycare).cast::<u8>())
                .wrapping_add(80)
                .cast::<crate::c::Rec4<56>>()
                .write_unaligned(
                    (((daycare).cast::<u8>()).wrapping_offset(140))
                        .wrapping_add(80)
                        .cast::<crate::c::Rec4<56>>()
                        .read_unaligned(),
                );
            (((daycare).cast::<u8>()).wrapping_add(136).cast::<u32>()).write(
                ((((daycare).cast::<u8>()).wrapping_offset(140))
                    .wrapping_add(136)
                    .cast::<u32>())
                .read(),
            );
            ((((daycare).cast::<u8>()).wrapping_offset(140))
                .wrapping_add(136)
                .cast::<u32>())
            .write(0u32);
            ClearDaycareMonMail((((daycare).cast::<u8>()).wrapping_offset(140)).wrapping_add(80));
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyDaycareExperience(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut i: i32 = 0i32;
        let mut firstMove: u8 = 0u8;
        let mut learnedMove: u16 = 0u16;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 100i32) {
                    break 'l1;
                }
                'l2: {
                    if (TryIncrementMonLevel(mon)) != 0 {
                        firstMove = 1u8;
                        'l3: loop {
                            if !((({
                                let __v1 = MonTryLearningNewMove(mon, firstMove);
                                learnedMove = __v1;
                                __v1
                            }) as i32)
                                != 0i32)
                            {
                                break 'l3;
                            }
                            firstMove = 0u8;
                            if ((learnedMove) as i32) == 65535i32 {
                                DeleteFirstMoveAndGiveMoveToMon(
                                    mon,
                                    ((&raw mut gMoveToLearn).cast::<u16>()).read(),
                                );
                            }
                        }
                    } else {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CalculateMonStats(mon);
    }
}
pub(crate) unsafe extern "C" fn TakeSelectedPokemonFromDaycare(daycareMon: *mut u8) -> u16 {
    unsafe {
        let mut daycareMon = daycareMon;
        let mut species: u16 = 0u16;
        let mut experience: u32 = 0u32;
        let mut pokemon = crate::ffi::Align4([0u8; 100]);
        GetBoxMonNickname((daycareMon), (&raw mut gStringVar1).cast::<u8>());
        species = ((GetBoxMonData2((daycareMon), 11i32)) as u16);
        BoxMonToMon((daycareMon), (&raw mut pokemon).cast::<u8>());
        if GetMonData2((&raw mut pokemon).cast::<u8>(), 56i32) != 100u32 {
            experience = (GetMonData2((&raw mut pokemon).cast::<u8>(), 25i32))
                .wrapping_add(((daycareMon).wrapping_add(136).cast::<u32>()).read());
            SetMonData(
                (&raw mut pokemon).cast::<u8>(),
                25i32,
                (&raw mut experience).cast::<u8>(),
            );
            ApplyDaycareExperience((&raw mut pokemon).cast::<u8>());
        }
        ((&raw mut gPlayerParty).cast::<u8>())
            .wrapping_offset(500)
            .cast::<crate::c::Rec4<100>>()
            .write_unaligned(
                (&raw mut pokemon)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<100>>()
                    .read_unaligned(),
            );
        if ((((daycareMon).wrapping_add(80))
            .wrapping_add(32)
            .cast::<u16>())
        .read())
            != 0
        {
            GiveMailToMon(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(500),
                ((daycareMon).wrapping_add(80)),
            );
            ClearDaycareMonMail((daycareMon).wrapping_add(80));
        }
        ZeroBoxMonData((daycareMon));
        ((daycareMon).wrapping_add(136).cast::<u32>()).write(0u32);
        CompactPartySlots();
        CalculatePlayerPartyCount();
        return species;
    }
}
pub(crate) unsafe extern "C" fn TakeSelectedPokemonMonFromDaycareShiftSlots(
    daycare: *mut u8,
    slotId: u8,
) -> u16 {
    unsafe {
        let mut daycare = daycare;
        let mut slotId = slotId;
        let mut species: u16 = TakeSelectedPokemonFromDaycare(
            ((daycare).cast::<u8>()).wrapping_offset(((slotId) as i32) as isize * 140),
        );
        ShiftDaycareSlots(daycare);
        return species;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TakePokemonFromDaycare() -> u16 {
    unsafe {
        return TakeSelectedPokemonMonFromDaycareShiftSlots(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn GetLevelAfterDaycareSteps(mon: *mut u8, steps: u32) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut steps = steps;
        let mut tempMon = crate::ffi::Align4([0u8; 80]);
        (&raw mut tempMon)
            .cast::<u8>()
            .cast::<crate::c::Rec4<80>>()
            .write_unaligned(mon.cast::<crate::c::Rec4<80>>().read_unaligned());
        let mut experience: u32 = (GetBoxMonData2(mon, 25i32)).wrapping_add(steps);
        SetBoxMonData(
            (&raw mut tempMon).cast::<u8>(),
            25i32,
            (&raw mut experience).cast::<u8>(),
        );
        return GetLevelFromBoxMonExp((&raw mut tempMon).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn GetNumLevelsGainedFromSteps(daycareMon: *mut u8) -> u8 {
    unsafe {
        let mut daycareMon = daycareMon;
        let mut levelBefore: u8 = 0u8;
        let mut levelAfter: u8 = 0u8;
        levelBefore = GetLevelFromBoxMonExp((daycareMon));
        levelAfter = GetLevelAfterDaycareSteps(
            (daycareMon),
            ((daycareMon).wrapping_add(136).cast::<u32>()).read(),
        );
        return ((((levelAfter) as i32).wrapping_sub(((levelBefore) as i32))) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetNumLevelsGainedForDaycareMon(daycareMon: *mut u8) -> u8 {
    unsafe {
        let mut daycareMon = daycareMon;
        let mut numLevelsGained: u8 = GetNumLevelsGainedFromSteps(daycareMon);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((numLevelsGained) as i32),
            0i32,
            2u8,
        );
        GetBoxMonNickname((daycareMon), (&raw mut gStringVar1).cast::<u8>());
        return numLevelsGained;
    }
}
pub(crate) unsafe extern "C" fn PrepareDaycareCostStringForSelectedMon(daycareMon: *mut u8) -> u32 {
    unsafe {
        let mut daycareMon = daycareMon;
        let mut cost: u32 = 0u32;
        let mut numLevelsGained: u8 = GetNumLevelsGainedFromSteps(daycareMon);
        GetBoxMonNickname((daycareMon), (&raw mut gStringVar1).cast::<u8>());
        cost = (((100i32).wrapping_add((100i32).wrapping_mul(((numLevelsGained) as i32)))) as u32);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((cost) as i32),
            0i32,
            5u8,
        );
        return cost;
    }
}
pub(crate) unsafe extern "C" fn PrepareDaycareCostStringForMon(
    daycare: *mut u8,
    slotId: u8,
) -> u16 {
    unsafe {
        let mut daycare = daycare;
        let mut slotId = slotId;
        return ((PrepareDaycareCostStringForSelectedMon(
            ((daycare).cast::<u8>()).wrapping_offset(((slotId) as i32) as isize * 140),
        )) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDaycareCostAndPrepareString() {
    unsafe {
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(PrepareDaycareCostStringForMon(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
        ));
    }
}
pub(crate) unsafe extern "C" fn Debug_AddDaycareSteps(numSteps: u16) {
    unsafe {
        let mut numSteps = numSteps;
        let __p1 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336))
            .cast::<u8>())
        .wrapping_add(136)
        .cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(((numSteps) as u32)));
        let __p2 = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336))
            .cast::<u8>())
        .wrapping_offset(140))
        .wrapping_add(136)
        .cast::<u32>();
        (__p2).write(((__p2).read()).wrapping_add(((numSteps) as u32)));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumLevelsGainedFromDaycare() -> u8 {
    unsafe {
        if GetBoxMonData2(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 140,
            )),
            11i32,
        ) != 0u32
        {
            return GetNumLevelsGainedForDaycareMon(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 140,
                ),
            );
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ClearDaycareMonMail(mail: *mut u8) {
    unsafe {
        let mut mail = mail;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    ((((mail).wrapping_add(36)).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 11i32) {
                    break 'l3;
                }
                'l4: {
                    ((((mail).wrapping_add(44)).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ClearMail((mail));
    }
}
pub(crate) unsafe extern "C" fn ClearDaycareMon(daycareMon: *mut u8) {
    unsafe {
        let mut daycareMon = daycareMon;
        ZeroBoxMonData((daycareMon));
        ((daycareMon).wrapping_add(136).cast::<u32>()).write(0u32);
        ClearDaycareMonMail((daycareMon).wrapping_add(80));
    }
}
pub(crate) unsafe extern "C" fn ClearAllDaycareData(daycare: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    ClearDaycareMon(
                        ((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((daycare).wrapping_add(280).cast::<u32>()).write(0u32);
        ((daycare).wrapping_add(284)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn GetEggSpecies(species: u16) -> u16 {
    unsafe {
        let mut species = species;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut found: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    found = 0u8;
                    {
                        j = 1i32;
                        'l3: loop {
                            if !(j < 412i32) {
                                break 'l3;
                            }
                            'l4: {
                                {
                                    k = 0i32;
                                    'l5: loop {
                                        if !(k < 5i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            if ((((((((&raw mut gEvolutionTable).cast::<u8>())
                                                .wrapping_offset((j) as isize * 40))
                                            .cast::<u8>())
                                            .wrapping_offset((k) as isize * 8))
                                            .wrapping_add(4)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                == ((species) as i32)
                                            {
                                                species = ((j) as u16);
                                                found = 1u8;
                                                break 'l5;
                                            }
                                        }
                                        k = (k).wrapping_add(1);
                                    }
                                }
                                if (found) != 0 {
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if j == 412i32 {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return species;
    }
}
pub(crate) unsafe extern "C" fn GetParentToInheritNature(daycare: *mut u8) -> i32 {
    unsafe {
        let mut daycare = daycare;
        let mut species = crate::ffi::Align4([0u8; 8]);
        let mut i: i32 = 0i32;
        let mut dittoCount: i32 = 0i32;
        let mut parent: i32 = (-1i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetBoxMonGender(
                        (((daycare).cast::<u8>()).wrapping_offset((i) as isize * 140)),
                    )) as i32)
                        == 254i32
                    {
                        parent = i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            dittoCount = 0i32;
            i = 0i32;
            'l3: loop {
                if !(i < 2i32) {
                    break 'l3;
                }
                'l4: {
                    (((&raw mut species).cast::<u32>()).wrapping_offset((i) as isize)).write(
                        GetBoxMonData2(
                            (((daycare).cast::<u8>()).wrapping_offset((i) as isize * 140)),
                            11i32,
                        ),
                    );
                    if (((&raw mut species).cast::<u32>()).wrapping_offset((i) as isize)).read()
                        == 132u32
                    {
                        dittoCount = (dittoCount).wrapping_add(1);
                        parent = i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if dittoCount == 2i32 {
            if ((Random()) as i32) >= crate::c::div_i32(65535i32, 2i32) {
                parent = 0i32;
            } else {
                parent = 1i32;
            }
        }
        if (GetBoxMonData2(
            (((daycare).cast::<u8>()).wrapping_offset((parent) as isize * 140)),
            12i32,
        ) != 195u32)
            || (((Random()) as i32) >= crate::c::div_i32(65535i32, 2i32))
        {
            return (-1i32);
        }
        return parent;
    }
}
pub(crate) unsafe extern "C" fn _TriggerPendingDaycareEgg(daycare: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        let mut parent: i32 = 0i32;
        let mut natureTries: i32 = 0i32;
        SeedRng2(
            (((((&raw mut gMain).cast::<u8>())
                .wrapping_add(36)
                .cast::<u32>())
            .read()) as u16),
        );
        parent = GetParentToInheritNature(daycare);
        if parent < 0i32 {
            ((daycare).wrapping_add(280).cast::<u32>()).write(
                (((((Random2()) as i32) << 16)
                    | (crate::c::rem_i32(((Random()) as i32), 65534i32)).wrapping_add(1i32))
                    as u32),
            );
        } else {
            let mut wantedNature: u8 = GetNatureFromPersonality(GetBoxMonData3(
                (((daycare).cast::<u8>()).wrapping_offset((parent) as isize * 140)),
                0i32,
                core::ptr::null_mut(),
            ));
            let mut personality: u32 = 0u32;
            'l1: loop {
                'l2: {
                    personality = (((((Random2()) as i32) << 16) | ((Random()) as i32)) as u32);
                    if (((wantedNature) as i32) == ((GetNatureFromPersonality(personality)) as i32))
                        && (personality != 0u32)
                    {
                        break 'l1;
                    }
                    natureTries = (natureTries).wrapping_add(1);
                }
                if !(natureTries <= 2400i32) {
                    break 'l1;
                }
            }
            ((daycare).wrapping_add(280).cast::<u32>()).write(personality);
        }
        FlagSet(134u16);
    }
}
pub(crate) unsafe extern "C" fn _TriggerPendingDaycareMaleEgg(daycare: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        ((daycare).wrapping_add(280).cast::<u32>())
            .write(((((Random()) as i32) | 32768i32) as u32));
        FlagSet(134u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TriggerPendingDaycareEgg() {
    unsafe {
        _TriggerPendingDaycareEgg(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
    }
}
pub(crate) unsafe extern "C" fn TriggerPendingDaycareMaleEgg() {
    unsafe {
        _TriggerPendingDaycareMaleEgg(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
    }
}
pub(crate) unsafe extern "C" fn RemoveIVIndexFromList(ivs: *mut u8, selectedIv: u8) {
    unsafe {
        let mut ivs = ivs;
        let mut selectedIv = selectedIv;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut temp = crate::ffi::Align4([0u8; 6]);
        ((ivs).wrapping_offset(((selectedIv) as i32) as isize)).write(255u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut temp).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(((ivs).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        j = 0i32;
        {
            i = 0i32;
            'l3: loop {
                if !(i < 6i32) {
                    break 'l3;
                }
                'l4: {
                    if (((((&raw mut temp).cast::<u8>()).wrapping_offset((i) as isize)).read())
                        as i32)
                        != 255i32
                    {
                        ((ivs).wrapping_offset(
                            ({
                                let __t1 = j;
                                j = (j).wrapping_add(1);
                                __t1
                            }) as isize,
                        ))
                        .write(
                            (((&raw mut temp).cast::<u8>()).wrapping_offset((i) as isize)).read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InheritIVs(egg: *mut u8, daycare: *mut u8) {
    unsafe {
        let mut egg = egg;
        let mut daycare = daycare;
        let mut i: u8 = 0u8;
        let mut selectedIvs = crate::ffi::Align4([0u8; 3]);
        let mut availableIVs = crate::ffi::Align4([0u8; 6]);
        let mut whichParents = crate::ffi::Align4([0u8; 3]);
        let mut iv: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut availableIVs).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l3;
                }
                'l4: {
                    (((&raw mut selectedIvs).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            (((&raw mut availableIVs).cast::<u8>()).wrapping_offset(
                                (crate::c::rem_i32(
                                    ((Random()) as i32),
                                    (6i32).wrapping_sub(((i) as i32)),
                                )) as isize,
                            ))
                            .read(),
                        );
                    RemoveIVIndexFromList((&raw mut availableIVs).cast::<u8>(), i);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l5;
                }
                'l6: {
                    (((&raw mut whichParents).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(((crate::c::rem_i32(((Random()) as i32), 2i32)) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l7;
                }
                'l8: {
                    'l9: {
                        let __sw1 = (((((&raw mut selectedIvs).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32);
                        if __sw1 == 0i32 {
                            iv = ((GetBoxMonData2(
                                (((daycare).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut whichParents).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 140,
                                )),
                                39i32,
                            )) as u8);
                            SetMonData(egg, 39i32, &raw mut iv);
                            break 'l9;
                        }
                        if __sw1 == 1i32 {
                            iv = ((GetBoxMonData2(
                                (((daycare).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut whichParents).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 140,
                                )),
                                40i32,
                            )) as u8);
                            SetMonData(egg, 40i32, &raw mut iv);
                            break 'l9;
                        }
                        if __sw1 == 2i32 {
                            iv = ((GetBoxMonData2(
                                (((daycare).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut whichParents).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 140,
                                )),
                                41i32,
                            )) as u8);
                            SetMonData(egg, 41i32, &raw mut iv);
                            break 'l9;
                        }
                        if __sw1 == 3i32 {
                            iv = ((GetBoxMonData2(
                                (((daycare).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut whichParents).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 140,
                                )),
                                42i32,
                            )) as u8);
                            SetMonData(egg, 42i32, &raw mut iv);
                            break 'l9;
                        }
                        if __sw1 == 4i32 {
                            iv = ((GetBoxMonData2(
                                (((daycare).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut whichParents).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 140,
                                )),
                                43i32,
                            )) as u8);
                            SetMonData(egg, 43i32, &raw mut iv);
                            break 'l9;
                        }
                        if __sw1 == 5i32 {
                            iv = ((GetBoxMonData2(
                                (((daycare).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut whichParents).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 140,
                                )),
                                44i32,
                            )) as u8);
                            SetMonData(egg, 44i32, &raw mut iv);
                            break 'l9;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetEggMoves(pokemon: *mut u8, eggMoves: *mut u16) -> u8 {
    unsafe {
        let mut pokemon = pokemon;
        let mut eggMoves = eggMoves;
        let mut eggMoveIdx: u16 = 0u16;
        let mut numEggMoves: u16 = 0u16;
        let mut species: u16 = 0u16;
        let mut i: u16 = 0u16;
        numEggMoves = 0u16;
        eggMoveIdx = 0u16;
        species = ((GetMonData2(pokemon, 11i32)) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < (crate::c::div_u32(2278u32, 2u32)).wrapping_sub(1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const gEggMoves).cast::<u8>().cast_mut().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((species) as i32).wrapping_add(20000i32)
                    {
                        eggMoveIdx = ((((i) as i32).wrapping_add(1i32)) as u16);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l3;
                }
                'l4: {
                    if ((((((&raw const gEggMoves).cast::<u8>().cast_mut().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset((((eggMoveIdx) as i32).wrapping_add(((i) as i32))) as isize))
                    .read()) as i32)
                        > 20000i32
                    {
                        break 'l3;
                    }
                    ((eggMoves).wrapping_offset(((i) as i32) as isize)).write(
                        ((((&raw const gEggMoves).cast::<u8>().cast_mut().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(
                            (((eggMoveIdx) as i32).wrapping_add(((i) as i32))) as isize,
                        ))
                        .read(),
                    );
                    numEggMoves = (numEggMoves).wrapping_add(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((numEggMoves) as u8);
    }
}
pub(crate) unsafe extern "C" fn BuildEggMoveset(egg: *mut u8, father: *mut u8, mother: *mut u8) {
    unsafe {
        let mut egg = egg;
        let mut father = father;
        let mut mother = mother;
        let mut numSharedParentMoves: u16 = 0u16;
        let mut numLevelUpMoves: u32 = 0u32;
        let mut numEggMoves: u16 = 0u16;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        numSharedParentMoves = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sHatchedEggMotherMoves).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                    ((((&raw mut sHatchedEggFatherMoves).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                    ((((&raw mut sHatchedEggFinalMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut sHatchedEggEggMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l5: loop {
                if !(((i) as i32) < (if 20i32 > 50i32 { 20i32 } else { 50i32 })) {
                    break 'l5;
                }
                'l6: {
                    ((((&raw mut sHatchedEggLevelUpMoves)
                        .cast::<u8>()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        numLevelUpMoves = ((GetLevelUpMovesBySpecies(
            ((GetMonData2(egg, 11i32)) as u16),
            ((&raw mut sHatchedEggLevelUpMoves)
                .cast::<u8>()
                .cast::<u16>())
            .cast::<u16>(),
        )) as u32);
        {
            i = 0u16;
            'l7: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l7;
                }
                'l8: {
                    ((((&raw mut sHatchedEggFatherMoves).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(((GetBoxMonData2(father, (13i32).wrapping_add(((i) as i32)))) as u16));
                    ((((&raw mut sHatchedEggMotherMoves).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(((GetBoxMonData2(mother, (13i32).wrapping_add(((i) as i32)))) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        numEggMoves = ((GetEggMoves(
            egg,
            ((&raw mut sHatchedEggEggMoves).cast::<u8>().cast::<u16>()).cast::<u16>(),
        )) as u16);
        {
            i = 0u16;
            'l9: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l9;
                }
                'l10: {
                    if ((((((&raw mut sHatchedEggFatherMoves).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        {
                            j = 0u16;
                            'l11: loop {
                                if !(((j) as i32) < ((numEggMoves) as i32)) {
                                    break 'l11;
                                }
                                'l12: {
                                    if ((((((&raw mut sHatchedEggFatherMoves)
                                        .cast::<u8>()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        == ((((((&raw mut sHatchedEggEggMoves)
                                            .cast::<u8>()
                                            .cast::<u16>())
                                        .cast::<u16>())
                                        .wrapping_offset(((j) as i32) as isize))
                                        .read()) as i32)
                                    {
                                        if ((GiveMoveToMon(
                                            egg,
                                            ((((&raw mut sHatchedEggFatherMoves)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read(),
                                        )) as i32)
                                            == 65535i32
                                        {
                                            DeleteFirstMoveAndGiveMoveToMon(
                                                egg,
                                                ((((&raw mut sHatchedEggFatherMoves)
                                                    .cast::<u8>()
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(((i) as i32) as isize))
                                                .read(),
                                            );
                                        }
                                        break 'l11;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    } else {
                        break 'l9;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l13: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l13;
                }
                'l14: {
                    if ((((((&raw mut sHatchedEggFatherMoves).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        {
                            j = 0u16;
                            'l15: loop {
                                if !(((j) as i32) < 58i32) {
                                    break 'l15;
                                }
                                'l16: {
                                    if (((((((&raw mut sHatchedEggFatherMoves)
                                        .cast::<u8>()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        == ((ItemIdToBattleMoveId(
                                            (((289i32).wrapping_add(((j) as i32))) as u16),
                                        )) as i32))
                                        && ((CanMonLearnTMHM(egg, ((j) as u8))) != 0)
                                    {
                                        if ((GiveMoveToMon(
                                            egg,
                                            ((((&raw mut sHatchedEggFatherMoves)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read(),
                                        )) as i32)
                                            == 65535i32
                                        {
                                            DeleteFirstMoveAndGiveMoveToMon(
                                                egg,
                                                ((((&raw mut sHatchedEggFatherMoves)
                                                    .cast::<u8>()
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(((i) as i32) as isize))
                                                .read(),
                                            );
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
        {
            i = 0u16;
            'l17: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l17;
                }
                'l18: {
                    if ((((((&raw mut sHatchedEggFatherMoves).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        break 'l17;
                    }
                    {
                        j = 0u16;
                        'l19: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l19;
                            }
                            'l20: {
                                if (((((((&raw mut sHatchedEggFatherMoves)
                                    .cast::<u8>()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    == ((((((&raw mut sHatchedEggMotherMoves)
                                        .cast::<u8>()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read()) as i32))
                                    && (((((((&raw mut sHatchedEggFatherMoves)
                                        .cast::<u8>()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        != 0i32)
                                {
                                    ((((&raw mut sHatchedEggFinalMoves)
                                        .cast::<u8>()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(
                                        (({
                                            let __t1 = numSharedParentMoves;
                                            numSharedParentMoves =
                                                (numSharedParentMoves).wrapping_add(1);
                                            __t1
                                        }) as i32) as isize,
                                    ))
                                    .write(
                                        ((((&raw mut sHatchedEggFatherMoves)
                                            .cast::<u8>()
                                            .cast::<u16>())
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read(),
                                    );
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l21: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l21;
                }
                'l22: {
                    if ((((((&raw mut sHatchedEggFinalMoves).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        break 'l21;
                    }
                    {
                        j = 0u16;
                        'l23: loop {
                            if !(((j) as u32) < numLevelUpMoves) {
                                break 'l23;
                            }
                            'l24: {
                                if (((((((&raw mut sHatchedEggLevelUpMoves)
                                    .cast::<u8>()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .read()) as i32)
                                    != 0i32)
                                    && (((((((&raw mut sHatchedEggFinalMoves)
                                        .cast::<u8>()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        == ((((((&raw mut sHatchedEggLevelUpMoves)
                                            .cast::<u8>()
                                            .cast::<u16>())
                                        .cast::<u16>())
                                        .wrapping_offset(((j) as i32) as isize))
                                        .read())
                                            as i32))
                                {
                                    if ((GiveMoveToMon(
                                        egg,
                                        ((((&raw mut sHatchedEggFinalMoves)
                                            .cast::<u8>()
                                            .cast::<u16>())
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read(),
                                    )) as i32)
                                        == 65535i32
                                    {
                                        DeleteFirstMoveAndGiveMoveToMon(
                                            egg,
                                            ((((&raw mut sHatchedEggFinalMoves)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read(),
                                        );
                                    }
                                    break 'l23;
                                }
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
pub(crate) unsafe extern "C" fn RemoveEggFromDayCare(daycare: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        ((daycare).wrapping_add(280).cast::<u32>()).write(0u32);
        ((daycare).wrapping_add(284)).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RejectEggFromDayCare() {
    unsafe {
        RemoveEggFromDayCare(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
    }
}
pub(crate) unsafe extern "C" fn AlterEggSpeciesWithIncenseItem(
    species: *mut u16,
    daycare: *mut u8,
) {
    unsafe {
        let mut species = species;
        let mut daycare = daycare;
        let mut motherItem: u16 = 0u16;
        let mut fatherItem: u16 = 0u16;
        if ((((species).read()) as i32) == 360i32) || ((((species).read()) as i32) == 350i32) {
            motherItem = ((GetBoxMonData2(((daycare).cast::<u8>()), 12i32)) as u16);
            fatherItem =
                ((GetBoxMonData2((((daycare).cast::<u8>()).wrapping_offset(140)), 12i32)) as u16);
            if (((((species).read()) as i32) == 360i32) && (((motherItem) as i32) != 221i32))
                && (((fatherItem) as i32) != 221i32)
            {
                (species).write(202u16);
            }
            if (((((species).read()) as i32) == 350i32) && (((motherItem) as i32) != 220i32))
                && (((fatherItem) as i32) != 220i32)
            {
                (species).write(183u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GiveVoltTackleIfLightBall(mon: *mut u8, daycare: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut daycare = daycare;
        let mut motherItem: u32 = GetBoxMonData2(((daycare).cast::<u8>()), 12i32);
        let mut fatherItem: u32 =
            GetBoxMonData2((((daycare).cast::<u8>()).wrapping_offset(140)), 12i32);
        if (motherItem == 202u32) || (fatherItem == 202u32) {
            if ((GiveMoveToMon(mon, 344u16)) as i32) == 65535i32 {
                DeleteFirstMoveAndGiveMoveToMon(mon, 344u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DetermineEggSpeciesAndParentSlots(
    daycare: *mut u8,
    parentSlots: *mut u8,
) -> u16 {
    unsafe {
        let mut daycare = daycare;
        let mut parentSlots = parentSlots;
        let mut i: u16 = 0u16;
        let mut species = crate::ffi::Align4([0u8; 4]);
        let mut eggSpecies: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut species).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((GetBoxMonData2(
                                (((daycare).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 140)),
                                11i32,
                            )) as u16),
                        );
                    if (((((&raw mut species).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 132i32
                    {
                        (parentSlots).write(((((i) as i32) ^ 1i32) as u8));
                        ((parentSlots).wrapping_offset(1)).write(((i) as u8));
                    } else {
                        if ((GetBoxMonGender(
                            (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                        )) as i32)
                            == 254i32
                        {
                            (parentSlots).write(((i) as u8));
                            ((parentSlots).wrapping_offset(1)).write(((((i) as i32) ^ 1i32) as u8));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        eggSpecies = GetEggSpecies(
            (((&raw mut species).cast::<u16>())
                .wrapping_offset((((parentSlots).read()) as i32) as isize))
            .read(),
        );
        if (((eggSpecies) as i32) == 29i32)
            && ((((daycare).wrapping_add(280).cast::<u32>()).read() & 32768u32) != 0)
        {
            eggSpecies = 32u16;
        }
        if (((eggSpecies) as i32) == 387i32)
            && ((((daycare).wrapping_add(280).cast::<u32>()).read() & 32768u32) != 0)
        {
            eggSpecies = 386u16;
        }
        if ((((((&raw mut species).cast::<u16>())
            .wrapping_offset(((((parentSlots).wrapping_offset(1)).read()) as i32) as isize))
        .read()) as i32)
            == 132i32)
            && (((GetBoxMonGender(
                (((daycare).cast::<u8>())
                    .wrapping_offset((((parentSlots).read()) as i32) as isize * 140)),
            )) as i32)
                != 254i32)
        {
            let mut ditto: u8 = ((parentSlots).wrapping_offset(1)).read();
            ((parentSlots).wrapping_offset(1)).write((parentSlots).read());
            (parentSlots).write(ditto);
        }
        return eggSpecies;
    }
}
pub(crate) unsafe extern "C" fn _GiveEggFromDaycare(daycare: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        let mut egg = crate::ffi::Align4([0u8; 100]);
        let mut species: u16 = 0u16;
        let mut parentSlots = crate::ffi::Align4([0u8; 2]);
        let mut isEgg: u8 = 0u8;
        species = DetermineEggSpeciesAndParentSlots(daycare, (&raw mut parentSlots).cast::<u8>());
        AlterEggSpeciesWithIncenseItem(&raw mut species, daycare);
        SetInitialEggData((&raw mut egg).cast::<u8>(), species, daycare);
        InheritIVs((&raw mut egg).cast::<u8>(), daycare);
        BuildEggMoveset(
            (&raw mut egg).cast::<u8>(),
            (((daycare).cast::<u8>()).wrapping_offset(
                (((((&raw mut parentSlots).cast::<u8>()).wrapping_offset(1)).read()) as i32)
                    as isize
                    * 140,
            )),
            (((daycare).cast::<u8>()).wrapping_offset(
                ((((&raw mut parentSlots).cast::<u8>()).read()) as i32) as isize * 140,
            )),
        );
        if ((species) as i32) == 172i32 {
            GiveVoltTackleIfLightBall((&raw mut egg).cast::<u8>(), daycare);
        }
        isEgg = 1u8;
        SetMonData((&raw mut egg).cast::<u8>(), 45i32, &raw mut isEgg);
        ((&raw mut gPlayerParty).cast::<u8>())
            .wrapping_offset(500)
            .cast::<crate::c::Rec4<100>>()
            .write_unaligned(
                (&raw mut egg)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<100>>()
                    .read_unaligned(),
            );
        CompactPartySlots();
        CalculatePlayerPartyCount();
        RemoveEggFromDayCare(daycare);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateEgg(mon: *mut u8, species: u16, setHotSpringsLocation: u8) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut setHotSpringsLocation = setHotSpringsLocation;
        let mut metLevel: u8 = 0u8;
        let mut ball: u16 = 0u16;
        let mut language: u8 = 0u8;
        let mut metLocation: u8 = 0u8;
        let mut isEgg: u8 = 0u8;
        CreateMon(mon, species, 5u8, 32u8, 0u8, 0u32, 0u8, 0u32);
        metLevel = 0u8;
        ball = 4u16;
        language = 1u8;
        SetMonData(mon, 38i32, (&raw mut ball).cast::<u8>());
        SetMonData(
            mon,
            2i32,
            ((&raw const sJapaneseEggNickname).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        SetMonData(
            mon,
            32i32,
            (((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(17),
        );
        SetMonData(mon, 36i32, &raw mut metLevel);
        SetMonData(mon, 3i32, &raw mut language);
        if (setHotSpringsLocation) != 0 {
            metLocation = 253u8;
            SetMonData(mon, 35i32, &raw mut metLocation);
        }
        isEgg = 1u8;
        SetMonData(mon, 45i32, &raw mut isEgg);
    }
}
pub(crate) unsafe extern "C" fn SetInitialEggData(mon: *mut u8, species: u16, daycare: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut daycare = daycare;
        let mut personality: u32 = 0u32;
        let mut ball: u16 = 0u16;
        let mut metLevel: u8 = 0u8;
        let mut language: u8 = 0u8;
        personality = ((daycare).wrapping_add(280).cast::<u32>()).read();
        CreateMon(mon, species, 5u8, 32u8, 1u8, personality, 0u8, 0u32);
        metLevel = 0u8;
        ball = 4u16;
        language = 1u8;
        SetMonData(mon, 38i32, (&raw mut ball).cast::<u8>());
        SetMonData(
            mon,
            2i32,
            ((&raw const sJapaneseEggNickname).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        SetMonData(
            mon,
            32i32,
            (((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(17),
        );
        SetMonData(mon, 36i32, &raw mut metLevel);
        SetMonData(mon, 3i32, &raw mut language);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveEggFromDaycare() {
    unsafe {
        _GiveEggFromDaycare(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
    }
}
pub(crate) unsafe extern "C" fn TryProduceOrHatchEgg(daycare: *mut u8) -> u8 {
    unsafe {
        let mut daycare = daycare;
        let mut i: u32 = 0u32;
        let mut validEggs: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < 2u32) {
                    break 'l1;
                }
                'l2: {
                    if (GetBoxMonData2(
                        (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                        5i32,
                    )) != 0
                    {
                        let __p1 = (((daycare).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 140))
                        .wrapping_add(136)
                        .cast::<u32>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                        validEggs = (validEggs).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((daycare).wrapping_add(280).cast::<u32>()).read() == 0u32) && (validEggs == 2u32))
            && ((((((daycare).cast::<u8>()).wrapping_offset(140))
                .wrapping_add(136)
                .cast::<u32>())
            .read()
                & 255u32)
                == 255u32)
        {
            let mut compatibility: u8 = GetDaycareCompatibilityScore(daycare);
            if ((compatibility) as u32)
                > crate::c::div_u32(((Random()) as u32).wrapping_mul(100u32), 65535u32)
            {
                TriggerPendingDaycareEgg();
            }
        }
        if (({
            let __p2 = (daycare).wrapping_add(284);
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == 255i32
        {
            let mut eggCycles: u32 = 0u32;
            let mut toSub: u8 = GetEggCyclesToSubtract();
            {
                i = 0u32;
                'l3: loop {
                    if !(i < ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as u32)) {
                        break 'l3;
                    }
                    'l4: {
                        if !((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            45i32,
                        )) != 0)
                        {
                            break 'l4;
                        }
                        if (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            4i32,
                        )) != 0
                        {
                            break 'l4;
                        }
                        eggCycles = GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            32i32,
                        );
                        if eggCycles != 0u32 {
                            if eggCycles >= ((toSub) as u32) {
                                eggCycles = (eggCycles).wrapping_sub(((toSub) as u32));
                            } else {
                                eggCycles = (eggCycles).wrapping_sub(1u32);
                            }
                            SetMonData(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                32i32,
                                (&raw mut eggCycles).cast::<u8>(),
                            );
                        } else {
                            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(((i) as u16));
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldEggHatch() -> u8 {
    unsafe {
        return TryProduceOrHatchEgg(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
    }
}
pub(crate) unsafe extern "C" fn IsEggPending(daycare: *mut u8) -> u8 {
    unsafe {
        let mut daycare = daycare;
        return ((((daycare).wrapping_add(280).cast::<u32>()).read() != 0u32) as u8);
    }
}
pub(crate) unsafe extern "C" fn _GetDaycareMonNicknames(daycare: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        let mut otName = crate::ffi::Align4([0u8; 12]);
        if GetBoxMonData2(((daycare).cast::<u8>()), 11i32) != 0u32 {
            GetBoxMonNickname(
                ((daycare).cast::<u8>()),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            GetBoxMonData3(
                ((daycare).cast::<u8>()),
                7i32,
                (&raw mut otName).cast::<u8>(),
            );
            StringCopy(
                (&raw mut gStringVar3).cast::<u8>(),
                (&raw mut otName).cast::<u8>(),
            );
        }
        if GetBoxMonData2((((daycare).cast::<u8>()).wrapping_offset(140)), 11i32) != 0u32 {
            GetBoxMonNickname(
                (((daycare).cast::<u8>()).wrapping_offset(140)),
                (&raw mut gStringVar2).cast::<u8>(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSelectedMonNicknameAndSpecies() -> u16 {
    unsafe {
        GetBoxMonNickname(
            (((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetCursorSelectionMonId()) as i32) as isize * 100)),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        return ((GetBoxMonData2(
            (((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetCursorSelectionMonId()) as i32) as isize * 100)),
            11i32,
        )) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDaycareMonNicknames() {
    unsafe {
        _GetDaycareMonNicknames(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDaycareState() -> u8 {
    unsafe {
        let mut numMons: u8 = 0u8;
        if (IsEggPending(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        )) != 0
        {
            return 1u8;
        }
        numMons = CountPokemonInDaycare(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
        if ((numMons) as i32) != 0i32 {
            return ((((numMons) as i32).wrapping_add(1i32)) as u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetDaycarePokemonCount() -> u8 {
    unsafe {
        let mut ret: u8 = CountPokemonInDaycare(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
        if (ret) != 0 {
            return ret;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn EggGroupsOverlap(eggGroups1: *mut u16, eggGroups2: *mut u16) -> u8 {
    unsafe {
        let mut eggGroups1 = eggGroups1;
        let mut eggGroups2 = eggGroups2;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 2i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((eggGroups1).wrapping_offset((i) as isize)).read()) as i32)
                                    == ((((eggGroups2).wrapping_offset((j) as isize)).read())
                                        as i32)
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
pub(crate) unsafe extern "C" fn GetDaycareCompatibilityScore(daycare: *mut u8) -> u8 {
    unsafe {
        let mut daycare = daycare;
        let mut i: u32 = 0u32;
        let mut eggGroups = crate::ffi::Align4([0u8; 8]);
        let mut species = crate::ffi::Align4([0u8; 4]);
        let mut trainerIds = crate::ffi::Align4([0u8; 8]);
        let mut genders = crate::ffi::Align4([0u8; 8]);
        {
            i = 0u32;
            'l1: loop {
                if !(i < 2u32) {
                    break 'l1;
                }
                'l2: {
                    let mut personality: u32 = 0u32;
                    (((&raw mut species).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((GetBoxMonData2(
                                (((daycare).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 140)),
                                11i32,
                            )) as u16),
                        );
                    (((&raw mut trainerIds).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                        .write(GetBoxMonData2(
                            (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                            1i32,
                        ));
                    personality = GetBoxMonData2(
                        (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                        0i32,
                    );
                    (((&raw mut genders).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((GetGenderFromSpeciesAndPersonality(
                                (((&raw mut species).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                personality,
                            )) as u32),
                        );
                    ((((&raw mut eggGroups).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .write(
                        (((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                            (((((&raw mut species).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(20))
                        .cast::<u8>())
                        .read()) as u16),
                    );
                    (((((&raw mut eggGroups).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(
                        ((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                            (((((&raw mut species).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((&raw mut eggGroups).cast::<u8>()).cast::<u16>()).read()) as i32) == 15i32)
            || (((((((&raw mut eggGroups).cast::<u8>()).wrapping_offset(4)).cast::<u16>()).read())
                as i32)
                == 15i32)
        {
            return 0u8;
        }
        if ((((((&raw mut eggGroups).cast::<u8>()).cast::<u16>()).read()) as i32) == 13i32)
            && (((((((&raw mut eggGroups).cast::<u8>()).wrapping_offset(4)).cast::<u16>()).read())
                as i32)
                == 13i32)
        {
            return 0u8;
        }
        if ((((((&raw mut eggGroups).cast::<u8>()).cast::<u16>()).read()) as i32) == 13i32)
            || (((((((&raw mut eggGroups).cast::<u8>()).wrapping_offset(4)).cast::<u16>()).read())
                as i32)
                == 13i32)
        {
            if ((&raw mut trainerIds).cast::<u32>()).read()
                == (((&raw mut trainerIds).cast::<u32>()).wrapping_offset(1)).read()
            {
                return 20u8;
            }
            return 50u8;
        } else {
            if ((&raw mut genders).cast::<u32>()).read()
                == (((&raw mut genders).cast::<u32>()).wrapping_offset(1)).read()
            {
                return 0u8;
            }
            if (((&raw mut genders).cast::<u32>()).read() == 255u32)
                || ((((&raw mut genders).cast::<u32>()).wrapping_offset(1)).read() == 255u32)
            {
                return 0u8;
            }
            if !((EggGroupsOverlap(
                ((&raw mut eggGroups).cast::<u8>()).cast::<u16>(),
                (((&raw mut eggGroups).cast::<u8>()).wrapping_offset(4)).cast::<u16>(),
            )) != 0)
            {
                return 0u8;
            }
            if ((((&raw mut species).cast::<u16>()).read()) as i32)
                == (((((&raw mut species).cast::<u16>()).wrapping_offset(1)).read()) as i32)
            {
                if ((&raw mut trainerIds).cast::<u32>()).read()
                    == (((&raw mut trainerIds).cast::<u32>()).wrapping_offset(1)).read()
                {
                    return 50u8;
                }
                return 70u8;
            } else {
                if ((&raw mut trainerIds).cast::<u32>()).read()
                    != (((&raw mut trainerIds).cast::<u32>()).wrapping_offset(1)).read()
                {
                    return 50u8;
                }
                return 20u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetDaycareCompatibilityScoreFromSave() -> u8 {
    unsafe {
        return GetDaycareCompatibilityScore(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDaycareCompatibilityString() {
    unsafe {
        let mut whichString: u8 = 0u8;
        let mut relationshipScore: u8 = 0u8;
        relationshipScore = GetDaycareCompatibilityScoreFromSave();
        whichString = 0u8;
        if ((relationshipScore) as i32) == 0i32 {
            whichString = 3u8;
        }
        if ((relationshipScore) as i32) == 20i32 {
            whichString = 2u8;
        }
        if ((relationshipScore) as i32) == 50i32 {
            whichString = 1u8;
        }
        if ((relationshipScore) as i32) == 70i32 {
            whichString = 0u8;
        }
        StringCopy(
            (&raw mut gStringVar4).cast::<u8>(),
            ((((&raw const sCompatibilityMessages)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((whichString) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn NameHasGenderSymbol(name: *mut u8, genderRatio: u8) -> u8 {
    unsafe {
        let mut name = name;
        let mut genderRatio = genderRatio;
        let mut i: u8 = 0u8;
        let mut symbolsCount = crate::ffi::Align4([0u8; 2]);
        ((&raw mut symbolsCount).cast::<u8>()).write({
            let __v1 = 0u8;
            (((&raw mut symbolsCount).cast::<u8>()).wrapping_offset(1)).write(__v1);
            __v1
        });
        {
            i = 0u8;
            'l1: loop {
                if !(((((name).wrapping_offset(((i) as i32) as isize)).read()) as i32) != 255i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((name).wrapping_offset(((i) as i32) as isize)).read()) as i32) == 181i32 {
                        let __p2 = (&raw mut symbolsCount).cast::<u8>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                    if ((((name).wrapping_offset(((i) as i32) as isize)).read()) as i32) == 182i32 {
                        let __p3 = ((&raw mut symbolsCount).cast::<u8>()).wrapping_offset(1);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((genderRatio) as i32) == 0i32)
            && (((((&raw mut symbolsCount).cast::<u8>()).read()) as i32) != 0i32))
            && ((((((&raw mut symbolsCount).cast::<u8>()).wrapping_offset(1)).read()) as i32)
                == 0i32)
        {
            return 1u8;
        }
        if ((((genderRatio) as i32) == 254i32)
            && ((((((&raw mut symbolsCount).cast::<u8>()).wrapping_offset(1)).read()) as i32)
                != 0i32))
            && (((((&raw mut symbolsCount).cast::<u8>()).read()) as i32) == 0i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AppendGenderSymbol(name: *mut u8, gender: u8) -> *mut u8 {
    unsafe {
        let mut name = name;
        let mut gender = gender;
        if ((gender) as i32) == 0i32 {
            if !((NameHasGenderSymbol(name, 0u8)) != 0) {
                return StringAppend(name, (&raw mut gText_MaleSymbol4).cast::<u8>());
            }
        } else {
            if ((gender) as i32) == 254i32 {
                if !((NameHasGenderSymbol(name, 254u8)) != 0) {
                    return StringAppend(name, (&raw mut gText_FemaleSymbol4).cast::<u8>());
                }
            }
        }
        return StringAppend(name, (&raw mut gText_GenderlessSymbol).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn AppendMonGenderSymbol(name: *mut u8, boxMon: *mut u8) -> *mut u8 {
    unsafe {
        let mut name = name;
        let mut boxMon = boxMon;
        return AppendGenderSymbol(name, GetBoxMonGender(boxMon));
    }
}
pub(crate) unsafe extern "C" fn GetDaycareLevelMenuText(daycare: *mut u8, dest: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        let mut dest = dest;
        let mut monNames = crate::ffi::Align4([0u8; 40]);
        let mut i: u8 = 0u8;
        (dest).write(255u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    GetBoxMonNickname(
                        (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                        (((&raw mut monNames).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                        .cast::<u8>(),
                    );
                    AppendMonGenderSymbol(
                        (((&raw mut monNames).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                        .cast::<u8>(),
                        (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy(dest, ((&raw mut monNames).cast::<u8>()).cast::<u8>());
        StringAppend(dest, (&raw mut gText_NewLine2).cast::<u8>());
        StringAppend(
            dest,
            (((&raw mut monNames).cast::<u8>()).wrapping_offset(20)).cast::<u8>(),
        );
        StringAppend(dest, (&raw mut gText_NewLine2).cast::<u8>());
        StringAppend(dest, (&raw mut gText_Exit4).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn GetDaycareLevelMenuLevelText(daycare: *mut u8, dest: *mut u8) {
    unsafe {
        let mut daycare = daycare;
        let mut dest = dest;
        let mut i: u8 = 0u8;
        let mut level: u8 = 0u8;
        let mut text = crate::ffi::Align4([0u8; 20]);
        (dest).write(255u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    StringAppend(dest, (&raw mut gText_Lv).cast::<u8>());
                    level = GetLevelAfterDaycareSteps(
                        (((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140)),
                        ((((daycare).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 140))
                            .wrapping_add(136)
                            .cast::<u32>())
                        .read(),
                    );
                    ConvertIntToDecimalStringN(
                        (&raw mut text).cast::<u8>(),
                        ((level) as i32),
                        0i32,
                        3u8,
                    );
                    StringAppend(dest, (&raw mut text).cast::<u8>());
                    StringAppend(dest, (&raw mut gText_NewLine2).cast::<u8>());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DaycareAddTextPrinter(windowId: u8, text: *mut u8, x: u32, y: u32) {
    unsafe {
        let mut windowId = windowId;
        let mut text = text;
        let mut x = x;
        let mut y = y;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printer).cast::<u8>()).cast::<*mut u8>()).write(text);
        (((&raw mut printer).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(1u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(((x) as u8));
        (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(((y) as u8));
        (((&raw mut printer).cast::<u8>()).wrapping_add(8)).write(((x) as u8));
        (((&raw mut printer).cast::<u8>()).wrapping_add(9)).write(((y) as u8));
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            1,
            1,
            (0u8) as i32,
        );
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(1u8);
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (2u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (3u8) as i32,
        );
        AddTextPrinter((&raw mut printer).cast::<u8>(), 255u8, None);
    }
}
pub(crate) unsafe extern "C" fn DaycarePrintMonNickname(
    daycare: *mut u8,
    windowId: u8,
    daycareSlotId: u32,
    y: u32,
) {
    unsafe {
        let mut daycare = daycare;
        let mut windowId = windowId;
        let mut daycareSlotId = daycareSlotId;
        let mut y = y;
        let mut nickname = crate::ffi::Align4([0u8; 20]);
        GetBoxMonNickname(
            (((daycare).cast::<u8>()).wrapping_offset(((daycareSlotId) as i32) as isize * 140)),
            (&raw mut nickname).cast::<u8>(),
        );
        AppendMonGenderSymbol(
            (&raw mut nickname).cast::<u8>(),
            (((daycare).cast::<u8>()).wrapping_offset(((daycareSlotId) as i32) as isize * 140)),
        );
        DaycareAddTextPrinter(windowId, (&raw mut nickname).cast::<u8>(), 8u32, y);
    }
}
pub(crate) unsafe extern "C" fn DaycarePrintMonLvl(
    daycare: *mut u8,
    windowId: u8,
    daycareSlotId: u32,
    y: u32,
) {
    unsafe {
        let mut daycare = daycare;
        let mut windowId = windowId;
        let mut daycareSlotId = daycareSlotId;
        let mut y = y;
        let mut level: u8 = 0u8;
        let mut x: u32 = 0u32;
        let mut lvlText = crate::ffi::Align4([0u8; 12]);
        let mut intText = crate::ffi::Align4([0u8; 8]);
        StringCopy(
            (&raw mut lvlText).cast::<u8>(),
            (&raw mut gText_Lv).cast::<u8>(),
        );
        level = GetLevelAfterDaycareSteps(
            (((daycare).cast::<u8>()).wrapping_offset(((daycareSlotId) as i32) as isize * 140)),
            ((((daycare).cast::<u8>()).wrapping_offset(((daycareSlotId) as i32) as isize * 140))
                .wrapping_add(136)
                .cast::<u32>())
            .read(),
        );
        ConvertIntToDecimalStringN((&raw mut intText).cast::<u8>(), ((level) as i32), 0i32, 3u8);
        StringAppend(
            (&raw mut lvlText).cast::<u8>(),
            (&raw mut intText).cast::<u8>(),
        );
        x = ((GetStringRightAlignXOffset(1i32, (&raw mut lvlText).cast::<u8>(), 112i32)) as u32);
        DaycareAddTextPrinter(windowId, (&raw mut lvlText).cast::<u8>(), x, y);
    }
}
pub(crate) unsafe extern "C" fn DaycarePrintMonInfo(windowId: u8, daycareSlotId: u32, y: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut daycareSlotId = daycareSlotId;
        let mut y = y;
        if daycareSlotId < 2u32 {
            DaycarePrintMonNickname(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
                windowId,
                daycareSlotId,
                ((y) as u32),
            );
            DaycarePrintMonLvl(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12336),
                windowId,
                daycareSlotId,
                ((y) as u32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleDaycareLevelMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut input: u32 = ((ListMenu_ProcessInput(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8),
        )) as u32);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            'l1: {
                let __sw1 = input;
                if __sw1 == 0u32 || __sw1 == 1u32 {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((input) as u16));
                    break 'l1;
                }
                if __sw1 == 5u32 {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
                    break 'l1;
                }
            }
            DestroyListMenuTask(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
            ClearStdWindowAndFrame(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
                1u8,
            );
            RemoveWindow(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
            );
            DestroyTask(taskId);
            ScriptContext_Enable();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
                DestroyListMenuTask(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u8),
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                );
                ClearStdWindowAndFrame(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u8),
                    1u8,
                );
                RemoveWindow(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u8),
                );
                DestroyTask(taskId);
                ScriptContext_Enable();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowDaycareLevelMenu() {
    unsafe {
        let mut menuTemplate = crate::ffi::Align4([0u8; 24]);
        let mut windowId: u8 = 0u8;
        let mut listMenuTaskId: u8 = 0u8;
        let mut daycareMenuTaskId: u8 = 0u8;
        windowId = ((AddWindow(
            (&raw const sDaycareLevelMenuWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
        )) as u8);
        DrawStdWindowFrame(windowId, 0u8);
        (&raw mut menuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sDaycareListMenuLevelTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut menuTemplate).cast::<u8>()).wrapping_add(16)).write(windowId);
        listMenuTaskId = ListMenuInit((&raw mut menuTemplate).cast::<u8>(), 0u16, 0u16);
        CopyWindowToVram(windowId, 3u8);
        daycareMenuTaskId = CreateTask(Some(Task_HandleDaycareLevelMenuInput), 3u8);
        (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((daycareMenuTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .write(((listMenuTaskId) as i16));
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((daycareMenuTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((windowId) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseSendDaycareMon() {
    unsafe {
        ChooseMonForDaycare();
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_ReturnToField));
    }
}
