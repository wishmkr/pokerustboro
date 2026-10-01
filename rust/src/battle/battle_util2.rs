//! Battle resource allocation, faint friendship, in-game multi party order and
//! the Battle Palace sleep/freeze escape check.

use crate::Random;
use crate::battle_main::{
    gActiveBattler, gBattleAnimBgTileBuffer, gBattleAnimBgTilemapBuffer, gBattleTypeFlags,
    gLinkBattleRecvBuffer, gLinkBattleSendBuffer,
};
use crate::event_data::VarGet;
use crate::ffi::POKEMON_SIZE;
use crate::malloc::{AllocZeroed, Free};

const BATTLE_TYPE_DOUBLE: u32 = 1 << 0;
const BATTLE_TYPE_TRAINER_HILL: u32 = 1 << 26;
const BATTLE_TYPE_SECRET_BASE: u32 = 1 << 27;
const BATTLE_BUFFER_LINK_SIZE: u32 = 0x1000;
const BATTLE_STRUCT_SIZE: u32 = 0x2a4;
const BATTLE_STRUCT_PARTY_ORDERS: usize = 0x60;
const PARTY_ORDER_SIZE: usize = 3;

const VAR_CURRENT_SECRET_BASE: u16 = 0x4054;
const SB1_SECRET_BASES: usize = 0x1a9c;
const SECRET_BASE_SIZE: usize = 0xa0;

/// `struct BattleResources`: eight pointers, each to its own allocation.
const RESOURCES_SIZE: u32 = 0x20;
const RESOURCE_SIZES: [u32; 8] = [
    0xa0, // secretBase
    0x10, // flags
    0x24, // battleScriptsStack
    0x24, // battleCallbackStack
    0x0c, // beforeLvlUp
    0x1c, // ai
    0x54, // battleHistory
    0x24, // AI_ScriptsStack
];

/// `struct BattlePokemon`
const BATTLE_MON_SIZE: usize = 0x58;
const BM_ABILITY: usize = 0x20;
const BM_LEVEL: usize = 0x2a;
const BM_STATUS1: usize = 0x4c;
const BM_STATUS2: usize = 0x50;

const B_POSITION_OPPONENT_LEFT: u8 = 1;
const B_POSITION_OPPONENT_RIGHT: u8 = 3;
const B_SIDE_OPPONENT: u8 = 1;
const FRIENDSHIP_EVENT_FAINT_SMALL: u8 = 6;
const FRIENDSHIP_EVENT_FAINT_LARGE: u8 = 8;

const MULTIUSE_STATE: usize = 0;
const MULTISTRING_CHOOSER: usize = 5;
const STATUS1_SLEEP: u32 = 7;
const STATUS1_FREEZE: u32 = 1 << 5;
const STATUS2_NIGHTMARE: u32 = 1 << 27;
const B_MSG_WOKE_UP: u8 = 0;
const B_MSG_WOKE_UP_UPROAR: u8 = 1;
const B_MSG_DEFROSTED: u8 = 0;
const ABILITY_EARLY_BIRD: u8 = 0x30;
const B_COMM_TO_CONTROLLER: u8 = 0;
const REQUEST_STATUS_BATTLE: u8 = 0x28;

/// `InitTrainerHillBattleStruct` with this module's view of its types.
#[inline]
unsafe fn InitTrainerHillBattleStruct() {
    unsafe {
        crate::trainer_hill::InitTrainerHillBattleStruct();
    }
}
/// `FreeTrainerHillBattleStruct` with this module's view of its types.
#[inline]
unsafe fn FreeTrainerHillBattleStruct() {
    unsafe {
        crate::trainer_hill::FreeTrainerHillBattleStruct();
    }
}
/// `CreateSecretBaseEnemyParty` with this module's view of its types.
#[inline]
unsafe fn CreateSecretBaseEnemyParty(a0: *mut u8) {
    unsafe {
        crate::pokemon::CreateSecretBaseEnemyParty(a0 as _);
    }
}
/// `GetBattlerAtPosition` with this module's view of its types.
#[inline]
unsafe fn GetBattlerAtPosition(a0: u8) -> u8 {
    unsafe { crate::battle_anim_mons::GetBattlerAtPosition(a0) }
}
/// `GetBattlerSide` with this module's view of its types.
#[inline]
unsafe fn GetBattlerSide(a0: u8) -> u8 {
    unsafe { crate::battle_anim_mons::GetBattlerSide(a0) }
}
/// `AdjustFriendship` with this module's view of its types.
#[inline]
unsafe fn AdjustFriendship(a0: *mut u8, a1: u8) {
    unsafe {
        crate::pokemon::AdjustFriendship(a0 as _, a1);
    }
}
/// `SwitchPartyMonSlots` with this module's view of its types.
#[inline]
unsafe fn SwitchPartyMonSlots(a0: u8, a1: u8) {
    unsafe {
        crate::party_menu::SwitchPartyMonSlots(a0, a1);
    }
}
/// `GetPartyIdFromBattlePartyId` with this module's view of its types.
#[inline]
unsafe fn GetPartyIdFromBattlePartyId(a0: u8) -> u8 {
    unsafe { crate::party_menu::GetPartyIdFromBattlePartyId(a0) }
}
/// `UproarWakeUpCheck` with this module's view of its types.
#[inline]
unsafe fn UproarWakeUpCheck(a0: u8) -> u8 {
    unsafe { crate::battle_script_commands::UproarWakeUpCheck(a0) }
}
/// `BattleScriptPushCursor` with this module's view of its types.
#[inline]
unsafe fn BattleScriptPushCursor() {
    unsafe {
        crate::battle_util::BattleScriptPushCursor();
    }
}
/// `BtlController_EmitSetMonData` with this module's view of its types.
#[inline]
unsafe fn BtlController_EmitSetMonData(a0: u8, a1: u8, a2: u8, a3: u8, a4: *mut u8) {
    unsafe {
        crate::battle_controllers::BtlController_EmitSetMonData(a0, a1, a2, a3, a4 as _);
    }
}
/// `MarkBattlerForControllerExec` with this module's view of its types.
#[inline]
unsafe fn MarkBattlerForControllerExec(a0: u8) {
    unsafe {
        crate::battle_util::MarkBattlerForControllerExec(a0);
    }
}

#[inline]
fn battle_type_has(flag: u32) -> bool {
    let flags = unsafe { (&raw const gBattleTypeFlags).read() };
    flags & flag != 0
}

#[inline]
unsafe fn battle_mon(battler: u8) -> *mut u8 {
    unsafe {
        (&raw mut (*(&raw const crate::battle_main::gBattleMons)
            .cast::<u8>()
            .cast_mut()))
            .add(usize::from(battler) * BATTLE_MON_SIZE)
    }
}

#[inline]
unsafe fn level(battler: u8) -> u8 {
    unsafe { battle_mon(battler).add(BM_LEVEL).read() }
}

#[inline]
unsafe fn status(battler: u8, offset: usize) -> *mut u32 {
    unsafe { battle_mon(battler).add(offset).cast() }
}

#[inline]
unsafe fn communication(index: usize) -> *mut u8 {
    unsafe {
        (&raw mut (*(&raw const crate::battle_main::gBattleCommunication)
            .cast::<[u8; 8]>()
            .cast_mut()))
            .cast::<u8>()
            .add(index)
    }
}

#[unsafe(no_mangle)]
pub unsafe fn AllocateBattleResources() {
    if battle_type_has(BATTLE_TYPE_TRAINER_HILL) {
        unsafe { InitTrainerHillBattleStruct() };
    }

    unsafe {
        (&raw mut (*(&raw const crate::battle_main::gBattleStruct)
            .cast::<*mut u8>()
            .cast_mut()))
            .write(AllocZeroed(BATTLE_STRUCT_SIZE))
    };
    let resources = unsafe { AllocZeroed(RESOURCES_SIZE) }.cast::<*mut u8>();
    unsafe {
        (&raw mut (*(&raw const crate::battle_main::gBattleResources)
            .cast::<*mut *mut u8>()
            .cast_mut()))
            .write(resources)
    };
    for (i, size) in RESOURCE_SIZES.into_iter().enumerate() {
        unsafe { resources.add(i).write(AllocZeroed(size)) };
    }

    unsafe { (&raw mut gLinkBattleSendBuffer).write(AllocZeroed(BATTLE_BUFFER_LINK_SIZE)) };
    unsafe { (&raw mut gLinkBattleRecvBuffer).write(AllocZeroed(BATTLE_BUFFER_LINK_SIZE)) };
    unsafe { (&raw mut gBattleAnimBgTileBuffer).write(AllocZeroed(0x2000)) };
    unsafe { (&raw mut gBattleAnimBgTilemapBuffer).write(AllocZeroed(0x1000)) };

    if battle_type_has(BATTLE_TYPE_SECRET_BASE) {
        let id = usize::from(unsafe { VarGet(VAR_CURRENT_SECRET_BASE) });
        let sb1 = unsafe {
            (&raw const (*(&raw const crate::load_save::gSaveBlock1Ptr)
                .cast::<*mut u8>()
                .cast_mut()))
                .read()
        };
        unsafe { CreateSecretBaseEnemyParty(sb1.add(SB1_SECRET_BASES + id * SECRET_BASE_SIZE)) };
    }
}

unsafe fn free_and_clear(slot: *mut *mut u8) {
    unsafe { Free(slot.read()) };
    unsafe { slot.write(core::ptr::null_mut()) };
}

#[unsafe(no_mangle)]
pub unsafe fn FreeBattleResources() {
    if battle_type_has(BATTLE_TYPE_TRAINER_HILL) {
        unsafe { FreeTrainerHillBattleStruct() };
    }

    let resources = unsafe {
        (&raw const (*(&raw const crate::battle_main::gBattleResources)
            .cast::<*mut *mut u8>()
            .cast_mut()))
            .read()
    };
    if resources.is_null() {
        return;
    }
    unsafe {
        free_and_clear(
            &raw mut (*(&raw const crate::battle_main::gBattleStruct)
                .cast::<*mut u8>()
                .cast_mut()),
        )
    };
    for i in 0..RESOURCE_SIZES.len() {
        unsafe { free_and_clear(resources.add(i)) };
    }
    unsafe {
        free_and_clear(
            (&raw mut (*(&raw const crate::battle_main::gBattleResources)
                .cast::<*mut *mut u8>()
                .cast_mut()))
                .cast(),
        )
    };
    unsafe { free_and_clear(&raw mut gLinkBattleSendBuffer) };
    unsafe { free_and_clear(&raw mut gLinkBattleRecvBuffer) };
    unsafe { free_and_clear(&raw mut gBattleAnimBgTileBuffer) };
    unsafe { free_and_clear(&raw mut gBattleAnimBgTilemapBuffer) };
}

#[unsafe(no_mangle)]
pub unsafe fn AdjustFriendshipOnBattleFaint(battler: u8) {
    let mut opponent = unsafe { GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT) };
    if battle_type_has(BATTLE_TYPE_DOUBLE) {
        let opponent2 = unsafe { GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT) };
        if unsafe { level(opponent2) } > unsafe { level(opponent) } {
            opponent = opponent2;
        }
    }

    let their_level = unsafe { level(opponent) };
    let our_level = unsafe { level(battler) };
    let event = if their_level > our_level && their_level - our_level > 29 {
        FRIENDSHIP_EVENT_FAINT_LARGE
    } else {
        FRIENDSHIP_EVENT_FAINT_SMALL
    };
    let party_index = usize::from(unsafe {
        (*(&raw const crate::battle_main::gBattlerPartyIndexes).cast::<[u16; 4]>())
            [usize::from(battler) % 4]
    });
    let mon = unsafe {
        (&raw mut (*(&raw const crate::pokemon::gPlayerParty)
            .cast::<u8>()
            .cast_mut()))
            .cast::<u8>()
            .add(party_index * POKEMON_SIZE)
    };
    unsafe { AdjustFriendship(mon, event) };
}

#[unsafe(no_mangle)]
pub unsafe fn SwitchPartyOrderInGameMulti(battler: u8, arg1: u8) {
    if unsafe { GetBattlerSide(battler) } == B_SIDE_OPPONENT {
        return;
    }
    let orders = unsafe {
        (&raw const (*(&raw const crate::battle_main::gBattleStruct)
            .cast::<*mut u8>()
            .cast_mut()))
            .read()
            .add(BATTLE_STRUCT_PARTY_ORDERS)
    };
    let current = (&raw mut (*(&raw const crate::party_menu::gBattlePartyCurrentOrder)
        .cast::<[u8; PARTY_ORDER_SIZE]>()
        .cast_mut()))
        .cast::<u8>();
    unsafe { core::ptr::copy_nonoverlapping(orders, current, PARTY_ORDER_SIZE) };

    let index = unsafe {
        (*(&raw const crate::battle_main::gBattlerPartyIndexes).cast::<[u16; 4]>())
            [usize::from(battler) % 4]
    } as u8;
    let slot = unsafe { GetPartyIdFromBattlePartyId(index) };
    let slot2 = unsafe { GetPartyIdFromBattlePartyId(arg1) };
    unsafe { SwitchPartyMonSlots(slot, slot2) };

    unsafe { core::ptr::copy_nonoverlapping(current, orders, PARTY_ORDER_SIZE) };
}

unsafe fn set_script(script: *const u8) {
    unsafe {
        (&raw mut (*(&raw const crate::battle_main::gBattlescriptCurrInstr)
            .cast::<*const u8>()
            .cast_mut()))
            .write(script)
    };
}

/// Called when a Pokémon can't attack in the Battle Palace: if it's asleep
/// or frozen, try to cure that.
#[unsafe(no_mangle)]
pub unsafe fn BattlePalace_TryEscapeStatus(battler: u8) -> u32 {
    let mut effect = 0u32;
    let state = unsafe { communication(MULTIUSE_STATE) };
    let status1 = unsafe { status(battler, BM_STATUS1) };
    let status2 = unsafe { status(battler, BM_STATUS2) };

    loop {
        match unsafe { state.read() } {
            0 => {
                if unsafe { status1.read() } & STATUS1_SLEEP != 0 {
                    if unsafe { UproarWakeUpCheck(battler) } != 0 {
                        unsafe { status1.write(status1.read() & !STATUS1_SLEEP) };
                        unsafe { status2.write(status2.read() & !STATUS2_NIGHTMARE) };
                        unsafe { BattleScriptPushCursor() };
                        unsafe { communication(MULTISTRING_CHOOSER).write(B_MSG_WOKE_UP_UPROAR) };
                        unsafe {
                            set_script(
                                &raw const (*crate::asmdata::BattleScript_MoveUsedWokeUp
                                    .cast::<u8>()),
                            )
                        };
                    } else {
                        let ability = unsafe { battle_mon(battler).add(BM_ABILITY).read() };
                        let to_sub = if ability == ABILITY_EARLY_BIRD { 2 } else { 1 };
                        let value = unsafe { status1.read() };
                        if value & STATUS1_SLEEP < to_sub {
                            unsafe { status1.write(value & !STATUS1_SLEEP) };
                        } else {
                            unsafe { status1.write(value - to_sub) };
                        }

                        if unsafe { status1.read() } & STATUS1_SLEEP != 0 {
                            unsafe {
                                set_script(
                                    &raw const (*crate::asmdata::BattleScript_MoveUsedIsAsleep
                                        .cast::<u8>()),
                                )
                            };
                        } else {
                            unsafe { status2.write(status2.read() & !STATUS2_NIGHTMARE) };
                            unsafe { BattleScriptPushCursor() };
                            unsafe { communication(MULTISTRING_CHOOSER).write(B_MSG_WOKE_UP) };
                            unsafe {
                                set_script(
                                    &raw const (*crate::asmdata::BattleScript_MoveUsedWokeUp
                                        .cast::<u8>()),
                                )
                            };
                        }
                    }
                    effect = 2;
                }
                unsafe { state.write(state.read().wrapping_add(1)) };
            }
            1 => {
                if unsafe { status1.read() } & STATUS1_FREEZE != 0 {
                    if Random() % 5 != 0 {
                        unsafe {
                            set_script(
                                &raw const (*crate::asmdata::BattleScript_MoveUsedIsFrozen
                                    .cast::<u8>()),
                            )
                        };
                    } else {
                        unsafe { status1.write(status1.read() & !STATUS1_FREEZE) };
                        unsafe { BattleScriptPushCursor() };
                        unsafe {
                            set_script(
                                &raw const (*crate::asmdata::BattleScript_MoveUsedUnfroze
                                    .cast::<u8>()),
                            )
                        };
                        unsafe { communication(MULTISTRING_CHOOSER).write(B_MSG_DEFROSTED) };
                    }
                    effect = 2;
                }
                unsafe { state.write(state.read().wrapping_add(1)) };
            }
            _ => {}
        }
        if unsafe { state.read() } == 2 || effect != 0 {
            break;
        }
    }

    if effect == 2 {
        unsafe { (&raw mut gActiveBattler).write(battler) };
        unsafe {
            BtlController_EmitSetMonData(
                B_COMM_TO_CONTROLLER,
                REQUEST_STATUS_BATTLE,
                0,
                4,
                status1.cast(),
            )
        };
        unsafe { MarkBattlerForControllerExec(battler) };
    }

    effect
}
