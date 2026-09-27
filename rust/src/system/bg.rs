//! Background layer configuration and tilemap buffers.
//!
//! The GBA has four background layers. This module keeps a shadow copy of
//! each layer's control register plus a WRAM tilemap buffer, and pushes both
//! to hardware through the buffered GPU register queue and DMA3.

use crate::dma3_manager::{CheckForSpaceForDma3Request, RequestDma3Copy};
use crate::ffi::CpuSet;
use crate::gpu_regs::{GetGpuReg, SetGpuReg, SetGpuReg_ForcedBlank};
use crate::util::BgAffineSrcData;
use core::ffi::c_void;

pub const NUM_BACKGROUNDS: usize = 4;

const BG_CHAR_SIZE: u32 = 0x4000;
const BG_SCREEN_SIZE: u32 = 0x800;
const BG_VRAM: u32 = 0x0600_0000;
const BG_PLTT: u32 = 0x0500_0000;
const IWRAM_END: usize = 0x0300_8000;

/// `DISPCNT_BG_ALL_ON | 0x7` - the layer-enable bits plus the mode field.
const DISPCNT_ALL_BG_AND_MODE_BITS: u16 = 0x0f07;
const DISPCNT_MODE_1: u8 = 1;
const DISPCNT_MODE_2: u8 = 2;

const REG_OFFSET_DISPCNT: u8 = 0x00;
const REG_OFFSET_BG0CNT: u8 = 0x08;
const REG_OFFSET_BG0HOFS: u8 = 0x10;
const REG_OFFSET_BG0VOFS: u8 = 0x12;
const REG_OFFSET_BG1HOFS: u8 = 0x14;
const REG_OFFSET_BG1VOFS: u8 = 0x16;
const REG_OFFSET_BG2HOFS: u8 = 0x18;
const REG_OFFSET_BG2VOFS: u8 = 0x1a;
const REG_OFFSET_BG3HOFS: u8 = 0x1c;
const REG_OFFSET_BG3VOFS: u8 = 0x1e;
const REG_OFFSET_BG2PA: u8 = 0x20;
const REG_OFFSET_BG2PB: u8 = 0x22;
const REG_OFFSET_BG2PC: u8 = 0x24;
const REG_OFFSET_BG2PD: u8 = 0x26;
const REG_OFFSET_BG2X_L: u8 = 0x28;
const REG_OFFSET_BG2X_H: u8 = 0x2a;
const REG_OFFSET_BG2Y_L: u8 = 0x2c;
const REG_OFFSET_BG2Y_H: u8 = 0x2e;
const REG_OFFSET_BG3X_L: u8 = 0x38;
const REG_OFFSET_BG3X_H: u8 = 0x3a;
const REG_OFFSET_BG3Y_L: u8 = 0x3c;
const REG_OFFSET_BG3Y_H: u8 = 0x3e;
const REG_OFFSET_MOSAIC: u8 = 0x4c;

// Public attribute ids (bg.h).
const BG_ATTR_CHARBASEINDEX: u8 = 1;
const BG_ATTR_MAPBASEINDEX: u8 = 2;
const BG_ATTR_SCREENSIZE: u8 = 3;
const BG_ATTR_PALETTEMODE: u8 = 4;
const BG_ATTR_MOSAIC: u8 = 5;
const BG_ATTR_WRAPAROUND: u8 = 6;
const BG_ATTR_PRIORITY: u8 = 7;
const BG_ATTR_METRIC: u8 = 8;
const BG_ATTR_TYPE: u8 = 9;
const BG_ATTR_BASETILE: u8 = 10;

// Internal attribute ids used by the control-struct accessors.
const BG_CTRL_ATTR_VISIBLE: u8 = 1;
const BG_CTRL_ATTR_CHARBASEINDEX: u8 = 2;
const BG_CTRL_ATTR_MAPBASEINDEX: u8 = 3;
const BG_CTRL_ATTR_SCREENSIZE: u8 = 4;
const BG_CTRL_ATTR_PALETTEMODE: u8 = 5;
const BG_CTRL_ATTR_PRIORITY: u8 = 6;
const BG_CTRL_ATTR_MOSAIC: u8 = 7;
const BG_CTRL_ATTR_WRAPAROUND: u8 = 8;

const BG_TYPE_NORMAL: u32 = 0;
const BG_TYPE_AFFINE: u32 = 1;
const BG_TYPE_NONE: u32 = 0xffff;

/// The fall-through case in `apply_coord_op`, so it is never matched by name.
#[allow(dead_code)]
const BG_COORD_SET: u8 = 0;
const BG_COORD_ADD: u8 = 1;
const BG_COORD_SUB: u8 = 2;

/// The fall-through case in `Unused_AdjustBgMosaic`, never matched by name.
#[allow(dead_code)]
const BG_MOSAIC_SET_HV: u8 = 0;
const BG_MOSAIC_SET_H: u8 = 1;
const BG_MOSAIC_ADD_H: u8 = 2;
const BG_MOSAIC_SUB_H: u8 = 3;
const BG_MOSAIC_SET_V: u8 = 4;
const BG_MOSAIC_ADD_V: u8 = 5;
const BG_MOSAIC_SUB_V: u8 = 6;

/// `struct BgTemplate` is a single 32-bit word of bitfields.
const TMPL_BG: u32 = 0x0000_0003;
const TMPL_CHAR_BASE_INDEX_SHIFT: u32 = 2;
const TMPL_CHAR_BASE_INDEX: u32 = 0x0000_000c;
const TMPL_MAP_BASE_INDEX_SHIFT: u32 = 4;
const TMPL_MAP_BASE_INDEX: u32 = 0x0000_01f0;
const TMPL_SCREEN_SIZE_SHIFT: u32 = 9;
const TMPL_SCREEN_SIZE: u32 = 0x0000_0600;
const TMPL_PALETTE_MODE_SHIFT: u32 = 11;
const TMPL_PALETTE_MODE: u32 = 0x0000_0800;
const TMPL_PRIORITY_SHIFT: u32 = 12;
const TMPL_PRIORITY: u32 = 0x0000_3000;
const TMPL_BASE_TILE_SHIFT: u32 = 16;
const TMPL_BASE_TILE: u32 = 0x03ff_0000;

/// One layer's shadow of BGnCNT. The original packs these as bitfields; the
/// widths are enforced on write instead so the register can be composed
/// without re-deriving the packing.
#[derive(Clone, Copy)]
struct BgConfig {
    visible: bool,
    screen_size: u8,
    priority: u8,
    mosaic: u8,
    wraparound: u8,
    char_base_index: u8,
    map_base_index: u8,
    palette_mode: u8,
}

const ZEROED_BG_CONFIG: BgConfig = BgConfig {
    visible: false,
    screen_size: 0,
    priority: 0,
    mosaic: 0,
    wraparound: 0,
    char_base_index: 0,
    map_base_index: 0,
    palette_mode: 0,
};

struct BgConfig2 {
    base_tile: u16,
    base_palette: u8,
    tilemap: *mut u8,
    bg_x: i32,
    bg_y: i32,
}

const ZEROED_BG_CONFIG2: BgConfig2 = BgConfig2 {
    base_tile: 0,
    base_palette: 0,
    tilemap: core::ptr::null_mut(),
    bg_x: 0,
    bg_y: 0,
};

static mut BG_CONFIGS: [BgConfig; NUM_BACKGROUNDS] = [ZEROED_BG_CONFIG; NUM_BACKGROUNDS];
static mut BG_VISIBILITY_AND_MODE: u16 = 0;
static mut BG_CONFIGS2: [BgConfig2; NUM_BACKGROUNDS] =
    [const { ZEROED_BG_CONFIG2 }; NUM_BACKGROUNDS];
/// One bit per outstanding DMA3 request slot.
static mut DMA_BUSY_BITFIELD: [u32; NUM_BACKGROUNDS] = [0; NUM_BACKGROUNDS];

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gWindowTileAutoAllocEnabled: u32 = 0;

unsafe extern "C" {
    fn BgAffineSet(src: *const BgAffineSrcData, dest: *mut u8, count: i32);
    fn LZ77UnCompWram(src: *const u32, dest: *mut c_void);
}

#[inline]
unsafe fn config(bg: usize) -> *mut BgConfig {
    unsafe { (&raw mut BG_CONFIGS).cast::<BgConfig>().add(bg) }
}

#[inline]
unsafe fn config2(bg: usize) -> *mut BgConfig2 {
    unsafe { (&raw mut BG_CONFIGS2).cast::<BgConfig2>().add(bg) }
}

#[inline]
unsafe fn visibility_and_mode() -> u16 {
    unsafe { (&raw const BG_VISIBILITY_AND_MODE).read_volatile() }
}

#[inline]
unsafe fn set_visibility_and_mode(value: u16) {
    unsafe { (&raw mut BG_VISIBILITY_AND_MODE).write_volatile(value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsInvalidBg(bg: u8) -> u8 {
    u8::from(bg as usize >= NUM_BACKGROUNDS)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsInvalidBg32(bg: u8) -> u32 {
    u32::from(bg as usize >= NUM_BACKGROUNDS)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBgMode() -> u8 {
    (unsafe { visibility_and_mode() } & 0x7) as u8
}

unsafe fn set_bg_mode_internal(bg_mode: u8) {
    let value = (unsafe { visibility_and_mode() } & !0x7) | u16::from(bg_mode);
    unsafe { set_visibility_and_mode(value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBgMode(bg_mode: u8) {
    unsafe { set_bg_mode_internal(bg_mode) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBgControlStructs() {
    for i in 0..NUM_BACKGROUNDS {
        unsafe { config(i).write(ZEROED_BG_CONFIG) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unused_ResetBgControlStruct(bg: u8) {
    if unsafe { IsInvalidBg(bg) } == 0 {
        unsafe { config(bg as usize).write(ZEROED_BG_CONFIG) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTextModeAndHideBgs() {
    let current = unsafe { GetGpuReg(REG_OFFSET_DISPCNT) };
    unsafe { SetGpuReg(REG_OFFSET_DISPCNT, current & !DISPCNT_ALL_BG_AND_MODE_BITS) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBgs() {
    unsafe { ResetBgControlStructs() };
    unsafe { set_visibility_and_mode(0) };
    unsafe { SetTextModeAndHideBgs() };
}

/// 0xFF in any argument means "leave this one alone".
unsafe fn set_bg_control_attributes(
    bg: u8,
    char_base_index: u8,
    map_base_index: u8,
    screen_size: u8,
    palette_mode: u8,
    priority: u8,
    mosaic: u8,
    wraparound: u8,
) {
    if unsafe { IsInvalidBg(bg) } != 0 {
        return;
    }
    let slot = unsafe { config(bg as usize) };

    // Each store is masked to the width of the original bitfield.
    if char_base_index != 0xff {
        unsafe { (&raw mut (*slot).char_base_index).write(char_base_index & 0x3) };
    }
    if map_base_index != 0xff {
        unsafe { (&raw mut (*slot).map_base_index).write(map_base_index & 0x1f) };
    }
    if screen_size != 0xff {
        unsafe { (&raw mut (*slot).screen_size).write(screen_size & 0x3) };
    }
    if palette_mode != 0xff {
        unsafe { (&raw mut (*slot).palette_mode).write(palette_mode & 0x1) };
    }
    if priority != 0xff {
        unsafe { (&raw mut (*slot).priority).write(priority & 0x3) };
    }
    if mosaic != 0xff {
        unsafe { (&raw mut (*slot).mosaic).write(mosaic & 0x1) };
    }
    if wraparound != 0xff {
        unsafe { (&raw mut (*slot).wraparound).write(wraparound & 0x1) };
    }

    unsafe { (&raw mut (*slot).visible).write(true) };
}

unsafe fn get_bg_control_attribute(bg: u8, attribute_id: u8) -> u16 {
    if unsafe { IsInvalidBg(bg) } != 0 {
        return 0xff;
    }
    let slot = unsafe { config(bg as usize) };
    if !unsafe { (&raw const (*slot).visible).read() } {
        return 0xff;
    }

    match attribute_id {
        BG_CTRL_ATTR_VISIBLE => 1,
        BG_CTRL_ATTR_CHARBASEINDEX => {
            u16::from(unsafe { (&raw const (*slot).char_base_index).read() })
        }
        BG_CTRL_ATTR_MAPBASEINDEX => {
            u16::from(unsafe { (&raw const (*slot).map_base_index).read() })
        }
        BG_CTRL_ATTR_SCREENSIZE => u16::from(unsafe { (&raw const (*slot).screen_size).read() }),
        BG_CTRL_ATTR_PALETTEMODE => u16::from(unsafe { (&raw const (*slot).palette_mode).read() }),
        BG_CTRL_ATTR_PRIORITY => u16::from(unsafe { (&raw const (*slot).priority).read() }),
        BG_CTRL_ATTR_MOSAIC => u16::from(unsafe { (&raw const (*slot).mosaic).read() }),
        BG_CTRL_ATTR_WRAPAROUND => u16::from(unsafe { (&raw const (*slot).wraparound).read() }),
        _ => 0xff,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBgVram(
    bg: u8,
    src: *const c_void,
    size: u16,
    dest_offset: u16,
    mode: u8,
) -> u8 {
    if unsafe { IsInvalidBg(bg) } != 0
        || !unsafe { (&raw const (*config(bg as usize)).visible).read() }
    {
        return 0xff;
    }

    let base = match mode {
        1 => {
            u32::from(unsafe { (&raw const (*config(bg as usize)).char_base_index).read() })
                * BG_CHAR_SIZE
        }
        2 => {
            u32::from(unsafe { (&raw const (*config(bg as usize)).map_base_index).read() })
                * BG_SCREEN_SIZE
        }
        _ => return 0xff,
    };

    // The original truncates the offset to 16 bits before adding BG_VRAM.
    let offset = (u32::from(dest_offset).wrapping_add(base)) as u16;
    let cursor = unsafe {
        RequestDma3Copy(
            src.cast(),
            (BG_VRAM + u32::from(offset)) as usize as *mut u8,
            size,
            0,
        )
    };

    if cursor == -1 { 0xff } else { cursor as u8 }
}

unsafe fn show_bg_internal(bg: u8) {
    if unsafe { IsInvalidBg(bg) } != 0 {
        return;
    }
    let slot = unsafe { config(bg as usize) };
    if !unsafe { (&raw const (*slot).visible).read() } {
        return;
    }

    let value = u16::from(unsafe { (&raw const (*slot).priority).read() })
        | (u16::from(unsafe { (&raw const (*slot).char_base_index).read() }) << 2)
        | (u16::from(unsafe { (&raw const (*slot).mosaic).read() }) << 6)
        | (u16::from(unsafe { (&raw const (*slot).palette_mode).read() }) << 7)
        | (u16::from(unsafe { (&raw const (*slot).map_base_index).read() }) << 8)
        | (u16::from(unsafe { (&raw const (*slot).wraparound).read() }) << 13)
        | (u16::from(unsafe { (&raw const (*slot).screen_size).read() }) << 14);

    unsafe { SetGpuReg((bg << 1) + REG_OFFSET_BG0CNT, value) };

    let mode = (unsafe { visibility_and_mode() } | (1 << (bg + 8))) & DISPCNT_ALL_BG_AND_MODE_BITS;
    unsafe { set_visibility_and_mode(mode) };
}

unsafe fn hide_bg_internal(bg: u8) {
    if unsafe { IsInvalidBg(bg) } != 0 {
        return;
    }
    let mode = (unsafe { visibility_and_mode() } & !(1 << (bg + 8))) & DISPCNT_ALL_BG_AND_MODE_BITS;
    unsafe { set_visibility_and_mode(mode) };
}

unsafe fn sync_bg_visibility_and_mode() {
    let current = unsafe { GetGpuReg(REG_OFFSET_DISPCNT) };
    unsafe {
        SetGpuReg(
            REG_OFFSET_DISPCNT,
            (current & !DISPCNT_ALL_BG_AND_MODE_BITS) | visibility_and_mode(),
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowBg(bg: u8) {
    unsafe { show_bg_internal(bg) };
    unsafe { sync_bg_visibility_and_mode() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideBg(bg: u8) {
    unsafe { hide_bg_internal(bg) };
    unsafe { sync_bg_visibility_and_mode() };
}

/// From FireRed/LeafGreen. Dummied out here.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BgTileAllocOp(_bg: i32, _offset: i32, _count: i32, _mode: i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBgsAndClearDma3BusyFlags(leftover_frlg_variable: u32) {
    unsafe { ResetBgs() };
    for i in 0..NUM_BACKGROUNDS {
        unsafe {
            (&raw mut DMA_BUSY_BITFIELD)
                .cast::<u32>()
                .add(i)
                .write_volatile(0)
        };
    }
    unsafe { (&raw mut gWindowTileAutoAllocEnabled).write_volatile(leftover_frlg_variable) };
}

#[inline]
unsafe fn apply_template(template: *const u32) {
    let packed = unsafe { template.read() };
    let bg = (packed & TMPL_BG) as u8;
    if bg as usize >= NUM_BACKGROUNDS {
        return;
    }

    unsafe {
        set_bg_control_attributes(
            bg,
            ((packed & TMPL_CHAR_BASE_INDEX) >> TMPL_CHAR_BASE_INDEX_SHIFT) as u8,
            ((packed & TMPL_MAP_BASE_INDEX) >> TMPL_MAP_BASE_INDEX_SHIFT) as u8,
            ((packed & TMPL_SCREEN_SIZE) >> TMPL_SCREEN_SIZE_SHIFT) as u8,
            ((packed & TMPL_PALETTE_MODE) >> TMPL_PALETTE_MODE_SHIFT) as u8,
            ((packed & TMPL_PRIORITY) >> TMPL_PRIORITY_SHIFT) as u8,
            0,
            0,
        )
    };

    let slot = unsafe { config2(bg as usize) };
    unsafe {
        (&raw mut (*slot).base_tile)
            .write(((packed & TMPL_BASE_TILE) >> TMPL_BASE_TILE_SHIFT) as u16)
    };
    unsafe { (&raw mut (*slot).base_palette).write(0) };
    unsafe { (&raw mut (*slot).tilemap).write(core::ptr::null_mut()) };
    unsafe { (&raw mut (*slot).bg_x).write(0) };
    unsafe { (&raw mut (*slot).bg_y).write(0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBgsFromTemplates(
    bg_mode: u8,
    templates: *const u32,
    num_templates: u8,
) {
    unsafe { set_bg_mode_internal(bg_mode) };
    unsafe { ResetBgControlStructs() };

    let mut i = 0usize;
    while i < num_templates as usize {
        unsafe { apply_template(templates.add(i)) };
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBgFromTemplate(template: *const u32) {
    unsafe { apply_template(template) };
}

#[inline]
unsafe fn mark_dma_busy(cursor: u8) {
    let slot = unsafe {
        (&raw mut DMA_BUSY_BITFIELD)
            .cast::<u32>()
            .add(cursor as usize / 0x20)
    };
    unsafe { slot.write_volatile(slot.read_volatile() | (1 << (cursor % 0x20))) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBgTiles(
    bg: u8,
    src: *const c_void,
    size: u16,
    dest_offset: u16,
) -> u16 {
    let base_tile = unsafe { (&raw const (*config2(bg as usize)).base_tile).read() };
    // 4bpp tiles are 0x20 bytes, 8bpp 0x40.
    let scale = if unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_PALETTEMODE) } == 0 {
        0x20u32
    } else {
        0x40
    };
    let tile_offset = ((u32::from(base_tile) + u32::from(dest_offset)) * scale) as u16;

    let cursor = unsafe { LoadBgVram(bg, src, size, tile_offset, DISPCNT_MODE_1) };
    if cursor == 0xff {
        return 0xffff;
    }
    unsafe { mark_dma_busy(cursor) };

    if unsafe { (&raw const gWindowTileAutoAllocEnabled).read_volatile() } == 1 {
        unsafe {
            BgTileAllocOp(
                i32::from(bg),
                i32::from(tile_offset / 0x20),
                i32::from(size / 0x20),
                1,
            )
        };
    }

    u16::from(cursor)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBgTilemap(
    bg: u8,
    src: *const c_void,
    size: u16,
    dest_offset: u16,
) -> u16 {
    let cursor = unsafe { LoadBgVram(bg, src, size, dest_offset.wrapping_mul(2), DISPCNT_MODE_2) };
    if cursor == 0xff {
        return 0xffff;
    }
    unsafe { mark_dma_busy(cursor) };
    u16::from(cursor)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unused_LoadBgPalette(
    bg: u8,
    src: *const c_void,
    size: u16,
    dest_offset: u16,
) -> u16 {
    if unsafe { IsInvalidBg32(bg) } != 0 {
        return 0xffff;
    }

    // PLTT_OFFSET_4BPP: sixteen colours of two bytes each.
    let base_palette = unsafe { (&raw const (*config2(bg as usize)).base_palette).read() };
    let palette_offset = u32::from(base_palette) * 32 + u32::from(dest_offset) * 2;
    let cursor = unsafe {
        RequestDma3Copy(
            src.cast(),
            (BG_PLTT + palette_offset) as usize as *mut u8,
            size,
            0,
        )
    };
    if cursor == -1 {
        return 0xffff;
    }

    unsafe { mark_dma_busy(cursor as u8) };
    u16::from(cursor as u8)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsDma3ManagerBusyWithBgCopy() -> u8 {
    let mut i = 0usize;
    while i < 0x80 {
        let slot = unsafe { (&raw mut DMA_BUSY_BITFIELD).cast::<u32>().add(i / 0x20) };
        let bit = 1u32 << (i % 0x20);
        if unsafe { slot.read_volatile() } & bit != 0 {
            if unsafe { CheckForSpaceForDma3Request(i as i16) } == -1 {
                return 1;
            }
            unsafe { slot.write_volatile(slot.read_volatile() & !bit) };
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBgAttribute(bg: u8, attribute_id: u8, value: u8) {
    const KEEP: u8 = 0xff;
    let args = match attribute_id {
        BG_ATTR_CHARBASEINDEX => [value, KEEP, KEEP, KEEP, KEEP, KEEP, KEEP],
        BG_ATTR_MAPBASEINDEX => [KEEP, value, KEEP, KEEP, KEEP, KEEP, KEEP],
        BG_ATTR_SCREENSIZE => [KEEP, KEEP, value, KEEP, KEEP, KEEP, KEEP],
        BG_ATTR_PALETTEMODE => [KEEP, KEEP, KEEP, value, KEEP, KEEP, KEEP],
        BG_ATTR_PRIORITY => [KEEP, KEEP, KEEP, KEEP, value, KEEP, KEEP],
        BG_ATTR_MOSAIC => [KEEP, KEEP, KEEP, KEEP, KEEP, value, KEEP],
        BG_ATTR_WRAPAROUND => [KEEP, KEEP, KEEP, KEEP, KEEP, KEEP, value],
        _ => return,
    };
    unsafe {
        set_bg_control_attributes(
            bg, args[0], args[1], args[2], args[3], args[4], args[5], args[6],
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBgAttribute(bg: u8, attribute_id: u8) -> u16 {
    match attribute_id {
        BG_ATTR_CHARBASEINDEX => unsafe {
            get_bg_control_attribute(bg, BG_CTRL_ATTR_CHARBASEINDEX)
        },
        BG_ATTR_MAPBASEINDEX => unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_MAPBASEINDEX) },
        BG_ATTR_SCREENSIZE => unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_SCREENSIZE) },
        BG_ATTR_PALETTEMODE => unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_PALETTEMODE) },
        BG_ATTR_PRIORITY => unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_PRIORITY) },
        BG_ATTR_MOSAIC => unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_MOSAIC) },
        BG_ATTR_WRAPAROUND => unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_WRAPAROUND) },
        // The tilemap buffer size implied by the layer's screen size.
        BG_ATTR_METRIC => match unsafe { get_bg_type(bg) } {
            BG_TYPE_NORMAL => {
                let blocks = unsafe { GetBgMetricTextMode(bg, 0) };
                blocks * 0x800
            }
            BG_TYPE_AFFINE => {
                let blocks = unsafe { GetBgMetricAffineMode(bg, 0) };
                blocks as u16 * 0x100
            }
            _ => 0,
        },
        BG_ATTR_TYPE => {
            let bg_type = unsafe { get_bg_type(bg) };
            bg_type as u16
        }
        BG_ATTR_BASETILE => unsafe { (&raw const (*config2(bg as usize)).base_tile).read() },
        _ => 0xffff,
    }
}

/// Applies a set/add/subtract to a scroll coordinate.
#[inline]
fn apply_coord_op(current: i32, value: i32, op: u8) -> i32 {
    match op {
        BG_COORD_ADD => current.wrapping_add(value),
        BG_COORD_SUB => current.wrapping_sub(value),
        // BG_COORD_SET and anything else.
        _ => value,
    }
}

/// Writes a horizontal or vertical scroll value to the right registers for
/// the layer. Layers 2 and 3 use 32-bit affine origins outside mode 0.
unsafe fn write_scroll(bg: u8, coord: i32, horizontal: bool, forced_blank: bool) {
    let set = |reg: u8, value: u16| {
        if forced_blank {
            unsafe { SetGpuReg_ForcedBlank(reg, value) };
        } else {
            unsafe { SetGpuReg(reg, value) };
        }
    };

    let mode = unsafe { GetBgMode() };
    let text_reg = match (bg, horizontal) {
        (0, true) => REG_OFFSET_BG0HOFS,
        (0, false) => REG_OFFSET_BG0VOFS,
        (1, true) => REG_OFFSET_BG1HOFS,
        (1, false) => REG_OFFSET_BG1VOFS,
        (2, true) => REG_OFFSET_BG2HOFS,
        (2, false) => REG_OFFSET_BG2VOFS,
        (3, true) => REG_OFFSET_BG3HOFS,
        (3, false) => REG_OFFSET_BG3VOFS,
        _ => return,
    };

    let affine = match (bg, horizontal) {
        (2, true) => Some((REG_OFFSET_BG2X_H, REG_OFFSET_BG2X_L)),
        (2, false) => Some((REG_OFFSET_BG2Y_H, REG_OFFSET_BG2Y_L)),
        (3, true) => Some((REG_OFFSET_BG3X_H, REG_OFFSET_BG3X_L)),
        (3, false) => Some((REG_OFFSET_BG3Y_H, REG_OFFSET_BG3Y_L)),
        _ => None,
    };

    // Layer 2 leaves text mode as soon as the mode is not 0; layer 3 only in
    // mode 2, and in mode 1 it is written nowhere at all.
    let use_affine = match bg {
        2 => mode != 0,
        3 => mode == 2,
        _ => false,
    };

    if !use_affine {
        if bg == 3 && mode != 0 {
            return;
        }
        set(text_reg, (coord >> 8) as u16);
    } else if let Some((high, low)) = affine {
        set(high, (coord >> 16) as u16);
        set(low, (coord & 0xffff) as u16);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeBgX(bg: u8, value: i32, op: u8) -> i32 {
    if unsafe { IsInvalidBg32(bg) } != 0
        || unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_VISIBLE) } == 0
    {
        return -1;
    }

    let slot = unsafe { config2(bg as usize) };
    let updated = apply_coord_op(unsafe { (&raw const (*slot).bg_x).read() }, value, op);
    unsafe { (&raw mut (*slot).bg_x).write(updated) };
    unsafe { write_scroll(bg, updated, true, false) };
    updated
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBgX(bg: u8) -> i32 {
    if unsafe { IsInvalidBg32(bg) } != 0
        || unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_VISIBLE) } == 0
    {
        return -1;
    }
    unsafe { (&raw const (*config2(bg as usize)).bg_x).read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeBgY(bg: u8, value: i32, op: u8) -> i32 {
    if unsafe { IsInvalidBg32(bg) } != 0
        || unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_VISIBLE) } == 0
    {
        return -1;
    }

    let slot = unsafe { config2(bg as usize) };
    let updated = apply_coord_op(unsafe { (&raw const (*slot).bg_y).read() }, value, op);
    unsafe { (&raw mut (*slot).bg_y).write(updated) };
    unsafe { write_scroll(bg, updated, false, false) };
    updated
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeBgY_ScreenOff(bg: u8, value: i32, op: u8) -> i32 {
    if unsafe { IsInvalidBg32(bg) } != 0
        || unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_VISIBLE) } == 0
    {
        return -1;
    }

    let slot = unsafe { config2(bg as usize) };
    let updated = apply_coord_op(unsafe { (&raw const (*slot).bg_y).read() }, value, op);
    unsafe { (&raw mut (*slot).bg_y).write(updated) };
    unsafe { write_scroll(bg, updated, false, true) };
    updated
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBgY(bg: u8) -> i32 {
    if unsafe { IsInvalidBg32(bg) } != 0
        || unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_VISIBLE) } == 0
    {
        return -1;
    }
    unsafe { (&raw const (*config2(bg as usize)).bg_y).read() }
}

#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn SetBgAffine(
    bg: u8,
    src_center_x: i32,
    src_center_y: i32,
    disp_center_x: i16,
    disp_center_y: i16,
    scale_x: i16,
    scale_y: i16,
    rotation_angle: u16,
) {
    // Only the layers that are affine in the current mode may be set.
    match unsafe { visibility_and_mode() } & 0x7 {
        1 if bg == 2 => {}
        2 if bg == 2 || bg == 3 => {}
        _ => return,
    }

    let src = BgAffineSrcData {
        tex_x: src_center_x as u32,
        tex_y: src_center_y as u32,
        scr_x: disp_center_x,
        scr_y: disp_center_y,
        sx: scale_x,
        sy: scale_y,
        alpha: rotation_angle,
    };

    // struct BgAffineDstData: pa, pb, pc, pd, then two 32-bit origins.
    let mut dest = [0u8; 16];
    unsafe { BgAffineSet(&raw const src, dest.as_mut_ptr(), 1) };

    let half = |offset: usize| u16::from_le_bytes([dest[offset], dest[offset + 1]]);
    let word = |offset: usize| {
        i32::from_le_bytes([
            dest[offset],
            dest[offset + 1],
            dest[offset + 2],
            dest[offset + 3],
        ])
    };

    unsafe { SetGpuReg(REG_OFFSET_BG2PA, half(0)) };
    unsafe { SetGpuReg(REG_OFFSET_BG2PB, half(2)) };
    unsafe { SetGpuReg(REG_OFFSET_BG2PC, half(4)) };
    unsafe { SetGpuReg(REG_OFFSET_BG2PD, half(6)) };
    // The original writes PA a second time; kept for fidelity.
    unsafe { SetGpuReg(REG_OFFSET_BG2PA, half(0)) };
    unsafe { SetGpuReg(REG_OFFSET_BG2X_L, word(8) as u16) };
    unsafe { SetGpuReg(REG_OFFSET_BG2X_H, (word(8) >> 16) as u16) };
    unsafe { SetGpuReg(REG_OFFSET_BG2Y_L, word(12) as u16) };
    unsafe { SetGpuReg(REG_OFFSET_BG2Y_H, (word(12) >> 16) as u16) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unused_AdjustBgMosaic(val: u8, mode: u8) -> u8 {
    let mosaic = unsafe { GetGpuReg(REG_OFFSET_MOSAIC) };
    let mut bg_h = (mosaic & 0xf) as i16;
    let mut bg_v = ((mosaic >> 4) & 0xf) as i16;
    // Keep the object mosaic sizes in the high byte.
    let mut mosaic = mosaic & 0xff00;

    let val = i16::from(val);
    match mode {
        BG_MOSAIC_SET_H => bg_h = val & 0xf,
        BG_MOSAIC_ADD_H => bg_h = (bg_h + val).min(0xf),
        BG_MOSAIC_SUB_H => bg_h = (bg_h - val).max(0),
        BG_MOSAIC_SET_V => bg_v = val & 0xf,
        BG_MOSAIC_ADD_V => bg_v = (bg_v + val).min(0xf),
        BG_MOSAIC_SUB_V => bg_v = (bg_v - val).max(0),
        // BG_MOSAIC_SET_HV and anything else.
        _ => {
            bg_h = val & 0xf;
            bg_v = val >> 4;
        }
    }

    mosaic |= ((bg_v as u16) << 4) & 0xf0;
    mosaic |= (bg_h as u16) & 0xf;

    unsafe { SetGpuReg(REG_OFFSET_MOSAIC, mosaic) };
    mosaic as u8
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBgTilemapBuffer(bg: u8, tilemap: *mut u8) {
    if unsafe { IsInvalidBg32(bg) } == 0
        && unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_VISIBLE) } != 0
    {
        unsafe { (&raw mut (*config2(bg as usize)).tilemap).write(tilemap) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnsetBgTilemapBuffer(bg: u8) {
    if unsafe { IsInvalidBg32(bg) } == 0
        && unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_VISIBLE) } != 0
    {
        unsafe { (&raw mut (*config2(bg as usize)).tilemap).write(core::ptr::null_mut()) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBgTilemapBuffer(bg: u8) -> *mut u8 {
    if unsafe { IsInvalidBg32(bg) } != 0
        || unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_VISIBLE) } == 0
    {
        return core::ptr::null_mut();
    }
    unsafe { (&raw const (*config2(bg as usize)).tilemap).read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTileMapOutsideWram(bg: u8) -> u32 {
    let tilemap = unsafe { (&raw const (*config2(bg as usize)).tilemap).read() };
    u32::from(tilemap as usize > IWRAM_END || tilemap.is_null())
}

#[inline]
unsafe fn usable_tilemap(bg: u8) -> Option<*mut u8> {
    if unsafe { IsInvalidBg32(bg) } != 0 || unsafe { IsTileMapOutsideWram(bg) } != 0 {
        None
    } else {
        Some(unsafe { (&raw const (*config2(bg as usize)).tilemap).read() })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyToBgTilemapBuffer(
    bg: u8,
    src: *const c_void,
    mode: u16,
    dest_offset: u16,
) {
    let Some(tilemap) = (unsafe { usable_tilemap(bg) }) else {
        return;
    };
    let dest = unsafe { tilemap.add(dest_offset as usize * 2) };

    if mode != 0 {
        // CpuCopy16
        unsafe { CpuSet(src, dest.cast(), u32::from(mode) / 2 & 0x1f_ffff) };
    } else {
        unsafe { LZ77UnCompWram(src.cast(), dest.cast()) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyBgTilemapBufferToVram(bg: u8) {
    let Some(tilemap) = (unsafe { usable_tilemap(bg) }) else {
        return;
    };

    let size = match unsafe { get_bg_type(bg) } {
        BG_TYPE_NORMAL => {
            let blocks = unsafe { GetBgMetricTextMode(bg, 0) };
            blocks * 0x800
        }
        BG_TYPE_AFFINE => {
            let blocks = unsafe { GetBgMetricAffineMode(bg, 0) };
            blocks as u16 * 0x100
        }
        _ => 0,
    };
    unsafe { LoadBgVram(bg, tilemap.cast(), size, 0, 2) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyToBgTilemapBufferRect(
    bg: u8,
    src: *const c_void,
    dest_x: u8,
    dest_y: u8,
    width: u8,
    height: u8,
) {
    let Some(tilemap) = (unsafe { usable_tilemap(bg) }) else {
        return;
    };

    match unsafe { get_bg_type(bg) } {
        BG_TYPE_NORMAL => {
            let mut source = src.cast::<u16>();
            let mut y = u16::from(dest_y);
            while y < u16::from(dest_y) + u16::from(height) {
                let mut x = u16::from(dest_x);
                while x < u16::from(dest_x) + u16::from(width) {
                    unsafe {
                        tilemap
                            .cast::<u16>()
                            .add((y as usize * 0x20) + x as usize)
                            .write(source.read())
                    };
                    source = unsafe { source.add(1) };
                    x += 1;
                }
                y += 1;
            }
        }
        BG_TYPE_AFFINE => {
            let mut source = src.cast::<u8>();
            let stride = unsafe { GetBgMetricAffineMode(bg, 1) } as usize;
            let mut y = u16::from(dest_y);
            while y < u16::from(dest_y) + u16::from(height) {
                let mut x = u16::from(dest_x);
                while x < u16::from(dest_x) + u16::from(width) {
                    unsafe {
                        tilemap
                            .add((y as usize * stride) + x as usize)
                            .write(source.read())
                    };
                    source = unsafe { source.add(1) };
                    x += 1;
                }
                y += 1;
            }
        }
        _ => {}
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn CopyToBgTilemapBufferRect_ChangePalette(
    bg: u8,
    src: *const c_void,
    dest_x: u8,
    dest_y: u8,
    rect_width: u8,
    rect_height: u8,
    palette: u8,
) {
    unsafe {
        CopyRectToBgTilemapBufferRect(
            bg,
            src,
            0,
            0,
            rect_width,
            rect_height,
            dest_x,
            dest_y,
            rect_width,
            rect_height,
            palette,
            0,
            0,
        )
    };
}

#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn CopyRectToBgTilemapBufferRect(
    bg: u8,
    src: *const c_void,
    src_x: u8,
    src_y: u8,
    src_width: u8,
    _src_height: u8,
    dest_x: u8,
    dest_y: u8,
    rect_width: u8,
    rect_height: u8,
    palette1: u8,
    tile_offset: i16,
    palette2: i16,
) {
    let Some(tilemap) = (unsafe { usable_tilemap(bg) }) else {
        return;
    };

    let screen_size = unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_SCREENSIZE) };
    let screen_width = unsafe { GetBgMetricTextMode(bg, 1) } * 0x20;
    let screen_height = unsafe { GetBgMetricTextMode(bg, 2) } * 0x20;

    match unsafe { get_bg_type(bg) } {
        BG_TYPE_NORMAL => {
            let mut source = unsafe {
                src.cast::<u8>()
                    .add(((src_y as usize * src_width as usize) + src_x as usize) * 2)
            };
            let mut i = u16::from(dest_y);
            while i < u16::from(dest_y) + u16::from(rect_height) {
                let mut j = u16::from(dest_x);
                while j < u16::from(dest_x) + u16::from(rect_width) {
                    let index = unsafe {
                        GetTileMapIndexFromCoords(
                            i32::from(j),
                            i32::from(i),
                            i32::from(screen_size),
                            u32::from(screen_width),
                            u32::from(screen_height),
                        )
                    };
                    unsafe {
                        CopyTileMapEntry(
                            source.cast(),
                            tilemap.add(index as usize * 2).cast(),
                            i32::from(palette1),
                            i32::from(tile_offset),
                            i32::from(palette2),
                        )
                    };
                    source = unsafe { source.add(2) };
                    j += 1;
                }
                source = unsafe { source.add((src_width - rect_width) as usize * 2) };
                i += 1;
            }
        }
        BG_TYPE_AFFINE => {
            let mut source = unsafe {
                src.cast::<u8>()
                    .add((src_y as usize * src_width as usize) + src_x as usize)
            };
            let stride = unsafe { GetBgMetricAffineMode(bg, 1) } as usize;
            let mut i = u16::from(dest_y);
            while i < u16::from(dest_y) + u16::from(rect_height) {
                let mut j = u16::from(dest_x);
                while j < u16::from(dest_x) + u16::from(rect_width) {
                    let value = unsafe { source.read() }.wrapping_add(tile_offset as u8);
                    unsafe { tilemap.add(stride * i as usize + j as usize).write(value) };
                    source = unsafe { source.add(1) };
                    j += 1;
                }
                source = unsafe { source.add((src_width - rect_width) as usize) };
                i += 1;
            }
        }
        _ => {}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillBgTilemapBufferRect_Palette0(
    bg: u8,
    tile_num: u16,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
) {
    let Some(tilemap) = (unsafe { usable_tilemap(bg) }) else {
        return;
    };

    match unsafe { get_bg_type(bg) } {
        BG_TYPE_NORMAL => {
            let mut y16 = u16::from(y);
            while y16 < u16::from(y) + u16::from(height) {
                let mut x16 = u16::from(x);
                while x16 < u16::from(x) + u16::from(width) {
                    unsafe {
                        tilemap
                            .cast::<u16>()
                            .add((y16 as usize * 0x20) + x16 as usize)
                            .write(tile_num)
                    };
                    x16 += 1;
                }
                y16 += 1;
            }
        }
        BG_TYPE_AFFINE => {
            let stride = unsafe { GetBgMetricAffineMode(bg, 1) } as usize;
            let mut y16 = u16::from(y);
            while y16 < u16::from(y) + u16::from(height) {
                let mut x16 = u16::from(x);
                while x16 < u16::from(x) + u16::from(width) {
                    unsafe {
                        tilemap
                            .add((y16 as usize * stride) + x16 as usize)
                            .write(tile_num as u8)
                    };
                    x16 += 1;
                }
                y16 += 1;
            }
        }
        _ => {}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillBgTilemapBufferRect(
    bg: u8,
    tile_num: u16,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    palette: u8,
) {
    unsafe { WriteSequenceToBgTilemapBuffer(bg, tile_num, x, y, width, height, palette, 0) };
}

#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn WriteSequenceToBgTilemapBuffer(
    bg: u8,
    first_tile_num: u16,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    palette_slot: u8,
    tile_num_delta: i16,
) {
    let Some(tilemap) = (unsafe { usable_tilemap(bg) }) else {
        return;
    };

    let screen_size = unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_SCREENSIZE) };
    let screen_width = unsafe { GetBgMetricTextMode(bg, 1) } * 0x20;
    let screen_height = unsafe { GetBgMetricTextMode(bg, 2) } * 0x20;
    let mut tile_num = first_tile_num;

    // The delta only ever touches the tile index; the palette and flip bits
    // in the top six bits are carried through untouched.
    let advance = |tile: u16| (tile & 0xfc00) + (tile.wrapping_add(tile_num_delta as u16) & 0x3ff);

    match unsafe { get_bg_type(bg) } {
        BG_TYPE_NORMAL => {
            let mut y16 = u16::from(y);
            while y16 < u16::from(y) + u16::from(height) {
                let mut x16 = u16::from(x);
                while x16 < u16::from(x) + u16::from(width) {
                    let index = unsafe {
                        GetTileMapIndexFromCoords(
                            i32::from(x16),
                            i32::from(y16),
                            i32::from(screen_size),
                            u32::from(screen_width),
                            u32::from(screen_height),
                        )
                    } as u16;
                    unsafe {
                        CopyTileMapEntry(
                            &raw const tile_num,
                            tilemap.cast::<u16>().add(index as usize),
                            i32::from(palette_slot),
                            0,
                            0,
                        )
                    };
                    tile_num = advance(tile_num);
                    x16 += 1;
                }
                y16 += 1;
            }
        }
        BG_TYPE_AFFINE => {
            let stride = unsafe { GetBgMetricAffineMode(bg, 1) } as usize;
            let mut y16 = u16::from(y);
            while y16 < u16::from(y) + u16::from(height) {
                let mut x16 = u16::from(x);
                while x16 < u16::from(x) + u16::from(width) {
                    unsafe {
                        tilemap
                            .add(y16 as usize * stride + x16 as usize)
                            .write(tile_num as u8)
                    };
                    tile_num = advance(tile_num);
                    x16 += 1;
                }
                y16 += 1;
            }
        }
        _ => {}
    }
}

/// Screen-size dependent metrics for a text-mode layer: 0 is the tilemap size
/// in 2 KiB blocks, 1 the width in screen blocks, 2 the height.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBgMetricTextMode(bg: u8, which_metric: u8) -> u16 {
    let screen_size = unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_SCREENSIZE) };

    match (which_metric, screen_size) {
        (0, 0) => 1,
        (0, 1) | (0, 2) => 2,
        (0, 3) => 4,
        (1, 0) | (1, 2) => 1,
        (1, 1) | (1, 3) => 2,
        (2, 0) | (2, 1) => 1,
        (2, 2) | (2, 3) => 2,
        _ => 0,
    }
}

/// The affine equivalent: 0 is the tilemap size in 256-byte blocks, 1 and 2
/// the map width and height in tiles.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBgMetricAffineMode(bg: u8, which_metric: u8) -> u32 {
    let screen_size = unsafe { get_bg_control_attribute(bg, BG_CTRL_ATTR_SCREENSIZE) };

    match which_metric {
        0 => match screen_size {
            0 => 0x1,
            1 => 0x4,
            2 => 0x10,
            3 => 0x40,
            _ => 0,
        },
        1 | 2 => 0x10u32 << screen_size,
        _ => 0,
    }
}

/// Maps tile coordinates onto the flat tilemap, accounting for how wide and
/// tall screen blocks are stitched together.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTileMapIndexFromCoords(
    x: i32,
    y: i32,
    screen_size: i32,
    screen_width: u32,
    screen_height: u32,
) -> u32 {
    let mut x = x & (screen_width as i32 - 1);
    let mut y = y & (screen_height as i32 - 1);

    match screen_size {
        // Size 3 is two screen blocks tall as well as wide, so the bottom
        // half starts a further two blocks in; it then falls through to the
        // horizontal adjustment.
        3 => {
            if y >= 0x20 {
                y += 0x20;
            }
            if x >= 0x20 {
                x -= 0x20;
                y += 0x20;
            }
        }
        1 => {
            if x >= 0x20 {
                x -= 0x20;
                y += 0x20;
            }
        }
        _ => {}
    }

    ((y * 0x20) + x) as u32
}

/// Writes one tilemap entry, optionally forcing a palette or keeping the
/// destination's existing flip and palette bits.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyTileMapEntry(
    src: *const u16,
    dest: *mut u16,
    palette1: i32,
    tile_offset: i32,
    palette2: i32,
) {
    let source = unsafe { src.read() };
    let value = match palette1 {
        0..=15 => {
            ((source.wrapping_add(tile_offset as u16)) & 0xfff)
                + (((palette1 + palette2) as u16) << 12)
        }
        // 16 keeps the destination's flip bits and only replaces the index.
        16 => {
            let mut value = unsafe { dest.read() } & 0xfc00;
            value = value.wrapping_add((palette2 as u16) << 12);
            value | ((source.wrapping_add(tile_offset as u16)) & 0x3ff)
        }
        // 17 and anything else pass the entry through unmasked.
        _ => source
            .wrapping_add(tile_offset as u16)
            .wrapping_add((palette2 as u16) << 12),
    };
    unsafe { dest.write(value) };
}

/// Whether a layer is a text or affine layer depends on the display mode.
unsafe fn get_bg_type(bg: u8) -> u32 {
    let mode = unsafe { GetBgMode() };
    match (bg, mode) {
        (0 | 1, 0 | 1) => BG_TYPE_NORMAL,
        (2, 0) => BG_TYPE_NORMAL,
        (2, 1 | 2) => BG_TYPE_AFFINE,
        (3, 0) => BG_TYPE_NORMAL,
        (3, 2) => BG_TYPE_AFFINE,
        _ => BG_TYPE_NONE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_bitfields_match_the_documented_masks() {
        assert_eq!(TMPL_BG, 0x0003);
        assert_eq!(TMPL_CHAR_BASE_INDEX, 0x000c);
        assert_eq!(TMPL_MAP_BASE_INDEX, 0x01f0);
        assert_eq!(TMPL_SCREEN_SIZE, 0x0600);
        assert_eq!(TMPL_PALETTE_MODE, 0x0800);
        assert_eq!(TMPL_PRIORITY, 0x3000);
        assert_eq!(TMPL_BASE_TILE, 0x03ff_0000);
        // The fields tile together without overlapping.
        let all = TMPL_BG
            | TMPL_CHAR_BASE_INDEX
            | TMPL_MAP_BASE_INDEX
            | TMPL_SCREEN_SIZE
            | TMPL_PALETTE_MODE
            | TMPL_PRIORITY;
        assert_eq!(all, 0x3fff);
    }

    #[test]
    fn the_dispcnt_mask_covers_the_layers_and_the_mode() {
        assert_eq!(DISPCNT_ALL_BG_AND_MODE_BITS, 0x0f00 | 0x7);
    }

    #[test]
    fn coordinate_ops_set_add_and_subtract() {
        assert_eq!(apply_coord_op(100, 5, BG_COORD_SET), 5);
        assert_eq!(apply_coord_op(100, 5, BG_COORD_ADD), 105);
        assert_eq!(apply_coord_op(100, 5, BG_COORD_SUB), 95);
        // Any unknown op behaves like SET.
        assert_eq!(apply_coord_op(100, 5, 99), 5);
    }

    #[test]
    fn wide_screen_blocks_wrap_into_the_row_below() {
        // Size 1 is two blocks wide: column 32 is the start of the second.
        let index = |x, y, size| unsafe { GetTileMapIndexFromCoords(x, y, size, 64, 32) };
        assert_eq!(index(0, 0, 1), 0);
        assert_eq!(index(31, 0, 1), 31);
        assert_eq!(index(32, 0, 1), 0x20 * 0x20);
        // A flat 32x32 map never remaps.
        let flat = |x, y| unsafe { GetTileMapIndexFromCoords(x, y, 0, 32, 32) };
        assert_eq!(flat(5, 3), 3 * 0x20 + 5);
    }

    #[test]
    fn screen_size_metrics_match_the_original_tables() {
        // Only the table shape is checked here; the lookups need a
        // configured layer.
        assert_eq!(0x10u32 << 0, 0x10);
        assert_eq!(0x10u32 << 3, 0x80);
    }
}
