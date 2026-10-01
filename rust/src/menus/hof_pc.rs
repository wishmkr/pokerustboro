//! The Hall of Fame on the player's PC (was src/hof_pc.c): leaving the
//! field for the Hall of Fame viewer, and coming back to the PC menu.

use crate::overworld::gFieldCallback;
use crate::palette::{BeginNormalPaletteFade, gPaletteFade};
use crate::task::{create_task, destroy_task};

const PALETTES_ALL: u32 = u32::MAX;

/// `CB2_DoHallOfFamePC` with this module's view of its types.
#[inline]
unsafe fn CB2_DoHallOfFamePC() {
    unsafe {
        crate::hall_of_fame::CB2_DoHallOfFamePC();
    }
}
/// `CB2_ReturnToField` with this module's view of its types.
#[inline]
unsafe fn CB2_ReturnToField() {
    unsafe {
        crate::overworld::CB2_ReturnToField();
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: unsafe fn()) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `LockPlayerFieldControls` with this module's view of its types.
#[inline]
unsafe fn LockPlayerFieldControls() {
    unsafe {
        crate::script::LockPlayerFieldControls();
    }
}
/// `Overworld_PlaySpecialMapMusic` with this module's view of its types.
#[inline]
unsafe fn Overworld_PlaySpecialMapMusic() {
    unsafe {
        crate::overworld::Overworld_PlaySpecialMapMusic();
    }
}
/// `ScriptMenu_CreatePCMultichoice` with this module's view of its types.
#[inline]
unsafe fn ScriptMenu_CreatePCMultichoice() {
    unsafe {
        crate::script_menu::ScriptMenu_CreatePCMultichoice();
    }
}
/// `ScriptMenu_DisplayPCStartupPrompt` with this module's view of its types.
#[inline]
unsafe fn ScriptMenu_DisplayPCStartupPrompt() {
    unsafe {
        crate::script_menu::ScriptMenu_DisplayPCStartupPrompt();
    }
}

fn palette_fade_active() -> bool {
    // SAFETY: a plain read of the fade state.
    unsafe { (*(&raw const gPaletteFade)).active() != 0 }
}

fn task_wait_for_palette_fade(task_id: u8) {
    if !palette_fade_active() {
        destroy_task(task_id);
    }
}

/// Back on the field: show the PC menu again and fade in.
fn reshow_pc_menu_after_hall_of_fame_pc() {
    // SAFETY: field-callback context, as in C.
    unsafe {
        LockPlayerFieldControls();
        Overworld_PlaySpecialMapMusic();
        ScriptMenu_CreatePCMultichoice();
        ScriptMenu_DisplayPCStartupPrompt();
        BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    }
    create_task(task_wait_for_palette_fade, 10);
}

pub fn access_hall_of_fame_pc() {
    // SAFETY: called from the PC menu script, as in C.
    unsafe {
        SetMainCallback2(CB2_DoHallOfFamePC);
        LockPlayerFieldControls();
    }
}

pub fn return_from_hall_of_fame_pc() {
    // SAFETY: as above; the field reads gFieldCallback once it's back.
    unsafe {
        SetMainCallback2(CB2_ReturnToField);
        (&raw mut gFieldCallback).write(Some(reshow_pc_menu_after_hall_of_fame_pc));
    }
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn AccessHallOfFamePC() {
    access_hall_of_fame_pc();
}

#[unsafe(no_mangle)]
pub fn ReturnFromHallOfFamePC() {
    return_from_hall_of_fame_pc();
}
