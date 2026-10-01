//! Translated from `src/save.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_flash::{
    EraseFlashSector, ProgramFlashByte, ProgramFlashSectorAndVerify, ReadFlash,
};
use crate::agb_main::gSoftResetDisabled;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::fieldmap::SaveMapView;
use crate::link::{IsLinkTaskFinished, SetLinkStandbyCallback};
use crate::load_save::{
    ClearContinueGameWarpStatus2, CopyPartyAndObjectsFromSave, CopyPartyAndObjectsToSave,
    SetContinueGameWarpStatusToDynamicWarp, gFlashMemoryPresent,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::overworld::{GetGameStat, IncrementGameStat};
use crate::save_failed_screen::DoSaveFailedScreen;
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::trainer_hill::gTrainerHillVBlankCounter;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sSaveSlotLayout

/// `__typeof__(sSaveSlotLayout[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sSaveSlotLayout_0_t {
    pub offset: u16,
    pub size: u16,
}

unsafe impl Sync for sSaveSlotLayout_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<sSaveSlotLayout_0_t>() == 4);
    assert!(offset_of!(sSaveSlotLayout_0_t, offset) == 0);
    assert!(offset_of!(sSaveSlotLayout_0_t, size) == 2);
};

static sSaveSlotLayout: Table<CArray<sSaveSlotLayout_0_t, 14>> =
    Table((&raw const crate::data::save::sSaveSlotLayout).cast());

#[unsafe(link_section = "common_data")]
pub static gLastWrittenSector: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gLastSaveCounter: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gLastKnownGoodSector: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static gDamagedSaveSectors: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gSaveCounter: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static mut gReadWriteSector: *mut SaveSector = null_mut();
#[unsafe(link_section = "common_data")]
pub static gIncrementalSectorId: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gSaveUnusedVar: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveFileStatus: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gGameContinueCallback: Option<unsafe fn()> = None;
#[unsafe(link_section = "common_data")]
pub static mut gRamSaveSectorLocations: CArray<SaveSectorLocation, 14> = unsafe { zeroed() };
#[unsafe(link_section = "common_data")]
pub static gSaveUnusedVar2: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gSaveAttemptStatus: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSaveDataBuffer: SaveSector = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedVar: u8 = 0;

#[unsafe(no_mangle)]
pub unsafe fn ClearSaveData() {
    for i in 0..16u16 {
        EraseFlashSector.unwrap_unchecked()(i);
        EraseFlashSector.unwrap_unchecked()(i + 16);
    }
}
#[unsafe(no_mangle)]
pub fn Save_ResetSaveCounters() {
    gSaveCounter.set(0);
    gLastWrittenSector.set(0);
    gDamagedSaveSectors.set(0);
}
fn SetDamagedSectorBits(op: u8, sectorId: u8) -> u32 {
    let mut retVal: u32 = FALSE as u32;
    match op {
        ENABLE => {
            {
                let rhs = shl_i32(1, sectorId as u32) as u32;
                gDamagedSaveSectors.set(gDamagedSaveSectors.get() | rhs)
            };
        }
        DISABLE => {
            {
                let rhs = !(shl_i32(1, sectorId as u32) as u32);
                gDamagedSaveSectors.set(gDamagedSaveSectors.get() & rhs)
            };
        }
        CHECK if gDamagedSaveSectors.get() & shl_i32(1, sectorId as u32) as u32 != 0 => {
            retVal = TRUE as u32;
        }
        _ => {}
    }
    retVal
}
unsafe fn WriteSaveSectorOrSlot(sectorId: u16, locations: *mut SaveSectorLocation) -> u8 {
    let mut status: u32 = 0;
    gReadWriteSector = &raw mut gSaveDataBuffer;
    if sectorId != FULL_SAVE_SLOT {
        status = HandleWriteSector(sectorId, locations) as u32;
    } else {
        gLastKnownGoodSector.set(gLastWrittenSector.get());
        gLastSaveCounter.set(gSaveCounter.get());
        gLastWrittenSector.set(gLastWrittenSector.get() + 1);
        gLastWrittenSector.set((gLastWrittenSector.get() as i32 % 14) as u16);
        gSaveCounter.set(gSaveCounter.get() + 1);
        status = SAVE_STATUS_OK as u32;
        for i in 0..NUM_SECTORS_PER_SLOT {
            HandleWriteSector(i, locations);
        }
        if gDamagedSaveSectors.get() != 0 {
            status = SAVE_STATUS_ERROR as u32;
            gLastWrittenSector.set(gLastKnownGoodSector.get());
            gSaveCounter.set(gLastSaveCounter.get());
        }
    }
    status as u8
}
unsafe fn HandleWriteSector(sectorId: u16, locations: *mut SaveSectorLocation) -> u8 {
    let mut data: *mut u8 = null_mut();
    let mut size: u16 = 0;
    let mut sector: u16 = sectorId + gLastWrittenSector.get();
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter.get() % 2) as u16;
    data = (*locations.at(sectorId)).data as *mut u8;
    size = (*locations.at(sectorId)).size;
    let mut i: u16 = 0;
    while i < SECTOR_SIZE as u16 {
        *(gReadWriteSector as *mut u8).at(i) = 0;
        i += 1;
    }
    (*gReadWriteSector).id = sectorId;
    (*gReadWriteSector).signature = SECTOR_SIGNATURE;
    (*gReadWriteSector).counter = gSaveCounter.get();
    for i in 0..size {
        (*gReadWriteSector).data[i] = *data.at(i);
    }
    (*gReadWriteSector).checksum = CalculateChecksum(data as *mut c_void, size);
    TryWriteSector(sector as u8, (*gReadWriteSector).data.as_mut_ptr())
}
unsafe fn HandleWriteSectorNBytes(sectorId: u8, data: *mut u8, size: u16) -> u8 {
    let sector: *mut SaveSector = &raw mut gSaveDataBuffer;
    let mut i: u16 = 0;
    while i < SECTOR_SIZE as u16 {
        *(sector as *mut u8).at(i) = 0;
        i += 1;
    }
    (*sector).signature = SECTOR_SIGNATURE;
    for i in 0..size {
        (*sector).data[i] = *data.at(i);
    }
    (*sector).id = CalculateChecksum(data as *mut c_void, size);
    TryWriteSector(sectorId, (*sector).data.as_mut_ptr())
}
unsafe fn TryWriteSector(sector: u8, data: *mut u8) -> u8 {
    if ProgramFlashSectorAndVerify(sector as u16, data) != 0 {
        SetDamagedSectorBits(ENABLE, sector);
        return SAVE_STATUS_ERROR;
    } else {
        SetDamagedSectorBits(DISABLE, sector);
        return SAVE_STATUS_OK;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn RestoreSaveBackupVarsAndIncrement(locations: *mut SaveSectorLocation) -> u32 {
    gReadWriteSector = &raw mut gSaveDataBuffer;
    gLastKnownGoodSector.set(gLastWrittenSector.get());
    gLastSaveCounter.set(gSaveCounter.get());
    gLastWrittenSector.set(gLastWrittenSector.get() + 1);
    gLastWrittenSector.set((gLastWrittenSector.get() as i32 % 14) as u16);
    gSaveCounter.set(gSaveCounter.get() + 1);
    gIncrementalSectorId.set(0);
    gDamagedSaveSectors.set(0);
    0
}
unsafe fn RestoreSaveBackupVars(locations: *mut SaveSectorLocation) -> u32 {
    gReadWriteSector = &raw mut gSaveDataBuffer;
    gLastKnownGoodSector.set(gLastWrittenSector.get());
    gLastSaveCounter.set(gSaveCounter.get());
    gIncrementalSectorId.set(0);
    gDamagedSaveSectors.set(0);
    0
}
unsafe fn HandleWriteIncrementalSector(numSectors: u16, locations: *mut SaveSectorLocation) -> u8 {
    let mut status: u8 = 0;
    if (gIncrementalSectorId.get() as i32) < numSectors as i32 - 1 {
        status = SAVE_STATUS_OK;
        HandleWriteSector(gIncrementalSectorId.get(), locations);
        gIncrementalSectorId.set(gIncrementalSectorId.get() + 1);
        if gDamagedSaveSectors.get() != 0 {
            status = SAVE_STATUS_ERROR;
            gLastWrittenSector.set(gLastKnownGoodSector.get());
            gSaveCounter.set(gLastSaveCounter.get());
        }
    } else {
        status = SAVE_STATUS_ERROR;
    }
    status
}
unsafe fn HandleReplaceSectorAndVerify(sectorId: u16, locations: *mut SaveSectorLocation) -> u8 {
    let mut status: u8 = SAVE_STATUS_OK;
    HandleReplaceSector(sectorId - 1, locations);
    if gDamagedSaveSectors.get() != 0 {
        status = SAVE_STATUS_ERROR;
        gLastWrittenSector.set(gLastKnownGoodSector.get());
        gSaveCounter.set(gLastSaveCounter.get());
    }
    status
}
unsafe fn HandleReplaceSector(sectorId: u16, locations: *mut SaveSectorLocation) -> u8 {
    let mut data: *mut u8 = null_mut();
    let mut size: u16 = 0;
    let mut sector: u16 = sectorId + gLastWrittenSector.get();
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter.get() % 2) as u16;
    data = (*locations.at(sectorId)).data as *mut u8;
    size = (*locations.at(sectorId)).size;
    for i in 0..(SECTOR_SIZE as u16) {
        *(gReadWriteSector as *mut u8).at(i) = 0;
    }
    (*gReadWriteSector).id = sectorId;
    (*gReadWriteSector).signature = SECTOR_SIGNATURE;
    (*gReadWriteSector).counter = gSaveCounter.get();
    for i in 0..size {
        (*gReadWriteSector).data[i] = *data.at(i);
    }
    (*gReadWriteSector).checksum = CalculateChecksum(data as *mut c_void, size);
    EraseFlashSector.unwrap_unchecked()(sector);
    let mut status: u8 = SAVE_STATUS_OK;
    let mut i: u16 = 0;
    while i < 4088 {
        if ProgramFlashByte.unwrap_unchecked()(
            sector,
            i as u32,
            *(gReadWriteSector as *mut u8).at(i),
        ) != 0
        {
            status = SAVE_STATUS_ERROR;
            break;
        }
        i += 1;
    }
    if status == SAVE_STATUS_ERROR {
        SetDamagedSectorBits(ENABLE, sector as u8);
        return SAVE_STATUS_ERROR;
    } else {
        status = SAVE_STATUS_OK;
        for i in 0..7u16 {
            if ProgramFlashByte.unwrap_unchecked()(
                sector,
                4089 + i as u32,
                *(gReadWriteSector as *mut u8).at(4089 + i as u32),
            ) != 0
            {
                status = SAVE_STATUS_ERROR;
                break;
            }
        }
        if status == SAVE_STATUS_ERROR {
            SetDamagedSectorBits(ENABLE, sector as u8);
            return SAVE_STATUS_ERROR;
        } else {
            SetDamagedSectorBits(DISABLE, sector as u8);
            return SAVE_STATUS_OK;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn WriteSectorSignatureByte_NoOffset(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut sector: u16 = sectorId + gLastWrittenSector.get();
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter.get() % 2) as u16;
    if ProgramFlashByte.unwrap_unchecked()(sector, 4088, 37) != 0 {
        SetDamagedSectorBits(ENABLE, sector as u8);
        gLastWrittenSector.set(gLastKnownGoodSector.get());
        gSaveCounter.set(gLastSaveCounter.get());
        return SAVE_STATUS_ERROR;
    } else {
        SetDamagedSectorBits(DISABLE, sector as u8);
        return SAVE_STATUS_OK;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CopySectorSignatureByte(sectorId: u16, locations: *mut SaveSectorLocation) -> u8 {
    let mut sector: u16 = sectorId + gLastWrittenSector.get() - 1;
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter.get() % 2) as u16;
    if ProgramFlashByte.unwrap_unchecked()(sector, 4088, *(gReadWriteSector as *mut u8).at(4088))
        != 0
    {
        SetDamagedSectorBits(ENABLE, sector as u8);
        gLastWrittenSector.set(gLastKnownGoodSector.get());
        gSaveCounter.set(gLastSaveCounter.get());
        return SAVE_STATUS_ERROR;
    } else {
        SetDamagedSectorBits(DISABLE, sector as u8);
        return SAVE_STATUS_OK;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn WriteSectorSignatureByte(sectorId: u16, locations: *mut SaveSectorLocation) -> u8 {
    let mut sector: u16 = sectorId + gLastWrittenSector.get() - 1;
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter.get() % 2) as u16;
    if ProgramFlashByte.unwrap_unchecked()(sector, 4088, 37) != 0 {
        SetDamagedSectorBits(ENABLE, sector as u8);
        gLastWrittenSector.set(gLastKnownGoodSector.get());
        gSaveCounter.set(gLastSaveCounter.get());
        return SAVE_STATUS_ERROR;
    } else {
        SetDamagedSectorBits(DISABLE, sector as u8);
        return SAVE_STATUS_OK;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn TryLoadSaveSlot(sectorId: u16, locations: *mut SaveSectorLocation) -> u8 {
    let mut status: u8 = 0;
    gReadWriteSector = &raw mut gSaveDataBuffer;
    if sectorId != FULL_SAVE_SLOT {
        status = SAVE_STATUS_ERROR;
    } else {
        status = GetSaveValidStatus(locations);
        CopySaveSlotData(FULL_SAVE_SLOT, locations);
    }
    status
}
unsafe fn CopySaveSlotData(sectorId: u16, locations: *mut SaveSectorLocation) -> u8 {
    let mut checksum: u16 = 0;
    let slotOffset: u16 = NUM_SECTORS_PER_SLOT * (gSaveCounter.get() % 2) as u16;
    let mut id: u16 = 0;
    for i in 0..NUM_SECTORS_PER_SLOT {
        ReadFlashSector(i as u8 + slotOffset as u8, gReadWriteSector);
        id = (*gReadWriteSector).id;
        if id == 0 {
            gLastWrittenSector.set(i);
        }
        checksum = CalculateChecksum(
            (*gReadWriteSector).data.as_mut_ptr() as *mut c_void,
            (*locations.at(id)).size,
        );
        if (*gReadWriteSector).signature == SECTOR_SIGNATURE
            && (*gReadWriteSector).checksum == checksum
        {
            let mut j: u16 = 0;
            while j < (*locations.at(id)).size {
                *((*locations.at(id)).data as *mut u8).at(j) = (*gReadWriteSector).data[j];
                j += 1;
            }
        }
    }
    SAVE_STATUS_OK
}
unsafe fn GetSaveValidStatus(locations: *mut SaveSectorLocation) -> u8 {
    let mut checksum: u16 = 0;
    let mut saveSlot1Counter: u32 = 0;
    let mut saveSlot2Counter: u32 = 0;
    let mut validSectorFlags: u32 = 0;
    let mut signatureValid: u8 = FALSE;
    let mut saveSlot1Status: u8 = 0;
    let mut saveSlot2Status: u8 = 0;
    let mut i: u16 = 0;
    while i < NUM_SECTORS_PER_SLOT {
        ReadFlashSector(i as u8, gReadWriteSector);
        if (*gReadWriteSector).signature == SECTOR_SIGNATURE {
            signatureValid = TRUE;
            checksum = CalculateChecksum(
                (*gReadWriteSector).data.as_mut_ptr() as *mut c_void,
                (*locations.at((*gReadWriteSector).id)).size,
            );
            if (*gReadWriteSector).checksum == checksum {
                saveSlot1Counter = (*gReadWriteSector).counter;
                validSectorFlags |= shl_i32(1, (*gReadWriteSector).id as u32) as u32;
            }
        }
        i += 1;
    }
    if signatureValid != 0 {
        if validSectorFlags == 16383 {
            saveSlot1Status = SAVE_STATUS_OK;
        } else {
            saveSlot1Status = SAVE_STATUS_ERROR;
        }
    } else {
        saveSlot1Status = SAVE_STATUS_EMPTY;
    }
    validSectorFlags = 0;
    signatureValid = FALSE;
    for i in 0..NUM_SECTORS_PER_SLOT {
        ReadFlashSector(i as u8 + NUM_SECTORS_PER_SLOT as u8, gReadWriteSector);
        if (*gReadWriteSector).signature == SECTOR_SIGNATURE {
            signatureValid = TRUE;
            checksum = CalculateChecksum(
                (*gReadWriteSector).data.as_mut_ptr() as *mut c_void,
                (*locations.at((*gReadWriteSector).id)).size,
            );
            if (*gReadWriteSector).checksum == checksum {
                saveSlot2Counter = (*gReadWriteSector).counter;
                validSectorFlags |= shl_i32(1, (*gReadWriteSector).id as u32) as u32;
            }
        }
    }
    if signatureValid != 0 {
        if validSectorFlags == 16383 {
            saveSlot2Status = SAVE_STATUS_OK;
        } else {
            saveSlot2Status = SAVE_STATUS_ERROR;
        }
    } else {
        saveSlot2Status = SAVE_STATUS_EMPTY;
    }
    if saveSlot1Status == SAVE_STATUS_OK && saveSlot2Status == SAVE_STATUS_OK {
        if saveSlot1Counter == 0xffffffff && saveSlot2Counter == 0
            || saveSlot1Counter == 0 && saveSlot2Counter == 0xffffffff
        {
            if saveSlot1Counter + 1 < saveSlot2Counter + 1 {
                gSaveCounter.set(saveSlot2Counter);
            } else {
                gSaveCounter.set(saveSlot1Counter);
            }
        } else {
            if saveSlot1Counter < saveSlot2Counter {
                gSaveCounter.set(saveSlot2Counter);
            } else {
                gSaveCounter.set(saveSlot1Counter);
            }
        }
        return SAVE_STATUS_OK;
    }
    if saveSlot1Status == SAVE_STATUS_OK {
        gSaveCounter.set(saveSlot1Counter);
        if saveSlot2Status == SAVE_STATUS_ERROR {
            return SAVE_STATUS_ERROR;
        }
        return 1;
    }
    if saveSlot2Status == SAVE_STATUS_OK {
        gSaveCounter.set(saveSlot2Counter);
        if saveSlot1Status == SAVE_STATUS_ERROR {
            return SAVE_STATUS_ERROR;
        }
        return 1;
    }
    if saveSlot1Status == SAVE_STATUS_EMPTY && saveSlot2Status == SAVE_STATUS_EMPTY {
        gSaveCounter.set(0);
        gLastWrittenSector.set(0);
        return SAVE_STATUS_EMPTY;
    }
    gSaveCounter.set(0);
    gLastWrittenSector.set(0);
    SAVE_STATUS_CORRUPT as u8
}
unsafe fn TryLoadSaveSector(sectorId: u8, data: *mut u8, size: u16) -> u8 {
    let sector: *mut SaveSector = &raw mut gSaveDataBuffer;
    ReadFlashSector(sectorId, sector);
    if (*sector).signature == SECTOR_SIGNATURE {
        let checksum: u16 = CalculateChecksum((*sector).data.as_mut_ptr() as *mut c_void, size);
        if (*sector).id == checksum {
            for i in 0..size {
                *data.at(i) = (*sector).data[i];
            }
            return SAVE_STATUS_OK;
        } else {
            return SAVE_STATUS_CORRUPT as u8;
        }
    } else {
        return SAVE_STATUS_EMPTY;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ReadFlashSector(sectorId: u8, sector: *mut SaveSector) -> u8 {
    ReadFlash(sectorId as u16, 0, (*sector).data.as_mut_ptr(), SECTOR_SIZE);
    TRUE
}
unsafe fn CalculateChecksum(mut data: *mut c_void, size: u16) -> u16 {
    let mut checksum: u32 = 0;
    let mut i: u16 = 0;
    while (i as i32) < size as i32 / 4 {
        checksum += *(data as *mut u32);
        data = (data as *mut u8).at(4) as *mut c_void;
        i += 1;
    }
    (checksum >> 16) as u16 + checksum as u16
}
unsafe fn UpdateSaveAddresses() {
    let mut i: i32 = SECTOR_ID_SAVEBLOCK2 as i32;
    gRamSaveSectorLocations[i].data =
        (gSaveBlock2Ptr as *mut c_void as *mut u8).at(sSaveSlotLayout[i].offset) as *mut c_void;
    gRamSaveSectorLocations[i].size = sSaveSlotLayout[i].size;
    i = SECTOR_ID_SAVEBLOCK1_START;
    while i <= SECTOR_ID_SAVEBLOCK1_END as i32 {
        gRamSaveSectorLocations[i].data =
            (gSaveBlock1Ptr as *mut c_void as *mut u8).at(sSaveSlotLayout[i].offset) as *mut c_void;
        gRamSaveSectorLocations[i].size = sSaveSlotLayout[i].size;
        i += 1;
    }
    while i <= SECTOR_ID_PKMN_STORAGE_END {
        gRamSaveSectorLocations[i].data = ((*(&raw const crate::load_save::gPokemonStoragePtr)
            .cast::<*mut PokemonStorage>()
            .cast_mut()) as *mut c_void as *mut u8)
            .at(sSaveSlotLayout[i].offset) as *mut c_void;
        gRamSaveSectorLocations[i].size = sSaveSlotLayout[i].size;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn HandleSavingData(saveType: u8) -> u8 {
    let mut i: u8 = 0;
    let backupVar: *mut u32 = gTrainerHillVBlankCounter;
    let mut tempAddr: *mut u8 = null_mut();
    gTrainerHillVBlankCounter = null_mut();
    UpdateSaveAddresses();
    'l1: {
        let sw1: u8 = saveType;
        let matched = sw1 == SAVE_HALL_OF_FAME_ERASE_BEFORE
            || sw1 == SAVE_HALL_OF_FAME
            || sw1 == SAVE_NORMAL
            || sw1 == SAVE_LINK
            || sw1 == SAVE_EREADER
            || sw1 == SAVE_OVERWRITE_DIFFERENT_FILE;
        let mut fall = false;
        if sw1 == SAVE_HALL_OF_FAME_ERASE_BEFORE {
            fall = true;
            for i in SECTOR_ID_HOF_1..SECTORS_COUNT {
                EraseFlashSector.unwrap_unchecked()(i as u16);
            }
        }
        if fall || sw1 == SAVE_HALL_OF_FAME {
            if GetGameStat(GAME_STAT_ENTERED_HOF) < 999 {
                IncrementGameStat(GAME_STAT_ENTERED_HOF);
            }
            CopyPartyAndObjectsToSave();
            WriteSaveSectorOrSlot(FULL_SAVE_SLOT, gRamSaveSectorLocations.as_mut_ptr());
            tempAddr = (*(&raw const crate::decompress::gDecompressionBuffer)
                .cast::<CArray<u8, 16384>>()
                .cast_mut())
            .as_mut_ptr();
            HandleWriteSectorNBytes(SECTOR_ID_HOF_1, tempAddr, SECTOR_DATA_SIZE);
            HandleWriteSectorNBytes(SECTOR_ID_HOF_2, tempAddr.at(3968), SECTOR_DATA_SIZE);
            break 'l1;
        }
        if sw1 == SAVE_NORMAL || !matched {
            CopyPartyAndObjectsToSave();
            WriteSaveSectorOrSlot(FULL_SAVE_SLOT, gRamSaveSectorLocations.as_mut_ptr());
            break 'l1;
        }
        if sw1 == SAVE_LINK || sw1 == SAVE_EREADER {
            CopyPartyAndObjectsToSave();
            i = SECTOR_ID_SAVEBLOCK2;
            while i <= SECTOR_ID_SAVEBLOCK1_END {
                HandleReplaceSector(i as u16, gRamSaveSectorLocations.as_mut_ptr());
                i += 1;
            }
            i = SECTOR_ID_SAVEBLOCK2;
            while i <= SECTOR_ID_SAVEBLOCK1_END {
                WriteSectorSignatureByte_NoOffset(i as u16, gRamSaveSectorLocations.as_mut_ptr());
                i += 1;
            }
            break 'l1;
        }
        if sw1 == SAVE_OVERWRITE_DIFFERENT_FILE {
            for i in SECTOR_ID_HOF_1..SECTORS_COUNT {
                EraseFlashSector.unwrap_unchecked()(i as u16);
            }
            CopyPartyAndObjectsToSave();
            WriteSaveSectorOrSlot(FULL_SAVE_SLOT, gRamSaveSectorLocations.as_mut_ptr());
            break 'l1;
        }
    }
    gTrainerHillVBlankCounter = backupVar;
    0
}
#[unsafe(no_mangle)]
pub unsafe fn TrySavingData(saveType: u8) -> u8 {
    if gFlashMemoryPresent != TRUE as u32 {
        gSaveAttemptStatus.set(SAVE_STATUS_ERROR as u16);
        return SAVE_STATUS_ERROR;
    }
    HandleSavingData(saveType);
    if gDamagedSaveSectors.get() == 0 {
        gSaveAttemptStatus.set(SAVE_STATUS_OK as u16);
        return SAVE_STATUS_OK;
    } else {
        DoSaveFailedScreen(saveType);
        gSaveAttemptStatus.set(SAVE_STATUS_ERROR as u16);
        return SAVE_STATUS_ERROR;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn LinkFullSave_Init() -> u8 {
    if gFlashMemoryPresent != TRUE as u32 {
        return TRUE;
    }
    UpdateSaveAddresses();
    CopyPartyAndObjectsToSave();
    RestoreSaveBackupVarsAndIncrement(gRamSaveSectorLocations.as_mut_ptr());
    FALSE
}
pub unsafe fn LinkFullSave_WriteSector() -> u8 {
    let status: u8 =
        HandleWriteIncrementalSector(NUM_SECTORS_PER_SLOT, gRamSaveSectorLocations.as_mut_ptr());
    if gDamagedSaveSectors.get() != 0 {
        DoSaveFailedScreen(SAVE_NORMAL);
    }
    if status == SAVE_STATUS_ERROR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn LinkFullSave_ReplaceLastSector() -> u8 {
    HandleReplaceSectorAndVerify(NUM_SECTORS_PER_SLOT, gRamSaveSectorLocations.as_mut_ptr());
    if gDamagedSaveSectors.get() != 0 {
        DoSaveFailedScreen(SAVE_NORMAL);
    }
    FALSE
}
pub unsafe fn LinkFullSave_SetLastSectorSignature() -> u8 {
    CopySectorSignatureByte(NUM_SECTORS_PER_SLOT, gRamSaveSectorLocations.as_mut_ptr());
    if gDamagedSaveSectors.get() != 0 {
        DoSaveFailedScreen(SAVE_NORMAL);
    }
    FALSE
}
pub unsafe fn WriteSaveBlock2() -> u8 {
    if gFlashMemoryPresent != TRUE as u32 {
        return TRUE;
    }
    UpdateSaveAddresses();
    CopyPartyAndObjectsToSave();
    RestoreSaveBackupVars(gRamSaveSectorLocations.as_mut_ptr());
    HandleReplaceSectorAndVerify(
        gIncrementalSectorId.get() + 1,
        gRamSaveSectorLocations.as_mut_ptr(),
    );
    FALSE
}
pub unsafe fn WriteSaveBlock1Sector() -> u8 {
    let mut finished: u8 = FALSE;
    let sectorId: u16 = {
        gIncrementalSectorId.set(gIncrementalSectorId.get() + 1);
        gIncrementalSectorId.get()
    };
    if sectorId <= SECTOR_ID_SAVEBLOCK1_END as u16 {
        HandleReplaceSectorAndVerify(
            gIncrementalSectorId.get() + 1,
            gRamSaveSectorLocations.as_mut_ptr(),
        );
        WriteSectorSignatureByte(sectorId, gRamSaveSectorLocations.as_mut_ptr());
    } else {
        WriteSectorSignatureByte(sectorId, gRamSaveSectorLocations.as_mut_ptr());
        finished = TRUE;
    }
    if gDamagedSaveSectors.get() != 0 {
        DoSaveFailedScreen(SAVE_LINK);
    }
    finished
}
#[unsafe(no_mangle)]
pub unsafe fn LoadGameSave(saveType: u8) -> u8 {
    let mut status: u8 = 0;
    if gFlashMemoryPresent != TRUE as u32 {
        gSaveFileStatus = SAVE_STATUS_NO_FLASH;
        return SAVE_STATUS_ERROR;
    }
    UpdateSaveAddresses();
    match saveType {
        SAVE_HALL_OF_FAME => {
            status = TryLoadSaveSector(
                SECTOR_ID_HOF_1,
                (*(&raw const crate::decompress::gDecompressionBuffer)
                    .cast::<CArray<u8, 16384>>()
                    .cast_mut())
                .as_mut_ptr(),
                SECTOR_DATA_SIZE,
            );
            if status == SAVE_STATUS_OK {
                status = TryLoadSaveSector(
                    SECTOR_ID_HOF_2,
                    &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
                        .cast::<CArray<u8, 16384>>()
                        .cast_mut())[3968],
                    SECTOR_DATA_SIZE,
                );
            }
        }
        _ => {
            status = TryLoadSaveSlot(FULL_SAVE_SLOT, gRamSaveSectorLocations.as_mut_ptr());
            CopyPartyAndObjectsFromSave();
            gSaveFileStatus = status as u16;
            gGameContinueCallback = None;
        }
    }
    status
}
#[unsafe(no_mangle)]
pub unsafe fn GetSaveBlocksPointersBaseOffset() -> u16 {
    let sector: *mut SaveSector = {
        gReadWriteSector = &raw mut gSaveDataBuffer;
        gReadWriteSector
    };
    if gFlashMemoryPresent != TRUE as u32 {
        return 0;
    }
    UpdateSaveAddresses();
    GetSaveValidStatus(gRamSaveSectorLocations.as_mut_ptr());
    let slotOffset: u16 = NUM_SECTORS_PER_SLOT * (gSaveCounter.get() % 2) as u16;
    for i in 0..NUM_SECTORS_PER_SLOT {
        ReadFlashSector(i as u8 + slotOffset as u8, gReadWriteSector);
        if (*gReadWriteSector).id == SECTOR_ID_SAVEBLOCK2 as u16 {
            return (*sector).data[10] as u16
                + (*sector).data[11] as u16
                + (*sector).data[12] as u16
                + (*sector).data[13] as u16;
        }
    }
    0
}
pub unsafe fn TryReadSpecialSaveSector(sector: u8, dst: *mut u8) -> u32 {
    if sector != SECTOR_ID_TRAINER_HILL && sector != SECTOR_ID_RECORDED_BATTLE {
        return SAVE_STATUS_ERROR as u32;
    }
    ReadFlash(
        sector as u16,
        0,
        &raw mut gSaveDataBuffer as *mut u8,
        SECTOR_SIZE,
    );
    if *(&raw mut gSaveDataBuffer.data[0] as *mut u32) != SPECIAL_SECTOR_SENTINEL {
        return SAVE_STATUS_ERROR as u32;
    }
    let mut i: i32 = 0;
    let size: i32 = 4091;
    let savData: *mut u8 = &raw mut gSaveDataBuffer.data[4];
    while i <= size {
        *dst.at(i) = *savData.at(i);
        i += 1;
    }
    SAVE_STATUS_OK as u32
}
pub unsafe fn TryWriteSpecialSaveSector(sector: u8, src: *mut u8) -> u32 {
    if sector != SECTOR_ID_TRAINER_HILL && sector != SECTOR_ID_RECORDED_BATTLE {
        return SAVE_STATUS_ERROR as u32;
    }
    let savDataBuffer: *mut c_void = &raw mut gSaveDataBuffer as *mut c_void;
    *(savDataBuffer as *mut u32) = SPECIAL_SECTOR_SENTINEL;
    let mut i: i32 = 0;
    let size: i32 = 4091;
    let savData: *mut u8 = &raw mut gSaveDataBuffer.data[4];
    while i <= size {
        *savData.at(i) = *src.at(i);
        i += 1;
    }
    if ProgramFlashSectorAndVerify(sector as u16, savDataBuffer as *mut u8) != 0 {
        return SAVE_STATUS_ERROR as u32;
    }
    SAVE_STATUS_OK as u32
}
pub unsafe fn Task_LinkFullSave(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            gSoftResetDisabled = TRUE;
            *data = 1;
        }
        1 => {
            SetLinkStandbyCallback();
            *data = 2;
        }
        2 => {
            if IsLinkTaskFinished() != 0 {
                if *data.at(2) == 0 {
                    SaveMapView();
                }
                *data = 3;
            }
        }
        3 => {
            if *data.at(2) == 0 {
                SetContinueGameWarpStatusToDynamicWarp();
            }
            LinkFullSave_Init();
            *data = 4;
        }
        4 => {
            if ({
                *data.at(1) += 1;
                *data.at(1)
            }) == 5
            {
                *data.at(1) = 0;
                *data = 5;
            }
        }
        5 => {
            if LinkFullSave_WriteSector() != 0 {
                *data = 6;
            } else {
                *data = 4;
            }
        }
        6 => {
            LinkFullSave_ReplaceLastSector();
            *data = 7;
        }
        7 => {
            if *data.at(2) == 0 {
                ClearContinueGameWarpStatus2();
            }
            SetLinkStandbyCallback();
            *data = 8;
        }
        8 => {
            if IsLinkTaskFinished() != 0 {
                LinkFullSave_SetLastSectorSignature();
                *data = 9;
            }
        }
        9 => {
            SetLinkStandbyCallback();
            *data = 10;
        }
        10 => {
            if IsLinkTaskFinished() != 0 {
                *data += 1;
            }
        }
        11 if ({
            *data.at(1) += 1;
            *data.at(1)
        }) > 5 =>
        {
            gSoftResetDisabled = FALSE;
            DestroyTask(taskId);
        }
        _ => {}
    }
}
