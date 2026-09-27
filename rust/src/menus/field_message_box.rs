use crate::ffi::{StringExpandPlaceholders, gStringVar4};
use core::ptr::{addr_of, addr_of_mut};

const FIELD_MESSAGE_BOX_HIDDEN: u8 = 0;
const FIELD_MESSAGE_BOX_NORMAL: u8 = 2;
const FIELD_MESSAGE_BOX_AUTO_SCROLL: u8 = 3;
const TASK_NONE: u8 = 0xff;
const TASK_SIZE: usize = 40;
const TASK_DATA_OFFSET: usize = 8;

type TaskFunc = unsafe extern "C" fn(u8);

#[unsafe(link_section = "ewram_data")]
static mut FIELD_MESSAGE_BOX_MODE: u8 = FIELD_MESSAGE_BOX_HIDDEN;

unsafe extern "C" {
    static mut gTextFlags: u8;
    static mut gTasks: u8;

    fn LoadMessageBoxAndBorderGfx();
    fn DrawDialogueFrame(window_id: u8, copy_to_vram: u8);
    fn RunTextPrintersAndIsPrinter0Active() -> u16;
    fn CreateTask(function: TaskFunc, priority: u8) -> u8;
    fn DestroyTask(task_id: u8);
    fn FindTaskIdByFunc(function: TaskFunc) -> u8;
    fn IsMatchCallTaskActive() -> u32;
    fn StartMatchCallFromScript(message: *const u8);
    fn AddTextPrinterForMessage(allow_skipping_delay_with_button_press: u8);
    fn ClearDialogWindowAndFrame(window_id: u8, copy_to_vram: u8);
}

unsafe fn task_state(task_id: u8) -> *mut i16 {
    unsafe {
        (&raw mut gTasks)
            .add(task_id as usize * TASK_SIZE + TASK_DATA_OFFSET)
            .cast()
    }
}

unsafe extern "C" fn task_draw_field_message(task_id: u8) {
    let state = unsafe { task_state(task_id) };
    match unsafe { state.read() } {
        0 => {
            unsafe { LoadMessageBoxAndBorderGfx() };
            unsafe { state.write(1) };
        }
        1 => {
            unsafe { DrawDialogueFrame(0, 1) };
            unsafe { state.write(2) };
        }
        2 => {
            if unsafe { RunTextPrintersAndIsPrinter0Active() } != 1 {
                unsafe { addr_of_mut!(FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
                unsafe { DestroyTask(task_id) };
            }
        }
        _ => {}
    }
}

unsafe fn create_draw_task() {
    let _ = unsafe { CreateTask(task_draw_field_message, 0x50) };
}

unsafe fn destroy_draw_task() {
    let task_id = unsafe { FindTaskIdByFunc(task_draw_field_message) };
    if task_id != TASK_NONE {
        unsafe { DestroyTask(task_id) };
    }
}

unsafe fn expand_and_start(message: *const u8, allow_skipping: bool) {
    let _ = unsafe { StringExpandPlaceholders((&raw mut gStringVar4).cast(), message) };
    unsafe { AddTextPrinterForMessage(allow_skipping as u8) };
    unsafe { create_draw_task() };
}

unsafe fn start_from_buffer() {
    unsafe { AddTextPrinterForMessage(1) };
    unsafe { create_draw_task() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitFieldMessageBox() {
    unsafe { addr_of_mut!(FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
    let flags = unsafe { addr_of!(gTextFlags).read() };
    unsafe { addr_of_mut!(gTextFlags).write(flags & !0x0f) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFieldMessage(message: *const u8) -> u8 {
    if unsafe { addr_of!(FIELD_MESSAGE_BOX_MODE).read() } != FIELD_MESSAGE_BOX_HIDDEN {
        return 0;
    }
    unsafe { expand_and_start(message, true) };
    unsafe { addr_of_mut!(FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_NORMAL) };
    1
}

unsafe extern "C" fn task_hide_pokenav_message_when_done(task_id: u8) {
    if unsafe { IsMatchCallTaskActive() } == 0 {
        unsafe { addr_of_mut!(FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
        unsafe { DestroyTask(task_id) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokenavFieldMessage(message: *const u8) -> u8 {
    if unsafe { addr_of!(FIELD_MESSAGE_BOX_MODE).read() } != FIELD_MESSAGE_BOX_HIDDEN {
        return 0;
    }
    let _ = unsafe { StringExpandPlaceholders((&raw mut gStringVar4).cast(), message) };
    let _ = unsafe { CreateTask(task_hide_pokenav_message_when_done, 0) };
    unsafe { StartMatchCallFromScript(message) };
    unsafe { addr_of_mut!(FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_NORMAL) };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFieldAutoScrollMessage(message: *const u8) -> u8 {
    if unsafe { addr_of!(FIELD_MESSAGE_BOX_MODE).read() } != FIELD_MESSAGE_BOX_HIDDEN {
        return 0;
    }
    unsafe { addr_of_mut!(FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_AUTO_SCROLL) };
    unsafe { expand_and_start(message, false) };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFieldMessageFromBuffer() -> u8 {
    if unsafe { addr_of!(FIELD_MESSAGE_BOX_MODE).read() } != FIELD_MESSAGE_BOX_HIDDEN {
        return 0;
    }
    unsafe { addr_of_mut!(FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_NORMAL) };
    unsafe { start_from_buffer() };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideFieldMessageBox() {
    unsafe { destroy_draw_task() };
    unsafe { ClearDialogWindowAndFrame(0, 1) };
    unsafe { addr_of_mut!(FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFieldMessageBoxMode() -> u8 {
    unsafe { addr_of!(FIELD_MESSAGE_BOX_MODE).read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFieldMessageBoxHidden() -> u8 {
    (unsafe { addr_of!(FIELD_MESSAGE_BOX_MODE).read() } == FIELD_MESSAGE_BOX_HIDDEN) as u8
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopFieldMessage() {
    unsafe { destroy_draw_task() };
    unsafe { addr_of_mut!(FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_state_uses_the_engine_task_layout() {
        assert_eq!(TASK_DATA_OFFSET, 8);
        assert_eq!(TASK_SIZE, 40);
    }
}
