//! Translated from `src/palette.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::missing_transmute_annotations,
    dead_code,
    unused_assignments,
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::task::gTasks;
use crate::task::{DestroyTask, GetWordTaskArg, SetWordTaskArg};
use crate::task::{task_func, task_get, task_set};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
// The C's names for task and sprite data slots.
const tCoeff: usize = 0;
const tCoeffTarget: usize = 1;
const tCoeffDelta: usize = 2;
const tDelay: usize = 3;
const tColor: usize = 7;
const tId: usize = 8;
// Data tables (translate with cdata.py): sDummyPaletteStructTemplate sRoundedDownGrayscaleMap

/// `struct PaletteStruct`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PaletteStruct {
    pub template: *mut PaletteStructTemplate,
    bits_4: u32,
    pub countdown1: u8,
    pub countdown2: u8,
}

impl PaletteStruct {
    #[inline(always)]
    pub fn active(&self) -> u32 {
        self.bits_4 & 0x1
    }
    #[inline(always)]
    pub fn set_active(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn flag(&self) -> u32 {
        (self.bits_4 >> 1) & 0x1
    }
    #[inline(always)]
    pub fn set_flag(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 1)) | ((v & 0x1) << 1);
    }
    #[inline(always)]
    pub fn baseDestOffset(&self) -> u32 {
        (self.bits_4 >> 2) & 0x1ff
    }
    #[inline(always)]
    pub fn set_baseDestOffset(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1ff << 2)) | ((v & 0x1ff) << 2);
    }
    #[inline(always)]
    pub fn destOffset(&self) -> u32 {
        (self.bits_4 >> 11) & 0x3ff
    }
    #[inline(always)]
    pub fn set_destOffset(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x3ff << 11)) | ((v & 0x3ff) << 11);
    }
    #[inline(always)]
    pub fn srcIndex(&self) -> u32 {
        (self.bits_4 >> 21) & 0x7f
    }
    #[inline(always)]
    pub fn set_srcIndex(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x7f << 21)) | ((v & 0x7f) << 21);
    }
}

unsafe impl Sync for PaletteStruct {}

/// `struct PaletteStructTemplate`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PaletteStructTemplate {
    pub id: u16,
    pub src: *mut u16,
    bits_8: u16,
    pub time1: u8,
    bits_11: u8,
    pub time2: u8,
}

impl PaletteStructTemplate {
    #[inline(always)]
    pub fn pst_field_8_0(&self) -> u16 {
        ((self.bits_8 as u32) & 0x1) as u16
    }
    #[inline(always)]
    pub fn set_pst_field_8_0(&mut self, v: u16) {
        self.bits_8 = (self.bits_8 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn unused(&self) -> u16 {
        ((self.bits_8 as u32 >> 1) & 0x1ff) as u16
    }
    #[inline(always)]
    pub fn set_unused(&mut self, v: u16) {
        self.bits_8 = (self.bits_8 & !(0x1ff << 1)) | ((v & 0x1ff) << 1);
    }
    #[inline(always)]
    pub fn size(&self) -> u16 {
        ((self.bits_8 as u32 >> 10) & 0x1f) as u16
    }
    #[inline(always)]
    pub fn set_size(&mut self, v: u16) {
        self.bits_8 = (self.bits_8 & !(0x1f << 10)) | ((v & 0x1f) << 10);
    }
    #[inline(always)]
    pub fn srcCount(&self) -> u8 {
        ((self.bits_11 as u32) & 0x1f) as u8
    }
    #[inline(always)]
    pub fn set_srcCount(&mut self, v: u8) {
        self.bits_11 = (self.bits_11 & !0x1f) | (v & 0x1f);
    }
    #[inline(always)]
    pub fn state(&self) -> u8 {
        ((self.bits_11 as u32 >> 5) & 0x7) as u8
    }
    #[inline(always)]
    pub fn set_state(&mut self, v: u8) {
        self.bits_11 = (self.bits_11 & !(0x7 << 5)) | ((v & 0x7) << 5);
    }
}

unsafe impl Sync for PaletteStructTemplate {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PaletteStruct>() == 12);
    assert!(offset_of!(PaletteStruct, template) == 0);
    assert!(offset_of!(PaletteStruct, bits_4) == 4);
    assert!(offset_of!(PaletteStruct, countdown1) == 8);
    assert!(offset_of!(PaletteStruct, countdown2) == 9);
    assert!(size_of::<PaletteStructTemplate>() == 16);
    assert!(offset_of!(PaletteStructTemplate, id) == 0);
    assert!(offset_of!(PaletteStructTemplate, src) == 4);
    assert!(offset_of!(PaletteStructTemplate, bits_8) == 8);
    assert!(offset_of!(PaletteStructTemplate, time1) == 10);
    assert!(offset_of!(PaletteStructTemplate, bits_11) == 11);
    assert!(offset_of!(PaletteStructTemplate, time2) == 12);
};

const FAST_FADE: u16 = 1;
const HARDWARE_FADE: u16 = 2;
const NORMAL_FADE: u16 = 0;
const NUM_PALETTE_STRUCTS: u8 = 16;
const tPalettes: u8 = 5;

static sDummyPaletteStructTemplate: Table<PaletteStructTemplate> =
    Table((&raw const crate::data::palette::sDummyPaletteStructTemplate).cast());
static sRoundedDownGrayscaleMap: Table<CArray<u8, 32>> =
    Table((&raw const crate::data::palette::sRoundedDownGrayscaleMap).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlttBufferUnfaded: Aligned<CArray<u16, 512>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlttBufferFaded: Aligned<CArray<u16, 512>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPaletteStructs: CArray<PaletteStruct, 16> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPaletteFade: PaletteFadeControl = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFiller: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static sPlttBufferTransferPending: crate::global::Global<u32> =
    crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPaletteDecompressionBuffer: Aligned<CArray<u8, 1024>> =
    Aligned(unsafe { zeroed() });

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn LoadCompressedPalette(src: *mut u32, offset: u16, size: u16) {
    LZDecompressWram(src, gPaletteDecompressionBuffer.as_mut_ptr() as *mut c_void);
    CpuSet(
        gPaletteDecompressionBuffer.as_mut_ptr() as *mut c_void,
        &raw mut gPlttBufferUnfaded[offset] as *mut c_void,
        (size as i32 / 2) as u32 & 0x1FFFFF,
    );
    CpuSet(
        gPaletteDecompressionBuffer.as_mut_ptr() as *mut c_void,
        &raw mut gPlttBufferFaded[offset] as *mut c_void,
        (size as i32 / 2) as u32 & 0x1FFFFF,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn LoadPalette(src: *mut c_void, offset: u16, size: u16) {
    CpuSet(
        src,
        &raw mut gPlttBufferUnfaded[offset] as *mut c_void,
        (size as i32 / 2) as u32 & 0x1FFFFF,
    );
    CpuSet(
        src,
        &raw mut gPlttBufferFaded[offset] as *mut c_void,
        (size as i32 / 2) as u32 & 0x1FFFFF,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn FillPalette(value: u16, offset: u16, size: u16) {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, value);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut gPlttBufferUnfaded[offset] as *mut c_void,
                0x1000000 | (size as i32 / 2) as u32 & 0x1FFFFF,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, value);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut gPlttBufferFaded[offset] as *mut c_void,
                0x1000000 | (size as i32 / 2) as u32 & 0x1FFFFF,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn TransferPlttBuffer() {
    if gPaletteFade.bufferTransferDisabled() == 0 {
        {
            let mut _src: *mut c_void = gPlttBufferFaded.as_mut_ptr() as *mut c_void;
            let mut _dest: *mut c_void = PLTT as i32 as usize as *mut c_void;
            let mut _size: u32 = PLTT_SIZE;
            {
                {
                    {
                        let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                        volatile_write(dmaRegs, _src as usize as u32);
                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                        volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                        let _ = (dmaRegs.at(2)).read_volatile();
                    }
                }
            }
        }
        sPlttBufferTransferPending.set(FALSE as u32);
        if gPaletteFade.mode() == HARDWARE_FADE && gPaletteFade.active() != 0 {
            UpdateBlendRegisters();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn UpdatePaletteFade() -> u8 {
    let mut result: u8 = 0;
    let dummy: u8 = 0;
    if sPlttBufferTransferPending.get() != 0 {
        return PALETTE_FADE_STATUS_LOADING;
    }
    if gPaletteFade.mode() == NORMAL_FADE {
        result = UpdateNormalPaletteFade();
    } else if gPaletteFade.mode() == FAST_FADE {
        result = UpdateFastPaletteFade();
    } else {
        result = UpdateHardwarePaletteFade();
    }
    sPlttBufferTransferPending.set(gPaletteFade.multipurpose1 | dummy as u32);
    result
}
#[unsafe(no_mangle)]
pub unsafe fn ResetPaletteFade() {
    for i in 0..NUM_PALETTE_STRUCTS {
        PaletteStruct_Reset(i);
    }
    ResetPaletteFadeControl();
}
unsafe fn ReadPlttIntoBuffers() {
    let pltt: *mut u16 = PLTT as i32 as usize as *mut u16;
    for i in 0..512u16 {
        gPlttBufferUnfaded[i] = *pltt.at(i);
        gPlttBufferFaded[i] = *pltt.at(i);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BeginNormalPaletteFade(
    selectedPalettes: u32,
    mut delay: i8,
    startY: u8,
    targetY: u8,
    blendColor: u16,
) -> u8 {
    let mut temp: u8 = 0;
    let color: u16 = blendColor;
    if gPaletteFade.active() != 0 {
        return FALSE;
    } else {
        gPaletteFade.set_deltaY(2);
        if delay < 0 {
            gPaletteFade.set_deltaY(gPaletteFade.deltaY() + delay as u8 * 255);
            delay = 0;
        }
        gPaletteFade.multipurpose1 = selectedPalettes;
        gPaletteFade.set_delayCounter(delay as u8);
        gPaletteFade.set_multipurpose2(delay as u16);
        gPaletteFade.set_y(startY as u16);
        gPaletteFade.set_targetY(targetY as u16);
        gPaletteFade.set_blendColor(color);
        gPaletteFade.set_active(TRUE as u16);
        gPaletteFade.set_mode(NORMAL_FADE);
        if startY < targetY {
            gPaletteFade.set_yDec(0);
        } else {
            gPaletteFade.set_yDec(1);
        }
        UpdatePaletteFade();
        temp = gPaletteFade.bufferTransferDisabled() as u8;
        gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
        CpuSet(
            gPlttBufferFaded.as_mut_ptr() as *mut c_void,
            PLTT as i32 as usize as *mut c_void,
            0x4000100,
        );
        sPlttBufferTransferPending.set(FALSE as u32);
        if gPaletteFade.mode() == HARDWARE_FADE && gPaletteFade.active() != 0 {
            UpdateBlendRegisters();
        }
        gPaletteFade.set_bufferTransferDisabled(temp as u16);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn BeginPlttFade(
    selectedPalettes: u32,
    delay: u8,
    startY: u8,
    targetY: u8,
    blendColor: u16,
) -> u8 {
    ReadPlttIntoBuffers();
    BeginNormalPaletteFade(selectedPalettes, delay as i8, startY, targetY, blendColor)
}
unsafe fn PaletteStruct_Run(a1: u8, unkFlags: *mut u32) {
    for i in 0..NUM_PALETTE_STRUCTS {
        'l1: {
            let palstruct: *mut PaletteStruct = &raw mut sPaletteStructs[i];
            if (*palstruct).active() != 0 && (*(*palstruct).template).pst_field_8_0() == a1 as u16 {
                if (*palstruct).srcIndex() == (*(*palstruct).template).srcCount() as u32 {
                    PaletteStruct_TryEnd(palstruct);
                    if (*palstruct).active() == 0 {
                        break 'l1;
                    }
                }
                if (*palstruct).countdown1 == 0 {
                    PaletteStruct_Copy(palstruct, unkFlags);
                } else {
                    (*palstruct).countdown1 -= 1;
                }
                PaletteStruct_Blend(palstruct, unkFlags);
            }
        }
    }
}
unsafe fn PaletteStruct_Copy(palStruct: *mut PaletteStruct, unkFlags: *mut u32) {
    let mut srcIndex: i32 = 0;
    let mut srcCount: i32 = 0;
    let mut i: u8 = 0;
    let mut srcOffset: u16 = (*palStruct).srcIndex() as u16 * (*(*palStruct).template).size();
    if (*(*palStruct).template).pst_field_8_0() == 0 {
        while (i as u16) < (*(*palStruct).template).size() {
            gPlttBufferUnfaded[(*palStruct).destOffset()] =
                *(*(*palStruct).template).src.at(srcOffset);
            gPlttBufferFaded[(*palStruct).destOffset()] =
                *(*(*palStruct).template).src.at(srcOffset);
            i += 1;
            (*palStruct).set_destOffset((*palStruct).destOffset() + 1);
            srcOffset += 1;
        }
    } else {
        while (i as u16) < (*(*palStruct).template).size() {
            gPlttBufferFaded[(*palStruct).destOffset()] =
                *(*(*palStruct).template).src.at(srcOffset);
            i += 1;
            (*palStruct).set_destOffset((*palStruct).destOffset() + 1);
            srcOffset += 1;
        }
    }
    (*palStruct).set_destOffset((*palStruct).baseDestOffset());
    (*palStruct).countdown1 = (*(*palStruct).template).time1;
    (*palStruct).set_srcIndex((*palStruct).srcIndex() + 1);
    srcIndex = (*palStruct).srcIndex() as i32;
    srcCount = (*(*palStruct).template).srcCount() as i32;
    if srcIndex >= srcCount {
        if (*palStruct).countdown2 != 0 {
            (*palStruct).countdown2 -= 1;
        }
        (*palStruct).set_srcIndex(0);
    }
    *unkFlags |= shl_i32(1, (*palStruct).baseDestOffset() >> 4) as u32;
}
unsafe fn PaletteStruct_Blend(palStruct: *mut PaletteStruct, unkFlags: *mut u32) {
    if gPaletteFade.active() != 0
        && shl_i32(1, (*palStruct).baseDestOffset() >> 4) as u32 & gPaletteFade.multipurpose1 != 0
    {
        if (*(*palStruct).template).pst_field_8_0() == 0 {
            if gPaletteFade.delayCounter() as u16 != gPaletteFade.multipurpose2() {
                BlendPalette(
                    (*palStruct).baseDestOffset() as u16,
                    (*(*palStruct).template).size(),
                    gPaletteFade.y() as u8,
                    gPaletteFade.blendColor(),
                );
            }
        } else {
            if gPaletteFade.delayCounter() == 0
                && (*palStruct).countdown1 != (*(*palStruct).template).time1
            {
                let srcOffset: u32 =
                    (*palStruct).srcIndex() * (*(*palStruct).template).size() as u32;
                let mut i: u8 = 0;
                while (i as u16) < (*(*palStruct).template).size() {
                    gPlttBufferFaded[(*palStruct).baseDestOffset() + i as u32] =
                        *(*(*palStruct).template).src.at(srcOffset + i as u32);
                    i += 1;
                }
            }
        }
    }
}
unsafe fn PaletteStruct_TryEnd(pal: *mut PaletteStruct) {
    if (*pal).countdown2 == 0 {
        let state: i32 = (*(*pal).template).state() as i32;
        if state == 0 {
            (*pal).set_srcIndex(0);
            (*pal).countdown1 = (*(*pal).template).time1;
            (*pal).countdown2 = (*(*pal).template).time2;
            (*pal).set_destOffset((*pal).baseDestOffset());
        } else {
            if state < 0 {
                return;
            }
            if state > 2 {
                return;
            }
            PaletteStruct_ResetById((*(*pal).template).id);
        }
    } else {
        (*pal).countdown2 -= 1;
    }
}
pub unsafe fn PaletteStruct_ResetById(id: u16) {
    let paletteNum: u8 = PaletteStruct_GetPalNum(id);
    if paletteNum != NUM_PALETTE_STRUCTS {
        PaletteStruct_Reset(paletteNum);
    }
}
unsafe fn PaletteStruct_Reset(paletteNum: u8) {
    sPaletteStructs[paletteNum].template = (&raw const *sDummyPaletteStructTemplate).cast_mut();
    sPaletteStructs[paletteNum].set_active(FALSE as u32);
    sPaletteStructs[paletteNum].set_baseDestOffset(0);
    sPaletteStructs[paletteNum].set_destOffset(0);
    sPaletteStructs[paletteNum].set_srcIndex(0);
    sPaletteStructs[paletteNum].set_flag(0);
    sPaletteStructs[paletteNum].countdown1 = 0;
    sPaletteStructs[paletteNum].countdown2 = 0;
}
pub unsafe fn ResetPaletteFadeControl() {
    gPaletteFade.multipurpose1 = 0;
    gPaletteFade.set_multipurpose2(0);
    gPaletteFade.set_delayCounter(0);
    gPaletteFade.set_y(0);
    gPaletteFade.set_targetY(0);
    gPaletteFade.set_blendColor(0);
    gPaletteFade.set_active(FALSE as u16);
    gPaletteFade.set_multipurpose2(0);
    gPaletteFade.set_yDec(0);
    gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
    gPaletteFade.set_shouldResetBlendRegisters(FALSE as u16);
    gPaletteFade.set_hardwareFadeFinishing(FALSE as u16);
    gPaletteFade.set_softwareFadeFinishing(FALSE as u16);
    gPaletteFade.set_softwareFadeFinishingCounter(0);
    gPaletteFade.set_objPaletteToggle(0);
    gPaletteFade.set_deltaY(2);
}
unsafe fn PaletteStruct_SetUnusedFlag(id: u16) {
    let paletteNum: u8 = PaletteStruct_GetPalNum(id);
    if paletteNum != NUM_PALETTE_STRUCTS {
        sPaletteStructs[paletteNum].set_flag(TRUE as u32);
    }
}
unsafe fn PaletteStruct_ClearUnusedFlag(id: u16) {
    let paletteNum: u8 = PaletteStruct_GetPalNum(id);
    if paletteNum != NUM_PALETTE_STRUCTS {
        sPaletteStructs[paletteNum].set_flag(FALSE as u32);
    }
}
unsafe fn PaletteStruct_GetPalNum(id: u16) -> u8 {
    for i in 0..NUM_PALETTE_STRUCTS {
        if (*sPaletteStructs[i].template).id == id {
            return i;
        }
    }
    NUM_PALETTE_STRUCTS
}
unsafe fn UpdateNormalPaletteFade() -> u8 {
    let mut paletteOffset: u16 = 0;
    let mut selectedPalettes: u16 = 0;
    if gPaletteFade.active() == 0 {
        return PALETTE_FADE_STATUS_DONE as u8;
    }
    if IsSoftwarePaletteFadeFinishing() != 0 {
        return (if gPaletteFade.active() != 0 {
            PALETTE_FADE_STATUS_ACTIVE
        } else {
            PALETTE_FADE_STATUS_DONE
        }) as u8;
    } else {
        if gPaletteFade.objPaletteToggle() == 0 {
            if (gPaletteFade.delayCounter() as u16) < gPaletteFade.multipurpose2() {
                gPaletteFade.set_delayCounter(gPaletteFade.delayCounter() + 1);
                return 2;
            }
            gPaletteFade.set_delayCounter(0);
        }
        paletteOffset = 0;
        if gPaletteFade.objPaletteToggle() == 0 {
            selectedPalettes = gPaletteFade.multipurpose1 as u16;
        } else {
            selectedPalettes = (gPaletteFade.multipurpose1 >> 16) as u16;
            paletteOffset = OBJ_PLTT_OFFSET;
        }
        while selectedPalettes != 0 {
            if selectedPalettes as i32 & 1 != 0 {
                BlendPalette(
                    paletteOffset,
                    16,
                    gPaletteFade.y() as u8,
                    gPaletteFade.blendColor(),
                );
            }
            selectedPalettes >>= 1;
            paletteOffset += 16;
        }
        gPaletteFade.set_objPaletteToggle(gPaletteFade.objPaletteToggle() ^ 1);
        if gPaletteFade.objPaletteToggle() == 0 {
            if gPaletteFade.y() == gPaletteFade.targetY() {
                gPaletteFade.multipurpose1 = 0;
                gPaletteFade.set_softwareFadeFinishing(TRUE as u16);
            } else {
                let mut val: i8 = 0;
                if gPaletteFade.yDec() == 0 {
                    val = gPaletteFade.y() as i8;
                    val += gPaletteFade.deltaY() as i8;
                    if val as i32 > gPaletteFade.targetY() as i32 {
                        val = gPaletteFade.targetY() as i8;
                    }
                    gPaletteFade.set_y(val as u16);
                } else {
                    val = gPaletteFade.y() as i8;
                    val -= gPaletteFade.deltaY() as i8;
                    if (val as i32) < gPaletteFade.targetY() as i32 {
                        val = gPaletteFade.targetY() as i8;
                    }
                    gPaletteFade.set_y(val as u16);
                }
            }
        }
        return (if gPaletteFade.active() != 0 {
            PALETTE_FADE_STATUS_ACTIVE
        } else {
            PALETTE_FADE_STATUS_DONE
        }) as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn InvertPlttBuffer(mut selectedPalettes: u32) {
    let mut paletteOffset: u16 = 0;
    while selectedPalettes != 0 {
        if selectedPalettes & 1 != 0 {
            for i in 0..16u8 {
                gPlttBufferFaded[paletteOffset as i32 + i as i32] =
                    !gPlttBufferFaded[paletteOffset as i32 + i as i32];
            }
        }
        selectedPalettes >>= 1;
        paletteOffset += 16;
    }
}
pub unsafe fn TintPlttBuffer(mut selectedPalettes: u32, r: i8, g: i8, b: i8) {
    let mut paletteOffset: u16 = 0;
    while selectedPalettes != 0 {
        if selectedPalettes & 1 != 0 {
            for i in 0..16u8 {
                let data: *mut PlttData =
                    &raw mut gPlttBufferFaded[paletteOffset as i32 + i as i32] as *mut PlttData;
                (*data).set_r((*data).r() + r as u16);
                (*data).set_g((*data).g() + g as u16);
                (*data).set_b((*data).b() + b as u16);
            }
        }
        selectedPalettes >>= 1;
        paletteOffset += 16;
    }
}
pub unsafe fn UnfadePlttBuffer(mut selectedPalettes: u32) {
    let mut paletteOffset: u16 = 0;
    while selectedPalettes != 0 {
        if selectedPalettes & 1 != 0 {
            for i in 0..16u8 {
                gPlttBufferFaded[paletteOffset as i32 + i as i32] =
                    gPlttBufferUnfaded[paletteOffset as i32 + i as i32];
            }
        }
        selectedPalettes >>= 1;
        paletteOffset += 16;
    }
}
pub unsafe fn BeginFastPaletteFade(submode: u8) {
    gPaletteFade.set_deltaY(2);
    BeginFastPaletteFadeInternal(submode);
}
unsafe fn BeginFastPaletteFadeInternal(submode: u8) {
    gPaletteFade.set_y(31);
    gPaletteFade.set_multipurpose2(submode as u16 & 0x3F);
    gPaletteFade.set_active(TRUE as u16);
    gPaletteFade.set_mode(FAST_FADE);
    if submode == FAST_FADE_IN_FROM_BLACK as u8 {
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    gPlttBufferFaded.as_mut_ptr() as *mut c_void,
                    0x1000200,
                );
            }
        }
    }
    if submode == FAST_FADE_IN_FROM_WHITE as u8 {
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 32767);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    gPlttBufferFaded.as_mut_ptr() as *mut c_void,
                    0x1000200,
                );
            }
        }
    }
    UpdatePaletteFade();
}
unsafe fn UpdateFastPaletteFade() -> u8 {
    let mut paletteOffsetStart: u16 = 0;
    let mut paletteOffsetEnd: u16 = 0;
    let mut r0: i8 = 0;
    let mut g0: i8 = 0;
    let mut b0: i8 = 0;
    let mut r: i8 = 0;
    let mut g: i8 = 0;
    let mut b: i8 = 0;
    if gPaletteFade.active() == 0 {
        return PALETTE_FADE_STATUS_DONE as u8;
    }
    if IsSoftwarePaletteFadeFinishing() != 0 {
        return (if gPaletteFade.active() != 0 {
            PALETTE_FADE_STATUS_ACTIVE
        } else {
            PALETTE_FADE_STATUS_DONE
        }) as u8;
    }
    if gPaletteFade.objPaletteToggle() != 0 {
        paletteOffsetStart = OBJ_PLTT_OFFSET;
        paletteOffsetEnd = 512;
    } else {
        paletteOffsetStart = 0;
        paletteOffsetEnd = OBJ_PLTT_OFFSET;
    }
    match gPaletteFade.multipurpose2() {
        FAST_FADE_IN_FROM_WHITE => {
            for i in paletteOffsetStart..paletteOffsetEnd {
                let unfaded: *mut PlttData = &raw mut gPlttBufferUnfaded[i] as *mut PlttData;
                r0 = (*unfaded).r() as i8;
                g0 = (*unfaded).g() as i8;
                b0 = (*unfaded).b() as i8;
                let faded: *mut PlttData = &raw mut gPlttBufferFaded[i] as *mut PlttData;
                r = (*faded).r() as i8 - 2;
                g = (*faded).g() as i8 - 2;
                b = (*faded).b() as i8 - 2;
                if r < r0 {
                    r = r0;
                }
                if g < g0 {
                    g = g0;
                }
                if b < b0 {
                    b = b0;
                }
                gPlttBufferFaded[i] = r as u16 | (g as u16) << 5 | (b as u16) << 10;
            }
        }
        FAST_FADE_OUT_TO_WHITE => {
            for i in paletteOffsetStart..paletteOffsetEnd {
                let data: *mut PlttData = &raw mut gPlttBufferFaded[i] as *mut PlttData;
                r = (*data).r() as i8 + 2;
                g = (*data).g() as i8 + 2;
                b = (*data).b() as i8 + 2;
                if r > 31 {
                    r = 31;
                }
                if g > 31 {
                    g = 31;
                }
                if b > 31 {
                    b = 31;
                }
                gPlttBufferFaded[i] = r as u16 | (g as u16) << 5 | (b as u16) << 10;
            }
        }
        FAST_FADE_IN_FROM_BLACK => {
            for i in paletteOffsetStart..paletteOffsetEnd {
                let unfaded: *mut PlttData = &raw mut gPlttBufferUnfaded[i] as *mut PlttData;
                r0 = (*unfaded).r() as i8;
                g0 = (*unfaded).g() as i8;
                b0 = (*unfaded).b() as i8;
                let faded: *mut PlttData = &raw mut gPlttBufferFaded[i] as *mut PlttData;
                r = (*faded).r() as i8 + 2;
                g = (*faded).g() as i8 + 2;
                b = (*faded).b() as i8 + 2;
                if r > r0 {
                    r = r0;
                }
                if g > g0 {
                    g = g0;
                }
                if b > b0 {
                    b = b0;
                }
                gPlttBufferFaded[i] = r as u16 | (g as u16) << 5 | (b as u16) << 10;
            }
        }
        FAST_FADE_OUT_TO_BLACK => {
            for i in paletteOffsetStart..paletteOffsetEnd {
                let data: *mut PlttData = &raw mut gPlttBufferFaded[i] as *mut PlttData;
                r = (*data).r() as i8 - 2;
                g = (*data).g() as i8 - 2;
                b = (*data).b() as i8 - 2;
                if r < 0 {
                    r = 0;
                }
                if g < 0 {
                    g = 0;
                }
                if b < 0 {
                    b = 0;
                }
                gPlttBufferFaded[i] = r as u16 | (g as u16) << 5 | (b as u16) << 10;
            }
        }
        _ => {}
    }
    gPaletteFade.set_objPaletteToggle(gPaletteFade.objPaletteToggle() ^ 1);
    if gPaletteFade.objPaletteToggle() != 0 {
        return (if gPaletteFade.active() != 0 {
            PALETTE_FADE_STATUS_ACTIVE
        } else {
            PALETTE_FADE_STATUS_DONE
        }) as u8;
    }
    if (gPaletteFade.y() as i32 - gPaletteFade.deltaY() as i32) < 0 {
        gPaletteFade.set_y(0);
    } else {
        gPaletteFade.set_y(gPaletteFade.y() - gPaletteFade.deltaY() as u16);
    }
    if gPaletteFade.y() == 0 {
        match gPaletteFade.multipurpose2() {
            FAST_FADE_IN_FROM_WHITE | FAST_FADE_IN_FROM_BLACK => {
                CpuSet(
                    gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
                    gPlttBufferFaded.as_mut_ptr() as *mut c_void,
                    0x4000100,
                );
            }
            FAST_FADE_OUT_TO_WHITE => {
                let mut tmp: u32 = 0;
                volatile_write(&raw mut tmp, 0xffffffff);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    gPlttBufferFaded.as_mut_ptr() as *mut c_void,
                    0x5000100,
                );
            }
            FAST_FADE_OUT_TO_BLACK => {
                let mut tmp: u32 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    gPlttBufferFaded.as_mut_ptr() as *mut c_void,
                    0x5000100,
                );
            }
            _ => {}
        }
        gPaletteFade.set_mode(NORMAL_FADE);
        gPaletteFade.set_softwareFadeFinishing(TRUE as u16);
    }
    (if gPaletteFade.active() != 0 {
        PALETTE_FADE_STATUS_ACTIVE
    } else {
        PALETTE_FADE_STATUS_DONE
    }) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn BeginHardwarePaletteFade(
    blendCnt: u8,
    delay: u8,
    y: u8,
    targetY: u8,
    shouldResetBlendRegisters: u8,
) {
    gPaletteFade.multipurpose1 = blendCnt as u32;
    gPaletteFade.set_delayCounter(delay);
    gPaletteFade.set_multipurpose2(delay as u16);
    gPaletteFade.set_y(y as u16);
    gPaletteFade.set_targetY(targetY as u16);
    gPaletteFade.set_active(TRUE as u16);
    gPaletteFade.set_mode(HARDWARE_FADE);
    gPaletteFade.set_shouldResetBlendRegisters(shouldResetBlendRegisters as u16 & 1);
    gPaletteFade.set_hardwareFadeFinishing(FALSE as u16);
    if y < targetY {
        gPaletteFade.set_yDec(0);
    } else {
        gPaletteFade.set_yDec(1);
    }
}
unsafe fn UpdateHardwarePaletteFade() -> u8 {
    if gPaletteFade.active() == 0 {
        return PALETTE_FADE_STATUS_DONE as u8;
    }
    if (gPaletteFade.delayCounter() as u16) < gPaletteFade.multipurpose2() {
        gPaletteFade.set_delayCounter(gPaletteFade.delayCounter() + 1);
        return PALETTE_FADE_STATUS_DELAY;
    }
    gPaletteFade.set_delayCounter(0);
    if gPaletteFade.yDec() == 0 {
        gPaletteFade.set_y(gPaletteFade.y() + 1);
        if gPaletteFade.y() > gPaletteFade.targetY() {
            gPaletteFade.set_hardwareFadeFinishing(gPaletteFade.hardwareFadeFinishing() + 1);
            gPaletteFade.set_y(gPaletteFade.y() - 1);
        }
    } else {
        let y: i32 = ({
            let t1 = gPaletteFade.y();
            gPaletteFade.set_y(gPaletteFade.y() - 1);
            t1
        }) as i32;
        if y - 1 < gPaletteFade.targetY() as i32 {
            gPaletteFade.set_hardwareFadeFinishing(gPaletteFade.hardwareFadeFinishing() + 1);
            gPaletteFade.set_y(gPaletteFade.y() + 1);
        }
    }
    if gPaletteFade.hardwareFadeFinishing() != 0 {
        if gPaletteFade.shouldResetBlendRegisters() != 0 {
            gPaletteFade.multipurpose1 = 0;
            gPaletteFade.set_y(0);
        }
        gPaletteFade.set_shouldResetBlendRegisters(FALSE as u16);
    }
    (if gPaletteFade.active() != 0 {
        PALETTE_FADE_STATUS_ACTIVE
    } else {
        PALETTE_FADE_STATUS_DONE
    }) as u8
}
unsafe fn UpdateBlendRegisters() {
    SetGpuReg(REG_OFFSET_BLDCNT, gPaletteFade.multipurpose1 as u16);
    SetGpuReg(REG_OFFSET_BLDY, gPaletteFade.y());
    if gPaletteFade.hardwareFadeFinishing() != 0 {
        gPaletteFade.set_hardwareFadeFinishing(FALSE as u16);
        gPaletteFade.set_mode(0);
        gPaletteFade.multipurpose1 = 0;
        gPaletteFade.set_y(0);
        gPaletteFade.set_active(FALSE as u16);
    }
}
unsafe fn IsSoftwarePaletteFadeFinishing() -> u8 {
    if gPaletteFade.softwareFadeFinishing() != 0 {
        if gPaletteFade.softwareFadeFinishingCounter() == 4 {
            gPaletteFade.set_active(FALSE as u16);
            gPaletteFade.set_softwareFadeFinishing(FALSE as u16);
            gPaletteFade.set_softwareFadeFinishingCounter(0);
        } else {
            gPaletteFade
                .set_softwareFadeFinishingCounter(gPaletteFade.softwareFadeFinishingCounter() + 1);
        }
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BlendPalettes(mut selectedPalettes: u32, coeff: u8, color: u16) {
    let mut paletteOffset: u16 = 0;
    while selectedPalettes != 0 {
        if selectedPalettes & 1 != 0 {
            BlendPalette(paletteOffset, 16, coeff, color);
        }
        selectedPalettes >>= 1;
        paletteOffset += 16;
    }
}
pub unsafe fn BlendPalettesUnfaded(selectedPalettes: u32, coeff: u8, color: u16) {
    {
        let mut _src: *mut c_void = gPlttBufferUnfaded.as_mut_ptr() as *mut c_void;
        let mut _dest: *mut c_void = gPlttBufferFaded.as_mut_ptr() as *mut c_void;
        let mut _size: u32 = PLTT_SIZE;
        {
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, _src as usize as u32);
                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x84000000 | (_size / 4));
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    BlendPalettes(selectedPalettes, coeff, color);
}
pub unsafe fn TintPalette_GrayScale(mut palette: *mut u16, count: u16) {
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    let mut gray: u32 = 0;
    for i in 0..(count as i32) {
        r = *palette as i32 & 0x1F;
        g = (*palette >> 5) as i32 & 0x1F;
        b = (*palette >> 10) as i32 & 0x1F;
        gray = ((r * (0_f32 * 256_f32) as i16 as i32
            + g * (0_f32 * 256_f32) as i16 as i32
            + b * (0_f32 * 256_f32) as i16 as i32)
            >> 8) as u32;
        *({
            let t1 = palette;
            palette = palette.at(1);
            t1
        }) = (gray as u16) << 10 | (gray as u16) << 5 | gray as u16;
    }
}
pub unsafe fn TintPalette_GrayScale2(mut palette: *mut u16, count: u16) {
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    let mut gray: u32 = 0;
    for i in 0..(count as i32) {
        r = *palette as i32 & 0x1F;
        g = (*palette >> 5) as i32 & 0x1F;
        b = (*palette >> 10) as i32 & 0x1F;
        gray = ((r * (0_f32 * 256_f32) as i16 as i32
            + g * (0_f32 * 256_f32) as i16 as i32
            + b * (0_f32 * 256_f32) as i16 as i32)
            >> 8) as u32;
        if gray > 31 {
            gray = 31;
        }
        gray = sRoundedDownGrayscaleMap[gray] as u32;
        *({
            let t1 = palette;
            palette = palette.at(1);
            t1
        }) = (gray as u16) << 10 | (gray as u16) << 5 | gray as u16;
    }
}
pub unsafe fn TintPalette_SepiaTone(mut palette: *mut u16, count: u16) {
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    let mut gray: u32 = 0;
    for i in 0..(count as i32) {
        r = *palette as i32 & 0x1F;
        g = (*palette >> 5) as i32 & 0x1F;
        b = (*palette >> 10) as i32 & 0x1F;
        gray = ((r * (0_f32 * 256_f32) as i16 as i32
            + g * (0_f32 * 256_f32) as i16 as i32
            + b * (0_f32 * 256_f32) as i16 as i32)
            >> 8) as u32;
        r = ((1_f32 * 256_f32) as i16 as u16 as i32 * gray as u16 as i32) >> 8;
        g = ((1_f32 * 256_f32) as i16 as u16 as i32 * gray as u16 as i32) >> 8;
        b = ((0_f32 * 256_f32) as i16 as u16 as i32 * gray as u16 as i32) >> 8;
        if r > 31 {
            r = 31;
        }
        *({
            let t1 = palette;
            palette = palette.at(1);
            t1
        }) = (b as u16) << 10 | (g as u16) << 5 | r as u16;
    }
}
pub unsafe fn TintPalette_CustomTone(
    mut palette: *mut u16,
    count: u16,
    rTone: u16,
    gTone: u16,
    bTone: u16,
) {
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    let mut gray: u32 = 0;
    for i in 0..(count as i32) {
        r = *palette as i32 & 0x1F;
        g = (*palette >> 5) as i32 & 0x1F;
        b = (*palette >> 10) as i32 & 0x1F;
        gray = ((r * (0_f32 * 256_f32) as i16 as i32
            + g * (0_f32 * 256_f32) as i16 as i32
            + b * (0_f32 * 256_f32) as i16 as i32)
            >> 8) as u32;
        r = (rTone as i32 * gray as u16 as i32) >> 8;
        g = (gTone as i32 * gray as u16 as i32) >> 8;
        b = (bTone as i32 * gray as u16 as i32) >> 8;
        if r > 31 {
            r = 31;
        }
        if g > 31 {
            g = 31;
        }
        if b > 31 {
            b = 31;
        }
        *({
            let t1 = palette;
            palette = palette.at(1);
            t1
        }) = (b as u16) << 10 | (g as u16) << 5 | r as u16;
    }
}
pub unsafe fn BlendPalettesGradually(
    selectedPalettes: u32,
    delay: i8,
    coeff: u8,
    coeffTarget: u8,
    color: u16,
    priority: u8,
    id: u8,
) {
    let taskId: u8 = CreateTask(Some(Task_BlendPalettesGradually), priority);
    task_set(taskId, tCoeff, coeff as i16);
    task_set(taskId, tCoeffTarget, coeffTarget as i16);
    if delay >= 0 {
        task_set(taskId, tDelay, delay as i16);
        task_set(taskId, tCoeffDelta, 1);
    } else {
        task_set(taskId, tDelay, 0);
        task_set(taskId, tCoeffDelta, -(delay as i16) + 1);
    }
    if coeffTarget < coeff {
        task_set(taskId, tCoeffDelta, -task_get(taskId, tCoeffDelta));
    }
    SetWordTaskArg(taskId, tPalettes, selectedPalettes);
    task_set(taskId, tColor, color as i16);
    task_set(taskId, tId, id as i16);
    task_func(taskId).unwrap_unchecked()(taskId);
}
unsafe fn IsBlendPalettesGraduallyTaskActive(id: u8) -> u32 {
    for i in 0..NUM_TASKS {
        if (*gTasks.as_ptr())[i].isActive == TRUE
            && task_func(i) == Some(Task_BlendPalettesGradually as unsafe fn(u8))
            && task_get(i, tId) == id as i16
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn DestroyBlendPalettesGraduallyTask() {
    let mut taskId: u8 = 0;
    loop {
        taskId = FindTaskIdByFunc(Some(Task_BlendPalettesGradually));
        if taskId == TASK_NONE {
            break;
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_BlendPalettesGradually(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    let mut target: i16 = 0;
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let palettes: u32 = GetWordTaskArg(taskId, tPalettes);
    if ({
        *data.at(4) += 1;
        *data.at(4)
    }) > *data.at(3)
    {
        *data.at(4) = 0;
        BlendPalettes(palettes, *data as u8, *data.at(7) as u16);
        target = *data.at(1);
        if *data == target {
            DestroyTask(taskId);
        } else {
            *data += *data.at(2);
            if *data.at(2) >= 0 {
                if *data < target {
                    return;
                }
            } else if *data > target {
                return;
            }
            *data = target;
        }
    }
}
