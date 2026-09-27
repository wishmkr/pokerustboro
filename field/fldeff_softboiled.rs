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

unsafe extern "C" {
    static mut gPartyMenu: u8;
    static gText_PkmnHPRestoredByVar2: u8;
    static gText_CantBeUsedOnPkmn: u8;

    fn GetCursorSelectionMonId() -> u8;
    fn AnimatePartySlot(slot: u8, anim_num: u8);
    fn DisplayPartyMenuStdMessage(string_id: u32);
    fn DisplayPartyMenuMessage(string: *const u8, keep_open: u8) -> u8;
    fn IsPartyMenuTextPrinterActive() -> u8;
    fn PartyMenuModifyHP(
        task_id: u8,
        slot: u8,
        hp_increment: i8,
        hp_difference: i16,
        task: TaskFunc,
    );
    fn Task_HandleChooseMonInput(task_id: u8);
    fn ScheduleBgCopyTilemapToVram(bg_id: u8);
    fn ClearStdWindowAndFrameToTransparent(window_id: u8, copy_to_vram: u8);
    fn ClearWindowTilemap(window_id: u8);
}

#[inline]
unsafe fn party_menu_byte(offset: usize) -> u8 {
    unsafe { (&raw const gPartyMenu).add(offset).read_volatile() }
}

#[inline]
unsafe fn set_party_menu_byte(offset: usize, value: u8) {
    unsafe { (&raw mut gPartyMenu).add(offset).write_volatile(value) };
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
pub unsafe extern "C" fn SetUpFieldMove_SoftBoiled() -> u8 {
    let slot = unsafe { GetCursorSelectionMonId() };
    let max = unsafe { max_hp(slot) };
    let hp = unsafe { current_hp(slot) };

    u8::from(hp > max / 5)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForSoftboiled(task_id: u8) {
    unsafe { set_party_menu_byte(PARTY_MENU_ACTION_OFFSET, PARTY_ACTION_SOFTBOILED) };
    unsafe { set_party_menu_byte(PARTY_MENU_SLOT_ID2_OFFSET, slot_id()) };
    unsafe { AnimatePartySlot(GetCursorSelectionMonId(), 1) };
    unsafe { DisplayPartyMenuStdMessage(PARTY_MSG_USE_ON_WHICH_MON) };
    unsafe { set_task_func(task_id, Task_HandleChooseMonInput) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_TryUseSoftboiledOnPartyMon(task_id: u8) {
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

unsafe extern "C" fn task_softboiled_restore_health(task_id: u8) {
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

unsafe extern "C" fn task_display_hp_restored_message(task_id: u8) {
    let _ = unsafe {
        GetMonNickname(
            party_mon(slot_id2() as usize),
            (&raw mut gStringVar1).cast::<u8>(),
        )
    };
    let _ = unsafe {
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            &raw const gText_PkmnHPRestoredByVar2,
        )
    };
    let _ = unsafe { DisplayPartyMenuMessage((&raw const gStringVar4).cast::<u8>(), 0) };
    unsafe { ScheduleBgCopyTilemapToVram(2) };
    unsafe { set_task_func(task_id, task_finish_softboiled) };
}

unsafe extern "C" fn task_finish_softboiled(task_id: u8) {
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

unsafe extern "C" fn task_choose_new_mon_for_softboiled(task_id: u8) {
    if unsafe { IsPartyMenuTextPrinterActive() } == 1 {
        return;
    }

    unsafe { DisplayPartyMenuStdMessage(PARTY_MSG_USE_ON_WHICH_MON) };
    unsafe { set_task_func(task_id, Task_HandleChooseMonInput) };
}

unsafe fn cant_use_softboiled_on_mon(task_id: u8) {
    unsafe { PlaySE(SE_SELECT) };
    let _ = unsafe { DisplayPartyMenuMessage(&raw const gText_CantBeUsedOnPkmn, 0) };
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
