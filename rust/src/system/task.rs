//! The task scheduler: a fixed pool of 16 slots threaded into a
//! priority-ordered intrusive linked list.

use crate::ffi::{
    HEAD_SENTINEL, NUM_TASK_DATA, NUM_TASKS, TAIL_SENTINEL, TASK_FUNC_OFFSET,
    TASK_IS_ACTIVE_OFFSET, TASK_NEXT_OFFSET, TASK_PREV_OFFSET, TASK_PRIORITY_OFFSET, TASK_SIZE,
    TaskArm, TaskFunc, set_task_data, task, task_data,
};

/// `COMMON_DATA struct Task gTasks[NUM_TASKS]`
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gTasks: [TaskArm; NUM_TASKS] = [const { TaskArm([0; TASK_SIZE]) }; NUM_TASKS];

/// Followup functions are stashed in the last two data slots.
const FOLLOWUP_FUNC_INDEX: usize = NUM_TASK_DATA - 2;

#[inline]
unsafe fn is_active(task_id: u8) -> bool {
    let flag = unsafe { task(task_id).add(TASK_IS_ACTIVE_OFFSET).read_volatile() };
    flag != 0
}

#[inline]
unsafe fn set_is_active(task_id: u8, active: bool) {
    unsafe {
        task(task_id)
            .add(TASK_IS_ACTIVE_OFFSET)
            .write_volatile(u8::from(active))
    };
}

#[inline]
unsafe fn prev(task_id: u8) -> u8 {
    unsafe { task(task_id).add(TASK_PREV_OFFSET).read_volatile() }
}

#[inline]
unsafe fn set_prev(task_id: u8, value: u8) {
    unsafe { task(task_id).add(TASK_PREV_OFFSET).write_volatile(value) };
}

#[inline]
unsafe fn next(task_id: u8) -> u8 {
    unsafe { task(task_id).add(TASK_NEXT_OFFSET).read_volatile() }
}

#[inline]
unsafe fn set_next(task_id: u8, value: u8) {
    unsafe { task(task_id).add(TASK_NEXT_OFFSET).write_volatile(value) };
}

#[inline]
unsafe fn priority(task_id: u8) -> u8 {
    unsafe { task(task_id).add(TASK_PRIORITY_OFFSET).read_volatile() }
}

#[inline]
unsafe fn set_priority(task_id: u8, value: u8) {
    unsafe {
        task(task_id)
            .add(TASK_PRIORITY_OFFSET)
            .write_volatile(value)
    };
}

/// Compares two task functions the way the C does: by address. Rust's
/// `fn` equality is lint-flagged because codegen may merge or duplicate
/// functions, but on this target the scheduler genuinely identifies tasks by
/// the pointer it was handed.
#[inline]
unsafe fn func_is(task_id: u8, function: TaskFunc) -> bool {
    unsafe { func(task_id) }.is_some_and(|stored| stored as usize == function as usize)
}

#[inline]
unsafe fn func(task_id: u8) -> Option<TaskFunc> {
    unsafe {
        task(task_id)
            .add(TASK_FUNC_OFFSET)
            .cast::<Option<TaskFunc>>()
            .read()
    }
}

#[inline]
unsafe fn set_func(task_id: u8, value: Option<TaskFunc>) {
    unsafe {
        task(task_id)
            .add(TASK_FUNC_OFFSET)
            .cast::<Option<TaskFunc>>()
            .write(value)
    };
}

#[inline]
unsafe fn clear_data(task_id: u8) {
    let mut index = 0usize;
    while index < NUM_TASK_DATA {
        unsafe { set_task_data(task_id, index, 0) };
        index += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetTasks() {
    let mut i = 0u8;
    while (i as usize) < NUM_TASKS {
        unsafe { set_is_active(i, false) };
        unsafe { set_func(i, Some(TaskDummy)) };
        unsafe { set_prev(i, i) };
        unsafe { set_next(i, i.wrapping_add(1)) };
        // `priority = -1` on a u8 field.
        unsafe { set_priority(i, 0xff) };
        unsafe { clear_data(i) };
        i += 1;
    }

    unsafe { set_prev(0, HEAD_SENTINEL) };
    unsafe { set_next(NUM_TASKS as u8 - 1, TAIL_SENTINEL) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask(function: TaskFunc, task_priority: u8) -> u8 {
    let mut i = 0u8;
    while (i as usize) < NUM_TASKS {
        if !unsafe { is_active(i) } {
            unsafe { set_func(i, Some(function)) };
            unsafe { set_priority(i, task_priority) };
            unsafe { insert_task(i) };
            unsafe { clear_data(i) };
            unsafe { set_is_active(i, true) };
            return i;
        }
        i += 1;
    }

    // The pool is full; the caller silently reuses slot 0's id.
    0
}

unsafe fn insert_task(new_task_id: u8) {
    let mut task_id = unsafe { find_first_active_task() };

    if task_id as usize == NUM_TASKS {
        // The new task is the only task.
        unsafe { set_prev(new_task_id, HEAD_SENTINEL) };
        unsafe { set_next(new_task_id, TAIL_SENTINEL) };
        return;
    }

    loop {
        if unsafe { priority(new_task_id) } < unsafe { priority(task_id) } {
            // A task with a higher priority value: insert before it.
            unsafe { set_prev(new_task_id, prev(task_id)) };
            unsafe { set_next(new_task_id, task_id) };
            if unsafe { prev(task_id) } != HEAD_SENTINEL {
                unsafe { set_next(prev(task_id), new_task_id) };
            }
            unsafe { set_prev(task_id, new_task_id) };
            return;
        }

        if unsafe { next(task_id) } == TAIL_SENTINEL {
            // The end of the list.
            unsafe { set_prev(new_task_id, task_id) };
            unsafe { set_next(new_task_id, next(task_id)) };
            unsafe { set_next(task_id, new_task_id) };
            return;
        }

        task_id = unsafe { next(task_id) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyTask(task_id: u8) {
    if !unsafe { is_active(task_id) } {
        return;
    }
    unsafe { set_is_active(task_id, false) };

    if unsafe { prev(task_id) } == HEAD_SENTINEL {
        if unsafe { next(task_id) } != TAIL_SENTINEL {
            unsafe { set_prev(next(task_id), HEAD_SENTINEL) };
        }
    } else if unsafe { next(task_id) } == TAIL_SENTINEL {
        unsafe { set_next(prev(task_id), TAIL_SENTINEL) };
    } else {
        unsafe { set_next(prev(task_id), next(task_id)) };
        unsafe { set_prev(next(task_id), prev(task_id)) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunTasks() {
    let mut task_id = unsafe { find_first_active_task() };
    if task_id as usize == NUM_TASKS {
        return;
    }

    loop {
        if let Some(function) = unsafe { func(task_id) } {
            unsafe { function(task_id) };
        }
        task_id = unsafe { next(task_id) };
        if task_id == TAIL_SENTINEL {
            return;
        }
    }
}

unsafe fn find_first_active_task() -> u8 {
    let mut task_id = 0u8;
    while (task_id as usize) < NUM_TASKS {
        if unsafe { is_active(task_id) } && unsafe { prev(task_id) } == HEAD_SENTINEL {
            break;
        }
        task_id += 1;
    }
    task_id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TaskDummy(_task_id: u8) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTaskFuncWithFollowupFunc(
    task_id: u8,
    function: TaskFunc,
    followup_func: TaskFunc,
) {
    // The followup is stored low half first, the opposite order to the
    // field-move callbacks in data[8]/data[9].
    let address = followup_func as usize as u32;
    unsafe { set_task_data(task_id, FOLLOWUP_FUNC_INDEX, address as i16) };
    unsafe { set_task_data(task_id, FOLLOWUP_FUNC_INDEX + 1, (address >> 16) as i16) };
    unsafe { set_func(task_id, Some(function)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwitchTaskToFollowupFunc(task_id: u8) {
    let low = unsafe { task_data(task_id, FOLLOWUP_FUNC_INDEX) } as u16;
    // The high half is sign-extended before shifting in the original, which
    // is harmless for a ROM address but is preserved anyway.
    let high = i32::from(unsafe { task_data(task_id, FOLLOWUP_FUNC_INDEX + 1) }) << 16;
    let address = (i32::from(low) | high) as u32;
    unsafe {
        set_func(
            task_id,
            core::mem::transmute::<usize, Option<TaskFunc>>(address as usize),
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FuncIsActiveTask(function: TaskFunc) -> u8 {
    let mut i = 0u8;
    while (i as usize) < NUM_TASKS {
        if unsafe { is_active(i) } && unsafe { func_is(i, function) } {
            return 1;
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FindTaskIdByFunc(function: TaskFunc) -> u8 {
    let mut i = 0u8;
    while (i as usize) < NUM_TASKS {
        if unsafe { is_active(i) } && unsafe { func_is(i, function) } {
            return i;
        }
        i += 1;
    }
    // TASK_NONE
    TAIL_SENTINEL
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTaskCount() -> u8 {
    let mut count = 0u8;
    let mut i = 0u8;
    while (i as usize) < NUM_TASKS {
        if unsafe { is_active(i) } {
            count += 1;
        }
        i += 1;
    }
    count
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWordTaskArg(task_id: u8, data_elem: u8, value: u32) {
    if (data_elem as usize) < NUM_TASK_DATA - 1 {
        unsafe { set_task_data(task_id, data_elem as usize, value as i16) };
        unsafe { set_task_data(task_id, data_elem as usize + 1, (value >> 16) as i16) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWordTaskArg(task_id: u8, data_elem: u8) -> u32 {
    if (data_elem as usize) < NUM_TASK_DATA - 1 {
        let low = unsafe { task_data(task_id, data_elem as usize) } as u16;
        let high = i32::from(unsafe { task_data(task_id, data_elem as usize + 1) }) << 16;
        (i32::from(low) | high) as u32
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_task_pool_is_exactly_the_common_data_block() {
        assert_eq!(core::mem::size_of::<TaskArm>(), 40);
        assert_eq!(TASK_SIZE * NUM_TASKS, 640);
        assert_eq!(FOLLOWUP_FUNC_INDEX, 14);
    }

    #[test]
    fn sentinels_cannot_collide_with_a_real_task_id() {
        assert!(HEAD_SENTINEL as usize >= NUM_TASKS);
        assert!(TAIL_SENTINEL as usize >= NUM_TASKS);
        assert_ne!(HEAD_SENTINEL, TAIL_SENTINEL);
    }

    #[test]
    fn word_args_round_trip_through_two_signed_halves() {
        for value in [0u32, 1, 0x0000_ffff, 0x8000_0000, 0x0812_3456, u32::MAX] {
            let low = value as i16;
            let high = (value >> 16) as i16;
            let rebuilt = (i32::from(low as u16) | (i32::from(high) << 16)) as u32;
            assert_eq!(rebuilt, value);
        }
    }
}
