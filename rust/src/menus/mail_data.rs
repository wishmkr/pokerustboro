use crate::ffi::{
    GetBoxMonData2, GetMonData2, MON_DATA_HELD_ITEM, MON_DATA_MAIL, MON_DATA_PERSONALITY,
    MON_DATA_SPECIES, PARTY_SIZE, SetMonData,
};

/// `sizeof(struct Mail)`
const MAIL_SIZE: usize = 36;
const MAIL_WORDS_OFFSET: usize = 0;
const MAIL_PLAYER_NAME_OFFSET: usize = 0x12;
const MAIL_TRAINER_ID_OFFSET: usize = 0x1a;
const MAIL_SPECIES_OFFSET: usize = 0x1e;
const MAIL_ITEM_ID_OFFSET: usize = 0x20;

const MAIL_COUNT: usize = 16;
const MAIL_WORDS_COUNT: usize = 9;
const PLAYER_NAME_LENGTH: usize = 7;
const TRAINER_ID_LENGTH: usize = 4;

const EC_EMPTY_WORD: u16 = 0xffff;
const EOS: u8 = 0xff;
const CHAR_SPACE: u8 = 0x00;
const SPECIES_BULBASAUR: u16 = 1;
const SPECIES_UNOWN: u16 = 201;
const NUM_UNOWN_FORMS: u16 = 28;
const ITEM_NONE: u16 = 0;
const MAIL_NONE: u8 = 0xff;

/// `ITEM_ORANGE_MAIL` through `ITEM_RETRO_MAIL` are a contiguous run.
const FIRST_MAIL_ITEM: u16 = 121;
const LAST_MAIL_ITEM: u16 = 132;

const UNOWN_OFFSET: u16 = 30000;

/// `offsetof(struct SaveBlock1, mail)`
const SAVE1_MAIL_OFFSET: usize = 0x2be0;
/// `offsetof(struct SaveBlock2, playerName)`
const SAVE2_PLAYER_NAME_OFFSET: usize = 0x00;
/// `offsetof(struct SaveBlock2, playerTrainerId)`
const SAVE2_TRAINER_ID_OFFSET: usize = 0x0a;

/// `GetUnownLetterByPersonality` with this module's view of its types.
#[inline]
unsafe fn GetUnownLetterByPersonality(a0: u32) -> u16 {
    crate::pokemon_icon::GetUnownLetterByPersonality(a0)
}
/// `PadNameString` with this module's view of its types.
#[inline]
unsafe fn PadNameString(a0: *mut u8, a1: u8) {
    unsafe {
        crate::international_string_util::PadNameString(a0 as _, a1);
    }
}

/// `&gSaveBlock1Ptr->mail[index]`
#[inline]
unsafe fn mail_slot(index: usize) -> *mut u8 {
    unsafe {
        (*(&raw const crate::load_save::gSaveBlock1Ptr)
            .cast::<*mut u8>()
            .cast_mut())
        .add(SAVE1_MAIL_OFFSET + index * MAIL_SIZE)
    }
}

#[inline]
unsafe fn mail_item_id(mail: *mut u8) -> u16 {
    unsafe { mail.add(MAIL_ITEM_ID_OFFSET).cast::<u16>().read() }
}

#[inline]
unsafe fn set_mail_item_id(mail: *mut u8, item_id: u16) {
    unsafe { mail.add(MAIL_ITEM_ID_OFFSET).cast::<u16>().write(item_id) };
}

#[unsafe(no_mangle)]
pub unsafe fn ClearMail(mail: *mut u8) {
    let mut i = 0usize;
    while i < MAIL_WORDS_COUNT {
        unsafe {
            mail.add(MAIL_WORDS_OFFSET + i * 2)
                .cast::<u16>()
                .write(EC_EMPTY_WORD)
        };
        i += 1;
    }

    let mut i = 0usize;
    while i < PLAYER_NAME_LENGTH + 1 {
        unsafe { mail.add(MAIL_PLAYER_NAME_OFFSET + i).write(EOS) };
        i += 1;
    }

    let mut i = 0usize;
    while i < TRAINER_ID_LENGTH {
        unsafe { mail.add(MAIL_TRAINER_ID_OFFSET + i).write(0) };
        i += 1;
    }

    unsafe {
        mail.add(MAIL_SPECIES_OFFSET)
            .cast::<u16>()
            .write(SPECIES_BULBASAUR)
    };
    unsafe { set_mail_item_id(mail, ITEM_NONE) };
}

#[unsafe(no_mangle)]
pub unsafe fn ClearAllMail() {
    let mut i = 0usize;
    while i < MAIL_COUNT {
        unsafe { ClearMail(mail_slot(i)) };
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ItemIsMail(item_id: u16) -> u8 {
    u8::from((FIRST_MAIL_ITEM..=LAST_MAIL_ITEM).contains(&item_id))
}

#[unsafe(no_mangle)]
pub unsafe fn MonHasMail(mon: *mut u8) -> u8 {
    let held_item = unsafe { GetMonData2(mon, MON_DATA_HELD_ITEM) } as u16;
    let has_mail = unsafe { ItemIsMail(held_item) } != 0
        && unsafe { GetMonData2(mon, MON_DATA_MAIL) } != u32::from(MAIL_NONE);
    u8::from(has_mail)
}

#[unsafe(no_mangle)]
pub unsafe fn SpeciesToMailSpecies(species: u16, personality: u32) -> u16 {
    if species == SPECIES_UNOWN {
        unsafe { GetUnownLetterByPersonality(personality) }.wrapping_add(UNOWN_OFFSET)
    } else {
        species
    }
}

#[unsafe(no_mangle)]
pub unsafe fn MailSpeciesToSpecies(mail_species: u16, buffer: *mut u16) -> u16 {
    if mail_species >= UNOWN_OFFSET && mail_species < UNOWN_OFFSET + NUM_UNOWN_FORMS {
        unsafe { buffer.write(mail_species - UNOWN_OFFSET) };
        SPECIES_UNOWN
    } else {
        mail_species
    }
}

#[unsafe(no_mangle)]
pub unsafe fn GiveMailToMonByItemId(mon: *mut u8, item_id: u16) -> u8 {
    let held_item = [item_id as u8, (item_id >> 8) as u8];

    // Only the first PARTY_SIZE slots are handed out here; the rest of the
    // mailbox is reserved for TakeMailFromMonAndSave.
    let mut id = 0usize;
    while id < PARTY_SIZE {
        let mail = unsafe { mail_slot(id) };
        if unsafe { mail_item_id(mail) } != ITEM_NONE {
            id += 1;
            continue;
        }

        let mut i = 0usize;
        while i < MAIL_WORDS_COUNT {
            unsafe {
                mail.add(MAIL_WORDS_OFFSET + i * 2)
                    .cast::<u16>()
                    .write(EC_EMPTY_WORD)
            };
            i += 1;
        }

        let mut i = 0usize;
        while i < PLAYER_NAME_LENGTH {
            let character = unsafe {
                (*(&raw const crate::load_save::gSaveBlock2Ptr)
                    .cast::<*mut u8>()
                    .cast_mut())
                .add(SAVE2_PLAYER_NAME_OFFSET + i)
                .read()
            };
            unsafe { mail.add(MAIL_PLAYER_NAME_OFFSET + i).write(character) };
            i += 1;
        }
        unsafe {
            mail.add(MAIL_PLAYER_NAME_OFFSET + PLAYER_NAME_LENGTH)
                .write(EOS)
        };
        unsafe { PadNameString(mail.add(MAIL_PLAYER_NAME_OFFSET), CHAR_SPACE) };

        let mut i = 0usize;
        while i < TRAINER_ID_LENGTH {
            let byte = unsafe {
                (*(&raw const crate::load_save::gSaveBlock2Ptr)
                    .cast::<*mut u8>()
                    .cast_mut())
                .add(SAVE2_TRAINER_ID_OFFSET + i)
                .read()
            };
            unsafe { mail.add(MAIL_TRAINER_ID_OFFSET + i).write(byte) };
            i += 1;
        }

        // `&mon->box` is the Pokemon pointer itself; box data starts at byte 0.
        let species = unsafe { GetBoxMonData2(mon, MON_DATA_SPECIES) } as u16;
        let personality = unsafe { GetBoxMonData2(mon, MON_DATA_PERSONALITY) };
        unsafe {
            mail.add(MAIL_SPECIES_OFFSET)
                .cast::<u16>()
                .write(SpeciesToMailSpecies(species, personality))
        };
        unsafe { set_mail_item_id(mail, item_id) };

        let mail_id = id as u8;
        unsafe { SetMonData(mon, MON_DATA_MAIL, (&raw const mail_id).cast()) };
        unsafe { SetMonData(mon, MON_DATA_HELD_ITEM, held_item.as_ptr().cast()) };
        return mail_id;
    }

    MAIL_NONE
}

#[unsafe(no_mangle)]
pub unsafe fn GiveMailToMon(mon: *mut u8, mail: *mut u8) -> u8 {
    let item_id = unsafe { mail_item_id(mail) };
    let mail_id = unsafe { GiveMailToMonByItemId(mon, item_id) };

    if mail_id == MAIL_NONE {
        return MAIL_NONE;
    }

    unsafe {
        core::ptr::copy_nonoverlapping(mail.cast_const(), mail_slot(mail_id as usize), MAIL_SIZE)
    };
    unsafe { SetMonData(mon, MON_DATA_MAIL, (&raw const mail_id).cast()) };

    let held_item = [item_id as u8, (item_id >> 8) as u8];
    unsafe { SetMonData(mon, MON_DATA_HELD_ITEM, held_item.as_ptr().cast()) };

    mail_id
}

#[unsafe(no_mangle)]
pub unsafe fn TakeMailFromMon(mon: *mut u8) {
    if unsafe { MonHasMail(mon) } == 0 {
        return;
    }

    let mail_id = unsafe { GetMonData2(mon, MON_DATA_MAIL) } as u8;
    unsafe { set_mail_item_id(mail_slot(mail_id as usize), ITEM_NONE) };

    let none = MAIL_NONE;
    let held_item = [ITEM_NONE as u8, (ITEM_NONE << 8) as u8];
    unsafe { SetMonData(mon, MON_DATA_MAIL, (&raw const none).cast()) };
    unsafe { SetMonData(mon, MON_DATA_HELD_ITEM, held_item.as_ptr().cast()) };
}

#[unsafe(no_mangle)]
pub unsafe fn ClearMailItemId(mail_id: u8) {
    unsafe { set_mail_item_id(mail_slot(mail_id as usize), ITEM_NONE) };
}

#[unsafe(no_mangle)]
pub unsafe fn TakeMailFromMonAndSave(mon: *mut u8) -> u8 {
    let new_held_item = [ITEM_NONE as u8, (ITEM_NONE << 8) as u8];
    let new_mail_id = MAIL_NONE;

    // Slots below PARTY_SIZE belong to held mail; storage starts after them.
    let mut i = PARTY_SIZE;
    while i < MAIL_COUNT {
        let destination = unsafe { mail_slot(i) };
        if unsafe { mail_item_id(destination) } == ITEM_NONE {
            let source_id = unsafe { GetMonData2(mon, MON_DATA_MAIL) } as usize;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    mail_slot(source_id).cast_const(),
                    destination,
                    MAIL_SIZE,
                )
            };
            // The original re-reads MON_DATA_MAIL here rather than caching it.
            let source_id = unsafe { GetMonData2(mon, MON_DATA_MAIL) } as usize;
            unsafe { set_mail_item_id(mail_slot(source_id), ITEM_NONE) };
            unsafe { SetMonData(mon, MON_DATA_MAIL, (&raw const new_mail_id).cast()) };
            unsafe { SetMonData(mon, MON_DATA_HELD_ITEM, new_held_item.as_ptr().cast()) };
            return i as u8;
        }
        i += 1;
    }

    // No space to save mail.
    MAIL_NONE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mail_field_offsets_match_the_arm_structure() {
        assert_eq!(
            MAIL_WORDS_OFFSET + MAIL_WORDS_COUNT * 2,
            MAIL_PLAYER_NAME_OFFSET
        );
        assert_eq!(
            MAIL_PLAYER_NAME_OFFSET + PLAYER_NAME_LENGTH + 1,
            MAIL_TRAINER_ID_OFFSET
        );
        assert_eq!(
            MAIL_TRAINER_ID_OFFSET + TRAINER_ID_LENGTH,
            MAIL_SPECIES_OFFSET
        );
        assert_eq!(MAIL_SPECIES_OFFSET + 2, MAIL_ITEM_ID_OFFSET);
        // 34 used bytes, rounded up to 36 by the APCS structure-size boundary.
        assert_eq!(MAIL_ITEM_ID_OFFSET + 2, 34);
        assert_eq!(MAIL_SIZE, 36);
        assert_eq!(MAIL_SIZE % 4, 0);
    }

    #[test]
    fn the_mail_item_run_covers_all_twelve_letters() {
        assert_eq!(LAST_MAIL_ITEM - FIRST_MAIL_ITEM + 1, 12);
    }

    #[test]
    fn unown_mail_species_round_trips() {
        for letter in 0..NUM_UNOWN_FORMS {
            let mail_species = UNOWN_OFFSET + letter;
            let mut buffer = 0u16;
            let species = unsafe { MailSpeciesToSpecies(mail_species, &raw mut buffer) };
            assert_eq!(species, SPECIES_UNOWN);
            assert_eq!(buffer, letter);
        }

        let mut buffer = 0xffffu16;
        assert_eq!(
            unsafe { MailSpeciesToSpecies(UNOWN_OFFSET + NUM_UNOWN_FORMS, &raw mut buffer) },
            UNOWN_OFFSET + NUM_UNOWN_FORMS
        );
        assert_eq!(buffer, 0xffff);
    }
}
