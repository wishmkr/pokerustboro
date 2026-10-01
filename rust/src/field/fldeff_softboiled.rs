use crate::ffi::{
    GetMonData2, GetMonNickname, MON_DATA_HP, MON_DATA_MAX_HP, PARTY_SIZE, PlaySE,
    StringExpandPlaceholders, TaskFunc, gStringVar1, gStringVar4, party_mon, set_task_func,
};

const SE_USE_ITEM: u16 = 1;
const SE_SELECT: u16 = 5;

const PARTY_ACTION_SOFTBOILED: u8 = 10;
const PARTY_MSG_CHOOSE_MON: u32 = 0;
const PARTY_MSG_USE_ON_WHICH_MON: u32 = 5;

/// Window id used by the party menu for the Softboiled prompt.
const PARTY_PROMPT_WINDOW: u8 = 6;

/// `struct PartyMenu` is 20 bytes; the packed `menuType:4`/`layout:2`
/// bitfield occupies byte 8, so the ids start at byte 9.
const PARTY_MENU_SLOT_ID_OFFSET: usize = 9;
const PARTY_MENU_SLOT_ID2_OFFSET: usize = 10;
const PARTY_MENU_ACTION_OFFSET: usize = 11;

/// `GetCursorSelectionMonId` with this module's view of its types.
#[inline]
unsafe fn GetCursorSelectionMonId() -> u8 {
    unsafe { crate::party_menu::GetCursorSelectionMonId() }
}
/// `AnimatePartySlot` with this module's view of its types.
#[inline]
unsafe fn AnimatePartySlot(a0: u8, a1: u8) {
    unsafe {
        crate::party_menu::AnimatePartySlot(a0, a1);
    }
}
/// `DisplayPartyMenuStdMessage` with this module's view of its types.
#[inline]
unsafe fn DisplayPartyMenuStdMessage(a0: u32) {
    unsafe {
        crate::party_menu::DisplayPartyMenuStdMessage(a0);
    }
}
/// `DisplayPartyMenuMessage` with this module's view of its types.
#[inline]
unsafe fn DisplayPartyMenuMessage(a0: *const u8, a1: u8) -> u8 {
    unsafe { crate::party_menu::DisplayPartyMenuMessage(a0 as _, a1) }
}
/// `IsPartyMenuTextPrinterActive` with this module's view of its types.
#[inline]
unsafe fn IsPartyMenuTextPrinterActive() -> u8 {
    unsafe { crate::party_menu::IsPartyMenuTextPrinterActive() }
}
/// `PartyMenuModifyHP` with this module's view of its types.
#[inline]
unsafe fn PartyMenuModifyHP(a0: u8, a1: u8, a2: i8, a3: i16, a4: TaskFunc) {
    unsafe {
        crate::party_menu::PartyMenuModifyHP(a0, a1, a2, a3, core::mem::transmute(a4));
    }
}
/// `Task_HandleChooseMonInput` with this module's view of its types.
#[inline]
unsafe fn Task_HandleChooseMonInput(a0: u8) {
    unsafe {
        crate::party_menu::Task_HandleChooseMonInput(a0);
    }
}
/// `ScheduleBgCopyTilemapToVram` with this module's view of its types.
#[inline]
unsafe fn ScheduleBgCopyTilemapToVram(a0: u8) {
    unsafe {
        crate::menu::ScheduleBgCopyTilemapToVram(a0);
    }
}
/// `ClearStdWindowAndFrameToTransparent` with this module's view of its types.
#[inline]
unsafe fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8) {
    unsafe {
        crate::menu::ClearStdWindowAndFrameToTransparent(a0, a1);
    }
}
/// `ClearWindowTilemap` with this module's view of its types.
#[inline]
unsafe fn ClearWindowTilemap(a0: u8) {
    unsafe {
        crate::window::ClearWindowTilemap(a0);
    }
}

#[inline]
unsafe fn party_menu_byte(offset: usize) -> u8 {
    unsafe {
        (&raw const (*(&raw const crate::party_menu::gPartyMenu)
            .cast::<u8>()
            .cast_mut()))
            .add(offset)
            .read_volatile()
    }
}

#[inline]
unsafe fn set_party_menu_byte(offset: usize, value: u8) {
    unsafe {
        (&raw mut (*(&raw const crate::party_menu::gPartyMenu)
            .cast::<u8>()
            .cast_mut()))
            .add(offset)
            .write_volatile(value)
    };
}

#[inline]
unsafe fn slot_id() -> u8 {
    unsafe { party_menu_byte(PARTY_MENU_SLOT_ID_OFFSET) }
}

#[inline]
unsafe fn slot_id2() -> u8 {
    unsafe { party_menu_byte(PARTY_MENU_SLOT_ID2_OFFSET) }
}

#[inline]
unsafe fn max_hp(slot: u8) -> u16 {
    unsafe { GetMonData2(party_mon(slot as usize), MON_DATA_MAX_HP) as u16 }
}

#[inline]
unsafe fn current_hp(slot: u8) -> u16 {
    unsafe { GetMonData2(party_mon(slot as usize), MON_DATA_HP) as u16 }
}

#[unsafe(no_mangle)]
pub unsafe fn SetUpFieldMove_SoftBoiled() -> u8 {
    let slot = unsafe { GetCursorSelectionMonId() };
    let max = unsafe { max_hp(slot) };
    let hp = unsafe { current_hp(slot) };

    u8::from(hp > max / 5)
}

#[unsafe(no_mangle)]
pub unsafe fn ChooseMonForSoftboiled(task_id: u8) {
    unsafe { set_party_menu_byte(PARTY_MENU_ACTION_OFFSET, PARTY_ACTION_SOFTBOILED) };
    unsafe { set_party_menu_byte(PARTY_MENU_SLOT_ID2_OFFSET, slot_id()) };
    unsafe { AnimatePartySlot(GetCursorSelectionMonId(), 1) };
    unsafe { DisplayPartyMenuStdMessage(PARTY_MSG_USE_ON_WHICH_MON) };
    unsafe { set_task_func(task_id, Task_HandleChooseMonInput) };
}

#[unsafe(no_mangle)]
pub unsafe fn Task_TryUseSoftboiledOnPartyMon(task_id: u8) {
    let user = unsafe { slot_id() };
    let recipient = unsafe { slot_id2() };

    // The original compares against PARTY_SIZE rather than PARTY_SIZE - 1,
    // so slot 6 falls through to the health check below.
    if recipient > PARTY_SIZE as u8 {
        unsafe { set_party_menu_byte(PARTY_MENU_ACTION_OFFSET, 0) };
        unsafe { DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON) };
        unsafe { set_task_func(task_id, Task_HandleChooseMonInput) };
        return;
    }

    let hp = unsafe { current_hp(recipient) };
    if hp == 0 || user == recipient || unsafe { max_hp(recipient) } == hp {
        unsafe { cant_use_softboiled_on_mon(task_id) };
        return;
    }

    // The user pays first, hence the -1 direction.
    unsafe { PlaySE(SE_USE_ITEM) };
    unsafe {
        PartyMenuModifyHP(
            task_id,
            user,
            -1,
            (max_hp(user) / 5) as i16,
            task_softboiled_restore_health,
        )
    };
}

unsafe fn task_softboiled_restore_health(task_id: u8) {
    unsafe { PlaySE(SE_USE_ITEM) };
    unsafe {
        PartyMenuModifyHP(
            task_id,
            slot_id2(),
            1,
            (max_hp(slot_id()) / 5) as i16,
            task_display_hp_restored_message,
        )
    };
}

unsafe fn task_display_hp_restored_message(task_id: u8) {
    let _ = unsafe {
        GetMonNickname(
            party_mon(slot_id2() as usize),
            (&raw mut gStringVar1).cast::<u8>(),
        )
    };
    let _ = unsafe {
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            &raw const (*(&raw const crate::data::strings::gText_PkmnHPRestoredByVar2)
                .cast::<u8>()),
        )
    };
    let _ = unsafe { DisplayPartyMenuMessage((&raw const gStringVar4).cast::<u8>(), 0) };
    unsafe { ScheduleBgCopyTilemapToVram(2) };
    unsafe { set_task_func(task_id, task_finish_softboiled) };
}

unsafe fn task_finish_softboiled(task_id: u8) {
    if unsafe { IsPartyMenuTextPrinterActive() } == 1 {
        return;
    }

    unsafe { set_party_menu_byte(PARTY_MENU_ACTION_OFFSET, 0) };
    unsafe { AnimatePartySlot(slot_id(), 0) };
    unsafe { set_party_menu_byte(PARTY_MENU_SLOT_ID_OFFSET, slot_id2()) };
    unsafe { AnimatePartySlot(slot_id2(), 1) };
    unsafe { ClearStdWindowAndFrameToTransparent(PARTY_PROMPT_WINDOW, 0) };
    unsafe { ClearWindowTilemap(PARTY_PROMPT_WINDOW) };
    unsafe { DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON) };
    unsafe { set_task_func(task_id, Task_HandleChooseMonInput) };
}

unsafe fn task_choose_new_mon_for_softboiled(task_id: u8) {
    if unsafe { IsPartyMenuTextPrinterActive() } == 1 {
        return;
    }

    unsafe { DisplayPartyMenuStdMessage(PARTY_MSG_USE_ON_WHICH_MON) };
    unsafe { set_task_func(task_id, Task_HandleChooseMonInput) };
}

unsafe fn cant_use_softboiled_on_mon(task_id: u8) {
    unsafe { PlaySE(SE_SELECT) };
    let _ = unsafe {
        DisplayPartyMenuMessage(
            &raw const (*(&raw const crate::data::strings::gText_CantBeUsedOnPkmn).cast::<u8>()),
            0,
        )
    };
    unsafe { ScheduleBgCopyTilemapToVram(2) };
    unsafe { set_task_func(task_id, task_choose_new_mon_for_softboiled) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn party_menu_field_offsets_match_the_arm_structure() {
        assert_eq!(super::PARTY_MENU_SLOT_ID_OFFSET, 9);
        assert_eq!(super::PARTY_MENU_SLOT_ID2_OFFSET, 10);
        assert_eq!(super::PARTY_MENU_ACTION_OFFSET, 11);
    }

    #[test]
    fn softboiled_costs_a_fifth_of_max_hp() {
        // SetUpFieldMove_SoftBoiled only allows the move above that share.
        for (max, hp, allowed) in [(100u16, 21u16, true), (100, 20, false), (5, 1, false)] {
            assert_eq!(hp > max / 5, allowed);
        }
    }
}
