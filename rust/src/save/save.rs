//! Translated from `src/save.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens,
    unused_braces,
    unused_labels,
    unused_comparisons,
    overflowing_literals,
    unused_unsafe,
    dead_code,
    unreachable_code,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastWrittenSector: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastSaveCounter: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastKnownGoodSector: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gDamagedSaveSectors: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveCounter: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReadWriteSector: *mut SaveSector = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gIncrementalSectorId: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveUnusedVar: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveFileStatus: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gGameContinueCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRamSaveSectorLocations: CArray<SaveSectorLocation, 14> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveUnusedVar2: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveAttemptStatus: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSaveDataBuffer: SaveSector = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedVar: u8 = 0;

unsafe extern "C" {
    static mut EraseFlashSector: Option<unsafe extern "C" fn(u16) -> u16>;
    static mut ProgramFlashByte: Option<unsafe extern "C" fn(u16, u32, u8) -> u16>;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gFlashMemoryPresent: u32;
    static mut gPokemonStoragePtr: *mut PokemonStorage;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSoftResetDisabled: u8;
    static mut gTasks: CArray<Task, 0>;
    static mut gTrainerHillVBlankCounter: *mut u32;
    fn ClearContinueGameWarpStatus2();
    fn CopyPartyAndObjectsFromSave();
    fn CopyPartyAndObjectsToSave();
    fn DestroyTask(a0: u8);
    fn DoSaveFailedScreen(a0: u8);
    fn GetGameStat(a0: u8) -> u32;
    fn IncrementGameStat(a0: u8);
    fn IsLinkTaskFinished() -> u8;
    fn ProgramFlashSectorAndVerify(a0: u16, a1: *mut u8) -> u32;
    fn ReadFlash(a0: u16, a1: u32, a2: *mut u8, a3: u32);
    fn SaveMapView();
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetLinkStandbyCallback();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSaveData() {
    let mut i: u16 = 0;
    i = 0;
    while i < 16 {
        EraseFlashSector.unwrap_unchecked()(i);
        EraseFlashSector.unwrap_unchecked()(i + 16);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Save_ResetSaveCounters() {
    gSaveCounter = 0;
    gLastWrittenSector = 0;
    gDamagedSaveSectors = 0;
}
pub(crate) unsafe extern "C" fn SetDamagedSectorBits(op: u8, sectorId: u8) -> u32 {
    let mut retVal: u32 = FALSE as u32;
    match op {
        ENABLE => {
            gDamagedSaveSectors |= shl_i32(1, sectorId as u32) as u32;
        }
        DISABLE => {
            gDamagedSaveSectors &= !(shl_i32(1, sectorId as u32) as u32);
        }
        CHECK => {
            if gDamagedSaveSectors & shl_i32(1, sectorId as u32) as u32 != 0 {
                retVal = TRUE as u32;
            }
        }
        _ => {}
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn WriteSaveSectorOrSlot(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut status: u32 = 0;
    let mut i: u16 = 0;
    gReadWriteSector = &raw mut gSaveDataBuffer;
    if sectorId != FULL_SAVE_SLOT {
        status = HandleWriteSector(sectorId, locations) as u32;
    } else {
        gLastKnownGoodSector = gLastWrittenSector;
        gLastSaveCounter = gSaveCounter;
        gLastWrittenSector += 1;
        gLastWrittenSector = (gLastWrittenSector as i32 % 14) as u16;
        gSaveCounter += 1;
        status = SAVE_STATUS_OK as u32;
        i = 0;
        while i < NUM_SECTORS_PER_SLOT {
            HandleWriteSector(i, locations);
            i += 1;
        }
        if gDamagedSaveSectors != 0 {
            status = SAVE_STATUS_ERROR as u32;
            gLastWrittenSector = gLastKnownGoodSector;
            gSaveCounter = gLastSaveCounter;
        }
    }
    return status as u8;
}
pub(crate) unsafe extern "C" fn HandleWriteSector(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut i: u16 = 0;
    let mut sector: u16 = 0;
    let mut data: *mut u8 = null_mut();
    let mut size: u16 = 0;
    sector = sectorId + gLastWrittenSector;
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter % 2) as u16;
    data = (*locations.at(sectorId)).data as *mut u8;
    size = (*locations.at(sectorId)).size;
    i = 0;
    while i < SECTOR_SIZE as u16 {
        *(gReadWriteSector as *mut u8).at(i) = 0;
        i += 1;
    }
    (*gReadWriteSector).id = sectorId;
    (*gReadWriteSector).signature = SECTOR_SIGNATURE;
    (*gReadWriteSector).counter = gSaveCounter;
    i = 0;
    while i < size {
        (*gReadWriteSector).data[i] = *data.at(i);
        i += 1;
    }
    (*gReadWriteSector).checksum = CalculateChecksum(data as *mut c_void, size);
    return TryWriteSector(sector as u8, (*gReadWriteSector).data.as_mut_ptr());
}
pub(crate) unsafe extern "C" fn HandleWriteSectorNBytes(
    sectorId: u8,
    data: *mut u8,
    size: u16,
) -> u8 {
    let mut i: u16 = 0;
    let mut sector: *mut SaveSector = &raw mut gSaveDataBuffer;
    i = 0;
    while i < SECTOR_SIZE as u16 {
        *(sector as *mut u8).at(i) = 0;
        i += 1;
    }
    (*sector).signature = SECTOR_SIGNATURE;
    i = 0;
    while i < size {
        (*sector).data[i] = *data.at(i);
        i += 1;
    }
    (*sector).id = CalculateChecksum(data as *mut c_void, size);
    return TryWriteSector(sectorId, (*sector).data.as_mut_ptr());
}
pub(crate) unsafe extern "C" fn TryWriteSector(sector: u8, data: *mut u8) -> u8 {
    if ProgramFlashSectorAndVerify(sector as u16, data) != 0 {
        SetDamagedSectorBits(ENABLE, sector);
        return SAVE_STATUS_ERROR;
    } else {
        SetDamagedSectorBits(DISABLE, sector);
        return SAVE_STATUS_OK;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn RestoreSaveBackupVarsAndIncrement(
    locations: *mut SaveSectorLocation,
) -> u32 {
    gReadWriteSector = &raw mut gSaveDataBuffer;
    gLastKnownGoodSector = gLastWrittenSector;
    gLastSaveCounter = gSaveCounter;
    gLastWrittenSector += 1;
    gLastWrittenSector = (gLastWrittenSector as i32 % 14) as u16;
    gSaveCounter += 1;
    gIncrementalSectorId = 0;
    gDamagedSaveSectors = 0;
    return 0;
}
pub(crate) unsafe extern "C" fn RestoreSaveBackupVars(locations: *mut SaveSectorLocation) -> u32 {
    gReadWriteSector = &raw mut gSaveDataBuffer;
    gLastKnownGoodSector = gLastWrittenSector;
    gLastSaveCounter = gSaveCounter;
    gIncrementalSectorId = 0;
    gDamagedSaveSectors = 0;
    return 0;
}
pub(crate) unsafe extern "C" fn HandleWriteIncrementalSector(
    numSectors: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut status: u8 = 0;
    if (gIncrementalSectorId as i32) < numSectors as i32 - 1 {
        status = SAVE_STATUS_OK;
        HandleWriteSector(gIncrementalSectorId, locations);
        gIncrementalSectorId += 1;
        if gDamagedSaveSectors != 0 {
            status = SAVE_STATUS_ERROR;
            gLastWrittenSector = gLastKnownGoodSector;
            gSaveCounter = gLastSaveCounter;
        }
    } else {
        status = SAVE_STATUS_ERROR;
    }
    return status;
}
pub(crate) unsafe extern "C" fn HandleReplaceSectorAndVerify(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut status: u8 = SAVE_STATUS_OK;
    HandleReplaceSector(sectorId - 1, locations);
    if gDamagedSaveSectors != 0 {
        status = SAVE_STATUS_ERROR;
        gLastWrittenSector = gLastKnownGoodSector;
        gSaveCounter = gLastSaveCounter;
    }
    return status;
}
pub(crate) unsafe extern "C" fn HandleReplaceSector(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut i: u16 = 0;
    let mut sector: u16 = 0;
    let mut data: *mut u8 = null_mut();
    let mut size: u16 = 0;
    let mut status: u8 = 0;
    sector = sectorId + gLastWrittenSector;
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter % 2) as u16;
    data = (*locations.at(sectorId)).data as *mut u8;
    size = (*locations.at(sectorId)).size;
    i = 0;
    while i < SECTOR_SIZE as u16 {
        *(gReadWriteSector as *mut u8).at(i) = 0;
        i += 1;
    }
    (*gReadWriteSector).id = sectorId;
    (*gReadWriteSector).signature = SECTOR_SIGNATURE;
    (*gReadWriteSector).counter = gSaveCounter;
    i = 0;
    while i < size {
        (*gReadWriteSector).data[i] = *data.at(i);
        i += 1;
    }
    (*gReadWriteSector).checksum = CalculateChecksum(data as *mut c_void, size);
    EraseFlashSector.unwrap_unchecked()(sector);
    status = SAVE_STATUS_OK;
    i = 0;
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
        i = 0;
        while i < 7 {
            if ProgramFlashByte.unwrap_unchecked()(
                sector,
                4089 + i as u32,
                *(gReadWriteSector as *mut u8).at(4089 + i as u32),
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
            SetDamagedSectorBits(DISABLE, sector as u8);
            return SAVE_STATUS_OK;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn WriteSectorSignatureByte_NoOffset(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut sector: u16 = sectorId + gLastWrittenSector;
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter % 2) as u16;
    if ProgramFlashByte.unwrap_unchecked()(sector, 4088, 37) != 0 {
        SetDamagedSectorBits(ENABLE, sector as u8);
        gLastWrittenSector = gLastKnownGoodSector;
        gSaveCounter = gLastSaveCounter;
        return SAVE_STATUS_ERROR;
    } else {
        SetDamagedSectorBits(DISABLE, sector as u8);
        return SAVE_STATUS_OK;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CopySectorSignatureByte(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut sector: u16 = sectorId + gLastWrittenSector - 1;
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter % 2) as u16;
    if ProgramFlashByte.unwrap_unchecked()(sector, 4088, *(gReadWriteSector as *mut u8).at(4088))
        != 0
    {
        SetDamagedSectorBits(ENABLE, sector as u8);
        gLastWrittenSector = gLastKnownGoodSector;
        gSaveCounter = gLastSaveCounter;
        return SAVE_STATUS_ERROR;
    } else {
        SetDamagedSectorBits(DISABLE, sector as u8);
        return SAVE_STATUS_OK;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn WriteSectorSignatureByte(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut sector: u16 = sectorId + gLastWrittenSector - 1;
    sector = (sector as i32 % 14) as u16;
    sector += NUM_SECTORS_PER_SLOT * (gSaveCounter % 2) as u16;
    if ProgramFlashByte.unwrap_unchecked()(sector, 4088, 37) != 0 {
        SetDamagedSectorBits(ENABLE, sector as u8);
        gLastWrittenSector = gLastKnownGoodSector;
        gSaveCounter = gLastSaveCounter;
        return SAVE_STATUS_ERROR;
    } else {
        SetDamagedSectorBits(DISABLE, sector as u8);
        return SAVE_STATUS_OK;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn TryLoadSaveSlot(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut status: u8 = 0;
    gReadWriteSector = &raw mut gSaveDataBuffer;
    if sectorId != FULL_SAVE_SLOT {
        status = SAVE_STATUS_ERROR;
    } else {
        status = GetSaveValidStatus(locations);
        CopySaveSlotData(FULL_SAVE_SLOT, locations);
    }
    return status;
}
pub(crate) unsafe extern "C" fn CopySaveSlotData(
    sectorId: u16,
    locations: *mut SaveSectorLocation,
) -> u8 {
    let mut i: u16 = 0;
    let mut checksum: u16 = 0;
    let mut slotOffset: u16 = NUM_SECTORS_PER_SLOT * (gSaveCounter % 2) as u16;
    let mut id: u16 = 0;
    i = 0;
    while i < NUM_SECTORS_PER_SLOT {
        ReadFlashSector(i as u8 + slotOffset as u8, gReadWriteSector);
        id = (*gReadWriteSector).id;
        if id == 0 {
            gLastWrittenSector = i;
        }
        checksum = CalculateChecksum(
            (*gReadWriteSector).data.as_mut_ptr() as *mut c_void,
            (*locations.at(id)).size,
        );
        if (*gReadWriteSector).signature == SECTOR_SIGNATURE
            && (*gReadWriteSector).checksum == checksum
        {
            let mut j: u16 = 0;
            j = 0;
            while j < (*locations.at(id)).size {
                *((*locations.at(id)).data as *mut u8).at(j) = (*gReadWriteSector).data[j];
                j += 1;
            }
        }
        i += 1;
    }
    return SAVE_STATUS_OK;
}
pub(crate) unsafe extern "C" fn GetSaveValidStatus(locations: *mut SaveSectorLocation) -> u8 {
    let mut i: u16 = 0;
    let mut checksum: u16 = 0;
    let mut saveSlot1Counter: u32 = 0;
    let mut saveSlot2Counter: u32 = 0;
    let mut validSectorFlags: u32 = 0;
    let mut signatureValid: u8 = FALSE;
    let mut saveSlot1Status: u8 = 0;
    let mut saveSlot2Status: u8 = 0;
    i = 0;
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
    i = 0;
    while i < NUM_SECTORS_PER_SLOT {
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
        i += 1;
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
                gSaveCounter = saveSlot2Counter;
            } else {
                gSaveCounter = saveSlot1Counter;
            }
        } else {
            if saveSlot1Counter < saveSlot2Counter {
                gSaveCounter = saveSlot2Counter;
            } else {
                gSaveCounter = saveSlot1Counter;
            }
        }
        return SAVE_STATUS_OK;
    }
    if saveSlot1Status == SAVE_STATUS_OK {
        gSaveCounter = saveSlot1Counter;
        if saveSlot2Status == SAVE_STATUS_ERROR {
            return SAVE_STATUS_ERROR;
        }
        return 1;
    }
    if saveSlot2Status == SAVE_STATUS_OK {
        gSaveCounter = saveSlot2Counter;
        if saveSlot1Status == SAVE_STATUS_ERROR {
            return SAVE_STATUS_ERROR;
        }
        return 1;
    }
    if saveSlot1Status == SAVE_STATUS_EMPTY && saveSlot2Status == SAVE_STATUS_EMPTY {
        gSaveCounter = 0;
        gLastWrittenSector = 0;
        return SAVE_STATUS_EMPTY;
    }
    gSaveCounter = 0;
    gLastWrittenSector = 0;
    return SAVE_STATUS_CORRUPT as u8;
}
pub(crate) unsafe extern "C" fn TryLoadSaveSector(
    sectorId: u8,
    mut data: *mut u8,
    size: u16,
) -> u8 {
    let mut i: u16 = 0;
    let mut sector: *mut SaveSector = &raw mut gSaveDataBuffer;
    ReadFlashSector(sectorId, sector);
    if (*sector).signature == SECTOR_SIGNATURE {
        let mut checksum: u16 = CalculateChecksum((*sector).data.as_mut_ptr() as *mut c_void, size);
        if (*sector).id == checksum {
            i = 0;
            while i < size {
                *data.at(i) = (*sector).data[i];
                i += 1;
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ReadFlashSector(sectorId: u8, sector: *mut SaveSector) -> u8 {
    ReadFlash(sectorId as u16, 0, (*sector).data.as_mut_ptr(), SECTOR_SIZE);
    return TRUE;
}
pub(crate) unsafe extern "C" fn CalculateChecksum(mut data: *mut c_void, size: u16) -> u16 {
    let mut i: u16 = 0;
    let mut checksum: u32 = 0;
    i = 0;
    while (i as i32) < size as i32 / 4 {
        checksum += *(data as *mut u32);
        data = (data as *mut u8).at(4) as *mut c_void;
        i += 1;
    }
    return (checksum >> 16) as u16 + checksum as u16;
}
pub(crate) unsafe extern "C" fn UpdateSaveAddresses() {
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
        gRamSaveSectorLocations[i].data = (gPokemonStoragePtr as *mut c_void as *mut u8)
            .at(sSaveSlotLayout[i].offset) as *mut c_void;
        gRamSaveSectorLocations[i].size = sSaveSlotLayout[i].size;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleSavingData(saveType: u8) -> u8 {
    let mut i: u8 = 0;
    let mut backupVar: *mut u32 = gTrainerHillVBlankCounter;
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
            i = SECTOR_ID_HOF_1;
            while i < SECTORS_COUNT {
                EraseFlashSector.unwrap_unchecked()(i as u16);
                i += 1;
            }
        }
        if fall || sw1 == SAVE_HALL_OF_FAME {
            fall = true;
            if GetGameStat(GAME_STAT_ENTERED_HOF) < 999 {
                IncrementGameStat(GAME_STAT_ENTERED_HOF);
            }
            CopyPartyAndObjectsToSave();
            WriteSaveSectorOrSlot(FULL_SAVE_SLOT, gRamSaveSectorLocations.as_mut_ptr());
            tempAddr = gDecompressionBuffer.as_mut_ptr();
            HandleWriteSectorNBytes(SECTOR_ID_HOF_1, tempAddr, SECTOR_DATA_SIZE);
            HandleWriteSectorNBytes(SECTOR_ID_HOF_2, tempAddr.at(3968), SECTOR_DATA_SIZE);
            break 'l1;
        }
        if sw1 == SAVE_NORMAL || !matched {
            fall = true;
            CopyPartyAndObjectsToSave();
            WriteSaveSectorOrSlot(FULL_SAVE_SLOT, gRamSaveSectorLocations.as_mut_ptr());
            break 'l1;
        }
        if sw1 == SAVE_LINK || sw1 == SAVE_EREADER {
            fall = true;
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
            fall = true;
            i = SECTOR_ID_HOF_1;
            while i < SECTORS_COUNT {
                EraseFlashSector.unwrap_unchecked()(i as u16);
                i += 1;
            }
            CopyPartyAndObjectsToSave();
            WriteSaveSectorOrSlot(FULL_SAVE_SLOT, gRamSaveSectorLocations.as_mut_ptr());
            break 'l1;
        }
    }
    gTrainerHillVBlankCounter = backupVar;
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySavingData(saveType: u8) -> u8 {
    if gFlashMemoryPresent != TRUE as u32 {
        gSaveAttemptStatus = SAVE_STATUS_ERROR as u16;
        return SAVE_STATUS_ERROR;
    }
    HandleSavingData(saveType);
    if gDamagedSaveSectors == 0 {
        gSaveAttemptStatus = SAVE_STATUS_OK as u16;
        return SAVE_STATUS_OK;
    } else {
        DoSaveFailedScreen(saveType);
        gSaveAttemptStatus = SAVE_STATUS_ERROR as u16;
        return SAVE_STATUS_ERROR;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkFullSave_Init() -> u8 {
    if gFlashMemoryPresent != TRUE as u32 {
        return TRUE;
    }
    UpdateSaveAddresses();
    CopyPartyAndObjectsToSave();
    RestoreSaveBackupVarsAndIncrement(gRamSaveSectorLocations.as_mut_ptr());
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkFullSave_WriteSector() -> u8 {
    let mut status: u8 =
        HandleWriteIncrementalSector(NUM_SECTORS_PER_SLOT, gRamSaveSectorLocations.as_mut_ptr());
    if gDamagedSaveSectors != 0 {
        DoSaveFailedScreen(SAVE_NORMAL);
    }
    if status == SAVE_STATUS_ERROR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkFullSave_ReplaceLastSector() -> u8 {
    HandleReplaceSectorAndVerify(NUM_SECTORS_PER_SLOT, gRamSaveSectorLocations.as_mut_ptr());
    if gDamagedSaveSectors != 0 {
        DoSaveFailedScreen(SAVE_NORMAL);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkFullSave_SetLastSectorSignature() -> u8 {
    CopySectorSignatureByte(NUM_SECTORS_PER_SLOT, gRamSaveSectorLocations.as_mut_ptr());
    if gDamagedSaveSectors != 0 {
        DoSaveFailedScreen(SAVE_NORMAL);
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WriteSaveBlock2() -> u8 {
    if gFlashMemoryPresent != TRUE as u32 {
        return TRUE;
    }
    UpdateSaveAddresses();
    CopyPartyAndObjectsToSave();
    RestoreSaveBackupVars(gRamSaveSectorLocations.as_mut_ptr());
    HandleReplaceSectorAndVerify(
        gIncrementalSectorId + 1,
        gRamSaveSectorLocations.as_mut_ptr(),
    );
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WriteSaveBlock1Sector() -> u8 {
    let mut finished: u8 = FALSE;
    let mut sectorId: u16 = {
        gIncrementalSectorId += 1;
        gIncrementalSectorId
    };
    if sectorId <= SECTOR_ID_SAVEBLOCK1_END as u16 {
        HandleReplaceSectorAndVerify(
            gIncrementalSectorId + 1,
            gRamSaveSectorLocations.as_mut_ptr(),
        );
        WriteSectorSignatureByte(sectorId, gRamSaveSectorLocations.as_mut_ptr());
    } else {
        WriteSectorSignatureByte(sectorId, gRamSaveSectorLocations.as_mut_ptr());
        finished = TRUE;
    }
    if gDamagedSaveSectors != 0 {
        DoSaveFailedScreen(SAVE_LINK);
    }
    return finished;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadGameSave(saveType: u8) -> u8 {
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
                gDecompressionBuffer.as_mut_ptr(),
                SECTOR_DATA_SIZE,
            );
            if status == SAVE_STATUS_OK {
                status = TryLoadSaveSector(
                    SECTOR_ID_HOF_2,
                    &raw mut gDecompressionBuffer[3968],
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
    return status;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSaveBlocksPointersBaseOffset() -> u16 {
    let mut i: u16 = 0;
    let mut slotOffset: u16 = 0;
    let mut sector: *mut SaveSector = null_mut();
    sector = {
        gReadWriteSector = &raw mut gSaveDataBuffer;
        gReadWriteSector
    };
    if gFlashMemoryPresent != TRUE as u32 {
        return 0;
    }
    UpdateSaveAddresses();
    GetSaveValidStatus(gRamSaveSectorLocations.as_mut_ptr());
    slotOffset = NUM_SECTORS_PER_SLOT * (gSaveCounter % 2) as u16;
    i = 0;
    while i < NUM_SECTORS_PER_SLOT {
        ReadFlashSector(i as u8 + slotOffset as u8, gReadWriteSector);
        if (*gReadWriteSector).id == SECTOR_ID_SAVEBLOCK2 as u16 {
            return (*sector).data[10] as u16
                + (*sector).data[11] as u16
                + (*sector).data[12] as u16
                + (*sector).data[13] as u16;
        }
        i += 1;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryReadSpecialSaveSector(sector: u8, mut dst: *mut u8) -> u32 {
    let mut i: i32 = 0;
    let mut size: i32 = 0;
    let mut savData: *mut u8 = null_mut();
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
    i = 0;
    size = 4091;
    savData = &raw mut gSaveDataBuffer.data[4];
    while i <= size {
        *dst.at(i) = *savData.at(i);
        i += 1;
    }
    return SAVE_STATUS_OK as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryWriteSpecialSaveSector(sector: u8, src: *mut u8) -> u32 {
    let mut i: i32 = 0;
    let mut size: i32 = 0;
    let mut savData: *mut u8 = null_mut();
    let mut savDataBuffer: *mut c_void = null_mut();
    if sector != SECTOR_ID_TRAINER_HILL && sector != SECTOR_ID_RECORDED_BATTLE {
        return SAVE_STATUS_ERROR as u32;
    }
    savDataBuffer = &raw mut gSaveDataBuffer as *mut c_void;
    *(savDataBuffer as *mut u32) = SPECIAL_SECTOR_SENTINEL;
    i = 0;
    size = 4091;
    savData = &raw mut gSaveDataBuffer.data[4];
    while i <= size {
        *savData.at(i) = *src.at(i);
        i += 1;
    }
    if ProgramFlashSectorAndVerify(sector as u16, savDataBuffer as *mut u8) != 0 {
        return SAVE_STATUS_ERROR as u32;
    }
    return SAVE_STATUS_OK as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkFullSave(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
        11 => {
            if ({
                *data.at(1) += 1;
                *data.at(1)
            }) > 5
            {
                gSoftResetDisabled = FALSE;
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
