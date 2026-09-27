use core::ptr::{addr_of, addr_of_mut};

const REG_IME: *mut u16 = 0x0400_0208 as *mut u16;
const MAIN_IN_BATTLE_BYTE_OFFSET: usize = 0x439;

type MainCallback = unsafe extern "C" fn();

#[repr(C)]
struct SaveBlock2OptionsView {
    prefix: [u8; 0x14],
    options: u16,
}

unsafe extern "C" {
    static mut gMain: u8;
    static mut gSaveBlock2Ptr: *mut SaveBlock2OptionsView;
    static mut gSaveFileStatus: u8;
    static mut gHeap: u8;

    fn RegisterRamReset(reset_flags: u32);
    fn ClearGpuRegBits(register_offset: u8, mask: u16);
    fn GetSaveBlocksPointersBaseOffset() -> u16;
    fn SetSaveBlocksPointers(offset: u16);
    fn ResetMenuAndMonGlobals();
    fn Save_ResetSaveCounters();
    fn LoadGameSave(save_type: u8) -> u8;
    fn Sav2_ClearSetDefault();
    fn SetPokemonCryStereo(value: u32);
    fn InitHeap(heap_start: *mut u8, heap_size: u32);
    fn SetMainCallback2(callback: MainCallback);
    fn CB2_ContinueSavedGame();
}

const fn options_sound(options: u16) -> u32 {
    ((options >> 8) & 1) as u32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReloadSave() {
    let interrupt_master_enable = unsafe { REG_IME.read_volatile() };
    unsafe { REG_IME.write_volatile(0) };
    unsafe { RegisterRamReset(1) };
    unsafe { ClearGpuRegBits(0, 0x80) };
    unsafe { REG_IME.write_volatile(interrupt_master_enable) };

    let main_flags = unsafe { (&raw mut gMain).add(MAIN_IN_BATTLE_BYTE_OFFSET) };
    unsafe { main_flags.write(main_flags.read() & !0x02) };

    let offset = unsafe { GetSaveBlocksPointersBaseOffset() };
    unsafe { SetSaveBlocksPointers(offset) };
    unsafe { ResetMenuAndMonGlobals() };
    unsafe { Save_ResetSaveCounters() };
    let _ = unsafe { LoadGameSave(0) };
    let status = unsafe { addr_of!(gSaveFileStatus).read() };
    if status == 0 || status == 2 {
        unsafe { Sav2_ClearSetDefault() };
    }

    let save = unsafe { gSaveBlock2Ptr };
    let options = unsafe { addr_of!((*save).options).read() };
    unsafe { SetPokemonCryStereo(options_sound(options)) };
    unsafe { InitHeap(addr_of_mut!(gHeap), 0x1c000) };
    unsafe { SetMainCallback2(CB2_ContinueSavedGame) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sound_option_matches_the_c_bitfield() {
        assert_eq!(core::mem::offset_of!(SaveBlock2OptionsView, options), 0x14);
        assert_eq!(options_sound(0), 0);
        assert_eq!(options_sound(1 << 8), 1);
        assert_eq!(options_sound(u16::MAX), 1);
    }
}
