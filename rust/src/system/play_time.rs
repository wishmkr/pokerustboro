//! The play time counter (was src/play_time.c).
//!
//! While it runs, the VBlank handler ticks the play time kept in the save
//! (hours, minutes, seconds and frames) once a frame. It stops at 999:59:59.

use crate::global::Global;
use crate::types::SaveBlock2;

const MAX_HOURS: u16 = 999;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum CounterState {
    Stopped,
    Running,
    /// Reached 999:59:59 and won't move any more.
    MaxedOut,
}

#[unsafe(link_section = ".bss")]
static STATE: Global<CounterState> = Global::new(CounterState::Stopped);

/// Advances the play time by one frame. Returns false when that goes past
/// the maximum.
fn tick(save: &mut SaveBlock2) -> bool {
    save.playTimeVBlanks += 1;
    if save.playTimeVBlanks < 60 {
        return true;
    }
    save.playTimeVBlanks = 0;
    save.playTimeSeconds += 1;
    if save.playTimeSeconds < 60 {
        return true;
    }
    save.playTimeSeconds = 0;
    save.playTimeMinutes += 1;
    if save.playTimeMinutes < 60 {
        return true;
    }
    save.playTimeMinutes = 0;
    save.playTimeHours += 1;
    save.playTimeHours <= MAX_HOURS
}

fn set_time(save: &mut SaveBlock2, hours: u16, minutes: u8, seconds: u8, vblanks: u8) {
    save.playTimeHours = hours;
    save.playTimeMinutes = minutes;
    save.playTimeSeconds = seconds;
    save.playTimeVBlanks = vblanks;
}

/// Stops the counter and sets the time to 0:00:00.
pub fn reset(save: &mut SaveBlock2) {
    STATE.set(CounterState::Stopped);
    set_time(save, 0, 0, 0, 0);
}

/// Starts counting (or pins the time at the maximum if it's already past).
pub fn start(save: &mut SaveBlock2) {
    STATE.set(CounterState::Running);
    if save.playTimeHours > MAX_HOURS {
        set_to_max(save);
    }
}

pub fn stop() {
    STATE.set(CounterState::Stopped);
}

/// Counts one frame, if the counter is running.
pub fn update(save: &mut SaveBlock2) {
    if STATE.get() == CounterState::Running && !tick(save) {
        set_to_max(save);
    }
}

/// Sets the time to 999:59:59 for good.
pub fn set_to_max(save: &mut SaveBlock2) {
    STATE.set(CounterState::MaxedOut);
    set_time(save, MAX_HOURS, 59, 59, 59);
}

// ------------------------------------------------------------------ C names

unsafe extern "C" {}

/// # Safety
/// The save blocks must be set up and not in use elsewhere meanwhile.
unsafe fn save() -> &'static mut SaveBlock2 {
    unsafe { crate::save_blocks::save_block2() }
}

#[unsafe(no_mangle)]
pub unsafe fn PlayTimeCounter_Reset() {
    reset(unsafe { save() });
}

#[unsafe(no_mangle)]
pub unsafe fn PlayTimeCounter_Start() {
    start(unsafe { save() });
}

#[unsafe(no_mangle)]
pub fn PlayTimeCounter_Stop() {
    stop();
}

#[unsafe(no_mangle)]
pub unsafe fn PlayTimeCounter_Update() {
    update(unsafe { save() });
}

#[unsafe(no_mangle)]
pub unsafe fn PlayTimeCounter_SetToMax() {
    set_to_max(unsafe { save() });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn save_at(hours: u16, minutes: u8, seconds: u8, vblanks: u8) -> SaveBlock2 {
        // SAFETY: SaveBlock2 is plain data; all zeroes is a valid value.
        let mut save: SaveBlock2 = unsafe { core::mem::zeroed() };
        set_time(&mut save, hours, minutes, seconds, vblanks);
        save
    }

    fn time(save: &SaveBlock2) -> (u16, u8, u8, u8) {
        (
            save.playTimeHours,
            save.playTimeMinutes,
            save.playTimeSeconds,
            save.playTimeVBlanks,
        )
    }

    #[test]
    fn a_frame_carries_through_to_the_hours() {
        let mut save = save_at(12, 59, 59, 59);
        assert!(tick(&mut save));
        assert_eq!(time(&save), (13, 0, 0, 0));
    }

    #[test]
    fn the_time_stops_at_the_maximum() {
        let mut save = save_at(999, 59, 59, 59);
        STATE.set(CounterState::Running);
        update(&mut save);
        assert_eq!(time(&save), (999, 59, 59, 59));
        assert!(STATE.get() == CounterState::MaxedOut);
        // maxed out: no more counting
        update(&mut save);
        assert_eq!(time(&save), (999, 59, 59, 59));
    }
}
