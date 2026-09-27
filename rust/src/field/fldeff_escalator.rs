//! The Pokemon Center escalator. Its animation is driven by swapping
//! metatiles under the player through a three-frame cycle.

use crate::ffi::{CreateTask, DestroyTask, TaskFunc, set_task_data, task_data};

const ESCALATOR_STAGES: i16 = 3;
const LAST_ESCALATOR_STAGE: i16 = ESCALATOR_STAGES - 1;

/// Metatiles under the escalator's mid-section are impassable.
const MAPGRID_IMPASSABLE: u16 = 0x0c00;

// Task data slots, matching the original's macros.
const T_STATE: usize = 0;
const T_TRANSITION_STAGE: usize = 1;
const T_GOING_UP: usize = 2;
const T_DRAWING_ESCALATOR: usize = 3;
const T_PLAYER_X: usize = 4;
const T_PLAYER_Y: usize = 5;

// The 1F frames run backwards and the 2F frames forwards, which is what
// makes the two floors' steps appear to move in the same direction.
const ESCALATOR_1F_0: [i16; 3] = [0x284, 0x282, 0x280];
const ESCALATOR_1F_1: [i16; 3] = [0x285, 0x283, 0x281];
const ESCALATOR_1F_2: [i16; 3] = [0x28c, 0x28a, 0x288];
const ESCALATOR_1F_3: [i16; 3] = [0x28d, 0x28b, 0x289];
const ESCALATOR_2F_0: [i16; 3] = [0x2a0, 0x2a2, 0x2a4];
const ESCALATOR_2F_1: [i16; 3] = [0x2a1, 0x2a3, 0x2a5];
const ESCALATOR_2F_2: [i16; 3] = [0x2a8, 0x2aa, 0x2ac];

#[unsafe(link_section = "ewram_data")]
static mut ESCALATOR_ANIM_TASK_ID: u8 = 0;

unsafe extern "C" {
    fn MapGridGetMetatileIdAt(x: i32, y: i32) -> i32;
    fn MapGridSetMetatileIdAt(x: i32, y: i32, metatile: u16);
    fn DrawWholeMapView();
    fn PlayerGetDestCoords(x: *mut i16, y: *mut i16);
}

/// Advances whichever of the 3x3 tiles around the player currently shows
/// this section's frame, leaving the others alone.
unsafe fn set_escalator_metatile(task_id: u8, metatile_ids: &[i16; 3], metatile_masks: u16) {
    let x = unsafe { task_data(task_id, T_PLAYER_X) } - 1;
    let y = unsafe { task_data(task_id, T_PLAYER_Y) } - 1;
    let stage = unsafe { task_data(task_id, T_TRANSITION_STAGE) };
    let going_up = unsafe { task_data(task_id, T_GOING_UP) } != 0;

    // Going up walks the frame list in reverse. The stage is always 0..=2;
    // reducing it modulo 3 lets the compiler prove the indices in bounds so no
    // panic path reaches the ROM.
    let frame = |i: i16| metatile_ids[(i as usize) % 3];
    let (looked_for, next) = if going_up {
        let next = if stage != LAST_ESCALATOR_STAGE {
            frame(1 - stage)
        } else {
            frame(LAST_ESCALATOR_STAGE)
        };
        (frame(LAST_ESCALATOR_STAGE - stage), next)
    } else {
        let next = if stage != LAST_ESCALATOR_STAGE {
            frame(stage + 1)
        } else {
            frame(0)
        };
        (frame(stage), next)
    };

    for i in 0..3i32 {
        for j in 0..3i32 {
            let tile_x = i32::from(x) + j;
            let tile_y = i32::from(y) + i;
            if unsafe { MapGridGetMetatileIdAt(tile_x, tile_y) } as i16 == looked_for {
                unsafe { MapGridSetMetatileIdAt(tile_x, tile_y, metatile_masks | next as u16) };
            }
        }
    }
}

unsafe extern "C" fn task_draw_escalator(task_id: u8) {
    unsafe { set_task_data(task_id, T_DRAWING_ESCALATOR, 1) };

    // One escalator section per frame, in sequence.
    match unsafe { task_data(task_id, T_STATE) } {
        0 => unsafe { set_escalator_metatile(task_id, &ESCALATOR_1F_0, 0) },
        1 => unsafe { set_escalator_metatile(task_id, &ESCALATOR_1F_1, 0) },
        2 => unsafe { set_escalator_metatile(task_id, &ESCALATOR_1F_2, MAPGRID_IMPASSABLE) },
        3 => unsafe { set_escalator_metatile(task_id, &ESCALATOR_1F_3, 0) },
        4 => unsafe { set_escalator_metatile(task_id, &ESCALATOR_2F_0, MAPGRID_IMPASSABLE) },
        5 => unsafe { set_escalator_metatile(task_id, &ESCALATOR_2F_1, 0) },
        6 => unsafe { set_escalator_metatile(task_id, &ESCALATOR_2F_2, 0) },
        _ => {}
    }

    // Eight states for seven sections, so there is one idle frame per cycle.
    let state = (unsafe { task_data(task_id, T_STATE) } + 1) & 7;
    unsafe { set_task_data(task_id, T_STATE, state) };

    if state == 0 {
        unsafe { DrawWholeMapView() };
        let stage = (unsafe { task_data(task_id, T_TRANSITION_STAGE) } + 1) % ESCALATOR_STAGES;
        unsafe { set_task_data(task_id, T_TRANSITION_STAGE, stage) };
        unsafe { set_task_data(task_id, T_DRAWING_ESCALATOR, 0) };
    }
}

unsafe fn create_escalator_task(going_up: u16) -> u8 {
    let task_id = unsafe { CreateTask(task_draw_escalator, 0) };

    let mut x = 0i16;
    let mut y = 0i16;
    unsafe { PlayerGetDestCoords(&raw mut x, &raw mut y) };
    unsafe { set_task_data(task_id, T_PLAYER_X, x) };
    unsafe { set_task_data(task_id, T_PLAYER_Y, y) };
    unsafe { set_task_data(task_id, T_STATE, 0) };
    unsafe { set_task_data(task_id, T_TRANSITION_STAGE, 0) };
    unsafe { set_task_data(task_id, T_GOING_UP, going_up as i16) };

    unsafe { task_draw_escalator(task_id) };
    task_id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartEscalator(going_up: u8) {
    let task_id = unsafe { create_escalator_task(u16::from(going_up)) };
    unsafe { (&raw mut ESCALATOR_ANIM_TASK_ID).write_volatile(task_id) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopEscalator() {
    let task_id = unsafe { (&raw const ESCALATOR_ANIM_TASK_ID).read_volatile() };
    unsafe { DestroyTask(task_id) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsEscalatorMoving() -> u8 {
    let task_id = unsafe { (&raw const ESCALATOR_ANIM_TASK_ID).read_volatile() };
    // Only stopped once a full cycle has finished on the last stage.
    let finished = unsafe { task_data(task_id, T_DRAWING_ESCALATOR) } == 0
        && unsafe { task_data(task_id, T_TRANSITION_STAGE) } == LAST_ESCALATOR_STAGE;
    u8::from(!finished)
}

const _: TaskFunc = task_draw_escalator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_section_cycles_through_three_frames() {
        for frames in [
            ESCALATOR_1F_0,
            ESCALATOR_1F_1,
            ESCALATOR_1F_2,
            ESCALATOR_1F_3,
            ESCALATOR_2F_0,
            ESCALATOR_2F_1,
            ESCALATOR_2F_2,
        ] {
            assert_eq!(frames.len(), ESCALATOR_STAGES as usize);
            assert_ne!(frames[0], frames[1]);
            assert_ne!(frames[1], frames[2]);
        }
    }

    #[test]
    fn the_two_floors_run_their_frames_in_opposite_order() {
        // 1F counts down, 2F counts up, which is what makes both sets of
        // steps appear to travel the same way.
        assert!(ESCALATOR_1F_0[0] > ESCALATOR_1F_0[2]);
        assert!(ESCALATOR_2F_0[0] < ESCALATOR_2F_0[2]);
    }

    #[test]
    fn the_state_counter_wraps_after_eight_frames() {
        let step = |state: i16| (state + 1) & 7;
        assert_eq!(step(6), 7);
        assert_eq!(step(7), 0);
    }
}
