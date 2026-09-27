use crate::ffi::{StringGet_Nickname, gSpecialVar_Result, gStringVar1};
use core::ffi::{c_int, c_void};

const PARTY_SIZE: usize = 6;
const POKEMON_SIZE: usize = 100;
const TASK_SIZE: usize = 40;
const TASK_DATA_OFFSET: usize = 8;

const SPECIES_NONE: u32 = 0;
const SPECIES_EGG: u32 = 412;
const MON_DATA_NICKNAME: c_int = 2;
const MON_DATA_SANITY_HAS_SPECIES: c_int = 5;
const MON_DATA_STATUS: c_int = 55;
const MON_DATA_HP: c_int = 57;
const MON_DATA_SPECIES_OR_EGG: c_int = 65;
const AILMENT_PSN: u8 = 1;
const FRIENDSHIP_EVENT_FAINT_FIELD_PSN: u8 = 7;

type TaskFunc = unsafe extern "C" fn(u8);

unsafe extern "C" {
    static mut gPlayerParty: u8;
    static mut gTasks: u8;
    static gText_PkmnFainted_FldPsn: u8;

    fn GetMonData2(mon: *mut u8, field: c_int) -> u32;
    fn GetMonData3(mon: *mut u8, field: c_int, destination: *mut u8) -> u32;
    fn SetMonData(mon: *mut u8, field: c_int, value: *const c_void);
    fn AdjustFriendship(mon: *mut u8, event: u8);
    fn GetAilmentFromStatus(status: u32) -> u8;
    fn ShowFieldMessage(string: *const u8) -> u8;
    fn IsFieldMessageBoxHidden() -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn InBattlePike() -> u8;
    fn InTrainerHillChallenge() -> u8;
    fn ScriptContext_Enable();
    fn ScriptContext_Stop();
    fn CreateTask(function: TaskFunc, priority: u8) -> u8;
    fn DestroyTask(task_id: u8);
    fn FldEffPoison_Start();
}

const fn task_data_offset(task_id: u8, index: usize) -> usize {
    task_id as usize * TASK_SIZE + TASK_DATA_OFFSET + index * 2
}

unsafe fn task_data(task_id: u8, index: usize) -> *mut i16 {
    unsafe {
        (&raw mut gTasks)
            .add(task_data_offset(task_id, index))
            .cast()
    }
}

unsafe fn party_mon(index: usize) -> *mut u8 {
    unsafe { (&raw mut gPlayerParty).add(index * POKEMON_SIZE) }
}

unsafe fn is_valid_species(mon: *mut u8) -> bool {
    let species = unsafe { GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) };
    species != SPECIES_NONE && species != SPECIES_EGG
}

unsafe fn all_mons_fainted() -> bool {
    let mut index = 0;
    while index < PARTY_SIZE {
        let mon = unsafe { party_mon(index) };
        if unsafe { is_valid_species(mon) } && unsafe { GetMonData2(mon, MON_DATA_HP) } != 0 {
            return false;
        }
        index += 1;
    }
    true
}

unsafe fn faint_from_field_poison(party_index: usize) {
    let mon = unsafe { party_mon(party_index) };
    let status = 0u32;
    unsafe { AdjustFriendship(mon, FRIENDSHIP_EVENT_FAINT_FIELD_PSN) };
    unsafe { SetMonData(mon, MON_DATA_STATUS, (&raw const status).cast()) };
    let string = (&raw mut gStringVar1).cast::<u8>();
    let _ = unsafe { GetMonData3(mon, MON_DATA_NICKNAME, string) };
    let _ = unsafe { StringGet_Nickname(string) };
}

unsafe fn mon_fainted_from_poison(party_index: usize) -> bool {
    let mon = unsafe { party_mon(party_index) };
    let valid = unsafe { is_valid_species(mon) };
    let hp = unsafe { GetMonData2(mon, MON_DATA_HP) };
    let ailment = unsafe { GetAilmentFromStatus(GetMonData2(mon, MON_DATA_STATUS)) };
    valid && hp == 0 && ailment == AILMENT_PSN
}

unsafe extern "C" fn task_try_field_poison_white_out(task_id: u8) {
    let state = unsafe { task_data(task_id, 0) };
    let party_index = unsafe { task_data(task_id, 1) };
    match unsafe { state.read() } {
        0 => {
            while unsafe { party_index.read() } < PARTY_SIZE as i16 {
                let index = unsafe { party_index.read() } as usize;
                if unsafe { mon_fainted_from_poison(index) } {
                    unsafe { faint_from_field_poison(index) };
                    let _ = unsafe { ShowFieldMessage(&raw const gText_PkmnFainted_FldPsn) };
                    unsafe { state.write(state.read() + 1) };
                    return;
                }
                unsafe { party_index.write(party_index.read() + 1) };
            }
            unsafe { state.write(2) };
        }
        1 => {
            if unsafe { IsFieldMessageBoxHidden() } != 0 {
                unsafe { state.write(state.read() - 1) };
            }
        }
        2 => {
            let result = if unsafe { all_mons_fainted() } {
                if (unsafe { CurrentBattlePyramidLocation() } | unsafe { InBattlePike() }) != 0
                    || unsafe { InTrainerHillChallenge() } != 0
                {
                    2
                } else {
                    1
                }
            } else {
                0
            };
            unsafe { (&raw mut gSpecialVar_Result).write(result) };
            unsafe { ScriptContext_Enable() };
            unsafe { DestroyTask(task_id) };
        }
        _ => {}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryFieldPoisonWhiteOut() {
    let _ = unsafe { CreateTask(task_try_field_poison_white_out, 80) };
    unsafe { ScriptContext_Stop() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPoisonFieldEffect() -> i32 {
    let mut poisoned = 0u32;
    let mut fainted = 0u32;
    let mut index = 0usize;
    while index < PARTY_SIZE {
        let mon = unsafe { party_mon(index) };
        if unsafe { GetMonData2(mon, MON_DATA_SANITY_HAS_SPECIES) } != 0
            && unsafe { GetAilmentFromStatus(GetMonData2(mon, MON_DATA_STATUS)) } == AILMENT_PSN
        {
            let mut hp = unsafe { GetMonData2(mon, MON_DATA_HP) };
            if hp == 0 {
                fainted += 1;
            } else {
                hp -= 1;
                if hp == 0 {
                    fainted += 1;
                }
            }
            unsafe { SetMonData(mon, MON_DATA_HP, (&raw const hp).cast()) };
            poisoned += 1;
        }
        index += 1;
    }

    if fainted != 0 || poisoned != 0 {
        unsafe { FldEffPoison_Start() };
    }
    if fainted != 0 {
        2
    } else if poisoned != 0 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_offsets_match_the_gba_task_structure() {
        assert_eq!(task_data_offset(0, 0), 8);
        assert_eq!(task_data_offset(1, 1), 50);
    }
}
