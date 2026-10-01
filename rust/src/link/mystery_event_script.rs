//! The interpreter for Mystery Event scripts received over the link cable
//! (mystery_event_menu.rs). It reuses the field script engine with its own
//! command table; addresses inside the received script are relative to the
//! sender's base and get rebased onto our buffer.

use crate::ffi::{POKEMON_SIZE, VarSet, gStringVar1, gStringVar2, gStringVar4};
use crate::gift_ribbon::GiveGiftRibbonToParty;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::pokemon::gPlayerPartyCount;
use crate::string_util::{StringCompare, StringCopyN, StringExpandPlaceholders};
use crate::util::{CalcByteArraySum, CalcCRC16};

/// 0x1 in FireRed, 0x2 in LeafGreen, 0x80 in Ruby, 0x100 in Sapphire.
const VERSION_MASK: u32 = 1 << 9;

const MEVENT_STATUS_LOAD_OK: u32 = 0;
const MEVENT_STATUS_LOAD_ERROR: u32 = 1;
const MEVENT_STATUS_SUCCESS: u32 = 2;
const MEVENT_STATUS_FAILURE: u32 = 3;
const MEVENT_STATUS_FF: u8 = 0xff;

/// `struct ScriptContext`: `scriptPtr` at 8, `data[4]` at 0x64.
const SCRIPT_CONTEXT_SIZE: usize = 0x74;
const CTX_SCRIPT_PTR: usize = 0x08;
const CTX_DATA: usize = 0x64;
const SCRIPT_BASE: usize = 0;
const OFFSET: usize = 1;
const STATUS: usize = 2;
const VALID: usize = 3;

const SB1_RECORD_MIXING_GIFT: usize = 0x3b14;
const GIFT_SIZE: usize = 0x10;
const GIFT_CHECKSUM: usize = 0;
const GIFT_DATA: usize = 4;
const GIFT_DATA_SIZE: usize = 0x0c;
const SB1_ENIGMA_BERRY_NAME: usize = 0x31f8;
const BERRY_NAME_LENGTH: u8 = 6;
const VAR_ENIGMA_BERRY_AVAILABLE: u16 = 0x402d;
const MON_DATA_SPECIES_OR_EGG: i32 = 0x41;
const MON_DATA_HELD_ITEM: i32 = 0x0c;
const SPECIES_EGG: u16 = 0x19c;
const POKEMON_NAME_LENGTH: u8 = 10;
const MAIL_SIZE: usize = 0x24;
const PARTY_SIZE: u8 = 6;
const FLAG_SET_SEEN: u8 = 2;
const FLAG_SET_CAUGHT: u8 = 3;
const SB2_EREADER_TRAINER: usize = 0xbec;
const EREADER_TRAINER_SIZE: usize = 0xbc;

#[unsafe(link_section = "ewram_data")]
static mut CONTEXT: crate::ffi::Align4<[u8; SCRIPT_CONTEXT_SIZE]> =
    crate::ffi::Align4([0; SCRIPT_CONTEXT_SIZE]);

/// `InitScriptContext` with this module's view of its types.
#[inline]
unsafe fn InitScriptContext(a0: *mut u8, a1: *const u8, a2: *const u8) {
    unsafe {
        crate::script::InitScriptContext(a0 as _, a1 as _, a2 as _);
    }
}
/// `SetupBytecodeScript` with this module's view of its types.
#[inline]
unsafe fn SetupBytecodeScript(a0: *mut u8, a1: *const u8) -> u8 {
    unsafe { crate::script::SetupBytecodeScript(a0 as _, a1 as _) }
}
/// `RunScriptCommand` with this module's view of its types.
#[inline]
unsafe fn RunScriptCommand(a0: *mut u8) -> u8 {
    unsafe { crate::script::RunScriptCommand(a0 as _) }
}
/// `StopScript` with this module's view of its types.
#[inline]
unsafe fn StopScript(a0: *mut u8) {
    unsafe {
        crate::script::StopScript(a0 as _);
    }
}
/// `ScriptReadWord` with this module's view of its types.
#[inline]
unsafe fn ScriptReadWord(a0: *mut u8) -> u32 {
    unsafe { crate::script::ScriptReadWord(a0 as _) }
}
/// `ScriptReadHalfword` with this module's view of its types.
#[inline]
unsafe fn ScriptReadHalfword(a0: *mut u8) -> u16 {
    unsafe { crate::script::ScriptReadHalfword(a0 as _) }
}
/// `RunScriptImmediately` with this module's view of its types.
#[inline]
unsafe fn RunScriptImmediately(a0: *const u8) {
    unsafe {
        crate::script::RunScriptImmediately(a0 as _);
    }
}
/// `IsEnigmaBerryValid` with this module's view of its types.
#[inline]
unsafe fn IsEnigmaBerryValid() -> u32 {
    unsafe { crate::berry::IsEnigmaBerryValid() }
}
/// `SetEnigmaBerry` with this module's view of its types.
#[inline]
unsafe fn SetEnigmaBerry(a0: *mut u8) {
    unsafe {
        crate::berry::SetEnigmaBerry(a0 as _);
    }
}
/// `InitRamScript` with this module's view of its types.
#[inline]
unsafe fn InitRamScript(a0: *const u8, a1: u16, a2: u8, a3: u8, a4: u8) -> u8 {
    unsafe { crate::script::InitRamScript(a0 as _, a1, a2, a3, a4) }
}
/// `EnableNationalPokedex` with this module's view of its types.
#[inline]
unsafe fn EnableNationalPokedex() {
    {
        crate::event_data::EnableNationalPokedex();
    }
}
/// `UnlockTrendySaying` with this module's view of its types.
#[inline]
unsafe fn UnlockTrendySaying(a0: u8) {
    unsafe {
        crate::easy_chat::UnlockTrendySaying(a0);
    }
}
/// `GetMonData2` with this module's view of its types.
#[inline]
unsafe fn GetMonData2(a0: *mut u8, a1: i32) -> u32 {
    unsafe { crate::pokemon::GetMonData2(a0 as _, a1) }
}
/// `SpeciesToNationalPokedexNum` with this module's view of its types.
#[inline]
unsafe fn SpeciesToNationalPokedexNum(a0: u16) -> u16 {
    unsafe { crate::pokemon::SpeciesToNationalPokedexNum(a0) }
}
/// `GetSetPokedexFlag` with this module's view of its types.
#[inline]
unsafe fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8 {
    unsafe { crate::pokedex::GetSetPokedexFlag(a0, a1) }
}
/// `ItemIsMail` with this module's view of its types.
#[inline]
unsafe fn ItemIsMail(a0: u16) -> u8 {
    unsafe { crate::mail_data::ItemIsMail(a0) }
}
/// `GiveMailToMon` with this module's view of its types.
#[inline]
unsafe fn GiveMailToMon(a0: *mut u8, a1: *mut u8) -> u8 {
    unsafe { crate::mail_data::GiveMailToMon(a0 as _, a1 as _) }
}
/// `CompactPartySlots` with this module's view of its types.
#[inline]
unsafe fn CompactPartySlots() -> i16 {
    unsafe { crate::pokemon_storage_system::CompactPartySlots() }
}
/// `CalculatePlayerPartyCount` with this module's view of its types.
#[inline]
unsafe fn CalculatePlayerPartyCount() -> u8 {
    unsafe { crate::pokemon::CalculatePlayerPartyCount() }
}
/// `ValidateEReaderTrainer` with this module's view of its types.
#[inline]
unsafe fn ValidateEReaderTrainer() {
    unsafe {
        crate::battle_tower::ValidateEReaderTrainer();
    }
}
/// `EnableResetRTC` with this module's view of its types.
#[inline]
unsafe fn EnableResetRTC() {
    {
        crate::event_data::EnableResetRTC();
    }
}

#[inline]
fn context() -> *mut u8 {
    (&raw mut CONTEXT).cast()
}

#[inline]
unsafe fn data(ctx: *mut u8, index: usize) -> *mut u32 {
    unsafe { ctx.add(CTX_DATA + index * 4).cast() }
}

/// `*ctx->scriptPtr++`
#[inline]
unsafe fn read_byte(ctx: *mut u8) -> u8 {
    let ptr = unsafe { ctx.add(CTX_SCRIPT_PTR).cast::<*const u8>() };
    let current = unsafe { ptr.read() };
    unsafe { ptr.write(current.wrapping_add(1)) };
    unsafe { current.read() }
}

/// Reads a sender-relative address and rebases it onto our copy.
#[inline]
unsafe fn read_pointer(ctx: *mut u8) -> *mut u8 {
    let raw = unsafe { ScriptReadWord(ctx) };
    let base = unsafe { data(ctx, SCRIPT_BASE).read() };
    let offset = unsafe { data(ctx, OFFSET).read() };
    raw.wrapping_sub(offset).wrapping_add(base) as usize as *mut u8
}

#[inline]
unsafe fn set_status(ctx: *mut u8, status: u32) {
    unsafe { data(ctx, STATUS).write(status) };
}

#[inline]
unsafe fn expand(message: *const u8) {
    unsafe { StringExpandPlaceholders((&raw mut gStringVar4).cast(), message) };
}

fn check_compatibility(unk0: u16, unk1: u32, unk2: u16, version: u32) -> bool {
    // 0x1 in English FRLG, 0x2 English RS, 0x4 German RS; then 0x1 FRLG / 0x4 RS.
    unk0 & 0x1 != 0 && unk1 & 0x1 != 0 && unk2 & 0x4 != 0 && version & VERSION_MASK != 0
}

unsafe fn init_mystery_event_script(ctx: *mut u8, script: *mut u8) {
    unsafe {
        InitScriptContext(
            ctx,
            &raw const (*crate::asmdata::gMysteryEventScriptCmdTable.cast::<u8>()),
            &raw const (*crate::asmdata::gMysteryEventScriptCmdTableEnd.cast::<u8>()),
        )
    };
    unsafe { SetupBytecodeScript(ctx, script) };
    unsafe { data(ctx, SCRIPT_BASE).write(script as usize as u32) };
    unsafe { data(ctx, OFFSET).write(0) };
    unsafe { set_status(ctx, MEVENT_STATUS_LOAD_OK) };
    unsafe { data(ctx, VALID).write(0) };
}

unsafe fn run_command(ctx: *mut u8) -> bool {
    let ran = unsafe { RunScriptCommand(ctx) } != 0;
    ran && unsafe { data(ctx, VALID).read() } != 0
}

#[unsafe(no_mangle)]
pub unsafe fn InitMysteryEventScriptContext(script: *mut u8) {
    unsafe { init_mystery_event_script(context(), script) };
}

#[unsafe(no_mangle)]
pub unsafe fn RunMysteryEventScriptContextCommand(status: *mut u32) -> u32 {
    let ctx = context();
    let running = unsafe { run_command(ctx) };
    unsafe { status.write(data(ctx, STATUS).read()) };
    u32::from(running)
}

#[unsafe(no_mangle)]
pub unsafe fn RunMysteryEventScript(script: *mut u8) -> u32 {
    let ctx = context();
    unsafe { init_mystery_event_script(ctx, script) };
    while unsafe { run_command(ctx) } {}
    unsafe { data(ctx, STATUS).read() }
}

#[unsafe(no_mangle)]
pub unsafe fn SetMysteryEventScriptStatus(status: u32) {
    unsafe { set_status(context(), status) };
}

#[inline]
unsafe fn gift() -> *mut u8 {
    unsafe {
        (&raw const gSaveBlock1Ptr)
            .read()
            .cast::<u8>()
            .add(SB1_RECORD_MIXING_GIFT)
    }
}

unsafe fn gift_checksum() -> i32 {
    let data = unsafe { gift().add(GIFT_DATA) };
    (0..GIFT_DATA_SIZE)
        .map(|i| i32::from(unsafe { data.add(i).read() }))
        .sum()
}

unsafe fn clear_record_mixing_gift() {
    unsafe { gift().write_bytes(0, GIFT_SIZE) };
}

#[unsafe(no_mangle)]
pub unsafe fn GetRecordMixingGift() -> u16 {
    let g = unsafe { gift() };
    let data = unsafe { g.add(GIFT_DATA) };
    let checksum = unsafe { gift_checksum() };
    let stored = unsafe { g.add(GIFT_CHECKSUM).cast::<u32>().read() } as i32;
    let (unk0, quantity, item_id) = unsafe {
        (
            data.read(),
            data.add(1).read(),
            data.add(2).cast::<u16>().read(),
        )
    };
    if unk0 == 0 || quantity == 0 || item_id == 0 || checksum == 0 || checksum != stored {
        unsafe { clear_record_mixing_gift() };
        return 0;
    }
    let remaining = quantity - 1;
    unsafe { data.add(1).write(remaining) };
    if remaining == 0 {
        unsafe { clear_record_mixing_gift() };
    } else {
        unsafe {
            g.add(GIFT_CHECKSUM)
                .cast::<u32>()
                .write(gift_checksum() as u32)
        };
    }
    item_id
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_end(ctx: *mut u8) -> u8 {
    unsafe { StopScript(ctx) };
    1
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_checkcompat(ctx: *mut u8) -> u8 {
    let offset = unsafe { ScriptReadWord(ctx) };
    unsafe { data(ctx, OFFSET).write(offset) };
    let unk0 = unsafe { ScriptReadHalfword(ctx) };
    let unk1 = unsafe { ScriptReadWord(ctx) };
    let unk2 = unsafe { ScriptReadHalfword(ctx) };
    let version = unsafe { ScriptReadWord(ctx) };
    if check_compatibility(unk0, unk1, unk2, version) {
        unsafe { data(ctx, VALID).write(1) };
    } else {
        unsafe {
            expand(&raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventCantBeUsed).cast::<u8>()))
        };
        unsafe { set_status(context(), MEVENT_STATUS_FAILURE) };
    }
    1
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_nop(_ctx: *mut u8) -> u8 {
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_setstatus(ctx: *mut u8) -> u8 {
    let status = unsafe { read_byte(ctx) };
    unsafe { set_status(ctx, u32::from(status)) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_setmsg(ctx: *mut u8) -> u8 {
    let status = unsafe { read_byte(ctx) };
    let message = unsafe { read_pointer(ctx) };
    if status == MEVENT_STATUS_FF || u32::from(status) == unsafe { data(ctx, STATUS).read() } {
        unsafe { expand(message) };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_runscript(ctx: *mut u8) -> u8 {
    let script = unsafe { read_pointer(ctx) };
    unsafe { RunScriptImmediately(script) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_setenigmaberry(ctx: *mut u8) -> u8 {
    let had_berry = unsafe { IsEnigmaBerryValid() } != 0;
    let berry = unsafe { read_pointer(ctx) };
    let name = unsafe {
        (&raw const gSaveBlock1Ptr)
            .read()
            .cast::<u8>()
            .add(SB1_ENIGMA_BERRY_NAME)
    };
    let var1 = (&raw mut gStringVar1).cast::<u8>();
    let var2 = (&raw mut gStringVar2).cast::<u8>();
    unsafe { StringCopyN(var1, name, BERRY_NAME_LENGTH + 1) };
    unsafe { SetEnigmaBerry(berry) };
    unsafe { StringCopyN(var2, name, BERRY_NAME_LENGTH + 1) };

    let message = if !had_berry {
        &raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventBerry)
            .cast::<u8>())
    } else if unsafe { StringCompare(var1, var2) } != 0 {
        &raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventBerryTransform)
            .cast::<u8>())
    } else {
        &raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventBerryObtained)
            .cast::<u8>())
    };
    unsafe { expand(message) };
    unsafe { set_status(ctx, MEVENT_STATUS_SUCCESS) };
    if unsafe { IsEnigmaBerryValid() } == 1 {
        unsafe { VarSet(VAR_ENIGMA_BERRY_AVAILABLE, 1) };
    } else {
        unsafe { set_status(ctx, MEVENT_STATUS_LOAD_ERROR) };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_giveribbon(ctx: *mut u8) -> u8 {
    let index = unsafe { read_byte(ctx) };
    let ribbon_id = unsafe { read_byte(ctx) };
    GiveGiftRibbonToParty(index, ribbon_id);
    unsafe {
        expand(&raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventSpecialRibbon).cast::<u8>()))
    };
    unsafe { set_status(ctx, MEVENT_STATUS_SUCCESS) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_initramscript(ctx: *mut u8) -> u8 {
    let map_group = unsafe { read_byte(ctx) };
    let map_num = unsafe { read_byte(ctx) };
    let object_id = unsafe { read_byte(ctx) };
    let script = unsafe { read_pointer(ctx) };
    let script_end = unsafe { read_pointer(ctx) };
    let size = (script_end as usize).wrapping_sub(script as usize) as u16;
    unsafe { InitRamScript(script, size, map_group, map_num, object_id) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_givenationaldex(ctx: *mut u8) -> u8 {
    unsafe { EnableNationalPokedex() };
    unsafe {
        expand(&raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventNationalDex).cast::<u8>()))
    };
    unsafe { set_status(ctx, MEVENT_STATUS_SUCCESS) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_addrareword(ctx: *mut u8) -> u8 {
    let word = unsafe { read_byte(ctx) };
    unsafe { UnlockTrendySaying(word) };
    unsafe {
        expand(
            &raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventRareWord)
                .cast::<u8>()),
        )
    };
    unsafe { set_status(ctx, MEVENT_STATUS_SUCCESS) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_setrecordmixinggift(ctx: *mut u8) -> u8 {
    let unk = unsafe { read_byte(ctx) };
    let quantity = unsafe { read_byte(ctx) };
    let item_id = unsafe { ScriptReadHalfword(ctx) };
    if unk == 0 || quantity == 0 || item_id == 0 {
        unsafe { clear_record_mixing_gift() };
    } else {
        let data = unsafe { gift().add(GIFT_DATA) };
        unsafe { data.write(unk) };
        unsafe { data.add(1).write(quantity) };
        unsafe { data.add(2).cast::<u16>().write(item_id) };
        unsafe {
            gift()
                .add(GIFT_CHECKSUM)
                .cast::<u32>()
                .write(gift_checksum() as u32)
        };
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_givepokemon(ctx: *mut u8) -> u8 {
    let source = unsafe { read_pointer(ctx) };
    let mut pokemon = crate::ffi::Align4([0u8; POKEMON_SIZE]);
    let mut mail = crate::ffi::Align4([0u8; MAIL_SIZE]);
    unsafe { core::ptr::copy_nonoverlapping(source, pokemon.0.as_mut_ptr(), POKEMON_SIZE) };
    let species = unsafe { GetMonData2(pokemon.0.as_mut_ptr(), MON_DATA_SPECIES_OR_EGG) } as u16;
    let name = if species == SPECIES_EGG {
        &raw const (*(&raw const crate::data::strings::gText_EggNickname).cast::<u8>())
    } else {
        &raw const (*(&raw const crate::data::strings::gText_Pokemon).cast::<u8>())
    };
    unsafe { StringCopyN((&raw mut gStringVar1).cast(), name, POKEMON_NAME_LENGTH + 1) };

    if unsafe { (&raw const gPlayerPartyCount).read() } == PARTY_SIZE {
        unsafe {
            expand(&raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventFullParty).cast::<u8>()))
        };
        unsafe { set_status(ctx, MEVENT_STATUS_FAILURE) };
        return 0;
    }

    let last = unsafe {
        (&raw mut (*(&raw const crate::pokemon::gPlayerParty)
            .cast::<u8>()
            .cast_mut()))
            .cast::<u8>()
            .add((PARTY_SIZE as usize - 1) * POKEMON_SIZE)
    };
    unsafe { core::ptr::copy_nonoverlapping(source, last, POKEMON_SIZE) };
    unsafe {
        core::ptr::copy_nonoverlapping(source.add(POKEMON_SIZE), mail.0.as_mut_ptr(), MAIL_SIZE)
    };
    if species != SPECIES_EGG {
        let dex = unsafe { SpeciesToNationalPokedexNum(species) };
        unsafe { GetSetPokedexFlag(dex, FLAG_SET_SEEN) };
        unsafe { GetSetPokedexFlag(dex, FLAG_SET_CAUGHT) };
    }
    let held_item = unsafe { GetMonData2(last, MON_DATA_HELD_ITEM) } as u16;
    if unsafe { ItemIsMail(held_item) } != 0 {
        unsafe { GiveMailToMon(last, mail.0.as_mut_ptr()) };
    }
    unsafe { CompactPartySlots() };
    unsafe { CalculatePlayerPartyCount() };
    unsafe {
        expand(
            &raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventSentOver)
                .cast::<u8>()),
        )
    };
    unsafe { set_status(ctx, MEVENT_STATUS_SUCCESS) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_addtrainer(ctx: *mut u8) -> u8 {
    let source = unsafe { read_pointer(ctx) };
    let dest = unsafe {
        (&raw const gSaveBlock2Ptr)
            .read()
            .cast::<u8>()
            .add(SB2_EREADER_TRAINER)
    };
    unsafe { core::ptr::copy_nonoverlapping(source, dest, EREADER_TRAINER_SIZE) };
    unsafe { ValidateEReaderTrainer() };
    unsafe {
        expand(
            &raw const (*(&raw const crate::data::mystery_event_msg::gText_MysteryEventNewTrainer)
                .cast::<u8>()),
        )
    };
    unsafe { set_status(ctx, MEVENT_STATUS_SUCCESS) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_enableresetrtc(ctx: *mut u8) -> u8 {
    unsafe { EnableResetRTC() };
    unsafe {
        expand(
            &raw const (*(&raw const crate::data::strings::gText_InGameClockUsable).cast::<u8>()),
        )
    };
    unsafe { set_status(ctx, MEVENT_STATUS_SUCCESS) };
    0
}

unsafe fn read_range(ctx: *mut u8) -> (*mut u8, usize) {
    let start = unsafe { read_pointer(ctx) };
    let end = unsafe { read_pointer(ctx) };
    (start, (end as usize).wrapping_sub(start as usize))
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_checksum(ctx: *mut u8) -> u8 {
    let expected = unsafe { ScriptReadWord(ctx) } as i32;
    let (start, length) = unsafe { read_range(ctx) };
    if expected != unsafe { CalcByteArraySum(start, length as u32) } as i32 {
        unsafe { data(ctx, VALID).write(0) };
        unsafe { set_status(ctx, MEVENT_STATUS_LOAD_ERROR) };
    }
    1
}

#[unsafe(no_mangle)]
pub unsafe fn MEScrCmd_crc(ctx: *mut u8) -> u8 {
    let expected = unsafe { ScriptReadWord(ctx) } as i32;
    let (start, length) = unsafe { read_range(ctx) };
    if expected != i32::from(unsafe { CalcCRC16(start, length as i32) }) {
        unsafe { data(ctx, VALID).write(0) };
        unsafe { set_status(ctx, MEVENT_STATUS_LOAD_ERROR) };
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_emerald_scripts_are_compatible() {
        assert!(check_compatibility(1, 1, 4, 1 << 9));
        assert!(!check_compatibility(1, 1, 4, 0x80));
        assert!(!check_compatibility(0, 1, 4, 1 << 9));
    }
}
