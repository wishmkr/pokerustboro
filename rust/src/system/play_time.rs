use core::ptr::{addr_of, addr_of_mut};

const STOPPED: u8 = 0;
const RUNNING: u8 = 1;
const MAXED_OUT: u8 = 2;

#[repr(C)]
struct SaveBlock2TimePrefix {
    player_name: [u8; 8],
    player_gender: u8,
    special_save_warp_flags: u8,
    player_trainer_id: [u8; 4],
    play_time_hours: u16,
    play_time_minutes: u8,
    play_time_seconds: u8,
    play_time_vblanks: u8,
}

unsafe extern "C" {
    static mut gSaveBlock2Ptr: *mut SaveBlock2TimePrefix;
}

#[unsafe(link_section = ".bss")]
static mut PLAY_TIME_COUNTER_STATE: u8 = STOPPED;

unsafe fn set_to_max(save: *mut SaveBlock2TimePrefix) {
    unsafe { addr_of_mut!((*save).play_time_hours).write(999) };
    unsafe { addr_of_mut!((*save).play_time_minutes).write(59) };
    unsafe { addr_of_mut!((*save).play_time_seconds).write(59) };
    unsafe { addr_of_mut!((*save).play_time_vblanks).write(59) };
}

unsafe fn update_counter(save: *mut SaveBlock2TimePrefix) {
    let vblanks = unsafe { addr_of!((*save).play_time_vblanks).read() }.wrapping_add(1);
    unsafe { addr_of_mut!((*save).play_time_vblanks).write(vblanks) };
    if vblanks < 60 {
        return;
    }

    unsafe { addr_of_mut!((*save).play_time_vblanks).write(0) };
    let seconds = unsafe { addr_of!((*save).play_time_seconds).read() }.wrapping_add(1);
    unsafe { addr_of_mut!((*save).play_time_seconds).write(seconds) };
    if seconds < 60 {
        return;
    }

    unsafe { addr_of_mut!((*save).play_time_seconds).write(0) };
    let minutes = unsafe { addr_of!((*save).play_time_minutes).read() }.wrapping_add(1);
    unsafe { addr_of_mut!((*save).play_time_minutes).write(minutes) };
    if minutes < 60 {
        return;
    }

    unsafe { addr_of_mut!((*save).play_time_minutes).write(0) };
    let hours = unsafe { addr_of!((*save).play_time_hours).read() }.wrapping_add(1);
    unsafe { addr_of_mut!((*save).play_time_hours).write(hours) };
    if hours > 999 {
        unsafe { (&raw mut PLAY_TIME_COUNTER_STATE).write(MAXED_OUT) };
        unsafe { set_to_max(save) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayTimeCounter_Reset() {
    unsafe { (&raw mut PLAY_TIME_COUNTER_STATE).write(STOPPED) };
    let save = unsafe { gSaveBlock2Ptr };
    unsafe { addr_of_mut!((*save).play_time_hours).write(0) };
    unsafe { addr_of_mut!((*save).play_time_minutes).write(0) };
    unsafe { addr_of_mut!((*save).play_time_seconds).write(0) };
    unsafe { addr_of_mut!((*save).play_time_vblanks).write(0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayTimeCounter_Start() {
    unsafe { (&raw mut PLAY_TIME_COUNTER_STATE).write(RUNNING) };
    let save = unsafe { gSaveBlock2Ptr };
    if unsafe { addr_of!((*save).play_time_hours).read() } > 999 {
        unsafe { (&raw mut PLAY_TIME_COUNTER_STATE).write(MAXED_OUT) };
        unsafe { set_to_max(save) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayTimeCounter_Stop() {
    unsafe { (&raw mut PLAY_TIME_COUNTER_STATE).write(STOPPED) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayTimeCounter_Update() {
    if unsafe { (&raw const PLAY_TIME_COUNTER_STATE).read() } != RUNNING {
        return;
    }
    unsafe { update_counter(gSaveBlock2Ptr) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayTimeCounter_SetToMax() {
    unsafe { (&raw mut PLAY_TIME_COUNTER_STATE).write(MAXED_OUT) };
    unsafe { set_to_max(gSaveBlock2Ptr) };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn save(hours: u16, minutes: u8, seconds: u8, vblanks: u8) -> SaveBlock2TimePrefix {
        SaveBlock2TimePrefix {
            player_name: [0; 8],
            player_gender: 0,
            special_save_warp_flags: 0,
            player_trainer_id: [0; 4],
            play_time_hours: hours,
            play_time_minutes: minutes,
            play_time_seconds: seconds,
            play_time_vblanks: vblanks,
        }
    }

    #[test]
    fn save_prefix_offsets_match_the_c_structure() {
        assert_eq!(
            core::mem::offset_of!(SaveBlock2TimePrefix, play_time_hours),
            0x0e
        );
        assert_eq!(
            core::mem::offset_of!(SaveBlock2TimePrefix, play_time_minutes),
            0x10
        );
        assert_eq!(
            core::mem::offset_of!(SaveBlock2TimePrefix, play_time_seconds),
            0x11
        );
        assert_eq!(
            core::mem::offset_of!(SaveBlock2TimePrefix, play_time_vblanks),
            0x12
        );
    }

    #[test]
    fn update_carries_frames_through_hours() {
        let mut value = save(12, 59, 59, 59);
        unsafe { update_counter(&raw mut value) };
        assert_eq!(value.play_time_hours, 13);
        assert_eq!(value.play_time_minutes, 0);
        assert_eq!(value.play_time_seconds, 0);
        assert_eq!(value.play_time_vblanks, 0);
    }

    #[test]
    fn maximum_time_is_saturated() {
        let mut value = save(999, 59, 59, 59);
        unsafe { update_counter(&raw mut value) };
        assert_eq!(value.play_time_hours, 999);
        assert_eq!(value.play_time_minutes, 59);
        assert_eq!(value.play_time_seconds, 59);
        assert_eq!(value.play_time_vblanks, 59);
    }
}
