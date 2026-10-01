//! The task scheduler (was src/task.c).
//!
//! A task is a function called once a frame with its id, plus sixteen
//! `i16`s of data it keeps between frames. There are 16 slots; active tasks
//! form a linked list ordered by priority (lower runs first), threaded
//! through `prev`/`next` with [`HEAD_SENTINEL`] and [`TAIL_SENTINEL`] at its
//! ends.
//!
//! Task functions reach `gTasks` themselves while the scheduler runs them
//! (to read their data, destroy themselves, start other tasks), so the
//! scheduler never holds a reference to it across a call.

use crate::c::{CArray, CIndex};
use crate::ffi::TaskFunc;
use crate::global::Global;
use crate::types::Task;

pub const NUM_TASKS: usize = 16;
pub const NUM_TASK_DATA: usize = 16;
pub const HEAD_SENTINEL: u8 = 0xfe;
pub const TAIL_SENTINEL: u8 = 0xff;
/// What [`find_task_id_by_func`]'s C name returns for "none".
pub const TASK_NONE: u8 = TAIL_SENTINEL;

/// Where `SetTaskFuncWithFollowupFunc` keeps the followup function: the
/// last two data slots.
const FOLLOWUP_FUNC_INDEX: usize = NUM_TASK_DATA - 2;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static gTasks: Global<CArray<Task, NUM_TASKS>> = Global::new(unsafe { core::mem::zeroed() });

/// The task slots.
///
/// # Safety
/// No other reference to them may be in use while the result lives: keep
/// it within one step, not across a call to a task function.
unsafe fn tasks<'a>() -> &'a mut CArray<Task, NUM_TASKS> {
    // SAFETY: a static; exclusivity is the caller's promise.
    unsafe { &mut *gTasks.as_ptr() }
}

// Single reads and writes of a task's fields, for code that keeps no
// reference to the tasks. Like C (and CArray) they don't check the indices.

/// Task `id`'s data slot `slot`.
#[inline(always)]
pub fn task_get(id: impl CIndex, slot: impl CIndex) -> i16 {
    // SAFETY: a plain read; no reference to the tasks is kept.
    unsafe { tasks()[id].data[slot] }
}

/// Sets task `id`'s data slot `slot`.
#[inline(always)]
pub fn task_set(id: impl CIndex, slot: impl CIndex, value: i16) {
    // SAFETY: as in task_get.
    unsafe { tasks()[id].data[slot] = value }
}

/// Task `id`'s function.
#[inline(always)]
pub fn task_func(id: impl CIndex) -> Option<TaskFunc> {
    // SAFETY: as in task_get.
    unsafe { tasks()[id].func }
}

/// Makes task `id` run `func`.
#[inline(always)]
pub fn task_set_func(id: impl CIndex, func: Option<TaskFunc>) {
    // SAFETY: as in task_get.
    unsafe { tasks()[id].func = func }
}

/// A pointer to task `id`'s data slot `slot`, for code that works through
/// pointers.
#[inline(always)]
pub fn task_data_ptr(id: impl CIndex, slot: impl CIndex) -> *mut i16 {
    // SAFETY: only an address is computed.
    unsafe { &raw mut tasks()[id].data[slot] }
}

/// Task `id`'s data, for the task's own code.
///
/// # Safety
/// As for [`tasks`]: the result must not live across a call that may touch
/// the tasks (another task's code, [`create_task`]...).
pub unsafe fn task_data<'a>(id: u8) -> &'a mut CArray<i16, NUM_TASK_DATA> {
    unsafe { &mut tasks()[id].data }
}

fn is_same_func(a: Option<TaskFunc>, b: TaskFunc) -> bool {
    a.is_some_and(|a| a as usize == b as usize)
}

/// Empties every slot.
pub fn reset_tasks() {
    // SAFETY: the scheduler isn't running any task now.
    let tasks = unsafe { tasks() };
    for (i, task) in tasks.0.iter_mut().enumerate() {
        *task = Task {
            func: Some(TaskDummy),
            isActive: 0,
            prev: i as u8,
            next: i as u8 + 1,
            priority: u8::MAX,
            data: CArray([0; NUM_TASK_DATA]),
        };
    }
    tasks[0].prev = HEAD_SENTINEL;
    tasks[NUM_TASKS - 1].next = TAIL_SENTINEL;
}

/// Starts `func` in the first free slot and returns its id. When all 16 are
/// taken it creates nothing and returns 0, as C does.
pub fn create_task(func: TaskFunc, priority: u8) -> u8 {
    // SAFETY: the borrow ends before any task code runs.
    let tasks = unsafe { tasks() };
    let Some(id) = tasks.0.iter().position(|t| t.isActive == 0) else {
        return 0;
    };
    let id = id as u8;
    tasks[id].func = Some(func);
    tasks[id].priority = priority;
    insert_task(tasks, id);
    tasks[id].data = CArray([0; NUM_TASK_DATA]);
    tasks[id].isActive = 1;
    id
}

/// Links task `new` into the list, before the first task with a higher
/// priority value.
fn insert_task(tasks: &mut CArray<Task, NUM_TASKS>, new: u8) {
    let mut id = first_active_task(tasks);
    if id == NUM_TASKS as u8 {
        // the only task
        tasks[new].prev = HEAD_SENTINEL;
        tasks[new].next = TAIL_SENTINEL;
        return;
    }
    loop {
        if tasks[new].priority < tasks[id].priority {
            let prev = tasks[id].prev;
            tasks[new].prev = prev;
            tasks[new].next = id;
            if prev != HEAD_SENTINEL {
                tasks[prev].next = new;
            }
            tasks[id].prev = new;
            return;
        }
        if tasks[id].next == TAIL_SENTINEL {
            tasks[new].prev = id;
            tasks[new].next = TAIL_SENTINEL;
            tasks[id].next = new;
            return;
        }
        id = tasks[id].next;
    }
}

/// Stops task `id` and unlinks it (its data stays).
pub fn destroy_task(id: u8) {
    // SAFETY: the borrow ends here.
    let tasks = unsafe { tasks() };
    if tasks[id].isActive == 0 {
        return;
    }
    tasks[id].isActive = 0;
    let (prev, next) = (tasks[id].prev, tasks[id].next);
    if prev == HEAD_SENTINEL {
        if next != TAIL_SENTINEL {
            tasks[next].prev = HEAD_SENTINEL;
        }
    } else if next == TAIL_SENTINEL {
        tasks[prev].next = TAIL_SENTINEL;
    } else {
        tasks[prev].next = next;
        tasks[next].prev = prev;
    }
}

/// Runs every active task once, in list order.
pub fn run_tasks() {
    // SAFETY: each borrow of the tasks ends before the task function runs.
    let mut id = first_active_task(unsafe { tasks() });
    if id == NUM_TASKS as u8 {
        return;
    }
    loop {
        let func = unsafe { tasks() }[id].func;
        if let Some(func) = func {
            // SAFETY: task functions take their own id.
            unsafe { func(id) };
        }
        id = unsafe { tasks() }[id].next;
        if id == TAIL_SENTINEL {
            break;
        }
    }
}

/// The head of the list, or NUM_TASKS if no task is active.
fn first_active_task(tasks: &CArray<Task, NUM_TASKS>) -> u8 {
    tasks
        .0
        .iter()
        .position(|t| t.isActive == 1 && t.prev == HEAD_SENTINEL)
        .unwrap_or(NUM_TASKS) as u8
}

/// Makes task `id` run `func`, and `followup` once it calls
/// [`switch_task_to_followup_func`].
pub fn set_task_func_with_followup_func(id: u8, func: TaskFunc, followup: TaskFunc) {
    // SAFETY: the borrow ends here.
    let task = &mut unsafe { tasks() }[id];
    let address = followup as usize as u32;
    task.data[FOLLOWUP_FUNC_INDEX] = address as i16;
    task.data[FOLLOWUP_FUNC_INDEX + 1] = (address >> 16) as i16;
    task.func = Some(func);
}

pub fn switch_task_to_followup_func(id: u8) {
    // SAFETY: the borrow ends here.
    let task = &mut unsafe { tasks() }[id];
    let address = u32::from(task.data[FOLLOWUP_FUNC_INDEX] as u16)
        | (task.data[FOLLOWUP_FUNC_INDEX + 1] as u32) << 16;
    // SAFETY: the address is the one set_task_func_with_followup_func stored
    // (or 0, None); fn pointers and addresses have the same size here.
    task.func = unsafe { core::mem::transmute::<usize, Option<TaskFunc>>(address as usize) };
}

/// The id of an active task running `func`, if any.
pub fn find_task_id_by_func(func: TaskFunc) -> Option<u8> {
    // SAFETY: the borrow ends here.
    let tasks = unsafe { tasks() };
    tasks
        .0
        .iter()
        .position(|t| t.isActive == 1 && is_same_func(t.func, func))
        .map(|i| i as u8)
}

pub fn task_count() -> u8 {
    // SAFETY: the borrow ends here.
    unsafe { tasks() }
        .0
        .iter()
        .filter(|t| t.isActive == 1)
        .count() as u8
}

/// Stores a 32-bit value in data slots `elem` and `elem + 1`.
pub fn set_word_task_arg(id: u8, elem: u8, value: u32) {
    if usize::from(elem) < NUM_TASK_DATA - 1 {
        // SAFETY: the borrow ends here.
        let data = unsafe { task_data(id) };
        data[elem] = value as i16;
        data[elem + 1] = (value >> 16) as i16;
    }
}

/// The 32-bit value in data slots `elem` and `elem + 1` (0 for the last slot).
pub fn get_word_task_arg(id: u8, elem: u8) -> u32 {
    if usize::from(elem) < NUM_TASK_DATA - 1 {
        // SAFETY: the borrow ends here.
        let data = unsafe { task_data(id) };
        u32::from(data[elem] as u16) | (data[elem + 1] as u32) << 16
    } else {
        0
    }
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn ResetTasks() {
    reset_tasks();
}

#[unsafe(no_mangle)]
pub fn CreateTask(func: TaskFunc, priority: u8) -> u8 {
    create_task(func, priority)
}

#[unsafe(no_mangle)]
pub fn DestroyTask(id: u8) {
    destroy_task(id);
}

#[unsafe(no_mangle)]
pub fn RunTasks() {
    run_tasks();
}

#[unsafe(no_mangle)]
pub fn TaskDummy(_id: u8) {}

#[unsafe(no_mangle)]
pub fn SetTaskFuncWithFollowupFunc(id: u8, func: TaskFunc, followup: TaskFunc) {
    set_task_func_with_followup_func(id, func, followup);
}

#[unsafe(no_mangle)]
pub fn SwitchTaskToFollowupFunc(id: u8) {
    switch_task_to_followup_func(id);
}

#[unsafe(no_mangle)]
pub fn FuncIsActiveTask(func: TaskFunc) -> u8 {
    find_task_id_by_func(func).is_some().into()
}

#[unsafe(no_mangle)]
pub fn FindTaskIdByFunc(func: TaskFunc) -> u8 {
    find_task_id_by_func(func).unwrap_or(TASK_NONE)
}

#[unsafe(no_mangle)]
pub fn GetTaskCount() -> u8 {
    task_count()
}

#[unsafe(no_mangle)]
pub fn SetWordTaskArg(id: u8, elem: u8, value: u32) {
    set_word_task_arg(id, elem, value);
}

#[unsafe(no_mangle)]
pub fn GetWordTaskArg(id: u8, elem: u8) -> u32 {
    get_word_task_arg(id, elem)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task_a(_: u8) {}
    fn task_b(_: u8) {}

    fn order() -> [u8; 4] {
        let tasks = unsafe { tasks() };
        let mut out = [TAIL_SENTINEL; 4];
        let mut id = first_active_task(tasks);
        let mut i = 0;
        while id != TAIL_SENTINEL && i < 4 {
            out[i] = id;
            id = tasks[id].next;
            i += 1;
        }
        out
    }

    #[test]
    fn the_list_is_kept_in_priority_order() {
        reset_tasks();
        let late = create_task(task_a, 50);
        let early = create_task(task_b, 10);
        let middle = create_task(task_a, 20);
        assert_eq!(order(), [early, middle, late, TAIL_SENTINEL]);
        assert_eq!(task_count(), 3);
        assert_eq!(find_task_id_by_func(task_b), Some(early));
        destroy_task(middle);
        assert_eq!(order(), [early, late, TAIL_SENTINEL, TAIL_SENTINEL]);
        // a freed slot is reused first
        assert_eq!(create_task(task_b, 99), middle);
        word_args_round_trip_through_two_signed_halves();
    }

    // (one test: the tests run in parallel and share gTasks)
    fn word_args_round_trip_through_two_signed_halves() {
        reset_tasks();
        let id = create_task(task_a, 0);
        for value in [0u32, 1, 0x0000_ffff, 0x8000_0000, 0x0812_3456, u32::MAX] {
            set_word_task_arg(id, 3, value);
            assert_eq!(get_word_task_arg(id, 3), value);
        }
        assert_eq!(get_word_task_arg(id, 15), 0);
    }
}
