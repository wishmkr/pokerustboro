//! Per-scanline register effects: an HBlank DMA copies one value per line
//! from a double-buffered table into a video register. Also the "wave" task
//! that ripples a background horizontally or vertically.

use crate::ffi::{Align4, CreateTask, DestroyTask, TASK_NONE, set_task_data, task_data};
use crate::trig::gSineTable;

const REG_ADDR_BG0HOFS: usize = 0x0400_0010;
const REG_ADDR_DMA0: usize = 0x0400_00b0;
const DMA_ENABLE: u16 = 0x8000;
const DMA_START_MASK: u16 = 0x3000;
const DMA_DREQ_ON: u16 = 0x0800;
const DMA_REPEAT: u16 = 0x0200;

/// One 16-bit transfer per HBlank, repeating, destination reloaded.
pub const SCANLINE_EFFECT_DMACNT_16BIT: u32 = 0xa260_0001;

const REG_BG0HOFS: u8 = 0x0;
const REG_BG0VOFS: u8 = 0x2;
const REG_BG1HOFS: u8 = 0x4;
const REG_BG1VOFS: u8 = 0x6;
const REG_BG2HOFS: u8 = 0x8;
const REG_BG2VOFS: u8 = 0xa;
const REG_BG3HOFS: u8 = 0xc;
const REG_BG3VOFS: u8 = 0xe;

const LINES: usize = 0x3c0;
/// The generated wave lives past the visible lines of buffer 0.
const WAVE_OFFSET: usize = 320;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gScanlineEffectRegBuffers: Align4<[[u16; LINES]; 2]> = Align4([[0; LINES]; 2]);

/// `struct ScanlineEffect`, 28 bytes.
#[repr(C, align(4))]
pub struct ScanlineEffect {
    pub dma_src_buffers: [*mut u8; 2],
    pub dma_dest: *mut u8,
    pub dma_control: u32,
    pub set_first_scanline_reg: Option<unsafe extern "C" fn()>,
    pub src_buffer: u8,
    pub state: u8,
    pub unused16: u8,
    pub unused17: u8,
    pub wave_task_id: u8,
}

/// `struct ScanlineEffectParams`, passed by value.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct ScanlineEffectParams {
    pub dma_dest: *mut u8,
    pub dma_control: u32,
    pub init_state: u8,
    pub unused9: u8,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gScanlineEffect: ScanlineEffect = ScanlineEffect {
    dma_src_buffers: [core::ptr::null_mut(); 2],
    dma_dest: core::ptr::null_mut(),
    dma_control: 0,
    set_first_scanline_reg: None,
    src_buffer: 0,
    state: 0,
    unused16: 0,
    unused17: 0,
    wave_task_id: 0,
};

#[unsafe(link_section = "ewram_data")]
static mut SHOULD_STOP_WAVE_TASK: u8 = 0;

unsafe extern "C" {
    static gBattle_BG0_X: u16;
    static gBattle_BG0_Y: u16;
    static gBattle_BG1_X: u16;
    static gBattle_BG1_Y: u16;
    static gBattle_BG2_X: u16;
    static gBattle_BG2_Y: u16;
    static gBattle_BG3_X: u16;
    static gBattle_BG3_Y: u16;
}

#[inline]
fn effect() -> *mut ScanlineEffect {
    &raw mut gScanlineEffect
}

#[inline]
fn buffer(index: usize) -> *mut u16 {
    (&raw mut gScanlineEffectRegBuffers)
        .cast::<u16>()
        .wrapping_add(index * LINES)
}

/// `DmaStop(0)`
unsafe fn dma0_stop() {
    let control = (REG_ADDR_DMA0 + 10) as *mut u16;
    unsafe {
        control
            .write_volatile(control.read_volatile() & !(DMA_START_MASK | DMA_DREQ_ON | DMA_REPEAT))
    };
    unsafe { control.write_volatile(control.read_volatile() & !DMA_ENABLE) };
    unsafe { control.read_volatile() };
}

/// `DmaSet(0, src, dest, control)`
unsafe fn dma0_set(src: *const u8, dest: *mut u8, control: u32) {
    let regs = REG_ADDR_DMA0 as *mut u32;
    unsafe { regs.write_volatile(src as usize as u32) };
    unsafe { regs.add(1).write_volatile(dest as usize as u32) };
    unsafe { regs.add(2).write_volatile(control) };
    unsafe { regs.add(2).read_volatile() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScanlineEffect_Stop() {
    let e = effect();
    unsafe { (*e).state = 0 };
    unsafe { dma0_stop() };
    let task = unsafe { (*e).wave_task_id };
    if task != TASK_NONE {
        unsafe { DestroyTask(task) };
        unsafe { (*e).wave_task_id = TASK_NONE };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScanlineEffect_Clear() {
    unsafe { buffer(0).write_bytes(0, 2 * LINES) };
    let e = effect();
    unsafe {
        (*e).dma_src_buffers = [core::ptr::null_mut(); 2];
        (*e).dma_dest = core::ptr::null_mut();
        (*e).dma_control = 0;
        (*e).src_buffer = 0;
        (*e).state = 0;
        (*e).unused16 = 0;
        (*e).unused17 = 0;
        (*e).wave_task_id = TASK_NONE;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScanlineEffect_SetParams(params: ScanlineEffectParams) {
    let e = effect();
    // The DMA source starts at the *second* line's value: the first transfer
    // happens in the HBlank after line 0 is drawn.
    if params.dma_control == SCANLINE_EFFECT_DMACNT_16BIT {
        unsafe {
            (*e).dma_src_buffers = [buffer(0).add(1).cast(), buffer(1).add(1).cast()];
            (*e).set_first_scanline_reg = Some(copy_value_16bit);
        }
    } else {
        unsafe {
            (*e).dma_src_buffers = [
                buffer(0).cast::<u32>().add(1).cast(),
                buffer(1).cast::<u32>().add(1).cast(),
            ];
            (*e).set_first_scanline_reg = Some(copy_value_32bit);
        }
    }
    unsafe {
        (*e).dma_control = params.dma_control;
        (*e).dma_dest = params.dma_dest;
        (*e).state = params.init_state;
        (*e).unused16 = params.unused9;
        (*e).unused17 = params.unused9;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScanlineEffect_InitHBlankDmaTransfer() {
    let e = effect();
    match unsafe { (*e).state } {
        0 => {}
        3 => {
            unsafe { (*e).state = 0 };
            unsafe { dma0_stop() };
            unsafe { (&raw mut SHOULD_STOP_WAVE_TASK).write(1) };
        }
        _ => {
            unsafe { dma0_stop() };
            let src = unsafe { (*e).dma_src_buffers[usize::from((*e).src_buffer & 1)] };
            unsafe { dma0_set(src, (*e).dma_dest, (*e).dma_control) };
            if let Some(set_first) = unsafe { (*e).set_first_scanline_reg } {
                unsafe { set_first() };
            }
            unsafe { (*e).src_buffer ^= 1 };
        }
    }
}

unsafe extern "C" fn copy_value_16bit() {
    let e = effect();
    let src = buffer(usize::from(unsafe { (*e).src_buffer }));
    let dest = unsafe { (*e).dma_dest }.cast::<u16>();
    unsafe { dest.write_volatile(src.read_volatile()) };
}

unsafe extern "C" fn copy_value_32bit() {
    let e = effect();
    let src = buffer(usize::from(unsafe { (*e).src_buffer })).cast::<u32>();
    let dest = unsafe { (*e).dma_dest }.cast::<u32>();
    unsafe { dest.write_volatile(src.read_volatile()) };
}

// Wave task data.
const T_START_LINE: usize = 0;
const T_END_LINE: usize = 1;
const T_WAVE_LENGTH: usize = 2;
const T_SRC_BUFFER_OFFSET: usize = 3;
const T_FRAMES_UNTIL_MOVE: usize = 4;
const T_DELAY_INTERVAL: usize = 5;
const T_REG_OFFSET: usize = 6;
const T_APPLY_BATTLE_BG_OFFSETS: usize = 7;

unsafe extern "C" fn task_update_wave_per_frame(task_id: u8) {
    if unsafe { (&raw const SHOULD_STOP_WAVE_TASK).read() } != 0 {
        unsafe { DestroyTask(task_id) };
        unsafe { (*effect()).wave_task_id = TASK_NONE };
        return;
    }

    let data = |i| unsafe { task_data(task_id, i) };
    let mut value = 0i32;
    if data(T_APPLY_BATTLE_BG_OFFSETS) != 0 {
        let register = match data(T_REG_OFFSET) as u8 {
            REG_BG0HOFS => Some(&raw const gBattle_BG0_X),
            REG_BG0VOFS => Some(&raw const gBattle_BG0_Y),
            REG_BG1HOFS => Some(&raw const gBattle_BG1_X),
            REG_BG1VOFS => Some(&raw const gBattle_BG1_Y),
            REG_BG2HOFS => Some(&raw const gBattle_BG2_X),
            REG_BG2VOFS => Some(&raw const gBattle_BG2_Y),
            REG_BG3HOFS => Some(&raw const gBattle_BG3_X),
            REG_BG3VOFS => Some(&raw const gBattle_BG3_Y),
            _ => None,
        };
        if let Some(register) = register {
            value = i32::from(unsafe { register.read() });
        }
    }

    let moving = data(T_FRAMES_UNTIL_MOVE) == 0;
    if moving {
        unsafe { set_task_data(task_id, T_FRAMES_UNTIL_MOVE, data(T_DELAY_INTERVAL)) };
    } else {
        unsafe { set_task_data(task_id, T_FRAMES_UNTIL_MOVE, data(T_FRAMES_UNTIL_MOVE) - 1) };
    }

    let dest = buffer(usize::from(unsafe { (*effect()).src_buffer }));
    let wave = buffer(0);
    let mut offset = i32::from(data(T_SRC_BUFFER_OFFSET)) + WAVE_OFFSET as i32;
    for line in i32::from(data(T_START_LINE))..i32::from(data(T_END_LINE)) {
        let wave_value = i32::from(unsafe { wave.wrapping_offset(offset as isize).read() });
        unsafe {
            dest.wrapping_offset(line as isize)
                .write((wave_value + value) as u16)
        };
        offset += 1;
    }

    if moving {
        let mut src_offset = data(T_SRC_BUFFER_OFFSET).wrapping_add(1);
        if src_offset == data(T_WAVE_LENGTH) {
            src_offset = 0;
        }
        unsafe { set_task_data(task_id, T_SRC_BUFFER_OFFSET, src_offset) };
    }
}

unsafe fn generate_wave(buffer: *mut u16, frequency: u8, amplitude: u8) {
    let mut theta = 0u8;
    for i in 0..256 {
        let sine = i32::from(gSineTable[usize::from(theta)]);
        unsafe {
            buffer
                .add(i)
                .write((sine * i32::from(amplitude) / 256) as u16)
        };
        theta = theta.wrapping_add(frequency);
    }
}

/// Starts a wave on lines `start_line..end_line` of the register at
/// `REG_ADDR_BG0HOFS + reg_offset`. The wave moves up one line every
/// `delay_interval + 1` frames.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScanlineEffect_InitWave(
    start_line: u8,
    end_line: u8,
    frequency: u8,
    amplitude: u8,
    delay_interval: u8,
    reg_offset: u8,
    apply_battle_bg_offsets: u8,
) -> u8 {
    unsafe { ScanlineEffect_Clear() };
    unsafe {
        ScanlineEffect_SetParams(ScanlineEffectParams {
            dma_dest: (REG_ADDR_BG0HOFS + usize::from(reg_offset)) as *mut u8,
            dma_control: SCANLINE_EFFECT_DMACNT_16BIT,
            init_state: 1,
            unused9: 0,
        })
    };

    let task_id = unsafe { CreateTask(task_update_wave_per_frame, 0) };
    // 256 / frequency; the original divides without a guard.
    let wave_length = if frequency == 0 {
        0
    } else {
        256 / i16::from(frequency)
    };
    let fields = [
        (T_START_LINE, i16::from(start_line)),
        (T_END_LINE, i16::from(end_line)),
        (T_WAVE_LENGTH, wave_length),
        (T_SRC_BUFFER_OFFSET, 0),
        (T_FRAMES_UNTIL_MOVE, i16::from(delay_interval)),
        (T_DELAY_INTERVAL, i16::from(delay_interval)),
        (T_REG_OFFSET, i16::from(reg_offset)),
        (
            T_APPLY_BATTLE_BG_OFFSETS,
            i16::from(apply_battle_bg_offsets),
        ),
    ];
    for (index, value) in fields {
        unsafe { set_task_data(task_id, index, value) };
    }

    unsafe { (*effect()).wave_task_id = task_id };
    unsafe { (&raw mut SHOULD_STOP_WAVE_TASK).write(0) };

    unsafe { generate_wave(buffer(0).add(WAVE_OFFSET), frequency, amplitude) };
    let mut offset = WAVE_OFFSET;
    for line in usize::from(start_line)..usize::from(end_line) {
        let value = unsafe { buffer(0).add(offset).read() };
        unsafe { buffer(0).add(line).write(value) };
        unsafe { buffer(1).add(line).write(value) };
        offset += 1;
    }
    task_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_struct_is_28_bytes_on_the_gba_layout() {
        // Five pointer-or-word fields, then five bytes, padded to a word.
        let expected = 5 * core::mem::size_of::<usize>() + 5;
        let size = core::mem::size_of::<ScanlineEffect>();
        assert!(size >= expected && size % core::mem::align_of::<ScanlineEffect>() == 0);
    }
}
