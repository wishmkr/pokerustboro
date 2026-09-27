use crate::ffi::FlagGet;
use core::ptr::{addr_of, addr_of_mut};

const FLAG_WATTSON_REMATCH_AVAILABLE: u16 = 0x5b;
const FLAG_SYS_GAME_CLEAR: u16 = 0x864;
const REMATCHES_COUNT: usize = 5;
const REMATCH_TABLE_ENTRIES: usize = 78;
const MAX_REMATCH_ENTRIES: usize = 100;

const AFTER_NEW_MAUVILLE: [u16; 8] = [65, 66, 67, 68, 69, 70, 71, 72];
const BEFORE_NEW_MAUVILLE: [u16; 7] = [65, 66, 68, 69, 70, 71, 72];

#[repr(C)]
struct RematchTrainer {
    trainer_ids: [u16; REMATCHES_COUNT],
    map_group: u16,
    map_num: u16,
}

#[repr(C)]
struct SaveBlock1RematchView {
    prefix: [u8; 0x9ca],
    trainer_rematches: [u8; MAX_REMATCH_ENTRIES],
}

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut SaveBlock1RematchView;
    static gRematchTable: [RematchTrainer; REMATCH_TABLE_ENTRIES];
    fn Random() -> u16;
    fn HasTrainerBeenFought(trainer_id: u16) -> u8;
}

unsafe fn rematch_index(trainer_index: usize) -> i32 {
    let table = &raw const gRematchTable;
    let entry = unsafe { table.cast::<RematchTrainer>().add(trainer_index) };
    let mut index = 0usize;
    while index < REMATCHES_COUNT {
        let trainer_id = unsafe { addr_of!((*entry).trainer_ids[index]).read() };
        if unsafe { HasTrainerBeenFought(trainer_id) } == 0 {
            return index as i32;
        }
        index += 1;
    }
    REMATCHES_COUNT as i32
}

unsafe fn update_from_array(entries: &[u16], max_rematch: i32) {
    let save = unsafe { gSaveBlock1Ptr };
    let rematches = unsafe { addr_of_mut!((*save).trainer_rematches).cast::<u8>() };
    let mut candidates = 0i32;
    let mut lowest_index = REMATCHES_COUNT as i32;

    for &entry in entries {
        if unsafe { rematches.add(entry as usize).read() } == 0 {
            let index = unsafe { rematch_index(entry as usize) };
            if index < lowest_index {
                lowest_index = index;
            }
            candidates += 1;
        }
    }

    if candidates == 0 || lowest_index > max_rematch {
        return;
    }

    candidates = 0;
    for &entry in entries {
        if unsafe { rematches.add(entry as usize).read() } == 0
            && unsafe { rematch_index(entry as usize) } == lowest_index
        {
            candidates += 1;
        }
    }

    if candidates == 0 {
        return;
    }

    let mut chosen = i32::from(unsafe { Random() }) % candidates;
    for &entry in entries {
        if unsafe { rematches.add(entry as usize).read() } != 0
            || unsafe { rematch_index(entry as usize) } != lowest_index
        {
            continue;
        }
        if chosen == 0 {
            unsafe { rematches.add(entry as usize).write(lowest_index as u8) };
            break;
        }
        chosen -= 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateGymLeaderRematch() {
    if unsafe { FlagGet(FLAG_SYS_GAME_CLEAR) } == 0 || unsafe { Random() } % 100 > 30 {
        return;
    }

    if unsafe { FlagGet(FLAG_WATTSON_REMATCH_AVAILABLE) } != 0 {
        unsafe { update_from_array(&AFTER_NEW_MAUVILLE, 5) };
    } else {
        unsafe { update_from_array(&BEFORE_NEW_MAUVILLE, 1) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layouts_and_indices_match_the_c_tables() {
        assert_eq!(core::mem::size_of::<RematchTrainer>(), 14);
        assert_eq!(core::mem::offset_of!(RematchTrainer, map_group), 10);
        assert_eq!(
            core::mem::offset_of!(SaveBlock1RematchView, trainer_rematches),
            0x9ca
        );
        assert_eq!(AFTER_NEW_MAUVILLE, [65, 66, 67, 68, 69, 70, 71, 72]);
        assert_eq!(BEFORE_NEW_MAUVILLE, [65, 66, 68, 69, 70, 71, 72]);
    }
}
