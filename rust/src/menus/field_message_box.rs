use crate::ffi::{StringExpandPlaceholders, gStringVar4};
use crate::task::task_data_ptr;
use crate::text::gTextFlags;

const FIELD_MESSAGE_BOX_HIDDEN: u8 = 0;
const FIELD_MESSAGE_BOX_NORMAL: u8 = 2;
const FIELD_MESSAGE_BOX_AUTO_SCROLL: u8 = 3;
const TASK_NONE: u8 = 0xff;

type TaskFunc = unsafe fn(u8);

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
static FIELD_MESSAGE_BOX_MODE: crate::global::Global<u8> =
    crate::global::Global::new(FIELD_MESSAGE_BOX_HIDDEN);

/// `LoadMessageBoxAndBorderGfx` with this module's view of its types.
#[inline]
unsafe fn LoadMessageBoxAndBorderGfx() {
    unsafe {
        crate::menu::LoadMessageBoxAndBorderGfx();
    }
}
/// `DrawDialogueFrame` with this module's view of its types.
#[inline]
unsafe fn DrawDialogueFrame(a0: u8, a1: u8) {
    unsafe {
        crate::menu::DrawDialogueFrame(a0, a1);
    }
}
/// `RunTextPrintersAndIsPrinter0Active` with this module's view of its types.
#[inline]
unsafe fn RunTextPrintersAndIsPrinter0Active() -> u16 {
    unsafe { crate::menu::RunTextPrintersAndIsPrinter0Active() }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: TaskFunc, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `DestroyTask` with this module's view of its types.
#[inline]
unsafe fn DestroyTask(a0: u8) {
    {
        crate::task::DestroyTask(a0);
    }
}
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: TaskFunc) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `IsMatchCallTaskActive` with this module's view of its types.
#[inline]
unsafe fn IsMatchCallTaskActive() -> u32 {
    unsafe { crate::match_call::IsMatchCallTaskActive() }
}
/// `StartMatchCallFromScript` with this module's view of its types.
#[inline]
unsafe fn StartMatchCallFromScript(a0: *const u8) {
    unsafe {
        crate::match_call::StartMatchCallFromScript(a0 as _);
    }
}
/// `AddTextPrinterForMessage` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterForMessage(a0: u8) {
    unsafe {
        crate::menu::AddTextPrinterForMessage(a0);
    }
}
/// `ClearDialogWindowAndFrame` with this module's view of its types.
#[inline]
unsafe fn ClearDialogWindowAndFrame(a0: u8, a1: u8) {
    unsafe {
        crate::menu::ClearDialogWindowAndFrame(a0, a1);
    }
}

fn task_state(task_id: u8) -> *mut i16 {
    task_data_ptr(task_id, 0)
}

unsafe fn task_draw_field_message(task_id: u8) {
    let state = task_state(task_id);
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
                unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
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
pub unsafe fn InitFieldMessageBox() {
    unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
    let flags = unsafe { crate::gl!(&raw gTextFlags).cast_const().read() };
    unsafe { crate::gl!(&raw gTextFlags).write(flags & !0x0f) };
}

#[unsafe(no_mangle)]
pub unsafe fn ShowFieldMessage(message: *const u8) -> u8 {
    if unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).cast_const().read() }
        != FIELD_MESSAGE_BOX_HIDDEN
    {
        return 0;
    }
    unsafe { expand_and_start(message, true) };
    unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_NORMAL) };
    1
}

unsafe fn task_hide_pokenav_message_when_done(task_id: u8) {
    if unsafe { IsMatchCallTaskActive() } == 0 {
        unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
        unsafe { DestroyTask(task_id) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ShowPokenavFieldMessage(message: *const u8) -> u8 {
    if unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).cast_const().read() }
        != FIELD_MESSAGE_BOX_HIDDEN
    {
        return 0;
    }
    let _ = unsafe { StringExpandPlaceholders((&raw mut gStringVar4).cast(), message) };
    let _ = unsafe { CreateTask(task_hide_pokenav_message_when_done, 0) };
    unsafe { StartMatchCallFromScript(message) };
    unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_NORMAL) };
    1
}

#[unsafe(no_mangle)]
pub unsafe fn ShowFieldAutoScrollMessage(message: *const u8) -> u8 {
    if unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).cast_const().read() }
        != FIELD_MESSAGE_BOX_HIDDEN
    {
        return 0;
    }
    unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_AUTO_SCROLL) };
    unsafe { expand_and_start(message, false) };
    1
}

#[unsafe(no_mangle)]
pub unsafe fn ShowFieldMessageFromBuffer() -> u8 {
    if unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).cast_const().read() }
        != FIELD_MESSAGE_BOX_HIDDEN
    {
        return 0;
    }
    unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_NORMAL) };
    unsafe { start_from_buffer() };
    1
}

#[unsafe(no_mangle)]
pub unsafe fn HideFieldMessageBox() {
    unsafe { destroy_draw_task() };
    unsafe { ClearDialogWindowAndFrame(0, 1) };
    unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
}

#[unsafe(no_mangle)]
pub unsafe fn GetFieldMessageBoxMode() -> u8 {
    unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).cast_const().read() }
}

#[unsafe(no_mangle)]
pub unsafe fn IsFieldMessageBoxHidden() -> u8 {
    (unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).cast_const().read() }
        == FIELD_MESSAGE_BOX_HIDDEN) as u8
}

#[unsafe(no_mangle)]
pub unsafe fn StopFieldMessage() {
    unsafe { destroy_draw_task() };
    unsafe { crate::gl!(&raw FIELD_MESSAGE_BOX_MODE).write(FIELD_MESSAGE_BOX_HIDDEN) };
}
