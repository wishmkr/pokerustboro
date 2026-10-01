//! The script engine: bytecode contexts, the global script context that
//! yields to the game loop on wait commands, map header scripts, and the
//! "RAM script" that Mystery Gift can install into the save.

use crate::event_data::VarGet;
use crate::load_save::gSaveBlock1Ptr;
use crate::util::CalcCRC16WithTable;

const RAM_SCRIPT_MAGIC: u8 = 51;

const SCRIPT_MODE_STOPPED: u8 = 0;
const SCRIPT_MODE_BYTECODE: u8 = 1;
const SCRIPT_MODE_NATIVE: u8 = 2;

const CONTEXT_RUNNING: u8 = 0;
const CONTEXT_WAITING: u8 = 1;
const CONTEXT_SHUTDOWN: u8 = 2;

const MAP_SCRIPT_ON_LOAD: u8 = 1;
const MAP_SCRIPT_ON_FRAME_TABLE: u8 = 2;
const MAP_SCRIPT_ON_TRANSITION: u8 = 3;
const MAP_SCRIPT_ON_WARP_INTO_MAP_TABLE: u8 = 4;
const MAP_SCRIPT_ON_RESUME: u8 = 5;
const MAP_SCRIPT_ON_DIVE_WARP: u8 = 6;
const MAP_SCRIPT_ON_RETURN_TO_FIELD: u8 = 7;
const MAP_UNDEFINED_GROUP: u8 = 0xff;
const MAP_UNDEFINED_NUM: u8 = 0xff;
/// `LOCALID_PLAYER`, used here to mean "no object".
const NO_OBJECT: u8 = 0xff;

/// `struct ScriptContext`, 0x74 bytes.
const CTX_SIZE: usize = 0x74;
const CTX_STACK_DEPTH: usize = 0x00;
const CTX_MODE: usize = 0x01;
const CTX_NATIVE_PTR: usize = 0x04;
const CTX_SCRIPT_PTR: usize = 0x08;
const CTX_STACK: usize = 0x0c;
const STACK_SIZE: usize = 20;
const CTX_CMD_TABLE: usize = 0x5c;
const CTX_CMD_TABLE_END: usize = 0x60;
const CTX_DATA: usize = 0x64;

/// `gSaveBlock1Ptr->ramScript`: checksum, then `struct RamScriptData`.
const SB1_RAM_SCRIPT: usize = 0x3728;
const RAM_SCRIPT_SIZE: usize = 0x3ec;
const RAM_SCRIPT_DATA: usize = 4;
const RAM_SCRIPT_DATA_SIZE: u32 = 0x3e8;
const DATA_MAGIC: usize = 0;
const DATA_MAP_GROUP: usize = 1;
const DATA_MAP_NUM: usize = 2;
const DATA_LOCAL_ID: usize = 3;
const DATA_SCRIPT: usize = 4;
const RAM_SCRIPT_CAPACITY: u16 = 0x3e3;
const SB1_LOCATION_MAP_GROUP: usize = 4;
const SB1_LOCATION_MAP_NUM: usize = 5;
const MAP_HEADER_MAP_SCRIPTS: usize = 8;

type ScrCmdFunc = unsafe fn(*mut u8) -> u8;

static GLOBAL_STATUS: crate::global::Global<u8> = crate::global::Global::new(0);
static mut GLOBAL_CONTEXT: crate::ffi::Align4<[u8; CTX_SIZE]> = crate::ffi::Align4([0; CTX_SIZE]);
static mut IMMEDIATE_CONTEXT: crate::ffi::Align4<[u8; CTX_SIZE]> =
    crate::ffi::Align4([0; CTX_SIZE]);
static LOCK_FIELD_CONTROLS: crate::global::Global<u8> = crate::global::Global::new(0);

/// `ValidateSavedWonderCard` with this module's view of its types.
#[inline]
unsafe fn ValidateSavedWonderCard() -> u32 {
    unsafe { crate::mystery_gift::ValidateSavedWonderCard() }
}

#[inline]
fn global_context() -> *mut u8 {
    (&raw mut GLOBAL_CONTEXT).cast()
}

#[inline]
fn immediate_context() -> *mut u8 {
    (&raw mut IMMEDIATE_CONTEXT).cast()
}

#[inline]
unsafe fn script_ptr(ctx: *mut u8) -> *mut *const u8 {
    unsafe { ctx.add(CTX_SCRIPT_PTR).cast() }
}

#[inline]
fn set_global_status(status: u8) {
    unsafe { (GLOBAL_STATUS.as_ptr()).write(status) };
}

#[unsafe(no_mangle)]
pub unsafe fn InitScriptContext(ctx: *mut u8, cmd_table: *const u8, cmd_table_end: *const u8) {
    unsafe {
        ctx.add(CTX_MODE).write(SCRIPT_MODE_STOPPED);
        script_ptr(ctx).write(core::ptr::null());
        ctx.add(CTX_STACK_DEPTH).write(0);
        ctx.add(CTX_NATIVE_PTR)
            .cast::<*const u8>()
            .write(core::ptr::null());
        ctx.add(CTX_CMD_TABLE).cast::<*const u8>().write(cmd_table);
        ctx.add(CTX_CMD_TABLE_END)
            .cast::<*const u8>()
            .write(cmd_table_end);
        ctx.add(CTX_DATA).write_bytes(0, 16);
        ctx.add(CTX_STACK).write_bytes(0, STACK_SIZE * 4);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn SetupBytecodeScript(ctx: *mut u8, ptr: *const u8) -> u8 {
    unsafe { script_ptr(ctx).write(ptr) };
    unsafe { ctx.add(CTX_MODE).write(SCRIPT_MODE_BYTECODE) };
    1
}

#[unsafe(no_mangle)]
pub unsafe fn SetupNativeScript(ctx: *mut u8, ptr: Option<unsafe fn() -> u8>) {
    unsafe { ctx.add(CTX_MODE).write(SCRIPT_MODE_NATIVE) };
    unsafe {
        ctx.add(CTX_NATIVE_PTR)
            .cast::<Option<unsafe fn() -> u8>>()
            .write(ptr)
    };
}

#[unsafe(no_mangle)]
pub unsafe fn StopScript(ctx: *mut u8) {
    unsafe { ctx.add(CTX_MODE).write(SCRIPT_MODE_STOPPED) };
    unsafe { script_ptr(ctx).write(core::ptr::null()) };
}

/// Runs commands until one asks to yield. Returns FALSE once the script has
/// stopped.
#[unsafe(no_mangle)]
pub unsafe fn RunScriptCommand(ctx: *mut u8) -> u8 {
    match unsafe { ctx.add(CTX_MODE).read() } {
        SCRIPT_MODE_STOPPED => return 0,
        SCRIPT_MODE_NATIVE => {
            // Call the native function; continue with bytecode once it (or
            // the missing function) says so.
            let native = unsafe {
                ctx.add(CTX_NATIVE_PTR)
                    .cast::<Option<unsafe fn() -> u8>>()
                    .read()
            };
            if let Some(native) = native {
                if unsafe { native() } == 1 {
                    unsafe { ctx.add(CTX_MODE).write(SCRIPT_MODE_BYTECODE) };
                }
                return 1;
            }
            unsafe { ctx.add(CTX_MODE).write(SCRIPT_MODE_BYTECODE) };
        }
        SCRIPT_MODE_BYTECODE => {}
        _ => return 1,
    }

    loop {
        let ptr = unsafe { script_ptr(ctx).read() };
        if ptr.is_null() {
            unsafe { ctx.add(CTX_MODE).write(SCRIPT_MODE_STOPPED) };
            return 0;
        }
        if ptr
            == unsafe {
                (&raw const (*(&raw const crate::data::scrcmd::gNullScriptPtr).cast::<*const u8>()))
                    .read()
            }
        {
            loop {
                halt();
            }
        }
        let cmd_code = unsafe { ptr.read() };
        unsafe { script_ptr(ctx).write(ptr.wrapping_add(1)) };
        let table = unsafe { ctx.add(CTX_CMD_TABLE).cast::<*const ScrCmdFunc>().read() };
        let end = unsafe {
            ctx.add(CTX_CMD_TABLE_END)
                .cast::<*const ScrCmdFunc>()
                .read()
        };
        let func = table.wrapping_add(usize::from(cmd_code));
        if func >= end {
            unsafe { ctx.add(CTX_MODE).write(SCRIPT_MODE_STOPPED) };
            return 0;
        }
        if unsafe { (func.read())(ctx) } == 1 {
            return 1;
        }
    }
}

/// `svc 2`: the BIOS Halt.
#[inline]
fn halt() {
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!("svc 2")
    };
}

unsafe fn script_push(ctx: *mut u8, ptr: *const u8) -> bool {
    let depth = usize::from(unsafe { ctx.add(CTX_STACK_DEPTH).read() });
    if depth + 1 >= STACK_SIZE {
        return true;
    }
    unsafe {
        ctx.add(CTX_STACK + depth * 4)
            .cast::<*const u8>()
            .write(ptr)
    };
    unsafe { ctx.add(CTX_STACK_DEPTH).write(depth as u8 + 1) };
    false
}

unsafe fn script_pop(ctx: *mut u8) -> *const u8 {
    let depth = unsafe { ctx.add(CTX_STACK_DEPTH).read() };
    if depth == 0 {
        return core::ptr::null();
    }
    unsafe { ctx.add(CTX_STACK_DEPTH).write(depth - 1) };
    unsafe {
        ctx.add(CTX_STACK + usize::from(depth - 1) * 4)
            .cast::<*const u8>()
            .read()
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptJump(ctx: *mut u8, ptr: *const u8) {
    unsafe { script_ptr(ctx).write(ptr) };
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptCall(ctx: *mut u8, ptr: *const u8) {
    unsafe { script_push(ctx, script_ptr(ctx).read()) };
    unsafe { script_ptr(ctx).write(ptr) };
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptReturn(ctx: *mut u8) {
    let ptr = unsafe { script_pop(ctx) };
    unsafe { script_ptr(ctx).write(ptr) };
}

#[inline]
unsafe fn read_byte(ctx: *mut u8) -> u8 {
    let ptr = unsafe { script_ptr(ctx).read() };
    unsafe { script_ptr(ctx).write(ptr.wrapping_add(1)) };
    unsafe { ptr.read() }
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptReadHalfword(ctx: *mut u8) -> u16 {
    let low = u16::from(unsafe { read_byte(ctx) });
    let high = u16::from(unsafe { read_byte(ctx) });
    low | (high << 8)
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptReadWord(ctx: *mut u8) -> u32 {
    let bytes = unsafe {
        [
            read_byte(ctx),
            read_byte(ctx),
            read_byte(ctx),
            read_byte(ctx),
        ]
    };
    u32::from_le_bytes(bytes)
}

#[unsafe(no_mangle)]
pub unsafe fn LockPlayerFieldControls() {
    unsafe { (LOCK_FIELD_CONTROLS.as_ptr()).write(1) };
}

#[unsafe(no_mangle)]
pub unsafe fn UnlockPlayerFieldControls() {
    unsafe { (LOCK_FIELD_CONTROLS.as_ptr()).write(0) };
}

#[unsafe(no_mangle)]
pub unsafe fn ArePlayerFieldControlsLocked() -> u8 {
    unsafe { (LOCK_FIELD_CONTROLS.as_ptr().cast_const()).read() }
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptContext_IsEnabled() -> u8 {
    u8::from(unsafe { (GLOBAL_STATUS.as_ptr().cast_const()).read() } == CONTEXT_RUNNING)
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptContext_Init() {
    unsafe {
        InitScriptContext(
            global_context(),
            &raw const (*crate::asmdata::gScriptCmdTable.cast::<u8>()),
            &raw const (*crate::asmdata::gScriptCmdTableEnd.cast::<u8>()),
        )
    };
    set_global_status(CONTEXT_SHUTDOWN);
}

/// Runs the global script until it waits. Returns TRUE if there is more to
/// run, FALSE when it finished or is waiting/shut down.
#[unsafe(no_mangle)]
pub unsafe fn ScriptContext_RunScript() -> u8 {
    let status = unsafe { (GLOBAL_STATUS.as_ptr().cast_const()).read() };
    if status == CONTEXT_SHUTDOWN || status == CONTEXT_WAITING {
        return 0;
    }
    unsafe { LockPlayerFieldControls() };
    if unsafe { RunScriptCommand(global_context()) } == 0 {
        set_global_status(CONTEXT_SHUTDOWN);
        unsafe { UnlockPlayerFieldControls() };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptContext_SetupScript(ptr: *const u8) {
    unsafe {
        InitScriptContext(
            global_context(),
            &raw const (*crate::asmdata::gScriptCmdTable.cast::<u8>()),
            &raw const (*crate::asmdata::gScriptCmdTableEnd.cast::<u8>()),
        )
    };
    unsafe { SetupBytecodeScript(global_context(), ptr) };
    unsafe { LockPlayerFieldControls() };
    set_global_status(CONTEXT_RUNNING);
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptContext_Stop() {
    set_global_status(CONTEXT_WAITING);
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptContext_Enable() {
    set_global_status(CONTEXT_RUNNING);
    unsafe { LockPlayerFieldControls() };
}

/// Runs a script to completion in its own context (map header scripts).
#[unsafe(no_mangle)]
pub unsafe fn RunScriptImmediately(ptr: *const u8) {
    let ctx = immediate_context();
    unsafe {
        InitScriptContext(
            ctx,
            &raw const (*crate::asmdata::gScriptCmdTable.cast::<u8>()),
            &raw const (*crate::asmdata::gScriptCmdTableEnd.cast::<u8>()),
        )
    };
    unsafe { SetupBytecodeScript(ctx, ptr) };
    while unsafe { RunScriptCommand(ctx) } == 1 {}
}

/// `T2_READ_PTR`: an unaligned little-endian pointer.
#[inline]
unsafe fn read_unaligned_ptr(ptr: *const u8) -> *mut u8 {
    unsafe { ptr.cast::<u32>().read_unaligned() as usize as *mut u8 }
}

#[unsafe(no_mangle)]
pub unsafe fn MapHeaderGetScriptTable(tag: u8) -> *mut u8 {
    let mut scripts = unsafe {
        (&raw const (*(&raw const crate::fieldmap::gMapHeader).cast::<u8>()))
            .add(MAP_HEADER_MAP_SCRIPTS)
            .cast::<*const u8>()
            .read()
    };
    if scripts.is_null() {
        return core::ptr::null_mut();
    }
    loop {
        let entry = unsafe { scripts.read() };
        if entry == 0 {
            return core::ptr::null_mut();
        }
        if entry == tag {
            return unsafe { read_unaligned_ptr(scripts.add(1)) };
        }
        scripts = scripts.wrapping_add(5);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn MapHeaderRunScriptType(tag: u8) {
    let ptr = unsafe { MapHeaderGetScriptTable(tag) };
    if !ptr.is_null() {
        unsafe { RunScriptImmediately(ptr) };
    }
}

/// Finds the first entry of a frame/warp table whose two vars are equal.
#[unsafe(no_mangle)]
pub unsafe fn MapHeaderCheckScriptTable(tag: u8) -> *mut u8 {
    let mut ptr = unsafe { MapHeaderGetScriptTable(tag) }.cast_const();
    if ptr.is_null() {
        return core::ptr::null_mut();
    }
    loop {
        let var1 = unsafe { ptr.cast::<u16>().read_unaligned() };
        if var1 == 0 {
            return core::ptr::null_mut();
        }
        let var2 = unsafe { ptr.add(2).cast::<u16>().read_unaligned() };
        if unsafe { VarGet(var1) } == unsafe { VarGet(var2) } {
            return unsafe { read_unaligned_ptr(ptr.add(4)) };
        }
        ptr = ptr.wrapping_add(8);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn RunOnLoadMapScript() {
    unsafe { MapHeaderRunScriptType(MAP_SCRIPT_ON_LOAD) };
}

#[unsafe(no_mangle)]
pub unsafe fn RunOnTransitionMapScript() {
    unsafe { MapHeaderRunScriptType(MAP_SCRIPT_ON_TRANSITION) };
}

#[unsafe(no_mangle)]
pub unsafe fn RunOnResumeMapScript() {
    unsafe { MapHeaderRunScriptType(MAP_SCRIPT_ON_RESUME) };
}

#[unsafe(no_mangle)]
pub unsafe fn RunOnReturnToFieldMapScript() {
    unsafe { MapHeaderRunScriptType(MAP_SCRIPT_ON_RETURN_TO_FIELD) };
}

#[unsafe(no_mangle)]
pub unsafe fn RunOnDiveWarpMapScript() {
    unsafe { MapHeaderRunScriptType(MAP_SCRIPT_ON_DIVE_WARP) };
}

#[unsafe(no_mangle)]
pub unsafe fn TryRunOnFrameMapScript() -> u8 {
    let ptr = unsafe { MapHeaderCheckScriptTable(MAP_SCRIPT_ON_FRAME_TABLE) };
    if ptr.is_null() {
        return 0;
    }
    unsafe { ScriptContext_SetupScript(ptr) };
    1
}

#[unsafe(no_mangle)]
pub unsafe fn TryRunOnWarpIntoMapScript() {
    let ptr = unsafe { MapHeaderCheckScriptTable(MAP_SCRIPT_ON_WARP_INTO_MAP_TABLE) };
    if !ptr.is_null() {
        unsafe { RunScriptImmediately(ptr) };
    }
}

#[inline]
unsafe fn ram_script() -> *mut u8 {
    unsafe {
        (&raw const gSaveBlock1Ptr)
            .read()
            .cast::<u8>()
            .add(SB1_RAM_SCRIPT)
    }
}

#[inline]
unsafe fn ram_script_data() -> *mut u8 {
    unsafe { ram_script().add(RAM_SCRIPT_DATA) }
}

#[inline]
unsafe fn stored_checksum() -> u32 {
    unsafe { ram_script().cast::<u32>().read() }
}

#[unsafe(no_mangle)]
pub unsafe fn CalculateRamScriptChecksum() -> u32 {
    u32::from(unsafe { CalcCRC16WithTable(ram_script_data(), RAM_SCRIPT_DATA_SIZE) })
}

#[unsafe(no_mangle)]
pub unsafe fn ClearRamScript() {
    unsafe { ram_script().write_bytes(0, RAM_SCRIPT_SIZE) };
}

#[unsafe(no_mangle)]
pub unsafe fn InitRamScript(
    script: *const u8,
    script_size: u16,
    map_group: u8,
    map_num: u8,
    local_id: u8,
) -> u8 {
    unsafe { ClearRamScript() };
    if script_size > RAM_SCRIPT_CAPACITY {
        return 0;
    }
    let data = unsafe { ram_script_data() };
    unsafe {
        data.add(DATA_MAGIC).write(RAM_SCRIPT_MAGIC);
        data.add(DATA_MAP_GROUP).write(map_group);
        data.add(DATA_MAP_NUM).write(map_num);
        data.add(DATA_LOCAL_ID).write(local_id);
        core::ptr::copy_nonoverlapping(script, data.add(DATA_SCRIPT), usize::from(script_size));
        ram_script()
            .cast::<u32>()
            .write(CalculateRamScriptChecksum());
    }
    1
}

/// Replaces an object's script with the installed RAM script, if one is
/// installed for this map and object and its checksum holds.
#[unsafe(no_mangle)]
pub unsafe fn GetRamScript(local_id: u8, script: *const u8) -> *const u8 {
    unsafe {
        (&raw mut (*(&raw const crate::scrcmd::gRamScriptRetAddr)
            .cast::<*const u8>()
            .cast_mut()))
            .write(core::ptr::null())
    };
    let data = unsafe { ram_script_data() };
    let sb1 = unsafe { (&raw const gSaveBlock1Ptr).read().cast::<u8>() };
    let matches = unsafe {
        data.add(DATA_MAGIC).read() == RAM_SCRIPT_MAGIC
            && data.add(DATA_MAP_GROUP).read() == sb1.add(SB1_LOCATION_MAP_GROUP).read()
            && data.add(DATA_MAP_NUM).read() == sb1.add(SB1_LOCATION_MAP_NUM).read()
            && data.add(DATA_LOCAL_ID).read() == local_id
    };
    if !matches {
        return script;
    }
    if unsafe { CalculateRamScriptChecksum() } != unsafe { stored_checksum() } {
        unsafe { ClearRamScript() };
        return script;
    }
    unsafe {
        (&raw mut (*(&raw const crate::scrcmd::gRamScriptRetAddr)
            .cast::<*const u8>()
            .cast_mut()))
            .write(script)
    };
    unsafe { data.add(DATA_SCRIPT) }
}

unsafe fn is_unbound_ram_script(data: *const u8) -> bool {
    unsafe {
        data.add(DATA_MAGIC).read() == RAM_SCRIPT_MAGIC
            && data.add(DATA_MAP_GROUP).read() == MAP_UNDEFINED_GROUP
            && data.add(DATA_MAP_NUM).read() == MAP_UNDEFINED_NUM
            && data.add(DATA_LOCAL_ID).read() == NO_OBJECT
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ValidateSavedRamScript() -> u32 {
    let data = unsafe { ram_script_data() };
    let valid = unsafe { is_unbound_ram_script(data) }
        && unsafe { CalculateRamScriptChecksum() } == unsafe { stored_checksum() };
    u32::from(valid)
}

#[unsafe(no_mangle)]
pub unsafe fn GetSavedRamScriptIfValid() -> *mut u8 {
    let data = unsafe { ram_script_data() };
    if unsafe { ValidateSavedWonderCard() } == 0 || !unsafe { is_unbound_ram_script(data) } {
        return core::ptr::null_mut();
    }
    if unsafe { CalculateRamScriptChecksum() } != unsafe { stored_checksum() } {
        unsafe { ClearRamScript() };
        return core::ptr::null_mut();
    }
    unsafe { data.add(DATA_SCRIPT) }
}

#[unsafe(no_mangle)]
pub unsafe fn InitRamScript_NoObjectEvent(script: *const u8, script_size: u16) {
    let size = script_size.min(RAM_SCRIPT_CAPACITY);
    unsafe {
        InitRamScript(
            script,
            size,
            MAP_UNDEFINED_GROUP,
            MAP_UNDEFINED_NUM,
            NO_OBJECT,
        )
    };
}
