//! Translated from `src/daycare.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::missing_transmute_annotations,
    clippy::unnecessary_cast,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::battle_main::gMoveToLearn;
use crate::box_mon::{GetBoxMonData2, GetBoxMonData3};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::egg_hatch::GetEggCyclesToSubtract;
use crate::event_data::FlagSet;
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_Result};
use crate::list_menu::{DestroyListMenuTask, ListMenu_ProcessInput, ListMenuInit};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::menu::{ClearStdWindowAndFrame, DrawStdWindowFrame};
use crate::overworld::CB2_ReturnToField;
use crate::party_menu::{ChooseMonForDaycare, GetCursorSelectionMonId, ItemIdToBattleMoveId};
use crate::pokemon::{
    BoxMonRestorePP, BoxMonToMon, CalculateMonStats, CalculatePlayerPartyCount, CanMonLearnTMHM,
    CreateMon, DeleteFirstMoveAndGiveMoveToMon, GetBoxMonGender,
    GetGenderFromSpeciesAndPersonality, GetLevelFromBoxMonExp, GetLevelUpMovesBySpecies,
    GetMonData2, GetMonData3, GetNatureFromPersonality, GiveMoveToMon, MonTryLearningNewMove,
    SetMonData, TryIncrementMonLevel, ZeroBoxMonData, ZeroMonData, gPlayerParty, gPlayerPartyCount,
};
use crate::pokemon_storage_system::CompactPartySlots;
use crate::random::{Random, Random2, SeedRng2};
use crate::script::ScriptContext_Enable;
use crate::string_util::StripExtCtrlCodes;
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::DestroyTask;
use crate::task::{task_get, task_set};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, RemoveWindow};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `ClearMail` with this module's view of its types.
#[inline]
unsafe fn ClearMail(a0: *mut Mail) {
    unsafe {
        crate::mail_data::ClearMail(a0 as _);
    }
}
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `GetStringRightAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringRightAlignXOffset(a0, a1 as _, a2) }
}
/// `GiveMailToMon` with this module's view of its types.
#[inline]
unsafe fn GiveMailToMon(a0: *mut Pokemon, a1: *mut Mail) -> u8 {
    unsafe { crate::mail_data::GiveMailToMon(a0 as _, a1 as _) }
}
/// `MonHasMail` with this module's view of its types.
#[inline]
unsafe fn MonHasMail(a0: *mut Pokemon) -> u8 {
    unsafe { crate::mail_data::MonHasMail(a0 as _) }
}
/// `SetBoxMonData` with this module's view of its types.
#[inline]
unsafe fn SetBoxMonData(a0: *mut BoxPokemon, a1: i32, a2: *mut c_void) {
    unsafe {
        crate::box_mon::SetBoxMonData(a0 as _, a1, a2 as _);
    }
}
/// `StringAppend` with this module's view of its types.
#[inline]
unsafe fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringAppend(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCopy_Nickname` with this module's view of its types.
#[inline]
unsafe fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy_Nickname(a0 as _, a1 as _) as *mut u8 }
}
/// `TakeMailFromMon` with this module's view of its types.
#[inline]
unsafe fn TakeMailFromMon(a0: *mut Pokemon) {
    unsafe {
        crate::mail_data::TakeMailFromMon(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const tMenuListTaskId: usize = 0;
const tWindowId: usize = 1;
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

/// `AddTextPrinter` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinter(
    a0: *mut TextPrinterTemplate,
    a1: u8,
    a2: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe { crate::text::AddTextPrinter(a0 as _, a1, core::mem::transmute(a2)) }
}

pub unsafe fn GetMonNickname2(mon: *mut Pokemon, dest: *mut u8) -> *mut u8 {
    let mut nickname: CArray<u8, 20> = zeroed();
    GetMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
    StringCopy_Nickname(dest, nickname.as_mut_ptr())
}
pub unsafe fn GetBoxMonNickname(mon: *mut BoxPokemon, dest: *mut u8) -> *mut u8 {
    let mut nickname: CArray<u8, 20> = zeroed();
    GetBoxMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
    StringCopy_Nickname(dest, nickname.as_mut_ptr())
}
pub unsafe fn CountPokemonInDaycare(daycare: *mut DayCare) -> u8 {
    let mut count: u8 = 0;
    for i in 0..DAYCARE_MON_COUNT {
        if GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES) != 0 {
            count += 1;
        }
    }
    count
}
pub unsafe fn InitDaycareMailRecordMixing(
    daycare: *mut DayCare,
    mixMail: *mut RecordMixingDaycareMail,
) {
    let mut numDaycareMons: u8 = 0;
    for i in 0..DAYCARE_MON_COUNT {
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
    }
    (*mixMail).numDaycareMons = numDaycareMons as u32;
}
unsafe fn Daycare_FindEmptySpot(daycare: *mut DayCare) -> i8 {
    for i in 0..DAYCARE_MON_COUNT {
        if GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES) == SPECIES_NONE as u32
        {
            return i as i8;
        }
    }
    -1
}
unsafe fn StorePokemonInDaycare(mon: *mut Pokemon, daycareMon: *mut DaycareMon) {
    if MonHasMail(mon) != 0 {
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
        let mailId: u8 = GetMonData2(mon, MON_DATA_MAIL) as u8;
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
unsafe fn StorePokemonInEmptyDaycareSlot(mon: *mut Pokemon, daycare: *mut DayCare) {
    let slotId: i8 = Daycare_FindEmptySpot(daycare);
    StorePokemonInDaycare(mon, &raw mut (*daycare).mons[slotId]);
}
#[unsafe(no_mangle)]
pub unsafe fn StoreSelectedPokemonInDaycare() {
    let monId: u8 = GetCursorSelectionMonId();
    StorePokemonInEmptyDaycareSlot(
        &raw mut gPlayerParty[monId],
        &raw mut (*gSaveBlock1Ptr).daycare,
    );
}
unsafe fn ShiftDaycareSlots(daycare: *mut DayCare) {
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
unsafe fn ApplyDaycareExperience(mon: *mut Pokemon) {
    let mut firstMove: u8 = 0;
    let mut learnedMove: u16 = 0;
    for i in 0..(MAX_LEVEL as i32) {
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
    }
    CalculateMonStats(mon);
}
unsafe fn TakeSelectedPokemonFromDaycare(daycareMon: *mut DaycareMon) -> u16 {
    let mut experience: u32 = 0;
    let mut pokemon: Pokemon = zeroed();
    GetBoxMonNickname(&raw mut (*daycareMon).mon, gStringVar1.as_mut_ptr());
    let species: u16 = GetBoxMonData2(&raw mut (*daycareMon).mon, MON_DATA_SPECIES) as u16;
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
    species
}
unsafe fn TakeSelectedPokemonMonFromDaycareShiftSlots(daycare: *mut DayCare, slotId: u8) -> u16 {
    let species: u16 = TakeSelectedPokemonFromDaycare(&raw mut (*daycare).mons[slotId]);
    ShiftDaycareSlots(daycare);
    species
}
#[unsafe(no_mangle)]
pub unsafe fn TakePokemonFromDaycare() -> u16 {
    TakeSelectedPokemonMonFromDaycareShiftSlots(
        &raw mut (*gSaveBlock1Ptr).daycare,
        gSpecialVar_0x8004 as u8,
    )
}
unsafe fn GetLevelAfterDaycareSteps(mon: *mut BoxPokemon, steps: u32) -> u8 {
    let mut tempMon: BoxPokemon = *mon;
    let mut experience: u32 = GetBoxMonData2(mon, MON_DATA_EXP) + steps;
    SetBoxMonData(
        &raw mut tempMon,
        MON_DATA_EXP,
        &raw mut experience as *mut c_void,
    );
    GetLevelFromBoxMonExp(&raw mut tempMon)
}
unsafe fn GetNumLevelsGainedFromSteps(daycareMon: *mut DaycareMon) -> u8 {
    let levelBefore: u8 = GetLevelFromBoxMonExp(&raw mut (*daycareMon).mon);
    let levelAfter: u8 = GetLevelAfterDaycareSteps(&raw mut (*daycareMon).mon, (*daycareMon).steps);
    levelAfter - levelBefore
}
unsafe fn GetNumLevelsGainedForDaycareMon(daycareMon: *mut DaycareMon) -> u8 {
    let numLevelsGained: u8 = GetNumLevelsGainedFromSteps(daycareMon);
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        numLevelsGained as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        2,
    );
    GetBoxMonNickname(&raw mut (*daycareMon).mon, gStringVar1.as_mut_ptr());
    numLevelsGained
}
unsafe fn PrepareDaycareCostStringForSelectedMon(daycareMon: *mut DaycareMon) -> u32 {
    let numLevelsGained: u8 = GetNumLevelsGainedFromSteps(daycareMon);
    GetBoxMonNickname(&raw mut (*daycareMon).mon, gStringVar1.as_mut_ptr());
    let cost: u32 = 100 + 100 * numLevelsGained as u32;
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        cost as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        5,
    );
    cost
}
unsafe fn PrepareDaycareCostStringForMon(daycare: *mut DayCare, slotId: u8) -> u16 {
    PrepareDaycareCostStringForSelectedMon(&raw mut (*daycare).mons[slotId]) as u16
}
#[unsafe(no_mangle)]
pub unsafe fn GetDaycareCostAndPrepareString() {
    gSpecialVar_0x8005 = PrepareDaycareCostStringForMon(
        &raw mut (*gSaveBlock1Ptr).daycare,
        gSpecialVar_0x8004 as u8,
    );
}
unsafe fn Debug_AddDaycareSteps(numSteps: u16) {
    (*gSaveBlock1Ptr).daycare.mons[0].steps += numSteps as u32;
    (*gSaveBlock1Ptr).daycare.mons[1].steps += numSteps as u32;
}
#[unsafe(no_mangle)]
pub unsafe fn GetNumLevelsGainedFromDaycare() -> u8 {
    if GetBoxMonData2(
        &raw mut (*gSaveBlock1Ptr).daycare.mons[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()]
        .mon,
        MON_DATA_SPECIES,
    ) != 0
    {
        return GetNumLevelsGainedForDaycareMon(
            &raw mut (*gSaveBlock1Ptr).daycare.mons[*(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut()],
        );
    }
    0
}
unsafe fn ClearDaycareMonMail(mail: *mut DaycareMail) {
    for i in 0..8i32 {
        (*mail).otName[i] = 0;
    }
    for i in 0..11i32 {
        (*mail).monName[i] = 0;
    }
    ClearMail(&raw mut (*mail).message);
}
unsafe fn ClearDaycareMon(daycareMon: *mut DaycareMon) {
    ZeroBoxMonData(&raw mut (*daycareMon).mon);
    (*daycareMon).steps = 0;
    ClearDaycareMonMail(&raw mut (*daycareMon).mail);
}
unsafe fn ClearAllDaycareData(daycare: *mut DayCare) {
    for i in 0..DAYCARE_MON_COUNT {
        ClearDaycareMon(&raw mut (*daycare).mons[i]);
    }
    (*daycare).offspringPersonality = 0;
    (*daycare).stepCounter = 0;
}
unsafe fn GetEggSpecies(mut species: u16) -> u16 {
    let mut j: i32 = 0;
    let mut found: u8 = 0;
    for i in 0..EVOS_PER_MON {
        found = FALSE;
        j = 1;
        while j < NUM_SPECIES as i32 {
            for k in 0..EVOS_PER_MON {
                if (*(&raw const crate::data::pokemon::gEvolutionTable)
                    .cast::<CArray<CArray<Evolution, 5>, 0>>())[j][k]
                    .targetSpecies
                    == species
                {
                    species = j as u16;
                    found = TRUE;
                    break;
                }
            }
            if found != 0 {
                break;
            }
            j += 1;
        }
        if j == NUM_SPECIES as i32 {
            break;
        }
    }
    species
}
unsafe fn GetParentToInheritNature(daycare: *mut DayCare) -> i32 {
    let mut species: CArray<u32, 2> = zeroed();
    let mut parent: i32 = -1;
    let mut i: i32 = 0;
    while i < DAYCARE_MON_COUNT as i32 {
        if GetBoxMonGender(&raw mut (*daycare).mons[i].mon) == MON_FEMALE {
            parent = i;
        }
        i += 1;
    }
    let mut dittoCount: i32 = 0;
    for i in 0..(DAYCARE_MON_COUNT as i32) {
        species[i] = GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES);
        if species[i] == SPECIES_DITTO as u32 {
            dittoCount += 1;
            parent = i;
        }
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
    parent
}
unsafe fn _TriggerPendingDaycareEgg(daycare: *mut DayCare) {
    let mut natureTries: i32 = 0;
    SeedRng2(gMain.vblankCounter2 as u16);
    let parent: i32 = GetParentToInheritNature(daycare);
    if parent < 0 {
        (*daycare).offspringPersonality =
            ((Random2() as u32) << 16) | ((Random() as i32 % 65534) as u32 + 1);
    } else {
        let wantedNature: u8 = GetNatureFromPersonality(GetBoxMonData3(
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
unsafe fn _TriggerPendingDaycareMaleEgg(daycare: *mut DayCare) {
    (*daycare).offspringPersonality = Random() as u32 | EGG_GENDER_MALE;
    FlagSet(FLAG_PENDING_DAYCARE_EGG);
}
pub unsafe fn TriggerPendingDaycareEgg() {
    _TriggerPendingDaycareEgg(&raw mut (*gSaveBlock1Ptr).daycare);
}
unsafe fn TriggerPendingDaycareMaleEgg() {
    _TriggerPendingDaycareMaleEgg(&raw mut (*gSaveBlock1Ptr).daycare);
}
unsafe fn RemoveIVIndexFromList(ivs: *mut u8, selectedIv: u8) {
    let mut temp: CArray<u8, 6> = zeroed();
    *ivs.at(selectedIv) = 0xFF;
    let mut i: i32 = 0;
    while i < NUM_STATS {
        temp[i] = *ivs.at(i);
        i += 1;
    }
    let mut j: i32 = 0;
    for i in 0..NUM_STATS {
        if temp[i] != 0xFF {
            *ivs.at({
                let t1 = j;
                j += 1;
                t1
            }) = temp[i];
        }
    }
}
unsafe fn InheritIVs(egg: *mut Pokemon, daycare: *mut DayCare) {
    let mut selectedIvs: CArray<u8, 3> = zeroed();
    let mut availableIVs: CArray<u8, 6> = zeroed();
    let mut whichParents: CArray<u8, 3> = zeroed();
    let mut iv: u8 = 0;
    for i in 0..(NUM_STATS as u8) {
        availableIVs[i] = i;
    }
    for i in 0..INHERITED_IV_COUNT {
        selectedIvs[i] = availableIVs[rem_i32(Random() as i32, NUM_STATS - i as i32)];
        RemoveIVIndexFromList(availableIVs.as_mut_ptr(), i);
    }
    let mut i: u8 = 0;
    while i < INHERITED_IV_COUNT {
        whichParents[i] = (Random() as i32 % 2) as u8;
        i += 1;
    }
    for i in 0..INHERITED_IV_COUNT {
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
    }
}
unsafe fn GetEggMoves(pokemon: *mut Pokemon, eggMoves: *mut u16) -> u8 {
    let mut numEggMoves: u16 = 0;
    let mut eggMoveIdx: u16 = 0;
    let species: u16 = GetMonData2(pokemon, MON_DATA_SPECIES) as u16;
    let mut i: u16 = 0;
    while i < 1138 {
        if gEggMoves[i] as i32 == species as i32 + EGG_MOVES_SPECIES_OFFSET {
            eggMoveIdx = i + 1;
            break;
        }
        i += 1;
    }
    for i in 0..EGG_MOVES_ARRAY_COUNT {
        if gEggMoves[eggMoveIdx as i32 + i as i32] > EGG_MOVES_SPECIES_OFFSET as u16 {
            break;
        }
        *eggMoves.at(i) = gEggMoves[eggMoveIdx as i32 + i as i32];
        numEggMoves += 1;
    }
    numEggMoves as u8
}
unsafe fn BuildEggMoveset(egg: *mut Pokemon, father: *mut BoxPokemon, mother: *mut BoxPokemon) {
    let mut j: u16 = 0;
    let mut numSharedParentMoves: u16 = 0;
    for i in 0..(MAX_MON_MOVES as u16) {
        sHatchedEggMotherMoves[i] = MOVE_NONE;
        sHatchedEggFatherMoves[i] = MOVE_NONE;
        sHatchedEggFinalMoves[i] = MOVE_NONE;
    }
    for i in 0..EGG_MOVES_ARRAY_COUNT {
        sHatchedEggEggMoves[i] = MOVE_NONE;
    }
    let mut i: u16 = 0;
    while (i as i32) < (if 20 > 50 { 20 } else { 50 }) {
        sHatchedEggLevelUpMoves[i] = MOVE_NONE;
        i += 1;
    }
    let numLevelUpMoves: u32 = GetLevelUpMovesBySpecies(
        GetMonData2(egg, MON_DATA_SPECIES) as u16,
        sHatchedEggLevelUpMoves.as_mut_ptr(),
    ) as u32;
    for i in 0..(MAX_MON_MOVES as u16) {
        sHatchedEggFatherMoves[i] = GetBoxMonData2(father, MON_DATA_MOVE1 + i as i32) as u16;
        sHatchedEggMotherMoves[i] = GetBoxMonData2(mother, MON_DATA_MOVE1 + i as i32) as u16;
    }
    let numEggMoves: u16 = GetEggMoves(egg, sHatchedEggEggMoves.as_mut_ptr()) as u16;
    for i in 0..(MAX_MON_MOVES as u16) {
        if sHatchedEggFatherMoves[i] != MOVE_NONE {
            for j in 0..numEggMoves {
                if sHatchedEggFatherMoves[i] == sHatchedEggEggMoves[j] {
                    if GiveMoveToMon(egg, sHatchedEggFatherMoves[i]) == MON_HAS_MAX_MOVES {
                        DeleteFirstMoveAndGiveMoveToMon(egg, sHatchedEggFatherMoves[i]);
                    }
                    break;
                }
            }
        } else {
            break;
        }
    }
    for i in 0..(MAX_MON_MOVES as u16) {
        if sHatchedEggFatherMoves[i] != MOVE_NONE {
            for j in 0..58u16 {
                if sHatchedEggFatherMoves[i] == ItemIdToBattleMoveId(ITEM_TM01 + j)
                    && CanMonLearnTMHM(egg, j as u8) != 0
                    && GiveMoveToMon(egg, sHatchedEggFatherMoves[i]) == MON_HAS_MAX_MOVES
                {
                    DeleteFirstMoveAndGiveMoveToMon(egg, sHatchedEggFatherMoves[i]);
                }
            }
        }
    }
    i = 0;
    while i < MAX_MON_MOVES as u16 {
        if sHatchedEggFatherMoves[i] == MOVE_NONE {
            break;
        }
        for j in 0..(MAX_MON_MOVES as u16) {
            if sHatchedEggFatherMoves[i] == sHatchedEggMotherMoves[j]
                && sHatchedEggFatherMoves[i] != MOVE_NONE
            {
                sHatchedEggFinalMoves[{
                    let t1 = numSharedParentMoves;
                    numSharedParentMoves += 1;
                    t1
                }] = sHatchedEggFatherMoves[i];
            }
        }
        i += 1;
    }
    for i in 0..(MAX_MON_MOVES as u16) {
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
    }
}
unsafe fn RemoveEggFromDayCare(daycare: *mut DayCare) {
    (*daycare).offspringPersonality = 0;
    (*daycare).stepCounter = 0;
}
#[unsafe(no_mangle)]
pub unsafe fn RejectEggFromDayCare() {
    RemoveEggFromDayCare(&raw mut (*gSaveBlock1Ptr).daycare);
}
unsafe fn AlterEggSpeciesWithIncenseItem(species: *mut u16, daycare: *mut DayCare) {
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
unsafe fn GiveVoltTackleIfLightBall(mon: *mut Pokemon, daycare: *mut DayCare) {
    let motherItem: u32 = GetBoxMonData2(&raw mut (*daycare).mons[0].mon, MON_DATA_HELD_ITEM);
    let fatherItem: u32 = GetBoxMonData2(&raw mut (*daycare).mons[1].mon, MON_DATA_HELD_ITEM);
    if (motherItem == ITEM_LIGHT_BALL || fatherItem == ITEM_LIGHT_BALL)
        && GiveMoveToMon(mon, MOVE_VOLT_TACKLE) == MON_HAS_MAX_MOVES
    {
        DeleteFirstMoveAndGiveMoveToMon(mon, MOVE_VOLT_TACKLE);
    }
}
unsafe fn DetermineEggSpeciesAndParentSlots(daycare: *mut DayCare, parentSlots: *mut u8) -> u16 {
    let mut species: CArray<u16, 2> = zeroed();
    for i in 0..(DAYCARE_MON_COUNT as u16) {
        species[i] = GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES) as u16;
        if species[i] == SPECIES_DITTO {
            *parentSlots = i as u8 ^ 1;
            *parentSlots.at(1) = i as u8;
        } else if GetBoxMonGender(&raw mut (*daycare).mons[i].mon) == MON_FEMALE {
            *parentSlots = i as u8;
            *parentSlots.at(1) = i as u8 ^ 1;
        }
    }
    let mut eggSpecies: u16 = GetEggSpecies(species[*parentSlots]);
    if eggSpecies == SPECIES_NIDORAN_F && (*daycare).offspringPersonality & EGG_GENDER_MALE != 0 {
        eggSpecies = SPECIES_NIDORAN_M;
    }
    if eggSpecies == SPECIES_ILLUMISE && (*daycare).offspringPersonality & EGG_GENDER_MALE != 0 {
        eggSpecies = SPECIES_VOLBEAT;
    }
    if species[*parentSlots.at(1)] == SPECIES_DITTO
        && GetBoxMonGender(&raw mut (*daycare).mons[*parentSlots].mon) != MON_FEMALE
    {
        let ditto: u8 = *parentSlots.at(1);
        *parentSlots.at(1) = *parentSlots;
        *parentSlots = ditto;
    }
    eggSpecies
}
unsafe fn _GiveEggFromDaycare(daycare: *mut DayCare) {
    let mut egg: Pokemon = zeroed();
    let mut parentSlots: CArray<u8, 2> = zeroed();
    let mut species: u16 = DetermineEggSpeciesAndParentSlots(daycare, parentSlots.as_mut_ptr());
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
    let mut isEgg: u8 = TRUE;
    SetMonData(&raw mut egg, MON_DATA_IS_EGG, &raw mut isEgg as *mut c_void);
    gPlayerParty[5] = egg;
    CompactPartySlots();
    CalculatePlayerPartyCount();
    RemoveEggFromDayCare(daycare);
}
#[unsafe(no_mangle)]
pub unsafe fn CreateEgg(mon: *mut Pokemon, species: u16, setHotSpringsLocation: u8) {
    let mut metLocation: u8 = 0;
    CreateMon(mon, species, EGG_HATCH_LEVEL, USE_RANDOM_IVS, 0, 0, 0, 0);
    let mut metLevel: u8 = 0;
    let mut ball: u16 = ITEM_POKE_BALL;
    let mut language: u8 = LANGUAGE_JAPANESE;
    SetMonData(mon, MON_DATA_POKEBALL, &raw mut ball as *mut c_void);
    SetMonData(
        mon,
        MON_DATA_NICKNAME,
        sJapaneseEggNickname.as_ptr().cast_mut() as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_FRIENDSHIP,
        (&raw const (*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[species]
            .eggCycles)
            .cast_mut() as *mut c_void,
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
    let mut isEgg: u8 = TRUE;
    SetMonData(mon, MON_DATA_IS_EGG, &raw mut isEgg as *mut c_void);
}
unsafe fn SetInitialEggData(mon: *mut Pokemon, species: u16, daycare: *mut DayCare) {
    let personality: u32 = (*daycare).offspringPersonality;
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
    let mut metLevel: u8 = 0;
    let mut ball: u16 = ITEM_POKE_BALL;
    let mut language: u8 = LANGUAGE_JAPANESE;
    SetMonData(mon, MON_DATA_POKEBALL, &raw mut ball as *mut c_void);
    SetMonData(
        mon,
        MON_DATA_NICKNAME,
        sJapaneseEggNickname.as_ptr().cast_mut() as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_FRIENDSHIP,
        (&raw const (*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[species]
            .eggCycles)
            .cast_mut() as *mut c_void,
    );
    SetMonData(mon, MON_DATA_MET_LEVEL, &raw mut metLevel as *mut c_void);
    SetMonData(mon, MON_DATA_LANGUAGE, &raw mut language as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe fn GiveEggFromDaycare() {
    _GiveEggFromDaycare(&raw mut (*gSaveBlock1Ptr).daycare);
}
unsafe fn TryProduceOrHatchEgg(daycare: *mut DayCare) -> u8 {
    let mut validEggs: u32 = 0;
    let mut i: u32 = 0;
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
        let compatibility: u8 = GetDaycareCompatibilityScore(daycare);
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
        let toSub: u8 = GetEggCyclesToSubtract();
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
    FALSE
}
pub unsafe fn ShouldEggHatch() -> u8 {
    TryProduceOrHatchEgg(&raw mut (*gSaveBlock1Ptr).daycare)
}
unsafe fn IsEggPending(daycare: *mut DayCare) -> u8 {
    ((*daycare).offspringPersonality != 0) as u8
}
unsafe fn _GetDaycareMonNicknames(daycare: *mut DayCare) {
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
pub unsafe fn GetSelectedMonNicknameAndSpecies() -> u16 {
    GetBoxMonNickname(
        &raw mut gPlayerParty[GetCursorSelectionMonId()].r#box,
        gStringVar1.as_mut_ptr(),
    );
    GetBoxMonData2(
        &raw mut gPlayerParty[GetCursorSelectionMonId()].r#box,
        MON_DATA_SPECIES,
    ) as u16
}
#[unsafe(no_mangle)]
pub unsafe fn GetDaycareMonNicknames() {
    _GetDaycareMonNicknames(&raw mut (*gSaveBlock1Ptr).daycare);
}
#[unsafe(no_mangle)]
pub unsafe fn GetDaycareState() -> u8 {
    if IsEggPending(&raw mut (*gSaveBlock1Ptr).daycare) != 0 {
        return DAYCARE_EGG_WAITING;
    }
    let numMons: u8 = CountPokemonInDaycare(&raw mut (*gSaveBlock1Ptr).daycare);
    if numMons != 0 {
        return numMons + 1;
    }
    DAYCARE_NO_MONS
}
unsafe fn GetDaycarePokemonCount() -> u8 {
    let ret: u8 = CountPokemonInDaycare(&raw mut (*gSaveBlock1Ptr).daycare);
    if ret != 0 {
        return ret;
    }
    0
}
unsafe fn EggGroupsOverlap(eggGroups1: *mut u16, eggGroups2: *mut u16) -> u8 {
    for i in 0..EGG_GROUPS_PER_MON {
        for j in 0..EGG_GROUPS_PER_MON {
            if *eggGroups1.at(i) == *eggGroups2.at(j) {
                return TRUE;
            }
        }
    }
    FALSE
}
unsafe fn GetDaycareCompatibilityScore(daycare: *mut DayCare) -> u8 {
    let mut eggGroups: CArray<CArray<u16, 2>, 2> = zeroed();
    let mut species: CArray<u16, 2> = zeroed();
    let mut trainerIds: CArray<u32, 2> = zeroed();
    let mut genders: CArray<u32, 2> = zeroed();
    for i in 0..(DAYCARE_MON_COUNT as u32) {
        species[i] = GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_SPECIES) as u16;
        trainerIds[i] = GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_OT_ID);
        let personality: u32 =
            GetBoxMonData2(&raw mut (*daycare).mons[i].mon, MON_DATA_PERSONALITY);
        genders[i] = GetGenderFromSpeciesAndPersonality(species[i], personality) as u32;
        eggGroups[i][0] = (*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[species[i]]
            .eggGroups[0] as u16;
        eggGroups[i][1] = (*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[species[i]]
            .eggGroups[1] as u16;
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
        0
    }
}
unsafe fn GetDaycareCompatibilityScoreFromSave() -> u8 {
    GetDaycareCompatibilityScore(&raw mut (*gSaveBlock1Ptr).daycare)
}
#[unsafe(no_mangle)]
pub unsafe fn SetDaycareCompatibilityString() {
    let relationshipScore: u8 = GetDaycareCompatibilityScoreFromSave();
    let mut whichString: u8 = 0;
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
pub unsafe fn NameHasGenderSymbol(name: *mut u8, genderRatio: u8) -> u8 {
    let mut symbolsCount: CArray<u8, 2> = zeroed();
    symbolsCount[0] = {
        symbolsCount[1] = 0;
        symbolsCount[1]
    };
    let mut i: u8 = 0;
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
    FALSE
}
unsafe fn AppendGenderSymbol(name: *mut u8, gender: u8) -> *mut u8 {
    if gender == MON_MALE {
        if NameHasGenderSymbol(name, MON_MALE) == 0 {
            return StringAppend(
                name,
                (*(&raw const crate::data::trade::gText_MaleSymbol4).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
    } else if gender == MON_FEMALE && NameHasGenderSymbol(name, MON_FEMALE) == 0 {
        return StringAppend(
            name,
            (*(&raw const crate::data::trade::gText_FemaleSymbol4).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
    StringAppend(
        name,
        (*(&raw const crate::data::trade::gText_GenderlessSymbol).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    )
}
unsafe fn AppendMonGenderSymbol(name: *mut u8, boxMon: *mut BoxPokemon) -> *mut u8 {
    AppendGenderSymbol(name, GetBoxMonGender(boxMon))
}
unsafe fn GetDaycareLevelMenuText(daycare: *mut DayCare, dest: *mut u8) {
    let mut monNames: CArray<CArray<u8, 20>, 2> = zeroed();
    *dest = EOS;
    for i in 0..DAYCARE_MON_COUNT {
        GetBoxMonNickname(&raw mut (*daycare).mons[i].mon, monNames[i].as_mut_ptr());
        AppendMonGenderSymbol(monNames[i].as_mut_ptr(), &raw mut (*daycare).mons[i].mon);
    }
    StringCopy(dest, monNames[0].as_mut_ptr());
    StringAppend(
        dest,
        (*(&raw const crate::data::strings::gText_NewLine2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    StringAppend(dest, monNames[1].as_mut_ptr());
    StringAppend(
        dest,
        (*(&raw const crate::data::strings::gText_NewLine2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    StringAppend(
        dest,
        (*(&raw const crate::data::strings::gText_Exit4).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
unsafe fn GetDaycareLevelMenuLevelText(daycare: *mut DayCare, dest: *mut u8) {
    let mut level: u8 = 0;
    let mut text: CArray<u8, 20> = zeroed();
    *dest = EOS;
    for i in 0..DAYCARE_MON_COUNT {
        StringAppend(
            dest,
            (*(&raw const crate::data::strings::gText_Lv).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        level =
            GetLevelAfterDaycareSteps(&raw mut (*daycare).mons[i].mon, (*daycare).mons[i].steps);
        ConvertIntToDecimalStringN(text.as_mut_ptr(), level as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
        StringAppend(dest, text.as_mut_ptr());
        StringAppend(
            dest,
            (*(&raw const crate::data::strings::gText_NewLine2).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
}
unsafe fn DaycareAddTextPrinter(windowId: u8, text: *mut u8, x: u32, y: u32) {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.currentChar = text;
    printer.windowId = windowId;
    printer.fontId = FONT_NORMAL;
    printer.x = x as u8;
    printer.y = y as u8;
    printer.currentX = x as u8;
    printer.currentY = y as u8;
    printer.set_unk(0);
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_useAlternateDownArrow(0);
    printer.letterSpacing = 0;
    printer.lineSpacing = 1;
    printer.set_fgColor(2);
    printer.set_bgColor(1);
    printer.set_shadowColor(3);
    AddTextPrinter(&raw mut printer, TEXT_SKIP_DRAW, None);
}
unsafe fn DaycarePrintMonNickname(daycare: *mut DayCare, windowId: u8, daycareSlotId: u32, y: u32) {
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
unsafe fn DaycarePrintMonLvl(daycare: *mut DayCare, windowId: u8, daycareSlotId: u32, y: u32) {
    let mut lvlText: CArray<u8, 12> = zeroed();
    let mut intText: CArray<u8, 8> = zeroed();
    StringCopy(
        lvlText.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_Lv).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let level: u8 = GetLevelAfterDaycareSteps(
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
    let x: u32 = GetStringRightAlignXOffset(FONT_NORMAL as i32, lvlText.as_mut_ptr(), 112) as u32;
    DaycareAddTextPrinter(windowId, lvlText.as_mut_ptr(), x, y);
}
pub(crate) unsafe fn DaycarePrintMonInfo(windowId: u8, daycareSlotId: u32, y: u8) {
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
pub(crate) unsafe fn Task_HandleDaycareLevelMenuInput(taskId: u8) {
    let input: u32 = ListMenu_ProcessInput(task_get(taskId, tMenuListTaskId) as u8) as u32;
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
        DestroyListMenuTask(
            task_get(taskId, tMenuListTaskId) as u8,
            null_mut(),
            null_mut(),
        );
        ClearStdWindowAndFrame(task_get(taskId, tWindowId) as u8, TRUE);
        RemoveWindow(task_get(taskId, tWindowId) as u8);
        DestroyTask(taskId);
        ScriptContext_Enable();
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        gSpecialVar_Result = DAYCARE_EXITED_LEVEL_MENU;
        DestroyListMenuTask(
            task_get(taskId, tMenuListTaskId) as u8,
            null_mut(),
            null_mut(),
        );
        ClearStdWindowAndFrame(task_get(taskId, tWindowId) as u8, TRUE);
        RemoveWindow(task_get(taskId, tWindowId) as u8);
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ShowDaycareLevelMenu() {
    let windowId: u8 = AddWindow((&raw const *sDaycareLevelMenuWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(windowId, FALSE);
    let mut menuTemplate: ListMenuTemplate = *sDaycareListMenuLevelTemplate;
    menuTemplate.windowId = windowId;
    let listMenuTaskId: u8 = ListMenuInit(&raw mut menuTemplate, 0, 0);
    CopyWindowToVram(windowId, COPYWIN_FULL);
    let daycareMenuTaskId: u8 = CreateTask(Some(Task_HandleDaycareLevelMenuInput), 3);
    task_set(daycareMenuTaskId, tMenuListTaskId, listMenuTaskId as i16);
    task_set(daycareMenuTaskId, tWindowId, windowId as i16);
}
#[unsafe(no_mangle)]
pub unsafe fn ChooseSendDaycareMon() {
    ChooseMonForDaycare();
    gMain.savedCallback = Some(CB2_ReturnToField);
}
