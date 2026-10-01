//! Reloading the save after a link battle or a record mix (was
//! src/reload_save.c): the RAM is wiped and the game continues from the save.

use crate::agb_main::main;
use crate::consts::*;
use crate::malloc::{InitHeap, gHeap};
use crate::save::gSaveFileStatus;
use crate::save_blocks::save_block2;

const REG_IME: *mut u16 = 0x0400_0208 as *mut u16;
const RESET_EWRAM: u32 = 1 << 0;
const DISPCNT_FORCED_BLANK: u16 = 1 << 7;
const SAVE_NORMAL: u8 = 0;

/// `RegisterRamReset` with this module's view of its types.
#[inline]
unsafe fn RegisterRamReset(a0: u32) {
    unsafe {
        crate::syscall::RegisterRamReset(a0);
    }
}
/// `ClearGpuRegBits` with this module's view of its types.
#[inline]
unsafe fn ClearGpuRegBits(a0: u8, a1: u16) {
    unsafe {
        crate::gpu_regs::ClearGpuRegBits(a0, a1);
    }
}
/// `GetSaveBlocksPointersBaseOffset` with this module's view of its types.
#[inline]
unsafe fn GetSaveBlocksPointersBaseOffset() -> u16 {
    unsafe { crate::save::GetSaveBlocksPointersBaseOffset() }
}
/// `SetSaveBlocksPointers` with this module's view of its types.
#[inline]
unsafe fn SetSaveBlocksPointers(a0: u16) {
    unsafe {
        crate::load_save::SetSaveBlocksPointers(a0);
    }
}
/// `ResetMenuAndMonGlobals` with this module's view of its types.
#[inline]
unsafe fn ResetMenuAndMonGlobals() {
    unsafe {
        crate::new_game::ResetMenuAndMonGlobals();
    }
}
/// `Save_ResetSaveCounters` with this module's view of its types.
#[inline]
unsafe fn Save_ResetSaveCounters() {
    {
        crate::save::Save_ResetSaveCounters();
    }
}
/// `LoadGameSave` with this module's view of its types.
#[inline]
unsafe fn LoadGameSave(a0: u8) -> u8 {
    unsafe { crate::save::LoadGameSave(a0) }
}
/// `Sav2_ClearSetDefault` with this module's view of its types.
#[inline]
unsafe fn Sav2_ClearSetDefault() {
    unsafe {
        crate::new_game::Sav2_ClearSetDefault();
    }
}
/// `SetPokemonCryStereo` with this module's view of its types.
#[inline]
unsafe fn SetPokemonCryStereo(a0: u32) {
    unsafe {
        crate::m4a::SetPokemonCryStereo(a0);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: unsafe fn()) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `CB2_ContinueSavedGame` with this module's view of its types.
#[inline]
unsafe fn CB2_ContinueSavedGame() {
    unsafe {
        crate::overworld::CB2_ContinueSavedGame();
    }
}

pub fn reload_save() {
    // SAFETY: this runs from a callback with nothing borrowed; each step is
    // C's own, in C's order.
    unsafe {
        // wipe EWRAM with interrupts off
        let ime = REG_IME.read_volatile();
        REG_IME.write_volatile(0);
        RegisterRamReset(RESET_EWRAM);
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_FORCED_BLANK);
        REG_IME.write_volatile(ime);

        main().set_inBattle(0);
        SetSaveBlocksPointers(GetSaveBlocksPointersBaseOffset());
        ResetMenuAndMonGlobals();
        Save_ResetSaveCounters();
        LoadGameSave(SAVE_NORMAL);
        let status = *(&raw const gSaveFileStatus);
        if status == u16::from(SAVE_STATUS_EMPTY) || status == SAVE_STATUS_CORRUPT {
            Sav2_ClearSetDefault();
        }
        SetPokemonCryStereo(save_block2().optionsSound().into());
        InitHeap((&raw mut gHeap).cast::<u8>(), HEAP_SIZE);
        SetMainCallback2(CB2_ContinueSavedGame);
    }
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn ReloadSave() {
    reload_save();
}
