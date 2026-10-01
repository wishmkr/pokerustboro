//! Party helpers reached from event scripts: healing, gifting Pokemon and
//! eggs, and picking a subset of the party for a facility battle.

use crate::ffi::{
    CpuSet, GetMonData2, MON_DATA_HELD_ITEM, MON_DATA_HP, MON_DATA_MAX_HP, PARTY_SIZE,
    POKEMON_SIZE, SetMonData, VarSet, gSpecialVar_0x8004, gSpecialVar_Result, gStringVar1,
    party_mon,
};
use crate::pokemon::gPlayerPartyCount;
use core::ffi::c_int;

const MON_DATA_MOVE1: c_int = 13;
const MON_DATA_PP1: c_int = 17;
const MON_DATA_PP_BONUSES: c_int = 21;
const MON_DATA_IS_EGG: c_int = 45;
const MON_DATA_STATUS: c_int = 55;
const MON_DATA_SPECIES_OR_EGG: c_int = 65;

const MAX_MON_MOVES: usize = 4;
const MAX_FRONTIER_PARTY_SIZE: usize = 4;

const SPECIES_NONE: u16 = 0;
const SPECIES_EGG: u16 = 412;
const ITEM_ENIGMA_BERRY: u16 = 175;
const USE_RANDOM_IVS: u8 = 32;
const OT_ID_PLAYER_ID: u8 = 0;

const MON_GIVEN_TO_PARTY: u8 = 0;
const MON_GIVEN_TO_PC: u8 = 1;
const FLAG_SET_SEEN: u8 = 2;
const FLAG_SET_CAUGHT: u8 = 3;

const PLAYER_HAS_TWO_USABLE_MONS: u16 = 0;
const PLAYER_HAS_ONE_MON: u16 = 1;
const PLAYER_HAS_ONE_USABLE_MON: u16 = 2;

const VAR_FRONTIER_FACILITY: u16 = 0x40cf;
const FACILITY_MULTI_OR_EREADER: u16 = 9;

/// `offsetof(struct Main, savedCallback)`
const MAIN_SAVED_CALLBACK: usize = 8;

const CPU_SET_SRC_FIXED: u32 = 0x0100_0000;
const CPU_SET_32BIT: u32 = 0x0400_0000;

type MainCallback = unsafe fn();

/// `CalculatePPWithBonus` with this module's view of its types.
#[inline]
unsafe fn CalculatePPWithBonus(a0: u16, a1: u8, a2: u8) -> u8 {
    unsafe { crate::pokemon::CalculatePPWithBonus(a0, a1, a2) }
}
/// `CreateMon` with this module's view of its types.
#[inline]
unsafe fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32) {
    unsafe {
        crate::pokemon::CreateMon(a0 as _, a1, a2, a3, a4, a5, a6, a7);
    }
}
/// `CreateEgg` with this module's view of its types.
#[inline]
unsafe fn CreateEgg(a0: *mut u8, a1: u16, a2: u8) {
    unsafe {
        crate::daycare::CreateEgg(a0 as _, a1, a2);
    }
}
/// `GiveMonToPlayer` with this module's view of its types.
#[inline]
unsafe fn GiveMonToPlayer(a0: *mut u8) -> u8 {
    unsafe { crate::pokemon::GiveMonToPlayer(a0 as _) }
}
/// `GetSetPokedexFlag` with this module's view of its types.
#[inline]
unsafe fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8 {
    unsafe { crate::pokedex::GetSetPokedexFlag(a0, a1) }
}
/// `SpeciesToNationalPokedexNum` with this module's view of its types.
#[inline]
unsafe fn SpeciesToNationalPokedexNum(a0: u16) -> u16 {
    unsafe { crate::pokemon::SpeciesToNationalPokedexNum(a0) }
}
/// `GetMonsStateToDoubles` with this module's view of its types.
#[inline]
unsafe fn GetMonsStateToDoubles() -> u8 {
    unsafe { crate::pokemon::GetMonsStateToDoubles() }
}
/// `GetBerryNameByBerryType` with this module's view of its types.
#[inline]
unsafe fn GetBerryNameByBerryType(a0: u8, a1: *mut u8) {
    unsafe {
        crate::berry::GetBerryNameByBerryType(a0, a1 as _);
    }
}
/// `ItemIdToBerryType` with this module's view of its types.
#[inline]
unsafe fn ItemIdToBerryType(a0: u16) -> u8 {
    unsafe { crate::berry::ItemIdToBerryType(a0) }
}
/// `ZeroEnemyPartyMons` with this module's view of its types.
#[inline]
unsafe fn ZeroEnemyPartyMons() {
    unsafe {
        crate::pokemon::ZeroEnemyPartyMons();
    }
}
/// `SetMonMoveSlot` with this module's view of its types.
#[inline]
unsafe fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8) {
    unsafe {
        crate::pokemon::SetMonMoveSlot(a0 as _, a1, a2);
    }
}
/// `InitChooseHalfPartyForBattle` with this module's view of its types.
#[inline]
unsafe fn InitChooseHalfPartyForBattle(a0: u8) {
    unsafe {
        crate::party_menu::InitChooseHalfPartyForBattle(a0);
    }
}
/// `CalculatePlayerPartyCount` with this module's view of its types.
#[inline]
unsafe fn CalculatePlayerPartyCount() -> u8 {
    unsafe { crate::pokemon::CalculatePlayerPartyCount() }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: MainCallback) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `CB2_ReturnToFieldContinueScriptPlayMapMusic` with this module's view of its types.
#[inline]
unsafe fn CB2_ReturnToFieldContinueScriptPlayMapMusic() {
    unsafe {
        crate::overworld::CB2_ReturnToFieldContinueScriptPlayMapMusic();
    }
}

/// `CpuFill32(0, dest, size)`
#[inline]
unsafe fn cpu_fill32_zero(dest: *mut u8, size: usize) {
    let zero = 0u32;
    unsafe {
        CpuSet(
            (&raw const zero).cast(),
            dest.cast(),
            CPU_SET_32BIT | CPU_SET_SRC_FIXED | (size as u32 / 4 & 0x1f_ffff),
        )
    };
}

#[inline]
unsafe fn set_saved_callback(callback: MainCallback) {
    unsafe {
        (&raw mut (*(&raw const crate::agb_main::gMain).cast::<u8>().cast_mut()))
            .add(MAIN_SAVED_CALLBACK)
            .cast::<MainCallback>()
            .write(callback)
    };
}

#[unsafe(no_mangle)]
pub unsafe fn HealPlayerParty() {
    let count = unsafe { (&raw const gPlayerPartyCount).read_volatile() } as usize;

    for i in 0..count {
        let mon = unsafe { party_mon(i) };

        // Full HP.
        let max_hp = unsafe { GetMonData2(mon, MON_DATA_MAX_HP) } as u16;
        let mut arg = [max_hp as u8, (max_hp >> 8) as u8, 0, 0];
        unsafe { SetMonData(mon, MON_DATA_HP, arg.as_ptr().cast()) };

        // Full PP on every move, honouring the PP Up bonuses.
        let pp_bonuses = unsafe { GetMonData2(mon, MON_DATA_PP_BONUSES) } as u8;
        for j in 0..MAX_MON_MOVES {
            let move_id = unsafe { GetMonData2(mon, MON_DATA_MOVE1 + j as c_int) } as u16;
            arg[0] = unsafe { CalculatePPWithBonus(move_id, pp_bonuses, j as u8) };
            unsafe { SetMonData(mon, MON_DATA_PP1 + j as c_int, arg.as_ptr().cast()) };
        }

        // Status is a u32, so all four bytes are cleared rather than one.
        arg = [0; 4];
        unsafe { SetMonData(mon, MON_DATA_STATUS, arg.as_ptr().cast()) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptGiveMon(
    species: u16,
    level: u8,
    item: u16,
    _unused1: u32,
    _unused2: u32,
    _unused3: u8,
) -> u8 {
    let mut mon = [0u8; POKEMON_SIZE];
    let mon = mon.as_mut_ptr();

    unsafe {
        CreateMon(
            mon,
            species,
            level,
            USE_RANDOM_IVS,
            0,
            0,
            OT_ID_PLAYER_ID,
            0,
        )
    };
    let held_item = [item as u8, (item >> 8) as u8];
    unsafe { SetMonData(mon, MON_DATA_HELD_ITEM, held_item.as_ptr().cast()) };

    let sent_to_pc = unsafe { GiveMonToPlayer(mon) };
    let national_num = unsafe { SpeciesToNationalPokedexNum(species) };

    // MON_CANT_GIVE must not set the Pokedex flags.
    if sent_to_pc == MON_GIVEN_TO_PARTY || sent_to_pc == MON_GIVEN_TO_PC {
        unsafe { GetSetPokedexFlag(national_num, FLAG_SET_SEEN) };
        unsafe { GetSetPokedexFlag(national_num, FLAG_SET_CAUGHT) };
    }

    sent_to_pc
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptGiveEgg(species: u16) -> u8 {
    let mut mon = [0u8; POKEMON_SIZE];
    let mon = mon.as_mut_ptr();

    unsafe { CreateEgg(mon, species, 1) };
    let is_egg = 1u8;
    unsafe { SetMonData(mon, MON_DATA_IS_EGG, (&raw const is_egg).cast()) };

    unsafe { GiveMonToPlayer(mon) }
}

#[unsafe(no_mangle)]
pub unsafe fn HasEnoughMonsForDoubleBattle() {
    let state = u16::from(unsafe { GetMonsStateToDoubles() });
    // Anything outside the three known states leaves the var untouched.
    if state == PLAYER_HAS_TWO_USABLE_MONS
        || state == PLAYER_HAS_ONE_MON
        || state == PLAYER_HAS_ONE_USABLE_MON
    {
        unsafe { (&raw mut gSpecialVar_Result).write_volatile(state) };
    }
}

unsafe fn party_has_held_item(item: u16) -> bool {
    for i in 0..PARTY_SIZE {
        let mon = unsafe { party_mon(i) };
        let species = unsafe { GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) } as u16;
        if species != SPECIES_NONE
            && species != SPECIES_EGG
            && unsafe { GetMonData2(mon, MON_DATA_HELD_ITEM) } as u16 == item
        {
            return true;
        }
    }
    false
}

#[unsafe(no_mangle)]
pub unsafe fn DoesPartyHaveEnigmaBerry() -> u8 {
    let has_item = unsafe { party_has_held_item(ITEM_ENIGMA_BERRY) };
    if has_item {
        unsafe {
            GetBerryNameByBerryType(
                ItemIdToBerryType(ITEM_ENIGMA_BERRY),
                (&raw mut gStringVar1).cast::<u8>(),
            )
        };
    }
    u8::from(has_item)
}

#[unsafe(no_mangle)]
pub unsafe fn CreateScriptedWildMon(species: u16, level: u8, item: u16) {
    let mon = (&raw mut (*(&raw const crate::pokemon::gEnemyParty)
        .cast::<u8>()
        .cast_mut()))
        .cast::<u8>();
    unsafe { ZeroEnemyPartyMons() };
    unsafe {
        CreateMon(
            mon,
            species,
            level,
            USE_RANDOM_IVS,
            0,
            0,
            OT_ID_PLAYER_ID,
            0,
        )
    };

    if item != 0 {
        let held_item = [item as u8, (item >> 8) as u8];
        unsafe { SetMonData(mon, MON_DATA_HELD_ITEM, held_item.as_ptr().cast()) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptSetMonMoveSlot(mon_index: u8, move_id: u16, slot: u8) {
    // The bound is `>` rather than `>=`, so index PARTY_SIZE reads one mon
    // past the party. No script passes that value, and the check is left as
    // the original wrote it.
    let mon_index = if mon_index as usize > PARTY_SIZE {
        unsafe { (&raw const gPlayerPartyCount).read_volatile() }.wrapping_sub(1)
    } else {
        mon_index
    };

    unsafe { SetMonMoveSlot(party_mon(mon_index as usize), move_id, slot) };
}

/// Back in the event script, `gSpecialVar_Result` is TRUE when the player
/// actually picked a party.
#[unsafe(no_mangle)]
pub unsafe fn ChooseHalfPartyForBattle() {
    unsafe { set_saved_callback(cb2_return_from_choose_half_party) };
    unsafe { VarSet(VAR_FRONTIER_FACILITY, FACILITY_MULTI_OR_EREADER) };
    unsafe { InitChooseHalfPartyForBattle(0) };
}

#[inline]
unsafe fn report_selection_result() {
    let picked = unsafe {
        (&raw const (*(&raw const crate::party_menu::gSelectedOrderFromParty)
            .cast::<[u8; 4]>()
            .cast_mut()))
            .cast::<u8>()
            .read_volatile()
    };
    unsafe { (&raw mut gSpecialVar_Result).write_volatile(u16::from(picked != 0)) };
    unsafe { SetMainCallback2(CB2_ReturnToFieldContinueScriptPlayMapMusic) };
}

unsafe fn cb2_return_from_choose_half_party() {
    unsafe { report_selection_result() };
}

#[unsafe(no_mangle)]
pub unsafe fn ChoosePartyForBattleFrontier() {
    unsafe { set_saved_callback(cb2_return_from_choose_battle_frontier_party) };
    let case_id = unsafe { (&raw const gSpecialVar_0x8004).read_volatile() } as u8 + 1;
    unsafe { InitChooseHalfPartyForBattle(case_id) };
}

unsafe fn cb2_return_from_choose_battle_frontier_party() {
    unsafe { report_selection_result() };
}

#[unsafe(no_mangle)]
pub unsafe fn ReducePlayerPartyToSelectedMons() {
    let mut party = [0u8; POKEMON_SIZE * MAX_FRONTIER_PARTY_SIZE];
    unsafe { cpu_fill32_zero(party.as_mut_ptr(), party.len()) };

    // The order array stops at the first zero, so a short selection simply
    // leaves the rest of the scratch party blank.
    for i in 0..MAX_FRONTIER_PARTY_SIZE {
        let choice = unsafe {
            (&raw const (*(&raw const crate::party_menu::gSelectedOrderFromParty)
                .cast::<[u8; 4]>()
                .cast_mut()))
                .cast::<u8>()
                .add(i)
                .read_volatile()
        };
        if choice != 0 {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    party_mon(choice as usize - 1),
                    party.as_mut_ptr().add(i * POKEMON_SIZE),
                    POKEMON_SIZE,
                )
            };
        }
    }

    unsafe { cpu_fill32_zero(party_mon(0), POKEMON_SIZE * PARTY_SIZE) };
    for i in 0..MAX_FRONTIER_PARTY_SIZE {
        unsafe {
            core::ptr::copy_nonoverlapping(
                party.as_ptr().add(i * POKEMON_SIZE),
                party_mon(i),
                POKEMON_SIZE,
            )
        };
    }

    unsafe { CalculatePlayerPartyCount() };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_frontier_party_is_a_subset_of_the_full_party() {
        assert_eq!(MAX_FRONTIER_PARTY_SIZE, 4);
        assert!(MAX_FRONTIER_PARTY_SIZE < PARTY_SIZE);
        assert_eq!(POKEMON_SIZE * MAX_FRONTIER_PARTY_SIZE, 400);
    }

    #[test]
    fn the_selection_order_is_one_based() {
        // A zero entry means "no more picks"; a 1 means party slot 0.
        let slot = |choice: u8| choice as usize - 1;
        assert_eq!(slot(1), 0);
        assert_eq!(slot(6), 5);
    }

    #[test]
    fn healing_clears_all_four_status_bytes() {
        // MON_DATA_STATUS is a u32 field, so a one-byte write would leave
        // three bytes of stack behind.
        let arg = [0u8; 4];
        assert_eq!(arg.len(), 4);
        assert_eq!(MAX_MON_MOVES, 4);
    }
}
