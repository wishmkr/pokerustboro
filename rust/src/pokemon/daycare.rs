//! Translated from `src/daycare.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gEggMoves sDaycareLevelMenuWindowTemplate sLevelMenuItems sDaycareListMenuLevelTemplate sCompatibilityMessages sJapaneseEggNickname

const EGG_MOVES_SPECIES_OFFSET: i32 = 20000;

static gEggMoves: Table<CArray<u16, 1139>> =
    Table((&raw const crate::data::daycare::gEggMoves).cast());
static sCompatibilityMessages: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::daycare::sCompatibilityMessages).cast());
static sDaycareLevelMenuWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::daycare::sDaycareLevelMenuWindowTemplate).cast());
static sDaycareListMenuLevelTemplate: Table<ListMenuTemplate> =
    Table((&raw const crate::data::daycare::sDaycareListMenuLevelTemplate).cast());
static sJapaneseEggNickname: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::daycare::sJapaneseEggNickname).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggLevelUpMoves: Aligned<CArray<u16, 50>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggFatherMoves: Aligned<CArray<u16, 4>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggFinalMoves: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggEggMoves: Aligned<CArray<u16, 10>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHatchedEggMotherMoves: Aligned<CArray<u16, 4>> =
    Aligned(unsafe { zeroed() });

unsafe extern "C" {
    static gEvolutionTable: CArray<CArray<Evolution, 5>, 0>;
    static mut gMain: Main;
    static mut gMoveToLearn: u16;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    static gText_Exit4: CArray<u8, 0>;
    static gText_FemaleSymbol4: CArray<u8, 0>;
    static gText_GenderlessSymbol: CArray<u8, 0>;
    static gText_Lv: CArray<u8, 0>;
    static gText_MaleSymbol4: CArray<u8, 0>;
    static gText_NewLine2: CArray<u8, 0>;
    fn AddTextPrinter(
        a0: *mut TextPrinterTemplate,
        a1: u8,
        a2: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn BoxMonRestorePP(a0: *mut BoxPokemon);
    fn BoxMonToMon(a0: *mut BoxPokemon, a1: *mut Pokemon);
    fn CB2_ReturnToField();
    fn CalculateMonStats(a0: *mut Pokemon);
    fn CalculatePlayerPartyCount() -> u8;
    fn CanMonLearnTMHM(a0: *mut Pokemon, a1: u8) -> u32;
    fn ChooseMonForDaycare();
    fn ClearMail(a0: *mut Mail);
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn CompactPartySlots() -> i16;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMon(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeleteFirstMoveAndGiveMoveToMon(a0: *mut Pokemon, a1: u16);
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroyTask(a0: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn FlagSet(a0: u16) -> u8;
    fn GetBoxMonData2(a0: *mut BoxPokemon, a1: i32) -> u32;
    fn GetBoxMonData3(a0: *mut BoxPokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxMonGender(a0: *mut BoxPokemon) -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn GetEggCyclesToSubtract() -> u8;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetLevelFromBoxMonExp(a0: *mut BoxPokemon) -> u8;
    fn GetLevelUpMovesBySpecies(a0: u16, a1: *mut u16) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GiveMailToMon(a0: *mut Pokemon, a1: *mut Mail) -> u8;
    fn GiveMoveToMon(a0: *mut Pokemon, a1: u16) -> u16;
    fn ItemIdToBattleMoveId(a0: u16) -> u16;
    fn ListMenuInit(a0: *mut ListMenuTemplate, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn MonHasMail(a0: *mut Pokemon) -> u8;
    fn MonTryLearningNewMove(a0: *mut Pokemon, a1: u8) -> u16;
    fn Random() -> u16;
    fn Random2() -> u16;
    fn RemoveWindow(a0: u8);
    fn ScriptContext_Enable();
    fn SeedRng2(a0: u16);
    fn SetBoxMonData(a0: *mut BoxPokemon, a1: i32, a2: *mut c_void);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn TakeMailFromMon(a0: *mut Pokemon);
    fn TryIncrementMonLevel(a0: *mut Pokemon) -> u8;
    fn ZeroBoxMonData(a0: *mut BoxPokemon);
    fn ZeroMonData(a0: *mut Pokemon);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonNickname2(mon: *mut Pokemon, dest: *mut u8) -> *mut u8 {
    let mut nickname: CArray<u8, 20> = zeroed();
    GetMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
    return StringCopy_Nickname(dest, nickname.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonNickname(mon: *mut BoxPokemon, dest: *mut u8) -> *mut u8 {
    let mut nickname: CArray<u8, 20> = zeroed();
    GetBoxMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
    return StringCopy_Nickname(dest, nickname.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPokemonInDaycare(daycare: *mut DayCare) -> u8 {
    let mut i: u8 = 0;
    let mut count: u8 = 0;
    count = 0;
    i = 0;
    while i < DAYCARE_MON_COUNT {
        if GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES) != 0 {
            count += 1;
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitDaycareMailRecordMixing(
    daycare: *mut DayCare,
    mixMail: *mut RecordMixingDaycareMail,
) {
    let mut i: u8 = 0;
    let mut numDaycareMons: u8 = 0;
    i = 0;
    while i < DAYCARE_MON_COUNT {
        if GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES) != SPECIES_NONE as u32
        {
            numDaycareMons += 1;
            if GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_HELD_ITEM)
                == ITEM_NONE as u32
            {
                (*mixMail).cantHoldItem[i] = FALSE as u16;
            } else {
                (*mixMail).cantHoldItem[i] = TRUE as u16;
            }
        } else {
            (*mixMail).cantHoldItem[i] = TRUE as u16;
        }
        i += 1;
    }
    (*mixMail).numDaycareMons = numDaycareMons as u32;
}
pub(crate) unsafe extern "C" fn Daycare_FindEmptySpot(daycare: *mut DayCare) -> i8 {
    let mut i: u8 = 0;
    i = 0;
    while i < DAYCARE_MON_COUNT {
        if GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES) == SPECIES_NONE as u32
        {
            return i as i8;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn StorePokemonInDaycare(
    mon: *mut Pokemon,
    daycareMon: *mut DaycareMon,
) {
    if MonHasMail(mon) != 0 {
        let mut mailId: u8 = 0;
        StringCopy(
            (*daycareMon).mail.otName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        GetMonNickname2(mon, (*daycareMon).mail.monName.as_mut_ptr());
        StripExtCtrlCodes((*daycareMon).mail.monName.as_mut_ptr());
        (*daycareMon).mail.set_gameLanguage(GAME_LANGUAGE);
        (*daycareMon)
            .mail
            .set_monLanguage(GetMonData2(mon, MON_DATA_LANGUAGE) as u8);
        mailId = GetMonData2(mon, MON_DATA_MAIL) as u8;
        (*daycareMon).mail.message = (*gSaveBlock1Ptr).mail[mailId];
        TakeMailFromMon(mon);
    }
    (*daycareMon).mon = (*mon).r#box;
    BoxMonRestorePP(&raw mut (*daycareMon).mon);
    (*daycareMon).steps = 0;
    ZeroMonData(mon);
    CompactPartySlots();
    CalculatePlayerPartyCount();
}
pub(crate) unsafe extern "C" fn StorePokemonInEmptyDaycareSlot(
    mon: *mut Pokemon,
    daycare: *mut DayCare,
) {
    let mut slotId: i8 = Daycare_FindEmptySpot(daycare);
    StorePokemonInDaycare(mon, &raw mut (*daycare).mons[slotId]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StoreSelectedPokemonInDaycare() {
    let mut monId: u8 = GetCursorSelectionMonId();
    StorePokemonInEmptyDaycareSlot(
        &raw mut gPlayerParty[monId],
        &raw mut (*gSaveBlock1Ptr).daycare,
    );
}
pub(crate) unsafe extern "C" fn ShiftDaycareSlots(daycare: *mut DayCare) {
    if GetBoxMonData2(&raw mut (*daycare).mons[1].mon, MON_DATA_SPECIES) != SPECIES_NONE as u32
        && GetBoxMonData2(&raw mut (*daycare).mons[0].mon, MON_DATA_SPECIES) == 0
    {
        (*daycare).mons[0].mon = (*daycare).mons[1].mon;
        ZeroBoxMonData(&raw mut (*daycare).mons[1].mon);
        (*daycare).mons[0].mail = (*daycare).mons[1].mail;
        (*daycare).mons[0].steps = (*daycare).mons[1].steps;
        (*daycare).mons[1].steps = 0;
        ClearDaycareMonMail(&raw mut (*daycare).mons[1].mail);
    }
}
pub(crate) unsafe extern "C" fn ApplyDaycareExperience(mon: *mut Pokemon) {
    let mut i: i32 = 0;
    let mut firstMove: u8 = 0;
    let mut learnedMove: u16 = 0;
    i = 0;
    while i < MAX_LEVEL as i32 {
        if TryIncrementMonLevel(mon) != 0 {
            firstMove = TRUE;
            while ({
                learnedMove = MonTryLearningNewMove(mon, firstMove);
                learnedMove
            }) != 0
            {
                firstMove = FALSE;
                if learnedMove == MON_HAS_MAX_MOVES {
                    DeleteFirstMoveAndGiveMoveToMon(mon, gMoveToLearn);
                }
            }
        } else {
            break;
        }
        i += 1;
    }
    CalculateMonStats(mon);
}
pub(crate) unsafe extern "C" fn TakeSelectedPokemonFromDaycare(daycareMon: *mut DaycareMon) -> u16 {
    let mut species: u16 = 0;
    let mut experience: u32 = 0;
    let mut pokemon: Pokemon = zeroed();
    GetBoxMonNickname(&raw mut (*daycareMon).mon, gStringVar1.as_mut_ptr());
    species = GetBoxMonData2(&raw mut (*daycareMon).mon, MON_DATA_SPECIES) as u16;
    BoxMonToMon(&raw mut (*daycareMon).mon, &raw mut pokemon);
    if GetMonData2(&raw mut pokemon, MON_DATA_LEVEL) != MAX_LEVEL {
        experience = GetMonData2(&raw mut pokemon, MON_DATA_EXP) + (*daycareMon).steps;
        SetMonData(
            &raw mut pokemon,
            MON_DATA_EXP,
            &raw mut experience as *mut c_void,
        );
        ApplyDaycareExperience(&raw mut pokemon);
    }
    gPlayerParty[5] = pokemon;
    if (*daycareMon).mail.message.itemId != 0 {
        GiveMailToMon(
            &raw mut gPlayerParty[5],
            &raw mut (*daycareMon).mail.message,
        );
        ClearDaycareMonMail(&raw mut (*daycareMon).mail);
    }
    ZeroBoxMonData(&raw mut (*daycareMon).mon);
    (*daycareMon).steps = 0;
    CompactPartySlots();
    CalculatePlayerPartyCount();
    return species;
}
pub(crate) unsafe extern "C" fn TakeSelectedPokemonMonFromDaycareShiftSlots(
    daycare: *mut DayCare,
    slotId: u8,
) -> u16 {
    let mut species: u16 = TakeSelectedPokemonFromDaycare(&raw mut (*daycare).mons[slotId]);
    ShiftDaycareSlots(daycare);
    return species;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TakePokemonFromDaycare() -> u16 {
    return TakeSelectedPokemonMonFromDaycareShiftSlots(
        &raw mut (*gSaveBlock1Ptr).daycare,
        gSpecialVar_0x8004 as u8,
    );
}
pub(crate) unsafe extern "C" fn GetLevelAfterDaycareSteps(mon: *mut BoxPokemon, steps: u32) -> u8 {
    let mut tempMon: BoxPokemon = zeroed();
    tempMon = *mon;
    let mut experience: u32 = GetBoxMonData2(mon, MON_DATA_EXP) + steps;
    SetBoxMonData(
        &raw mut tempMon,
        MON_DATA_EXP,
        &raw mut experience as *mut c_void,
    );
    return GetLevelFromBoxMonExp(&raw mut tempMon);
}
pub(crate) unsafe extern "C" fn GetNumLevelsGainedFromSteps(daycareMon: *mut DaycareMon) -> u8 {
    let mut levelBefore: u8 = 0;
    let mut levelAfter: u8 = 0;
    levelBefore = GetLevelFromBoxMonExp(&raw mut (*daycareMon).mon);
    levelAfter = GetLevelAfterDaycareSteps(&raw mut (*daycareMon).mon, (*daycareMon).steps);
    return levelAfter - levelBefore;
}
pub(crate) unsafe extern "C" fn GetNumLevelsGainedForDaycareMon(daycareMon: *mut DaycareMon) -> u8 {
    let mut numLevelsGained: u8 = GetNumLevelsGainedFromSteps(daycareMon);
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        numLevelsGained as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        2,
    );
    GetBoxMonNickname(&raw mut (*daycareMon).mon, gStringVar1.as_mut_ptr());
    return numLevelsGained;
}
pub(crate) unsafe extern "C" fn PrepareDaycareCostStringForSelectedMon(
    daycareMon: *mut DaycareMon,
) -> u32 {
    let mut cost: u32 = 0;
    let mut numLevelsGained: u8 = GetNumLevelsGainedFromSteps(daycareMon);
    GetBoxMonNickname(&raw mut (*daycareMon).mon, gStringVar1.as_mut_ptr());
    cost = 100 + 100 * numLevelsGained as u32;
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        cost as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        5,
    );
    return cost;
}
pub(crate) unsafe extern "C" fn PrepareDaycareCostStringForMon(
    daycare: *mut DayCare,
    slotId: u8,
) -> u16 {
    return PrepareDaycareCostStringForSelectedMon(&raw mut (*daycare).mons[slotId]) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDaycareCostAndPrepareString() {
    gSpecialVar_0x8005 = PrepareDaycareCostStringForMon(
        &raw mut (*gSaveBlock1Ptr).daycare,
        gSpecialVar_0x8004 as u8,
    );
}
pub(crate) unsafe extern "C" fn Debug_AddDaycareSteps(numSteps: u16) {
    (*gSaveBlock1Ptr).daycare.mons[0].steps += numSteps as u32;
    (*gSaveBlock1Ptr).daycare.mons[1].steps += numSteps as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumLevelsGainedFromDaycare() -> u8 {
    if GetBoxMonData2(
        &raw mut (*gSaveBlock1Ptr).daycare.mons[gSpecialVar_0x8004].mon,
        MON_DATA_SPECIES,
    ) != 0
    {
        return GetNumLevelsGainedForDaycareMon(
            &raw mut (*gSaveBlock1Ptr).daycare.mons[gSpecialVar_0x8004],
        );
    }
    return 0;
}
pub(crate) unsafe extern "C" fn ClearDaycareMonMail(mail: *mut DaycareMail) {
    let mut i: i32 = 0;
    i = 0;
    while i < 8 {
        (*mail).otName[i] = 0;
        i += 1;
    }
    i = 0;
    while i < 11 {
        (*mail).monName[i] = 0;
        i += 1;
    }
    ClearMail(&raw mut (*mail).message);
}
pub(crate) unsafe extern "C" fn ClearDaycareMon(daycareMon: *mut DaycareMon) {
    ZeroBoxMonData(&raw mut (*daycareMon).mon);
    (*daycareMon).steps = 0;
    ClearDaycareMonMail(&raw mut (*daycareMon).mail);
}
pub(crate) unsafe extern "C" fn ClearAllDaycareData(daycare: *mut DayCare) {
    let mut i: u8 = 0;
    i = 0;
    while i < DAYCARE_MON_COUNT {
        ClearDaycareMon(&raw mut (*daycare).mons[i]);
        i += 1;
    }
    (*daycare).offspringPersonality = 0;
    (*daycare).stepCounter = 0;
}
pub(crate) unsafe extern "C" fn GetEggSpecies(mut species: u16) -> u16 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut found: u8 = 0;
    i = 0;
    while i < EVOS_PER_MON {
        found = FALSE;
        j = 1;
        while j < NUM_SPECIES as i32 {
            k = 0;
            while k < EVOS_PER_MON {
                if gEvolutionTable[j][k].targetSpecies == species {
                    species = j as u16;
                    found = TRUE;
                    break;
                }
                k += 1;
            }
            if found != 0 {
                break;
            }
            j += 1;
        }
        if j == NUM_SPECIES as i32 {
            break;
        }
        i += 1;
    }
    return species;
}
pub(crate) unsafe extern "C" fn GetParentToInheritNature(daycare: *mut DayCare) -> i32 {
    let mut species: CArray<u32, 2> = zeroed();
    let mut i: i32 = 0;
    let mut dittoCount: i32 = 0;
    let mut parent: i32 = -1;
    i = 0;
    while i < DAYCARE_MON_COUNT as i32 {
        if GetBoxMonGender(&raw mut (*daycare).mons[i].mon) == MON_FEMALE {
            parent = i;
        }
        i += 1;
    }
    dittoCount = 0;
    i = 0;
    while i < DAYCARE_MON_COUNT as i32 {
        species[i] = GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES);
        if species[i] == SPECIES_DITTO as u32 {
            dittoCount += 1;
            parent = i;
        }
        i += 1;
    }
    if dittoCount == DAYCARE_MON_COUNT as i32 {
        if Random() >= 32767 {
            parent = 0;
        } else {
            parent = 1;
        }
    }
    if GetBoxMonData2(&raw mut (*daycare).mons[parent].mon, MON_DATA_HELD_ITEM) != ITEM_EVERSTONE
        || Random() >= 32767
    {
        return -1;
    }
    return parent;
}
pub(crate) unsafe extern "C" fn _TriggerPendingDaycareEgg(daycare: *mut DayCare) {
    let mut parent: i32 = 0;
    let mut natureTries: i32 = 0;
    SeedRng2(gMain.vblankCounter2 as u16);
    parent = GetParentToInheritNature(daycare);
    if parent < 0 {
        (*daycare).offspringPersonality =
            (Random2() as u32) << 16 | (Random() as i32 % 65534) as u32 + 1;
    } else {
        let mut wantedNature: u8 = GetNatureFromPersonality(GetBoxMonData3(
            &raw mut (*daycare).mons[parent].mon,
            MON_DATA_PERSONALITY,
            null_mut(),
        ));
        let mut personality: u32 = 0;
        loop {
            personality = (Random2() as u32) << 16 | Random() as u32;
            if wantedNature == GetNatureFromPersonality(personality) && personality != 0 {
                break;
            }
            natureTries += 1;
            if natureTries > 2400 {
                break;
            }
        }
        (*daycare).offspringPersonality = personality;
    }
    FlagSet(FLAG_PENDING_DAYCARE_EGG);
}
pub(crate) unsafe extern "C" fn _TriggerPendingDaycareMaleEgg(daycare: *mut DayCare) {
    (*daycare).offspringPersonality = Random() as u32 | EGG_GENDER_MALE;
    FlagSet(FLAG_PENDING_DAYCARE_EGG);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TriggerPendingDaycareEgg() {
    _TriggerPendingDaycareEgg(&raw mut (*gSaveBlock1Ptr).daycare);
}
pub(crate) unsafe extern "C" fn TriggerPendingDaycareMaleEgg() {
    _TriggerPendingDaycareMaleEgg(&raw mut (*gSaveBlock1Ptr).daycare);
}
pub(crate) unsafe extern "C" fn RemoveIVIndexFromList(mut ivs: *mut u8, selectedIv: u8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut temp: CArray<u8, 6> = zeroed();
    *ivs.at(selectedIv) = 0xFF;
    i = 0;
    while i < NUM_STATS {
        temp[i] = *ivs.at(i);
        i += 1;
    }
    j = 0;
    i = 0;
    while i < NUM_STATS {
        if temp[i] != 0xFF {
            *ivs.at({
                let t1 = j;
                j += 1;
                t1
            }) = temp[i];
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InheritIVs(egg: *mut Pokemon, daycare: *mut DayCare) {
    let mut i: u8 = 0;
    let mut selectedIvs: CArray<u8, 3> = zeroed();
    let mut availableIVs: CArray<u8, 6> = zeroed();
    let mut whichParents: CArray<u8, 3> = zeroed();
    let mut iv: u8 = 0;
    i = 0;
    while i < NUM_STATS as u8 {
        availableIVs[i] = i;
        i += 1;
    }
    i = 0;
    while i < INHERITED_IV_COUNT {
        selectedIvs[i] = availableIVs[rem_i32(Random() as i32, NUM_STATS - i as i32)];
        RemoveIVIndexFromList(availableIVs.as_mut_ptr(), i);
        i += 1;
    }
    i = 0;
    while i < INHERITED_IV_COUNT {
        whichParents[i] = (Random() as i32 % 2) as u8;
        i += 1;
    }
    i = 0;
    while i < INHERITED_IV_COUNT {
        match selectedIvs[i] {
            0 => {
                iv = GetBoxMonData2(
                    &raw mut (*daycare).mons[whichParents[i]].mon,
                    MON_DATA_HP_IV,
                ) as u8;
                SetMonData(egg, MON_DATA_HP_IV, &raw mut iv as *mut c_void);
            }
            1 => {
                iv = GetBoxMonData2(
                    &raw mut (*daycare).mons[whichParents[i]].mon,
                    MON_DATA_ATK_IV,
                ) as u8;
                SetMonData(egg, MON_DATA_ATK_IV, &raw mut iv as *mut c_void);
            }
            2 => {
                iv = GetBoxMonData2(
                    &raw mut (*daycare).mons[whichParents[i]].mon,
                    MON_DATA_DEF_IV,
                ) as u8;
                SetMonData(egg, MON_DATA_DEF_IV, &raw mut iv as *mut c_void);
            }
            3 => {
                iv = GetBoxMonData2(
                    &raw mut (*daycare).mons[whichParents[i]].mon,
                    MON_DATA_SPEED_IV,
                ) as u8;
                SetMonData(egg, MON_DATA_SPEED_IV, &raw mut iv as *mut c_void);
            }
            4 => {
                iv = GetBoxMonData2(
                    &raw mut (*daycare).mons[whichParents[i]].mon,
                    MON_DATA_SPATK_IV,
                ) as u8;
                SetMonData(egg, MON_DATA_SPATK_IV, &raw mut iv as *mut c_void);
            }
            5 => {
                iv = GetBoxMonData2(
                    &raw mut (*daycare).mons[whichParents[i]].mon,
                    MON_DATA_SPDEF_IV,
                ) as u8;
                SetMonData(egg, MON_DATA_SPDEF_IV, &raw mut iv as *mut c_void);
            }
            _ => {}
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetEggMoves(pokemon: *mut Pokemon, mut eggMoves: *mut u16) -> u8 {
    let mut eggMoveIdx: u16 = 0;
    let mut numEggMoves: u16 = 0;
    let mut species: u16 = 0;
    let mut i: u16 = 0;
    numEggMoves = 0;
    eggMoveIdx = 0;
    species = GetMonData2(pokemon, MON_DATA_SPECIES) as u16;
    i = 0;
    while i < 1138 {
        if gEggMoves[i] as i32 == species as i32 + EGG_MOVES_SPECIES_OFFSET {
            eggMoveIdx = i + 1;
            break;
        }
        i += 1;
    }
    i = 0;
    while i < EGG_MOVES_ARRAY_COUNT {
        if gEggMoves[eggMoveIdx as i32 + i as i32] > EGG_MOVES_SPECIES_OFFSET as u16 {
            break;
        }
        *eggMoves.at(i) = gEggMoves[eggMoveIdx as i32 + i as i32];
        numEggMoves += 1;
        i += 1;
    }
    return numEggMoves as u8;
}
pub(crate) unsafe extern "C" fn BuildEggMoveset(
    egg: *mut Pokemon,
    father: *mut BoxPokemon,
    mother: *mut BoxPokemon,
) {
    let mut numSharedParentMoves: u16 = 0;
    let mut numLevelUpMoves: u32 = 0;
    let mut numEggMoves: u16 = 0;
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    numSharedParentMoves = 0;
    i = 0;
    while i < MAX_MON_MOVES as u16 {
        sHatchedEggMotherMoves[i] = MOVE_NONE;
        sHatchedEggFatherMoves[i] = MOVE_NONE;
        sHatchedEggFinalMoves[i] = MOVE_NONE;
        i += 1;
    }
    i = 0;
    while i < EGG_MOVES_ARRAY_COUNT {
        sHatchedEggEggMoves[i] = MOVE_NONE;
        i += 1;
    }
    i = 0;
    while (i as i32) < (if 20 > 50 { 20 } else { 50 }) {
        sHatchedEggLevelUpMoves[i] = MOVE_NONE;
        i += 1;
    }
    numLevelUpMoves = GetLevelUpMovesBySpecies(
        GetMonData2(egg, MON_DATA_SPECIES) as u16,
        sHatchedEggLevelUpMoves.as_mut_ptr(),
    ) as u32;
    i = 0;
    while i < MAX_MON_MOVES as u16 {
        sHatchedEggFatherMoves[i] = GetBoxMonData2(father, MON_DATA_MOVE1 + i as i32) as u16;
        sHatchedEggMotherMoves[i] = GetBoxMonData2(mother, MON_DATA_MOVE1 + i as i32) as u16;
        i += 1;
    }
    numEggMoves = GetEggMoves(egg, sHatchedEggEggMoves.as_mut_ptr()) as u16;
    i = 0;
    while i < MAX_MON_MOVES as u16 {
        if sHatchedEggFatherMoves[i] != MOVE_NONE {
            j = 0;
            while j < numEggMoves {
                if sHatchedEggFatherMoves[i] == sHatchedEggEggMoves[j] {
                    if GiveMoveToMon(egg, sHatchedEggFatherMoves[i]) == MON_HAS_MAX_MOVES {
                        DeleteFirstMoveAndGiveMoveToMon(egg, sHatchedEggFatherMoves[i]);
                    }
                    break;
                }
                j += 1;
            }
        } else {
            break;
        }
        i += 1;
    }
    i = 0;
    while i < MAX_MON_MOVES as u16 {
        if sHatchedEggFatherMoves[i] != MOVE_NONE {
            j = 0;
            while j < 58 {
                if sHatchedEggFatherMoves[i] == ItemIdToBattleMoveId(ITEM_TM01 + j)
                    && CanMonLearnTMHM(egg, j as u8) != 0
                {
                    if GiveMoveToMon(egg, sHatchedEggFatherMoves[i]) == MON_HAS_MAX_MOVES {
                        DeleteFirstMoveAndGiveMoveToMon(egg, sHatchedEggFatherMoves[i]);
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    i = 0;
    while i < MAX_MON_MOVES as u16 {
        if sHatchedEggFatherMoves[i] == MOVE_NONE {
            break;
        }
        j = 0;
        while j < MAX_MON_MOVES as u16 {
            if sHatchedEggFatherMoves[i] == sHatchedEggMotherMoves[j]
                && sHatchedEggFatherMoves[i] != MOVE_NONE
            {
                sHatchedEggFinalMoves[{
                    let t1 = numSharedParentMoves;
                    numSharedParentMoves += 1;
                    t1
                }] = sHatchedEggFatherMoves[i];
            }
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < MAX_MON_MOVES as u16 {
        if sHatchedEggFinalMoves[i] == MOVE_NONE {
            break;
        }
        j = 0;
        while (j as u32) < numLevelUpMoves {
            if sHatchedEggLevelUpMoves[j] != MOVE_NONE
                && sHatchedEggFinalMoves[i] == sHatchedEggLevelUpMoves[j]
            {
                if GiveMoveToMon(egg, sHatchedEggFinalMoves[i]) == MON_HAS_MAX_MOVES {
                    DeleteFirstMoveAndGiveMoveToMon(egg, sHatchedEggFinalMoves[i]);
                }
                break;
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn RemoveEggFromDayCare(daycare: *mut DayCare) {
    (*daycare).offspringPersonality = 0;
    (*daycare).stepCounter = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RejectEggFromDayCare() {
    RemoveEggFromDayCare(&raw mut (*gSaveBlock1Ptr).daycare);
}
pub(crate) unsafe extern "C" fn AlterEggSpeciesWithIncenseItem(
    species: *mut u16,
    daycare: *mut DayCare,
) {
    let mut motherItem: u16 = 0;
    let mut fatherItem: u16 = 0;
    if *species == SPECIES_WYNAUT || *species == SPECIES_AZURILL {
        motherItem = GetBoxMonData2(&raw mut (*daycare).mons[0].mon, MON_DATA_HELD_ITEM) as u16;
        fatherItem = GetBoxMonData2(&raw mut (*daycare).mons[1].mon, MON_DATA_HELD_ITEM) as u16;
        if *species == SPECIES_WYNAUT
            && motherItem != ITEM_LAX_INCENSE
            && fatherItem != ITEM_LAX_INCENSE
        {
            *species = SPECIES_WOBBUFFET;
        }
        if *species == SPECIES_AZURILL
            && motherItem != ITEM_SEA_INCENSE
            && fatherItem != ITEM_SEA_INCENSE
        {
            *species = SPECIES_MARILL;
        }
    }
}
pub(crate) unsafe extern "C" fn GiveVoltTackleIfLightBall(
    mon: *mut Pokemon,
    daycare: *mut DayCare,
) {
    let mut motherItem: u32 = GetBoxMonData2(&raw mut (*daycare).mons[0].mon, MON_DATA_HELD_ITEM);
    let mut fatherItem: u32 = GetBoxMonData2(&raw mut (*daycare).mons[1].mon, MON_DATA_HELD_ITEM);
    if motherItem == ITEM_LIGHT_BALL || fatherItem == ITEM_LIGHT_BALL {
        if GiveMoveToMon(mon, MOVE_VOLT_TACKLE) == MON_HAS_MAX_MOVES {
            DeleteFirstMoveAndGiveMoveToMon(mon, MOVE_VOLT_TACKLE);
        }
    }
}
pub(crate) unsafe extern "C" fn DetermineEggSpeciesAndParentSlots(
    daycare: *mut DayCare,
    mut parentSlots: *mut u8,
) -> u16 {
    let mut i: u16 = 0;
    let mut species: CArray<u16, 2> = zeroed();
    let mut eggSpecies: u16 = 0;
    i = 0;
    while i < DAYCARE_MON_COUNT as u16 {
        species[i] = GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES) as u16;
        if species[i] == SPECIES_DITTO {
            *parentSlots = i as u8 ^ 1;
            *parentSlots.at(1) = i as u8;
        } else if GetBoxMonGender(&raw mut (*daycare).mons[i].mon) == MON_FEMALE {
            *parentSlots = i as u8;
            *parentSlots.at(1) = i as u8 ^ 1;
        }
        i += 1;
    }
    eggSpecies = GetEggSpecies(species[*parentSlots]);
    if eggSpecies == SPECIES_NIDORAN_F && (*daycare).offspringPersonality & EGG_GENDER_MALE != 0 {
        eggSpecies = SPECIES_NIDORAN_M;
    }
    if eggSpecies == SPECIES_ILLUMISE && (*daycare).offspringPersonality & EGG_GENDER_MALE != 0 {
        eggSpecies = SPECIES_VOLBEAT;
    }
    if species[*parentSlots.at(1)] == SPECIES_DITTO
        && GetBoxMonGender(&raw mut (*daycare).mons[*parentSlots].mon) != MON_FEMALE
    {
        let mut ditto: u8 = *parentSlots.at(1);
        *parentSlots.at(1) = *parentSlots;
        *parentSlots = ditto;
    }
    return eggSpecies;
}
pub(crate) unsafe extern "C" fn _GiveEggFromDaycare(daycare: *mut DayCare) {
    let mut egg: Pokemon = zeroed();
    let mut species: u16 = 0;
    let mut parentSlots: CArray<u8, 2> = zeroed();
    let mut isEgg: u8 = 0;
    species = DetermineEggSpeciesAndParentSlots(daycare, parentSlots.as_mut_ptr());
    AlterEggSpeciesWithIncenseItem(&raw mut species, daycare);
    SetInitialEggData(&raw mut egg, species, daycare);
    InheritIVs(&raw mut egg, daycare);
    BuildEggMoveset(
        &raw mut egg,
        &raw mut (*daycare).mons[parentSlots[1]].mon,
        &raw mut (*daycare).mons[parentSlots[0]].mon,
    );
    if species == SPECIES_PICHU {
        GiveVoltTackleIfLightBall(&raw mut egg, daycare);
    }
    isEgg = TRUE;
    SetMonData(&raw mut egg, MON_DATA_IS_EGG, &raw mut isEgg as *mut c_void);
    gPlayerParty[5] = egg;
    CompactPartySlots();
    CalculatePlayerPartyCount();
    RemoveEggFromDayCare(daycare);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateEgg(mon: *mut Pokemon, species: u16, setHotSpringsLocation: u8) {
    let mut metLevel: u8 = 0;
    let mut ball: u16 = 0;
    let mut language: u8 = 0;
    let mut metLocation: u8 = 0;
    let mut isEgg: u8 = 0;
    CreateMon(mon, species, EGG_HATCH_LEVEL, USE_RANDOM_IVS, 0, 0, 0, 0);
    metLevel = 0;
    ball = ITEM_POKE_BALL;
    language = LANGUAGE_JAPANESE;
    SetMonData(mon, MON_DATA_POKEBALL, &raw mut ball as *mut c_void);
    SetMonData(
        mon,
        MON_DATA_NICKNAME,
        sJapaneseEggNickname.as_ptr().cast_mut() as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_FRIENDSHIP,
        (&raw const gSpeciesInfo[species].eggCycles).cast_mut() as *mut c_void,
    );
    SetMonData(mon, MON_DATA_MET_LEVEL, &raw mut metLevel as *mut c_void);
    SetMonData(mon, MON_DATA_LANGUAGE, &raw mut language as *mut c_void);
    if setHotSpringsLocation != 0 {
        metLocation = METLOC_SPECIAL_EGG;
        SetMonData(
            mon,
            MON_DATA_MET_LOCATION,
            &raw mut metLocation as *mut c_void,
        );
    }
    isEgg = TRUE;
    SetMonData(mon, MON_DATA_IS_EGG, &raw mut isEgg as *mut c_void);
}
pub(crate) unsafe extern "C" fn SetInitialEggData(
    mon: *mut Pokemon,
    species: u16,
    daycare: *mut DayCare,
) {
    let mut personality: u32 = 0;
    let mut ball: u16 = 0;
    let mut metLevel: u8 = 0;
    let mut language: u8 = 0;
    personality = (*daycare).offspringPersonality;
    CreateMon(
        mon,
        species,
        EGG_HATCH_LEVEL,
        USE_RANDOM_IVS,
        TRUE,
        personality,
        0,
        0,
    );
    metLevel = 0;
    ball = ITEM_POKE_BALL;
    language = LANGUAGE_JAPANESE;
    SetMonData(mon, MON_DATA_POKEBALL, &raw mut ball as *mut c_void);
    SetMonData(
        mon,
        MON_DATA_NICKNAME,
        sJapaneseEggNickname.as_ptr().cast_mut() as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_FRIENDSHIP,
        (&raw const gSpeciesInfo[species].eggCycles).cast_mut() as *mut c_void,
    );
    SetMonData(mon, MON_DATA_MET_LEVEL, &raw mut metLevel as *mut c_void);
    SetMonData(mon, MON_DATA_LANGUAGE, &raw mut language as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveEggFromDaycare() {
    _GiveEggFromDaycare(&raw mut (*gSaveBlock1Ptr).daycare);
}
pub(crate) unsafe extern "C" fn TryProduceOrHatchEgg(daycare: *mut DayCare) -> u8 {
    let mut i: u32 = 0;
    let mut validEggs: u32 = 0;
    i = 0;
    while i < DAYCARE_MON_COUNT as u32 {
        if GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SANITY_HAS_SPECIES) != 0 {
            (*daycare).mons[i].steps += 1;
            validEggs += 1;
        }
        i += 1;
    }
    if (*daycare).offspringPersonality == 0
        && validEggs == DAYCARE_MON_COUNT as u32
        && (*daycare).mons[1].steps & 0xFF == 0xFF
    {
        let mut compatibility: u8 = GetDaycareCompatibilityScore(daycare);
        if compatibility as u32 > Random() as u32 * 100 / 65535 {
            TriggerPendingDaycareEgg();
        }
    }
    if ({
        (*daycare).stepCounter += 1;
        (*daycare).stepCounter
    }) == 255
    {
        let mut eggCycles: u32 = 0;
        let mut toSub: u8 = GetEggCyclesToSubtract();
        i = 0;
        while i < gPlayerPartyCount as u32 {
            'l2: {
                if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0 {
                    break 'l2;
                }
                if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_IS_BAD_EGG) != 0 {
                    break 'l2;
                }
                eggCycles = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_FRIENDSHIP);
                if eggCycles != 0 {
                    if eggCycles >= toSub as u32 {
                        eggCycles -= toSub as u32;
                    } else {
                        eggCycles -= 1;
                    }
                    SetMonData(
                        &raw mut gPlayerParty[i],
                        MON_DATA_FRIENDSHIP,
                        &raw mut eggCycles as *mut c_void,
                    );
                } else {
                    gSpecialVar_0x8004 = i as u16;
                    return TRUE;
                }
            }
            i += 1;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldEggHatch() -> u8 {
    return TryProduceOrHatchEgg(&raw mut (*gSaveBlock1Ptr).daycare);
}
pub(crate) unsafe extern "C" fn IsEggPending(daycare: *mut DayCare) -> u8 {
    return ((*daycare).offspringPersonality != 0) as u8;
}
pub(crate) unsafe extern "C" fn _GetDaycareMonNicknames(daycare: *mut DayCare) {
    let mut otName: CArray<u8, 12> = zeroed();
    if GetBoxMonData2(&raw mut (*daycare).mons[0].mon, MON_DATA_SPECIES) != 0 {
        GetBoxMonNickname(&raw mut (*daycare).mons[0].mon, gStringVar1.as_mut_ptr());
        GetBoxMonData3(
            &raw mut (*daycare).mons[0].mon,
            MON_DATA_OT_NAME,
            otName.as_mut_ptr(),
        );
        StringCopy(gStringVar3.as_mut_ptr(), otName.as_mut_ptr());
    }
    if GetBoxMonData2(&raw mut (*daycare).mons[1].mon, MON_DATA_SPECIES) != 0 {
        GetBoxMonNickname(&raw mut (*daycare).mons[1].mon, gStringVar2.as_mut_ptr());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSelectedMonNicknameAndSpecies() -> u16 {
    GetBoxMonNickname(
        &raw mut gPlayerParty[GetCursorSelectionMonId()].r#box,
        gStringVar1.as_mut_ptr(),
    );
    return GetBoxMonData2(
        &raw mut gPlayerParty[GetCursorSelectionMonId()].r#box,
        MON_DATA_SPECIES,
    ) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDaycareMonNicknames() {
    _GetDaycareMonNicknames(&raw mut (*gSaveBlock1Ptr).daycare);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDaycareState() -> u8 {
    let mut numMons: u8 = 0;
    if IsEggPending(&raw mut (*gSaveBlock1Ptr).daycare) != 0 {
        return DAYCARE_EGG_WAITING;
    }
    numMons = CountPokemonInDaycare(&raw mut (*gSaveBlock1Ptr).daycare);
    if numMons != 0 {
        return numMons + 1;
    }
    return DAYCARE_NO_MONS;
}
pub(crate) unsafe extern "C" fn GetDaycarePokemonCount() -> u8 {
    let mut ret: u8 = CountPokemonInDaycare(&raw mut (*gSaveBlock1Ptr).daycare);
    if ret != 0 {
        return ret;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn EggGroupsOverlap(eggGroups1: *mut u16, eggGroups2: *mut u16) -> u8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < EGG_GROUPS_PER_MON {
        j = 0;
        while j < EGG_GROUPS_PER_MON {
            if *eggGroups1.at(i) == *eggGroups2.at(j) {
                return TRUE;
            }
            j += 1;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetDaycareCompatibilityScore(daycare: *mut DayCare) -> u8 {
    let mut i: u32 = 0;
    let mut eggGroups: CArray<CArray<u16, 2>, 2> = zeroed();
    let mut species: CArray<u16, 2> = zeroed();
    let mut trainerIds: CArray<u32, 2> = zeroed();
    let mut genders: CArray<u32, 2> = zeroed();
    i = 0;
    while i < DAYCARE_MON_COUNT as u32 {
        let mut personality: u32 = 0;
        species[i] = GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES) as u16;
        trainerIds[i] = GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_OT_ID);
        personality = GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_PERSONALITY);
        genders[i] = GetGenderFromSpeciesAndPersonality(species[i], personality) as u32;
        eggGroups[i][0] = gSpeciesInfo[species[i]].eggGroups[0] as u16;
        eggGroups[i][1] = gSpeciesInfo[species[i]].eggGroups[1] as u16;
        i += 1;
    }
    if eggGroups[0][0] == EGG_GROUP_NO_EGGS_DISCOVERED
        || eggGroups[1][0] == EGG_GROUP_NO_EGGS_DISCOVERED
    {
        return PARENTS_INCOMPATIBLE;
    }
    if eggGroups[0][0] == EGG_GROUP_DITTO && eggGroups[1][0] == EGG_GROUP_DITTO {
        return PARENTS_INCOMPATIBLE;
    }
    if eggGroups[0][0] == EGG_GROUP_DITTO || eggGroups[1][0] == EGG_GROUP_DITTO {
        if trainerIds[0] == trainerIds[1] {
            return PARENTS_LOW_COMPATIBILITY;
        }
        return PARENTS_MED_COMPATIBILITY;
    } else {
        if genders[0] == genders[1] {
            return PARENTS_INCOMPATIBLE;
        }
        if genders[0] == MON_GENDERLESS as u32 || genders[1] == MON_GENDERLESS as u32 {
            return PARENTS_INCOMPATIBLE;
        }
        if EggGroupsOverlap(eggGroups[0].as_mut_ptr(), eggGroups[1].as_mut_ptr()) == 0 {
            return PARENTS_INCOMPATIBLE;
        }
        if species[0] == species[1] {
            if trainerIds[0] == trainerIds[1] {
                return PARENTS_MED_COMPATIBILITY;
            }
            return PARENTS_MAX_COMPATIBILITY;
        } else {
            if trainerIds[0] != trainerIds[1] {
                return PARENTS_MED_COMPATIBILITY;
            }
            return PARENTS_LOW_COMPATIBILITY;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetDaycareCompatibilityScoreFromSave() -> u8 {
    return GetDaycareCompatibilityScore(&raw mut (*gSaveBlock1Ptr).daycare);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDaycareCompatibilityString() {
    let mut whichString: u8 = 0;
    let mut relationshipScore: u8 = 0;
    relationshipScore = GetDaycareCompatibilityScoreFromSave();
    whichString = 0;
    if relationshipScore == PARENTS_INCOMPATIBLE {
        whichString = 3;
    }
    if relationshipScore == PARENTS_LOW_COMPATIBILITY {
        whichString = 2;
    }
    if relationshipScore == PARENTS_MED_COMPATIBILITY {
        whichString = 1;
    }
    if relationshipScore == PARENTS_MAX_COMPATIBILITY {
        whichString = 0;
    }
    StringCopy(
        gStringVar4.as_mut_ptr(),
        sCompatibilityMessages[whichString],
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn NameHasGenderSymbol(name: *mut u8, genderRatio: u8) -> u8 {
    let mut i: u8 = 0;
    let mut symbolsCount: CArray<u8, 2> = zeroed();
    symbolsCount[0] = {
        symbolsCount[1] = 0;
        symbolsCount[1]
    };
    i = 0;
    while *name.at(i) != EOS {
        if *name.at(i) == CHAR_MALE {
            symbolsCount[0] += 1;
        }
        if *name.at(i) == CHAR_FEMALE {
            symbolsCount[1] += 1;
        }
        i += 1;
    }
    if genderRatio == 0x00 && symbolsCount[0] != 0 && symbolsCount[1] == 0 {
        return TRUE;
    }
    if genderRatio == MON_FEMALE && symbolsCount[1] != 0 && symbolsCount[0] == 0 {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn AppendGenderSymbol(name: *mut u8, gender: u8) -> *mut u8 {
    if gender == MON_MALE {
        if NameHasGenderSymbol(name, MON_MALE) == 0 {
            return StringAppend(name, gText_MaleSymbol4.as_ptr().cast_mut());
        }
    } else if gender == MON_FEMALE {
        if NameHasGenderSymbol(name, MON_FEMALE) == 0 {
            return StringAppend(name, gText_FemaleSymbol4.as_ptr().cast_mut());
        }
    }
    return StringAppend(name, gText_GenderlessSymbol.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn AppendMonGenderSymbol(
    name: *mut u8,
    boxMon: *mut BoxPokemon,
) -> *mut u8 {
    return AppendGenderSymbol(name, GetBoxMonGender(boxMon));
}
pub(crate) unsafe extern "C" fn GetDaycareLevelMenuText(daycare: *mut DayCare, dest: *mut u8) {
    let mut monNames: CArray<CArray<u8, 20>, 2> = zeroed();
    let mut i: u8 = 0;
    *dest = EOS;
    i = 0;
    while i < DAYCARE_MON_COUNT {
        GetBoxMonNickname(&raw mut (*daycare).mons[i].mon, monNames[i].as_mut_ptr());
        AppendMonGenderSymbol(monNames[i].as_mut_ptr(), &raw mut (*daycare).mons[i].mon);
        i += 1;
    }
    StringCopy(dest, monNames[0].as_mut_ptr());
    StringAppend(dest, gText_NewLine2.as_ptr().cast_mut());
    StringAppend(dest, monNames[1].as_mut_ptr());
    StringAppend(dest, gText_NewLine2.as_ptr().cast_mut());
    StringAppend(dest, gText_Exit4.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn GetDaycareLevelMenuLevelText(daycare: *mut DayCare, dest: *mut u8) {
    let mut i: u8 = 0;
    let mut level: u8 = 0;
    let mut text: CArray<u8, 20> = zeroed();
    *dest = EOS;
    i = 0;
    while i < DAYCARE_MON_COUNT {
        StringAppend(dest, gText_Lv.as_ptr().cast_mut());
        level =
            GetLevelAfterDaycareSteps(&raw mut (*daycare).mons[i].mon, (*daycare).mons[i].steps);
        ConvertIntToDecimalStringN(text.as_mut_ptr(), level as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
        StringAppend(dest, text.as_mut_ptr());
        StringAppend(dest, gText_NewLine2.as_ptr().cast_mut());
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DaycareAddTextPrinter(windowId: u8, text: *mut u8, x: u32, y: u32) {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.currentChar = text;
    printer.windowId = windowId;
    printer.fontId = FONT_NORMAL;
    printer.x = x as u8;
    printer.y = y as u8;
    printer.currentX = x as u8;
    printer.currentY = y as u8;
    printer.set_unk(0);
    gTextFlags.set_useAlternateDownArrow(0);
    printer.letterSpacing = 0;
    printer.lineSpacing = 1;
    printer.set_fgColor(2);
    printer.set_bgColor(1);
    printer.set_shadowColor(3);
    AddTextPrinter(&raw mut printer, TEXT_SKIP_DRAW, None);
}
pub(crate) unsafe extern "C" fn DaycarePrintMonNickname(
    daycare: *mut DayCare,
    windowId: u8,
    daycareSlotId: u32,
    y: u32,
) {
    let mut nickname: CArray<u8, 20> = zeroed();
    GetBoxMonNickname(
        &raw mut (*daycare).mons[daycareSlotId].mon,
        nickname.as_mut_ptr(),
    );
    AppendMonGenderSymbol(
        nickname.as_mut_ptr(),
        &raw mut (*daycare).mons[daycareSlotId].mon,
    );
    DaycareAddTextPrinter(windowId, nickname.as_mut_ptr(), 8, y);
}
pub(crate) unsafe extern "C" fn DaycarePrintMonLvl(
    daycare: *mut DayCare,
    windowId: u8,
    daycareSlotId: u32,
    y: u32,
) {
    let mut level: u8 = 0;
    let mut x: u32 = 0;
    let mut lvlText: CArray<u8, 12> = zeroed();
    let mut intText: CArray<u8, 8> = zeroed();
    StringCopy(lvlText.as_mut_ptr(), gText_Lv.as_ptr().cast_mut());
    level = GetLevelAfterDaycareSteps(
        &raw mut (*daycare).mons[daycareSlotId].mon,
        (*daycare).mons[daycareSlotId].steps,
    );
    ConvertIntToDecimalStringN(
        intText.as_mut_ptr(),
        level as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    StringAppend(lvlText.as_mut_ptr(), intText.as_mut_ptr());
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, lvlText.as_mut_ptr(), 112) as u32;
    DaycareAddTextPrinter(windowId, lvlText.as_mut_ptr(), x, y);
}
pub(crate) unsafe extern "C" fn DaycarePrintMonInfo(windowId: u8, daycareSlotId: u32, y: u8) {
    if daycareSlotId < DAYCARE_MON_COUNT as u32 {
        DaycarePrintMonNickname(
            &raw mut (*gSaveBlock1Ptr).daycare,
            windowId,
            daycareSlotId,
            y as u32,
        );
        DaycarePrintMonLvl(
            &raw mut (*gSaveBlock1Ptr).daycare,
            windowId,
            daycareSlotId,
            y as u32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_HandleDaycareLevelMenuInput(taskId: u8) {
    let mut input: u32 = ListMenu_ProcessInput(gTasks[taskId].data[0] as u8) as u32;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        match input {
            0 | 1 => {
                gSpecialVar_Result = input as u16;
            }
            DAYCARE_LEVEL_MENU_EXIT => {
                gSpecialVar_Result = DAYCARE_EXITED_LEVEL_MENU;
            }
            _ => {}
        }
        DestroyListMenuTask(gTasks[taskId].data[0] as u8, null_mut(), null_mut());
        ClearStdWindowAndFrame(gTasks[taskId].data[1] as u8, TRUE);
        RemoveWindow(gTasks[taskId].data[1] as u8);
        DestroyTask(taskId);
        ScriptContext_Enable();
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        gSpecialVar_Result = DAYCARE_EXITED_LEVEL_MENU;
        DestroyListMenuTask(gTasks[taskId].data[0] as u8, null_mut(), null_mut());
        ClearStdWindowAndFrame(gTasks[taskId].data[1] as u8, TRUE);
        RemoveWindow(gTasks[taskId].data[1] as u8);
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowDaycareLevelMenu() {
    let mut menuTemplate: ListMenuTemplate = zeroed();
    let mut windowId: u8 = 0;
    let mut listMenuTaskId: u8 = 0;
    let mut daycareMenuTaskId: u8 = 0;
    windowId = AddWindow((&raw const *sDaycareLevelMenuWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(windowId, FALSE);
    menuTemplate = *sDaycareListMenuLevelTemplate;
    menuTemplate.windowId = windowId;
    listMenuTaskId = ListMenuInit(&raw mut menuTemplate, 0, 0);
    CopyWindowToVram(windowId, COPYWIN_FULL);
    daycareMenuTaskId = CreateTask(Some(Task_HandleDaycareLevelMenuInput), 3);
    gTasks[daycareMenuTaskId].data[0] = listMenuTaskId as i16;
    gTasks[daycareMenuTaskId].data[1] = windowId as i16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseSendDaycareMon() {
    ChooseMonForDaycare();
    gMain.savedCallback = Some(CB2_ReturnToField);
}
