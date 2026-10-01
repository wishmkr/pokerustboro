//! Poison damage while walking (was src/field_poison.c).
//!
//! Every few steps, each poisoned Pokémon in the party loses 1 HP. One that
//! reaches 0 faints (and is cured); if the whole party faints, the player
//! whites out.

use crate::box_mon::POKEMON_NAME_LENGTH;
use crate::event_data::var_set;
use crate::party::{PARTY_SIZE, player_party};
use crate::task::{create_task, destroy_task, task_data};
use crate::types::Pokemon;

const SPECIES_EGG: u16 = 412;
const AILMENT_PSN: u8 = 1;
const FRIENDSHIP_EVENT_FAINT_FIELD_PSN: u8 = 7;
const VAR_RESULT: u16 = 0x800d;

/// What a step of poison did ([`do_poison_field_effect`]).
const FLDPSN_NONE: i32 = 0;
const FLDPSN_PSN: i32 = 1;
const FLDPSN_FNT: i32 = 2;

/// `VAR_RESULT` after [`TryFieldPoisonWhiteOut`].
const FLDPSN_NO_WHITEOUT: u16 = 0;
const FLDPSN_WHITEOUT: u16 = 1;
const FLDPSN_FRONTIER_WHITEOUT: u16 = 2;

/// `AdjustFriendship` with this module's view of its types.
#[inline]
unsafe fn AdjustFriendship(a0: *mut Pokemon, a1: u8) {
    unsafe {
        crate::pokemon::AdjustFriendship(a0 as _, a1);
    }
}
/// `GetAilmentFromStatus` with this module's view of its types.
#[inline]
unsafe fn GetAilmentFromStatus(a0: u32) -> u8 {
    unsafe { crate::party_menu::GetAilmentFromStatus(a0) }
}
/// `ShowFieldMessage` with this module's view of its types.
#[inline]
unsafe fn ShowFieldMessage(a0: *const u8) -> u8 {
    unsafe { crate::field_message_box::ShowFieldMessage(a0 as _) }
}
/// `IsFieldMessageBoxHidden` with this module's view of its types.
#[inline]
unsafe fn IsFieldMessageBoxHidden() -> u8 {
    unsafe { crate::field_message_box::IsFieldMessageBoxHidden() }
}
/// `CurrentBattlePyramidLocation` with this module's view of its types.
#[inline]
unsafe fn CurrentBattlePyramidLocation() -> u8 {
    unsafe { crate::battle_pyramid::CurrentBattlePyramidLocation() }
}
/// `InBattlePike` with this module's view of its types.
#[inline]
unsafe fn InBattlePike() -> u8 {
    unsafe { crate::battle_pike::InBattlePike() }
}
/// `InTrainerHillChallenge` with this module's view of its types.
#[inline]
unsafe fn InTrainerHillChallenge() -> u8 {
    unsafe { crate::trainer_hill::InTrainerHillChallenge() }
}
/// `ScriptContext_Enable` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_Enable() {
    unsafe {
        crate::script::ScriptContext_Enable();
    }
}
/// `ScriptContext_Stop` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_Stop() {
    unsafe {
        crate::script::ScriptContext_Stop();
    }
}
/// `StringGet_Nickname` with this module's view of its types.
#[inline]
unsafe fn StringGet_Nickname(a0: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringGet_Nickname(a0 as _) as *mut u8 }
}
/// `FldEffPoison_Start` with this module's view of its types.
#[inline]
unsafe fn FldEffPoison_Start() {
    unsafe {
        crate::fldeff_misc::FldEffPoison_Start();
    }
}

/// Party slot `i`.
///
/// # Safety
/// The party must not be in use elsewhere while the result lives.
unsafe fn party_mon<'a>(i: usize) -> &'a mut Pokemon {
    unsafe { &mut player_party()[i] }
}

/// A Pokémon that can faint: one that isn't an egg (or an empty slot).
fn is_valid_species(mon: &mut Pokemon) -> bool {
    let species = mon.r#box.species_or_egg();
    species != 0 && species != SPECIES_EGG
}

fn is_poisoned(mon: &Pokemon) -> bool {
    // SAFETY: a plain C function.
    unsafe { GetAilmentFromStatus(mon.status) == AILMENT_PSN }
}

fn all_mons_fainted() -> bool {
    // SAFETY: the party isn't in use elsewhere during the poison task.
    (0..PARTY_SIZE).all(|i| {
        let mon = unsafe { party_mon(i) };
        !(is_valid_species(mon) && mon.hp != 0)
    })
}

fn mon_fainted_from_poison(i: usize) -> bool {
    // SAFETY: as in all_mons_fainted.
    let mon = unsafe { party_mon(i) };
    is_valid_species(mon) && mon.hp == 0 && is_poisoned(mon)
}

/// The Pokémon in slot `i` fainted: cure it, make it less friendly, and put
/// its name in `gStringVar1` for the message.
fn faint_from_field_poison(i: usize) {
    // SAFETY: as in all_mons_fainted; AdjustFriendship takes a party mon.
    let mon = unsafe { party_mon(i) };
    unsafe { AdjustFriendship(mon, FRIENDSHIP_EVENT_FAINT_FIELD_PSN) };
    mon.status = 0;
    let mut name = [0u8; POKEMON_NAME_LENGTH + 1];
    mon.r#box.nickname(&mut name);
    // SAFETY: gStringVar1 is a 256-byte string buffer, used only here now.
    unsafe {
        let buffer = &mut (*(&raw mut crate::string_util::gStringVar1)).0.0;
        for (dst, src) in buffer.iter_mut().zip(name) {
            *dst = src;
        }
        StringGet_Nickname(buffer.as_mut_ptr());
    }
}

/// The task behind [`TryFieldPoisonWhiteOut`]: shows "{mon} fainted" for
/// each Pokémon that fainted from poison, then sets `VAR_RESULT` to whether
/// the player whites out, and resumes the script.
///
/// Its data: 0, the state (0: checking the party, 1: waiting for the
/// message, 2: done); 1, the party slot being checked.
fn Task_TryFieldPoisonWhiteOut(task_id: u8) {
    // SAFETY: copied out; the borrow ends before any other code runs.
    let (mut state, mut slot) = {
        let data = unsafe { task_data(task_id) };
        (data[0], data[1])
    };
    match state {
        0 => {
            while (slot as usize) < PARTY_SIZE {
                if mon_fainted_from_poison(slot as usize) {
                    faint_from_field_poison(slot as usize);
                    // SAFETY: a plain C function showing static text.
                    unsafe {
                        ShowFieldMessage(&raw const (*(&raw const crate::data::strings::gText_PkmnFainted_FldPsn).cast::<u8>()))
                    };
                    state += 1;
                    break;
                }
                slot += 1;
            }
            if slot as usize >= PARTY_SIZE {
                state = 2;
            }
        }
        1 => {
            // SAFETY: plain C function.
            if unsafe { IsFieldMessageBoxHidden() } != 0 {
                state -= 1;
            }
        }
        2 => {
            let result = if !all_mons_fainted() {
                FLDPSN_NO_WHITEOUT
            } else if unsafe {
                CurrentBattlePyramidLocation() != 0
                    || InBattlePike() != 0
                    || InTrainerHillChallenge() != 0
            } {
                // the battle facilities have their own white out
                FLDPSN_FRONTIER_WHITEOUT
            } else {
                FLDPSN_WHITEOUT
            };
            var_set(VAR_RESULT, result);
            // SAFETY: plain C function.
            unsafe { ScriptContext_Enable() };
            destroy_task(task_id);
            return;
        }
        _ => {}
    }
    // SAFETY: as above.
    let data = unsafe { task_data(task_id) };
    data[0] = state;
    data[1] = slot;
}

/// One step's poison: 1 HP off each poisoned Pokémon, with the screen flash.
pub fn do_poison_field_effect() -> i32 {
    let (mut poisoned, mut fainted) = (0, 0);
    for i in 0..PARTY_SIZE {
        // SAFETY: the party isn't in use elsewhere during a step.
        let mon = unsafe { party_mon(i) };
        if mon.r#box.hasSpecies() != 0 && is_poisoned(mon) {
            if mon.hp <= 1 {
                fainted += 1;
            }
            mon.hp = mon.hp.saturating_sub(1);
            poisoned += 1;
        }
    }
    if fainted != 0 || poisoned != 0 {
        // SAFETY: plain C function.
        unsafe { FldEffPoison_Start() };
    }
    if fainted != 0 {
        FLDPSN_FNT
    } else if poisoned != 0 {
        FLDPSN_PSN
    } else {
        FLDPSN_NONE
    }
}

// ------------------------------------------------------------------ C names

/// Script special: runs [`Task_TryFieldPoisonWhiteOut`], pausing the script.
#[unsafe(no_mangle)]
pub fn TryFieldPoisonWhiteOut() {
    create_task(Task_TryFieldPoisonWhiteOut, 80);
    // SAFETY: plain C function.
    unsafe { ScriptContext_Stop() };
}

#[unsafe(no_mangle)]
pub fn DoPoisonFieldEffect() -> i32 {
    do_poison_field_effect()
}
