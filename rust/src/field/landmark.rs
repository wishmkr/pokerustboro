//! Named places inside a map section ("FLOWER SHOP", "ABANDONED SHIP"...),
//! shown on the region map once their flag is set. Tables in `data/landmark.rs`.

use crate::data::landmark::sLandmarkLists;
use crate::event_data::FlagGet;

const MAPSEC_NONE: u8 = 0xd5;
const NO_FLAG: u16 = 0xffff;

/// `struct LandmarkList { u8 mapSection; u8 id; const struct Landmark *const *landmarks; }`
const LIST_SIZE: usize = 8;
const LIST_LANDMARKS: usize = 4;
/// `struct Landmark { const u8 *name; u16 flag; }`
const LANDMARK_FLAG: usize = 4;

unsafe fn list_entry(index: usize) -> *const u8 {
    unsafe { sLandmarkLists.as_ptr().cast::<u8>().add(index * LIST_SIZE) }
}

unsafe fn get_landmarks(map_section: u8, id: u8) -> *const *const u8 {
    let mut i = 0usize;
    loop {
        let section = unsafe { list_entry(i).read() };
        if section == MAPSEC_NONE {
            return core::ptr::null();
        }
        if section > map_section {
            return core::ptr::null();
        }
        if section == map_section {
            break;
        }
        i += 1;
    }
    while unsafe { list_entry(i).read() } == map_section {
        let entry = unsafe { list_entry(i) };
        if unsafe { entry.add(1).read() } == id {
            return unsafe { entry.add(LIST_LANDMARKS).cast::<*const *const u8>().read() };
        }
        i += 1;
    }
    core::ptr::null()
}

/// The `count`th landmark (among those unlocked) at this map section and id.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLandmarkName(map_section: u8, id: u8, mut count: u8) -> *const u8 {
    let mut landmarks = unsafe { get_landmarks(map_section, id) };
    if landmarks.is_null() {
        return core::ptr::null();
    }
    loop {
        let landmark = unsafe { landmarks.read() };
        let flag = unsafe { landmark.add(LANDMARK_FLAG).cast::<u16>().read() };
        if flag == NO_FLAG || unsafe { FlagGet(flag) } == 1 {
            if count == 0 {
                break;
            }
            count -= 1;
        }
        landmarks = unsafe { landmarks.add(1) };
        if unsafe { landmarks.read() }.is_null() {
            return core::ptr::null();
        }
    }
    unsafe { landmarks.read().cast::<*const u8>().read() }
}
